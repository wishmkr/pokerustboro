//! Translated from `src/fieldmap.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDummyConnectionFlags
#[allow(unused_imports)]
use crate::data::fieldmap::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBackupMapData: crate::ffi::Align4<[u8; 20480]> =
    crate::ffi::Align4([0; 20480]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMapHeader: crate::ffi::Align4<[u8; 28]> = crate::ffi::Align4([0; 28]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCamera: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapConnectionFlags: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBackupMapLayout: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);

unsafe extern "C" {
    static mut gDirectionToVectors: u8;
    static mut gSaveBlock1Ptr: u8;
    fn ClearMirageTowerPulseBlendEffect();
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn FixLongGrassMetatilesWindowBottom(a0: i16, a1: i16);
    fn FixLongGrassMetatilesWindowTop(a0: i16, a1: i16);
    fn GenerateBattlePyramidFloorLayout(a0: *mut u16, a1: u8);
    fn GenerateTrainerHillFloorLayout(a0: *mut u16);
    fn InitSecretBaseAppearance(a0: u8);
    fn IsLargeBreakableDecoration(a0: u16, a1: u8) -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadMapFromCameraTransition(a0: u8, a1: u8);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn RunOnLoadMapScript();
    fn SetOccupiedSecretBaseEntranceMetatiles(a0: *mut u8);
    fn UpdateTVScreensOnMap(a0: i32, a1: i32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapHeaderFromConnection(connection: *mut u8) -> *mut u8 {
    unsafe {
        let mut connection = connection;
        return Overworld_GetMapHeaderByGroupAndId(
            ((((connection).wrapping_add(8)).read()) as u16),
            ((((connection).wrapping_add(9)).read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMap() {
    unsafe {
        InitMapLayoutData((&raw mut gMapHeader).cast::<u8>());
        SetOccupiedSecretBaseEntranceMetatiles(
            (((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        RunOnLoadMapScript();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMapFromSavedGame() {
    unsafe {
        InitMapLayoutData((&raw mut gMapHeader).cast::<u8>());
        InitSecretBaseAppearance(0u8);
        SetOccupiedSecretBaseEntranceMetatiles(
            (((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        LoadSavedMapView();
        RunOnLoadMapScript();
        UpdateTVScreensOnMap(
            (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
            (((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattlePyramidMap(setPlayerPosition: u8) {
    unsafe {
        let mut setPlayerPosition = setPlayerPosition;
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(67044351u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>())
                            .cast::<u8>(),
                        (16777216u32
                            | (crate::c::div_u32(
                                20480u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        GenerateBattlePyramidFloorLayout(
            ((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>(),
            setPlayerPosition,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTrainerHillMap() {
    unsafe {
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(67044351u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>())
                            .cast::<u8>(),
                        (16777216u32
                            | (crate::c::div_u32(
                                20480u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        GenerateTrainerHillFloorLayout(
            ((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitMapLayoutData(mapHeader: *mut u8) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut mapLayout: *mut u8 = ((mapHeader).cast::<*mut u8>()).read();
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(67044351u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>())
                            .cast::<u8>(),
                        (16777216u32
                            | (crate::c::div_u32(
                                20480u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .write(((&raw mut sBackupMapData).cast::<u8>().cast::<u16>()).cast::<u16>());
        (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>())
            .write((((mapLayout).cast::<i32>()).read()).wrapping_add(15i32));
        (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(4)
            .cast::<i32>())
        .write((((mapLayout).wrapping_add(4).cast::<i32>()).read()).wrapping_add(14i32));
        if ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()).wrapping_mul(
            (((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .read(),
        ) > 10240i32
        {
            return;
        }
        InitBackupMapLayoutData(
            ((mapLayout).wrapping_add(12).cast::<*mut u16>()).read(),
            ((((mapLayout).cast::<i32>()).read()) as u16),
            ((((mapLayout).wrapping_add(4).cast::<i32>()).read()) as u16),
        );
        InitBackupMapLayoutConnections(mapHeader);
    }
}
pub(crate) unsafe extern "C" fn InitBackupMapLayoutData(map: *mut u16, width: u16, height: u16) {
    unsafe {
        let mut map = map;
        let mut width = width;
        let mut height = height;
        let mut dest: *mut u16 = core::ptr::null_mut();
        let mut y: i32 = 0i32;
        dest = (((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .read();
        dest = (dest).wrapping_offset(
            ((((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                .wrapping_mul(7i32))
            .wrapping_add(7i32)) as isize,
        );
        {
            y = 0i32;
            'l1: loop {
                if !(y < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        (map).cast::<u8>(),
                                        (dest).cast::<u8>(),
                                        ((0i32
                                            | (crate::c::div_i32(
                                                ((width) as i32).wrapping_mul(2i32),
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    dest = (dest).wrapping_offset((((width) as i32).wrapping_add(15i32)) as isize);
                    map = (map).wrapping_offset(((width) as i32) as isize);
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitBackupMapLayoutConnections(mapHeader: *mut u8) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut count: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut offset: i32 = 0i32;
        let mut connection: *mut u8 = core::ptr::null_mut();
        let mut cMap: *mut u8 = core::ptr::null_mut();
        if !(!(((mapHeader).wrapping_add(12).cast::<*mut u8>()).read()).is_null()) {
            return;
        }
        count = ((((mapHeader).wrapping_add(12).cast::<*mut u8>()).read()).cast::<i32>()).read();
        connection = ((((mapHeader).wrapping_add(12).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        (&raw mut sMapConnectionFlags)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw const sDummyConnectionFlags)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    cMap = GetMapHeaderFromConnection(connection);
                    offset = ((connection).wrapping_add(4).cast::<i32>()).read();
                    'l3: {
                        let __sw1 = (((connection).read()) as i32);
                        if __sw1 == 1i32 {
                            FillSouthConnection(mapHeader, cMap, offset);
                            crate::c::bf_write(
                                ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                                0,
                                1,
                                (1u8) as i32,
                            );
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            FillNorthConnection(mapHeader, cMap, offset);
                            crate::c::bf_write(
                                ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                                1,
                                1,
                                (1u8) as i32,
                            );
                            break 'l3;
                        }
                        if __sw1 == 3i32 {
                            FillWestConnection(mapHeader, cMap, offset);
                            crate::c::bf_write(
                                ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                                2,
                                1,
                                (1u8) as i32,
                            );
                            break 'l3;
                        }
                        if __sw1 == 4i32 {
                            FillEastConnection(mapHeader, cMap, offset);
                            crate::c::bf_write(
                                ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                                3,
                                1,
                                (1u8) as i32,
                            );
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
                connection = (connection).wrapping_offset(12);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillConnection(
    x: i32,
    y: i32,
    connectedMapHeader: *mut u8,
    x2: i32,
    y2: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut connectedMapHeader = connectedMapHeader;
        let mut x2 = x2;
        let mut y2 = y2;
        let mut width = width;
        let mut height = height;
        let mut i: i32 = 0i32;
        let mut src: *mut u16 = core::ptr::null_mut();
        let mut dest: *mut u16 = core::ptr::null_mut();
        let mut mapWidth: i32 = 0i32;
        mapWidth = ((((connectedMapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read();
        src = (((((connectedMapHeader).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u16>())
        .read())
        .wrapping_offset((((mapWidth).wrapping_mul(y2)).wrapping_add(x2)) as isize);
        dest = ((((&raw mut gBackupMapLayout).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .read())
        .wrapping_offset(
            ((((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()).wrapping_mul(y))
                .wrapping_add(x)) as isize,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < height) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        (src).cast::<u8>(),
                                        (dest).cast::<u8>(),
                                        ((0i32
                                            | (crate::c::div_i32(
                                                (width).wrapping_mul(2i32),
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    dest = (dest).wrapping_offset(
                        ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                            as isize,
                    );
                    src = (src).wrapping_offset((mapWidth) as isize);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillSouthConnection(
    mapHeader: *mut u8,
    connectedMapHeader: *mut u8,
    offset: i32,
) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut connectedMapHeader = connectedMapHeader;
        let mut offset = offset;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut x2: i32 = 0i32;
        let mut width: i32 = 0i32;
        let mut cWidth: i32 = 0i32;
        if !(!(connectedMapHeader).is_null()) {
            return;
        }
        cWidth = ((((connectedMapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read();
        x = (offset).wrapping_add(7i32);
        y = (((((mapHeader).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<i32>())
        .read())
        .wrapping_add(7i32);
        if x < 0i32 {
            x2 = (x).wrapping_neg();
            x = (x).wrapping_add(cWidth);
            if x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read() {
                width = x;
            } else {
                width = (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read();
            }
            x = 0i32;
        } else {
            x2 = 0i32;
            if (x).wrapping_add(cWidth)
                < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()
            {
                width = cWidth;
            } else {
                width = ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                    .wrapping_sub(x);
            }
        }
        FillConnection(x, y, connectedMapHeader, x2, 0i32, width, 7i32);
    }
}
pub(crate) unsafe extern "C" fn FillNorthConnection(
    mapHeader: *mut u8,
    connectedMapHeader: *mut u8,
    offset: i32,
) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut connectedMapHeader = connectedMapHeader;
        let mut offset = offset;
        let mut x: i32 = 0i32;
        let mut x2: i32 = 0i32;
        let mut y2: i32 = 0i32;
        let mut width: i32 = 0i32;
        let mut cWidth: i32 = 0i32;
        let mut cHeight: i32 = 0i32;
        if !(!(connectedMapHeader).is_null()) {
            return;
        }
        cWidth = ((((connectedMapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read();
        cHeight = ((((connectedMapHeader).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<i32>())
        .read();
        x = (offset).wrapping_add(7i32);
        y2 = (cHeight).wrapping_sub(7i32);
        if x < 0i32 {
            x2 = (x).wrapping_neg();
            x = (x).wrapping_add(cWidth);
            if x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read() {
                width = x;
            } else {
                width = (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read();
            }
            x = 0i32;
        } else {
            x2 = 0i32;
            if (x).wrapping_add(cWidth)
                < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()
            {
                width = cWidth;
            } else {
                width = ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                    .wrapping_sub(x);
            }
        }
        FillConnection(x, 0i32, connectedMapHeader, x2, y2, width, 7i32);
    }
}
pub(crate) unsafe extern "C" fn FillWestConnection(
    mapHeader: *mut u8,
    connectedMapHeader: *mut u8,
    offset: i32,
) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut connectedMapHeader = connectedMapHeader;
        let mut offset = offset;
        let mut y: i32 = 0i32;
        let mut x2: i32 = 0i32;
        let mut y2: i32 = 0i32;
        let mut height: i32 = 0i32;
        let mut cWidth: i32 = 0i32;
        let mut cHeight: i32 = 0i32;
        if !(!(connectedMapHeader).is_null()) {
            return;
        }
        cWidth = ((((connectedMapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read();
        cHeight = ((((connectedMapHeader).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<i32>())
        .read();
        y = (offset).wrapping_add(7i32);
        x2 = (cWidth).wrapping_sub(7i32);
        if y < 0i32 {
            y2 = (y).wrapping_neg();
            if (y).wrapping_add(cHeight)
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()
            {
                height = (y).wrapping_add(cHeight);
            } else {
                height = (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read();
            }
            y = 0i32;
        } else {
            y2 = 0i32;
            if (y).wrapping_add(cHeight)
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()
            {
                height = cHeight;
            } else {
                height = ((((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
                .wrapping_sub(y);
            }
        }
        FillConnection(0i32, y, connectedMapHeader, x2, y2, 7i32, height);
    }
}
pub(crate) unsafe extern "C" fn FillEastConnection(
    mapHeader: *mut u8,
    connectedMapHeader: *mut u8,
    offset: i32,
) {
    unsafe {
        let mut mapHeader = mapHeader;
        let mut connectedMapHeader = connectedMapHeader;
        let mut offset = offset;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut y2: i32 = 0i32;
        let mut height: i32 = 0i32;
        let mut cHeight: i32 = 0i32;
        if !(!(connectedMapHeader).is_null()) {
            return;
        }
        cHeight = ((((connectedMapHeader).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<i32>())
        .read();
        x = (((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read()).wrapping_add(7i32);
        y = (offset).wrapping_add(7i32);
        if y < 0i32 {
            y2 = (y).wrapping_neg();
            if (y).wrapping_add(cHeight)
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()
            {
                height = (y).wrapping_add(cHeight);
            } else {
                height = (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read();
            }
            y = 0i32;
        } else {
            y2 = 0i32;
            if (y).wrapping_add(cHeight)
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()
            {
                height = cHeight;
            } else {
                height = ((((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
                .wrapping_sub(y);
            }
        }
        FillConnection(x, y, connectedMapHeader, 0i32, y2, 8i32, height);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetElevationAt(x: i32, y: i32) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut block: u16 = ((if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            (((((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add(
                    ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                        .wrapping_mul(y),
                )) as isize,
            ))
            .read()) as i32)
        } else {
            ((((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((x).wrapping_add(1i32) & 1i32)
                    .wrapping_add((((y).wrapping_add(1i32) & 1i32) << 1))) as isize,
            ))
            .read()) as i32)
                | 3072i32)
        }) as u16);
        if ((block) as i32) == 1023i32 {
            return 0u8;
        }
        return (((((block) as i32) & 61440i32) >> 12) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetCollisionAt(x: i32, y: i32) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut block: u16 = ((if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            (((((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add(
                    ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                        .wrapping_mul(y),
                )) as isize,
            ))
            .read()) as i32)
        } else {
            ((((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((x).wrapping_add(1i32) & 1i32)
                    .wrapping_add((((y).wrapping_add(1i32) & 1i32) << 1))) as isize,
            ))
            .read()) as i32)
                | 3072i32)
        }) as u16);
        if ((block) as i32) == 1023i32 {
            return 1u8;
        }
        return (((((block) as i32) & 3072i32) >> 10) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileIdAt(x: i32, y: i32) -> i32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut block: i32 = (if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            (((((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add(
                    ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                        .wrapping_mul(y),
                )) as isize,
            ))
            .read()) as i32)
        } else {
            ((((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((x).wrapping_add(1i32) & 1i32)
                    .wrapping_add((((y).wrapping_add(1i32) & 1i32) << 1))) as isize,
            ))
            .read()) as i32)
                | 3072i32)
        });
        if block == 1023i32 {
            return ((((((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((x).wrapping_add(1i32) & 1i32)
                    .wrapping_add((((y).wrapping_add(1i32) & 1i32) << 1))) as isize,
            ))
            .read()) as i32)
                | 3072i32)
                & 1023i32)
                >> 0);
        }
        return ((block & 1023i32) >> 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileBehaviorAt(x: i32, y: i32) -> i32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return ((((GetMetatileAttributesById(((MapGridGetMetatileIdAt(x, y)) as u16))) as i32)
            & 255i32)
            >> 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileLayerTypeAt(x: i32, y: i32) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return (((((GetMetatileAttributesById(((MapGridGetMetatileIdAt(x, y)) as u16))) as i32)
            & 61440i32)
            >> 12) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileIdAt(x: i32, y: i32, metatile: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatile = metatile;
        if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            let __p1 = ((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add((y).wrapping_mul(
                    (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
                ))) as isize,
            );
            (__p1).write((((((__p1).read()) as i32) & 61440i32) as u16));
            let __p2 = ((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add((y).wrapping_mul(
                    (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
                ))) as isize,
            );
            (__p2).write((((((__p2).read()) as i32) | (((metatile) as i32) & (-61441i32))) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileEntryAt(x: i32, y: i32, metatile: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatile = metatile;
        if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            (((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add(
                    ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                        .wrapping_mul(y),
                )) as isize,
            ))
            .write(metatile);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMetatileAttributesById(metatile: u16) -> u16 {
    unsafe {
        let mut metatile = metatile;
        if ((metatile) as i32) < 512i32 {
            return (((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<*mut u16>())
            .read())
            .wrapping_offset(((metatile) as i32) as isize))
            .read();
        } else {
            if ((metatile) as i32) < 1024i32 {
                return (((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(16)
                .cast::<*mut u16>())
                .read())
                .wrapping_offset((((metatile) as i32).wrapping_sub(512i32)) as isize))
                .read();
            } else {
                return 255u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveMapView() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut mapView: *mut u16 = core::ptr::null_mut();
        let mut width: i32 = 0i32;
        mapView =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(52)).cast::<u16>();
        width = (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read();
        x = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
            as i32);
        y = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32);
        {
            i = y;
            'l1: loop {
                if !(i < (y).wrapping_add(14i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = x;
                        'l3: loop {
                            if !(j < (x).wrapping_add(15i32)) {
                                break 'l3;
                            }
                            'l4: {
                                ({
                                    let __t1 = mapView;
                                    mapView = (mapView).wrapping_offset(1);
                                    __t1
                                })
                                .write(
                                    ((((&raw mut sBackupMapData).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        (((width).wrapping_mul(i)).wrapping_add(j)) as isize,
                                    ))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SavedMapViewIsEmpty() -> u32 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut marker: u32 = 0u32;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(512u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    marker = (marker
                        | ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        if marker == 0u32 {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ClearSavedMapView() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(52))
                                .cast::<u16>())
                                .cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        512u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadSavedMapView() {
    unsafe {
        let mut yMode: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut mapView: *mut u16 = core::ptr::null_mut();
        let mut width: i32 = 0i32;
        mapView =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(52)).cast::<u16>();
        if (SavedMapViewIsEmpty()) != 0 {
            return;
        }
        width = (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read();
        x = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
            as i32);
        y = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32);
        {
            i = y;
            'l1: loop {
                if !(i < (y).wrapping_add(14i32)) {
                    break 'l1;
                }
                'l2: {
                    if (i == y) && (i != 0i32) {
                        yMode = 0u8;
                    } else {
                        if (i == ((y).wrapping_add(14i32)).wrapping_sub(1i32))
                            && (i
                                != ((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<i32>())
                                .read())
                                .wrapping_sub(1i32))
                        {
                            yMode = 1u8;
                        } else {
                            yMode = 255u8;
                        }
                    }
                    {
                        j = x;
                        'l3: loop {
                            if !(j < (x).wrapping_add(15i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if !((SkipCopyingMetatileFromSavedMap(
                                    (((&raw mut sBackupMapData).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((j).wrapping_add((width).wrapping_mul(i))) as isize,
                                    ),
                                    ((width) as u16),
                                    yMode,
                                )) != 0)
                                {
                                    ((((&raw mut sBackupMapData).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((j).wrapping_add((width).wrapping_mul(i))) as isize,
                                    ))
                                    .write((mapView).read());
                                }
                                mapView = (mapView).wrapping_offset(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = x;
            'l5: loop {
                if !(j < (x).wrapping_add(15i32)) {
                    break 'l5;
                }
                'l6: {
                    if y != 0i32 {
                        FixLongGrassMetatilesWindowTop(
                            ((j) as i16),
                            (((y).wrapping_sub(1i32)) as i16),
                        );
                    }
                    if i < ((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read())
                    .wrapping_sub(1i32)
                    {
                        FixLongGrassMetatilesWindowBottom(
                            ((j) as i16),
                            ((((y).wrapping_add(14i32)).wrapping_sub(1i32)) as i16),
                        );
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        ClearSavedMapView();
    }
}
pub(crate) unsafe extern "C" fn MoveMapViewToBackup(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut width: i32 = 0i32;
        let mut x0: i32 = 0i32;
        let mut y0: i32 = 0i32;
        let mut x2: i32 = 0i32;
        let mut y2: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut mapView: *mut u16 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(52)).cast::<u16>();
        width = (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read();
        i = 0i32;
        j = 0i32;
        x0 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
            as i32);
        y0 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32);
        x2 = 15i32;
        y2 = 14i32;
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 2i32 {
                y0 = (y0).wrapping_add(1);
                y2 = 13i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                j = 1i32;
                y2 = 13i32;
                break 'l1;
            }
            if __sw1 == 3i32 {
                x0 = (x0).wrapping_add(1);
                x2 = 14i32;
                break 'l1;
            }
            if __sw1 == 4i32 {
                i = 1i32;
                x2 = 14i32;
                break 'l1;
            }
        }
        {
            y = 0i32;
            'l2: loop {
                if !(y < y2) {
                    break 'l2;
                }
                'l3: {
                    {
                        x = 0i32;
                        'l4: loop {
                            if !(x < x2) {
                                break 'l4;
                            }
                            'l5: {
                                ((((&raw mut sBackupMapData).cast::<u8>().cast::<u16>())
                                    .cast::<u16>())
                                .wrapping_offset(
                                    (((x).wrapping_add(x0))
                                        .wrapping_add((width).wrapping_mul((y).wrapping_add(y0))))
                                        as isize,
                                ))
                                .write(
                                    ((mapView).wrapping_offset(
                                        (((i).wrapping_add(x)).wrapping_add(
                                            (15i32).wrapping_mul((j).wrapping_add(y)),
                                        )) as isize,
                                    ))
                                    .read(),
                                );
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
        ClearSavedMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapBorderIdAt(x: i32, y: i32) -> i32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            (((((((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                ((x).wrapping_add(
                    ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                        .wrapping_mul(y),
                )) as isize,
            ))
            .read()) as i32)
        } else {
            ((((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(
                (((x).wrapping_add(1i32) & 1i32)
                    .wrapping_add((((y).wrapping_add(1i32) & 1i32) << 1))) as isize,
            ))
            .read()) as i32)
                | 3072i32)
        }) == 1023i32
        {
            return (-1i32);
        }
        if x >= ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
            .wrapping_sub(8i32)
        {
            if !((crate::c::bf_read(
                ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                3,
                1,
                false,
            ) as u8)
                != 0)
            {
                return (-1i32);
            }
            return 4i32;
        } else {
            if x < 7i32 {
                if !((crate::c::bf_read(
                    ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    return (-1i32);
                }
                return 3i32;
            } else {
                if y >= ((((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
                .wrapping_sub(7i32)
                {
                    if !((crate::c::bf_read(
                        ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0)
                    {
                        return (-1i32);
                    }
                    return 1i32;
                } else {
                    if y < 7i32 {
                        if !((crate::c::bf_read(
                            ((&raw mut sMapConnectionFlags).cast::<u8>()).wrapping_add(0),
                            1,
                            1,
                            false,
                        ) as u8)
                            != 0)
                        {
                            return (-1i32);
                        }
                        return 2i32;
                    } else {
                        return 0i32;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPostCameraMoveMapBorderId(x: i32, y: i32) -> i32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return GetMapBorderIdAt(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(7i32))
            .wrapping_add(x),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(7i32))
            .wrapping_add(y),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanCameraMoveInDirection(direction: i32) -> u32 {
    unsafe {
        let mut direction = direction;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        x = ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
            as i32)
            .wrapping_add(7i32)) as u32)
            .wrapping_add(
                ((((&raw mut gDirectionToVectors).cast::<u8>())
                    .wrapping_offset((direction) as isize * 8))
                .cast::<u32>())
                .read(),
            )) as i32);
        y = ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(7i32)) as u32)
            .wrapping_add(
                ((((&raw mut gDirectionToVectors).cast::<u8>())
                    .wrapping_offset((direction) as isize * 8))
                .wrapping_add(4)
                .cast::<u32>())
                .read(),
            )) as i32);
        if GetMapBorderIdAt(x, y) == (-1i32) {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn SetPositionFromConnection(
    connection: *mut u8,
    direction: i32,
    x: i32,
    y: i32,
) {
    unsafe {
        let mut connection = connection;
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        mapHeader = GetMapHeaderFromConnection(connection);
        'l1: {
            let __sw1 = direction;
            if __sw1 == 4i32 {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                    .write((((x).wrapping_neg()) as i16));
                let __p2 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                        as i16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).write(
                    ((((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read()) as i16),
                );
                let __p3 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                        as i16),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32)
                        .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                        as i16),
                );
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .write((((y).wrapping_neg()) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p5 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32)
                        .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                        as i16),
                );
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .write(
                    ((((((mapHeader).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read()) as i16),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CameraMove(x: i32, y: i32) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut direction: i32 = 0i32;
        let mut connection: *mut u8 = core::ptr::null_mut();
        let mut old_x: i32 = 0i32;
        let mut old_y: i32 = 0i32;
        crate::c::bf_write(
            ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (0u8) as i32,
        );
        direction = GetPostCameraMoveMapBorderId(x, y);
        if (direction == 0i32) || (direction == (-1i32)) {
            let __p1 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(x)) as i16));
            let __p2 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(y)) as i16));
        } else {
            SaveMapView();
            ClearMirageTowerPulseBlendEffect();
            old_x = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32);
            old_y = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32);
            connection = GetIncomingConnection(
                ((direction) as u8),
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as i32),
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32),
            );
            SetPositionFromConnection(connection, direction, x, y);
            LoadMapFromCameraTransition(
                ((connection).wrapping_add(8)).read(),
                ((connection).wrapping_add(9)).read(),
            );
            crate::c::bf_write(
                ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
            (((&raw mut gCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .write((old_x).wrapping_sub(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as i32),
            ));
            (((&raw mut gCamera).cast::<u8>())
                .wrapping_add(8)
                .cast::<i32>())
            .write(
                (old_y).wrapping_sub(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32),
                ),
            );
            let __p3 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(x)) as i16));
            let __p4 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(y)) as i16));
            MoveMapViewToBackup(((direction) as u8));
        }
        return (crate::c::bf_read(
            ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
            0,
            1,
            false,
        ) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetIncomingConnection(direction: u8, x: i32, y: i32) -> *mut u8 {
    unsafe {
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut count: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut connection: *mut u8 = core::ptr::null_mut();
        let mut connections: *mut u8 = (((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read();
        if (((connections) as usize) == 0usize)
            || (((((connections).wrapping_add(4).cast::<*mut u8>()).read()) as usize) == 0usize)
        {
            return core::ptr::null_mut();
        }
        count = ((connections).cast::<i32>()).read();
        connection = ((connections).wrapping_add(4).cast::<*mut u8>()).read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    if ((((connection).read()) as i32) == ((direction) as i32))
                        && (((IsPosInIncomingConnectingMap(direction, x, y, connection)) as i32)
                            == 1i32)
                    {
                        return connection;
                    }
                }
                i = (i).wrapping_add(1);
                connection = (connection).wrapping_offset(12);
            }
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn IsPosInIncomingConnectingMap(
    direction: u8,
    x: i32,
    y: i32,
    connection: *mut u8,
) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut connection = connection;
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        mapHeader = GetMapHeaderFromConnection(connection);
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 || __sw1 == 2i32 {
                return IsCoordInIncomingConnectingMap(
                    x,
                    (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                        .cast::<i32>())
                    .read(),
                    ((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read(),
                    ((connection).wrapping_add(4).cast::<i32>()).read(),
                );
            }
            if __sw1 == 3i32 || __sw1 == 4i32 {
                return IsCoordInIncomingConnectingMap(
                    y,
                    (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                    ((((mapHeader).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                    ((connection).wrapping_add(4).cast::<i32>()).read(),
                );
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsCoordInIncomingConnectingMap(
    coord: i32,
    srcMax: i32,
    destMax: i32,
    offset: i32,
) -> u8 {
    unsafe {
        let mut coord = coord;
        let mut srcMax = srcMax;
        let mut destMax = destMax;
        let mut offset = offset;
        let mut min: i32 = 0i32;
        let mut max: i32 = 0i32;
        if offset < 0i32 {
            min = 0i32;
        } else {
            min = offset;
        }
        if (destMax).wrapping_add(offset) < srcMax {
            max = (destMax).wrapping_add(offset);
        } else {
            max = srcMax;
        }
        if (min <= coord) && (coord <= max) {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsCoordInConnectingMap(coord: i32, max: i32) -> i32 {
    unsafe {
        let mut coord = coord;
        let mut max = max;
        if (coord >= 0i32) && (coord < max) {
            return 1i32;
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn IsPosInConnectingMap(connection: *mut u8, x: i32, y: i32) -> i32 {
    unsafe {
        let mut connection = connection;
        let mut x = x;
        let mut y = y;
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        mapHeader = GetMapHeaderFromConnection(connection);
        'l1: {
            let __sw1 = (((connection).read()) as i32);
            if __sw1 == 1i32 || __sw1 == 2i32 {
                return IsCoordInConnectingMap(
                    (x).wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()),
                    ((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read(),
                );
            }
            if __sw1 == 3i32 || __sw1 == 4i32 {
                return IsCoordInConnectingMap(
                    (y).wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()),
                    ((((mapHeader).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                );
            }
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapConnectionAtPos(x: i16, y: i16) -> *mut u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut count: i32 = 0i32;
        let mut connection: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut direction: u8 = 0u8;
        if !(!((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .is_null())
        {
            return core::ptr::null_mut();
        }
        count = (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .cast::<i32>())
        .read();
        connection = (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<*mut u8>())
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    direction = (connection).read();
                    if (((direction) as i32) == 5i32) || (((direction) as i32) == 6i32) {
                        break 'l2;
                    } else {
                        if (((direction) as i32) == 2i32) && (((y) as i32) > 6i32) {
                            break 'l2;
                        } else {
                            if (((direction) as i32) == 1i32)
                                && (((y) as i32)
                                    < ((((((&raw mut gMapHeader).cast::<u8>())
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<i32>())
                                    .read())
                                    .wrapping_add(7i32))
                            {
                                break 'l2;
                            } else {
                                if (((direction) as i32) == 3i32) && (((x) as i32) > 6i32) {
                                    break 'l2;
                                } else {
                                    if (((direction) as i32) == 4i32)
                                        && (((x) as i32)
                                            < ((((((&raw mut gMapHeader).cast::<u8>())
                                                .cast::<*mut u8>())
                                            .read())
                                            .cast::<i32>())
                                            .read())
                                            .wrapping_add(7i32))
                                    {
                                        break 'l2;
                                    }
                                }
                            }
                        }
                    }
                    if IsPosInConnectingMap(
                        connection,
                        ((x) as i32).wrapping_sub(7i32),
                        ((y) as i32).wrapping_sub(7i32),
                    ) == 1i32
                    {
                        return connection;
                    }
                }
                i = (i).wrapping_add(1);
                connection = (connection).wrapping_offset(12);
            }
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraFocusCoords(x: u16, y: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
            .write(((((x) as i32).wrapping_sub(7i32)) as i16));
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .write(((((y) as i32).wrapping_sub(7i32)) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraFocusCoords(x: *mut u16, y: *mut u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(7i32)) as u16),
        );
        (y).write(
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(7i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn SetCameraCoords(x: u16, y: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).write(((x) as i16));
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .write(((y) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraCoords(x: *mut u16, y: *mut u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as u16),
        );
        (y).write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileImpassabilityAt(x: i32, y: i32, impassable: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut impassable = impassable;
        if (((x >= 0i32)
            && (x < (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read()))
            && (y >= 0i32))
            && (y
                < (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
        {
            if (impassable) != 0 {
                let __p1 = ((((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(
                    ((x).wrapping_add(
                        ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                            .wrapping_mul(y),
                    )) as isize,
                );
                (__p1).write((((((__p1).read()) as i32) | 3072i32) as u16));
            } else {
                let __p2 = ((((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<*mut u16>())
                .read())
                .wrapping_offset(
                    ((x).wrapping_add(
                        ((((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read())
                            .wrapping_mul(y),
                    )) as isize,
                );
                (__p2).write((((((__p2).read()) as i32) & (-3073i32)) as u16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SkipCopyingMetatileFromSavedMap(
    mapBlock: *mut u16,
    mapWidth: u16,
    yMode: u8,
) -> u8 {
    unsafe {
        let mut mapBlock = mapBlock;
        let mut mapWidth = mapWidth;
        let mut yMode = yMode;
        if ((yMode) as i32) == 255i32 {
            return 0u8;
        }
        if ((yMode) as i32) == 0i32 {
            mapBlock = (mapBlock).wrapping_offset((((mapWidth) as i32).wrapping_neg()) as isize);
        } else {
            mapBlock = (mapBlock).wrapping_offset(((mapWidth) as i32) as isize);
        }
        if ((IsLargeBreakableDecoration(
            ((((((mapBlock).read()) as i32) & 1023i32) >> 0) as u16),
            yMode,
        )) as i32)
            == 1i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CopyTilesetToVram(tileset: *mut u8, numTiles: u16, offset: u16) {
    unsafe {
        let mut tileset = tileset;
        let mut numTiles = numTiles;
        let mut offset = offset;
        if !(tileset).is_null() {
            if !(((tileset).read()) != 0) {
                LoadBgTiles(
                    2u8,
                    (((tileset).wrapping_add(4).cast::<*mut u32>()).read()).cast::<u8>(),
                    ((((numTiles) as i32).wrapping_mul(32i32)) as u16),
                    offset,
                );
            } else {
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((tileset).wrapping_add(4).cast::<*mut u32>()).read()).cast::<u8>(),
                    ((((numTiles) as i32).wrapping_mul(32i32)) as u32),
                    offset,
                    0u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CopyTilesetToVramUsingHeap(
    tileset: *mut u8,
    numTiles: u16,
    offset: u16,
) {
    unsafe {
        let mut tileset = tileset;
        let mut numTiles = numTiles;
        let mut offset = offset;
        if !(tileset).is_null() {
            if !(((tileset).read()) != 0) {
                LoadBgTiles(
                    2u8,
                    (((tileset).wrapping_add(4).cast::<*mut u32>()).read()).cast::<u8>(),
                    ((((numTiles) as i32).wrapping_mul(32i32)) as u16),
                    offset,
                );
            } else {
                DecompressAndLoadBgGfxUsingHeap(
                    2u8,
                    (((tileset).wrapping_add(4).cast::<*mut u32>()).read()).cast::<u8>(),
                    ((((numTiles) as i32).wrapping_mul(32i32)) as u32),
                    offset,
                    0u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyGlobalTintToPaletteEntries(offset: u16, size: u16) {
    unsafe {
        let mut offset = offset;
        let mut size = size;
    }
}
pub(crate) unsafe extern "C" fn ApplyGlobalTintToPaletteSlot(slot: u8, count: u8) {
    unsafe {
        let mut slot = slot;
        let mut count = count;
    }
}
pub(crate) unsafe extern "C" fn LoadTilesetPalette(tileset: *mut u8, destOffset: u16, size: u16) {
    unsafe {
        let mut tileset = tileset;
        let mut destOffset = destOffset;
        let mut size = size;
        let mut black: u16 = 0u16;
        if !(tileset).is_null() {
            if ((((tileset).wrapping_add(1)).read()) as i32) == 0i32 {
                LoadPalette((&raw mut black).cast::<u8>(), destOffset, 2u16);
                LoadPalette(
                    (((((tileset).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u16>())
                        .wrapping_offset(1))
                    .cast::<u8>(),
                    ((((destOffset) as i32).wrapping_add(1i32)) as u16),
                    ((((size) as u32).wrapping_sub(2u32)) as u16),
                );
                ApplyGlobalTintToPaletteEntries(
                    ((((destOffset) as i32).wrapping_add(1i32)) as u16),
                    ((((size) as u32).wrapping_sub(2u32) >> 1) as u16),
                );
            } else {
                if ((((tileset).wrapping_add(1)).read()) as i32) == 1i32 {
                    LoadPalette(
                        (((((tileset).wrapping_add(8).cast::<*mut u8>()).read())
                            .wrapping_offset(192))
                        .cast::<u16>())
                        .cast::<u8>(),
                        destOffset,
                        size,
                    );
                    ApplyGlobalTintToPaletteEntries(destOffset, ((((size) as i32) >> 1) as u16));
                } else {
                    LoadCompressedPalette(
                        (((tileset).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u32>(),
                        destOffset,
                        size,
                    );
                    ApplyGlobalTintToPaletteEntries(destOffset, ((((size) as i32) >> 1) as u16));
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPrimaryTilesetToVram(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        CopyTilesetToVram(
            ((mapLayout).wrapping_add(16).cast::<*mut u8>()).read(),
            512u16,
            0u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopySecondaryTilesetToVram(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        CopyTilesetToVram(
            ((mapLayout).wrapping_add(20).cast::<*mut u8>()).read(),
            512u16,
            512u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopySecondaryTilesetToVramUsingHeap(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        CopyTilesetToVramUsingHeap(
            ((mapLayout).wrapping_add(20).cast::<*mut u8>()).read(),
            512u16,
            512u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadPrimaryTilesetPalette(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        LoadTilesetPalette(
            ((mapLayout).wrapping_add(16).cast::<*mut u8>()).read(),
            0u16,
            192u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadSecondaryTilesetPalette(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        LoadTilesetPalette(
            ((mapLayout).wrapping_add(20).cast::<*mut u8>()).read(),
            96u16,
            224u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMapTilesetsToVram(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        if !(mapLayout).is_null() {
            CopyTilesetToVramUsingHeap(
                ((mapLayout).wrapping_add(16).cast::<*mut u8>()).read(),
                512u16,
                0u16,
            );
            CopyTilesetToVramUsingHeap(
                ((mapLayout).wrapping_add(20).cast::<*mut u8>()).read(),
                512u16,
                512u16,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMapTilesetPalettes(mapLayout: *mut u8) {
    unsafe {
        let mut mapLayout = mapLayout;
        if !(mapLayout).is_null() {
            LoadPrimaryTilesetPalette(mapLayout);
            LoadSecondaryTilesetPalette(mapLayout);
        }
    }
}
