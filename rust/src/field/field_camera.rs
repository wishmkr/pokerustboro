//! Translated from `src/field_camera.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::berry::SetBerryTreesSeen;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::{AddCameraObject, UpdateObjectEventsForCameraUpdate};
use crate::field_player_avatar::{GetPlayerMovementDirection, gPlayerAvatar};
use crate::fieldmap::{
    CameraMove, MapGridGetMetatileIdAt, MapGridGetMetatileLayerTypeAt, gMapHeader,
};
use crate::gpu_regs::SetGpuReg;
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::ScheduleBgCopyTilemapToVram;
use crate::overworld::{
    gOverworldTilemapBuffer_Bg1, gOverworldTilemapBuffer_Bg2, gOverworldTilemapBuffer_Bg3,
};
use crate::rotating_gate::RotatingGatePuzzleCameraUpdate;
use crate::sprite::gSprites;
use crate::sprite::{gSpriteCoordOffsetX, gSpriteCoordOffsetY};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}

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

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gUnusedBikeCameraAheadPanback: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sFieldCameraOffset: crate::global::Global<FieldCameraOffset> =
    crate::global::Global::new(unsafe { zeroed() });
pub(crate) static sHorizontalCameraPan: crate::global::Global<i16> = crate::global::Global::new(0);
pub(crate) static sVerticalCameraPan: crate::global::Global<i16> = crate::global::Global::new(0);
pub(crate) static sBikeCameraPanFlag: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sFieldCameraPanningCallback: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gFieldCamera: crate::global::Global<CameraObject> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gTotalCameraPixelOffsetY: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gTotalCameraPixelOffsetX: crate::global::Global<u16> = crate::global::Global::new(0);

unsafe fn ResetCameraOffset(cameraOffset: *mut FieldCameraOffset) {
    (*cameraOffset).xTileOffset = 0;
    (*cameraOffset).yTileOffset = 0;
    (*cameraOffset).xPixelOffset = 0;
    (*cameraOffset).yPixelOffset = 0;
    (*cameraOffset).copyBGToVRAM = TRUE;
}
unsafe fn AddCameraTileOffset(cameraOffset: *mut FieldCameraOffset, xOffset: u32, yOffset: u32) {
    (*cameraOffset).xTileOffset += xOffset as u8;
    (*cameraOffset).xTileOffset = ((*cameraOffset).xTileOffset as i32 % 32) as u8;
    (*cameraOffset).yTileOffset += yOffset as u8;
    (*cameraOffset).yTileOffset = ((*cameraOffset).yTileOffset as i32 % 32) as u8;
}
unsafe fn AddCameraPixelOffset(cameraOffset: *mut FieldCameraOffset, xOffset: u32, yOffset: u32) {
    (*cameraOffset).xPixelOffset += xOffset as u8;
    (*cameraOffset).yPixelOffset += yOffset as u8;
}
pub unsafe fn ResetFieldCamera() {
    ResetCameraOffset(crate::gl!(&raw sFieldCameraOffset));
}
pub unsafe fn FieldUpdateBgTilemapScroll() {
    let r5: u32 =
        crate::gl!(sFieldCameraOffset.xPixelOffset) as u32 + sHorizontalCameraPan.get() as u32;
    let r4: u32 =
        sVerticalCameraPan.get() as u32 + crate::gl!(sFieldCameraOffset.yPixelOffset) as u32 + 8;
    SetGpuReg(REG_OFFSET_BG1HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG1VOFS, r4 as u16);
    SetGpuReg(REG_OFFSET_BG2HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG2VOFS, r4 as u16);
    SetGpuReg(REG_OFFSET_BG3HOFS, r5 as u16);
    SetGpuReg(REG_OFFSET_BG3VOFS, r4 as u16);
}
pub unsafe fn GetCameraOffsetWithPan(x: *mut i16, y: *mut i16) {
    *x = crate::gl!(sFieldCameraOffset.xPixelOffset) as i16 + sHorizontalCameraPan.get();
    *y = crate::gl!(sFieldCameraOffset.yPixelOffset) as i16 + sVerticalCameraPan.get() + 8;
}
#[unsafe(no_mangle)]
pub unsafe fn DrawWholeMapView() {
    DrawWholeMapViewInternal(
        (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32,
        (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32,
        crate::gl!(gMapHeader.mapLayout),
    );
    crate::gl!(sFieldCameraOffset.copyBGToVRAM = TRUE);
}
unsafe fn DrawWholeMapViewInternal(x: i32, y: i32, mapLayout: *mut MapLayout) {
    let mut j: u8 = 0;
    let mut r6: u32 = 0;
    let mut temp: u8 = 0;
    let mut i: u8 = 0;
    while i < 32 {
        temp = crate::gl!(sFieldCameraOffset.yTileOffset) + i;
        if temp >= 32 {
            temp -= 32;
        }
        r6 = temp as u32 * 32;
        j = 0;
        while j < 32 {
            temp = crate::gl!(sFieldCameraOffset.xTileOffset) + j;
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
unsafe fn RedrawMapSlicesForCameraUpdate(cameraOffset: *mut FieldCameraOffset, x: i32, y: i32) {
    let mapLayout: *mut MapLayout = crate::gl!(gMapHeader.mapLayout);
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
unsafe fn RedrawMapSliceNorth(cameraOffset: *mut FieldCameraOffset, mapLayout: *mut MapLayout) {
    let mut temp: u8 = (*cameraOffset).yTileOffset + 28;
    if temp >= 32 {
        temp -= 32;
    }
    let r7: u32 = temp as u32 * 32;
    let mut i: u8 = 0;
    while i < 32 {
        temp = (*cameraOffset).xTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            r7 as u16 + temp as u16,
            (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32 + i as i32 / 2,
            (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32 + 14,
        );
        i += 2;
    }
}
unsafe fn RedrawMapSliceSouth(cameraOffset: *mut FieldCameraOffset, mapLayout: *mut MapLayout) {
    let mut temp: u8 = 0;
    let r7: u32 = (*cameraOffset).yTileOffset as u32 * 32;
    let mut i: u8 = 0;
    while i < 32 {
        temp = (*cameraOffset).xTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            r7 as u16 + temp as u16,
            (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32 + i as i32 / 2,
            (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32,
        );
        i += 2;
    }
}
unsafe fn RedrawMapSliceEast(cameraOffset: *mut FieldCameraOffset, mapLayout: *mut MapLayout) {
    let mut temp: u8 = 0;
    let r6: u32 = (*cameraOffset).xTileOffset as u32;
    let mut i: u8 = 0;
    while i < 32 {
        temp = (*cameraOffset).yTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            temp as u16 * 32 + r6 as u16,
            (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32,
            (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32 + i as i32 / 2,
        );
        i += 2;
    }
}
unsafe fn RedrawMapSliceWest(cameraOffset: *mut FieldCameraOffset, mapLayout: *mut MapLayout) {
    let mut temp: u8 = 0;
    let mut r5: u8 = (*cameraOffset).xTileOffset + 28;
    if r5 >= 32 {
        r5 -= 32;
    }
    let mut i: u8 = 0;
    while i < 32 {
        temp = (*cameraOffset).yTileOffset + i;
        if temp >= 32 {
            temp -= 32;
        }
        DrawMetatileAt(
            mapLayout,
            temp as u16 * 32 + r5 as u16,
            (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32 + 14,
            (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32 + i as i32 / 2,
        );
        i += 2;
    }
}
pub unsafe fn CurrentMapDrawMetatileAt(x: i32, y: i32) {
    let offset: i32 = MapPosToBgTilemapOffset(crate::gl!(&raw sFieldCameraOffset), x, y);
    if offset >= 0 {
        DrawMetatileAt(crate::gl!(gMapHeader.mapLayout), offset as u16, x, y);
        crate::gl!(sFieldCameraOffset.copyBGToVRAM = TRUE);
    }
}
pub unsafe fn DrawDoorMetatileAt(x: i32, y: i32, tiles: *mut u16) {
    let offset: i32 = MapPosToBgTilemapOffset(crate::gl!(&raw sFieldCameraOffset), x, y);
    if offset >= 0 {
        DrawMetatile(METATILE_LAYER_TYPE_COVERED, tiles, offset as u16);
        crate::gl!(sFieldCameraOffset.copyBGToVRAM = TRUE);
    }
}
unsafe fn DrawMetatileAt(mapLayout: *mut MapLayout, offset: u16, x: i32, y: i32) {
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
unsafe fn DrawMetatile(metatileLayerType: i32, tiles: *mut u16, offset: u16) {
    match metatileLayerType {
        METATILE_LAYER_TYPE_SPLIT => {
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset) = *tiles;
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 1) = *tiles.at(1);
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(2);
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(3);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset) = 0;
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 1) = 0;
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x20) = 0;
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x21) = 0;
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset) = *tiles.at(4);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 1) = *tiles.at(5);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(6);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(7);
        }
        METATILE_LAYER_TYPE_COVERED => {
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset) = *tiles;
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 1) = *tiles.at(1);
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(2);
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(3);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset) = *tiles.at(4);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 1) = *tiles.at(5);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(6);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(7);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset) = 0;
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 1) = 0;
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x20) = 0;
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x21) = 0;
        }
        0 => {
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset) = 0x3014;
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 1) = 0x3014;
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x20) = 0x3014;
            *(*gOverworldTilemapBuffer_Bg3.as_ptr()).at(offset as i32 + 0x21) = 0x3014;
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset) = *tiles;
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 1) = *tiles.at(1);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(2);
            *(*gOverworldTilemapBuffer_Bg2.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(3);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset) = *tiles.at(4);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 1) = *tiles.at(5);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x20) = *tiles.at(6);
            *(*gOverworldTilemapBuffer_Bg1.as_ptr()).at(offset as i32 + 0x21) = *tiles.at(7);
        }
        _ => {}
    }
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
}
unsafe fn MapPosToBgTilemapOffset(
    cameraOffset: *mut FieldCameraOffset,
    mut x: i32,
    mut y: i32,
) -> i32 {
    x -= (*crate::gl!(gSaveBlock1Ptr)).pos.x as i32;
    x *= 2;
    if !(0..32).contains(&x) {
        return -1;
    }
    x += (*cameraOffset).xTileOffset as i32;
    if x >= 32 {
        x -= 32;
    }
    y = (y - (*crate::gl!(gSaveBlock1Ptr)).pos.y as i32) * 2;
    if !(0..32).contains(&y) {
        return -1;
    }
    y += (*cameraOffset).yTileOffset as i32;
    if y >= 32 {
        y -= 32;
    }
    y * 32 + x
}
pub(crate) unsafe fn CameraUpdateCallback(fieldCamera: *mut CameraObject) {
    if (*fieldCamera).spriteId != 0 {
        (*fieldCamera).movementSpeedX = ({
            let gl_i0 = (*fieldCamera).spriteId;
            crate::gl!(gSprites[gl_i0].data[2])
        }) as i32;
        (*fieldCamera).movementSpeedY = ({
            let gl_i1 = (*fieldCamera).spriteId;
            crate::gl!(gSprites[gl_i1].data[3])
        }) as i32;
    }
}
pub fn ResetCameraUpdateInfo() {
    crate::gl!(gFieldCamera.movementSpeedX = 0);
    crate::gl!(gFieldCamera.movementSpeedY = 0);
    crate::gl!(gFieldCamera.x = 0);
    crate::gl!(gFieldCamera.y = 0);
    crate::gl!(gFieldCamera.spriteId = 0);
    crate::gl!(gFieldCamera.callback = None);
}
pub unsafe fn InitCameraUpdateCallback(trackedSpriteId: u8) -> u32 {
    if crate::gl!(gFieldCamera.spriteId) != 0 {
        DestroySprite({
            let gl_i2 = crate::gl!(gFieldCamera.spriteId);
            crate::gl!(&raw gSprites[gl_i2])
        });
    }
    crate::gl!(gFieldCamera.spriteId = AddCameraObject(trackedSpriteId) as u32);
    crate::gl!(gFieldCamera.callback = Some(CameraUpdateCallback));
    0
}
pub unsafe fn CameraUpdate() {
    let mut movementSpeedX: i32 = 0;
    let mut movementSpeedY: i32 = 0;
    if (*gFieldCamera.as_ptr()).callback.is_some() {
        (*gFieldCamera.as_ptr()).callback.unwrap_unchecked()(crate::gl!(&raw gFieldCamera));
    }
    movementSpeedX = crate::gl!(gFieldCamera.movementSpeedX);
    movementSpeedY = crate::gl!(gFieldCamera.movementSpeedY);
    let mut deltaX: i32 = 0;
    let mut deltaY: i32 = 0;
    let curMovementOffsetX: i32 = crate::gl!(gFieldCamera.x);
    let curMovementOffsetY: i32 = crate::gl!(gFieldCamera.y);
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
    crate::gl!(gFieldCamera.x += movementSpeedX);
    crate::gl!(gFieldCamera.x %= 16);
    crate::gl!(gFieldCamera.y += movementSpeedY);
    crate::gl!(gFieldCamera.y %= 16);
    if deltaX != 0 || deltaY != 0 {
        CameraMove(deltaX, deltaY);
        UpdateObjectEventsForCameraUpdate(deltaX as i16, deltaY as i16);
        RotatingGatePuzzleCameraUpdate(deltaX as i16, deltaY as i16);
        SetBerryTreesSeen();
        AddCameraTileOffset(
            crate::gl!(&raw sFieldCameraOffset),
            deltaX as u32 * 2,
            deltaY as u32 * 2,
        );
        RedrawMapSlicesForCameraUpdate(crate::gl!(&raw sFieldCameraOffset), deltaX * 2, deltaY * 2);
    }
    AddCameraPixelOffset(
        crate::gl!(&raw sFieldCameraOffset),
        movementSpeedX as u32,
        movementSpeedY as u32,
    );
    gTotalCameraPixelOffsetX.set(gTotalCameraPixelOffsetX.get() - (movementSpeedX as u16));
    gTotalCameraPixelOffsetY.set(gTotalCameraPixelOffsetY.get() - (movementSpeedY as u16));
}
pub unsafe fn MoveCameraAndRedrawMap(deltaX: i32, deltaY: i32) {
    CameraMove(deltaX, deltaY);
    UpdateObjectEventsForCameraUpdate(deltaX as i16, deltaY as i16);
    DrawWholeMapView();
    gTotalCameraPixelOffsetX.set(gTotalCameraPixelOffsetX.get() - (deltaX as u16 * 16));
    gTotalCameraPixelOffsetY.set(gTotalCameraPixelOffsetY.get() - (deltaY as u16 * 16));
}
#[unsafe(no_mangle)]
pub fn SetCameraPanningCallback(callback: Option<unsafe fn()>) {
    crate::gl!(sFieldCameraPanningCallback = callback);
}
#[unsafe(no_mangle)]
pub unsafe fn SetCameraPanning(horizontal: i16, vertical: i16) {
    sHorizontalCameraPan.set(horizontal);
    sVerticalCameraPan.set(vertical + 32);
}
#[unsafe(no_mangle)]
pub fn InstallCameraPanAheadCallback() {
    crate::gl!(sFieldCameraPanningCallback = Some(CameraPanningCB_PanAhead));
    sBikeCameraPanFlag.set(FALSE);
    sHorizontalCameraPan.set(0);
    sVerticalCameraPan.set(32);
}
pub unsafe fn UpdateCameraPanning() {
    if (*sFieldCameraPanningCallback.as_ptr()).is_some() {
        (*sFieldCameraPanningCallback.as_ptr()).unwrap_unchecked()();
    }
    gSpriteCoordOffsetX.set(gTotalCameraPixelOffsetX.get() as i16 - sHorizontalCameraPan.get());
    gSpriteCoordOffsetY.set(gTotalCameraPixelOffsetY.get() as i16 - sVerticalCameraPan.get() - 8);
}
pub(crate) unsafe fn CameraPanningCB_PanAhead() {
    let mut var: u8 = 0;
    if gUnusedBikeCameraAheadPanback.get() == FALSE {
        InstallCameraPanAheadCallback();
    } else {
        if crate::gl!(gPlayerAvatar.tileTransitionState) == T_TILE_TRANSITION {
            sBikeCameraPanFlag.set(sBikeCameraPanFlag.get() ^ 1);
            if sBikeCameraPanFlag.get() == FALSE {
                return;
            }
        } else {
            sBikeCameraPanFlag.set(FALSE);
        }
        var = GetPlayerMovementDirection();
        if var == 2 {
            if sVerticalCameraPan.get() > -8 {
                sVerticalCameraPan.set(sVerticalCameraPan.get() - 2);
            }
        } else if var == 1 {
            if sVerticalCameraPan.get() < 72 {
                sVerticalCameraPan.set(sVerticalCameraPan.get() + 2);
            }
        } else if sVerticalCameraPan.get() < 32 {
            sVerticalCameraPan.set(sVerticalCameraPan.get() + 2);
        } else if sVerticalCameraPan.get() > 32 {
            sVerticalCameraPan.set(sVerticalCameraPan.get() - 2);
        }
    }
}
