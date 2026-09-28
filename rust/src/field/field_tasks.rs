//! Translated from `src/field_tasks.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sPerStepCallbacks sHalfSubmergedBridgeMetatileOffsets sFullySubmergedBridgeMetatileOffsets sFloatingBridgeMetatileOffsets sSootopolisGymIceRowVars sMuddySlopeMetatiles

/// `struct PacifidlogMetatileOffsets`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PacifidlogMetatileOffsets {
    pub x: i8,
    pub y: i8,
    pub metatileId: u16,
}

unsafe impl Sync for PacifidlogMetatileOffsets {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PacifidlogMetatileOffsets>() == 4);
    assert!(offset_of!(PacifidlogMetatileOffsets, x) == 0);
    assert!(offset_of!(PacifidlogMetatileOffsets, y) == 1);
    assert!(offset_of!(PacifidlogMetatileOffsets, metatileId) == 2);
};

const ICE_PUZZLE_HEIGHT: i32 = 14;
const ICE_PUZZLE_L: u32 = 3;
const ICE_PUZZLE_T: u16 = 6;
const ICE_PUZZLE_WIDTH: i32 = 11;
const SLOPE_ANIM_TIME: i16 = 32;
const SLOPE_DATA_SIZE: i32 = 3;
const SLOPE_DATA_START: i32 = 4;
const SLOPE_TIME: i32 = 0;
const SLOPE_X: i32 = 1;
const SLOPE_Y: i32 = 2;
const TIME_UPDATE_INTERVAL: u32 = 4096;

static sFloatingBridgeMetatileOffsets: Table<CArray<PacifidlogMetatileOffsets, 8>> =
    Table((&raw const crate::data::field_tasks::sFloatingBridgeMetatileOffsets).cast());
static sFullySubmergedBridgeMetatileOffsets: Table<CArray<PacifidlogMetatileOffsets, 8>> =
    Table((&raw const crate::data::field_tasks::sFullySubmergedBridgeMetatileOffsets).cast());
static sHalfSubmergedBridgeMetatileOffsets: Table<CArray<PacifidlogMetatileOffsets, 8>> =
    Table((&raw const crate::data::field_tasks::sHalfSubmergedBridgeMetatileOffsets).cast());
static sMuddySlopeMetatiles: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::field_tasks::sMuddySlopeMetatiles).cast());
static sPerStepCallbacks: Table<CArray<Option<unsafe extern "C" fn(u8)>, 8>> =
    Table((&raw const crate::data::field_tasks::sPerStepCallbacks).cast());
static sSootopolisGymIceRowVars: Table<CArray<u16, 26>> =
    Table((&raw const crate::data::field_tasks::sSootopolisGymIceRowVars).cast());

unsafe extern "C" {
    static mut gCamera: Camera;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gTasks: CArray<Task, 0>;
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
    let mut idx: i32 = gTasks[taskId].data[0] as i32;
    sPerStepCallbacks[idx].unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn RunTimeBasedEvents(mut data: *mut i16) {
    match *data {
        0 => {
            if gMain.vblankCounter1 & TIME_UPDATE_INTERVAL != 0 {
                DoTimeBasedEvents();
                *data += 1;
            }
        }
        1 => {
            if gMain.vblankCounter1 & TIME_UPDATE_INTERVAL == 0 {
                *data -= 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_RunTimeBasedEvents(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ArePlayerFieldControlsLocked() == 0 {
        RunTimeBasedEvents(data);
        UpdateAmbientCry(data.at(1), data.at(2) as *mut u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldTasks() {
    if FuncIsActiveTask(Some(Task_RunPerStepCallback)) == 0 {
        let mut taskId: u8 = CreateTask(Some(Task_RunPerStepCallback), 80);
        gTasks[taskId].data[0] = STEP_CB_DUMMY;
    }
    if FuncIsActiveTask(Some(Task_MuddySlope)) == 0 {
        CreateTask(Some(Task_MuddySlope), 80);
    }
    if FuncIsActiveTask(Some(Task_RunTimeBasedEvents)) == 0 {
        CreateTask(Some(Task_RunTimeBasedEvents), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActivatePerStepCallback(callbackId: u8) {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_RunPerStepCallback));
    if taskId != TASK_NONE {
        let mut i: i32 = 0;
        let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
        i = 0;
        while i < NUM_TASK_DATA as i32 {
            *data.at(i) = 0;
            i += 1;
        }
        if callbackId >= 8 {
            *data = STEP_CB_DUMMY;
        } else {
            *data = callbackId as i16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFieldTasksArgs() {
    let mut taskId: u8 = 0;
    let mut data: *mut i16 = null_mut();
    taskId = FindTaskIdByFunc(Some(Task_RunPerStepCallback));
    if taskId != TASK_NONE {
        data = gTasks[taskId].data.as_mut_ptr();
    }
    taskId = FindTaskIdByFunc(Some(Task_RunTimeBasedEvents));
    if taskId != TASK_NONE {
        data = gTasks[taskId].data.as_mut_ptr();
        *data.at(1) = 0;
        *data.at(2) = 0;
    }
}
pub(crate) unsafe extern "C" fn DummyPerStepCallback(taskId: u8) {}
pub(crate) unsafe extern "C" fn GetPacifidlogBridgeMetatileOffsets(
    mut offsets: *mut PacifidlogMetatileOffsets,
    metatileBehavior: u16,
) -> *mut PacifidlogMetatileOffsets {
    if MetatileBehavior_IsPacifidlogVerticalLogTop(metatileBehavior as u8) != 0 {
        return offsets;
    } else if MetatileBehavior_IsPacifidlogVerticalLogBottom(metatileBehavior as u8) != 0 {
        return offsets.at(2);
    } else if MetatileBehavior_IsPacifidlogHorizontalLogLeft(metatileBehavior as u8) != 0 {
        return offsets.at(4);
    } else if MetatileBehavior_IsPacifidlogHorizontalLogRight(metatileBehavior as u8) != 0 {
        return offsets.at(6);
    } else {
        return null_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn TrySetPacifidlogBridgeMetatiles(
    mut offsets: *mut PacifidlogMetatileOffsets,
    x: i16,
    y: i16,
    redrawMap: u32,
) {
    offsets = GetPacifidlogBridgeMetatileOffsets(
        offsets,
        MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16,
    );
    if !offsets.is_null() {
        MapGridSetMetatileIdAt(
            x as i32 + (*offsets).x as i32,
            y as i32 + (*offsets).y as i32,
            (*offsets).metatileId,
        );
        if redrawMap != 0 {
            CurrentMapDrawMetatileAt(
                x as i32 + (*offsets).x as i32,
                y as i32 + (*offsets).y as i32,
            );
        }
        MapGridSetMetatileIdAt(
            x as i32 + (*offsets.at(1)).x as i32,
            y as i32 + (*offsets.at(1)).y as i32,
            (*offsets.at(1)).metatileId,
        );
        if redrawMap != 0 {
            CurrentMapDrawMetatileAt(
                x as i32 + (*offsets.at(1)).x as i32,
                y as i32 + (*offsets.at(1)).y as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeHalfSubmerged(x: i16, y: i16, redrawMap: u32) {
    TrySetPacifidlogBridgeMetatiles(
        sHalfSubmergedBridgeMetatileOffsets.as_ptr().cast_mut(),
        x,
        y,
        redrawMap,
    );
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeFullySubmerged(x: i16, y: i16, redrawMap: u32) {
    TrySetPacifidlogBridgeMetatiles(
        sFullySubmergedBridgeMetatileOffsets.as_ptr().cast_mut(),
        x,
        y,
        redrawMap,
    );
}
pub(crate) unsafe extern "C" fn TrySetLogBridgeFloating(x: i16, y: i16, redrawMap: u32) {
    TrySetPacifidlogBridgeMetatiles(
        sFloatingBridgeMetatileOffsets.as_ptr().cast_mut(),
        x,
        y,
        redrawMap,
    );
}
pub(crate) unsafe extern "C" fn ShouldRaisePacifidlogLogs(
    newX: i16,
    newY: i16,
    oldX: i16,
    oldY: i16,
) -> u32 {
    let mut oldBehavior: u16 = MapGridGetMetatileBehaviorAt(oldX as i32, oldY as i32) as u16;
    if MetatileBehavior_IsPacifidlogVerticalLogTop(oldBehavior as u8) != 0 {
        if newY > oldY {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogVerticalLogBottom(oldBehavior as u8) != 0 {
        if newY < oldY {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogHorizontalLogLeft(oldBehavior as u8) != 0 {
        if newX > oldX {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogHorizontalLogRight(oldBehavior as u8) != 0 {
        if newX < oldX {
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn ShouldSinkPacifidlogLogs(
    newX: i16,
    newY: i16,
    oldX: i16,
    oldY: i16,
) -> u32 {
    let mut newBehavior: u16 = MapGridGetMetatileBehaviorAt(newX as i32, newY as i32) as u16;
    if MetatileBehavior_IsPacifidlogVerticalLogTop(newBehavior as u8) != 0 {
        if newY < oldY {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogVerticalLogBottom(newBehavior as u8) != 0 {
        if newY > oldY {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogHorizontalLogLeft(newBehavior as u8) != 0 {
        if newX < oldX {
            return FALSE as u32;
        }
    } else if MetatileBehavior_IsPacifidlogHorizontalLogRight(newBehavior as u8) != 0 {
        if newX > oldX {
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn PacifidlogBridgePerStepCallback(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    data = gTasks[taskId].data.as_mut_ptr();
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    match *data.at(1) {
        0 => {
            *data.at(2) = x;
            *data.at(3) = y;
            TrySetLogBridgeFullySubmerged(x, y, TRUE as u32);
            *data.at(1) = 1;
        }
        1 => {
            if x == *data.at(2) && y == *data.at(3) {
                return;
            }
            if ShouldRaisePacifidlogLogs(x, y, *data.at(2), *data.at(3)) != 0 {
                TrySetLogBridgeHalfSubmerged(*data.at(2), *data.at(3), TRUE as u32);
                TrySetLogBridgeFloating(*data.at(2), *data.at(3), FALSE as u32);
                *data.at(4) = *data.at(2);
                *data.at(5) = *data.at(3);
                *data.at(1) = 2;
                *data.at(6) = 8;
            } else {
                *data.at(4) = -1;
                *data.at(5) = -1;
            }
            if ShouldSinkPacifidlogLogs(x, y, *data.at(2), *data.at(3)) != 0 {
                TrySetLogBridgeHalfSubmerged(x, y, TRUE as u32);
                *data.at(1) = 2;
                *data.at(6) = 8;
            }
            *data.at(2) = x;
            *data.at(3) = y;
            if MetatileBehavior_IsPacifidlogLog(
                MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8
            ) != 0
            {
                PlaySE(SE_PUDDLE);
            }
        }
        2 => {
            if ({
                *data.at(6) -= 1;
                *data.at(6)
            }) == 0
            {
                TrySetLogBridgeFullySubmerged(x, y, TRUE as u32);
                if *data.at(4) != -1 && *data.at(5) != -1 {
                    TrySetLogBridgeFloating(*data.at(4), *data.at(5), TRUE as u32);
                }
                *data.at(1) = 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn TryLowerFortreeBridge(x: i16, y: i16) {
    let mut elevation: u8 = PlayerGetElevation();
    if elevation as i32 & 1 == 0 {
        match MapGridGetMetatileIdAt(x as i32, y as i32) {
            METATILE_Fortree_BridgeOverGrass_Raised => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32,
                    METATILE_Fortree_BridgeOverGrass_Lowered,
                );
            }
            METATILE_Fortree_BridgeOverTrees_Raised => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32,
                    METATILE_Fortree_BridgeOverTrees_Lowered,
                );
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn TryRaiseFortreeBridge(x: i16, y: i16) {
    let mut elevation: u8 = PlayerGetElevation();
    if elevation as i32 & 1 == 0 {
        match MapGridGetMetatileIdAt(x as i32, y as i32) {
            591 => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32,
                    METATILE_Fortree_BridgeOverGrass_Raised as u16,
                );
            }
            599 => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32,
                    METATILE_Fortree_BridgeOverTrees_Raised as u16,
                );
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn FortreeBridgePerStepCallback(taskId: u8) {
    let mut isFortreeBridgeCur: u8 = 0;
    let mut isFortreeBridgePrev: u8 = 0;
    let mut elevation: u8 = 0;
    let mut onBridgeElevation: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut prevX: i16 = 0;
    let mut prevY: i16 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    'l1: {
        let sw1: i16 = *data.at(1);
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2;
        let mut fall = false;
        if !matched {
            fall = true;
            break 'l1;
        }
        if sw1 == 0 {
            fall = true;
            *data.at(2) = x;
            *data.at(3) = y;
            if MetatileBehavior_IsFortreeBridge(
                MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8
            ) != 0
            {
                TryLowerFortreeBridge(x, y);
                CurrentMapDrawMetatileAt(x as i32, y as i32);
            }
            *data.at(1) = 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            prevX = *data.at(2);
            prevY = *data.at(3);
            if x == prevX && y == prevY {
                break 'l1;
            }
            isFortreeBridgeCur = MetatileBehavior_IsFortreeBridge(MapGridGetMetatileBehaviorAt(
                x as i32, y as i32,
            ) as u8);
            isFortreeBridgePrev = MetatileBehavior_IsFortreeBridge(MapGridGetMetatileBehaviorAt(
                prevX as i32,
                prevY as i32,
            ) as u8);
            elevation = PlayerGetElevation();
            onBridgeElevation = FALSE;
            if elevation as i32 & 1 == 0 {
                onBridgeElevation = TRUE;
            }
            if onBridgeElevation != 0 && (isFortreeBridgeCur == TRUE || isFortreeBridgePrev == TRUE)
            {
                PlaySE(SE_BRIDGE_WALK);
            }
            if isFortreeBridgePrev != 0 {
                TryRaiseFortreeBridge(prevX, prevY);
                CurrentMapDrawMetatileAt(prevX as i32, prevY as i32);
                TryLowerFortreeBridge(x, y);
                CurrentMapDrawMetatileAt(x as i32, y as i32);
            }
            *data.at(4) = prevX;
            *data.at(5) = prevY;
            *data.at(2) = x;
            *data.at(3) = y;
            if isFortreeBridgePrev == 0 {
                break 'l1;
            }
            *data.at(6) = 16;
            *data.at(1) = 2;
        }
        if fall || sw1 == 2 {
            fall = true;
            *data.at(6) -= 1;
            prevX = *data.at(4);
            prevY = *data.at(5);
            'l2: {
                let sw2: i16 = *data.at(6) % 7;
                let mut fall = false;
                if sw2 == 0 {
                    fall = true;
                    CurrentMapDrawMetatileAt(prevX as i32, prevY as i32);
                }
                if fall || sw2 == 1 || sw2 == 2 || sw2 == 3 {
                    fall = true;
                    break 'l2;
                }
                if sw2 == 4 {
                    fall = true;
                    TryLowerFortreeBridge(prevX, prevY);
                    CurrentMapDrawMetatileAt(prevX as i32, prevY as i32);
                    TryRaiseFortreeBridge(prevX, prevY);
                }
                if fall || sw2 == 5 || sw2 == 6 || sw2 == 7 {
                    fall = true;
                    break 'l2;
                }
            }
            if *data.at(6) == 0 {
                *data.at(1) = 1;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CoordInIcePuzzleRegion(x: i16, y: i16) -> u32 {
    if (x as u16 as i32 - ICE_PUZZLE_L as i32) < ICE_PUZZLE_WIDTH
        && (y as u16 as i32 - ICE_PUZZLE_T as i32) < ICE_PUZZLE_HEIGHT
        && sSootopolisGymIceRowVars[y] != 0
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MarkIcePuzzleCoordVisited(x: i16, y: i16) {
    if CoordInIcePuzzleRegion(x, y) != 0 {
        *GetVarPointer(sSootopolisGymIceRowVars[y]) |= shl_i32(1, x as u32 - ICE_PUZZLE_L) as u16;
    }
}
pub(crate) unsafe extern "C" fn IsIcePuzzleCoordVisited(x: i16, y: i16) -> u32 {
    let mut var: u16 = 0;
    if CoordInIcePuzzleRegion(x, y) == 0 {
        return FALSE as u32;
    }
    var = VarGet(sSootopolisGymIceRowVars[y]);
    if ({
        var &= shl_i32(1, x as u32 - ICE_PUZZLE_L) as u16;
        var
    }) != 0
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSootopolisGymCrackedIceMetatiles() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut width: i32 = (*gMapHeader.mapLayout).width;
    let mut height: i32 = (*gMapHeader.mapLayout).height;
    x = 0;
    while x < width {
        y = 0;
        while y < height {
            if IsIcePuzzleCoordVisited(x as i16, y as i16) == TRUE as u32 {
                MapGridSetMetatileIdAt(
                    x + MAP_OFFSET,
                    y + MAP_OFFSET,
                    METATILE_SootopolisGym_Ice_Cracked,
                );
            }
            y += 1;
        }
        x += 1;
    }
}
pub(crate) unsafe extern "C" fn SootopolisGymIcePerStepCallback(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut tileBehavior: u16 = 0;
    let mut iceStepCount: *mut u16 = null_mut();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data.at(1) {
        0 => {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            *data.at(2) = x;
            *data.at(3) = y;
            *data.at(1) = 1;
        }
        1 => {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            if x == *data.at(2) && y == *data.at(3) {
                return;
            }
            *data.at(2) = x;
            *data.at(3) = y;
            tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
            iceStepCount = GetVarPointer(VAR_ICE_STEP_COUNT);
            if MetatileBehavior_IsThinIce(tileBehavior as u8) == TRUE {
                *iceStepCount += 1;
                *data.at(6) = 4;
                *data.at(1) = 2;
                *data.at(4) = x;
                *data.at(5) = y;
            } else if MetatileBehavior_IsCrackedIce(tileBehavior as u8) == TRUE {
                *iceStepCount = 0;
                *data.at(6) = 4;
                *data.at(1) = 3;
                *data.at(4) = x;
                *data.at(5) = y;
            }
        }
        2 => {
            if *data.at(6) != 0 {
                *data.at(6) -= 1;
            } else {
                x = *data.at(4);
                y = *data.at(5);
                PlaySE(SE_ICE_CRACK);
                MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_SootopolisGym_Ice_Cracked);
                CurrentMapDrawMetatileAt(x as i32, y as i32);
                MarkIcePuzzleCoordVisited(x - MAP_OFFSET as i16, y - MAP_OFFSET as i16);
                *data.at(1) = 1;
            }
        }
        3 => {
            if *data.at(6) != 0 {
                *data.at(6) -= 1;
            } else {
                x = *data.at(4);
                y = *data.at(5);
                PlaySE(SE_ICE_BREAK);
                MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_SootopolisGym_Ice_Broken);
                CurrentMapDrawMetatileAt(x as i32, y as i32);
                *data.at(1) = 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AshGrassPerStepCallback(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut ashGatherCount: *mut u16 = null_mut();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    if x == *data.at(1) && y == *data.at(2) {
        return;
    }
    *data.at(1) = x;
    *data.at(2) = y;
    if MetatileBehavior_IsAshGrass(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) != 0 {
        if MapGridGetMetatileIdAt(x as i32, y as i32) == METATILE_Fallarbor_AshGrass {
            StartAshFieldEffect(x, y, METATILE_Fallarbor_NormalGrass, 4);
        } else {
            StartAshFieldEffect(x, y, METATILE_Lavaridge_NormalGrass, 4);
        }
        if CheckBagHasItem(ITEM_SOOT_SACK, 1) != 0 {
            ashGatherCount = GetVarPointer(VAR_ASH_GATHER_COUNT);
            if *ashGatherCount < 9999 {
                *ashGatherCount += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCrackedFloorHoleMetatile(x: i16, y: i16) {
    let mut metatileId: u16 =
        (if MapGridGetMetatileIdAt(x as i32, y as i32) == METATILE_Cave_CrackedFloor {
            METATILE_Cave_CrackedFloor_Hole
        } else {
            METATILE_Pacifidlog_SkyPillar_CrackedFloor_Hole
        }) as u16;
    MapGridSetMetatileIdAt(x as i32, y as i32, metatileId);
    CurrentMapDrawMetatileAt(x as i32, y as i32);
}
pub(crate) unsafe extern "C" fn CrackedFloorPerStepCallback(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: u16 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    behavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
    if *data.at(4) != 0
        && ({
            *data.at(4) -= 1;
            *data.at(4)
        }) == 0
    {
        SetCrackedFloorHoleMetatile(*data.at(5), *data.at(6));
    }
    if *data.at(7) != 0
        && ({
            *data.at(7) -= 1;
            *data.at(7)
        }) == 0
    {
        SetCrackedFloorHoleMetatile(*data.at(8), *data.at(9));
    }
    if MetatileBehavior_IsCrackedFloorHole(behavior as u8) != 0 {
        VarSet(VAR_ICE_STEP_COUNT, 0);
    }
    if x == *data.at(2) && y == *data.at(3) {
        return;
    }
    *data.at(2) = x;
    *data.at(3) = y;
    if MetatileBehavior_IsCrackedFloor(behavior as u8) != 0 {
        if GetPlayerSpeed() != PLAYER_SPEED_FASTEST {
            VarSet(VAR_ICE_STEP_COUNT, 0);
        }
        if *data.at(4) == 0 {
            *data.at(4) = 3;
            *data.at(5) = x;
            *data.at(6) = y;
        } else if *data.at(7) == 0 {
            *data.at(7) = 3;
            *data.at(8) = x;
            *data.at(9) = y;
        }
    }
}
pub(crate) unsafe extern "C" fn SetMuddySlopeMetatile(mut data: *mut i16, x: i16, y: i16) {
    let mut metatileId: u16 = 0;
    if ({
        *data -= 1;
        *data
    }) == 0
    {
        metatileId = METATILE_General_MuddySlope_Frame0;
    } else {
        metatileId = sMuddySlopeMetatiles[*data / 8];
    }
    MapGridSetMetatileIdAt(x as i32, y as i32, metatileId);
    CurrentMapDrawMetatileAt(x as i32, y as i32);
    MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_General_MuddySlope_Frame0);
}
pub(crate) unsafe extern "C" fn Task_MuddySlope(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut cameraOffsetX: i16 = 0;
    let mut cameraOffsetY: i16 = 0;
    let mut i: i32 = 0;
    let mut mapId: u16 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    mapId = ((*gSaveBlock1Ptr).location.mapGroup as u16) << 8
        | (*gSaveBlock1Ptr).location.mapNum as u16;
    'l1: {
        match *data.at(1) {
            0 => {
                *data = mapId as i16;
                *data.at(2) = x;
                *data.at(3) = y;
                *data.at(1) = 1;
                *data.at(4) = 0;
                *data.at(7) = 0;
                *data.at(10) = 0;
                *data.at(13) = 0;
            }
            1 => {
                if *data.at(2) == x && *data.at(3) == y {
                    break 'l1;
                }
                *data.at(2) = x;
                *data.at(3) = y;
                if MetatileBehavior_IsMuddySlope(
                    MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8
                ) != 0
                {
                    i = SLOPE_DATA_START;
                    while i <= 13 {
                        if *data.at(i) == 0 {
                            *data.at(i + SLOPE_TIME) = SLOPE_ANIM_TIME;
                            *data.at(i + SLOPE_X) = x;
                            *data.at(i + SLOPE_Y) = y;
                            break;
                        }
                        i += SLOPE_DATA_SIZE;
                    }
                }
            }
            _ => {}
        }
    }
    if gCamera.active() != 0 && mapId as i32 != *data as i32 {
        *data = mapId as i16;
        cameraOffsetX = gCamera.x as i16;
        cameraOffsetY = gCamera.y as i16;
    } else {
        cameraOffsetX = 0;
        cameraOffsetY = 0;
    }
    i = SLOPE_DATA_START;
    while i <= 13 {
        if *data.at(i + SLOPE_TIME) != 0 {
            *data.at(i + SLOPE_X) -= cameraOffsetX;
            *data.at(i + SLOPE_Y) -= cameraOffsetY;
            SetMuddySlopeMetatile(
                data.at(i + SLOPE_TIME),
                *data.at(i + SLOPE_X),
                *data.at(i + SLOPE_Y),
            );
        }
        i += SLOPE_DATA_SIZE;
    }
}
