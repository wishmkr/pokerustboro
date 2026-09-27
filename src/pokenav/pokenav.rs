//! Translated from `src/pokenav.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): PokenavMenuCallbacks
#[allow(unused_imports)]
use crate::data::pokenav::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gNextLoopedTaskId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokenavResources: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gTasks: u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CheckBoxMonSanityAt(a0: u32, a1: u32) -> u32;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeMenuHandlerSubstruct1();
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn InitKeys();
    fn InitPokenavMainMenu() -> u32;
    fn IsActiveMenuLoopTaskActive() -> u32;
    fn IsOverworldLinkActive() -> u32;
    fn LoadOam();
    fn Overworld_IsRecvQueueAtMax() -> u32;
    fn PokenavMainMenuLoopedTaskIsActive() -> u32;
    fn ProcessSpriteCopyRequests();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunMainMenuLoopedTask(a0: u32);
    fn RunTasks();
    fn SetActiveMenuLoopTasks(a0: *mut u8, a1: *mut u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShutdownPokenav();
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn WaitForPokenavShutdownFade() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateLoopedTask(
    loopedTask: Option<unsafe extern "C" fn(i32) -> u32>,
    priority: u32,
) -> u32 {
    unsafe {
        let mut loopedTask = loopedTask;
        let mut priority = priority;
        let mut taskId: u16 = 0u16;
        if !((IsOverworldLinkActive()) != 0) {
            taskId = ((CreateTask(Some(Task_RunLoopedTask), ((priority) as u8))) as u16);
        } else {
            taskId = ((CreateTask(Some(Task_RunLoopedTask_LinkMode), ((priority) as u8))) as u16);
        }
        SetWordTaskArg(
            ((taskId) as u8),
            1u8,
            (core::mem::transmute::<_, usize>(loopedTask) as u32),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((((&raw mut gNextLoopedTaskId).cast::<u8>().cast::<u8>()).read()) as i16));
        return ((((({
            let __p1 = (&raw mut gNextLoopedTaskId).cast::<u8>().cast::<u8>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            << 16)
            | ((taskId) as i32)) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLoopedTaskActive(taskId: u32) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut primaryId: u32 = (taskId & 65535u32);
        let mut secondaryId: u32 = (taskId >> 16);
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((primaryId) as i32) as isize * 40))
        .wrapping_add(4))
        .read())
            != 0)
            && ((core::mem::transmute::<_, usize>(
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((primaryId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read(),
            ) == (Task_RunLoopedTask as *const () as usize))
                || (core::mem::transmute::<_, usize>(
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((primaryId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read(),
                ) == (Task_RunLoopedTask_LinkMode as *const () as usize))))
            && (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((primaryId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u32)
                == secondaryId)
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
pub unsafe extern "C" fn FuncIsActiveLoopedTask(
    func: Option<unsafe extern "C" fn(i32) -> u32>,
) -> u32 {
    unsafe {
        let mut func = func;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                        .wrapping_add(4))
                    .read())
                        != 0)
                        && ((core::mem::transmute::<_, usize>(
                            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset((i) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .read(),
                        ) == (Task_RunLoopedTask as *const () as usize))
                            || (core::mem::transmute::<_, usize>(
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset((i) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .read(),
                            ) == (Task_RunLoopedTask_LinkMode as *const () as usize))))
                        && (core::mem::transmute::<_, usize>(
                            (core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(
                                (GetWordTaskArg(((i) as u8), 1u8)) as usize,
                            )),
                        ) == core::mem::transmute::<_, usize>(func))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_RunLoopedTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut loopedTask: Option<unsafe extern "C" fn(i32) -> u32> =
            (core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(
                (GetWordTaskArg(taskId, 1u8)) as usize,
            ));
        let mut state: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut exitLoop: u32 = 0u32;
        'l1: loop {
            if !(!((exitLoop) != 0)) {
                break 'l1;
            }
            let mut action: u32 = (loopedTask).unwrap_unchecked()((((state).read()) as i32));
            'l2: {
                let __sw1 = action;
                let __matched = __sw1 == 1u32
                    || __sw1 == 0u32
                    || __sw1 == 4u32
                    || __sw1 == 3u32
                    || __sw1 == 2u32;
                if __sw1 == 1u32 {
                    (state).write(((state).read()).wrapping_add(1));
                    break 'l2;
                }
                if __sw1 == 0u32 {
                    (state).write(((state).read()).wrapping_add(1));
                    return;
                }
                if __sw1 == 4u32 {
                    DestroyTask(taskId);
                    return;
                }
                if !__matched {
                    (state).write((((action).wrapping_sub(5u32)) as i16));
                    break 'l2;
                }
                if __sw1 == 3u32 {
                    break 'l2;
                }
                if __sw1 == 2u32 {
                    return;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RunLoopedTask_LinkMode(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: Option<unsafe extern "C" fn(i32) -> u32> = None;
        let mut state: *mut i16 = core::ptr::null_mut();
        let mut action: u32 = 0u32;
        if (Overworld_IsRecvQueueAtMax()) != 0 {
            return;
        }
        task = (core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(
            (GetWordTaskArg(taskId, 1u8)) as usize,
        ));
        state = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        action = (task).unwrap_unchecked()((((state).read()) as i32));
        'l1: {
            let __sw1 = action;
            let __matched =
                __sw1 == 0u32 || __sw1 == 1u32 || __sw1 == 4u32 || __sw1 == 2u32 || __sw1 == 3u32;
            if __sw1 == 0u32 || __sw1 == 1u32 {
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4u32 {
                DestroyTask(taskId);
                break 'l1;
            }
            if !__matched {
                (state).write((((action).wrapping_sub(5u32)) as i16));
                break 'l1;
            }
            if __sw1 == 2u32 || __sw1 == 3u32 {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitPokeNav() {
    unsafe {
        ((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).write(Alloc(92u32));
        if ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read()) as usize)
            == 0usize
        {
            SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
        } else {
            InitPokenavResources(
                ((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read(),
            );
            ResetTasks();
            SetVBlankCallback(None);
            CreateTask(Some(Task_Pokenav), 0u8);
            SetMainCallback2(Some(CB2_Pokenav));
            SetVBlankCallback(Some(VBlankCB_Pokenav));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavForTutorial() {
    unsafe {
        SetMainCallback2(Some(CB2_InitPokenavForTutorial));
        FadeScreen(1u8, 0i8);
    }
}
pub(crate) unsafe extern "C" fn CB2_InitPokenavForTutorial() {
    unsafe {
        UpdatePaletteFade();
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        ((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).write(Alloc(92u32));
        if ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read()) as usize)
            == 0usize
        {
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
        } else {
            InitPokenavResources(
                ((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read(),
            );
            ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .write(1u16);
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(None);
            CreateTask(Some(Task_Pokenav), 0u8);
            SetMainCallback2(Some(CB2_Pokenav));
            SetVBlankCallback(Some(VBlankCB_Pokenav));
        }
    }
}
pub(crate) unsafe extern "C" fn FreePokenavResources() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 19i32) {
                    break 'l1;
                }
                'l2: {
                    FreePokenavSubstruct(((i) as u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            Free(((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        InitKeys();
    }
}
pub(crate) unsafe extern "C" fn InitPokenavResources(resources: *mut u8) {
    unsafe {
        let mut resources = resources;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 19i32) {
                    break 'l1;
                }
                'l2: {
                    ((((resources).wrapping_add(16)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((resources).wrapping_add(8).cast::<u16>()).write(0u16);
        ((resources).wrapping_add(4).cast::<u32>()).write(0u32);
        ((resources).wrapping_add(12).cast::<u32>()).write(AnyMonHasRibbon());
        ((resources).cast::<Option<unsafe extern "C" fn() -> u32>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn AnyMonHasRibbon() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        5i32,
                    )) != 0)
                        && (!((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            6i32,
                        )) != 0)))
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            82i32,
                        ) != 0u32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l3: loop {
                if !(j < 14i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        i = 0i32;
                        'l5: loop {
                            if !(i < 30i32) {
                                break 'l5;
                            }
                            'l6: {
                                if ((CheckBoxMonSanityAt(((j) as u32), ((i) as u32))) != 0)
                                    && (GetBoxMonDataAt(((j) as u8), ((i) as u8), 82i32) != 0u32)
                                {
                                    return 1u32;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_Pokenav() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Pokenav() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
    }
}
pub(crate) unsafe extern "C" fn Task_Pokenav(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut menuId: u32 = 0u32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                InitPokenavMainMenu();
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (PokenavMainMenuLoopedTaskIsActive()) != 0 {
                    break 'l1;
                }
                SetActivePokenavMenu(100000u32);
                (data).write(4i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (IsActiveMenuLoopTaskActive()) != 0 {
                    break 'l1;
                }
                (data).write(3i16);
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                menuId = GetCurrentMenuCB();
                if menuId == 4294967295u32 {
                    ShutdownPokenav();
                    (data).write(5i16);
                } else {
                    if menuId >= 100000u32 {
                        ((((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked()();
                        ((((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(20)
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked()();
                        if (SetActivePokenavMenu(menuId)) != 0 {
                            (data).write(4i16);
                        } else {
                            ShutdownPokenav();
                            (data).write(5i16);
                        }
                    } else {
                        if menuId != 0u32 {
                            RunMainMenuLoopedTask(menuId);
                            if (IsActiveMenuLoopTaskActive()) != 0 {
                                (data).write(2i16);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if !((IsActiveMenuLoopTaskActive_()) != 0) {
                    (data).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if !((WaitForPokenavShutdownFade()) != 0) {
                    let mut calledFromScript: u32 =
                        ((((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8)
                        .cast::<u16>())
                        .read()) as i32)
                            != 0i32) as u32);
                    FreeMenuHandlerSubstruct1();
                    FreePokenavResources();
                    if (calledFromScript) != 0 {
                        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    } else {
                        SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetActivePokenavMenu(menuId: u32) -> u32 {
    unsafe {
        let mut menuId = menuId;
        let mut index: u32 = (menuId).wrapping_sub(100000u32);
        InitKeys_();
        if !((((((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((index) as i32) as isize * 28))
        .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .read())
        .unwrap_unchecked()())
            != 0)
        {
            return 0u32;
        }
        if !((((((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((index) as i32) as isize * 28))
        .wrapping_add(8)
        .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .read())
        .unwrap_unchecked()())
            != 0)
        {
            return 0u32;
        }
        SetActiveMenuLoopTasks(
            core::mem::transmute::<_, *mut u8>(
                (((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 28))
                .wrapping_add(12)
                .cast::<Option<unsafe extern "C" fn(i32)>>())
                .read(),
            ),
            core::mem::transmute::<_, *mut u8>(
                (((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 28))
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn() -> u32>>())
                .read(),
            ),
        );
        ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .write(
            (((((&raw const PokenavMenuCallbacks).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((index) as i32) as isize * 28))
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
            .read(),
        );
        ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u32>())
        .write(index);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn IsActiveMenuLoopTaskActive_() -> u32 {
    unsafe {
        return IsActiveMenuLoopTaskActive();
    }
}
pub(crate) unsafe extern "C" fn GetCurrentMenuCB() -> u32 {
    unsafe {
        return (((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn InitKeys_() {
    unsafe {
        InitKeys();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVBlankCallback_(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        SetVBlankCallback(callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokenavVBlankCallback() {
    unsafe {
        SetVBlankCallback(Some(VBlankCB_Pokenav));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocSubstruct(index: u32, size: u32) -> *mut u8 {
    unsafe {
        let mut index = index;
        let mut size = size;
        ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16))
        .cast::<*mut u8>())
        .wrapping_offset(((index) as i32) as isize))
        .write(Alloc(size));
        return ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16))
        .cast::<*mut u8>())
        .wrapping_offset(((index) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSubstructPtr(index: u32) -> *mut u8 {
    unsafe {
        let mut index = index;
        return ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16))
        .cast::<*mut u8>())
        .wrapping_offset(((index) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreePokenavSubstruct(index: u32) {
    unsafe {
        let mut index = index;
        if ((((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16))
        .cast::<*mut u8>())
        .wrapping_offset(((index) as i32) as isize))
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .cast::<*mut u8>())
                .wrapping_offset(((index) as i32) as isize))
                .read(),
            );
            ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<*mut u8>())
            .wrapping_offset(((index) as i32) as isize))
            .write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavMode() -> u32 {
    unsafe {
        return ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokenavMode(mode: u16) {
    unsafe {
        let mut mode = mode;
        ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(mode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSelectedConditionSearch(cursorPos: u32) {
    unsafe {
        let mut cursorPos = cursorPos;
        let mut searchId: u32 = cursorPos;
        if searchId > 4u32 {
            searchId = 0u32;
        }
        ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<u16>())
        .write(((searchId) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedConditionSearch() -> u32 {
    unsafe {
        return ((((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<u16>())
        .read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanViewRibbonsMenu() -> u32 {
    unsafe {
        return ((((&raw mut gPokenavResources).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<u32>())
        .read();
    }
}
