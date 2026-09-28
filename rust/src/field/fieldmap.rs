//! Translated from `src/fieldmap.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sDummyConnectionFlags

/// `struct ConnectionFlags`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ConnectionFlags {
    bits_0: u8,
}

impl ConnectionFlags {
    #[inline(always)]
    pub fn south(&self) -> u8 {
        ((self.bits_0 as u32 >> 0) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_south(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 0)) | ((v as u8 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn north(&self) -> u8 {
        ((self.bits_0 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_north(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 1)) | ((v as u8 & 0x1) << 1);
    }
    #[inline(always)]
    pub fn west(&self) -> u8 {
        ((self.bits_0 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_west(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 2)) | ((v as u8 & 0x1) << 2);
    }
    #[inline(always)]
    pub fn east(&self) -> u8 {
        ((self.bits_0 as u32 >> 3) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_east(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 3)) | ((v as u8 & 0x1) << 3);
    }
}

unsafe impl Sync for ConnectionFlags {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ConnectionFlags>() == 4);
    assert!(offset_of!(ConnectionFlags, bits_0) == 0);
};

static sDummyConnectionFlags: Table<ConnectionFlags> =
    Table((&raw const crate::data::fieldmap::sDummyConnectionFlags).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBackupMapData: Aligned<CArray<u16, 10240>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMapHeader: MapHeader = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCamera: Camera = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapConnectionFlags: ConnectionFlags = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBackupMapLayout: BackupMapLayout = unsafe { zeroed() };

unsafe extern "C" {
    static gDirectionToVectors: CArray<UCoords32, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    fn ClearMirageTowerPulseBlendEffect();
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn FixLongGrassMetatilesWindowBottom(a0: i16, a1: i16);
    fn FixLongGrassMetatilesWindowTop(a0: i16, a1: i16);
    fn GenerateBattlePyramidFloorLayout(a0: *mut u16, a1: u8);
    fn GenerateTrainerHillFloorLayout(a0: *mut u16);
    fn InitSecretBaseAppearance(a0: u8);
    fn IsLargeBreakableDecoration(a0: u16, a1: u8) -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadMapFromCameraTransition(a0: u8, a1: u8);
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn RunOnLoadMapScript();
    fn SetOccupiedSecretBaseEntranceMetatiles(a0: *mut MapEvents);
    fn UpdateTVScreensOnMap(a0: i32, a1: i32);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapHeaderFromConnection(
    connection: *mut MapConnection,
) -> *mut MapHeader {
    return Overworld_GetMapHeaderByGroupAndId(
        (*connection).mapGroup as u16,
        (*connection).mapNum as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMap() {
    InitMapLayoutData(&raw mut gMapHeader);
    SetOccupiedSecretBaseEntranceMetatiles(gMapHeader.events);
    RunOnLoadMapScript();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMapFromSavedGame() {
    InitMapLayoutData(&raw mut gMapHeader);
    InitSecretBaseAppearance(FALSE);
    SetOccupiedSecretBaseEntranceMetatiles(gMapHeader.events);
    LoadSavedMapView();
    RunOnLoadMapScript();
    UpdateTVScreensOnMap(gBackupMapLayout.width, gBackupMapLayout.height);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattlePyramidMap(setPlayerPosition: u8) {
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0x3ff03ff);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            sBackupMapData.as_mut_ptr() as *mut c_void,
            0x1001400,
        );
    }
    GenerateBattlePyramidFloorLayout(sBackupMapData.as_mut_ptr(), setPlayerPosition);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTrainerHillMap() {
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0x3ff03ff);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            sBackupMapData.as_mut_ptr() as *mut c_void,
            0x1001400,
        );
    }
    GenerateTrainerHillFloorLayout(sBackupMapData.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn InitMapLayoutData(mapHeader: *mut MapHeader) {
    let mut mapLayout: *mut MapLayout = (*mapHeader).mapLayout;
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0x3ff03ff);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            sBackupMapData.as_mut_ptr() as *mut c_void,
            0x1001400,
        );
    }
    gBackupMapLayout.map = sBackupMapData.as_mut_ptr();
    gBackupMapLayout.width = (*mapLayout).width + MAP_OFFSET_W;
    gBackupMapLayout.height = (*mapLayout).height + MAP_OFFSET_H;
    if gBackupMapLayout.width * gBackupMapLayout.height > MAX_MAP_DATA_SIZE {
        return;
    }
    InitBackupMapLayoutData(
        (*mapLayout).map,
        (*mapLayout).width as u16,
        (*mapLayout).height as u16,
    );
    InitBackupMapLayoutConnections(mapHeader);
}
pub(crate) unsafe extern "C" fn InitBackupMapLayoutData(
    mut map: *mut u16,
    width: u16,
    height: u16,
) {
    let mut dest: *mut u16 = null_mut();
    let mut y: i32 = 0;
    dest = gBackupMapLayout.map;
    dest = dest.at(gBackupMapLayout.width * MAP_OFFSET + MAP_OFFSET);
    y = 0;
    while y < height as i32 {
        CpuSet(
            map as *mut c_void,
            dest as *mut c_void,
            0x00000000 | (width as i32 * 2 / 2) as u32 & 0x1FFFFF,
        );
        dest = dest.at(width as i32 + MAP_OFFSET_W);
        map = map.at(width);
        y += 1;
    }
}
pub(crate) unsafe extern "C" fn InitBackupMapLayoutConnections(mapHeader: *mut MapHeader) {
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let mut offset: i32 = 0;
    let mut connection: *mut MapConnection = null_mut();
    let mut cMap: *mut MapHeader = null_mut();
    if (*mapHeader).connections.is_null() {
        return;
    }
    count = (*(*mapHeader).connections).count;
    connection = (*(*mapHeader).connections).connections;
    sMapConnectionFlags = *sDummyConnectionFlags;
    i = 0;
    while i < count {
        cMap = GetMapHeaderFromConnection(connection);
        offset = (*connection).offset;
        match (*connection).direction {
            CONNECTION_SOUTH => {
                FillSouthConnection(mapHeader, cMap, offset);
                sMapConnectionFlags.set_south(TRUE);
            }
            CONNECTION_NORTH => {
                FillNorthConnection(mapHeader, cMap, offset);
                sMapConnectionFlags.set_north(TRUE);
            }
            CONNECTION_WEST => {
                FillWestConnection(mapHeader, cMap, offset);
                sMapConnectionFlags.set_west(TRUE);
            }
            CONNECTION_EAST => {
                FillEastConnection(mapHeader, cMap, offset);
                sMapConnectionFlags.set_east(TRUE);
            }
            _ => {}
        }
        i += 1;
        connection = connection.at(1);
    }
}
pub(crate) unsafe extern "C" fn FillConnection(
    x: i32,
    y: i32,
    connectedMapHeader: *mut MapHeader,
    x2: i32,
    y2: i32,
    width: i32,
    height: i32,
) {
    let mut i: i32 = 0;
    let mut src: *mut u16 = null_mut();
    let mut dest: *mut u16 = null_mut();
    let mut mapWidth: i32 = 0;
    mapWidth = (*(*connectedMapHeader).mapLayout).width;
    src = (*(*connectedMapHeader).mapLayout)
        .map
        .at(mapWidth * y2 + x2);
    dest = gBackupMapLayout.map.at(gBackupMapLayout.width * y + x);
    i = 0;
    while i < height {
        CpuSet(
            src as *mut c_void,
            dest as *mut c_void,
            0x00000000 | (width * 2 / 2) as u32 & 0x1FFFFF,
        );
        dest = dest.at(gBackupMapLayout.width);
        src = src.at(mapWidth);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FillSouthConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut x2: i32 = 0;
    let mut width: i32 = 0;
    let mut cWidth: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    cWidth = (*(*connectedMapHeader).mapLayout).width;
    x = offset + MAP_OFFSET;
    y = (*(*mapHeader).mapLayout).height + MAP_OFFSET;
    if x < 0 {
        x2 = -x;
        x += cWidth;
        if x < gBackupMapLayout.width {
            width = x;
        } else {
            width = gBackupMapLayout.width;
        }
        x = 0;
    } else {
        x2 = 0;
        if x + cWidth < gBackupMapLayout.width {
            width = cWidth;
        } else {
            width = gBackupMapLayout.width - x;
        }
    }
    FillConnection(x, y, connectedMapHeader, x2, 0, width, MAP_OFFSET);
}
pub(crate) unsafe extern "C" fn FillNorthConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut x: i32 = 0;
    let mut x2: i32 = 0;
    let mut y2: i32 = 0;
    let mut width: i32 = 0;
    let mut cWidth: i32 = 0;
    let mut cHeight: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    cWidth = (*(*connectedMapHeader).mapLayout).width;
    cHeight = (*(*connectedMapHeader).mapLayout).height;
    x = offset + MAP_OFFSET;
    y2 = cHeight - MAP_OFFSET;
    if x < 0 {
        x2 = -x;
        x += cWidth;
        if x < gBackupMapLayout.width {
            width = x;
        } else {
            width = gBackupMapLayout.width;
        }
        x = 0;
    } else {
        x2 = 0;
        if x + cWidth < gBackupMapLayout.width {
            width = cWidth;
        } else {
            width = gBackupMapLayout.width - x;
        }
    }
    FillConnection(x, 0, connectedMapHeader, x2, y2, width, MAP_OFFSET);
}
pub(crate) unsafe extern "C" fn FillWestConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut y: i32 = 0;
    let mut x2: i32 = 0;
    let mut y2: i32 = 0;
    let mut height: i32 = 0;
    let mut cWidth: i32 = 0;
    let mut cHeight: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    cWidth = (*(*connectedMapHeader).mapLayout).width;
    cHeight = (*(*connectedMapHeader).mapLayout).height;
    y = offset + MAP_OFFSET;
    x2 = cWidth - MAP_OFFSET;
    if y < 0 {
        y2 = -y;
        if y + cHeight < gBackupMapLayout.height {
            height = y + cHeight;
        } else {
            height = gBackupMapLayout.height;
        }
        y = 0;
    } else {
        y2 = 0;
        if y + cHeight < gBackupMapLayout.height {
            height = cHeight;
        } else {
            height = gBackupMapLayout.height - y;
        }
    }
    FillConnection(0, y, connectedMapHeader, x2, y2, MAP_OFFSET, height);
}
pub(crate) unsafe extern "C" fn FillEastConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut y2: i32 = 0;
    let mut height: i32 = 0;
    let mut cHeight: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    cHeight = (*(*connectedMapHeader).mapLayout).height;
    x = (*(*mapHeader).mapLayout).width + MAP_OFFSET;
    y = offset + MAP_OFFSET;
    if y < 0 {
        y2 = -y;
        if y + cHeight < gBackupMapLayout.height {
            height = y + cHeight;
        } else {
            height = gBackupMapLayout.height;
        }
        y = 0;
    } else {
        y2 = 0;
        if y + cHeight < gBackupMapLayout.height {
            height = cHeight;
        } else {
            height = gBackupMapLayout.height - y;
        }
    }
    FillConnection(x, y, connectedMapHeader, 0, y2, 8, height);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetElevationAt(x: i32, y: i32) -> u8 {
    let mut block: u16 =
        (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at((x + 1 & 1) + ((y + 1 & 1) << 1)) as i32
                | 0x0C00
        }) as u16;
    if block == MAPGRID_UNDEFINED {
        return 0;
    }
    return ((block as i32 & 0xF000) >> 12) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetCollisionAt(x: i32, y: i32) -> u8 {
    let mut block: u16 =
        (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at((x + 1 & 1) + ((y + 1 & 1) << 1)) as i32
                | 0x0C00
        }) as u16;
    if block == MAPGRID_UNDEFINED {
        return 1;
    }
    return ((block as i32 & 0x0C00) >> 10) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileIdAt(x: i32, y: i32) -> i32 {
    let mut block: i32 =
        if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at((x + 1 & 1) + ((y + 1 & 1) << 1)) as i32
                | 0x0C00
        };
    if block == MAPGRID_UNDEFINED as i32 {
        return ((*(*gMapHeader.mapLayout)
            .border
            .at((x + 1 & 1) + ((y + 1 & 1) << 1)) as i32
            | 0x0C00)
            & 0x03FF)
            >> 0;
    }
    return (block & 0x03FF) >> 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileBehaviorAt(x: i32, y: i32) -> i32 {
    return (GetMetatileAttributesById(MapGridGetMetatileIdAt(x, y) as u16) as i32 & 0x00FF) >> 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridGetMetatileLayerTypeAt(x: i32, y: i32) -> u8 {
    return ((GetMetatileAttributesById(MapGridGetMetatileIdAt(x, y) as u16) as i32 & 0xF000) >> 12)
        as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileIdAt(x: i32, y: i32, metatile: u16) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + y * gBackupMapLayout.width) &= MAPGRID_ELEVATION_MASK;
        *gBackupMapLayout.map.at(x + y * gBackupMapLayout.width) |= metatile & 4095;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileEntryAt(x: i32, y: i32, metatile: u16) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) = metatile;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMetatileAttributesById(metatile: u16) -> u16 {
    if metatile < NUM_METATILES_IN_PRIMARY {
        return *(*(*gMapHeader.mapLayout).primaryTileset)
            .metatileAttributes
            .at(metatile);
    } else if metatile < NUM_METATILES_TOTAL {
        return *(*(*gMapHeader.mapLayout).secondaryTileset)
            .metatileAttributes
            .at(metatile as i32 - NUM_METATILES_IN_PRIMARY as i32);
    } else {
        return 255;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveMapView() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut mapView: *mut u16 = null_mut();
    let mut width: i32 = 0;
    mapView = (*gSaveBlock1Ptr).mapView.as_mut_ptr();
    width = gBackupMapLayout.width;
    x = (*gSaveBlock1Ptr).pos.x as i32;
    y = (*gSaveBlock1Ptr).pos.y as i32;
    i = y;
    while i < y + MAP_OFFSET_H {
        j = x;
        while j < x + MAP_OFFSET_W {
            *({
                let t1 = mapView;
                mapView = mapView.at(1);
                t1
            }) = sBackupMapData[width * i + j];
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SavedMapViewIsEmpty() -> u32 {
    let mut i: u16 = 0;
    let mut marker: u32 = 0;
    i = 0;
    while i < 256 {
        marker |= (*gSaveBlock1Ptr).mapView[i] as u32;
        i += 1;
    }
    if marker == 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ClearSavedMapView() {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gSaveBlock1Ptr).mapView.as_mut_ptr() as *mut c_void,
                0x1000100,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadSavedMapView() {
    let mut yMode: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut mapView: *mut u16 = null_mut();
    let mut width: i32 = 0;
    mapView = (*gSaveBlock1Ptr).mapView.as_mut_ptr();
    if SavedMapViewIsEmpty() != 0 {
        return;
    }
    width = gBackupMapLayout.width;
    x = (*gSaveBlock1Ptr).pos.x as i32;
    y = (*gSaveBlock1Ptr).pos.y as i32;
    i = y;
    while i < y + MAP_OFFSET_H {
        if i == y && i != 0 {
            yMode = 0;
        } else if i == y + MAP_OFFSET_H - 1 && i != (*gMapHeader.mapLayout).height - 1 {
            yMode = 1;
        } else {
            yMode = 0xFF;
        }
        j = x;
        while j < x + MAP_OFFSET_W {
            if SkipCopyingMetatileFromSavedMap(
                &raw mut sBackupMapData[j + width * i],
                width as u16,
                yMode,
            ) == 0
            {
                sBackupMapData[j + width * i] = *mapView;
            }
            mapView = mapView.at(1);
            j += 1;
        }
        i += 1;
    }
    j = x;
    while j < x + MAP_OFFSET_W {
        if y != 0 {
            FixLongGrassMetatilesWindowTop(j as i16, y as i16 - 1);
        }
        if i < (*gMapHeader.mapLayout).height - 1 {
            FixLongGrassMetatilesWindowBottom(j as i16, y as i16 + MAP_OFFSET_H as i16 - 1);
        }
        j += 1;
    }
    ClearSavedMapView();
}
pub(crate) unsafe extern "C" fn MoveMapViewToBackup(direction: u8) {
    let mut width: i32 = 0;
    let mut x0: i32 = 0;
    let mut y0: i32 = 0;
    let mut x2: i32 = 0;
    let mut y2: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut mapView: *mut u16 = (*gSaveBlock1Ptr).mapView.as_mut_ptr();
    width = gBackupMapLayout.width;
    i = 0;
    j = 0;
    x0 = (*gSaveBlock1Ptr).pos.x as i32;
    y0 = (*gSaveBlock1Ptr).pos.y as i32;
    x2 = MAP_OFFSET_W;
    y2 = MAP_OFFSET_H;
    match direction {
        CONNECTION_NORTH => {
            y0 += 1;
            y2 = 13;
        }
        CONNECTION_SOUTH => {
            j = 1;
            y2 = 13;
        }
        CONNECTION_WEST => {
            x0 += 1;
            x2 = 14;
        }
        CONNECTION_EAST => {
            i = 1;
            x2 = 14;
        }
        _ => {}
    }
    y = 0;
    while y < y2 {
        x = 0;
        while x < x2 {
            sBackupMapData[x + x0 + width * (y + y0)] = *mapView.at(i + x + MAP_OFFSET_W * (j + y));
            x += 1;
        }
        y += 1;
    }
    ClearSavedMapView();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapBorderIdAt(x: i32, y: i32) -> i32 {
    if (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
    } else {
        *(*gMapHeader.mapLayout)
            .border
            .at((x + 1 & 1) + ((y + 1 & 1) << 1)) as i32
            | 0x0C00
    }) == MAPGRID_UNDEFINED as i32
    {
        return CONNECTION_INVALID;
    }
    if x >= gBackupMapLayout.width - 8 {
        if sMapConnectionFlags.east() == 0 {
            return CONNECTION_INVALID;
        }
        return CONNECTION_EAST as i32;
    } else if x < MAP_OFFSET {
        if sMapConnectionFlags.west() == 0 {
            return CONNECTION_INVALID;
        }
        return CONNECTION_WEST as i32;
    } else if y >= gBackupMapLayout.height - MAP_OFFSET {
        if sMapConnectionFlags.south() == 0 {
            return CONNECTION_INVALID;
        }
        return CONNECTION_SOUTH as i32;
    } else if y < MAP_OFFSET {
        if sMapConnectionFlags.north() == 0 {
            return CONNECTION_INVALID;
        }
        return CONNECTION_NORTH as i32;
    } else {
        return CONNECTION_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPostCameraMoveMapBorderId(x: i32, y: i32) -> i32 {
    return GetMapBorderIdAt(
        (*gSaveBlock1Ptr).pos.x as i32 + MAP_OFFSET + x,
        (*gSaveBlock1Ptr).pos.y as i32 + MAP_OFFSET + y,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanCameraMoveInDirection(direction: i32) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    x = (*gSaveBlock1Ptr).pos.x as i32 + MAP_OFFSET + gDirectionToVectors[direction].x as i32;
    y = (*gSaveBlock1Ptr).pos.y as i32 + MAP_OFFSET + gDirectionToVectors[direction].y as i32;
    if GetMapBorderIdAt(x, y) == CONNECTION_INVALID {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn SetPositionFromConnection(
    connection: *mut MapConnection,
    direction: i32,
    x: i32,
    y: i32,
) {
    let mut mapHeader: *mut MapHeader = null_mut();
    mapHeader = GetMapHeaderFromConnection(connection);
    match direction {
        4 => {
            (*gSaveBlock1Ptr).pos.x = -(x as i16);
            (*gSaveBlock1Ptr).pos.y -= (*connection).offset as i16;
        }
        3 => {
            (*gSaveBlock1Ptr).pos.x = (*(*mapHeader).mapLayout).width as i16;
            (*gSaveBlock1Ptr).pos.y -= (*connection).offset as i16;
        }
        1 => {
            (*gSaveBlock1Ptr).pos.x -= (*connection).offset as i16;
            (*gSaveBlock1Ptr).pos.y = -(y as i16);
        }
        2 => {
            (*gSaveBlock1Ptr).pos.x -= (*connection).offset as i16;
            (*gSaveBlock1Ptr).pos.y = (*(*mapHeader).mapLayout).height as i16;
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CameraMove(x: i32, y: i32) -> u8 {
    let mut direction: i32 = 0;
    let mut connection: *mut MapConnection = null_mut();
    let mut old_x: i32 = 0;
    let mut old_y: i32 = 0;
    gCamera.set_active(FALSE);
    direction = GetPostCameraMoveMapBorderId(x, y);
    if direction == CONNECTION_NONE || direction == CONNECTION_INVALID {
        (*gSaveBlock1Ptr).pos.x += x as i16;
        (*gSaveBlock1Ptr).pos.y += y as i16;
    } else {
        SaveMapView();
        ClearMirageTowerPulseBlendEffect();
        old_x = (*gSaveBlock1Ptr).pos.x as i32;
        old_y = (*gSaveBlock1Ptr).pos.y as i32;
        connection = GetIncomingConnection(
            direction as u8,
            (*gSaveBlock1Ptr).pos.x as i32,
            (*gSaveBlock1Ptr).pos.y as i32,
        );
        SetPositionFromConnection(connection, direction, x, y);
        LoadMapFromCameraTransition((*connection).mapGroup, (*connection).mapNum);
        gCamera.set_active(TRUE);
        gCamera.x = old_x - (*gSaveBlock1Ptr).pos.x as i32;
        gCamera.y = old_y - (*gSaveBlock1Ptr).pos.y as i32;
        (*gSaveBlock1Ptr).pos.x += x as i16;
        (*gSaveBlock1Ptr).pos.y += y as i16;
        MoveMapViewToBackup(direction as u8);
    }
    return gCamera.active();
}
pub(crate) unsafe extern "C" fn GetIncomingConnection(
    direction: u8,
    x: i32,
    y: i32,
) -> *mut MapConnection {
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let mut connection: *mut MapConnection = null_mut();
    let mut connections: *mut MapConnections = gMapHeader.connections;
    if connections.is_null() || (*connections).connections.is_null() {
        return null_mut();
    }
    count = (*connections).count;
    connection = (*connections).connections;
    i = 0;
    while i < count {
        if (*connection).direction == direction
            && IsPosInIncomingConnectingMap(direction, x, y, connection) == TRUE
        {
            return connection;
        }
        i += 1;
        connection = connection.at(1);
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn IsPosInIncomingConnectingMap(
    direction: u8,
    x: i32,
    y: i32,
    connection: *mut MapConnection,
) -> u8 {
    let mut mapHeader: *mut MapHeader = null_mut();
    mapHeader = GetMapHeaderFromConnection(connection);
    match direction {
        CONNECTION_SOUTH | CONNECTION_NORTH => {
            return IsCoordInIncomingConnectingMap(
                x,
                (*gMapHeader.mapLayout).width,
                (*(*mapHeader).mapLayout).width,
                (*connection).offset,
            );
        }
        CONNECTION_WEST | CONNECTION_EAST => {
            return IsCoordInIncomingConnectingMap(
                y,
                (*gMapHeader.mapLayout).height,
                (*(*mapHeader).mapLayout).height,
                (*connection).offset,
            );
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsCoordInIncomingConnectingMap(
    coord: i32,
    srcMax: i32,
    destMax: i32,
    offset: i32,
) -> u8 {
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    if offset < 0 {
        min = 0;
    } else {
        min = offset;
    }
    if destMax + offset < srcMax {
        max = destMax + offset;
    } else {
        max = srcMax;
    }
    if min <= coord && coord <= max {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsCoordInConnectingMap(coord: i32, max: i32) -> i32 {
    if coord >= 0 && coord < max {
        return TRUE as i32;
    }
    return FALSE as i32;
}
pub(crate) unsafe extern "C" fn IsPosInConnectingMap(
    connection: *mut MapConnection,
    x: i32,
    y: i32,
) -> i32 {
    let mut mapHeader: *mut MapHeader = null_mut();
    mapHeader = GetMapHeaderFromConnection(connection);
    match (*connection).direction {
        CONNECTION_SOUTH | CONNECTION_NORTH => {
            return IsCoordInConnectingMap(
                x - (*connection).offset,
                (*(*mapHeader).mapLayout).width,
            );
        }
        CONNECTION_WEST | CONNECTION_EAST => {
            return IsCoordInConnectingMap(
                y - (*connection).offset,
                (*(*mapHeader).mapLayout).height,
            );
        }
        _ => {}
    }
    return FALSE as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapConnectionAtPos(x: i16, y: i16) -> *mut MapConnection {
    let mut count: i32 = 0;
    let mut connection: *mut MapConnection = null_mut();
    let mut i: i32 = 0;
    let mut direction: u8 = 0;
    if gMapHeader.connections.is_null() {
        return null_mut();
    }
    count = (*gMapHeader.connections).count;
    connection = (*gMapHeader.connections).connections;
    i = 0;
    while i < count {
        'l1: {
            direction = (*connection).direction;
            if direction == CONNECTION_DIVE || direction == CONNECTION_EMERGE {
                break 'l1;
            } else if direction == CONNECTION_NORTH && y > 6 {
                break 'l1;
            } else if direction == CONNECTION_SOUTH
                && (y as i32) < (*gMapHeader.mapLayout).height + MAP_OFFSET
            {
                break 'l1;
            } else if direction == CONNECTION_WEST && x > 6 {
                break 'l1;
            } else if direction == CONNECTION_EAST
                && (x as i32) < (*gMapHeader.mapLayout).width + MAP_OFFSET
            {
                break 'l1;
            }
            if IsPosInConnectingMap(connection, x as i32 - MAP_OFFSET, y as i32 - MAP_OFFSET)
                == TRUE as i32
            {
                return connection;
            }
        }
        i += 1;
        connection = connection.at(1);
    }
    return null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraFocusCoords(x: u16, y: u16) {
    (*gSaveBlock1Ptr).pos.x = x as i16 - MAP_OFFSET as i16;
    (*gSaveBlock1Ptr).pos.y = y as i16 - MAP_OFFSET as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraFocusCoords(x: *mut u16, y: *mut u16) {
    *x = (*gSaveBlock1Ptr).pos.x as u16 + MAP_OFFSET as u16;
    *y = (*gSaveBlock1Ptr).pos.y as u16 + MAP_OFFSET as u16;
}
pub(crate) unsafe extern "C" fn SetCameraCoords(x: u16, y: u16) {
    (*gSaveBlock1Ptr).pos.x = x as i16;
    (*gSaveBlock1Ptr).pos.y = y as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraCoords(x: *mut u16, y: *mut u16) {
    *x = (*gSaveBlock1Ptr).pos.x as u16;
    *y = (*gSaveBlock1Ptr).pos.y as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapGridSetMetatileImpassabilityAt(x: i32, y: i32, impassable: u32) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        if impassable != 0 {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) |= MAPGRID_COLLISION_MASK;
        } else {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) &= 62463;
        }
    }
}
pub(crate) unsafe extern "C" fn SkipCopyingMetatileFromSavedMap(
    mut mapBlock: *mut u16,
    mapWidth: u16,
    yMode: u8,
) -> u8 {
    if yMode == 0xFF {
        return FALSE;
    }
    if yMode == 0 {
        mapBlock = mapBlock.at(-(mapWidth as i32));
    } else {
        mapBlock = mapBlock.at(mapWidth);
    }
    if IsLargeBreakableDecoration(((*mapBlock as i32 & 0x03FF) >> 0) as u16, yMode) == TRUE {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CopyTilesetToVram(
    tileset: *mut Tileset,
    numTiles: u16,
    offset: u16,
) {
    if !tileset.is_null() {
        if (*tileset).isCompressed == 0 {
            LoadBgTiles(2, (*tileset).tiles as *mut c_void, numTiles * 32, offset);
        } else {
            DecompressAndCopyTileDataToVram(
                2,
                (*tileset).tiles as *mut c_void,
                numTiles as u32 * 32,
                offset,
                0,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CopyTilesetToVramUsingHeap(
    tileset: *mut Tileset,
    numTiles: u16,
    offset: u16,
) {
    if !tileset.is_null() {
        if (*tileset).isCompressed == 0 {
            LoadBgTiles(2, (*tileset).tiles as *mut c_void, numTiles * 32, offset);
        } else {
            DecompressAndLoadBgGfxUsingHeap(
                2,
                (*tileset).tiles as *mut c_void,
                numTiles as u32 * 32,
                offset,
                0,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ApplyGlobalTintToPaletteEntries(offset: u16, size: u16) {}
pub(crate) unsafe extern "C" fn ApplyGlobalTintToPaletteSlot(slot: u8, count: u8) {}
pub(crate) unsafe extern "C" fn LoadTilesetPalette(
    tileset: *mut Tileset,
    destOffset: u16,
    size: u16,
) {
    let mut black: u16 = 0;
    if !tileset.is_null() {
        if (*tileset).isSecondary == FALSE {
            LoadPalette(&raw mut black as *mut c_void, destOffset, 2);
            LoadPalette(
                (*(*tileset).palettes).as_mut_ptr().at(1) as *mut c_void,
                destOffset + 1,
                size - 2,
            );
            ApplyGlobalTintToPaletteEntries(destOffset + 1, (size as u32 - 2 >> 1) as u16);
        } else if (*tileset).isSecondary == TRUE {
            LoadPalette(
                (*(*tileset).palettes.at(6)).as_mut_ptr() as *mut c_void,
                destOffset,
                size,
            );
            ApplyGlobalTintToPaletteEntries(destOffset, size >> 1);
        } else {
            LoadCompressedPalette((*tileset).palettes as *mut u32, destOffset, size);
            ApplyGlobalTintToPaletteEntries(destOffset, size >> 1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPrimaryTilesetToVram(mapLayout: *mut MapLayout) {
    CopyTilesetToVram((*mapLayout).primaryTileset, NUM_TILES_IN_PRIMARY, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopySecondaryTilesetToVram(mapLayout: *mut MapLayout) {
    CopyTilesetToVram(
        (*mapLayout).secondaryTileset,
        NUM_TILES_IN_PRIMARY,
        NUM_TILES_IN_PRIMARY,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopySecondaryTilesetToVramUsingHeap(mapLayout: *mut MapLayout) {
    CopyTilesetToVramUsingHeap(
        (*mapLayout).secondaryTileset,
        NUM_TILES_IN_PRIMARY,
        NUM_TILES_IN_PRIMARY,
    );
}
pub(crate) unsafe extern "C" fn LoadPrimaryTilesetPalette(mapLayout: *mut MapLayout) {
    LoadTilesetPalette((*mapLayout).primaryTileset, 0, 192);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadSecondaryTilesetPalette(mapLayout: *mut MapLayout) {
    LoadTilesetPalette((*mapLayout).secondaryTileset, 96, 224);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMapTilesetsToVram(mapLayout: *mut MapLayout) {
    if !mapLayout.is_null() {
        CopyTilesetToVramUsingHeap((*mapLayout).primaryTileset, NUM_TILES_IN_PRIMARY, 0);
        CopyTilesetToVramUsingHeap(
            (*mapLayout).secondaryTileset,
            NUM_TILES_IN_PRIMARY,
            NUM_TILES_IN_PRIMARY,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMapTilesetPalettes(mapLayout: *mut MapLayout) {
    if !mapLayout.is_null() {
        LoadPrimaryTilesetPalette(mapLayout);
        LoadSecondaryTilesetPalette(mapLayout);
    }
}
