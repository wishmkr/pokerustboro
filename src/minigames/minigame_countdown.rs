//! Translated from `src/minigame_countdown.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): s321Start_Static_Pal s321Start_Static_Gfx sSpriteSheet_321Start_Static sSpritePalette_321Start_Static sAnim_StaticCountdown_Three sAnim_StaticCountdown_Two sAnim_StaticCountdown_One sAnim_StaticCountdown_StartLeft sAnim_StaticCountdown_StartMid sAnim_StaticCountdown_StartRight sAnims_StaticCountdown sSpriteTemplate_StaticCountdown sStaticCountdownFuncs s321Start_Pal s321Start_Gfx sOamData_Numbers sOamData_Start sAnim_Numbers_Three sAnim_Numbers_Two sAnim_Numbers_One sAnimTable_Numbers sAnim_StartLeft sAnim_StartRight sAnimTable_Start sAffineAnim_Numbers_Normal sAffineAnim_Numbers_Squash sAffineAnim_Numbers_Stretch sAffineAnim_Numbers_Land sAffineAnimTable_Numbers
#[allow(unused_imports)]
use crate::data::minigame_countdown::*;

unsafe extern "C" {
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetMultiplayerId() -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn PlaySE(a0: u16);
    fn Rfu_SendPacket(a0: *mut u8);
    fn SetSpriteMatrixAnchor(a0: *mut u8, a1: i16, a2: i16);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
}

pub(crate) unsafe extern "C" fn CreateStaticCountdownTask(funcSetId: u8, taskPriority: u8) -> u32 {
    unsafe {
        let mut funcSetId = funcSetId;
        let mut taskPriority = taskPriority;
        let mut taskId: u8 = CreateTask(Some(Task_StaticCountdown), taskPriority);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(((funcSetId) as i16));
        ((((((&raw const sStaticCountdownFuncs).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((funcSetId) as i32) as isize * 16))
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
        return ((taskId) as u32);
    }
}
pub(crate) unsafe extern "C" fn StartStaticCountdown() -> u32 {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_StaticCountdown));
        if ((taskId) as i32) == 255i32 {
            return 0u32;
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(2i16);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn IsStaticCountdownRunning() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_StaticCountdown))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 2i32 {
                (((((((&raw const sStaticCountdownFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((data).wrapping_offset(1)).read()) as i32) as isize * 16,
                    ))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .wrapping_offset(2))
                .read())
                .unwrap_unchecked()(taskId);
                (data).write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((((((&raw const sStaticCountdownFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((data).wrapping_offset(1)).read()) as i32) as isize * 16,
                    ))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .wrapping_offset(3))
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
            if __sw1 == 4i32 {
                (((((((&raw const sStaticCountdownFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((data).wrapping_offset(1)).read()) as i32) as isize * 16,
                    ))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .wrapping_offset(1))
                .read())
                .unwrap_unchecked()(taskId);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StaticCountdown_CreateSprites(taskId: u8, data: *mut i16) {
    unsafe {
        let mut taskId = taskId;
        let mut data = data;
        let mut i: u8 = 0u8;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        LoadCompressedSpriteSheet(
            (((&raw const sSpriteSheet_321Start_Static)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize * 8),
        );
        LoadSpritePalette(
            (((&raw const sSpritePalette_321Start_Static)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 8),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((data).wrapping_offset(8)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((data).wrapping_offset(((13i32).wrapping_add(((i) as i32))) as isize)).write(
                        ((CreateSprite(
                            (((&raw const sSpriteTemplate_StaticCountdown)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((data).wrapping_offset(2)).read()) as i32) as isize * 24,
                            ),
                            ((data).wrapping_offset(9)).read(),
                            ((data).wrapping_offset(10)).read(),
                            ((((data).wrapping_offset(7)).read()) as u8),
                        )) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((((data).wrapping_offset(8)).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(((13i32).wrapping_add(((i) as i32))) as isize))
                            .read()) as i32) as isize
                            * 68,
                    );
                    crate::c::bf_write(
                        (sprite).wrapping_add(5),
                        2,
                        2,
                        ((((data).wrapping_offset(6)).read()) as u16) as i32,
                    );
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(((data).wrapping_offset(5)).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                        .write(((taskId) as i16));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                        .write(((i) as i16));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                        .write(((data).wrapping_offset(13)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(2)).write(0i16);
        ((data).wrapping_offset(3)).write(0i16);
        ((data).wrapping_offset(4)).write(0i16);
        ((data).wrapping_offset(5)).write(60i16);
        ((data).wrapping_offset(6)).write(0i16);
        ((data).wrapping_offset(7)).write(0i16);
        ((data).wrapping_offset(8)).write(3i16);
        ((data).wrapping_offset(9)).write(120i16);
        ((data).wrapping_offset(10)).write(88i16);
        StaticCountdown_CreateSprites(taskId, data);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(14)).read()) as i32) as isize * 68),
            4u8,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(14)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write((-32i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(15)).read()) as i32) as isize * 68),
            5u8,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(15)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(32i16);
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Free(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((data).wrapping_offset(8)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((data)
                                .wrapping_offset(((13i32).wrapping_add(((i) as i32))) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FreeSpriteTilesByTag(
            (((((&raw const sSpriteSheet_321Start_Static)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        FreeSpritePaletteByTag(
            (((((&raw const sSpritePalette_321Start_Static)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StaticCountdown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>();
        if crate::c::rem_i32(
            ((((data).wrapping_offset(11)).read()) as i32),
            ((((data).wrapping_offset(5)).read()) as i32),
        ) != 0i32
        {
            return;
        }
        if ((((data).wrapping_offset(11)).read()) as i32)
            == ((((data).wrapping_offset(10)).read()) as i32)
        {
            return;
        }
        ((data).wrapping_offset(10)).write(((data).wrapping_offset(11)).read());
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if __fall || __sw1 == 1i32 || __sw1 == 2i32 {
                __fall = true;
                PlaySE(56u16);
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                PlaySE(21u16);
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(14)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(15)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(14)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(15)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                (data).write(4i16);
                return;
            }
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Start(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlaySE(56u16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(13)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_StaticCountdown));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(13)).read()) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(3i16);
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Run(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).read())
                as i32)
                == 32767i32
            {
                ((data).wrapping_offset(11)).write(
                    ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(2))
                        .read()) as i16),
                );
            }
            if ((GetMultiplayerId()) as i32) == 0i32 {
                let __p1 = (data).wrapping_offset(12);
                (__p1).write(((__p1).read()).wrapping_add(1));
                crate::c::memset(((&raw mut packet).cast::<u16>()).cast::<u8>(), 0i32, 12u32);
                ((&raw mut packet).cast::<u16>()).write(32767u16);
                (((&raw mut packet).cast::<u16>()).wrapping_offset(1))
                    .write(((((data).wrapping_offset(12)).read()) as u16));
                Rfu_SendPacket(((&raw mut packet).cast::<u16>()).cast::<u8>());
            }
        } else {
            let __p2 = (data).wrapping_offset(11);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMinigameCountdown(
    tilesTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) {
    unsafe {
        let mut tilesTag = tilesTag;
        let mut palTag = palTag;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut taskId: u8 = CreateTask(Some(Task_MinigameCountdown), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((tilesTag) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((palTag) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(x);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(y);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((subpriority) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMinigameCountdownRunning() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_MinigameCountdown))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_MinigameCountdown(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                Load321StartGfx(
                    ((((data).wrapping_offset(2)).read()) as u16),
                    ((((data).wrapping_offset(3)).read()) as u16),
                );
                ((data).wrapping_offset(7)).write(
                    ((CreateNumberSprite(
                        ((((data).wrapping_offset(2)).read()) as u16),
                        ((((data).wrapping_offset(3)).read()) as u16),
                        ((data).wrapping_offset(4)).read(),
                        ((data).wrapping_offset(5)).read(),
                        ((((data).wrapping_offset(6)).read()) as u8),
                    )) as i16),
                );
                CreateStartSprite(
                    ((((data).wrapping_offset(2)).read()) as u16),
                    ((((data).wrapping_offset(3)).read()) as u16),
                    ((data).wrapping_offset(4)).read(),
                    ((data).wrapping_offset(5)).read(),
                    ((((data).wrapping_offset(6)).read()) as u8),
                    (data).wrapping_offset(8),
                    (data).wrapping_offset(9),
                );
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RunMinigameCountdownDigitsAnim(((((data).wrapping_offset(7)).read()) as u8)))
                    != 0)
                {
                    InitStartGraphic(
                        ((((data).wrapping_offset(7)).read()) as u8),
                        ((((data).wrapping_offset(8)).read()) as u8),
                        ((((data).wrapping_offset(9)).read()) as u8),
                    );
                    FreeSpriteOamMatrix(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(7)).read()) as i32) as isize * 68,
                    ));
                    DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(7)).read()) as i32) as isize * 68,
                    ));
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsStartGraphicAnimRunning(((((data).wrapping_offset(8)).read()) as u8))) != 0)
                {
                    DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(8)).read()) as i32) as isize * 68,
                    ));
                    DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(9)).read()) as i32) as isize * 68,
                    ));
                    FreeSpriteTilesByTag(((((data).wrapping_offset(2)).read()) as u16));
                    FreeSpritePaletteByTag(((((data).wrapping_offset(3)).read()) as u16));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RunMinigameCountdownDigitsAnim(spriteId: u8) -> u32 {
    unsafe {
        let mut spriteId = spriteId;
        let mut sprite: *mut u8 =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetSpriteMatrixAnchor(sprite, 2048i16, 26i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 0i32
                {
                    PlaySE(57u16);
                }
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    >= 20i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    StartSpriteAffineAnim(sprite, 1u8);
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    >= 4i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p9 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    StartSpriteAffineAnim(sprite, 2u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                let __p10 = (sprite).wrapping_add(34).cast::<i16>();
                (__p10).write((((((__p10).read()) as i32).wrapping_sub(4i32)) as i16));
                if (({
                    let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    >= 8i32
                {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        < 2i32
                    {
                        StartSpriteAnim(
                            sprite,
                            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32)
                                .wrapping_add(1i32)) as u8),
                        );
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .write(0i16);
                        let __p13 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p13).write(((__p13).read()).wrapping_add(1));
                    } else {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(7i16);
                        return 0u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                let __p14 = (sprite).wrapping_add(34).cast::<i16>();
                (__p14).write((((((__p14).read()) as i32).wrapping_add(4i32)) as i16));
                if (({
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    >= 8i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    StartSpriteAffineAnim(sprite, 3u8);
                    let __p17 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    let __p18 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn InitStartGraphic(spriteId1: u8, spriteId2: u8, spriteId3: u8) {
    unsafe {
        let mut spriteId1 = spriteId1;
        let mut spriteId2 = spriteId2;
        let mut spriteId3 = spriteId3;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write((-40i16));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write((-40i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Start));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId3) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Start));
    }
}
pub(crate) unsafe extern "C" fn IsStartGraphicAnimRunning(spriteId: u8) -> u32 {
    unsafe {
        let mut spriteId = spriteId;
        return ((core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCB_Start as *const () as usize)) as u32);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Start(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: i32 = 0i32;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((data).wrapping_offset(4)).write(64i16);
                ((data).wrapping_offset(5)).write(
                    ((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p2 = (data).wrapping_offset(5);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(4)).read()) as i32)))
                        as i16),
                );
                let __p3 = (data).wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_add(1));
                ((sprite).wrapping_add(38).cast::<i16>())
                    .write(((((((data).wrapping_offset(5)).read()) as i32) >> 4) as i16));
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
                    PlaySE(57u16);
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p4 = (data).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(12i32)) as i16));
                if ((((data).wrapping_offset(1)).read()) as i32) >= 128i32 {
                    PlaySE(57u16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                y = ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize))
                .read()) as i32);
                ((sprite).wrapping_add(38).cast::<i16>()).write((((y >> 4).wrapping_neg()) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                let __p5 = (data).wrapping_offset(1);
                (__p5).write((((((__p5).read()) as i32).wrapping_add(16i32)) as i16));
                if ((((data).wrapping_offset(1)).read()) as i32) >= 128i32 {
                    PlaySE(57u16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize))
                    .read()) as i32)
                        >> 5)
                        .wrapping_neg()) as i16),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (({
                    let __p6 = (data).wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 40i32
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Load321StartGfx(tileTag: u16, palTag: u16) {
    unsafe {
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u32>()
            .write(
                ((&raw const s321Start_Gfx)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>(),
            );
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(3584u16);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(0u16);
        let mut spritePalette = crate::ffi::Align4([0u8; 8]);
        (&raw mut spritePalette)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const s321Start_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut spritePalette)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(0u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(tileTag);
        (((&raw mut spritePalette).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(palTag);
        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        LoadSpritePalette((&raw mut spritePalette).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateNumberSprite(
    tileTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    unsafe {
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOamData_Numbers).cast::<u8>().cast_mut());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnimTable_Numbers)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAffineAnimTable_Numbers)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(tileTag);
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(palTag);
        return CreateSprite((&raw mut spriteTemplate).cast::<u8>(), x, y, subpriority);
    }
}
pub(crate) unsafe extern "C" fn CreateStartSprite(
    tileTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
    spriteId1: *mut i16,
    spriteId2: *mut i16,
) {
    unsafe {
        let mut tileTag = tileTag;
        let mut palTag = palTag;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut spriteId1 = spriteId1;
        let mut spriteId2 = spriteId2;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOamData_Start).cast::<u8>().cast_mut());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnimTable_Start)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(tileTag);
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(palTag);
        (spriteId1).write(
            ((CreateSprite(
                (&raw mut spriteTemplate).cast::<u8>(),
                ((((x) as i32).wrapping_sub(32i32)) as i16),
                y,
                subpriority,
            )) as i16),
        );
        (spriteId2).write(
            ((CreateSprite(
                (&raw mut spriteTemplate).cast::<u8>(),
                ((((x) as i32).wrapping_add(32i32)) as i16),
                y,
                subpriority,
            )) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteId1).read()) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteId2).read()) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteId2).read()) as i32) as isize * 68),
            1u8,
        );
    }
}
