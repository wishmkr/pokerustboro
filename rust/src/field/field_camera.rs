//! Translated from `src/field_camera.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedBikeCameraAheadPanback: u8 = 0u8;
pub(crate) static mut sFieldCameraOffset: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
pub(crate) static mut sHorizontalCameraPan: i16 = 0i16;
pub(crate) static mut sVerticalCameraPan: i16 = 0i16;
pub(crate) static mut sBikeCameraPanFlag: u8 = 0u8;
pub(crate) static mut sFieldCameraPanningCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCamera: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTotalCameraPixelOffsetY: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTotalCameraPixelOffsetX: u16 = 0u16;

unsafe extern "C" {
    static mut gMapHeader: u8;
    static mut gOverworldTilemapBuffer_Bg1: u8;
    static mut gOverworldTilemapBuffer_Bg2: u8;
    static mut gOverworldTilemapBuffer_Bg3: u8;
    static mut gPlayerAvatar: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    fn AddCameraObject(a0: u8) -> u8;
    fn CameraMove(a0: i32, a1: i32) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn GetPlayerMovementDirection() -> u8;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileLayerTypeAt(a0: i32, a1: i32) -> u8;
    fn RotatingGatePuzzleCameraUpdate(a0: i16, a1: i16);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBerryTreesSeen();
    fn SetGpuReg(a0: u8, a1: u16);
    fn UpdateObjectEventsForCameraUpdate(a0: i16, a1: i16);
}

pub(crate) unsafe extern "C" fn ResetCameraOffset(cameraOffset: *mut u8) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        ((cameraOffset).wrapping_add(2)).write(0u8);
        ((cameraOffset).wrapping_add(3)).write(0u8);
        (cameraOffset).write(0u8);
        ((cameraOffset).wrapping_add(1)).write(0u8);
        ((cameraOffset).wrapping_add(4)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn AddCameraTileOffset(
    cameraOffset: *mut u8,
    xOffset: u32,
    yOffset: u32,
) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut xOffset = xOffset;
        let mut yOffset = yOffset;
        let __p1 = (cameraOffset).wrapping_add(2);
        (__p1).write((((((__p1).read()) as u32).wrapping_add(xOffset)) as u8));
        let __p2 = (cameraOffset).wrapping_add(2);
        (__p2).write(((crate::c::rem_i32((((__p2).read()) as i32), 32i32)) as u8));
        let __p3 = (cameraOffset).wrapping_add(3);
        (__p3).write((((((__p3).read()) as u32).wrapping_add(yOffset)) as u8));
        let __p4 = (cameraOffset).wrapping_add(3);
        (__p4).write(((crate::c::rem_i32((((__p4).read()) as i32), 32i32)) as u8));
    }
}
pub(crate) unsafe extern "C" fn AddCameraPixelOffset(
    cameraOffset: *mut u8,
    xOffset: u32,
    yOffset: u32,
) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut xOffset = xOffset;
        let mut yOffset = yOffset;
        let __p1 = (cameraOffset);
        (__p1).write((((((__p1).read()) as u32).wrapping_add(xOffset)) as u8));
        let __p2 = (cameraOffset).wrapping_add(1);
        (__p2).write((((((__p2).read()) as u32).wrapping_add(yOffset)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFieldCamera() {
    unsafe {
        ResetCameraOffset((&raw mut sFieldCameraOffset).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldUpdateBgTilemapScroll() {
    unsafe {
        let mut r4: u32 = 0u32;
        let mut r5: u32 = 0u32;
        r5 = ((((((&raw mut sFieldCameraOffset).cast::<u8>()).read()) as i32).wrapping_add(
            ((((&raw mut sHorizontalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32),
        )) as u32);
        r4 = (((((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32)
            .wrapping_add(
                (((((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(1)).read()) as i32),
            ))
        .wrapping_add(8i32)) as u32);
        SetGpuReg(20u8, ((r5) as u16));
        SetGpuReg(22u8, ((r4) as u16));
        SetGpuReg(24u8, ((r5) as u16));
        SetGpuReg(26u8, ((r4) as u16));
        SetGpuReg(28u8, ((r5) as u16));
        SetGpuReg(30u8, ((r4) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCameraOffsetWithPan(x: *mut i16, y: *mut i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((&raw mut sFieldCameraOffset).cast::<u8>()).read()) as i32).wrapping_add(
                ((((&raw mut sHorizontalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32),
            )) as i16),
        );
        (y).write(
            ((((((((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(1)).read()) as i32)
                .wrapping_add(
                    ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32),
                ))
            .wrapping_add(8i32)) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawWholeMapView() {
    unsafe {
        DrawWholeMapViewInternal(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32),
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
            (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
        );
        (((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(4)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn DrawWholeMapViewInternal(x: i32, y: i32, mapLayout: *mut u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut mapLayout = mapLayout;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut r6: u32 = 0u32;
        let mut temp: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    temp = (((((((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(3))
                        .read()) as i32)
                        .wrapping_add(((i) as i32))) as u8);
                    if ((temp) as i32) >= 32i32 {
                        temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                    }
                    r6 = ((((temp) as i32).wrapping_mul(32i32)) as u32);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                temp = (((((((&raw mut sFieldCameraOffset).cast::<u8>())
                                    .wrapping_add(2))
                                .read()) as i32)
                                    .wrapping_add(((j) as i32)))
                                    as u8);
                                if ((temp) as i32) >= 32i32 {
                                    temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                                }
                                DrawMetatileAt(
                                    mapLayout,
                                    (((r6).wrapping_add(((temp) as u32))) as u16),
                                    (x).wrapping_add(crate::c::div_i32(((j) as i32), 2i32)),
                                    (y).wrapping_add(crate::c::div_i32(((i) as i32), 2i32)),
                                );
                            }
                            j = ((((j) as i32).wrapping_add(2i32)) as u8);
                        }
                    }
                }
                i = ((((i) as i32).wrapping_add(2i32)) as u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSlicesForCameraUpdate(
    cameraOffset: *mut u8,
    x: i32,
    y: i32,
) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut x = x;
        let mut y = y;
        let mut mapLayout: *mut u8 =
            (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read();
        if x > 0i32 {
            RedrawMapSliceWest(cameraOffset, mapLayout);
        }
        if x < 0i32 {
            RedrawMapSliceEast(cameraOffset, mapLayout);
        }
        if y > 0i32 {
            RedrawMapSliceNorth(cameraOffset, mapLayout);
        }
        if y < 0i32 {
            RedrawMapSliceSouth(cameraOffset, mapLayout);
        }
        ((cameraOffset).wrapping_add(4)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceNorth(cameraOffset: *mut u8, mapLayout: *mut u8) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut mapLayout = mapLayout;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        let mut r7: u32 = 0u32;
        temp = ((((((cameraOffset).wrapping_add(3)).read()) as i32).wrapping_add(28i32)) as u8);
        if ((temp) as i32) >= 32i32 {
            temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
        }
        r7 = ((((temp) as i32).wrapping_mul(32i32)) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((((((cameraOffset).wrapping_add(2)).read()) as i32)
                        .wrapping_add(((i) as i32))) as u8);
                    if ((temp) as i32) >= 32i32 {
                        temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                    }
                    DrawMetatileAt(
                        mapLayout,
                        (((r7).wrapping_add(((temp) as u32))) as u16),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                            .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((i) as i32), 2i32)),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(14i32),
                    );
                }
                i = ((((i) as i32).wrapping_add(2i32)) as u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceSouth(cameraOffset: *mut u8, mapLayout: *mut u8) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut mapLayout = mapLayout;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        let mut r7: u32 =
            ((((((cameraOffset).wrapping_add(3)).read()) as i32).wrapping_mul(32i32)) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((((((cameraOffset).wrapping_add(2)).read()) as i32)
                        .wrapping_add(((i) as i32))) as u8);
                    if ((temp) as i32) >= 32i32 {
                        temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                    }
                    DrawMetatileAt(
                        mapLayout,
                        (((r7).wrapping_add(((temp) as u32))) as u16),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                            .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((i) as i32), 2i32)),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as i32),
                    );
                }
                i = ((((i) as i32).wrapping_add(2i32)) as u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceEast(cameraOffset: *mut u8, mapLayout: *mut u8) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut mapLayout = mapLayout;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        let mut r6: u32 = ((((cameraOffset).wrapping_add(2)).read()) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((((((cameraOffset).wrapping_add(3)).read()) as i32)
                        .wrapping_add(((i) as i32))) as u8);
                    if ((temp) as i32) >= 32i32 {
                        temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                    }
                    DrawMetatileAt(
                        mapLayout,
                        ((((((temp) as i32).wrapping_mul(32i32)) as u32).wrapping_add(r6)) as u16),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                            .read()) as i32),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((i) as i32), 2i32)),
                    );
                }
                i = ((((i) as i32).wrapping_add(2i32)) as u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawMapSliceWest(cameraOffset: *mut u8, mapLayout: *mut u8) {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut mapLayout = mapLayout;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        let mut r5: u8 =
            ((((((cameraOffset).wrapping_add(2)).read()) as i32).wrapping_add(28i32)) as u8);
        if ((r5) as i32) >= 32i32 {
            r5 = ((((r5) as i32).wrapping_sub(32i32)) as u8);
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((((((cameraOffset).wrapping_add(3)).read()) as i32)
                        .wrapping_add(((i) as i32))) as u8);
                    if ((temp) as i32) >= 32i32 {
                        temp = ((((temp) as i32).wrapping_sub(32i32)) as u8);
                    }
                    DrawMetatileAt(
                        mapLayout,
                        (((((temp) as i32).wrapping_mul(32i32)).wrapping_add(((r5) as i32)))
                            as u16),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
                            .read()) as i32)
                            .wrapping_add(14i32),
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(crate::c::div_i32(((i) as i32), 2i32)),
                    );
                }
                i = ((((i) as i32).wrapping_add(2i32)) as u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurrentMapDrawMetatileAt(x: i32, y: i32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut offset: i32 =
            MapPosToBgTilemapOffset((&raw mut sFieldCameraOffset).cast::<u8>(), x, y);
        if offset >= 0i32 {
            DrawMetatileAt(
                (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                ((offset) as u16),
                x,
                y,
            );
            (((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(4)).write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawDoorMetatileAt(x: i32, y: i32, tiles: *mut u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut tiles = tiles;
        let mut offset: i32 =
            MapPosToBgTilemapOffset((&raw mut sFieldCameraOffset).cast::<u8>(), x, y);
        if offset >= 0i32 {
            DrawMetatile(1i32, tiles, ((offset) as u16));
            (((&raw mut sFieldCameraOffset).cast::<u8>()).wrapping_add(4)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawMetatileAt(mapLayout: *mut u8, offset: u16, x: i32, y: i32) {
    unsafe {
        let mut mapLayout = mapLayout;
        let mut offset = offset;
        let mut x = x;
        let mut y = y;
        let mut metatileId: u16 = ((MapGridGetMetatileIdAt(x, y)) as u16);
        let mut metatiles: *mut u16 = core::ptr::null_mut();
        if ((metatileId) as i32) > 1024i32 {
            metatileId = 0u16;
        }
        if ((metatileId) as i32) < 512i32 {
            metatiles = ((((mapLayout).wrapping_add(16).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u16>())
            .read();
        } else {
            metatiles = ((((mapLayout).wrapping_add(20).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u16>())
            .read();
            metatileId = ((((metatileId) as i32).wrapping_sub(512i32)) as u16);
        }
        DrawMetatile(
            ((MapGridGetMetatileLayerTypeAt(x, y)) as i32),
            (metatiles).wrapping_offset((((metatileId) as i32).wrapping_mul(8i32)) as isize),
            offset,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawMetatile(metatileLayerType: i32, tiles: *mut u16, offset: u16) {
    unsafe {
        let mut metatileLayerType = metatileLayerType;
        let mut tiles = tiles;
        let mut offset = offset;
        'l1: {
            let __sw1 = metatileLayerType;
            if __sw1 == 2i32 {
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write((tiles).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(1)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(2)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(3)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(((tiles).wrapping_offset(4)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(5)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(6)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(7)).read());
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write((tiles).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(1)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(2)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(3)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(((tiles).wrapping_offset(4)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(5)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(6)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(7)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(0u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(0u16);
                break 'l1;
            }
            if __sw1 == 0i32 {
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(12308u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(12308u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(12308u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg3).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(12308u16);
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write((tiles).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(1)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(2)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg2).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(3)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset(((offset) as i32) as isize))
                .write(((tiles).wrapping_offset(4)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                .write(((tiles).wrapping_offset(5)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(32i32)) as isize))
                .write(((tiles).wrapping_offset(6)).read());
                ((((&raw mut gOverworldTilemapBuffer_Bg1).cast::<*mut u16>()).read())
                    .wrapping_offset((((offset) as i32).wrapping_add(33i32)) as isize))
                .write(((tiles).wrapping_offset(7)).read());
                break 'l1;
            }
        }
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn MapPosToBgTilemapOffset(
    cameraOffset: *mut u8,
    x: i32,
    y: i32,
) -> i32 {
    unsafe {
        let mut cameraOffset = cameraOffset;
        let mut x = x;
        let mut y = y;
        x = (x).wrapping_sub(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32),
        );
        x = (x).wrapping_mul(2i32);
        if (x >= 32i32) || (x < 0i32) {
            return (-1i32);
        }
        x = (x).wrapping_add(((((cameraOffset).wrapping_add(2)).read()) as i32));
        if x >= 32i32 {
            x = (x).wrapping_sub(32i32);
        }
        y = ((y).wrapping_sub(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        ))
        .wrapping_mul(2i32);
        if (y >= 32i32) || (y < 0i32) {
            return (-1i32);
        }
        y = (y).wrapping_add(((((cameraOffset).wrapping_add(3)).read()) as i32));
        if y >= 32i32 {
            y = (y).wrapping_sub(32i32);
        }
        return ((y).wrapping_mul(32i32)).wrapping_add(x);
    }
}
pub(crate) unsafe extern "C" fn CameraUpdateCallback(fieldCamera: *mut u8) {
    unsafe {
        let mut fieldCamera = fieldCamera;
        if ((fieldCamera).wrapping_add(4).cast::<u32>()).read() != 0u32 {
            ((fieldCamera).wrapping_add(8).cast::<i32>()).write(
                ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((fieldCamera).wrapping_add(4).cast::<u32>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32),
            );
            ((fieldCamera).wrapping_add(12).cast::<i32>()).write(
                ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((fieldCamera).wrapping_add(4).cast::<u32>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetCameraUpdateInfo() {
    unsafe {
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(8)
            .cast::<i32>())
        .write(0i32);
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(16)
            .cast::<i32>())
        .write(0i32);
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(20)
            .cast::<i32>())
        .write(0i32);
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(0u32);
        (((&raw mut gFieldCamera).cast::<u8>()).cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitCameraUpdateCallback(trackedSpriteId: u8) -> u32 {
    unsafe {
        let mut trackedSpriteId = trackedSpriteId;
        if (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            != 0u32
        {
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gFieldCamera).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .read()) as i32) as isize
                        * 68,
                ),
            );
        }
        (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(((AddCameraObject(trackedSpriteId)) as u32));
        (((&raw mut gFieldCamera).cast::<u8>()).cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(CameraUpdateCallback));
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CameraUpdate() {
    unsafe {
        let mut deltaX: i32 = 0i32;
        let mut deltaY: i32 = 0i32;
        let mut curMovementOffsetY: i32 = 0i32;
        let mut curMovementOffsetX: i32 = 0i32;
        let mut movementSpeedX: i32 = 0i32;
        let mut movementSpeedY: i32 = 0i32;
        if core::mem::transmute::<_, usize>(
            (((&raw mut gFieldCamera).cast::<u8>())
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) != 0usize
        {
            ((((&raw mut gFieldCamera).cast::<u8>())
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read())
            .unwrap_unchecked()((&raw mut gFieldCamera).cast::<u8>());
        }
        movementSpeedX = (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(8)
            .cast::<i32>())
        .read();
        movementSpeedY = (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(12)
            .cast::<i32>())
        .read();
        deltaX = 0i32;
        deltaY = 0i32;
        curMovementOffsetX = (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(16)
            .cast::<i32>())
        .read();
        curMovementOffsetY = (((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(20)
            .cast::<i32>())
        .read();
        if (curMovementOffsetX == 0i32) && (movementSpeedX != 0i32) {
            if movementSpeedX > 0i32 {
                deltaX = 1i32;
            } else {
                deltaX = (-1i32);
            }
        }
        if (curMovementOffsetY == 0i32) && (movementSpeedY != 0i32) {
            if movementSpeedY > 0i32 {
                deltaY = 1i32;
            } else {
                deltaY = (-1i32);
            }
        }
        if (curMovementOffsetX != 0i32) && (curMovementOffsetX == (movementSpeedX).wrapping_neg()) {
            if movementSpeedX > 0i32 {
                deltaX = 1i32;
            } else {
                deltaX = (-1i32);
            }
        }
        if (curMovementOffsetY != 0i32) && (curMovementOffsetY == (movementSpeedY).wrapping_neg()) {
            if movementSpeedY > 0i32 {
                deltaX = 1i32;
            } else {
                deltaX = (-1i32);
            }
        }
        let __p1 = ((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(16)
            .cast::<i32>();
        (__p1).write(((__p1).read()).wrapping_add(movementSpeedX));
        let __p2 = ((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(16)
            .cast::<i32>();
        (__p2).write(crate::c::rem_i32((__p2).read(), 16i32));
        let __p3 = ((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(20)
            .cast::<i32>();
        (__p3).write(((__p3).read()).wrapping_add(movementSpeedY));
        let __p4 = ((&raw mut gFieldCamera).cast::<u8>())
            .wrapping_add(20)
            .cast::<i32>();
        (__p4).write(crate::c::rem_i32((__p4).read(), 16i32));
        if (deltaX != 0i32) || (deltaY != 0i32) {
            CameraMove(deltaX, deltaY);
            UpdateObjectEventsForCameraUpdate(((deltaX) as i16), ((deltaY) as i16));
            RotatingGatePuzzleCameraUpdate(((deltaX) as i16), ((deltaY) as i16));
            SetBerryTreesSeen();
            AddCameraTileOffset(
                (&raw mut sFieldCameraOffset).cast::<u8>(),
                (((deltaX).wrapping_mul(2i32)) as u32),
                (((deltaY).wrapping_mul(2i32)) as u32),
            );
            RedrawMapSlicesForCameraUpdate(
                (&raw mut sFieldCameraOffset).cast::<u8>(),
                (deltaX).wrapping_mul(2i32),
                (deltaY).wrapping_mul(2i32),
            );
        }
        AddCameraPixelOffset(
            (&raw mut sFieldCameraOffset).cast::<u8>(),
            ((movementSpeedX) as u32),
            ((movementSpeedY) as u32),
        );
        let __p5 = (&raw mut gTotalCameraPixelOffsetX)
            .cast::<u8>()
            .cast::<u16>();
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(movementSpeedX)) as u16));
        let __p6 = (&raw mut gTotalCameraPixelOffsetY)
            .cast::<u8>()
            .cast::<u16>();
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(movementSpeedY)) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveCameraAndRedrawMap(deltaX: i32, deltaY: i32) {
    unsafe {
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        CameraMove(deltaX, deltaY);
        UpdateObjectEventsForCameraUpdate(((deltaX) as i16), ((deltaY) as i16));
        DrawWholeMapView();
        let __p1 = (&raw mut gTotalCameraPixelOffsetX)
            .cast::<u8>()
            .cast::<u16>();
        (__p1)
            .write((((((__p1).read()) as i32).wrapping_sub((deltaX).wrapping_mul(16i32))) as u16));
        let __p2 = (&raw mut gTotalCameraPixelOffsetY)
            .cast::<u8>()
            .cast::<u16>();
        (__p2)
            .write((((((__p2).read()) as i32).wrapping_sub((deltaY).wrapping_mul(16i32))) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraPanningCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        ((&raw mut sFieldCameraPanningCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCameraPanning(horizontal: i16, vertical: i16) {
    unsafe {
        let mut horizontal = horizontal;
        let mut vertical = vertical;
        ((&raw mut sHorizontalCameraPan).cast::<u8>().cast::<i16>()).write(horizontal);
        ((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>())
            .write(((((vertical) as i32).wrapping_add(32i32)) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InstallCameraPanAheadCallback() {
    unsafe {
        ((&raw mut sFieldCameraPanningCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CameraPanningCB_PanAhead));
        ((&raw mut sBikeCameraPanFlag).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sHorizontalCameraPan).cast::<u8>().cast::<i16>()).write(0i16);
        ((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).write(32i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCameraPanning() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            ((&raw mut sFieldCameraPanningCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) != 0usize
        {
            (((&raw mut sFieldCameraPanningCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
        }
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(
            ((((((&raw mut gTotalCameraPixelOffsetX)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                .wrapping_sub(
                    ((((&raw mut sHorizontalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32),
                )) as i16),
        );
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(
            (((((((&raw mut gTotalCameraPixelOffsetY)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                .wrapping_sub(
                    ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32),
                ))
            .wrapping_sub(8i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn CameraPanningCB_PanAhead() {
    unsafe {
        let mut var: u8 = 0u8;
        if ((((&raw mut gUnusedBikeCameraAheadPanback)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 0i32
        {
            InstallCameraPanAheadCallback();
        } else {
            if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 1i32 {
                let __p1 = (&raw mut sBikeCameraPanFlag).cast::<u8>().cast::<u8>();
                (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
                if ((((&raw mut sBikeCameraPanFlag).cast::<u8>().cast::<u8>()).read()) as i32)
                    == 0i32
                {
                    return;
                }
            } else {
                ((&raw mut sBikeCameraPanFlag).cast::<u8>().cast::<u8>()).write(0u8);
            }
            var = GetPlayerMovementDirection();
            if ((var) as i32) == 2i32 {
                if ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32)
                    > (-8i32)
                {
                    let __p2 = (&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
                }
            } else {
                if ((var) as i32) == 1i32 {
                    if ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32)
                        < 72i32
                    {
                        let __p3 = (&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>();
                        (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
                    }
                } else {
                    if ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read()) as i32)
                        < 32i32
                    {
                        let __p4 = (&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>();
                        (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
                    } else {
                        if ((((&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>()).read())
                            as i32)
                            > 32i32
                        {
                            let __p5 = (&raw mut sVerticalCameraPan).cast::<u8>().cast::<i16>();
                            (__p5).write((((((__p5).read()) as i32).wrapping_sub(2i32)) as i16));
                        }
                    }
                }
            }
        }
    }
}
