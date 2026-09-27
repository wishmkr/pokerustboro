//! Translated from `src/save.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sSaveSlotLayout
#[allow(unused_imports)]
use crate::data::save::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastWrittenSector: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastSaveCounter: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastKnownGoodSector: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDamagedSaveSectors: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveCounter: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadWriteSector: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gIncrementalSectorId: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveUnusedVar: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveFileStatus: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gGameContinueCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRamSaveSectorLocations: crate::ffi::Align4<[u8; 112]> =
    crate::ffi::Align4([0; 112]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveUnusedVar2: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSaveAttemptStatus: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSaveDataBuffer: crate::ffi::Align4<[u8; 4096]> = crate::ffi::Align4([0; 4096]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedVar: u8 = 0u8;

unsafe extern "C" {
    static mut EraseFlashSector: u8;
    static mut ProgramFlashByte: u8;
    static mut gDecompressionBuffer: u8;
    static mut gFlashMemoryPresent: u8;
    static mut gPokemonStoragePtr: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSoftResetDisabled: u8;
    static mut gTasks: u8;
    static mut gTrainerHillVBlankCounter: u8;
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
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(32i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut EraseFlashSector)
                        .cast::<Option<unsafe extern "C" fn(u16) -> u16>>())
                    .read())
                    .unwrap_unchecked()(i);
                    (((&raw mut EraseFlashSector)
                        .cast::<Option<unsafe extern "C" fn(u16) -> u16>>())
                    .read())
                    .unwrap_unchecked()(
                        ((((i) as i32).wrapping_add(crate::c::div_i32(32i32, 2i32))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Save_ResetSaveCounters() {
    unsafe {
        ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(0u32);
        ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).write(0u32);
    }
}
pub(crate) unsafe extern "C" fn SetDamagedSectorBits(op: u8, sectorId: u8) -> u32 {
    unsafe {
        let mut op = op;
        let mut sectorId = sectorId;
        let mut retVal: u32 = 0u32;
        'l1: {
            let __sw1 = ((op) as i32);
            if __sw1 == 0i32 {
                let __p2 = (&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>();
                (__p2).write(
                    ((__p2).read() | ((crate::c::shl_i32(1i32, ((sectorId) as u32))) as u32)),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>();
                (__p3).write(
                    ((__p3).read() & ((!(crate::c::shl_i32(1i32, ((sectorId) as u32)))) as u32)),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()
                    & ((crate::c::shl_i32(1i32, ((sectorId) as u32))) as u32))
                    != 0
                {
                    retVal = 1u32;
                }
                break 'l1;
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn WriteSaveSectorOrSlot(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut status: u32 = 0u32;
        let mut i: u16 = 0u16;
        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
            .write((&raw mut gSaveDataBuffer).cast::<u8>());
        if ((sectorId) as i32) != 65535i32 {
            status = ((HandleWriteSector(sectorId, locations)) as u32);
        } else {
            ((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>())
                .write(((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read());
            ((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>())
                .write(((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read());
            let __p1 = (&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).write(
                ((crate::c::rem_i32(
                    ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
                    14i32,
                )) as u16),
            );
            let __p2 = (&raw mut gSaveCounter).cast::<u8>().cast::<u32>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            status = 1u32;
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        HandleWriteSector(i, locations);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
                status = 255u32;
                ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                    .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
                ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                    .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
            }
        }
        return ((status) as u8);
    }
}
pub(crate) unsafe extern "C" fn HandleWriteSector(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut i: u16 = 0u16;
        let mut sector: u16 = 0u16;
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut size: u16 = 0u16;
        sector = ((((sectorId) as i32).wrapping_add(
            ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
        )) as u16);
        sector = ((crate::c::rem_i32(((sector) as i32), 14i32)) as u16);
        sector = ((((sector) as u32).wrapping_add((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        )))) as u16);
        data = (((locations).wrapping_offset(((sectorId) as i32) as isize * 8)).cast::<*mut u8>())
            .read();
        size = (((locations).wrapping_offset(((sectorId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
        .read();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4096i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4084)
            .cast::<u16>())
        .write(sectorId);
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4088)
            .cast::<u32>())
        .write(134291493u32);
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4092)
            .cast::<u32>())
        .write(((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read());
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((size) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((data).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4086)
            .cast::<u16>())
        .write(CalculateChecksum(data, size));
        return TryWriteSector(
            ((sector) as u8),
            (((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn HandleWriteSectorNBytes(
    sectorId: u8,
    data: *mut u8,
    size: u16,
) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut data = data;
        let mut size = size;
        let mut i: u16 = 0u16;
        let mut sector: *mut u8 = (&raw mut gSaveDataBuffer).cast::<u8>();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4096i32) {
                    break 'l1;
                }
                'l2: {
                    ((sector).wrapping_offset(((i) as i32) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((sector).wrapping_add(4088).cast::<u32>()).write(134291493u32);
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((size) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((sector).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(((data).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((sector).wrapping_add(4084).cast::<u16>()).write(CalculateChecksum(data, size));
        return TryWriteSector(sectorId, (sector).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn TryWriteSector(sector: u8, data: *mut u8) -> u8 {
    unsafe {
        let mut sector = sector;
        let mut data = data;
        if (ProgramFlashSectorAndVerify(((sector) as u16), data)) != 0 {
            SetDamagedSectorBits(0u8, sector);
            return 255u8;
        } else {
            SetDamagedSectorBits(1u8, sector);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreSaveBackupVarsAndIncrement(locations: *mut u8) -> u32 {
    unsafe {
        let mut locations = locations;
        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
            .write((&raw mut gSaveDataBuffer).cast::<u8>());
        ((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>())
            .write(((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read());
        ((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>())
            .write(((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read());
        let __p1 = (&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>();
        (__p2).write(((crate::c::rem_i32((((__p2).read()) as i32), 14i32)) as u16));
        let __p3 = (&raw mut gSaveCounter).cast::<u8>().cast::<u32>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        ((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).write(0u32);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn RestoreSaveBackupVars(locations: *mut u8) -> u32 {
    unsafe {
        let mut locations = locations;
        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
            .write((&raw mut gSaveDataBuffer).cast::<u8>());
        ((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>())
            .write(((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read());
        ((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>())
            .write(((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read());
        ((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).write(0u32);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleWriteIncrementalSector(
    numSectors: u16,
    locations: *mut u8,
) -> u8 {
    unsafe {
        let mut numSectors = numSectors;
        let mut locations = locations;
        let mut status: u8 = 0u8;
        if ((((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).read()) as i32)
            < ((numSectors) as i32).wrapping_sub(1i32)
        {
            status = 1u8;
            HandleWriteSector(
                ((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).read(),
                locations,
            );
            let __p1 = (&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
                status = 255u8;
                ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                    .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
                ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                    .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
            }
        } else {
            status = 255u8;
        }
        return status;
    }
}
pub(crate) unsafe extern "C" fn HandleReplaceSectorAndVerify(
    sectorId: u16,
    locations: *mut u8,
) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut status: u8 = 1u8;
        HandleReplaceSector(((((sectorId) as i32).wrapping_sub(1i32)) as u16), locations);
        if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
            status = 255u8;
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
        }
        return status;
    }
}
pub(crate) unsafe extern "C" fn HandleReplaceSector(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut i: u16 = 0u16;
        let mut sector: u16 = 0u16;
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut size: u16 = 0u16;
        let mut status: u8 = 0u8;
        sector = ((((sectorId) as i32).wrapping_add(
            ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
        )) as u16);
        sector = ((crate::c::rem_i32(((sector) as i32), 14i32)) as u16);
        sector = ((((sector) as u32).wrapping_add((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        )))) as u16);
        data = (((locations).wrapping_offset(((sectorId) as i32) as isize * 8)).cast::<*mut u8>())
            .read();
        size = (((locations).wrapping_offset(((sectorId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
        .read();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4096i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4084)
            .cast::<u16>())
        .write(sectorId);
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4088)
            .cast::<u32>())
        .write(134291493u32);
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4092)
            .cast::<u32>())
        .write(((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read());
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((size) as i32)) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(((data).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4086)
            .cast::<u16>())
        .write(CalculateChecksum(data, size));
        (((&raw mut EraseFlashSector).cast::<Option<unsafe extern "C" fn(u16) -> u16>>()).read())
            .unwrap_unchecked()(sector);
        status = 1u8;
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as u32) < 4088u32) {
                    break 'l5;
                }
                'l6: {
                    if ((((&raw mut ProgramFlashByte)
                        .cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
                    .read())
                    .unwrap_unchecked()(
                        sector,
                        ((i) as u32),
                        ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    )) != 0
                    {
                        status = 255u8;
                        break 'l5;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((status) as i32) == 255i32 {
            SetDamagedSectorBits(0u8, ((sector) as u8));
            return 255u8;
        } else {
            status = 1u8;
            {
                i = 0u16;
                'l7: loop {
                    if !(((i) as u32) < 7u32) {
                        break 'l7;
                    }
                    'l8: {
                        if ((((&raw mut ProgramFlashByte)
                            .cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
                        .read())
                        .unwrap_unchecked()(
                            sector,
                            (4089u32).wrapping_add(((i) as u32)),
                            ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((4089u32).wrapping_add(((i) as u32))) as i32) as isize,
                                ))
                            .read(),
                        )) != 0
                        {
                            status = 255u8;
                            break 'l7;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((status) as i32) == 255i32 {
                SetDamagedSectorBits(0u8, ((sector) as u8));
                return 255u8;
            } else {
                SetDamagedSectorBits(1u8, ((sector) as u8));
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn WriteSectorSignatureByte_NoOffset(
    sectorId: u16,
    locations: *mut u8,
) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut sector: u16 = ((((sectorId) as i32).wrapping_add(
            ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
        )) as u16);
        sector = ((crate::c::rem_i32(((sector) as i32), 14i32)) as u16);
        sector = ((((sector) as u32).wrapping_add((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        )))) as u16);
        if ((((&raw mut ProgramFlashByte)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(sector, 4088u32, 37u8))
            != 0
        {
            SetDamagedSectorBits(0u8, ((sector) as u8));
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
            return 255u8;
        } else {
            SetDamagedSectorBits(1u8, ((sector) as u8));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CopySectorSignatureByte(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut sector: u16 = (((((sectorId) as i32).wrapping_add(
            ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
        ))
        .wrapping_sub(1i32)) as u16);
        sector = ((crate::c::rem_i32(((sector) as i32), 14i32)) as u16);
        sector = ((((sector) as u32).wrapping_add((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        )))) as u16);
        if ((((&raw mut ProgramFlashByte)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(
            sector,
            4088u32,
            ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(4088))
            .read(),
        )) != 0
        {
            SetDamagedSectorBits(0u8, ((sector) as u8));
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
            return 255u8;
        } else {
            SetDamagedSectorBits(1u8, ((sector) as u8));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn WriteSectorSignatureByte(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut sector: u16 = (((((sectorId) as i32).wrapping_add(
            ((((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).read()) as i32),
        ))
        .wrapping_sub(1i32)) as u16);
        sector = ((crate::c::rem_i32(((sector) as i32), 14i32)) as u16);
        sector = ((((sector) as u32).wrapping_add((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        )))) as u16);
        if ((((&raw mut ProgramFlashByte)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(sector, 4088u32, 37u8))
            != 0
        {
            SetDamagedSectorBits(0u8, ((sector) as u8));
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>())
                .write(((&raw mut gLastKnownGoodSector).cast::<u8>().cast::<u16>()).read());
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>())
                .write(((&raw mut gLastSaveCounter).cast::<u8>().cast::<u32>()).read());
            return 255u8;
        } else {
            SetDamagedSectorBits(1u8, ((sector) as u8));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn TryLoadSaveSlot(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut status: u8 = 0u8;
        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
            .write((&raw mut gSaveDataBuffer).cast::<u8>());
        if ((sectorId) as i32) != 65535i32 {
            status = 255u8;
        } else {
            status = GetSaveValidStatus(locations);
            CopySaveSlotData(65535u16, locations);
        }
        return status;
    }
}
pub(crate) unsafe extern "C" fn CopySaveSlotData(sectorId: u16, locations: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut locations = locations;
        let mut i: u16 = 0u16;
        let mut checksum: u16 = 0u16;
        let mut slotOffset: u16 = (((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        ))) as u16);
        let mut id: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    ReadFlashSector(
                        ((((i) as i32).wrapping_add(((slotOffset) as i32))) as u8),
                        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    id = ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4084)
                        .cast::<u16>())
                    .read();
                    if ((id) as i32) == 0i32 {
                        ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).write(i);
                    }
                    checksum = CalculateChecksum(
                        (((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>(),
                        (((locations).wrapping_offset(((id) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read(),
                    );
                    if (((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4088)
                        .cast::<u32>())
                    .read()
                        == 134291493u32)
                        && (((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4086)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((checksum) as i32))
                    {
                        let mut j: u16 = 0u16;
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as i32)
                                    < (((((locations).wrapping_offset(((id) as i32) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u16>())
                                    .read()) as i32))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    (((((locations).wrapping_offset(((id) as i32) as isize * 8))
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        (((((&raw mut gReadWriteSector)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetSaveValidStatus(locations: *mut u8) -> u8 {
    unsafe {
        let mut locations = locations;
        let mut i: u16 = 0u16;
        let mut checksum: u16 = 0u16;
        let mut saveSlot1Counter: u32 = 0u32;
        let mut saveSlot2Counter: u32 = 0u32;
        let mut validSectorFlags: u32 = 0u32;
        let mut signatureValid: u8 = 0u8;
        let mut saveSlot1Status: u8 = 0u8;
        let mut saveSlot2Status: u8 = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    ReadFlashSector(
                        ((i) as u8),
                        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    if ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4088)
                        .cast::<u32>())
                    .read()
                        == 134291493u32
                    {
                        signatureValid = 1u8;
                        checksum = CalculateChecksum(
                            (((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>(),
                            (((locations).wrapping_offset(
                                ((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4084)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read(),
                        );
                        if ((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4086)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((checksum) as i32)
                        {
                            saveSlot1Counter =
                                ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4092)
                                .cast::<u32>())
                                .read();
                            validSectorFlags = (validSectorFlags
                                | ((crate::c::shl_i32(
                                    1i32,
                                    ((((((&raw mut gReadWriteSector)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4084)
                                    .cast::<u16>())
                                    .read()) as u32),
                                )) as u32));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (signatureValid) != 0 {
            if validSectorFlags == 16383u32 {
                saveSlot1Status = 1u8;
            } else {
                saveSlot1Status = 255u8;
            }
        } else {
            saveSlot1Status = 0u8;
        }
        validSectorFlags = 0u32;
        signatureValid = 0u8;
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l3;
                }
                'l4: {
                    ReadFlashSector(
                        ((((i) as i32).wrapping_add(14i32)) as u8),
                        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    if ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4088)
                        .cast::<u32>())
                    .read()
                        == 134291493u32
                    {
                        signatureValid = 1u8;
                        checksum = CalculateChecksum(
                            (((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>(),
                            (((locations).wrapping_offset(
                                ((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4084)
                                .cast::<u16>())
                                .read()) as i32) as isize
                                    * 8,
                            ))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read(),
                        );
                        if ((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4086)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((checksum) as i32)
                        {
                            saveSlot2Counter =
                                ((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4092)
                                .cast::<u32>())
                                .read();
                            validSectorFlags = (validSectorFlags
                                | ((crate::c::shl_i32(
                                    1i32,
                                    ((((((&raw mut gReadWriteSector)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4084)
                                    .cast::<u16>())
                                    .read()) as u32),
                                )) as u32));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (signatureValid) != 0 {
            if validSectorFlags == 16383u32 {
                saveSlot2Status = 1u8;
            } else {
                saveSlot2Status = 255u8;
            }
        } else {
            saveSlot2Status = 0u8;
        }
        if (((saveSlot1Status) as i32) == 1i32) && (((saveSlot2Status) as i32) == 1i32) {
            if ((saveSlot1Counter == 4294967295u32) && (saveSlot2Counter == 0u32))
                || ((saveSlot1Counter == 0u32) && (saveSlot2Counter == 4294967295u32))
            {
                if (saveSlot1Counter).wrapping_add(1u32) < (saveSlot2Counter).wrapping_add(1u32) {
                    ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot2Counter);
                } else {
                    ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot1Counter);
                }
            } else {
                if saveSlot1Counter < saveSlot2Counter {
                    ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot2Counter);
                } else {
                    ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot1Counter);
                }
            }
            return 1u8;
        }
        if ((saveSlot1Status) as i32) == 1i32 {
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot1Counter);
            if ((saveSlot2Status) as i32) == 255i32 {
                return 255u8;
            }
            return 1u8;
        }
        if ((saveSlot2Status) as i32) == 1i32 {
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(saveSlot2Counter);
            if ((saveSlot1Status) as i32) == 255i32 {
                return 255u8;
            }
            return 1u8;
        }
        if (((saveSlot1Status) as i32) == 0i32) && (((saveSlot2Status) as i32) == 0i32) {
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(0u32);
            ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).write(0u16);
            return 0u8;
        }
        ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).write(0u32);
        ((&raw mut gLastWrittenSector).cast::<u8>().cast::<u16>()).write(0u16);
        return 2u8;
    }
}
pub(crate) unsafe extern "C" fn TryLoadSaveSector(sectorId: u8, data: *mut u8, size: u16) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut data = data;
        let mut size = size;
        let mut i: u16 = 0u16;
        let mut sector: *mut u8 = (&raw mut gSaveDataBuffer).cast::<u8>();
        ReadFlashSector(sectorId, sector);
        if ((sector).wrapping_add(4088).cast::<u32>()).read() == 134291493u32 {
            let mut checksum: u16 = CalculateChecksum((sector).cast::<u8>(), size);
            if ((((sector).wrapping_add(4084).cast::<u16>()).read()) as i32) == ((checksum) as i32)
            {
                {
                    i = 0u16;
                    'l1: loop {
                        if !(((i) as i32) < ((size) as i32)) {
                            break 'l1;
                        }
                        'l2: {
                            ((data).wrapping_offset(((i) as i32) as isize)).write(
                                (((sector).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                                    .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return 1u8;
            } else {
                return 2u8;
            }
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ReadFlashSector(sectorId: u8, sector: *mut u8) -> u8 {
    unsafe {
        let mut sectorId = sectorId;
        let mut sector = sector;
        ReadFlash(((sectorId) as u16), 0u32, (sector).cast::<u8>(), 4096u32);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CalculateChecksum(data: *mut u8, size: u16) -> u16 {
    unsafe {
        let mut data = data;
        let mut size = size;
        let mut i: u16 = 0u16;
        let mut checksum: u32 = 0u32;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(((size) as i32), 4i32)) {
                    break 'l1;
                }
                'l2: {
                    checksum = (checksum).wrapping_add(((data).cast::<u32>()).read());
                    data = (data).wrapping_offset(4);
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((checksum >> 16).wrapping_add(checksum)) as u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateSaveAddresses() {
    unsafe {
        let mut i: i32 = 0i32;
        (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 8))
        .cast::<*mut u8>())
        .write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_offset(
                (((((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((i) as isize * 4))
                .cast::<u16>())
                .read()) as i32) as isize
                    * 1,
            ),
        );
        (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 8))
        .wrapping_add(4)
        .cast::<u16>())
        .write(
            (((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((i) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        {
            i = 1i32;
            'l1: loop {
                if !(i <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_offset(
                            (((((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 1,
                        ),
                    );
                    (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(i <= 13i32) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        (((&raw mut gPokemonStoragePtr).cast::<*mut u8>()).read()).wrapping_offset(
                            (((((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 1,
                        ),
                    );
                    (((((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(
                        (((((&raw const sSaveSlotLayout).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSavingData(saveType: u8) -> u8 {
    unsafe {
        let mut saveType = saveType;
        let mut i: u8 = 0u8;
        let mut backupVar: *mut u32 =
            ((&raw mut gTrainerHillVBlankCounter).cast::<*mut u32>()).read();
        let mut tempAddr: *mut u8 = core::ptr::null_mut();
        ((&raw mut gTrainerHillVBlankCounter).cast::<*mut u32>()).write(core::ptr::null_mut());
        UpdateSaveAddresses();
        'l1: {
            let __sw1 = ((saveType) as i32);
            let __matched = __sw1 == 5i32
                || __sw1 == 3i32
                || __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 4i32;
            let mut __fall = false;
            if __sw1 == 5i32 {
                __fall = true;
                {
                    i = 28u8;
                    'l2: loop {
                        if !(((i) as i32) < 32i32) {
                            break 'l2;
                        }
                        'l3: {
                            (((&raw mut EraseFlashSector)
                                .cast::<Option<unsafe extern "C" fn(u16) -> u16>>())
                            .read())
                            .unwrap_unchecked()(((i) as u16));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if GetGameStat(10u8) < 999u32 {
                    IncrementGameStat(10u8);
                }
                CopyPartyAndObjectsToSave();
                WriteSaveSectorOrSlot(
                    65535u16,
                    ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                );
                tempAddr = (&raw mut gDecompressionBuffer).cast::<u8>();
                HandleWriteSectorNBytes(28u8, tempAddr, 3968u16);
                HandleWriteSectorNBytes(29u8, (tempAddr).wrapping_offset(3968), 3968u16);
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                __fall = true;
                CopyPartyAndObjectsToSave();
                WriteSaveSectorOrSlot(
                    65535u16,
                    ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                __fall = true;
                CopyPartyAndObjectsToSave();
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            HandleReplaceSector(
                                ((i) as u16),
                                ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l6: loop {
                        if !(((i) as i32) <= 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            WriteSectorSignatureByte_NoOffset(
                                ((i) as u16),
                                ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                {
                    i = 28u8;
                    'l8: loop {
                        if !(((i) as i32) < 32i32) {
                            break 'l8;
                        }
                        'l9: {
                            (((&raw mut EraseFlashSector)
                                .cast::<Option<unsafe extern "C" fn(u16) -> u16>>())
                            .read())
                            .unwrap_unchecked()(((i) as u16));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                CopyPartyAndObjectsToSave();
                WriteSaveSectorOrSlot(
                    65535u16,
                    ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                );
                break 'l1;
            }
        }
        ((&raw mut gTrainerHillVBlankCounter).cast::<*mut u32>()).write(backupVar);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySavingData(saveType: u8) -> u8 {
    unsafe {
        let mut saveType = saveType;
        if ((&raw mut gFlashMemoryPresent).cast::<u32>()).read() != 1u32 {
            ((&raw mut gSaveAttemptStatus).cast::<u8>().cast::<u16>()).write(255u16);
            return 255u8;
        }
        HandleSavingData(saveType);
        if !((((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0) {
            ((&raw mut gSaveAttemptStatus).cast::<u8>().cast::<u16>()).write(1u16);
            return 1u8;
        } else {
            DoSaveFailedScreen(saveType);
            ((&raw mut gSaveAttemptStatus).cast::<u8>().cast::<u16>()).write(255u16);
            return 255u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_Init() -> u8 {
    unsafe {
        if ((&raw mut gFlashMemoryPresent).cast::<u32>()).read() != 1u32 {
            return 1u8;
        }
        UpdateSaveAddresses();
        CopyPartyAndObjectsToSave();
        RestoreSaveBackupVarsAndIncrement(
            ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_WriteSector() -> u8 {
    unsafe {
        let mut status: u8 = HandleWriteIncrementalSector(
            14u16,
            ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
        );
        if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
            DoSaveFailedScreen(0u8);
        }
        if ((status) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_ReplaceLastSector() -> u8 {
    unsafe {
        HandleReplaceSectorAndVerify(
            14u16,
            ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
        );
        if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
            DoSaveFailedScreen(0u8);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkFullSave_SetLastSectorSignature() -> u8 {
    unsafe {
        CopySectorSignatureByte(
            14u16,
            ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
        );
        if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
            DoSaveFailedScreen(0u8);
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteSaveBlock2() -> u8 {
    unsafe {
        if ((&raw mut gFlashMemoryPresent).cast::<u32>()).read() != 1u32 {
            return 1u8;
        }
        UpdateSaveAddresses();
        CopyPartyAndObjectsToSave();
        RestoreSaveBackupVars(((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>());
        HandleReplaceSectorAndVerify(
            ((((((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).read()) as i32)
                .wrapping_add(1i32)) as u16),
            ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteSaveBlock1Sector() -> u8 {
    unsafe {
        let mut finished: u8 = 0u8;
        let mut sectorId: u16 = {
            let __p1 = (&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        };
        if ((sectorId) as i32) <= 4i32 {
            HandleReplaceSectorAndVerify(
                ((((((&raw mut gIncrementalSectorId).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(1i32)) as u16),
                ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
            );
            WriteSectorSignatureByte(
                sectorId,
                ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
            );
        } else {
            WriteSectorSignatureByte(
                sectorId,
                ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
            );
            finished = 1u8;
        }
        if (((&raw mut gDamagedSaveSectors).cast::<u8>().cast::<u32>()).read()) != 0 {
            DoSaveFailedScreen(1u8);
        }
        return finished;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadGameSave(saveType: u8) -> u8 {
    unsafe {
        let mut saveType = saveType;
        let mut status: u8 = 0u8;
        if ((&raw mut gFlashMemoryPresent).cast::<u32>()).read() != 1u32 {
            ((&raw mut gSaveFileStatus).cast::<u8>().cast::<u16>()).write(4u16);
            return 255u8;
        }
        UpdateSaveAddresses();
        'l1: {
            let __sw1 = ((saveType) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                status = TryLoadSaveSlot(
                    65535u16,
                    ((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>(),
                );
                CopyPartyAndObjectsFromSave();
                ((&raw mut gSaveFileStatus).cast::<u8>().cast::<u16>()).write(((status) as u16));
                ((&raw mut gGameContinueCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(None);
                break 'l1;
            }
            if __sw1 == 3i32 {
                status =
                    TryLoadSaveSector(28u8, (&raw mut gDecompressionBuffer).cast::<u8>(), 3968u16);
                if ((status) as i32) == 1i32 {
                    status = TryLoadSaveSector(
                        29u8,
                        ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(3968),
                        3968u16,
                    );
                }
                break 'l1;
            }
        }
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSaveBlocksPointersBaseOffset() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut slotOffset: u16 = 0u16;
        let mut sector: *mut u8 = core::ptr::null_mut();
        sector = {
            let __v1 = (&raw mut gSaveDataBuffer).cast::<u8>();
            ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).write(__v1);
            __v1
        };
        if ((&raw mut gFlashMemoryPresent).cast::<u32>()).read() != 1u32 {
            return 0u16;
        }
        UpdateSaveAddresses();
        GetSaveValidStatus(((&raw mut gRamSaveSectorLocations).cast::<u8>()).cast::<u8>());
        slotOffset = (((14u32).wrapping_mul(crate::c::rem_u32(
            ((&raw mut gSaveCounter).cast::<u8>().cast::<u32>()).read(),
            2u32,
        ))) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 14i32) {
                    break 'l1;
                }
                'l2: {
                    ReadFlashSector(
                        ((((i) as i32).wrapping_add(((slotOffset) as i32))) as u8),
                        ((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    if ((((((&raw mut gReadWriteSector).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4084)
                        .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        return (((((((((sector).cast::<u8>()).wrapping_offset(10)).read())
                            as i32)
                            .wrapping_add(
                                (((((sector).cast::<u8>()).wrapping_offset(11)).read()) as i32),
                            ))
                        .wrapping_add(
                            (((((sector).cast::<u8>()).wrapping_offset(12)).read()) as i32),
                        ))
                        .wrapping_add(
                            (((((sector).cast::<u8>()).wrapping_offset(13)).read()) as i32),
                        )) as u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryReadSpecialSaveSector(sector: u8, dst: *mut u8) -> u32 {
    unsafe {
        let mut sector = sector;
        let mut dst = dst;
        let mut i: i32 = 0i32;
        let mut size: i32 = 0i32;
        let mut savData: *mut u8 = core::ptr::null_mut();
        if (((sector) as i32) != 30i32) && (((sector) as i32) != 31i32) {
            return 255u32;
        }
        ReadFlash(
            ((sector) as u16),
            0u32,
            (&raw mut gSaveDataBuffer).cast::<u8>(),
            4096u32,
        );
        if ((((&raw mut gSaveDataBuffer).cast::<u8>()).cast::<u8>()).cast::<u32>()).read()
            != 45981u32
        {
            return 255u32;
        }
        i = 0i32;
        size = 4091i32;
        savData = (((&raw mut gSaveDataBuffer).cast::<u8>()).cast::<u8>()).wrapping_offset(4);
        {
            'l1: loop {
                if !(i <= size) {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset((i) as isize))
                        .write(((savData).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryWriteSpecialSaveSector(sector: u8, src: *mut u8) -> u32 {
    unsafe {
        let mut sector = sector;
        let mut src = src;
        let mut i: i32 = 0i32;
        let mut size: i32 = 0i32;
        let mut savData: *mut u8 = core::ptr::null_mut();
        let mut savDataBuffer: *mut u8 = core::ptr::null_mut();
        if (((sector) as i32) != 30i32) && (((sector) as i32) != 31i32) {
            return 255u32;
        }
        savDataBuffer = (&raw mut gSaveDataBuffer).cast::<u8>();
        ((savDataBuffer).cast::<u32>()).write(45981u32);
        i = 0i32;
        size = 4091i32;
        savData = (((&raw mut gSaveDataBuffer).cast::<u8>()).cast::<u8>()).wrapping_offset(4);
        {
            'l1: loop {
                if !(i <= size) {
                    break 'l1;
                }
                'l2: {
                    ((savData).wrapping_offset((i) as isize))
                        .write(((src).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        if ProgramFlashSectorAndVerify(((sector) as u16), savDataBuffer) != 0u32 {
            return 255u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkFullSave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSoftResetDisabled).cast::<u8>()).write(1u8);
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetLinkStandbyCallback();
                (data).write(2i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if !((((data).wrapping_offset(2)).read()) != 0) {
                        SaveMapView();
                    }
                    (data).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((((data).wrapping_offset(2)).read()) != 0) {
                    SetContinueGameWarpStatusToDynamicWarp();
                }
                LinkFullSave_Init();
                (data).write(4i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p2 = (data).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 5i32
                {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(5i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (LinkFullSave_WriteSector()) != 0 {
                    (data).write(6i16);
                } else {
                    (data).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                LinkFullSave_ReplaceLastSector();
                (data).write(7i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((((data).wrapping_offset(2)).read()) != 0) {
                    ClearContinueGameWarpStatus2();
                }
                SetLinkStandbyCallback();
                (data).write(8i16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (IsLinkTaskFinished()) != 0 {
                    LinkFullSave_SetLastSectorSignature();
                    (data).write(9i16);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetLinkStandbyCallback();
                (data).write(10i16);
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (IsLinkTaskFinished()) != 0 {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (({
                    let __p4 = (data).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 5i32
                {
                    ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
