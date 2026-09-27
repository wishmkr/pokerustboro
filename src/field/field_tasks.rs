//! Translated from `src/field_tasks.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPerStepCallbacks sHalfSubmergedBridgeMetatileOffsets sFullySubmergedBridgeMetatileOffsets sFloatingBridgeMetatileOffsets sSootopolisGymIceRowVars sMuddySlopeMetatiles
#[allow(unused_imports)]
use crate::data::field_tasks::*;

unsafe extern "C" {
    static mut gCamera: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gTasks: u8;
    fn ArePlayerFieldControlsLocked() -> u8;
    fn CheckBagHasItem(a0: u16, a1: u16) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn DoTimeBasedEvents();
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetPlayerSpeed() -> i16;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MetatileBehavior_IsAshGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsCrackedFloor(a0: u8) -> u8;
    fn MetatileBehavior_IsCrackedFloorHole(a0: u8) -> u8;
    fn MetatileBehavior_IsCrackedIce(a0: u8) -> u8;
    fn MetatileBehavior_IsFortreeBridge(a0: u8) -> u8;
    fn MetatileBehavior_IsMuddySlope(a0: u8) -> u8;
    fn MetatileBehavior_IsPacifidlogHorizontalLogLeft(a0: u8) -> u8;
    fn MetatileBehavior_IsPacifidlogHorizontalLogRight(a0: u8) -> u8;
    fn MetatileBehavior_IsPacifidlogLog(a0: u8) -> u8;
    fn MetatileBehavior_IsPacifidlogVerticalLogBottom(a0: u8) -> u8;
    fn MetatileBehavior_IsPacifidlogVerticalLogTop(a0: u8) -> u8;
    fn MetatileBehavior_IsThinIce(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn StartAshFieldEffect(a0: i16, a1: i16, a2: u16, a3: i16);
    fn UpdateAmbientCry(a0: *mut i16, a1: *mut u16);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn Task_RunPerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut idx: i32 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32);
        (((((&raw const sPerStepCallbacks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .wrapping_offset((idx) as isize))
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn RunTimeBasedEvents(data: *mut i16) {
    unsafe {
        let mut data = data;
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 4096u32)
                    != 0
                {
                    DoTimeBasedEvents();
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !(((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read()
                    & 4096u32)
                    != 0)
                {
                    (data).write(((data).read()).wrapping_sub(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RunTimeBasedEvents(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((ArePlayerFieldControlsLocked()) != 0) {
            RunTimeBasedEvents(data);
            UpdateAmbientCry(
                (data).wrapping_offset(1),
                ((data).wrapping_offset(2)).cast::<u16>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldTasks() {
    unsafe {
        if !((FuncIsActiveTask(Some(Task_RunPerStepCallback))) != 0) {
            let mut taskId: u8 = CreateTask(Some(Task_RunPerStepCallback), 80u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
        if !((FuncIsActiveTask(Some(Task_MuddySlope))) != 0) {
            CreateTask(Some(Task_MuddySlope), 80u8);
        }
        if !((FuncIsActiveTask(Some(Task_RunTimeBasedEvents))) != 0) {
            CreateTask(Some(Task_RunTimeBasedEvents), 80u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActivatePerStepCallback(callbackId: u8) {
    unsafe {
        let mut callbackId = callbackId;
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_RunPerStepCallback));
        if ((taskId) as i32) != 255i32 {
            let mut i: i32 = 0i32;
            let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((data).wrapping_offset((i) as isize)).write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((callbackId) as u32) >= crate::c::div_u32(32u32, 4u32) {
                (data).write(0i16);
            } else {
                (data).write(((callbackId) as i16));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFieldTasksArgs() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        taskId = FindTaskIdByFunc(Some(Task_RunPerStepCallback));
        if ((taskId) as i32) != 255i32 {
            data = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
        }
        taskId = FindTaskIdByFunc(Some(Task_RunTimeBasedEvents));
        if ((taskId) as i32) != 255i32 {
            data = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            ((data).wrapping_offset(1)).write(0i16);
            ((data).wrapping_offset(2)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn DummyPerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
pub(crate) unsafe extern "C" fn GetPacifidlogBridgeMetatileOffsets(
    offsets: *mut u8,
    metatileBehavior: u16,
) -> *mut u8 {
    unsafe {
        let mut offsets = offsets;
        let mut metatileBehavior = metatileBehavior;
        if (MetatileBehavior_IsPacifidlogVerticalLogTop(((metatileBehavior) as u8))) != 0 {
            return offsets;
        } else {
            if (MetatileBehavior_IsPacifidlogVerticalLogBottom(((metatileBehavior) as u8))) != 0 {
                return (offsets).wrapping_offset(8);
            } else {
                if (MetatileBehavior_IsPacifidlogHorizontalLogLeft(((metatileBehavior) as u8))) != 0
                {
                    return (offsets).wrapping_offset(16);
                } else {
                    if (MetatileBehavior_IsPacifidlogHorizontalLogRight(((metatileBehavior) as u8)))
                        != 0
                    {
                        return (offsets).wrapping_offset(24);
                    } else {
                        return core::ptr::null_mut();
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn TrySetPacifidlogBridgeMetatiles(
    offsets: *mut u8,
    x: i16,
    y: i16,
    redrawMap: u32,
) {
    unsafe {
        let mut offsets = offsets;
        let mut x = x;
        let mut y = y;
        let mut redrawMap = redrawMap;
        offsets = GetPacifidlogBridgeMetatileOffsets(
            offsets,
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16),
        );
        if !(offsets).is_null() {
            MapGridSetMetatileIdAt(
                ((x) as i32).wrapping_add(((((offsets).cast::<i8>()).read()) as i32)),
                ((y) as i32)
                    .wrapping_add(((((offsets).wrapping_add(1).cast::<i8>()).read()) as i32)),
                ((offsets).wrapping_add(2).cast::<u16>()).read(),
            );
            if (redrawMap) != 0 {
                CurrentMapDrawMetatileAt(
                    ((x) as i32).wrapping_add(((((offsets).cast::<i8>()).read()) as i32)),
                    ((y) as i32)
                        .wrapping_add(((((offsets).wrapping_add(1).cast::<i8>()).read()) as i32)),
                );
            }
            MapGridSetMetatileIdAt(
                ((x) as i32)
                    .wrapping_add((((((offsets).wrapping_offset(4)).cast::<i8>()).read()) as i32)),
                ((y) as i32).wrapping_add(
                    (((((offsets).wrapping_offset(4)).wrapping_add(1).cast::<i8>()).read()) as i32),
                ),
                (((offsets).wrapping_offset(4)).wrapping_add(2).cast::<u16>()).read(),
            );
            if (redrawMap) != 0 {
                CurrentMapDrawMetatileAt(
                    ((x) as i32).wrapping_add(
                        (((((offsets).wrapping_offset(4)).cast::<i8>()).read()) as i32),
                    ),
                    ((y) as i32).wrapping_add(
                        (((((offsets).wrapping_offset(4)).wrapping_add(1).cast::<i8>()).read())
                            as i32),
                    ),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeHalfSubmerged(x: i16, y: i16, redrawMap: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut redrawMap = redrawMap;
        TrySetPacifidlogBridgeMetatiles(
            ((&raw const sHalfSubmergedBridgeMetatileOffsets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            x,
            y,
            redrawMap,
        );
    }
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeFullySubmerged(x: i16, y: i16, redrawMap: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut redrawMap = redrawMap;
        TrySetPacifidlogBridgeMetatiles(
            ((&raw const sFullySubmergedBridgeMetatileOffsets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            x,
            y,
            redrawMap,
        );
    }
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeFloating(x: i16, y: i16, redrawMap: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut redrawMap = redrawMap;
        TrySetPacifidlogBridgeMetatiles(
            ((&raw const sFloatingBridgeMetatileOffsets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            x,
            y,
            redrawMap,
        );
    }
}
pub(crate) unsafe extern "C" fn ShouldRaisePacifidlogLogs(
    newX: i16,
    newY: i16,
    oldX: i16,
    oldY: i16,
) -> u32 {
    unsafe {
        let mut newX = newX;
        let mut newY = newY;
        let mut oldX = oldX;
        let mut oldY = oldY;
        let mut oldBehavior: u16 =
            ((MapGridGetMetatileBehaviorAt(((oldX) as i32), ((oldY) as i32))) as u16);
        if (MetatileBehavior_IsPacifidlogVerticalLogTop(((oldBehavior) as u8))) != 0 {
            if ((newY) as i32) > ((oldY) as i32) {
                return 0u32;
            }
        } else {
            if (MetatileBehavior_IsPacifidlogVerticalLogBottom(((oldBehavior) as u8))) != 0 {
                if ((newY) as i32) < ((oldY) as i32) {
                    return 0u32;
                }
            } else {
                if (MetatileBehavior_IsPacifidlogHorizontalLogLeft(((oldBehavior) as u8))) != 0 {
                    if ((newX) as i32) > ((oldX) as i32) {
                        return 0u32;
                    }
                } else {
                    if (MetatileBehavior_IsPacifidlogHorizontalLogRight(((oldBehavior) as u8))) != 0
                    {
                        if ((newX) as i32) < ((oldX) as i32) {
                            return 0u32;
                        }
                    }
                }
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ShouldSinkPacifidlogLogs(
    newX: i16,
    newY: i16,
    oldX: i16,
    oldY: i16,
) -> u32 {
    unsafe {
        let mut newX = newX;
        let mut newY = newY;
        let mut oldX = oldX;
        let mut oldY = oldY;
        let mut newBehavior: u16 =
            ((MapGridGetMetatileBehaviorAt(((newX) as i32), ((newY) as i32))) as u16);
        if (MetatileBehavior_IsPacifidlogVerticalLogTop(((newBehavior) as u8))) != 0 {
            if ((newY) as i32) < ((oldY) as i32) {
                return 0u32;
            }
        } else {
            if (MetatileBehavior_IsPacifidlogVerticalLogBottom(((newBehavior) as u8))) != 0 {
                if ((newY) as i32) > ((oldY) as i32) {
                    return 0u32;
                }
            } else {
                if (MetatileBehavior_IsPacifidlogHorizontalLogLeft(((newBehavior) as u8))) != 0 {
                    if ((newX) as i32) < ((oldX) as i32) {
                        return 0u32;
                    }
                } else {
                    if (MetatileBehavior_IsPacifidlogHorizontalLogRight(((newBehavior) as u8))) != 0
                    {
                        if ((newX) as i32) > ((oldX) as i32) {
                            return 0u32;
                        }
                    }
                }
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn PacifidlogBridgePerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                TrySetLogBridgeFullySubmerged(x, y, 1u32);
                ((data).wrapping_offset(1)).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((x) as i32) == ((((data).wrapping_offset(2)).read()) as i32))
                    && (((y) as i32) == ((((data).wrapping_offset(3)).read()) as i32))
                {
                    return;
                }
                if (ShouldRaisePacifidlogLogs(
                    x,
                    y,
                    ((data).wrapping_offset(2)).read(),
                    ((data).wrapping_offset(3)).read(),
                )) != 0
                {
                    TrySetLogBridgeHalfSubmerged(
                        ((data).wrapping_offset(2)).read(),
                        ((data).wrapping_offset(3)).read(),
                        1u32,
                    );
                    TrySetLogBridgeFloating(
                        ((data).wrapping_offset(2)).read(),
                        ((data).wrapping_offset(3)).read(),
                        0u32,
                    );
                    ((data).wrapping_offset(4)).write(((data).wrapping_offset(2)).read());
                    ((data).wrapping_offset(5)).write(((data).wrapping_offset(3)).read());
                    ((data).wrapping_offset(1)).write(2i16);
                    ((data).wrapping_offset(6)).write(8i16);
                } else {
                    ((data).wrapping_offset(4)).write((-1i16));
                    ((data).wrapping_offset(5)).write((-1i16));
                }
                if (ShouldSinkPacifidlogLogs(
                    x,
                    y,
                    ((data).wrapping_offset(2)).read(),
                    ((data).wrapping_offset(3)).read(),
                )) != 0
                {
                    TrySetLogBridgeHalfSubmerged(x, y, 1u32);
                    ((data).wrapping_offset(1)).write(2i16);
                    ((data).wrapping_offset(6)).write(8i16);
                }
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                if (MetatileBehavior_IsPacifidlogLog(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                )) != 0
                {
                    PlaySE(70u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p2 = (data).wrapping_offset(6);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
                {
                    TrySetLogBridgeFullySubmerged(x, y, 1u32);
                    if (((((data).wrapping_offset(4)).read()) as i32) != (-1i32))
                        && (((((data).wrapping_offset(5)).read()) as i32) != (-1i32))
                    {
                        TrySetLogBridgeFloating(
                            ((data).wrapping_offset(4)).read(),
                            ((data).wrapping_offset(5)).read(),
                            1u32,
                        );
                    }
                    ((data).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryLowerFortreeBridge(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut elevation: u8 = PlayerGetElevation();
        if !((((elevation) as i32) & 1i32) != 0) {
            'l1: {
                let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
                if __sw1 == 590i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 591u16);
                    break 'l1;
                }
                if __sw1 == 598i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 599u16);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryRaiseFortreeBridge(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut elevation: u8 = PlayerGetElevation();
        if !((((elevation) as i32) & 1i32) != 0) {
            'l1: {
                let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
                if __sw1 == 591i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 590u16);
                    break 'l1;
                }
                if __sw1 == 599i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 598u16);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FortreeBridgePerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut isFortreeBridgeCur: u8 = 0u8;
        let mut isFortreeBridgePrev: u8 = 0u8;
        let mut elevation: u8 = 0u8;
        let mut onBridgeElevation: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut prevX: i16 = 0i16;
        let mut prevY: i16 = 0i16;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            let mut __fall = false;
            if !__matched {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 0i32 {
                __fall = true;
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                if (MetatileBehavior_IsFortreeBridge(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                )) != 0
                {
                    TryLowerFortreeBridge(x, y);
                    CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                }
                ((data).wrapping_offset(1)).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                prevX = ((data).wrapping_offset(2)).read();
                prevY = ((data).wrapping_offset(3)).read();
                if (((x) as i32) == ((prevX) as i32)) && (((y) as i32) == ((prevY) as i32)) {
                    break 'l1;
                }
                isFortreeBridgeCur = MetatileBehavior_IsFortreeBridge(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                );
                isFortreeBridgePrev = MetatileBehavior_IsFortreeBridge(
                    ((MapGridGetMetatileBehaviorAt(((prevX) as i32), ((prevY) as i32))) as u8),
                );
                elevation = PlayerGetElevation();
                onBridgeElevation = 0u8;
                if (((((elevation) as i32) & 1i32) as u8) as i32) == 0i32 {
                    onBridgeElevation = 1u8;
                }
                if ((onBridgeElevation) != 0)
                    && ((((isFortreeBridgeCur) as i32) == 1i32)
                        || (((isFortreeBridgePrev) as i32) == 1i32))
                {
                    PlaySE(71u16);
                }
                if (isFortreeBridgePrev) != 0 {
                    TryRaiseFortreeBridge(prevX, prevY);
                    CurrentMapDrawMetatileAt(((prevX) as i32), ((prevY) as i32));
                    TryLowerFortreeBridge(x, y);
                    CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                }
                ((data).wrapping_offset(4)).write(prevX);
                ((data).wrapping_offset(5)).write(prevY);
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                if !((isFortreeBridgePrev) != 0) {
                    break 'l1;
                }
                ((data).wrapping_offset(6)).write(16i16);
                ((data).wrapping_offset(1)).write(2i16);
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                let __p2 = (data).wrapping_offset(6);
                (__p2).write(((__p2).read()).wrapping_sub(1));
                prevX = ((data).wrapping_offset(4)).read();
                prevY = ((data).wrapping_offset(5)).read();
                'l2: {
                    let __sw3 =
                        crate::c::rem_i32(((((data).wrapping_offset(6)).read()) as i32), 7i32);
                    let mut __fall = false;
                    if __sw3 == 0i32 {
                        __fall = true;
                        CurrentMapDrawMetatileAt(((prevX) as i32), ((prevY) as i32));
                    }
                    if __fall || __sw3 == 1i32 || __sw3 == 2i32 || __sw3 == 3i32 {
                        __fall = true;
                        break 'l2;
                    }
                    if __sw3 == 4i32 {
                        __fall = true;
                        TryLowerFortreeBridge(prevX, prevY);
                        CurrentMapDrawMetatileAt(((prevX) as i32), ((prevY) as i32));
                        TryRaiseFortreeBridge(prevX, prevY);
                    }
                    if __fall || __sw3 == 5i32 || __sw3 == 6i32 || __sw3 == 7i32 {
                        __fall = true;
                        break 'l2;
                    }
                }
                if ((((data).wrapping_offset(6)).read()) as i32) == 0i32 {
                    ((data).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CoordInIcePuzzleRegion(x: i16, y: i16) -> u32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (((((((x) as i32).wrapping_sub(3i32)) as u16) as i32) < 11i32)
            && ((((((y) as i32).wrapping_sub(6i32)) as u16) as i32) < 14i32))
            && ((((((&raw const sSootopolisGymIceRowVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((y) as i32) as isize))
            .read())
                != 0)
        {
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
pub(crate) unsafe extern "C" fn MarkIcePuzzleCoordVisited(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (CoordInIcePuzzleRegion(x, y)) != 0 {
            let __p1 = GetVarPointer(
                ((((&raw const sSootopolisGymIceRowVars)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((y) as i32) as isize))
                .read(),
            );
            (__p1).write(
                (((((__p1).read()) as i32)
                    | crate::c::shl_i32(1i32, ((((x) as i32).wrapping_sub(3i32)) as u32)))
                    as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsIcePuzzleCoordVisited(x: i16, y: i16) -> u32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut var: u16 = 0u16;
        if !((CoordInIcePuzzleRegion(x, y)) != 0) {
            return 0u32;
        }
        var = VarGet(
            ((((&raw const sSootopolisGymIceRowVars)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((y) as i32) as isize))
            .read(),
        );
        if ({
            let __v1 = ((((var) as i32)
                & crate::c::shl_i32(1i32, ((((x) as i32).wrapping_sub(3i32)) as u32)))
                as u16);
            var = __v1;
            __v1
        }) != 0
        {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSootopolisGymCrackedIceMetatiles() {
    unsafe {
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut width: i32 = (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
            .cast::<i32>())
        .read();
        let mut height: i32 = (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<i32>())
        .read();
        {
            x = 0i32;
            'l1: loop {
                if !(x < width) {
                    break 'l1;
                }
                'l2: {
                    {
                        y = 0i32;
                        'l3: loop {
                            if !(y < height) {
                                break 'l3;
                            }
                            'l4: {
                                if IsIcePuzzleCoordVisited(((x) as i16), ((y) as i16)) == 1u32 {
                                    MapGridSetMetatileIdAt(
                                        (x).wrapping_add(7i32),
                                        (y).wrapping_add(7i32),
                                        526u16,
                                    );
                                }
                            }
                            y = (y).wrapping_add(1);
                        }
                    }
                }
                x = (x).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SootopolisGymIcePerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut tileBehavior: u16 = 0u16;
        let mut iceStepCount: *mut u16 = core::ptr::null_mut();
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                PlayerGetDestCoords(&raw mut x, &raw mut y);
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                ((data).wrapping_offset(1)).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                PlayerGetDestCoords(&raw mut x, &raw mut y);
                if (((x) as i32) == ((((data).wrapping_offset(2)).read()) as i32))
                    && (((y) as i32) == ((((data).wrapping_offset(3)).read()) as i32))
                {
                    return;
                }
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
                iceStepCount = GetVarPointer(16418u16);
                if ((MetatileBehavior_IsThinIce(((tileBehavior) as u8))) as i32) == 1i32 {
                    (iceStepCount).write(((iceStepCount).read()).wrapping_add(1));
                    ((data).wrapping_offset(6)).write(4i16);
                    ((data).wrapping_offset(1)).write(2i16);
                    ((data).wrapping_offset(4)).write(x);
                    ((data).wrapping_offset(5)).write(y);
                } else {
                    if ((MetatileBehavior_IsCrackedIce(((tileBehavior) as u8))) as i32) == 1i32 {
                        (iceStepCount).write(0u16);
                        ((data).wrapping_offset(6)).write(4i16);
                        ((data).wrapping_offset(1)).write(3i16);
                        ((data).wrapping_offset(4)).write(x);
                        ((data).wrapping_offset(5)).write(y);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((data).wrapping_offset(6)).read()) as i32) != 0i32 {
                    let __p2 = (data).wrapping_offset(6);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    x = ((data).wrapping_offset(4)).read();
                    y = ((data).wrapping_offset(5)).read();
                    PlaySE(42u16);
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 526u16);
                    CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                    MarkIcePuzzleCoordVisited(
                        ((((x) as i32).wrapping_sub(7i32)) as i16),
                        ((((y) as i32).wrapping_sub(7i32)) as i16),
                    );
                    ((data).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((data).wrapping_offset(6)).read()) as i32) != 0i32 {
                    let __p3 = (data).wrapping_offset(6);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                } else {
                    x = ((data).wrapping_offset(4)).read();
                    y = ((data).wrapping_offset(5)).read();
                    PlaySE(41u16);
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 518u16);
                    CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                    ((data).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AshGrassPerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut ashGatherCount: *mut u16 = core::ptr::null_mut();
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        if (((x) as i32) == ((((data).wrapping_offset(1)).read()) as i32))
            && (((y) as i32) == ((((data).wrapping_offset(2)).read()) as i32))
        {
            return;
        }
        ((data).wrapping_offset(1)).write(x);
        ((data).wrapping_offset(2)).write(y);
        if (MetatileBehavior_IsAshGrass(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) != 0
        {
            if MapGridGetMetatileIdAt(((x) as i32), ((y) as i32)) == 522i32 {
                StartAshFieldEffect(x, y, 530u16, 4i16);
            } else {
                StartAshFieldEffect(x, y, 518u16, 4i16);
            }
            if (CheckBagHasItem(270u16, 1u16)) != 0 {
                ashGatherCount = GetVarPointer(16456u16);
                if (((ashGatherCount).read()) as i32) < 9999i32 {
                    (ashGatherCount).write(((ashGatherCount).read()).wrapping_add(1));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCrackedFloorHoleMetatile(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileId: u16 = ((if MapGridGetMetatileIdAt(((x) as i32), ((y) as i32)) == 559i32
        {
            518i32
        } else {
            567i32
        }) as u16);
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), metatileId);
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
    }
}
pub(crate) unsafe extern "C" fn CrackedFloorPerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut behavior: u16 = 0u16;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        behavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
        if (((((data).wrapping_offset(4)).read()) as i32) != 0i32)
            && ((({
                let __p1 = (data).wrapping_offset(4);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            SetCrackedFloorHoleMetatile(
                ((data).wrapping_offset(5)).read(),
                ((data).wrapping_offset(6)).read(),
            );
        }
        if (((((data).wrapping_offset(7)).read()) as i32) != 0i32)
            && ((({
                let __p3 = (data).wrapping_offset(7);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32)
        {
            SetCrackedFloorHoleMetatile(
                ((data).wrapping_offset(8)).read(),
                ((data).wrapping_offset(9)).read(),
            );
        }
        if (MetatileBehavior_IsCrackedFloorHole(((behavior) as u8))) != 0 {
            VarSet(16418u16, 0u16);
        }
        if (((x) as i32) == ((((data).wrapping_offset(2)).read()) as i32))
            && (((y) as i32) == ((((data).wrapping_offset(3)).read()) as i32))
        {
            return;
        }
        ((data).wrapping_offset(2)).write(x);
        ((data).wrapping_offset(3)).write(y);
        if (MetatileBehavior_IsCrackedFloor(((behavior) as u8))) != 0 {
            if ((GetPlayerSpeed()) as i32) != 4i32 {
                VarSet(16418u16, 0u16);
            }
            if ((((data).wrapping_offset(4)).read()) as i32) == 0i32 {
                ((data).wrapping_offset(4)).write(3i16);
                ((data).wrapping_offset(5)).write(x);
                ((data).wrapping_offset(6)).write(y);
            } else {
                if ((((data).wrapping_offset(7)).read()) as i32) == 0i32 {
                    ((data).wrapping_offset(7)).write(3i16);
                    ((data).wrapping_offset(8)).write(x);
                    ((data).wrapping_offset(9)).write(y);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMuddySlopeMetatile(data: *mut i16, x: i16, y: i16) {
    unsafe {
        let mut data = data;
        let mut x = x;
        let mut y = y;
        let mut metatileId: u16 = 0u16;
        if (({
            let __t1 = ((data).read()).wrapping_sub(1);
            (data).write(__t1);
            __t1
        }) as i32)
            == 0i32
        {
            metatileId = 232u16;
        } else {
            metatileId = ((((&raw const sMuddySlopeMetatiles)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                (crate::c::div_i32(
                    (((data).read()) as i32),
                    crate::c::div_i32(32i32, ((crate::c::div_u32(8u32, 2u32)) as i32)),
                )) as isize,
            ))
            .read();
        }
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), metatileId);
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 232u16);
    }
}
pub(crate) unsafe extern "C" fn Task_MuddySlope(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut cameraOffsetX: i16 = 0i16;
        let mut cameraOffsetY: i16 = 0i16;
        let mut i: i32 = 0i32;
        let mut mapId: u16 = 0u16;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        mapId = ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            << 8)
            | (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)) as u16);
        'l1: {
            let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                (data).write(((mapId) as i16));
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                ((data).wrapping_offset(1)).write(1i16);
                ((data).wrapping_offset(4)).write(0i16);
                ((data).wrapping_offset(7)).write(0i16);
                ((data).wrapping_offset(10)).write(0i16);
                ((data).wrapping_offset(13)).write(0i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((data).wrapping_offset(2)).read()) as i32) == ((x) as i32))
                    && (((((data).wrapping_offset(3)).read()) as i32) == ((y) as i32))
                {
                    break 'l1;
                }
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                if (MetatileBehavior_IsMuddySlope(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                )) != 0
                {
                    {
                        i = 4i32;
                        'l2: loop {
                            if !(i <= 13i32) {
                                break 'l2;
                            }
                            'l3: {
                                if ((((data).wrapping_offset((i) as isize)).read()) as i32) == 0i32
                                {
                                    ((data).wrapping_offset(((i).wrapping_add(0i32)) as isize))
                                        .write(32i16);
                                    ((data).wrapping_offset(((i).wrapping_add(1i32)) as isize))
                                        .write(x);
                                    ((data).wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                        .write(y);
                                    break 'l2;
                                }
                            }
                            i = (i).wrapping_add(3i32);
                        }
                    }
                }
                break 'l1;
            }
        }
        if ((crate::c::bf_read(
            ((&raw mut gCamera).cast::<u8>()).wrapping_add(0),
            0,
            1,
            false,
        ) as u8)
            != 0)
            && (((mapId) as i32) != (((data).read()) as i32))
        {
            (data).write(((mapId) as i16));
            cameraOffsetX = (((((&raw mut gCamera).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .read()) as i16);
            cameraOffsetY = (((((&raw mut gCamera).cast::<u8>())
                .wrapping_add(8)
                .cast::<i32>())
            .read()) as i16);
        } else {
            cameraOffsetX = 0i16;
            cameraOffsetY = 0i16;
        }
        {
            i = 4i32;
            'l4: loop {
                if !(i <= 13i32) {
                    break 'l4;
                }
                'l5: {
                    if (((data).wrapping_offset(((i).wrapping_add(0i32)) as isize)).read()) != 0 {
                        let __p2 = (data).wrapping_offset(((i).wrapping_add(1i32)) as isize);
                        (__p2).write(
                            (((((__p2).read()) as i32).wrapping_sub(((cameraOffsetX) as i32)))
                                as i16),
                        );
                        let __p3 = (data).wrapping_offset(((i).wrapping_add(2i32)) as isize);
                        (__p3).write(
                            (((((__p3).read()) as i32).wrapping_sub(((cameraOffsetY) as i32)))
                                as i16),
                        );
                        SetMuddySlopeMetatile(
                            (data).wrapping_offset(((i).wrapping_add(0i32)) as isize),
                            ((data).wrapping_offset(((i).wrapping_add(1i32)) as isize)).read(),
                            ((data).wrapping_offset(((i).wrapping_add(2i32)) as isize)).read(),
                        );
                    }
                }
                i = (i).wrapping_add(3i32);
            }
        }
    }
}
