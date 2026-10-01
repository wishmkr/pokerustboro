//! Translated from `src/field_door.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sDoorAnimTiles_Littleroot sDoorNullPalette1 sDoorAnimTiles_BirchsLab sDoorNullPalette2 sDoorAnimTiles_FallarborLightRoof sDoorNullPalette3 sDoorAnimTiles_Lilycove sDoorNullPalette4 sDoorAnimTiles_LilycoveWooden sDoorNullPalette5 sDoorAnimTiles_General sDoorNullPalette6 sDoorAnimTiles_PokeCenter sDoorAnimTiles_Gym sDoorAnimTiles_PokeMart sDoorAnimTiles_RustboroTan sDoorNullPalette7 sDoorAnimTiles_RustboroGray sDoorNullPalette8 sDoorAnimTiles_Oldale sFiller1 sDoorAnimTiles_UnusedTops sFiller2 sDoorAnimTiles_UnusedBottoms sDoorNullPalette11 sDoorAnimTiles_Mauville sDoorNullPalette12 sDoorAnimTiles_Verdanturf sDoorNullPalette13 sDoorAnimTiles_Slateport sDoorNullPalette14 sDoorAnimTiles_Dewford sDoorNullPalette15 sDoorAnimTiles_Contest sDoorNullPalette16 sDoorAnimTiles_Mossdeep sDoorNullPalette17 sDoorAnimTiles_SootopolisPeakedRoof sDoorNullPalette18 sDoorAnimTiles_Sootopolis sDoorNullPalette19 sDoorAnimTiles_PokemonLeague sDoorNullPalette20 sDoorAnimTiles_Pacifidlog sDoorNullPalette21 sDoorAnimTiles_PetalburgGym sDoorNullPalette22 sDoorAnimTiles_CyclingRoad sDoorNullPalette23 sDoorAnimTiles_LilycoveDeptStore sDoorNullPalette24 sDoorAnimTiles_SafariZone sDoorNullPalette25 sDoorAnimTiles_MossdeepSpaceCenter sDoorNullPalette26 sDoorAnimTiles_CableClub sDoorNullPalette27 sDoorAnimTiles_AbandonedShip sDoorNullPalette28 sDoorAnimTiles_FallarborDarkRoof sDoorNullPalette29 sDoorAnimTiles_AbandonedShipRoom sDoorNullPalette30 sDoorAnimTiles_LilycoveDeptStoreElevator sDoorNullPalette31 sDoorAnimTiles_BattleTowerOld sDoorNullPalette32 sDoorAnimTiles_BattleTowerElevator sDoorNullPalette33 sDoorAnimTiles_UnusedBattleFrontier sDoorNullPalette34 sDoorAnimTiles_BattleDome sDoorNullPalette35 sDoorAnimTiles_BattleFactory sDoorNullPalette36 sDoorAnimTiles_BattleTower sDoorNullPalette37 sDoorAnimTiles_BattleArena sDoorNullPalette38 sDoorAnimTiles_BattleArenaLobby sDoorNullPalette39 sDoorAnimTiles_BattleDomeLobby sDoorNullPalette40 sDoorAnimTiles_BattlePalaceLobby sDoorAnimTiles_BattleTent sDoorNullPalette41 sDoorAnimTiles_BattleDomeCorridor sDoorNullPalette42 sDoorAnimTiles_BattleTowerMultiCorridor sDoorNullPalette43 sDoorAnimTiles_BattleFrontier sDoorNullPalette44 sDoorAnimTiles_BattleFrontierSliding sDoorNullPalette45 sDoorAnimTiles_BattleDomePreBattleRoom sDoorNullPalette46 sDoorAnimTiles_BattleTentInterior sDoorNullPalette47 sDoorAnimTiles_TrainerHillLobbyElevator sDoorNullPalette48 sDoorAnimTiles_TrainerHillRoofElevator sDoorNullPalette49 sDoorOpenAnimFrames sDoorCloseAnimFrames sBigDoorOpenAnimFrames sBigDoorCloseAnimFrames sDoorAnimPalettes_General sDoorAnimPalettes_PokeCenter sDoorAnimPalettes_Gym sDoorAnimPalettes_PokeMart sDoorAnimPalettes_Littleroot sDoorAnimPalettes_BirchsLab sDoorAnimPalettes_RustboroTan sDoorAnimPalettes_RustboroGray sDoorAnimPalettes_FallarborLightRoof sDoorAnimPalettes_Lilycove sDoorAnimPalettes_Oldale sDoorAnimPalettes_Mossdeep sDoorAnimPalettes_PokemonLeague sDoorAnimPalettes_Pacifidlog sDoorAnimPalettes_SootopolisPeakedRoof sDoorAnimPalettes_Sootopolis sDoorAnimPalettes_Dewford sDoorAnimPalettes_Slateport sDoorAnimPalettes_Mauville sDoorAnimPalettes_Verdanturf sDoorAnimPalettes_LilycoveWooden sDoorAnimPalettes_Contest sDoorAnimPalettes_PetalburgGym sDoorAnimPalettes_CyclingRoad sDoorAnimPalettes_LilycoveDeptStore sDoorAnimPalettes_SafariZone sDoorAnimPalettes_MossdeepSpaceCenter sDoorAnimPalettes_CableClub sDoorAnimPalettes_AbandonedShip sDoorAnimPalettes_FallarborDarkRoof sDoorAnimPalettes_AbandonedShipRoom sDoorAnimPalettes_LilycoveDeptStoreElevator sDoorAnimPalettes_BattleTowerOld sDoorAnimPalettes_BattleTowerElevator sDoorAnimPalettes_UnusedBattleFrontier sDoorAnimPalettes_BattleDome sDoorAnimPalettes_BattleFactory sDoorAnimPalettes_BattleTower sDoorAnimPalettes_BattleArena sDoorAnimPalettes_BattleArenaLobby sDoorAnimPalettes_BattleDomeLobby sDoorAnimPalettes_BattlePalaceLobby sDoorAnimPalettes_BattleTent sDoorAnimPalettes_BattleDomeCorridor sDoorAnimPalettes_BattleTowerMultiCorridor sDoorAnimPalettes_Unused sDoorAnimPalettes_BattleFrontier sDoorAnimPalettes_BattleDomePreBattleRoom sDoorAnimPalettes_BattleTentInterior sDoorAnimPalettes_TrainerHillLobbyElevator sDoorAnimPalettes_TrainerHillRoofElevator sDoorAnimGraphicsTable

/// `struct DoorGraphics`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DoorGraphics {
    pub metatileNum: u16,
    pub sound: u8,
    pub size: u8,
    pub tiles: *mut core::ffi::c_void,
    pub palettes: *mut core::ffi::c_void,
}

unsafe impl Sync for DoorGraphics {}

/// `struct DoorAnimFrame`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DoorAnimFrame {
    pub time: u8,
    pub offset: u16,
}

unsafe impl Sync for DoorAnimFrame {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<DoorGraphics>() == 12);
    assert!(offset_of!(DoorGraphics, metatileNum) == 0);
    assert!(offset_of!(DoorGraphics, sound) == 2);
    assert!(offset_of!(DoorGraphics, size) == 3);
    assert!(offset_of!(DoorGraphics, tiles) == 4);
    assert!(offset_of!(DoorGraphics, palettes) == 8);
    assert!(size_of::<DoorAnimFrame>() == 4);
    assert!(offset_of!(DoorAnimFrame, time) == 0);
    assert!(offset_of!(DoorAnimFrame, offset) == 2);
};

const DOOR_SOUND_ARENA: i32 = 2;
const DOOR_SOUND_NORMAL: i32 = 0;
const DOOR_SOUND_SLIDING: i32 = 1;
const DOOR_TILE_START_SIZE1: u16 = 1016;
const DOOR_TILE_START_SIZE2: u16 = 1008;

static sBigDoorOpenAnimFrames: Table<CArray<DoorAnimFrame, 5>> =
    Table((&raw const crate::data::field_door::sBigDoorOpenAnimFrames).cast());
static sDoorAnimGraphicsTable: Table<CArray<DoorGraphics, 54>> =
    Table((&raw const crate::data::field_door::sDoorAnimGraphicsTable).cast());
static sDoorCloseAnimFrames: Table<CArray<DoorAnimFrame, 5>> =
    Table((&raw const crate::data::field_door::sDoorCloseAnimFrames).cast());
static sDoorOpenAnimFrames: Table<CArray<DoorAnimFrame, 5>> =
    Table((&raw const crate::data::field_door::sDoorOpenAnimFrames).cast());

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gTasks: CArray<Task, 0>;
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn DestroyTask(a0: u8);
    fn DrawDoorMetatileAt(a0: i32, a1: i32, a2: *mut u16);
    fn FlagGet(a0: u16) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsDoor(a0: u8) -> u8;
}

pub(crate) unsafe extern "C" fn CopyDoorTilesToVram(
    gfx: *mut DoorGraphics,
    frame: *mut DoorAnimFrame,
) {
    if (*gfx).size == 2 {
        CpuFastSet(
            ((*gfx).tiles as *mut u8).at((*frame).offset) as *mut c_void,
            0x6007e00 as usize as *mut c_void,
            128,
        );
    } else {
        CpuFastSet(
            ((*gfx).tiles as *mut u8).at((*frame).offset) as *mut c_void,
            0x6007f00 as usize as *mut c_void,
            64,
        );
    }
}
pub(crate) unsafe extern "C" fn BuildDoorTiles(
    mut tiles: *mut u16,
    tileNum: u16,
    mut paletteNums: *mut u8,
) {
    let mut i: i32 = 0;
    let mut tile: u16 = 0;
    i = 0;
    while i < 4 {
        tile = (*({
            let t2 = paletteNums;
            paletteNums = paletteNums.at(1);
            t2
        }) as u16)
            << 12;
        *tiles.at(i) = tile | tileNum + i as u16;
        i += 1;
    }
    while i < 8 {
        tile = (*({
            let t4 = paletteNums;
            paletteNums = paletteNums.at(1);
            t4
        }) as u16)
            << 12;
        *tiles.at(i) = tile;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawCurrentDoorAnimFrame(
    gfx: *mut DoorGraphics,
    x: u32,
    y: u32,
    mut paletteNums: *mut u8,
) {
    let mut tiles: CArray<u16, 24> = zeroed();
    if (*gfx).size == 2 {
        BuildDoorTiles(&raw mut tiles[8], DOOR_TILE_START_SIZE2, paletteNums);
        DrawDoorMetatileAt(x as i32, y as i32 - 1, &raw mut tiles[8]);
        BuildDoorTiles(&raw mut tiles[8], 1012, paletteNums.at(4));
        DrawDoorMetatileAt(x as i32, y as i32, &raw mut tiles[8]);
        BuildDoorTiles(&raw mut tiles[8], 1016, paletteNums);
        DrawDoorMetatileAt(x as i32 + 1, y as i32 - 1, &raw mut tiles[8]);
        BuildDoorTiles(&raw mut tiles[8], 1020, paletteNums.at(4));
        DrawDoorMetatileAt(x as i32 + 1, y as i32, &raw mut tiles[8]);
    } else {
        BuildDoorTiles(&raw mut tiles[0], DOOR_TILE_START_SIZE1, paletteNums);
        DrawDoorMetatileAt(x as i32, y as i32 - 1, &raw mut tiles[0]);
        BuildDoorTiles(&raw mut tiles[0], 1020, paletteNums.at(4));
        DrawDoorMetatileAt(x as i32, y as i32, &raw mut tiles[0]);
    }
}
pub(crate) unsafe extern "C" fn DrawClosedDoorTiles(gfx: *mut DoorGraphics, x: u32, y: u32) {
    CurrentMapDrawMetatileAt(x as i32, y as i32 - 1);
    CurrentMapDrawMetatileAt(x as i32, y as i32);
    if (*gfx).size == 2 {
        CurrentMapDrawMetatileAt(x as i32 + 1, y as i32 - 1);
        CurrentMapDrawMetatileAt(x as i32 + 1, y as i32);
    }
}
pub(crate) unsafe extern "C" fn DrawDoor(
    gfx: *mut DoorGraphics,
    frame: *mut DoorAnimFrame,
    x: u32,
    y: u32,
) {
    if (*frame).offset == 0xFFFF {
        DrawClosedDoorTiles(gfx, x, y);
        if ShouldUseMultiCorridorDoor() != 0 {
            DrawClosedDoorTiles(
                gfx,
                gSpecialVar_0x8004 as u32 + MAP_OFFSET as u32,
                gSpecialVar_0x8005 as u32 + MAP_OFFSET as u32,
            );
        }
    } else {
        CopyDoorTilesToVram(gfx, frame);
        DrawCurrentDoorAnimFrame(gfx, x, y, (*gfx).palettes as *mut u8);
        if ShouldUseMultiCorridorDoor() != 0 {
            DrawCurrentDoorAnimFrame(
                gfx,
                gSpecialVar_0x8004 as u32 + MAP_OFFSET as u32,
                gSpecialVar_0x8005 as u32 + MAP_OFFSET as u32,
                (*gfx).palettes as *mut u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimateDoorFrame(
    gfx: *mut DoorGraphics,
    mut frames: *mut DoorAnimFrame,
    mut data: *mut i16,
) -> u32 {
    if *data.at(5) == 0 {
        DrawDoor(
            gfx,
            frames.at(*data.at(4)),
            *data.at(6) as u32,
            *data.at(7) as u32,
        );
    }
    if *data.at(5) == (*frames.at(*data.at(4))).time as i16 {
        *data.at(5) = 0;
        *data.at(4) += 1;
        if (*frames.at(*data.at(4))).time == 0 {
            return FALSE as u32;
        } else {
            return TRUE as u32;
        }
    }
    *data.at(5) += 1;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Task_AnimateDoor(taskId: u8) {
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    let mut frames: *mut DoorAnimFrame =
        ((*data as i32) << 16 | *data.at(1) as i32) as usize as *mut DoorAnimFrame;
    let mut gfx: *mut DoorGraphics =
        ((*data.at(2) as i32) << 16 | *data.at(3) as i32) as usize as *mut DoorGraphics;
    if AnimateDoorFrame(gfx, frames, data as *mut i16) == FALSE as u32 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn GetLastDoorFrame(
    mut frame: *mut DoorAnimFrame,
    unused: *mut c_void,
) -> *mut DoorAnimFrame {
    while (*frame).time != 0 {
        frame = frame.at(1);
    }
    return frame.at(-1);
}
pub(crate) unsafe extern "C" fn GetDoorGraphics(
    mut gfx: *mut DoorGraphics,
    metatileNum: u16,
) -> *mut DoorGraphics {
    while !(*gfx).tiles.is_null() {
        if (*gfx).metatileNum == metatileNum {
            return gfx;
        }
        gfx = gfx.at(1);
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn StartDoorAnimationTask(
    gfx: *mut DoorGraphics,
    frames: *mut DoorAnimFrame,
    x: u32,
    y: u32,
) -> i8 {
    if FuncIsActiveTask(Some(Task_AnimateDoor)) == TRUE {
        return -1;
    } else {
        let mut taskId: u8 = CreateTask(Some(Task_AnimateDoor), 0x50);
        let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
        *data.at(6) = x as i16;
        *data.at(7) = y as i16;
        *data.at(1) = frames as usize as u32 as i16;
        *data = (frames as usize as u32 >> 16) as i16;
        *data.at(3) = gfx as usize as u32 as i16;
        *data.at(2) = (gfx as usize as u32 >> 16) as i16;
        return taskId as i8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DrawClosedDoor(gfx: *mut DoorGraphics, x: u32, y: u32) {
    DrawClosedDoorTiles(gfx, x, y);
}
pub(crate) unsafe extern "C" fn DrawOpenedDoor(mut gfx: *mut DoorGraphics, x: u32, y: u32) {
    gfx = GetDoorGraphics(gfx, MapGridGetMetatileIdAt(x as i32, y as i32) as u16);
    if !gfx.is_null() {
        DrawDoor(
            gfx,
            GetLastDoorFrame(
                sDoorOpenAnimFrames.as_ptr().cast_mut(),
                sDoorOpenAnimFrames.as_ptr().cast_mut() as *mut c_void,
            ),
            x,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn StartDoorOpenAnimation(
    mut gfx: *mut DoorGraphics,
    x: u32,
    y: u32,
) -> i8 {
    gfx = GetDoorGraphics(gfx, MapGridGetMetatileIdAt(x as i32, y as i32) as u16);
    if gfx.is_null() {
        return -1;
    } else {
        if (*gfx).size == 2 {
            return StartDoorAnimationTask(gfx, sBigDoorOpenAnimFrames.as_ptr().cast_mut(), x, y);
        } else {
            return StartDoorAnimationTask(gfx, sDoorOpenAnimFrames.as_ptr().cast_mut(), x, y);
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn StartDoorCloseAnimation(
    mut gfx: *mut DoorGraphics,
    x: u32,
    y: u32,
) -> i8 {
    gfx = GetDoorGraphics(gfx, MapGridGetMetatileIdAt(x as i32, y as i32) as u16);
    if gfx.is_null() {
        return -1;
    } else {
        return StartDoorAnimationTask(gfx, sDoorCloseAnimFrames.as_ptr().cast_mut(), x, y);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetDoorSoundType(mut gfx: *mut DoorGraphics, x: u32, y: u32) -> i8 {
    gfx = GetDoorGraphics(gfx, MapGridGetMetatileIdAt(x as i32, y as i32) as u16);
    if gfx.is_null() {
        return -1;
    } else {
        return (*gfx).sound as i8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Debug_FieldAnimateDoorOpen(x: u32, y: u32) {
    StartDoorOpenAnimation(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldSetDoorOpened(x: u32, y: u32) {
    if MetatileBehavior_IsDoor(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) != 0 {
        DrawOpenedDoor(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldSetDoorClosed(x: u32, y: u32) {
    if MetatileBehavior_IsDoor(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) != 0 {
        DrawClosedDoor(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldAnimateDoorClose(x: u32, y: u32) -> i8 {
    if MetatileBehavior_IsDoor(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) == 0 {
        return -1;
    } else {
        return StartDoorCloseAnimation(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldAnimateDoorOpen(x: u32, y: u32) -> i8 {
    if MetatileBehavior_IsDoor(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) == 0 {
        return -1;
    } else {
        return StartDoorOpenAnimation(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldIsDoorAnimationRunning() -> u8 {
    return FuncIsActiveTask(Some(Task_AnimateDoor));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDoorSoundEffect(x: u32, y: u32) -> u32 {
    let mut sound: i32 = GetDoorSoundType(sDoorAnimGraphicsTable.as_ptr().cast_mut(), x, y) as i32;
    if sound == DOOR_SOUND_NORMAL {
        return SE_DOOR;
    } else if sound == DOOR_SOUND_SLIDING {
        return SE_SLIDING_DOOR;
    } else if sound == DOOR_SOUND_ARENA {
        return SE_REPEL;
    } else {
        return SE_DOOR;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ShouldUseMultiCorridorDoor() -> u8 {
    if FlagGet(FLAG_ENABLE_MULTI_CORRIDOR_DOOR) != 0 {
        if (*gSaveBlock1Ptr).location.mapGroup == 26 && (*gSaveBlock1Ptr).location.mapNum == 16 {
            return TRUE;
        }
    }
    return FALSE;
}
