//! Translated from `src/field_door.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDoorAnimTiles_Littleroot sDoorNullPalette1 sDoorAnimTiles_BirchsLab sDoorNullPalette2 sDoorAnimTiles_FallarborLightRoof sDoorNullPalette3 sDoorAnimTiles_Lilycove sDoorNullPalette4 sDoorAnimTiles_LilycoveWooden sDoorNullPalette5 sDoorAnimTiles_General sDoorNullPalette6 sDoorAnimTiles_PokeCenter sDoorAnimTiles_Gym sDoorAnimTiles_PokeMart sDoorAnimTiles_RustboroTan sDoorNullPalette7 sDoorAnimTiles_RustboroGray sDoorNullPalette8 sDoorAnimTiles_Oldale sFiller1 sDoorAnimTiles_UnusedTops sFiller2 sDoorAnimTiles_UnusedBottoms sDoorNullPalette11 sDoorAnimTiles_Mauville sDoorNullPalette12 sDoorAnimTiles_Verdanturf sDoorNullPalette13 sDoorAnimTiles_Slateport sDoorNullPalette14 sDoorAnimTiles_Dewford sDoorNullPalette15 sDoorAnimTiles_Contest sDoorNullPalette16 sDoorAnimTiles_Mossdeep sDoorNullPalette17 sDoorAnimTiles_SootopolisPeakedRoof sDoorNullPalette18 sDoorAnimTiles_Sootopolis sDoorNullPalette19 sDoorAnimTiles_PokemonLeague sDoorNullPalette20 sDoorAnimTiles_Pacifidlog sDoorNullPalette21 sDoorAnimTiles_PetalburgGym sDoorNullPalette22 sDoorAnimTiles_CyclingRoad sDoorNullPalette23 sDoorAnimTiles_LilycoveDeptStore sDoorNullPalette24 sDoorAnimTiles_SafariZone sDoorNullPalette25 sDoorAnimTiles_MossdeepSpaceCenter sDoorNullPalette26 sDoorAnimTiles_CableClub sDoorNullPalette27 sDoorAnimTiles_AbandonedShip sDoorNullPalette28 sDoorAnimTiles_FallarborDarkRoof sDoorNullPalette29 sDoorAnimTiles_AbandonedShipRoom sDoorNullPalette30 sDoorAnimTiles_LilycoveDeptStoreElevator sDoorNullPalette31 sDoorAnimTiles_BattleTowerOld sDoorNullPalette32 sDoorAnimTiles_BattleTowerElevator sDoorNullPalette33 sDoorAnimTiles_UnusedBattleFrontier sDoorNullPalette34 sDoorAnimTiles_BattleDome sDoorNullPalette35 sDoorAnimTiles_BattleFactory sDoorNullPalette36 sDoorAnimTiles_BattleTower sDoorNullPalette37 sDoorAnimTiles_BattleArena sDoorNullPalette38 sDoorAnimTiles_BattleArenaLobby sDoorNullPalette39 sDoorAnimTiles_BattleDomeLobby sDoorNullPalette40 sDoorAnimTiles_BattlePalaceLobby sDoorAnimTiles_BattleTent sDoorNullPalette41 sDoorAnimTiles_BattleDomeCorridor sDoorNullPalette42 sDoorAnimTiles_BattleTowerMultiCorridor sDoorNullPalette43 sDoorAnimTiles_BattleFrontier sDoorNullPalette44 sDoorAnimTiles_BattleFrontierSliding sDoorNullPalette45 sDoorAnimTiles_BattleDomePreBattleRoom sDoorNullPalette46 sDoorAnimTiles_BattleTentInterior sDoorNullPalette47 sDoorAnimTiles_TrainerHillLobbyElevator sDoorNullPalette48 sDoorAnimTiles_TrainerHillRoofElevator sDoorNullPalette49 sDoorOpenAnimFrames sDoorCloseAnimFrames sBigDoorOpenAnimFrames sBigDoorCloseAnimFrames sDoorAnimPalettes_General sDoorAnimPalettes_PokeCenter sDoorAnimPalettes_Gym sDoorAnimPalettes_PokeMart sDoorAnimPalettes_Littleroot sDoorAnimPalettes_BirchsLab sDoorAnimPalettes_RustboroTan sDoorAnimPalettes_RustboroGray sDoorAnimPalettes_FallarborLightRoof sDoorAnimPalettes_Lilycove sDoorAnimPalettes_Oldale sDoorAnimPalettes_Mossdeep sDoorAnimPalettes_PokemonLeague sDoorAnimPalettes_Pacifidlog sDoorAnimPalettes_SootopolisPeakedRoof sDoorAnimPalettes_Sootopolis sDoorAnimPalettes_Dewford sDoorAnimPalettes_Slateport sDoorAnimPalettes_Mauville sDoorAnimPalettes_Verdanturf sDoorAnimPalettes_LilycoveWooden sDoorAnimPalettes_Contest sDoorAnimPalettes_PetalburgGym sDoorAnimPalettes_CyclingRoad sDoorAnimPalettes_LilycoveDeptStore sDoorAnimPalettes_SafariZone sDoorAnimPalettes_MossdeepSpaceCenter sDoorAnimPalettes_CableClub sDoorAnimPalettes_AbandonedShip sDoorAnimPalettes_FallarborDarkRoof sDoorAnimPalettes_AbandonedShipRoom sDoorAnimPalettes_LilycoveDeptStoreElevator sDoorAnimPalettes_BattleTowerOld sDoorAnimPalettes_BattleTowerElevator sDoorAnimPalettes_UnusedBattleFrontier sDoorAnimPalettes_BattleDome sDoorAnimPalettes_BattleFactory sDoorAnimPalettes_BattleTower sDoorAnimPalettes_BattleArena sDoorAnimPalettes_BattleArenaLobby sDoorAnimPalettes_BattleDomeLobby sDoorAnimPalettes_BattlePalaceLobby sDoorAnimPalettes_BattleTent sDoorAnimPalettes_BattleDomeCorridor sDoorAnimPalettes_BattleTowerMultiCorridor sDoorAnimPalettes_Unused sDoorAnimPalettes_BattleFrontier sDoorAnimPalettes_BattleDomePreBattleRoom sDoorAnimPalettes_BattleTentInterior sDoorAnimPalettes_TrainerHillLobbyElevator sDoorAnimPalettes_TrainerHillRoofElevator sDoorAnimGraphicsTable
#[allow(unused_imports)]
use crate::data::field_door::*;

unsafe extern "C" {
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gTasks: u8;
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
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

pub(crate) unsafe extern "C" fn CopyDoorTilesToVram(gfx: *mut u8, frame: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut frame = frame;
        if ((((gfx).wrapping_add(3)).read()) as i32) == 2i32 {
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (((gfx).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(
                            ((((frame).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 1,
                        ),
                        (((100663296i32)
                            .wrapping_add((1008i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                            as usize as *mut u8),
                        ((crate::c::div_i32(
                            (16i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                            crate::c::div_i32(32i32, 8i32),
                        ) & 2097151i32) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        } else {
            'l3: loop {
                'l4: {
                    CpuFastSet(
                        (((gfx).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(
                            ((((frame).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 1,
                        ),
                        (((100663296i32)
                            .wrapping_add((1016i32).wrapping_mul(crate::c::div_i32(256i32, 8i32))))
                            as usize as *mut u8),
                        ((crate::c::div_i32(
                            (8i32).wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                            crate::c::div_i32(32i32, 8i32),
                        ) & 2097151i32) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l3;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuildDoorTiles(
    tiles: *mut u16,
    tileNum: u16,
    paletteNums: *mut u8,
) {
    unsafe {
        let mut tiles = tiles;
        let mut tileNum = tileNum;
        let mut paletteNums = paletteNums;
        let mut i: i32 = 0i32;
        let mut tile: u16 = 0u16;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    tile = ((((({
                        let __t2 = paletteNums;
                        paletteNums = (paletteNums).wrapping_offset(1);
                        __t2
                    })
                    .read()) as i32)
                        << 12) as u16);
                    ((tiles).wrapping_offset((i) as isize))
                        .write(((((tile) as i32) | ((tileNum) as i32).wrapping_add(i)) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    tile = ((((({
                        let __t4 = paletteNums;
                        paletteNums = (paletteNums).wrapping_offset(1);
                        __t4
                    })
                    .read()) as i32)
                        << 12) as u16);
                    ((tiles).wrapping_offset((i) as isize)).write(tile);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawCurrentDoorAnimFrame(
    gfx: *mut u8,
    x: u32,
    y: u32,
    paletteNums: *mut u8,
) {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        let mut paletteNums = paletteNums;
        let mut tiles = crate::ffi::Align4([0u8; 48]);
        if ((((gfx).wrapping_add(3)).read()) as i32) == 2i32 {
            BuildDoorTiles(
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
                1008u16,
                paletteNums,
            );
            DrawDoorMetatileAt(
                ((x) as i32),
                (((y).wrapping_sub(1u32)) as i32),
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
            );
            BuildDoorTiles(
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
                1012u16,
                (paletteNums).wrapping_offset(4),
            );
            DrawDoorMetatileAt(
                ((x) as i32),
                ((y) as i32),
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
            );
            BuildDoorTiles(
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
                1016u16,
                paletteNums,
            );
            DrawDoorMetatileAt(
                (((x).wrapping_add(1u32)) as i32),
                (((y).wrapping_sub(1u32)) as i32),
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
            );
            BuildDoorTiles(
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
                1020u16,
                (paletteNums).wrapping_offset(4),
            );
            DrawDoorMetatileAt(
                (((x).wrapping_add(1u32)) as i32),
                ((y) as i32),
                ((&raw mut tiles).cast::<u16>()).wrapping_offset(8),
            );
        } else {
            BuildDoorTiles((&raw mut tiles).cast::<u16>(), 1016u16, paletteNums);
            DrawDoorMetatileAt(
                ((x) as i32),
                (((y).wrapping_sub(1u32)) as i32),
                (&raw mut tiles).cast::<u16>(),
            );
            BuildDoorTiles(
                (&raw mut tiles).cast::<u16>(),
                1020u16,
                (paletteNums).wrapping_offset(4),
            );
            DrawDoorMetatileAt(((x) as i32), ((y) as i32), (&raw mut tiles).cast::<u16>());
        }
    }
}
pub(crate) unsafe extern "C" fn DrawClosedDoorTiles(gfx: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        CurrentMapDrawMetatileAt(((x) as i32), (((y).wrapping_sub(1u32)) as i32));
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
        if ((((gfx).wrapping_add(3)).read()) as i32) == 2i32 {
            CurrentMapDrawMetatileAt(
                (((x).wrapping_add(1u32)) as i32),
                (((y).wrapping_sub(1u32)) as i32),
            );
            CurrentMapDrawMetatileAt((((x).wrapping_add(1u32)) as i32), ((y) as i32));
        }
    }
}
pub(crate) unsafe extern "C" fn DrawDoor(gfx: *mut u8, frame: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut gfx = gfx;
        let mut frame = frame;
        let mut x = x;
        let mut y = y;
        if ((((frame).wrapping_add(2).cast::<u16>()).read()) as i32) == 65535i32 {
            DrawClosedDoorTiles(gfx, x, y);
            if (ShouldUseMultiCorridorDoor()) != 0 {
                DrawClosedDoorTiles(
                    gfx,
                    ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                        .wrapping_add(7i32)) as u32),
                    ((((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                        .wrapping_add(7i32)) as u32),
                );
            }
        } else {
            CopyDoorTilesToVram(gfx, frame);
            DrawCurrentDoorAnimFrame(gfx, x, y, ((gfx).wrapping_add(8).cast::<*mut u8>()).read());
            if (ShouldUseMultiCorridorDoor()) != 0 {
                DrawCurrentDoorAnimFrame(
                    gfx,
                    ((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                        .wrapping_add(7i32)) as u32),
                    ((((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                        .wrapping_add(7i32)) as u32),
                    ((gfx).wrapping_add(8).cast::<*mut u8>()).read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimateDoorFrame(
    gfx: *mut u8,
    frames: *mut u8,
    data: *mut i16,
) -> u32 {
    unsafe {
        let mut gfx = gfx;
        let mut frames = frames;
        let mut data = data;
        if ((((data).wrapping_offset(5)).read()) as i32) == 0i32 {
            DrawDoor(
                gfx,
                (frames)
                    .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 4),
                ((((data).wrapping_offset(6)).read()) as u32),
                ((((data).wrapping_offset(7)).read()) as u32),
            );
        }
        if ((((data).wrapping_offset(5)).read()) as i32)
            == ((((frames)
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 4))
            .read()) as i32)
        {
            ((data).wrapping_offset(5)).write(0i16);
            let __p1 = (data).wrapping_offset(4);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((frames)
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 4))
            .read()) as i32)
                == 0i32
            {
                return 0u32;
            } else {
                return 1u32;
            }
        }
        let __p2 = (data).wrapping_offset(5);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateDoor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        let mut frames: *mut u8 = ((((((data).read()) as i32) << 16)
            | ((((data).wrapping_offset(1)).read()) as i32))
            as usize as *mut u8);
        let mut gfx: *mut u8 = (((((((data).wrapping_offset(2)).read()) as i32) << 16)
            | ((((data).wrapping_offset(3)).read()) as i32))
            as usize as *mut u8);
        if AnimateDoorFrame(gfx, frames, (data).cast::<i16>()) == 0u32 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn GetLastDoorFrame(frame: *mut u8, unused: *mut u8) -> *mut u8 {
    unsafe {
        let mut frame = frame;
        let mut unused = unused;
        'l1: loop {
            if !((((frame).read()) as i32) != 0i32) {
                break 'l1;
            }
            frame = (frame).wrapping_offset(4);
        }
        return (frame).wrapping_offset(-4);
    }
}
pub(crate) unsafe extern "C" fn GetDoorGraphics(gfx: *mut u8, metatileNum: u16) -> *mut u8 {
    unsafe {
        let mut gfx = gfx;
        let mut metatileNum = metatileNum;
        'l1: loop {
            if !(((((gfx).wrapping_add(4).cast::<*mut u8>()).read()) as usize) != 0usize) {
                break 'l1;
            }
            if ((((gfx).cast::<u16>()).read()) as i32) == ((metatileNum) as i32) {
                return gfx;
            }
            gfx = (gfx).wrapping_offset(12);
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn StartDoorAnimationTask(
    gfx: *mut u8,
    frames: *mut u8,
    x: u32,
    y: u32,
) -> i8 {
    unsafe {
        let mut gfx = gfx;
        let mut frames = frames;
        let mut x = x;
        let mut y = y;
        if ((FuncIsActiveTask(Some(Task_AnimateDoor))) as i32) == 1i32 {
            return (-1i8);
        } else {
            let mut taskId: u8 = CreateTask(Some(Task_AnimateDoor), 80u8);
            let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            ((data).wrapping_offset(6)).write(((x) as i16));
            ((data).wrapping_offset(7)).write(((y) as i16));
            ((data).wrapping_offset(1)).write((((frames) as usize as u32) as i16));
            (data).write(((((frames) as usize as u32) >> 16) as i16));
            ((data).wrapping_offset(3)).write((((gfx) as usize as u32) as i16));
            ((data).wrapping_offset(2)).write(((((gfx) as usize as u32) >> 16) as i16));
            return ((taskId) as i8);
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn DrawClosedDoor(gfx: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        DrawClosedDoorTiles(gfx, x, y);
    }
}
pub(crate) unsafe extern "C" fn DrawOpenedDoor(gfx: *mut u8, x: u32, y: u32) {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        gfx = GetDoorGraphics(
            gfx,
            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16),
        );
        if ((gfx) as usize) != 0usize {
            DrawDoor(
                gfx,
                GetLastDoorFrame(
                    ((&raw const sDoorOpenAnimFrames).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((&raw const sDoorOpenAnimFrames).cast::<u8>().cast_mut()).cast::<u8>(),
                ),
                x,
                y,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn StartDoorOpenAnimation(gfx: *mut u8, x: u32, y: u32) -> i8 {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        gfx = GetDoorGraphics(
            gfx,
            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16),
        );
        if ((gfx) as usize) == 0usize {
            return (-1i8);
        } else {
            if ((((gfx).wrapping_add(3)).read()) as i32) == 2i32 {
                return StartDoorAnimationTask(
                    gfx,
                    ((&raw const sBigDoorOpenAnimFrames).cast::<u8>().cast_mut()).cast::<u8>(),
                    x,
                    y,
                );
            } else {
                return StartDoorAnimationTask(
                    gfx,
                    ((&raw const sDoorOpenAnimFrames).cast::<u8>().cast_mut()).cast::<u8>(),
                    x,
                    y,
                );
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn StartDoorCloseAnimation(gfx: *mut u8, x: u32, y: u32) -> i8 {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        gfx = GetDoorGraphics(
            gfx,
            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16),
        );
        if ((gfx) as usize) == 0usize {
            return (-1i8);
        } else {
            return StartDoorAnimationTask(
                gfx,
                ((&raw const sDoorCloseAnimFrames).cast::<u8>().cast_mut()).cast::<u8>(),
                x,
                y,
            );
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetDoorSoundType(gfx: *mut u8, x: u32, y: u32) -> i8 {
    unsafe {
        let mut gfx = gfx;
        let mut x = x;
        let mut y = y;
        gfx = GetDoorGraphics(
            gfx,
            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16),
        );
        if ((gfx) as usize) == 0usize {
            return (-1i8);
        } else {
            return ((((gfx).wrapping_add(2)).read()) as i8);
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
pub(crate) unsafe extern "C" fn Debug_FieldAnimateDoorOpen(x: u32, y: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        StartDoorOpenAnimation(
            ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldSetDoorOpened(x: u32, y: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (MetatileBehavior_IsDoor(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) != 0
        {
            DrawOpenedDoor(
                ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
                x,
                y,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldSetDoorClosed(x: u32, y: u32) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (MetatileBehavior_IsDoor(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) != 0
        {
            DrawClosedDoor(
                ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
                x,
                y,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldAnimateDoorClose(x: u32, y: u32) -> i8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if !((MetatileBehavior_IsDoor(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) != 0)
        {
            return (-1i8);
        } else {
            return StartDoorCloseAnimation(
                ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
                x,
                y,
            );
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldAnimateDoorOpen(x: u32, y: u32) -> i8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if !((MetatileBehavior_IsDoor(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) != 0)
        {
            return (-1i8);
        } else {
            return StartDoorOpenAnimation(
                ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
                x,
                y,
            );
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldIsDoorAnimationRunning() -> u8 {
    unsafe {
        return FuncIsActiveTask(Some(Task_AnimateDoor));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDoorSoundEffect(x: u32, y: u32) -> u32 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut sound: i32 = ((GetDoorSoundType(
            ((&raw const sDoorAnimGraphicsTable).cast::<u8>().cast_mut()).cast::<u8>(),
            x,
            y,
        )) as i32);
        if sound == 0i32 {
            return 8u32;
        } else {
            if sound == 1i32 {
                return 18u32;
            } else {
                if sound == 2i32 {
                    return 47u32;
                } else {
                    return 8u32;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ShouldUseMultiCorridorDoor() -> u8 {
    unsafe {
        if (FlagGet(16386u16)) != 0 {
            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 16i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
