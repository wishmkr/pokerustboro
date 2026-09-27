//! Translated from `src/field_effect_helpers.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sShadowEffectTemplateIds gShadowVerticalOffsets gFadeFootprintsTireTracksFuncs gAshFieldEffectFuncs sFigure8XOffsets sFigure8YOffsets
#[allow(unused_imports)]
use crate::data::field_effect_helpers::*;

unsafe extern "C" {
    static mut gCamera: u8;
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gDummySpriteAnimTable: u8;
    static mut gDummySpriteTemplate: u8;
    static mut gFieldEffectArguments: u8;
    static mut gFieldEffectObjectTemplatePointers: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gReflectionEffectPaletteMap: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    fn CreateCopySpriteAt(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn ElevationToPriority(a0: u8) -> u8;
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut u8, a1: u8);
    fn GetFigure8XOffset(a0: i16) -> i16;
    fn GetFigure8YOffset(a0: i16) -> i16;
    fn GetObjectEventGraphicsInfo(a0: u8) -> *mut u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetObjectPaletteTag(a0: u8) -> u16;
    fn LoadPlayerObjectReflectionPalette(a0: u16, a1: u8);
    fn LoadSpecialObjectReflectionPalette(a0: u16, a1: u8);
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MetatileBehavior_GetBridgeType(a0: u8) -> u8;
    fn MetatileBehavior_IsLongGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsPokeGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsReflective(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableWaterOrUnderwater(a0: u8) -> u8;
    fn MetatileBehavior_IsTallGrass(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn PatchObjectPalette(a0: u16, a1: u8);
    fn PlaySE(a0: u16);
    fn SeekSpriteAnim(a0: *mut u8, a1: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetObjectSubpriorityByElevation(a0: u8, a1: *mut u8, a2: u8);
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UpdateObjectEventSpriteInvisibility(a0: *mut u8, a1: u8);
    fn UpdateSpritePaletteWithWeather(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpReflection(
    objectEvent: *mut u8,
    sprite: *mut u8,
    stillReflection: u8,
) {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut sprite = sprite;
        let mut stillReflection = stillReflection;
        let mut reflectionSprite: *mut u8 = core::ptr::null_mut();
        reflectionSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((CreateCopySpriteAt(
                sprite,
                ((sprite).wrapping_add(32).cast::<i16>()).read(),
                ((sprite).wrapping_add(34).cast::<i16>()).read(),
                152u8,
            )) as i32) as isize
                * 68,
        );
        ((reflectionSprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(UpdateObjectReflectionSprite));
        crate::c::bf_write((reflectionSprite).wrapping_add(5), 2, 2, (3u16) as i32);
        crate::c::bf_write(
            (reflectionSprite).wrapping_add(5),
            4,
            4,
            (((((&raw mut gReflectionEffectPaletteMap).cast::<u8>()).wrapping_offset(
                ((crate::c::bf_read((reflectionSprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                    as isize,
            ))
            .read()) as u16) as i32,
        );
        crate::c::bf_write((reflectionSprite).wrapping_add(63), 6, 1, (1u16) as i32);
        ((reflectionSprite).wrapping_add(8).cast::<*mut *mut u8>())
            .write(((&raw mut gDummySpriteAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        StartSpriteAnim(reflectionSprite, 0u8);
        ((reflectionSprite).wrapping_add(16).cast::<*mut *mut u8>())
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        crate::c::bf_write((reflectionSprite).wrapping_add(63), 3, 1, (1u16) as i32);
        crate::c::bf_write((reflectionSprite).wrapping_add(66), 6, 2, (0u8) as i32);
        (((reflectionSprite).wrapping_add(46)).cast::<i16>())
            .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
        ((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((objectEvent).wrapping_add(8)).read()) as i16));
        ((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((stillReflection) as i16));
        LoadObjectReflectionPalette(objectEvent, reflectionSprite);
        if !((stillReflection) != 0) {
            crate::c::bf_write((reflectionSprite).wrapping_add(1), 0, 2, (1u32) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn GetReflectionVerticalOffset(objectEvent: *mut u8) -> i16 {
    unsafe {
        let mut objectEvent = objectEvent;
        return ((((((GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read()))
            .wrapping_add(10)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_sub(2i32)) as i16);
    }
}
pub(crate) unsafe extern "C" fn LoadObjectReflectionPalette(
    objectEvent: *mut u8,
    reflectionSprite: *mut u8,
) {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut reflectionSprite = reflectionSprite;
        let mut bridgeType: u8 = 0u8;
        let mut bridgeReflectionVerticalOffsets = crate::ffi::Align4([0u8; 6]);
        (&raw mut bridgeReflectionVerticalOffsets)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(12u16);
        (&raw mut bridgeReflectionVerticalOffsets)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(28u16);
        (&raw mut bridgeReflectionVerticalOffsets)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(44u16);
        ((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        if (!((crate::c::bf_read(
            (GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read())).wrapping_add(12),
            7,
            1,
            false,
        ) as u8)
            != 0))
            && ((({
                let __v1 = MetatileBehavior_GetBridgeType(((objectEvent).wrapping_add(31)).read());
                bridgeType = __v1;
                __v1
            }) != 0)
                || (({
                    let __v2 =
                        MetatileBehavior_GetBridgeType(((objectEvent).wrapping_add(30)).read());
                    bridgeType = __v2;
                    __v2
                }) != 0))
        {
            ((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                (((((&raw mut bridgeReflectionVerticalOffsets).cast::<u16>())
                    .wrapping_offset((((bridgeType) as i32).wrapping_sub(1i32)) as isize))
                .read()) as i16),
            );
            LoadObjectHighBridgeReflectionPalette(
                objectEvent,
                ((crate::c::bf_read((reflectionSprite).wrapping_add(5), 4, 4, false) as u16) as u8),
            );
        } else {
            LoadObjectRegularReflectionPalette(
                objectEvent,
                ((crate::c::bf_read((reflectionSprite).wrapping_add(5), 4, 4, false) as u16) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadObjectRegularReflectionPalette(
    objectEvent: *mut u8,
    paletteIndex: u8,
) {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut paletteIndex = paletteIndex;
        let mut graphicsInfo: *mut u8 =
            GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
        if ((((graphicsInfo).wrapping_add(4).cast::<u16>()).read()) as i32) != 4607i32 {
            if ((crate::c::bf_read((graphicsInfo).wrapping_add(12), 0, 4, false) as u8) as i32)
                == 0i32
            {
                LoadPlayerObjectReflectionPalette(
                    ((graphicsInfo).wrapping_add(2).cast::<u16>()).read(),
                    paletteIndex,
                );
            } else {
                if ((crate::c::bf_read((graphicsInfo).wrapping_add(12), 0, 4, false) as u8) as i32)
                    == 10i32
                {
                    LoadSpecialObjectReflectionPalette(
                        ((graphicsInfo).wrapping_add(2).cast::<u16>()).read(),
                        paletteIndex,
                    );
                } else {
                    PatchObjectPalette(GetObjectPaletteTag(paletteIndex), paletteIndex);
                }
            }
            UpdateSpritePaletteWithWeather(paletteIndex);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadObjectHighBridgeReflectionPalette(
    objectEvent: *mut u8,
    paletteNum: u8,
) {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut paletteNum = paletteNum;
        let mut graphicsInfo: *mut u8 =
            GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
        if ((((graphicsInfo).wrapping_add(4).cast::<u16>()).read()) as i32) != 4607i32 {
            PatchObjectPalette(
                ((graphicsInfo).wrapping_add(4).cast::<u16>()).read(),
                paletteNum,
            );
            UpdateSpritePaletteWithWeather(paletteNum);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateObjectReflectionSprite(reflectionSprite: *mut u8) {
    unsafe {
        let mut reflectionSprite = reflectionSprite;
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((reflectionSprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 36,
        );
        let mut mainSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
        if ((!((crate::c::bf_read((objectEvent).wrapping_add(0), 0, 1, false) as u32) != 0))
            || (!((crate::c::bf_read((objectEvent).wrapping_add(2), 1, 1, false) as u32) != 0)))
            || (((((objectEvent).wrapping_add(8)).read()) as i32)
                != ((((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32))
        {
            crate::c::bf_write((reflectionSprite).wrapping_add(62), 0, 1, (0u16) as i32);
        } else {
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(5),
                4,
                4,
                (((((&raw mut gReflectionEffectPaletteMap).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((mainSprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        as isize,
                ))
                .read()) as u16) as i32,
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(1),
                6,
                2,
                (crate::c::bf_read((mainSprite).wrapping_add(1), 6, 2, false) as u32) as i32,
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(3),
                6,
                2,
                (crate::c::bf_read((mainSprite).wrapping_add(3), 6, 2, false) as u32) as i32,
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(3),
                1,
                5,
                ((crate::c::bf_read((mainSprite).wrapping_add(3), 1, 5, false) as u32) | 16u32)
                    as i32,
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(4),
                0,
                10,
                (crate::c::bf_read((mainSprite).wrapping_add(4), 0, 10, false) as u16) as i32,
            );
            ((reflectionSprite).wrapping_add(24).cast::<*mut u8>())
                .write(((mainSprite).wrapping_add(24).cast::<*mut u8>()).read());
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(66),
                0,
                6,
                (crate::c::bf_read((mainSprite).wrapping_add(66), 0, 6, false) as u8) as i32,
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(62),
                2,
                1,
                (crate::c::bf_read((mainSprite).wrapping_add(62), 2, 1, false) as u16) as i32,
            );
            ((reflectionSprite).wrapping_add(32).cast::<i16>())
                .write(((mainSprite).wrapping_add(32).cast::<i16>()).read());
            ((reflectionSprite).wrapping_add(34).cast::<i16>()).write(
                (((((((mainSprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((GetReflectionVerticalOffset(objectEvent)) as i32)))
                .wrapping_add(
                    ((((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .read()) as i32),
                )) as i16),
            );
            ((reflectionSprite).wrapping_add(40).cast::<i8>())
                .write(((mainSprite).wrapping_add(40).cast::<i8>()).read());
            ((reflectionSprite).wrapping_add(41).cast::<i8>())
                .write(((mainSprite).wrapping_add(41).cast::<i8>()).read());
            ((reflectionSprite).wrapping_add(36).cast::<i16>())
                .write(((mainSprite).wrapping_add(36).cast::<i16>()).read());
            ((reflectionSprite).wrapping_add(38).cast::<i16>()).write(
                ((((((mainSprite).wrapping_add(38).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
            crate::c::bf_write(
                (reflectionSprite).wrapping_add(62),
                1,
                1,
                (crate::c::bf_read((mainSprite).wrapping_add(62), 1, 1, false) as u16) as i32,
            );
            if (crate::c::bf_read((objectEvent).wrapping_add(3), 3, 1, false) as u32) == 1u32 {
                crate::c::bf_write((reflectionSprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
            if ((((((reflectionSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                == 0i32
            {
                crate::c::bf_write((reflectionSprite).wrapping_add(3), 1, 5, (0u32) as i32);
                if ((crate::c::bf_read((mainSprite).wrapping_add(3), 1, 5, false) as u32) & 8u32)
                    != 0
                {
                    crate::c::bf_write((reflectionSprite).wrapping_add(3), 1, 5, (1u32) as i32);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWarpArrowSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(8))
            .read(),
            0i16,
            0i16,
            82u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteInvisible(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowWarpArrowSprite(spriteId: u8, direction: u8, x: i16, y: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        if (((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)
            || ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != ((x) as i32)))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                != ((y) as i32))
        {
            let mut x2: i16 = 0i16;
            let mut y2: i16 = 0i16;
            SetSpritePosToMapCoords(x, y, &raw mut x2, &raw mut y2);
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((x2) as i32).wrapping_add(8i32)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((((y2) as i32).wrapping_add(8i32)) as i16));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(x);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(y);
            StartSpriteAnim(sprite, ((((direction) as i32).wrapping_sub(1i32)) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Shadow() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut graphicsInfo: *mut u8 = GetObjectEventGraphicsInfo(
            ((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(5))
            .read(),
        );
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    ((((((&raw const sShadowEffectTemplateIds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read((graphicsInfo).wrapping_add(12), 4, 2, false) as u8)
                            as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
            .read(),
            0i16,
            0i16,
            148u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                (((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_sub(
                        ((((((&raw const gShadowVerticalOffsets)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((crate::c::bf_read((graphicsInfo).wrapping_add(12), 4, 2, false) as u8)
                                as i32) as isize,
                        ))
                        .read()) as i32),
                    )) as i16),
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateShadowFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if (TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objectEventId,
        )) != 0
        {
            FieldEffectStop(sprite, 3u8);
        } else {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36);
            let mut linkedSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read((linkedSprite).wrapping_add(5), 2, 2, false) as u16) as i32,
            );
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((linkedSprite).wrapping_add(32).cast::<i16>()).read());
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((linkedSprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            if ((((((!((crate::c::bf_read((objectEvent).wrapping_add(0), 0, 1, false) as u32)
                != 0))
                || (!((crate::c::bf_read((objectEvent).wrapping_add(2), 6, 1, false) as u32)
                    != 0)))
                || ((MetatileBehavior_IsPokeGrass(((objectEvent).wrapping_add(30)).read()))
                    != 0))
                || ((MetatileBehavior_IsSurfableWaterOrUnderwater(
                    ((objectEvent).wrapping_add(30)).read(),
                )) != 0))
                || ((MetatileBehavior_IsSurfableWaterOrUnderwater(
                    ((objectEvent).wrapping_add(31)).read(),
                )) != 0))
                || ((MetatileBehavior_IsReflective(((objectEvent).wrapping_add(30)).read())) != 0))
                || ((MetatileBehavior_IsReflective(((objectEvent).wrapping_add(31)).read())) != 0)
            {
                FieldEffectStop(sprite, 3u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_TallGrass() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut x: i16 =
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16);
        let mut y: i16 = ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .wrapping_offset(1))
        .read()) as i16);
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8i16, 8i16);
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(4))
            .read(),
            x,
            y,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read()) as i16),
            );
            if (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .wrapping_offset(7))
            .read())
                != 0
            {
                SeekSpriteAnim(sprite, 4u8);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTallGrassFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut metatileBehavior: u8 = 0u8;
        let mut localId: u8 = 0u8;
        let mut objectEventId: u8 = 0u8;
        let mut mapNum: u8 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            >> 8) as u8);
        let mut mapGroup: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        if ((crate::c::bf_read(
            ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
            0,
            1,
            false,
        ) as u8)
            != 0)
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != ((mapNum) as i32))
                || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    != ((mapGroup) as i32)))
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((&raw mut gCamera).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    (((&raw mut gCamera).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<i32>())
                    .read(),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                (((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8) as i32)
                    << 8)
                    | ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read()) as u8) as i32)) as i16),
            );
        }
        localId = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            >> 8) as u8);
        mapNum = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
        mapGroup =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8);
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
        )) as u8);
        if (((TryGetObjectEventIdByLocalIdAndMap(
            localId,
            mapNum,
            mapGroup,
            &raw mut objectEventId,
        )) != 0)
            || (!((MetatileBehavior_IsTallGrass(metatileBehavior)) != 0)))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0)
                && ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0))
        {
            FieldEffectStop(sprite, 4u8);
        } else {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36);
            if (((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32))
                || ((((((objectEvent).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)))
                && (((((((objectEvent).wrapping_add(20)).cast::<i16>()).read()) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32))
                    || ((((((objectEvent).wrapping_add(20))
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32)
                        != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read()) as i32)))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            }
            metatileBehavior = 0u8;
            if ((((sprite).wrapping_add(43)).read()) as i32) == 0i32 {
                metatileBehavior = 4u8;
            }
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
            UpdateGrassFieldEffectSubpriority(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                metatileBehavior,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpTallGrass() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            12i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(10))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(12i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FindTallGrassFieldEffectSpriteId(
    localId: u8,
    mapNum: u8,
    mapGroup: u8,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut localId = localId;
        let mut mapNum = mapNum;
        let mut mapGroup = mapGroup;
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 68))
                        .wrapping_add(62),
                        0,
                        1,
                        false,
                    ) as u16)
                        != 0
                    {
                        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 68);
                        if ((((core::mem::transmute::<_, usize>(
                            ((sprite)
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .read(),
                        ) == (UpdateTallGrassFieldEffect as *const () as usize))
                            && ((((x) as i32)
                                == ((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(1))
                                .read()) as i32))
                                && (((y) as i32)
                                    == ((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(2))
                                    .read()) as i32))))
                            && (((localId) as i32)
                                == (((((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(3))
                                .read()) as i32)
                                    >> 8) as u8) as i32)))
                            && (((mapNum) as i32)
                                == (((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(3))
                                .read()) as i32)
                                    & 255i32)))
                            && (((mapGroup) as i32)
                                == ((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(4))
                                .read()) as i32))
                        {
                            return i;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 64u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_LongGrass() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut x: i16 =
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16);
        let mut y: i16 = ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .wrapping_offset(1))
        .read()) as i16);
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8i16, 8i16);
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(15))
            .read(),
            x,
            y,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((ElevationToPriority(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(2))
                    .read()) as u8),
                )) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read()) as i16),
            );
            if (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .wrapping_offset(7))
            .read())
                != 0
            {
                SeekSpriteAnim(sprite, 6u8);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateLongGrassFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut metatileBehavior: u8 = 0u8;
        let mut localId: u8 = 0u8;
        let mut objectEventId: u8 = 0u8;
        let mut mapNum: u8 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            >> 8) as u8);
        let mut mapGroup: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        if ((crate::c::bf_read(
            ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
            0,
            1,
            false,
        ) as u8)
            != 0)
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != ((mapNum) as i32))
                || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    != ((mapGroup) as i32)))
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((&raw mut gCamera).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    (((&raw mut gCamera).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<i32>())
                    .read(),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                (((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8) as i32)
                    << 8)
                    | ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read()) as u8) as i32)) as i16),
            );
        }
        localId = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            >> 8) as u8);
        mapNum = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
        mapGroup =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8);
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
        )) as u8);
        if (((TryGetObjectEventIdByLocalIdAndMap(
            localId,
            mapNum,
            mapGroup,
            &raw mut objectEventId,
        )) != 0)
            || (!((MetatileBehavior_IsLongGrass(metatileBehavior)) != 0)))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0)
                && ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0))
        {
            FieldEffectStop(sprite, 17u8);
        } else {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36);
            if (((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32))
                || ((((((objectEvent).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)))
                && (((((((objectEvent).wrapping_add(20)).cast::<i16>()).read()) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32))
                    || ((((((objectEvent).wrapping_add(20))
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32)
                        != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .read()) as i32)))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            }
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
            UpdateGrassFieldEffectSubpriority(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                0u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpLongGrass() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(16))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(18i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_ShortGrass() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(30))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read(),
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateShortGrassFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if ((TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objectEventId,
        )) != 0)
            || (!((crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(2),
                2,
                1,
                false,
            ) as u32)
                != 0))
        {
            FieldEffectStop(sprite, 41u8);
        } else {
            let mut graphicsInfo: *mut u8 = GetObjectEventGraphicsInfo(
                ((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(5))
                .read(),
            );
            let mut linkedSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            );
            let mut parentY: i16 = ((linkedSprite).wrapping_add(34).cast::<i16>()).read();
            let mut parentX: i16 = ((linkedSprite).wrapping_add(32).cast::<i16>()).read();
            if (((parentX) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32))
                || (((parentY) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(parentX);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(parentY);
                if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                    StartSpriteAnim(sprite, 0u8);
                }
            }
            ((sprite).wrapping_add(32).cast::<i16>()).write(parentX);
            ((sprite).wrapping_add(34).cast::<i16>()).write(parentY);
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_sub(8i32)) as i16),
            );
            ((sprite).wrapping_add(67)).write(
                ((((((linkedSprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read((linkedSprite).wrapping_add(5), 2, 2, false) as u16) as i32,
            );
            UpdateObjectEventSpriteInvisibility(
                sprite,
                ((crate::c::bf_read((linkedSprite).wrapping_add(62), 2, 1, false) as u16) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SandFootprints() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(11))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(13i16);
            StartSpriteAnim(
                sprite,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as u8),
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_DeepSandFootprints() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(23))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(24i16);
            StartSpriteAnim(
                sprite,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as u8),
            );
        }
        return ((spriteId) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_BikeTireTracks() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(27))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(35i16);
            StartSpriteAnim(
                sprite,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as u8),
            );
        }
        return ((spriteId) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFootprintsTireTracksFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((((&raw const gFadeFootprintsTireTracksFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn FadeFootprintsTireTracks_Step0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 40i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        }
        UpdateObjectEventSpriteInvisibility(sprite, 0u8);
    }
}
pub(crate) unsafe extern "C" fn FadeFootprintsTireTracks_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32) ^ 1i32)
                as u16) as i32,
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        UpdateObjectEventSpriteInvisibility(
            sprite,
            ((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as u8),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 56i32
        {
            FieldEffectStop(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Splash() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(13))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut linkedSprite: *mut u8 = core::ptr::null_mut();
            let mut graphicsInfo: *mut u8 =
                GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            linkedSprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read((linkedSprite).wrapping_add(5), 2, 2, false) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_sub(4i32)) as i16),
            );
            PlaySE(70u16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSplashFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
            || ((TryGetObjectEventIdByLocalIdAndMap(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                &raw mut objectEventId,
            )) != 0)
        {
            FieldEffectStop(sprite, 15u8);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((objectEventId) as i32) as isize * 36))
                    .wrapping_add(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((objectEventId) as i32) as isize * 36))
                    .wrapping_add(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read(),
            );
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpSmallSplash() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            12i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(14))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(16i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpBigSplash() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(12))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(14i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FeetInFlowingWater() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(13))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut graphicsInfo: *mut u8 =
                GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(UpdateFeetInFlowingWaterFieldEffect));
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-1i16));
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_sub(4i32)) as i16),
            );
            StartSpriteAnim(sprite, 1u8);
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateFeetInFlowingWaterFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if ((TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objectEventId,
        )) != 0)
            || (!((crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(2),
                3,
                1,
                false,
            ) as u32)
                != 0))
        {
            FieldEffectStop(sprite, 34u8);
        } else {
            let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36);
            let mut linkedSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((linkedSprite).wrapping_add(32).cast::<i16>()).read());
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((linkedSprite).wrapping_add(34).cast::<i16>()).read());
            ((sprite).wrapping_add(67)).write(((linkedSprite).wrapping_add(67)).read());
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
            if ((((((objectEvent).wrapping_add(16)).cast::<i16>()).read()) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32))
                || ((((((objectEvent).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write((((objectEvent).wrapping_add(16)).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    (((objectEvent).wrapping_add(16))
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read(),
                );
                if !((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0) {
                    PlaySE(70u16);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Ripple() -> u32 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(5))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(5i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HotSpringsWater() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(31))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read(),
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateHotSpringsWaterFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if ((TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objectEventId,
        )) != 0)
            || (!((crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(2),
                5,
                1,
                false,
            ) as u32)
                != 0))
        {
            FieldEffectStop(sprite, 42u8);
        } else {
            let mut graphicsInfo: *mut u8 = GetObjectEventGraphicsInfo(
                ((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(5))
                .read(),
            );
            let mut linkedSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            );
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((linkedSprite).wrapping_add(32).cast::<i16>()).read());
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_add(
                        ((((linkedSprite).wrapping_add(34).cast::<i16>()).read()) as i32),
                    ))
                .wrapping_sub(8i32)) as i16),
            );
            ((sprite).wrapping_add(67)).write(
                ((((((linkedSprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8),
            );
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedGrass() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(17))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(19i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedGrass2() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(18))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedSand() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(19))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(21i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_WaterSurfacing() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(20))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(22i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartAshFieldEffect(x: i16, y: i16, metatileId: u16, delay: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileId = metatileId;
        let mut delay = delay;
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).write(((x) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
            .write(((y) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
            .write(82i32);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(3))
            .write(1i32);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(4))
            .write(((metatileId) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
            .write(((delay) as i32));
        FieldEffectStart(7u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Ash() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut x: i16 =
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16);
        let mut y: i16 = ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .wrapping_offset(1))
        .read()) as i16);
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8i16, 8i16);
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(6))
            .read(),
            x,
            y,
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(4))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read()) as i16),
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateAshFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((((&raw const gAshFieldEffectFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .wrapping_offset((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_Wait(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_Show(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
        MapGridSetMetatileIdAt(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
        );
        CurrentMapDrawMetatileAt(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
        );
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(0),
            2,
            1,
            (1u32) as i32,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
    }
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectStop(sprite, 7u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SurfBlob() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(7))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            150u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (0u16) as i32);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write((-1i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write((-1i16));
        }
        FieldEffectActiveListRemove(8u8);
        return ((spriteId) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_BobState(spriteId: u8, state: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut state = state;
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .read()) as i32)
                & (-16i32))
                | (((state) as i32) & 15i32)) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_DontSyncAnim(spriteId: u8, dontSync: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut dontSync = dontSync;
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .read()) as i32)
                & (-241i32))
                | ((((dontSync) as i32) & 15i32) << 4)) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_PlayerOffset(spriteId: u8, hasOffset: u8, offset: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut hasOffset = hasOffset;
        let mut offset = offset;
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .read()) as i32)
                & (-3841i32))
                | ((((hasOffset) as i32) & 15i32) << 8)) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(offset);
    }
}
pub(crate) unsafe extern "C" fn GetSurfBlob_BobState(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        return (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 15i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetSurfBlob_DontSyncAnim(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        return ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 240i32) >> 4)
            as u8);
    }
}
pub(crate) unsafe extern "C" fn GetSurfBlob_HasPlayerOffset(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        return ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 3840i32) >> 8)
            as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSurfBlobFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut playerObj: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 36,
        );
        let mut playerSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((playerObj).wrapping_add(4)).read()) as i32) as isize * 68);
        SynchronizeSurfAnim(playerObj, sprite);
        SynchronizeSurfPosition(playerObj, sprite);
        UpdateBobbingEffect(playerObj, playerSprite, sprite);
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            2,
            2,
            (crate::c::bf_read((playerSprite).wrapping_add(5), 2, 2, false) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SynchronizeSurfAnim(playerObj: *mut u8, sprite: *mut u8) {
    unsafe {
        let mut playerObj = playerObj;
        let mut sprite = sprite;
        let mut surfBlobDirectionAnims = crate::ffi::Align4([0u8; 9]);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(0)
            .write(0u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(1)
            .write(0u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(2)
            .write(1u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(3)
            .write(2u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(4)
            .write(3u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(5)
            .write(0u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(6)
            .write(0u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(7)
            .write(1u8);
        (&raw mut surfBlobDirectionAnims)
            .cast::<u8>()
            .wrapping_add(8)
            .write(1u8);
        if !((GetSurfBlob_DontSyncAnim(sprite)) != 0) {
            StartSpriteAnimIfDifferent(
                sprite,
                (((&raw mut surfBlobDirectionAnims).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((playerObj).wrapping_add(24), 4, 4, false) as u16) as i32)
                        as isize,
                ))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SynchronizeSurfPosition(playerObj: *mut u8, sprite: *mut u8) {
    unsafe {
        let mut playerObj = playerObj;
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        let mut x: i16 = (((playerObj).wrapping_add(16)).cast::<i16>()).read();
        let mut y: i16 = (((playerObj).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read();
        let mut spriteY: i32 = ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32);
        if (spriteY == 0i32)
            && ((((x) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32))
                || (((y) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)))
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(x);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(y);
            {
                i = 1u8;
                'l1: loop {
                    if !(((i) as i32) <= 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        MoveCoords(i, &raw mut x, &raw mut y);
                        if ((MapGridGetElevationAt(((x) as i32), ((y) as i32))) as i32) == 3i32 {
                            let __p1 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                    x = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read();
                    y = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBobbingEffect(
    playerObj: *mut u8,
    playerSprite: *mut u8,
    sprite: *mut u8,
) {
    unsafe {
        let mut playerObj = playerObj;
        let mut playerSprite = playerSprite;
        let mut sprite = sprite;
        let mut intervals = crate::ffi::Align4([0u8; 4]);
        (&raw mut intervals)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(3u16);
        (&raw mut intervals)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(7u16);
        let mut bobState: u8 = GetSurfBlob_BobState(sprite);
        if ((bobState) as i32) != 0i32 {
            if (((({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as u16) as i32)
                & (((((&raw mut intervals).cast::<u16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read()) as i32))
                == 0i32
            {
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
            }
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                & 15i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
            }
            if ((bobState) as i32) != 2i32 {
                if !((GetSurfBlob_HasPlayerOffset(sprite)) != 0) {
                    ((playerSprite).wrapping_add(38).cast::<i16>())
                        .write(((sprite).wrapping_add(38).cast::<i16>()).read());
                } else {
                    ((playerSprite).wrapping_add(38).cast::<i16>()).write(((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32))).wrapping_add((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))) as i16));
                }
                ((sprite).wrapping_add(32).cast::<i16>())
                    .write(((playerSprite).wrapping_add(32).cast::<i16>()).read());
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((playerSprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                        .wrapping_add(8i32)) as i16),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartUnderwaterSurfBlobBobbing(blobSpriteId: u8) -> u8 {
    unsafe {
        let mut blobSpriteId = blobSpriteId;
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw mut gDummySpriteTemplate).cast::<u8>(),
            0i16,
            0i16,
            255u8,
        );
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_UnderwaterSurfBlob));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((blobSpriteId) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UnderwaterSurfBlob(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut blobSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
        );
        if ((({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            & 3i32)
            == 0i32
        {
            let __p3 = (blobSprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
        }
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            & 15i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Dust() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            12i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(9))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(10i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SandPile() -> u32 {
    unsafe {
        let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let mut spriteId: u8 = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(29))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut graphicsInfo: *mut u8 =
                GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i32,
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read(),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                    .wrapping_sub(2i32)) as i16),
            );
            SeekSpriteAnim(sprite, 2u8);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSandPileFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        if ((TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objectEventId,
        )) != 0)
            || (!((crate::c::bf_read(
                (((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(2),
                4,
                1,
                false,
            ) as u32)
                != 0))
        {
            FieldEffectStop(sprite, 39u8);
        } else {
            let mut parentY: i16 = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read();
            let mut parentX: i16 = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read();
            if (((parentX) as i32)
                != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32))
                || (((parentY) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(parentX);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(parentY);
                if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                    StartSpriteAnim(sprite, 0u8);
                }
            }
            ((sprite).wrapping_add(32).cast::<i16>()).write(parentX);
            ((sprite).wrapping_add(34).cast::<i16>()).write(parentY);
            ((sprite).wrapping_add(67)).write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((objectEventId) as i32) as isize * 36))
                    .wrapping_add(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(67))
                .read(),
            );
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Bubbles() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            0i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(34))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            82u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateBubblesFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(256i32, 2i32))) as i16),
        );
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32) & 256i32) as i16));
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32)
                .wrapping_sub(((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8)))
                as i16),
        );
        UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        if ((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)
            || ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
        {
            FieldEffectStop(sprite, 53u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_BerryTreeGrowthSparkle() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            4i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(22))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(3))
                .read()) as u16) as i32,
            );
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (5u16) as i32);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(23i16);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTreeDisguiseFieldEffect() -> u32 {
    unsafe {
        return ShowDisguiseFieldEffect(28u8, 24u8, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowMountainDisguiseFieldEffect() -> u32 {
    unsafe {
        return ShowDisguiseFieldEffect(29u8, 25u8, 3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSandDisguiseFieldEffect() -> u32 {
    unsafe {
        return ShowDisguiseFieldEffect(36u8, 28u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn ShowDisguiseFieldEffect(
    fldEff: u8,
    fldEffObj: u8,
    paletteNum: u8,
) -> u32 {
    unsafe {
        let mut fldEff = fldEff;
        let mut fldEffObj = fldEffObj;
        let mut paletteNum = paletteNum;
        let mut spriteId: u8 = 0u8;
        if (TryGetObjectEventIdByLocalIdAndMap(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as u8),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
            &raw mut spriteId,
        )) != 0
        {
            FieldEffectActiveListRemove(fldEff);
            return 64u32;
        }
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((fldEffObj) as i32) as isize))
            .read(),
            0i16,
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                1,
                1,
                ((crate::c::bf_read((sprite).wrapping_add(62), 1, 1, false) as u16).wrapping_add(1))
                    as i32,
            );
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, ((paletteNum) as u16) as i32);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((fldEff) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as i16),
            );
        }
        return ((spriteId) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateDisguiseFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objectEventId: u8 = 0u8;
        let mut graphicsInfo: *mut u8 = core::ptr::null_mut();
        let mut linkedSprite: *mut u8 = core::ptr::null_mut();
        if (TryGetObjectEventIdByLocalIdAndMap(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
            &raw mut objectEventId,
        )) != 0
        {
            FieldEffectStop(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
        }
        graphicsInfo = GetObjectEventGraphicsInfo(
            ((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(5))
            .read(),
        );
        linkedSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36))
            .wrapping_add(4))
            .read()) as i32) as isize
                * 68,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            (crate::c::bf_read((linkedSprite).wrapping_add(62), 2, 1, false) as u16) as i32,
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((linkedSprite).wrapping_add(32).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32) >> 1)
                .wrapping_add(((((linkedSprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
            .wrapping_sub(16i32)) as i16),
        );
        ((sprite).wrapping_add(67)).write(
            ((((((linkedSprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 1i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            StartSpriteAnim(sprite, 1u8);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 2i32)
            && ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 3i32 {
            FieldEffectStop(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartRevealDisguise(objectEvent: *mut u8) {
    unsafe {
        let mut objectEvent = objectEvent;
        if ((((objectEvent).wrapping_add(33)).read()) as i32) == 1i32 {
            let __p1 = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((objectEvent).wrapping_add(26)).read()) as i32) as isize * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRevealDisguise(objectEvent: *mut u8) -> u8 {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((((objectEvent).wrapping_add(33)).read()) as i32) == 2i32 {
            return 1u8;
        }
        if ((((objectEvent).wrapping_add(33)).read()) as i32) == 0i32 {
            return 1u8;
        }
        sprite = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((objectEvent).wrapping_add(26)).read()) as i32) as isize * 68);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0 {
            ((objectEvent).wrapping_add(33)).write(2u8);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Sparkle() -> u32 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let __p1 = ((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>();
        (__p1).write(((__p1).read()).wrapping_add(7i32));
        let __p2 =
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1);
        (__p2).write(((__p2).read()).wrapping_add(7i32));
        SetSpritePosToOffsetMapCoords(
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<i16>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<i16>(),
            8i16,
            8i16,
        );
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(35))
            .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            82u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(2))
                .read()) as u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSparkleFieldEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0)
            && ((({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 34i32)
        {
            FieldEffectStop(sprite, 54u8);
        }
    }
}
pub(crate) unsafe extern "C" fn InitRayquazaForFigure8Anim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn AnimateRayquazaInFigure8(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut finished: u8 = 0u8;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((GetFigure8XOffset(
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                        )) as i32),
                    )) as i16),
                );
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((GetFigure8YOffset(
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                        )) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (sprite).wrapping_add(36).cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_sub(
                        ((GetFigure8XOffset(
                            (((71i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            )) as i16),
                        )) as i32),
                    )) as i16),
                );
                let __p5 = (sprite).wrapping_add(38).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((GetFigure8YOffset(
                            (((71i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            )) as i16),
                        )) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (sprite).wrapping_add(36).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_sub(
                        ((GetFigure8XOffset(
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                        )) as i32),
                    )) as i16),
                );
                let __p7 = (sprite).wrapping_add(38).cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_add(
                        ((GetFigure8YOffset(
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                        )) as i32),
                    )) as i16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p8 = (sprite).wrapping_add(36).cast::<i16>();
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        ((GetFigure8XOffset(
                            (((71i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            )) as i16),
                        )) as i32),
                    )) as i16),
                );
                let __p9 = (sprite).wrapping_add(38).cast::<i16>();
                (__p9).write(
                    (((((__p9).read()) as i32).wrapping_add(
                        ((GetFigure8YOffset(
                            (((71i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            )) as i16),
                        )) as i32),
                    )) as i16),
                );
                break 'l1;
            }
        }
        SetGpuReg(
            16u8,
            ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        if (({
            let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            let __t11 = ((__p10).read()).wrapping_add(1);
            (__p10).write(__t11);
            __t11
        }) as i32)
            == 72i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p12).write(((__p12).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 4i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            finished = 1u8;
        }
        return finished;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRayquazaSpotlightEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(
                    18u8,
                    (((crate::c::div_i32(240i32, 2i32)).wrapping_sub(crate::c::div_i32(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        3i32,
                    ))) as u16),
                );
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 96i32 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 3i32) {
                                break 'l2;
                            }
                            'l3: {
                                {
                                    j = 12u8;
                                    'l4: loop {
                                        if !(((j) as i32) < 18i32) {
                                            break 'l4;
                                        }
                                        'l5: {
                                            (((100726784i32) as usize as *mut u16)
                                                .wrapping_offset(
                                                    ((((i) as i32).wrapping_mul(32i32))
                                                        .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                            .write(
                                                (((((49140i32).wrapping_add(
                                                    ((i) as i32).wrapping_mul(6i32),
                                                ))
                                                .wrapping_add(((j) as i32)))
                                                .wrapping_add(1i32))
                                                    as u16),
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
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 311i32 {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (crate::c::div_i32(
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                            3i32,
                        )) as isize,
                    ))
                    .read()) as i32)
                        >> 2)
                        .wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        )) as i16),
                );
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 189i32 {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(2i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 60i32 {
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 7i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) == 0i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 5i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
                        let __p4 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_sub(1));
                    } else {
                        let __p5 = (sprite).wrapping_add(38).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 60i32 {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(5i16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                InitRayquazaForFigure8Anim(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(6i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (AnimateRayquazaInFigure8(sprite)) != 0 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    if (({
                        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                        let __t7 = ((__p6).read()).wrapping_add(1);
                        (__p6).write(__t7);
                        __t7
                    }) as i32)
                        <= 2i32
                    {
                        InitRayquazaForFigure8Anim(sprite);
                    } else {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .write(0i16);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .write(7i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 30i32 {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(8i16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                {
                    i = 0u8;
                    'l6: loop {
                        if !(((i) as i32) < 15i32) {
                            break 'l6;
                        }
                        'l7: {
                            {
                                j = 12u8;
                                'l8: loop {
                                    if !(((j) as i32) < 18i32) {
                                        break 'l8;
                                    }
                                    'l9: {
                                        (((100726784i32) as usize as *mut u16).wrapping_offset(
                                            ((((i) as i32).wrapping_mul(32i32))
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .write(0u16);
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetGpuReg(18u8, 0u16);
                FieldEffectStop(sprite, 64u8);
                break 'l1;
            }
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 1i32
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 7i32)
                == 0i32
            {
                let __p8 = (sprite).wrapping_add(38).cast::<i16>();
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
            }
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 15i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_neg()) as i16),
                );
            }
            let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p9).write(((__p9).read()).wrapping_add(1));
        }
        let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p10).write(((__p10).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateJumpImpactEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectStop(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
        } else {
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
            SetObjectSubpriorityByElevation(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
                sprite,
                0u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitFieldEffectSpriteAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            FieldEffectStop(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            );
        } else {
            UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateGrassFieldEffectSubpriority(
    sprite: *mut u8,
    elevation: u8,
    subpriority: u8,
) {
    unsafe {
        let mut sprite = sprite;
        let mut elevation = elevation;
        let mut subpriority = subpriority;
        let mut i: u8 = 0u8;
        let mut var: i16 = 0i16;
        let mut xhi: i16 = 0i16;
        let mut lyhi: i16 = 0i16;
        let mut yhi: i16 = 0i16;
        let mut ylo: i16 = 0i16;
        SetObjectSubpriorityByElevation(elevation, sprite, subpriority);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut objectEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36);
                    if (crate::c::bf_read((objectEvent).wrapping_add(0), 0, 1, false) as u32) != 0 {
                        let mut graphicsInfo: *mut u8 =
                            GetObjectEventGraphicsInfo(((objectEvent).wrapping_add(5)).read());
                        let mut linkedSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(
                                ((((objectEvent).wrapping_add(4)).read()) as i32) as isize * 68,
                            );
                        xhi = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                            .wrapping_add(
                                ((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32),
                            )) as i16);
                        var = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                            .wrapping_sub(
                                ((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32),
                            )) as i16);
                        if (((xhi) as i32)
                            < ((((linkedSprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                            && (((var) as i32)
                                > ((((linkedSprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                        {
                            lyhi = ((((((linkedSprite).wrapping_add(34).cast::<i16>()).read())
                                as i32)
                                .wrapping_add(
                                    ((((linkedSprite).wrapping_add(41).cast::<i8>()).read())
                                        as i32),
                                )) as i16);
                            var = ((linkedSprite).wrapping_add(34).cast::<i16>()).read();
                            ylo = ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                                .wrapping_sub(
                                    ((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32),
                                )) as i16);
                            yhi = ((((ylo) as i32).wrapping_add(
                                ((((linkedSprite).wrapping_add(41).cast::<i8>()).read()) as i32),
                            )) as i16);
                            if (((((lyhi) as i32) < ((yhi) as i32))
                                || (((lyhi) as i32) < ((ylo) as i32)))
                                && (((var) as i32) > ((yhi) as i32)))
                                && (((((sprite).wrapping_add(67)).read()) as i32)
                                    <= ((((linkedSprite).wrapping_add(67)).read()) as i32))
                            {
                                ((sprite).wrapping_add(67)).write(
                                    ((((((linkedSprite).wrapping_add(67)).read()) as i32)
                                        .wrapping_add(2i32))
                                        as u8),
                                );
                                break 'l1;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
