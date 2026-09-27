//! Translated from `src/faraway_island.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sFarawayIslandRockCoords
#[allow(unused_imports)]
use crate::data::faraway_island::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGrassSpriteId: u8 = 0u8;
pub(crate) static mut sPlayerToMewDeltaX: i16 = 0i16;
pub(crate) static mut sPlayerToMewDeltaY: i16 = 0i16;
pub(crate) static mut sMewDirectionCandidates: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);

unsafe extern "C" {
    static mut gFieldEffectObjectTemplatePointers: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_Facing: u8;
    static mut gSpritePalette_GeneralFieldEffect1: u8;
    static mut gSprites: u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FlagGet(a0: u16) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsPokeGrass(a0: u8) -> u8;
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UpdateSpritePaletteWithWeather(a0: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn GetMewObjectEventId() -> u8 {
    unsafe {
        let mut objectEventId: u8 = 0u8;
        TryGetObjectEventIdByLocalIdAndMap(
            1u8,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            &raw mut objectEventId,
        );
        return objectEventId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMewMoveDirection() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut mewSafeFromTrap: i32 = 0i32;
        let mut mew: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((GetMewObjectEventId()) as i32) as isize * 36);
        ((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>()).write(
            (((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(20))
            .cast::<i16>())
            .read()) as i32)
                .wrapping_sub((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>()).write(
            (((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(20))
            .wrapping_add(2)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(
                    (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32),
                )) as i16),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(20))
        .cast::<i16>())
        .read()) as i32)
            == (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read()) as i32))
            && ((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(20))
            .wrapping_add(2)
            .cast::<i16>())
            .read()) as i32)
                == (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(16))
                .wrapping_add(2)
                .cast::<i16>())
                .read()) as i32))
        {
            return 0u32;
        }
        if crate::c::rem_i32(((VarGet(16442u16)) as i32), 8i32) == 0i32 {
            crate::c::bf_write((mew).wrapping_add(1), 5, 1, (0u32) as i32);
        } else {
            crate::c::bf_write((mew).wrapping_add(1), 5, 1, (1u32) as i32);
        }
        if crate::c::rem_i32(((VarGet(16442u16)) as i32), 9i32) == 0i32 {
            return 0u32;
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize
                            * 36,
                    ))
                    .wrapping_add(20))
                    .cast::<i16>())
                    .read()) as i32)
                        == (((((((&raw const sFarawayIslandRockCoords)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .read()) as i32)
                    {
                        mewSafeFromTrap = 0i32;
                        if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(20))
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read()) as i32)
                            < ((((((((&raw const sFarawayIslandRockCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                        {
                            if (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read())
                                as i32)
                                <= ((((((((&raw const sFarawayIslandRockCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                            {
                                mewSafeFromTrap = 1i32;
                            }
                        } else {
                            if (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read())
                                as i32)
                                >= ((((((((&raw const sFarawayIslandRockCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                            {
                                mewSafeFromTrap = 1i32;
                            }
                        }
                        if !((mewSafeFromTrap) != 0) {
                            if ((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>()).read())
                                as i32)
                                > 0i32
                            {
                                if (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
                                    .wrapping_add(1i32)
                                    == (((((((&raw mut gObjectEvents).cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gPlayerAvatar).cast::<u8>())
                                                .wrapping_add(5))
                                            .read())
                                                as i32)
                                                as isize
                                                * 36,
                                        ))
                                    .wrapping_add(20))
                                    .cast::<i16>())
                                    .read()) as i32)
                                {
                                    if (CanMewMoveToCoords(
                                        (((((((mew).wrapping_add(16)).cast::<i16>()).read())
                                            as i32)
                                            .wrapping_add(1i32))
                                            as i16),
                                        (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                            .read(),
                                    )) != 0
                                    {
                                        return 4u32;
                                    }
                                }
                            } else {
                                if ((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>())
                                    .read()) as i32)
                                    < 0i32
                                {
                                    if (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
                                        .wrapping_sub(1i32)
                                        == (((((((&raw mut gObjectEvents).cast::<u8>())
                                            .wrapping_offset(
                                                (((((&raw mut gPlayerAvatar).cast::<u8>())
                                                    .wrapping_add(5))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 36,
                                            ))
                                        .wrapping_add(20))
                                        .cast::<i16>())
                                        .read()) as i32)
                                    {
                                        if (CanMewMoveToCoords(
                                            (((((((mew).wrapping_add(16)).cast::<i16>()).read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as i16),
                                            (((mew).wrapping_add(16))
                                                .wrapping_add(2)
                                                .cast::<i16>())
                                            .read(),
                                        )) != 0
                                        {
                                            return 3u32;
                                        }
                                    }
                                }
                            }
                            if (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
                                == (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5))
                                        .read()) as i32)
                                        as isize
                                        * 36,
                                ))
                                .wrapping_add(20))
                                .cast::<i16>())
                                .read()) as i32)
                            {
                                if ((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>())
                                    .read()) as i32)
                                    > 0i32
                                {
                                    if (CanMewMoveToCoords(
                                        (((mew).wrapping_add(16)).cast::<i16>()).read(),
                                        (((((((mew).wrapping_add(16))
                                            .wrapping_add(2)
                                            .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_sub(1i32))
                                            as i16),
                                    )) != 0
                                    {
                                        return 2u32;
                                    }
                                } else {
                                    if (CanMewMoveToCoords(
                                        (((mew).wrapping_add(16)).cast::<i16>()).read(),
                                        (((((((mew).wrapping_add(16))
                                            .wrapping_add(2)
                                            .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_add(1i32))
                                            as i16),
                                    )) != 0
                                    {
                                        return 1u32;
                                    }
                                }
                            }
                        }
                    }
                    if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize
                            * 36,
                    ))
                    .wrapping_add(20))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        == ((((((((&raw const sFarawayIslandRockCoords)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        mewSafeFromTrap = 0i32;
                        if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(20))
                        .cast::<i16>())
                        .read()) as i32)
                            < (((((((&raw const sFarawayIslandRockCoords)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .read()) as i32)
                        {
                            if (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
                                <= (((((((&raw const sFarawayIslandRockCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .read()) as i32)
                            {
                                mewSafeFromTrap = 1i32;
                            }
                        } else {
                            if (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
                                >= (((((((&raw const sFarawayIslandRockCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<i16>())
                                .read()) as i32)
                            {
                                mewSafeFromTrap = 1i32;
                            }
                        }
                        if !((mewSafeFromTrap) != 0) {
                            if ((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>()).read())
                                as i32)
                                > 0i32
                            {
                                if (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                    .read()) as i32)
                                    .wrapping_add(1i32)
                                    == (((((((&raw mut gObjectEvents).cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gPlayerAvatar).cast::<u8>())
                                                .wrapping_add(5))
                                            .read())
                                                as i32)
                                                as isize
                                                * 36,
                                        ))
                                    .wrapping_add(20))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read()) as i32)
                                {
                                    if (CanMewMoveToCoords(
                                        (((mew).wrapping_add(16)).cast::<i16>()).read(),
                                        (((((((mew).wrapping_add(16))
                                            .wrapping_add(2)
                                            .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_add(1i32))
                                            as i16),
                                    )) != 0
                                    {
                                        return 1u32;
                                    }
                                }
                            } else {
                                if ((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>())
                                    .read()) as i32)
                                    < 0i32
                                {
                                    if (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                        .read()) as i32)
                                        .wrapping_sub(1i32)
                                        == (((((((&raw mut gObjectEvents).cast::<u8>())
                                            .wrapping_offset(
                                                (((((&raw mut gPlayerAvatar).cast::<u8>())
                                                    .wrapping_add(5))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 36,
                                            ))
                                        .wrapping_add(20))
                                        .wrapping_add(2)
                                        .cast::<i16>())
                                        .read()) as i32)
                                    {
                                        if (CanMewMoveToCoords(
                                            (((mew).wrapping_add(16)).cast::<i16>()).read(),
                                            (((((((mew).wrapping_add(16))
                                                .wrapping_add(2)
                                                .cast::<i16>())
                                            .read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as i16),
                                        )) != 0
                                        {
                                            return 2u32;
                                        }
                                    }
                                }
                            }
                            if (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read())
                                as i32)
                                == (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5))
                                        .read()) as i32)
                                        as isize
                                        * 36,
                                ))
                                .wrapping_add(20))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read()) as i32)
                            {
                                if ((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>())
                                    .read()) as i32)
                                    > 0i32
                                {
                                    if (CanMewMoveToCoords(
                                        (((((((mew).wrapping_add(16)).cast::<i16>()).read())
                                            as i32)
                                            .wrapping_sub(1i32))
                                            as i16),
                                        (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                            .read(),
                                    )) != 0
                                    {
                                        return 3u32;
                                    }
                                } else {
                                    if (CanMewMoveToCoords(
                                        (((((((mew).wrapping_add(16)).cast::<i16>()).read())
                                            as i32)
                                            .wrapping_add(1i32))
                                            as i16),
                                        (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                            .read(),
                                    )) != 0
                                    {
                                        return 4u32;
                                    }
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (ShouldMewMoveNorth(mew, 0u8)) != 0 {
            if (ShouldMewMoveEast(mew, 1u8)) != 0 {
                return ((GetRandomMewDirectionCandidate(2u8)) as u32);
            } else {
                if (ShouldMewMoveWest(mew, 1u8)) != 0 {
                    return ((GetRandomMewDirectionCandidate(2u8)) as u32);
                } else {
                    return 2u32;
                }
            }
        }
        if (ShouldMewMoveSouth(mew, 0u8)) != 0 {
            if (ShouldMewMoveEast(mew, 1u8)) != 0 {
                return ((GetRandomMewDirectionCandidate(2u8)) as u32);
            } else {
                if (ShouldMewMoveWest(mew, 1u8)) != 0 {
                    return ((GetRandomMewDirectionCandidate(2u8)) as u32);
                } else {
                    return 1u32;
                }
            }
        }
        if (ShouldMewMoveEast(mew, 0u8)) != 0 {
            if (ShouldMewMoveNorth(mew, 1u8)) != 0 {
                return ((GetRandomMewDirectionCandidate(2u8)) as u32);
            } else {
                if (ShouldMewMoveSouth(mew, 1u8)) != 0 {
                    return ((GetRandomMewDirectionCandidate(2u8)) as u32);
                } else {
                    return 4u32;
                }
            }
        }
        if (ShouldMewMoveWest(mew, 0u8)) != 0 {
            if (ShouldMewMoveNorth(mew, 1u8)) != 0 {
                return ((GetRandomMewDirectionCandidate(2u8)) as u32);
            } else {
                if (ShouldMewMoveSouth(mew, 1u8)) != 0 {
                    return ((GetRandomMewDirectionCandidate(2u8)) as u32);
                } else {
                    return 3u32;
                }
            }
        }
        if ((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>()).read()) as i32) == 0i32 {
            if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read()) as i32)
                > (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
            {
                if (CanMewMoveToCoords(
                    (((mew).wrapping_add(16)).cast::<i16>()).read(),
                    (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                        .wrapping_sub(1i32)) as i16),
                )) != 0
                {
                    return 2u32;
                }
            }
            if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read()) as i32)
                < (((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
            {
                if (CanMewMoveToCoords(
                    (((mew).wrapping_add(16)).cast::<i16>()).read(),
                    (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                        .wrapping_add(1i32)) as i16),
                )) != 0
                {
                    return 1u32;
                }
            }
            if (CanMewMoveToCoords(
                (((mew).wrapping_add(16)).cast::<i16>()).read(),
                (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                    .wrapping_sub(1i32)) as i16),
            )) != 0
            {
                return 2u32;
            }
            if (CanMewMoveToCoords(
                (((mew).wrapping_add(16)).cast::<i16>()).read(),
                (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                    .wrapping_add(1i32)) as i16),
            )) != 0
            {
                return 1u32;
            }
        }
        if ((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>()).read()) as i32) == 0i32 {
            if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read()) as i32)
                > (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
            {
                if (CanMewMoveToCoords(
                    (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_sub(1i32))
                        as i16),
                    (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
                )) != 0
                {
                    return 3u32;
                }
            }
            if (((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read()) as i32)
                < (((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32)
            {
                if (CanMewMoveToCoords(
                    (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_add(1i32))
                        as i16),
                    (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
                )) != 0
                {
                    return 4u32;
                }
            }
            if (CanMewMoveToCoords(
                (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_add(1i32))
                    as i16),
                (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
            )) != 0
            {
                return 4u32;
            }
            if (CanMewMoveToCoords(
                (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_sub(1i32))
                    as i16),
                (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
            )) != 0
            {
                return 3u32;
            }
        }
        return ((GetValidMewMoveDirection(0u8)) as u32);
    }
}
pub(crate) unsafe extern "C" fn CanMewMoveToCoords(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if ((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(16))
        .cast::<i16>())
        .read()) as i32)
            == ((x) as i32))
            && ((((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read()) as i32)
                == ((y) as i32))
        {
            return 0u8;
        }
        return MetatileBehavior_IsPokeGrass(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn GetValidMewMoveDirection(ignoredDir: u8) -> u8 {
    unsafe {
        let mut ignoredDir = ignoredDir;
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut mew: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((GetMewObjectEventId()) as i32) as isize * 36);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((CanMewMoveToCoords(
            (((mew).wrapping_add(16)).cast::<i16>()).read(),
            (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                .wrapping_sub(1i32)) as i16),
        )) as i32)
            == 1i32)
            && (((ignoredDir) as i32) != 2i32)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((count) as i32) as isize))
            .write(2u8);
            count = (count).wrapping_add(1);
        }
        if (((CanMewMoveToCoords(
            (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_add(1i32))
                as i16),
            (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
        )) as i32)
            == 1i32)
            && (((ignoredDir) as i32) != 4i32)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((count) as i32) as isize))
            .write(4u8);
            count = (count).wrapping_add(1);
        }
        if (((CanMewMoveToCoords(
            (((mew).wrapping_add(16)).cast::<i16>()).read(),
            (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                .wrapping_add(1i32)) as i16),
        )) as i32)
            == 1i32)
            && (((ignoredDir) as i32) != 1i32)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((count) as i32) as isize))
            .write(1u8);
            count = (count).wrapping_add(1);
        }
        if (((CanMewMoveToCoords(
            (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_sub(1i32))
                as i16),
            (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
        )) as i32)
            == 1i32)
            && (((ignoredDir) as i32) != 3i32)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((count) as i32) as isize))
            .write(3u8);
            count = (count).wrapping_add(1);
        }
        if ((count) as i32) > 1i32 {
            return ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    (crate::c::rem_i32(((VarGet(16442u16)) as i32), ((count) as i32))) as isize,
                ))
            .read();
        } else {
            return (((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>()).read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFarawayIslandStepCounter() {
    unsafe {
        let mut steps: u16 = VarGet(16442u16);
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as i32)
            == 57i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
        {
            steps = (steps).wrapping_add(1);
            if ((steps) as i32) >= 9999i32 {
                VarSet(16442u16, 0u16);
            } else {
                VarSet(16442u16, steps);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventIsFarawayIslandMew(objectEvent: *mut u8) -> u8 {
    unsafe {
        let mut objectEvent = objectEvent;
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as i32)
            == 57i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
        {
            if ((((objectEvent).wrapping_add(5)).read()) as i32) == 229i32 {
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMewPlayingHideAndSeek() -> u8 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(1)
            .cast::<i8>())
        .read()) as i32)
            == 57i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
        {
            if (((FlagGet(458u16)) as i32) != 1i32) && (((FlagGet(718u16)) as i32) != 1i32) {
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldMewShakeGrass(objectEvent: *mut u8) -> u8 {
    unsafe {
        let mut objectEvent = objectEvent;
        if (((VarGet(16442u16)) as i32) != 65535i32)
            && (crate::c::rem_i32(((VarGet(16442u16)) as i32), 4i32) == 0i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMewAboveGrass() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut mew: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((GetMewObjectEventId()) as i32) as isize * 36);
        crate::c::bf_write((mew).wrapping_add(1), 5, 1, (0u32) as i32);
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
            crate::c::bf_write((mew).wrapping_add(3), 2, 1, (1u32) as i32);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((mew).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(66),
                6,
                2,
                (2u8) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((mew).wrapping_add(4)).read()) as i32) as isize * 68))
            .wrapping_add(67))
            .write(1u8);
        } else {
            VarSet(16442u16, 65535u16);
            crate::c::bf_write((mew).wrapping_add(3), 2, 1, (1u32) as i32);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((mew).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(66),
                6,
                2,
                (2u8) as i32,
            );
            if ((((&raw mut gSpecialVar_Facing).cast::<u16>()).read()) as i32) != 2i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((mew).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(67))
                .write(1u8);
            }
            LoadSpritePalette((&raw mut gSpritePalette_GeneralFieldEffect1).cast::<u8>());
            UpdateSpritePaletteWithWeather(IndexOfSpritePaletteTag(
                (((&raw mut gSpritePalette_GeneralFieldEffect1).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read(),
            ));
            x = (((mew).wrapping_add(16)).cast::<i16>()).read();
            y = (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read();
            SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8i16, 8i16);
            ((&raw mut sGrassSpriteId).cast::<u8>().cast::<u8>()).write(CreateSpriteAtEnd(
                ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(15))
                .read(),
                x,
                y,
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((mew).wrapping_add(4)).read()) as i32) as isize * 68))
                .wrapping_add(67))
                .read()) as i32)
                    .wrapping_sub(1i32)) as u8),
            ));
            if ((((&raw mut sGrassSpriteId).cast::<u8>().cast::<u8>()).read()) as i32) != 64i32 {
                let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sGrassSpriteId).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 68,
                );
                crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
                crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyMewEmergingGrassSprite() {
    unsafe {
        if ((((&raw mut sGrassSpriteId).cast::<u8>().cast::<u8>()).read()) as i32) != 64i32 {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((&raw mut sGrassSpriteId).cast::<u8>().cast::<u8>()).read()) as i32) as isize
                    * 68,
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn ShouldMewMoveNorth(mew: *mut u8, index: u8) -> u8 {
    unsafe {
        let mut mew = mew;
        let mut index = index;
        if (((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>()).read()) as i32) > 0i32)
            && ((CanMewMoveToCoords(
                (((mew).wrapping_add(16)).cast::<i16>()).read(),
                (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                    .wrapping_sub(1i32)) as i16),
            )) != 0)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
            .write(2u8);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldMewMoveEast(mew: *mut u8, index: u8) -> u8 {
    unsafe {
        let mut mew = mew;
        let mut index = index;
        if (((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>()).read()) as i32) < 0i32)
            && ((CanMewMoveToCoords(
                (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_add(1i32))
                    as i16),
                (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
            )) != 0)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
            .write(4u8);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldMewMoveSouth(mew: *mut u8, index: u8) -> u8 {
    unsafe {
        let mut mew = mew;
        let mut index = index;
        if (((((&raw mut sPlayerToMewDeltaY).cast::<u8>().cast::<i16>()).read()) as i32) < 0i32)
            && ((CanMewMoveToCoords(
                (((mew).wrapping_add(16)).cast::<i16>()).read(),
                (((((((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                    .wrapping_add(1i32)) as i16),
            )) != 0)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
            .write(1u8);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldMewMoveWest(mew: *mut u8, index: u8) -> u8 {
    unsafe {
        let mut mew = mew;
        let mut index = index;
        if (((((&raw mut sPlayerToMewDeltaX).cast::<u8>().cast::<i16>()).read()) as i32) > 0i32)
            && ((CanMewMoveToCoords(
                (((((((mew).wrapping_add(16)).cast::<i16>()).read()) as i32).wrapping_sub(1i32))
                    as i16),
                (((mew).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read(),
            )) != 0)
        {
            ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((index) as i32) as isize))
            .write(3u8);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetRandomMewDirectionCandidate(numDirections: u8) -> u8 {
    unsafe {
        let mut numDirections = numDirections;
        return ((((&raw mut sMewDirectionCandidates).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (crate::c::rem_i32(((VarGet(16442u16)) as i32), ((numDirections) as i32))) as isize,
        ))
        .read();
    }
}
