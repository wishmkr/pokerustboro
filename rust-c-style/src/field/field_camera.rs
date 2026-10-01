//! Translated from `src/field_camera.c` by tools/rustport/c2rs.py.
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

/// `struct FieldCameraOffset`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct FieldCameraOffset {
    pub xPixelOffset: u8,
    pub yPixelOffset: u8,
    pub xTileOffset: u8,
    pub yTileOffset: u8,
    pub copyBGToVRAM: u8,
}

unsafe impl Sync for FieldCameraOffset {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FieldCameraOffset>() == 8);
    assert!(offset_of!(FieldCameraOffset, xPixelOffset) == 0);
    assert!(offset_of!(FieldCameraOffset, yPixelOffset) == 1);
    assert!(offset_of!(FieldCameraOffset, xTileOffset) == 2);
    assert!(offset_of!(FieldCameraOffset, yTileOffset) == 3);
    assert!(offset_of!(FieldCameraOffset, copyBGToVRAM) == 4);
};

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedBikeCameraAheadPanback: u8 = 0;
pub(crate) static mut sFieldCameraOffset: FieldCameraOffset = unsafe { zeroed() };
pub(crate) static mut sHorizontalCameraPan: i16 = 0;
pub(crate) static mut sVerticalCameraPan: i16 = 0;
pub(crate) static mut sBikeCameraPanFlag: u8 = 0;
pub(crate) static mut sFieldCameraPanningCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCamera: CameraObject = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTotalCameraPixelOffsetY: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTotalCameraPixelOffsetX: u16 = 0;

unsafe extern "C" {
    static mut gMapHeader: MapHeader;
    static mut gOverworldTilemapBuffer_Bg1: *mut u16;
    static mut gOverworldTilemapBuffer_Bg2: *mut u16;
    static mut gOverworldTilemapBuffer_Bg3: *mut u16;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    fn AddCameraObject(a0: u8) -> u8;
    fn CameraMove(a0: i32, a1: i32) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn GetPlayerMovementDirection() -> u8;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileLayerTypeAt(a0: i32, a1: i32) -> u8;
    fn RotatingGatePuzzleCameraUpdate(a0: i16, a1: i16);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBerryTreesSeen();
    fn SetGpuReg(a0: u8, a1: u16);
    fn UpdateObjectEventsForCameraUpdate(a0: i16, a1: i16);
}

pub(crate) unsafe extern "C" fn ResetCameraOffset(cameraOffset: *mut FieldCameraOffset) {
    (*cameraOffset).xTileOffset = 0;
    (*cameraOffset).yTileOffset = 0;
    (*cameraOffset).xPixelOffset = 0;
    (*cameraOffset).yPixelOffset = 0;
    (*cameraOffset).copyBGToVRAM = TRUE;
}
pub(crate) unsafe extern "C" fn AddCameraTileOffset(
    cameraOffset: *mut FieldCameraOffset,
    xOffset: u32,
    yOffset: u32,
) {
    (*cameraOffset).xTileOffset += xOffset as u8;
    (*cameraOffset).xTileOffset = ((*cameraOffset).xTileOffset as i32 % 32) as u8;
    (*cameraOffset).yTileOffset += yOffset as u8;
    (*cameraOffset).yTileOffset = ((*cameraOffset).yTileOffset as i32 % 32) as u8;
}
pub(crate) unsafe extern "C" fn AddCameraPixelOffset(
    cameraOffset: *mut FieldCameraOffset,
    xOffset: u32,
    yOffset: u32,
) {
    (*cameraOffset).xPixelOffset += xOffset as u8;
    (*cameraOffset).yPixelOffset += yOffset as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFieldCamera() {
    ResetCameraOffset(&raw mut sFieldCameraOffset);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldUpdateBgTilemapScroll() {
    let mut r4: u32 = 0;
    let mut r5: u32 = 0;
    r5 = sFieldCameraOffset.xPixelOffset as u32 + sHorizontalCameraPan as u32;
    r4 = sVerticalCameraPan as u32 + sFieldCameraOffset.yPixelOffset as u32 + 8;
    SetGpuReg(REG_OFFSET_BG1HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG1VOFS, r4 as u16);
    SetGpuReg(REG_OFFSET_BG2HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG2VOFS, r4 as u16);
    SetGpuReg(REG_OFFSET_BG3HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG3VOFS, r4 as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraOffsetWithPan(x: *mut i16, y: *mut i16) {
    *x = sFieldCameraOffset.xPixelOffset as i16 + sHorizontalCameraPan;
    *y = sFieldCameraOffset.yPixelOffset as i16 + sVerticalCameraPan + 8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawWholeMapView() {
    DrawWholeMapViewInternal(
        (*gSaveBlock1Ptr).pos.x as i32,
        (*gSaveBlock1Ptr).pos.y as i32,
        gMapHeader.mapLayout,
    );
    sFieldCameraOffset.copyBGToVRAM = TRUE;
}
pub(crate) unsafe extern "C" fn DrawWholeMapViewInternal(
    x: i32,
    y: i32,
    mapLayout: *mut MapLayout,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut r6: u32 = 0;
    let mut temp: u8 = 0;
    i = 0;
    while i < 32 {
        temp = sFieldCameraOffset.yTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        r6 = temp as u32 * 32;
        j = 0;
        while j < 32 {
            temp = sFieldCameraOffset.xTileOffset + j;
            if temp >= 32 {
                temp -= 32;
            }
            DrawMetatileAt(
                mapLayout,
                r6 as u16 + temp as u16,
                x + j as i32 / 2,
                y + i as i32 / 2,
            );
            j += 2;
        }
        i += 2;
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSlicesForCameraUpdate(
    cameraOffset: *mut FieldCameraOffset,
    x: i32,
    y: i32,
) {
    let mut mapLayout: *mut MapLayout = gMapHeader.mapLayout;
    if x > 0 {
        RedrawMapSliceWest(cameraOffset, mapLayout);
    }
    if x < 0 {
        RedrawMapSliceEast(cameraOffset, mapLayout);
    }
    if y > 0 {
        RedrawMapSliceNorth(cameraOffset, mapLayout);
    }
    if y < 0 {
        RedrawMapSliceSouth(cameraOffset, mapLayout);
    }
    (*cameraOffset).copyBGToVRAM = TRUE;
}
pub(crate) unsafe extern "C" fn RedrawMapSliceNorth(
    cameraOffset: *mut FieldCameraOffset,
    mapLayout: *mut MapLayout,
) {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    let mut r7: u32 = 0;
    temp = (*cameraOffset).yTileOffset + 28;
    if temp >= 32 {
        temp -= 32;
    }
    r7 = temp as u32 * 32;
    i = 0;
    while i < 32 {
        temp = (*cameraOffset).xTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            r7 as u16 + temp as u16,
            (*gSaveBlock1Ptr).pos.x as i32 + i as i32 / 2,
            (*gSaveBlock1Ptr).pos.y as i32 + 14,
        );
        i += 2;
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceSouth(
    cameraOffset: *mut FieldCameraOffset,
    mapLayout: *mut MapLayout,
) {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    let mut r7: u32 = (*cameraOffset).yTileOffset as u32 * 32;
    i = 0;
    while i < 32 {
        temp = (*cameraOffset).xTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            r7 as u16 + temp as u16,
            (*gSaveBlock1Ptr).pos.x as i32 + i as i32 / 2,
            (*gSaveBlock1Ptr).pos.y as i32,
        );
        i += 2;
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceEast(
    cameraOffset: *mut FieldCameraOffset,
    mapLayout: *mut MapLayout,
) {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    let mut r6: u32 = (*cameraOffset).xTileOffset as u32;
    i = 0;
    while i < 32 {
        temp = (*cameraOffset).yTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            temp as u16 * 32 + r6 as u16,
            (*gSaveBlock1Ptr).pos.x as i32,
            (*gSaveBlock1Ptr).pos.y as i32 + i as i32 / 2,
        );
        i += 2;
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceWest(
    cameraOffset: *mut FieldCameraOffset,
    mapLayout: *mut MapLayout,
) {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    let mut r5: u8 = (*cameraOffset).xTileOffset + 28;
    if r5 >= 32 {
        r5 -= 32;
    }
    i = 0;
    while i < 32 {
        temp = (*cameraOffset).yTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            temp as u16 * 32 + r5 as u16,
            (*gSaveBlock1Ptr).pos.x as i32 + 14,
            (*gSaveBlock1Ptr).pos.y as i32 + i as i32 / 2,
        );
        i += 2;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurrentMapDrawMetatileAt(x: i32, y: i32) {
    let mut offset: i32 = MapPosToBgTilemapOffset(&raw mut sFieldCameraOffset, x, y);
    if offset >= 0 {
        DrawMetatileAt(gMapHeader.mapLayout, offset as u16, x, y);
        sFieldCameraOffset.copyBGToVRAM = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawDoorMetatileAt(x: i32, y: i32, tiles: *mut u16) {
    let mut offset: i32 = MapPosToBgTilemapOffset(&raw mut sFieldCameraOffset, x, y);
    if offset >= 0 {
        DrawMetatile(METATILE_LAYER_TYPE_COVERED, tiles, offset as u16);
        sFieldCameraOffset.copyBGToVRAM = TRUE;
    }
}
pub(crate) unsafe extern "C" fn DrawMetatileAt(
    mapLayout: *mut MapLayout,
    offset: u16,
    x: i32,
    y: i32,
) {
    let mut metatileId: u16 = MapGridGetMetatileIdAt(x, y) as u16;
    let mut metatiles: *mut u16 = null_mut();
    if metatileId > NUM_METATILES_TOTAL {
        metatileId = 0;
    }
    if metatileId < NUM_METATILES_IN_PRIMARY {
        metatiles = (*(*mapLayout).primaryTileset).metatiles;
    } else {
        metatiles = (*(*mapLayout).secondaryTileset).metatiles;
        metatileId -= NUM_METATILES_IN_PRIMARY;
    }
    DrawMetatile(
        MapGridGetMetatileLayerTypeAt(x, y) as i32,
        metatiles.at(metatileId as i32 * NUM_TILES_PER_METATILE),
        offset,
    );
}
pub(crate) unsafe extern "C" fn DrawMetatile(metatileLayerType: i32, tiles: *mut u16, offset: u16) {
    match metatileLayerType {
        METATILE_LAYER_TYPE_SPLIT => {
            *gOverworldTilemapBuffer_Bg3.at(offset) = *tiles;
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 1) = *tiles.at(1);
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x20) = *tiles.at(2);
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x21) = *tiles.at(3);
            *gOverworldTilemapBuffer_Bg2.at(offset) = 0;
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 1) = 0;
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x20) = 0;
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x21) = 0;
            *gOverworldTilemapBuffer_Bg1.at(offset) = *tiles.at(4);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 1) = *tiles.at(5);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x20) = *tiles.at(6);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x21) = *tiles.at(7);
        }
        METATILE_LAYER_TYPE_COVERED => {
            *gOverworldTilemapBuffer_Bg3.at(offset) = *tiles;
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 1) = *tiles.at(1);
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x20) = *tiles.at(2);
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x21) = *tiles.at(3);
            *gOverworldTilemapBuffer_Bg2.at(offset) = *tiles.at(4);
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 1) = *tiles.at(5);
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x20) = *tiles.at(6);
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x21) = *tiles.at(7);
            *gOverworldTilemapBuffer_Bg1.at(offset) = 0;
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 1) = 0;
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x20) = 0;
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x21) = 0;
        }
        0 => {
            *gOverworldTilemapBuffer_Bg3.at(offset) = 0x3014;
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 1) = 0x3014;
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x20) = 0x3014;
            *gOverworldTilemapBuffer_Bg3.at(offset as i32 + 0x21) = 0x3014;
            *gOverworldTilemapBuffer_Bg2.at(offset) = *tiles;
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 1) = *tiles.at(1);
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x20) = *tiles.at(2);
            *gOverworldTilemapBuffer_Bg2.at(offset as i32 + 0x21) = *tiles.at(3);
            *gOverworldTilemapBuffer_Bg1.at(offset) = *tiles.at(4);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 1) = *tiles.at(5);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x20) = *tiles.at(6);
            *gOverworldTilemapBuffer_Bg1.at(offset as i32 + 0x21) = *tiles.at(7);
        }
        _ => {}
    }
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
}
pub(crate) unsafe extern "C" fn MapPosToBgTilemapOffset(
    cameraOffset: *mut FieldCameraOffset,
    mut x: i32,
    mut y: i32,
) -> i32 {
    x -= (*gSaveBlock1Ptr).pos.x as i32;
    x *= 2;
    if x >= 32 || x < 0 {
        return -1;
    }
    x = x + (*cameraOffset).xTileOffset as i32;
    if x >= 32 {
        x -= 32;
    }
    y = (y - (*gSaveBlock1Ptr).pos.y as i32) * 2;
    if y >= 32 || y < 0 {
        return -1;
    }
    y = y + (*cameraOffset).yTileOffset as i32;
    if y >= 32 {
        y -= 32;
    }
    return y * 32 + x;
}
pub(crate) unsafe extern "C" fn CameraUpdateCallback(fieldCamera: *mut CameraObject) {
    if (*fieldCamera).spriteId != 0 {
        (*fieldCamera).movementSpeedX = gSprites[(*fieldCamera).spriteId].data[2] as i32;
        (*fieldCamera).movementSpeedY = gSprites[(*fieldCamera).spriteId].data[3] as i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetCameraUpdateInfo() {
    gFieldCamera.movementSpeedX = 0;
    gFieldCamera.movementSpeedY = 0;
    gFieldCamera.x = 0;
    gFieldCamera.y = 0;
    gFieldCamera.spriteId = 0;
    gFieldCamera.callback = None;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitCameraUpdateCallback(trackedSpriteId: u8) -> u32 {
    if gFieldCamera.spriteId != 0 {
        DestroySprite(&raw mut gSprites[gFieldCamera.spriteId]);
    }
    gFieldCamera.spriteId = AddCameraObject(trackedSpriteId) as u32;
    gFieldCamera.callback = Some(CameraUpdateCallback);
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CameraUpdate() {
    let mut deltaX: i32 = 0;
    let mut deltaY: i32 = 0;
    let mut curMovementOffsetY: i32 = 0;
    let mut curMovementOffsetX: i32 = 0;
    let mut movementSpeedX: i32 = 0;
    let mut movementSpeedY: i32 = 0;
    if gFieldCamera.callback.is_some() {
        gFieldCamera.callback.unwrap_unchecked()(&raw mut gFieldCamera);
    }
    movementSpeedX = gFieldCamera.movementSpeedX;
    movementSpeedY = gFieldCamera.movementSpeedY;
    deltaX = 0;
    deltaY = 0;
    curMovementOffsetX = gFieldCamera.x;
    curMovementOffsetY = gFieldCamera.y;
    if curMovementOffsetX == 0 && movementSpeedX != 0 {
        if movementSpeedX > 0 {
            deltaX = 1;
        } else {
            deltaX = -1;
        }
    }
    if curMovementOffsetY == 0 && movementSpeedY != 0 {
        if movementSpeedY > 0 {
            deltaY = 1;
        } else {
            deltaY = -1;
        }
    }
    if curMovementOffsetX != 0 && curMovementOffsetX == -movementSpeedX {
        if movementSpeedX > 0 {
            deltaX = 1;
        } else {
            deltaX = -1;
        }
    }
    if curMovementOffsetY != 0 && curMovementOffsetY == -movementSpeedY {
        if movementSpeedY > 0 {
            deltaX = 1;
        } else {
            deltaX = -1;
        }
    }
    gFieldCamera.x += movementSpeedX;
    gFieldCamera.x = gFieldCamera.x % 16;
    gFieldCamera.y += movementSpeedY;
    gFieldCamera.y = gFieldCamera.y % 16;
    if deltaX != 0 || deltaY != 0 {
        CameraMove(deltaX, deltaY);
        UpdateObjectEventsForCameraUpdate(deltaX as i16, deltaY as i16);
        RotatingGatePuzzleCameraUpdate(deltaX as i16, deltaY as i16);
        SetBerryTreesSeen();
        AddCameraTileOffset(
            &raw mut sFieldCameraOffset,
            deltaX as u32 * 2,
            deltaY as u32 * 2,
        );
        RedrawMapSlicesForCameraUpdate(&raw mut sFieldCameraOffset, deltaX * 2, deltaY * 2);
    }
    AddCameraPixelOffset(
        &raw mut sFieldCameraOffset,
        movementSpeedX as u32,
        movementSpeedY as u32,
    );
    gTotalCameraPixelOffsetX -= movementSpeedX as u16;
    gTotalCameraPixelOffsetY -= movementSpeedY as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveCameraAndRedrawMap(deltaX: i32, deltaY: i32) {
    CameraMove(deltaX, deltaY);
    UpdateObjectEventsForCameraUpdate(deltaX as i16, deltaY as i16);
    DrawWholeMapView();
    gTotalCameraPixelOffsetX -= deltaX as u16 * 16;
    gTotalCameraPixelOffsetY -= deltaY as u16 * 16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraPanningCallback(callback: Option<unsafe extern "C" fn()>) {
    sFieldCameraPanningCallback = callback;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraPanning(horizontal: i16, vertical: i16) {
    sHorizontalCameraPan = horizontal;
    sVerticalCameraPan = vertical + 32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InstallCameraPanAheadCallback() {
    sFieldCameraPanningCallback = Some(CameraPanningCB_PanAhead);
    sBikeCameraPanFlag = FALSE;
    sHorizontalCameraPan = 0;
    sVerticalCameraPan = 32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCameraPanning() {
    if sFieldCameraPanningCallback.is_some() {
        sFieldCameraPanningCallback.unwrap_unchecked()();
    }
    gSpriteCoordOffsetX = gTotalCameraPixelOffsetX as i16 - sHorizontalCameraPan;
    gSpriteCoordOffsetY = gTotalCameraPixelOffsetY as i16 - sVerticalCameraPan - 8;
}
pub(crate) unsafe extern "C" fn CameraPanningCB_PanAhead() {
    let mut var: u8 = 0;
    if gUnusedBikeCameraAheadPanback == FALSE {
        InstallCameraPanAheadCallback();
    } else {
        if gPlayerAvatar.tileTransitionState == T_TILE_TRANSITION {
            sBikeCameraPanFlag ^= 1;
            if sBikeCameraPanFlag == FALSE {
                return;
            }
        } else {
            sBikeCameraPanFlag = FALSE;
        }
        var = GetPlayerMovementDirection();
        if var == 2 {
            if sVerticalCameraPan > -8 {
                sVerticalCameraPan -= 2;
            }
        } else if var == 1 {
            if sVerticalCameraPan < 72 {
                sVerticalCameraPan += 2;
            }
        } else if sVerticalCameraPan < 32 {
            sVerticalCameraPan += 2;
        } else if sVerticalCameraPan > 32 {
            sVerticalCameraPan -= 2;
        }
    }
}
