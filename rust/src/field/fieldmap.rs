//! Translated from `src/fieldmap.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_pyramid::GenerateBattlePyramidFloorLayout;
use crate::bg::LoadBgTiles;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::fldeff_cut::{FixLongGrassMetatilesWindowBottom, FixLongGrassMetatilesWindowTop};
use crate::fldeff_misc::IsLargeBreakableDecoration;
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{DecompressAndCopyTileDataToVram, DecompressAndLoadBgGfxUsingHeap};
use crate::mirage_tower::ClearMirageTowerPulseBlendEffect;
use crate::overworld::{LoadMapFromCameraTransition, Overworld_GetMapHeaderByGroupAndId};
use crate::palette::{LoadCompressedPalette, LoadPalette};
use crate::script::RunOnLoadMapScript;
use crate::secret_base::{InitSecretBaseAppearance, SetOccupiedSecretBaseEntranceMetatiles};
use crate::trainer_hill::GenerateTrainerHillFloorLayout;
use crate::tv::UpdateTVScreensOnMap;
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
        ((self.bits_0 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_south(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn north(&self) -> u8 {
        ((self.bits_0 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_north(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 1)) | ((v & 0x1) << 1);
    }
    #[inline(always)]
    pub fn west(&self) -> u8 {
        ((self.bits_0 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_west(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 2)) | ((v & 0x1) << 2);
    }
    #[inline(always)]
    pub fn east(&self) -> u8 {
        ((self.bits_0 as u32 >> 3) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_east(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x1 << 3)) | ((v & 0x1) << 3);
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
#[unsafe(link_section = "ewram_data")]
pub static mut gCamera: Camera = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapConnectionFlags: ConnectionFlags = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u32 = 0;
#[unsafe(link_section = "common_data")]
pub static mut gBackupMapLayout: BackupMapLayout = unsafe { zeroed() };

/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn GetMapHeaderFromConnection(connection: *mut MapConnection) -> *mut MapHeader {
    Overworld_GetMapHeaderByGroupAndId((*connection).mapGroup as u16, (*connection).mapNum as u16)
}
pub unsafe fn InitMap() {
    InitMapLayoutData(&raw mut gMapHeader);
    SetOccupiedSecretBaseEntranceMetatiles(gMapHeader.events);
    RunOnLoadMapScript();
}
pub unsafe fn InitMapFromSavedGame() {
    InitMapLayoutData(&raw mut gMapHeader);
    InitSecretBaseAppearance(FALSE);
    SetOccupiedSecretBaseEntranceMetatiles(gMapHeader.events);
    LoadSavedMapView();
    RunOnLoadMapScript();
    UpdateTVScreensOnMap(gBackupMapLayout.width, gBackupMapLayout.height);
}
pub unsafe fn InitBattlePyramidMap(setPlayerPosition: u8) {
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
pub unsafe fn InitTrainerHillMap() {
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
unsafe fn InitMapLayoutData(mapHeader: *mut MapHeader) {
    let mapLayout: *mut MapLayout = (*mapHeader).mapLayout;
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
unsafe fn InitBackupMapLayoutData(mut map: *mut u16, width: u16, height: u16) {
    let mut dest: *mut u16 = gBackupMapLayout.map;
    dest = dest.at(gBackupMapLayout.width * MAP_OFFSET + MAP_OFFSET);
    for y in 0..(height as i32) {
        CpuSet(
            map as *mut c_void,
            dest as *mut c_void,
            (width as i32 * 2 / 2) as u32 & 0x1FFFFF,
        );
        dest = dest.at(width as i32 + MAP_OFFSET_W);
        map = map.at(width);
    }
}
unsafe fn InitBackupMapLayoutConnections(mapHeader: *mut MapHeader) {
    let mut count: i32 = 0;
    let mut offset: i32 = 0;
    let mut cMap: *mut MapHeader = null_mut();
    if (*mapHeader).connections.is_null() {
        return;
    }
    count = (*(*mapHeader).connections).count;
    let mut connection: *mut MapConnection = (*(*mapHeader).connections).connections;
    sMapConnectionFlags = *sDummyConnectionFlags;
    let mut i: i32 = 0;
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
unsafe fn FillConnection(
    x: i32,
    y: i32,
    connectedMapHeader: *mut MapHeader,
    x2: i32,
    y2: i32,
    width: i32,
    height: i32,
) {
    let mapWidth: i32 = (*(*connectedMapHeader).mapLayout).width;
    let mut src: *mut u16 = (*(*connectedMapHeader).mapLayout)
        .map
        .at(mapWidth * y2 + x2);
    let mut dest: *mut u16 = gBackupMapLayout.map.at(gBackupMapLayout.width * y + x);
    for i in 0..height {
        CpuSet(
            src as *mut c_void,
            dest as *mut c_void,
            (width * 2 / 2) as u32 & 0x1FFFFF,
        );
        dest = dest.at(gBackupMapLayout.width);
        src = src.at(mapWidth);
    }
}
unsafe fn FillSouthConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut x2: i32 = 0;
    let mut width: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    let cWidth: i32 = (*(*connectedMapHeader).mapLayout).width;
    let mut x: i32 = offset + MAP_OFFSET;
    let y: i32 = (*(*mapHeader).mapLayout).height + MAP_OFFSET;
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
unsafe fn FillNorthConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut x2: i32 = 0;
    let mut width: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    let cWidth: i32 = (*(*connectedMapHeader).mapLayout).width;
    let cHeight: i32 = (*(*connectedMapHeader).mapLayout).height;
    let mut x: i32 = offset + MAP_OFFSET;
    let y2: i32 = cHeight - MAP_OFFSET;
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
unsafe fn FillWestConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut y2: i32 = 0;
    let mut height: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    let cWidth: i32 = (*(*connectedMapHeader).mapLayout).width;
    let cHeight: i32 = (*(*connectedMapHeader).mapLayout).height;
    let mut y: i32 = offset + MAP_OFFSET;
    let x2: i32 = cWidth - MAP_OFFSET;
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
unsafe fn FillEastConnection(
    mapHeader: *mut MapHeader,
    connectedMapHeader: *mut MapHeader,
    offset: i32,
) {
    let mut y2: i32 = 0;
    let mut height: i32 = 0;
    if connectedMapHeader.is_null() {
        return;
    }
    let cHeight: i32 = (*(*connectedMapHeader).mapLayout).height;
    let x: i32 = (*(*mapHeader).mapLayout).width + MAP_OFFSET;
    let mut y: i32 = offset + MAP_OFFSET;
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
pub unsafe fn MapGridGetElevationAt(x: i32, y: i32) -> u8 {
    let block: u16 =
        (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at(((x + 1) & 1) + (((y + 1) & 1) << 1)) as i32
                | 0x0C00
        }) as u16;
    if block == MAPGRID_UNDEFINED {
        return 0;
    }
    ((block as i32 & 0xF000) >> 12) as u8
}
pub unsafe fn MapGridGetCollisionAt(x: i32, y: i32) -> u8 {
    let block: u16 =
        (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at(((x + 1) & 1) + (((y + 1) & 1) << 1)) as i32
                | 0x0C00
        }) as u16;
    if block == MAPGRID_UNDEFINED {
        return 1;
    }
    ((block as i32 & 0x0C00) >> 10) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn MapGridGetMetatileIdAt(x: i32, y: i32) -> i32 {
    let block: i32 =
        if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
        } else {
            *(*gMapHeader.mapLayout)
                .border
                .at(((x + 1) & 1) + (((y + 1) & 1) << 1)) as i32
                | 0x0C00
        };
    if block == MAPGRID_UNDEFINED as i32 {
        return (*(*gMapHeader.mapLayout)
            .border
            .at(((x + 1) & 1) + (((y + 1) & 1) << 1)) as i32
            | 0x0C00)
            & 0x03FF;
    }
    block & 0x03FF
}
pub unsafe fn MapGridGetMetatileBehaviorAt(x: i32, y: i32) -> i32 {
    GetMetatileAttributesById(MapGridGetMetatileIdAt(x, y) as u16) as i32 & 0x00FF
}
pub unsafe fn MapGridGetMetatileLayerTypeAt(x: i32, y: i32) -> u8 {
    ((GetMetatileAttributesById(MapGridGetMetatileIdAt(x, y) as u16) as i32 & 0xF000) >> 12) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn MapGridSetMetatileIdAt(x: i32, y: i32, metatile: u16) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + y * gBackupMapLayout.width) &= MAPGRID_ELEVATION_MASK;
        *gBackupMapLayout.map.at(x + y * gBackupMapLayout.width) |= metatile & 4095;
    }
}
pub unsafe fn MapGridSetMetatileEntryAt(x: i32, y: i32, metatile: u16) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) = metatile;
    }
}
pub unsafe fn GetMetatileAttributesById(metatile: u16) -> u16 {
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
        0
    }
}
pub unsafe fn SaveMapView() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut mapView: *mut u16 = null_mut();
    let mut width: i32 = 0;
    mapView = (*gSaveBlock1Ptr).mapView.as_mut_ptr();
    width = gBackupMapLayout.width;
    x = (*gSaveBlock1Ptr).pos.x as i32;
    y = (*gSaveBlock1Ptr).pos.y as i32;
    for i in y..(y + MAP_OFFSET_H) {
        for j in x..(x + MAP_OFFSET_W) {
            *({
                let t1 = mapView;
                mapView = mapView.at(1);
                t1
            }) = sBackupMapData[width * i + j];
        }
    }
}
unsafe fn SavedMapViewIsEmpty() -> u32 {
    let mut marker: u32 = 0;
    for i in 0..256u16 {
        marker |= (*gSaveBlock1Ptr).mapView[i] as u32;
    }
    if marker == 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ClearSavedMapView() {
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
unsafe fn LoadSavedMapView() {
    let mut yMode: u8 = 0;
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
    let mut i: i32 = y;
    while i < y + MAP_OFFSET_H {
        if i == y && i != 0 {
            yMode = 0;
        } else if i == y + MAP_OFFSET_H - 1 && i != (*gMapHeader.mapLayout).height - 1 {
            yMode = 1;
        } else {
            yMode = 0xFF;
        }
        for j in x..(x + MAP_OFFSET_W) {
            if SkipCopyingMetatileFromSavedMap(
                &raw mut sBackupMapData[j + width * i],
                width as u16,
                yMode,
            ) == 0
            {
                sBackupMapData[j + width * i] = *mapView;
            }
            mapView = mapView.at(1);
        }
        i += 1;
    }
    for j in x..(x + MAP_OFFSET_W) {
        if y != 0 {
            FixLongGrassMetatilesWindowTop(j as i16, y as i16 - 1);
        }
        if i < (*gMapHeader.mapLayout).height - 1 {
            FixLongGrassMetatilesWindowBottom(j as i16, y as i16 + MAP_OFFSET_H as i16 - 1);
        }
    }
    ClearSavedMapView();
}
unsafe fn MoveMapViewToBackup(direction: u8) {
    let mut width: i32 = 0;
    let mapView: *mut u16 = (*gSaveBlock1Ptr).mapView.as_mut_ptr();
    width = gBackupMapLayout.width;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x0: i32 = (*gSaveBlock1Ptr).pos.x as i32;
    let mut y0: i32 = (*gSaveBlock1Ptr).pos.y as i32;
    let mut x2: i32 = MAP_OFFSET_W;
    let mut y2: i32 = MAP_OFFSET_H;
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
    for y in 0..y2 {
        for x in 0..x2 {
            sBackupMapData[x + x0 + width * (y + y0)] = *mapView.at(i + x + MAP_OFFSET_W * (j + y));
        }
    }
    ClearSavedMapView();
}
pub unsafe fn GetMapBorderIdAt(x: i32, y: i32) -> i32 {
    if (if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) as i32
    } else {
        *(*gMapHeader.mapLayout)
            .border
            .at(((x + 1) & 1) + (((y + 1) & 1) << 1)) as i32
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
        0
    }
}
pub unsafe fn GetPostCameraMoveMapBorderId(x: i32, y: i32) -> i32 {
    GetMapBorderIdAt(
        (*gSaveBlock1Ptr).pos.x as i32 + MAP_OFFSET + x,
        (*gSaveBlock1Ptr).pos.y as i32 + MAP_OFFSET + y,
    )
}
pub unsafe fn CanCameraMoveInDirection(direction: i32) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    x = (*gSaveBlock1Ptr).pos.x as i32
        + MAP_OFFSET
        + (*(&raw const crate::data::overworld::gDirectionToVectors).cast::<CArray<UCoords32, 0>>())
            [direction]
            .x as i32;
    y = (*gSaveBlock1Ptr).pos.y as i32
        + MAP_OFFSET
        + (*(&raw const crate::data::overworld::gDirectionToVectors).cast::<CArray<UCoords32, 0>>())
            [direction]
            .y as i32;
    if GetMapBorderIdAt(x, y) == CONNECTION_INVALID {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn SetPositionFromConnection(
    connection: *mut MapConnection,
    direction: i32,
    x: i32,
    y: i32,
) {
    let mapHeader: *mut MapHeader = GetMapHeaderFromConnection(connection);
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
pub unsafe fn CameraMove(x: i32, y: i32) -> u8 {
    let mut connection: *mut MapConnection = null_mut();
    let mut old_x: i32 = 0;
    let mut old_y: i32 = 0;
    gCamera.set_active(FALSE);
    let direction: i32 = GetPostCameraMoveMapBorderId(x, y);
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
    gCamera.active()
}
unsafe fn GetIncomingConnection(direction: u8, x: i32, y: i32) -> *mut MapConnection {
    let mut count: i32 = 0;
    let connections: *mut MapConnections = gMapHeader.connections;
    if connections.is_null() || (*connections).connections.is_null() {
        return null_mut();
    }
    count = (*connections).count;
    let mut connection: *mut MapConnection = (*connections).connections;
    let mut i: i32 = 0;
    while i < count {
        if (*connection).direction == direction
            && IsPosInIncomingConnectingMap(direction, x, y, connection) == TRUE
        {
            return connection;
        }
        i += 1;
        connection = connection.at(1);
    }
    null_mut()
}
unsafe fn IsPosInIncomingConnectingMap(
    direction: u8,
    x: i32,
    y: i32,
    connection: *mut MapConnection,
) -> u8 {
    let mapHeader: *mut MapHeader = GetMapHeaderFromConnection(connection);
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
    FALSE
}
fn IsCoordInIncomingConnectingMap(coord: i32, srcMax: i32, destMax: i32, offset: i32) -> u8 {
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
    FALSE
}
fn IsCoordInConnectingMap(coord: i32, max: i32) -> i32 {
    if coord >= 0 && coord < max {
        return TRUE as i32;
    }
    FALSE as i32
}
unsafe fn IsPosInConnectingMap(connection: *mut MapConnection, x: i32, y: i32) -> i32 {
    let mapHeader: *mut MapHeader = GetMapHeaderFromConnection(connection);
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
    FALSE as i32
}
pub unsafe fn GetMapConnectionAtPos(x: i16, y: i16) -> *mut MapConnection {
    let mut count: i32 = 0;
    let mut direction: u8 = 0;
    if gMapHeader.connections.is_null() {
        return null_mut();
    }
    count = (*gMapHeader.connections).count;
    let mut connection: *mut MapConnection = (*gMapHeader.connections).connections;
    let mut i: i32 = 0;
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
    null_mut()
}
pub unsafe fn SetCameraFocusCoords(x: u16, y: u16) {
    (*gSaveBlock1Ptr).pos.x = x as i16 - MAP_OFFSET as i16;
    (*gSaveBlock1Ptr).pos.y = y as i16 - MAP_OFFSET as i16;
}
pub unsafe fn GetCameraFocusCoords(x: *mut u16, y: *mut u16) {
    *x = (*gSaveBlock1Ptr).pos.x as u16 + MAP_OFFSET as u16;
    *y = (*gSaveBlock1Ptr).pos.y as u16 + MAP_OFFSET as u16;
}
unsafe fn SetCameraCoords(x: u16, y: u16) {
    (*gSaveBlock1Ptr).pos.x = x as i16;
    (*gSaveBlock1Ptr).pos.y = y as i16;
}
pub unsafe fn GetCameraCoords(x: *mut u16, y: *mut u16) {
    *x = (*gSaveBlock1Ptr).pos.x as u16;
    *y = (*gSaveBlock1Ptr).pos.y as u16;
}
pub unsafe fn MapGridSetMetatileImpassabilityAt(x: i32, y: i32, impassable: u32) {
    if x >= 0 && x < gBackupMapLayout.width && y >= 0 && y < gBackupMapLayout.height {
        if impassable != 0 {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) |= MAPGRID_COLLISION_MASK;
        } else {
            *gBackupMapLayout.map.at(x + gBackupMapLayout.width * y) &= 62463;
        }
    }
}
unsafe fn SkipCopyingMetatileFromSavedMap(mut mapBlock: *mut u16, mapWidth: u16, yMode: u8) -> u8 {
    if yMode == 0xFF {
        return FALSE;
    }
    if yMode == 0 {
        mapBlock = mapBlock.at(-(mapWidth as i32));
    } else {
        mapBlock = mapBlock.at(mapWidth);
    }
    if IsLargeBreakableDecoration((*mapBlock as i32 & 0x03FF) as u16, yMode) == TRUE {
        return TRUE;
    }
    FALSE
}
unsafe fn CopyTilesetToVram(tileset: *mut Tileset, numTiles: u16, offset: u16) {
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
unsafe fn CopyTilesetToVramUsingHeap(tileset: *mut Tileset, numTiles: u16, offset: u16) {
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
fn ApplyGlobalTintToPaletteEntries(offset: u16, size: u16) {}
fn ApplyGlobalTintToPaletteSlot(slot: u8, count: u8) {}
unsafe fn LoadTilesetPalette(tileset: *mut Tileset, destOffset: u16, size: u16) {
    let mut black: u16 = 0;
    if !tileset.is_null() {
        if (*tileset).isSecondary == FALSE {
            LoadPalette(&raw mut black as *mut c_void, destOffset, 2);
            LoadPalette(
                (*(*tileset).palettes).as_mut_ptr().at(1) as *mut c_void,
                destOffset + 1,
                size - 2,
            );
            ApplyGlobalTintToPaletteEntries(destOffset + 1, ((size as u32 - 2) >> 1) as u16);
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
pub unsafe fn CopyPrimaryTilesetToVram(mapLayout: *mut MapLayout) {
    CopyTilesetToVram((*mapLayout).primaryTileset, NUM_TILES_IN_PRIMARY, 0);
}
pub unsafe fn CopySecondaryTilesetToVram(mapLayout: *mut MapLayout) {
    CopyTilesetToVram(
        (*mapLayout).secondaryTileset,
        NUM_TILES_IN_PRIMARY,
        NUM_TILES_IN_PRIMARY,
    );
}
pub unsafe fn CopySecondaryTilesetToVramUsingHeap(mapLayout: *mut MapLayout) {
    CopyTilesetToVramUsingHeap(
        (*mapLayout).secondaryTileset,
        NUM_TILES_IN_PRIMARY,
        NUM_TILES_IN_PRIMARY,
    );
}
unsafe fn LoadPrimaryTilesetPalette(mapLayout: *mut MapLayout) {
    LoadTilesetPalette((*mapLayout).primaryTileset, 0, 192);
}
pub unsafe fn LoadSecondaryTilesetPalette(mapLayout: *mut MapLayout) {
    LoadTilesetPalette((*mapLayout).secondaryTileset, 96, 224);
}
pub unsafe fn CopyMapTilesetsToVram(mapLayout: *mut MapLayout) {
    if !mapLayout.is_null() {
        CopyTilesetToVramUsingHeap((*mapLayout).primaryTileset, NUM_TILES_IN_PRIMARY, 0);
        CopyTilesetToVramUsingHeap(
            (*mapLayout).secondaryTileset,
            NUM_TILES_IN_PRIMARY,
            NUM_TILES_IN_PRIMARY,
        );
    }
}
pub unsafe fn LoadMapTilesetPalettes(mapLayout: *mut MapLayout) {
    if !mapLayout.is_null() {
        LoadPrimaryTilesetPalette(mapLayout);
        LoadSecondaryTilesetPalette(mapLayout);
    }
}
