//! Translated from `src/pokenav.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): PokenavMenuCallbacks

/// `struct PokenavResources`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokenavResources {
    pub currentMenuCb1: Option<unsafe extern "C" fn() -> u32>,
    pub currentMenuIndex: u32,
    pub mode: u16,
    pub conditionSearchId: u16,
    pub hasAnyRibbons: u32,
    pub substructPtrs: CArray<*mut core::ffi::c_void, 19>,
}

unsafe impl Sync for PokenavResources {}

/// `struct PokenavCallbacks`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokenavCallbacks {
    pub init: Option<unsafe extern "C" fn() -> u32>,
    pub callback: Option<unsafe extern "C" fn() -> u32>,
    pub open: Option<unsafe extern "C" fn() -> u32>,
    pub createLoopTask: Option<unsafe extern "C" fn(i32)>,
    pub isLoopTaskActive: Option<unsafe extern "C" fn() -> u32>,
    pub free1: Option<unsafe extern "C" fn()>,
    pub free2: Option<unsafe extern "C" fn()>,
}

unsafe impl Sync for PokenavCallbacks {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokenavResources>() == 92);
    assert!(offset_of!(PokenavResources, currentMenuCb1) == 0);
    assert!(offset_of!(PokenavResources, currentMenuIndex) == 4);
    assert!(offset_of!(PokenavResources, mode) == 8);
    assert!(offset_of!(PokenavResources, conditionSearchId) == 10);
    assert!(offset_of!(PokenavResources, hasAnyRibbons) == 12);
    assert!(offset_of!(PokenavResources, substructPtrs) == 16);
    assert!(size_of::<PokenavCallbacks>() == 28);
    assert!(offset_of!(PokenavCallbacks, init) == 0);
    assert!(offset_of!(PokenavCallbacks, callback) == 4);
    assert!(offset_of!(PokenavCallbacks, open) == 8);
    assert!(offset_of!(PokenavCallbacks, createLoopTask) == 12);
    assert!(offset_of!(PokenavCallbacks, isLoopTaskActive) == 16);
    assert!(offset_of!(PokenavCallbacks, free1) == 20);
    assert!(offset_of!(PokenavCallbacks, free2) == 24);
};

static PokenavMenuCallbacks: Table<CArray<PokenavCallbacks, 15>> =
    Table((&raw const crate::data::pokenav::PokenavMenuCallbacks).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gNextLoopedTaskId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokenavResources: *mut PokenavResources = null_mut();

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gTasks: CArray<Task, 0>;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CheckBoxMonSanityAt(a0: u32, a1: u32) -> u32;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeMenuHandlerSubstruct1();
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
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
    fn SetActiveMenuLoopTasks(a0: *mut c_void, a1: *mut c_void);
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
    let mut taskId: u16 = 0;
    if IsOverworldLinkActive() == 0 {
        taskId = CreateTask(Some(Task_RunLoopedTask), priority as u8) as u16;
    } else {
        taskId = CreateTask(Some(Task_RunLoopedTask_LinkMode), priority as u8) as u16;
    }
    SetWordTaskArg(
        taskId as u8,
        1,
        core::mem::transmute::<_, usize>(loopedTask) as u32,
    );
    gTasks[taskId].data[3] = gNextLoopedTaskId as i16;
    return (({
        let t1 = gNextLoopedTaskId;
        gNextLoopedTaskId += 1;
        t1
    }) as u32)
        << 16
        | taskId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLoopedTaskActive(taskId: u32) -> u32 {
    let mut primaryId: u32 = taskId & 0xFFFF;
    let mut secondaryId: u32 = taskId >> 16;
    if gTasks[primaryId].isActive != 0
        && (gTasks[primaryId].func == Some(Task_RunLoopedTask as unsafe extern "C" fn(u8))
            || gTasks[primaryId].func
                == Some(Task_RunLoopedTask_LinkMode as unsafe extern "C" fn(u8)))
        && gTasks[primaryId].data[3] as u32 == secondaryId
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
pub unsafe extern "C" fn FuncIsActiveLoopedTask(
    func: Option<unsafe extern "C" fn(i32) -> u32>,
) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_TASKS {
        if gTasks[i].isActive != 0
            && (gTasks[i].func == Some(Task_RunLoopedTask as unsafe extern "C" fn(u8))
                || gTasks[i].func == Some(Task_RunLoopedTask_LinkMode as unsafe extern "C" fn(u8)))
            && core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(
                GetWordTaskArg(i as u8, 1) as usize,
            ) == func
        {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Task_RunLoopedTask(taskId: u8) {
    let mut loopedTask: Option<unsafe extern "C" fn(i32) -> u32> =
        core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(GetWordTaskArg(
            taskId, 1,
        ) as usize);
    let mut state: *mut i16 = &raw mut gTasks[taskId].data[0];
    let mut exitLoop: u32 = FALSE as u32;
    while exitLoop == 0 {
        let mut action: u32 = loopedTask.unwrap_unchecked()(*state as i32);
        match action {
            LT_INC_AND_CONTINUE => {
                *state += 1;
            }
            LT_INC_AND_PAUSE => {
                *state += 1;
                return;
            }
            LT_FINISH => {
                DestroyTask(taskId);
                return;
            }
            LT_CONTINUE => {}
            LT_PAUSE => {
                return;
            }
            _ => {
                *state = action as i16 - 5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RunLoopedTask_LinkMode(taskId: u8) {
    let mut task: Option<unsafe extern "C" fn(i32) -> u32> = None;
    let mut state: *mut i16 = null_mut();
    let mut action: u32 = 0;
    if Overworld_IsRecvQueueAtMax() != 0 {
        return;
    }
    task = core::mem::transmute::<usize, Option<unsafe extern "C" fn(i32) -> u32>>(GetWordTaskArg(
        taskId, 1,
    ) as usize);
    state = &raw mut gTasks[taskId].data[0];
    action = task.unwrap_unchecked()(*state as i32);
    match action {
        LT_INC_AND_PAUSE | LT_INC_AND_CONTINUE => {
            *state += 1;
        }
        LT_FINISH => {
            DestroyTask(taskId);
        }
        LT_PAUSE | LT_CONTINUE => {}
        _ => {
            *state = action as i16 - 5;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitPokeNav() {
    gPokenavResources = Alloc(92) as *mut PokenavResources;
    if gPokenavResources.is_null() {
        SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
    } else {
        InitPokenavResources(gPokenavResources);
        ResetTasks();
        SetVBlankCallback(None);
        CreateTask(Some(Task_Pokenav), 0);
        SetMainCallback2(Some(CB2_Pokenav));
        SetVBlankCallback(Some(VBlankCB_Pokenav));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavForTutorial() {
    SetMainCallback2(Some(CB2_InitPokenavForTutorial));
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe extern "C" fn CB2_InitPokenavForTutorial() {
    UpdatePaletteFade();
    if gPaletteFade.active() != 0 {
        return;
    }
    gPokenavResources = Alloc(92) as *mut PokenavResources;
    if gPokenavResources.is_null() {
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    } else {
        InitPokenavResources(gPokenavResources);
        (*gPokenavResources).mode = POKENAV_MODE_FORCE_CALL_READY as u16;
        ResetTasks();
        ResetSpriteData();
        FreeAllSpritePalettes();
        SetVBlankCallback(None);
        CreateTask(Some(Task_Pokenav), 0);
        SetMainCallback2(Some(CB2_Pokenav));
        SetVBlankCallback(Some(VBlankCB_Pokenav));
    }
}
pub(crate) unsafe extern "C" fn FreePokenavResources() {
    let mut i: i32 = 0;
    i = 0;
    while i < POKENAV_SUBSTRUCT_COUNT {
        FreePokenavSubstruct(i as u32);
        i += 1;
    }
    Free(gPokenavResources as *mut c_void);
    gPokenavResources = null_mut();
    InitKeys();
}
pub(crate) unsafe extern "C" fn InitPokenavResources(resources: *mut PokenavResources) {
    let mut i: i32 = 0;
    i = 0;
    while i < POKENAV_SUBSTRUCT_COUNT {
        (*resources).substructPtrs[i] = null_mut();
        i += 1;
    }
    (*resources).mode = POKENAV_MODE_NORMAL;
    (*resources).currentMenuIndex = 0;
    (*resources).hasAnyRibbons = AnyMonHasRibbon();
    (*resources).currentMenuCb1 = None;
}
pub(crate) unsafe extern "C" fn AnyMonHasRibbon() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_HAS_SPECIES) != 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_RIBBON_COUNT) != 0
        {
            return TRUE as u32;
        }
        i += 1;
    }
    j = 0;
    while j < TOTAL_BOXES_COUNT as i32 {
        i = 0;
        while i < IN_BOX_COUNT {
            if CheckBoxMonSanityAt(j as u32, i as u32) != 0
                && GetBoxMonDataAt(j as u8, i as u8, MON_DATA_RIBBON_COUNT) != 0
            {
                return TRUE as u32;
            }
            i += 1;
        }
        j += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CB2_Pokenav() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_Pokenav() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe extern "C" fn Task_Pokenav(taskId: u8) {
    let mut menuId: u32 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            InitPokenavMainMenu();
            *data = 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if PokenavMainMenuLoopedTaskIsActive() != 0 {
                break 'l1;
            }
            SetActivePokenavMenu(POKENAV_MAIN_MENU);
            *data = 4;
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if IsActiveMenuLoopTaskActive() != 0 {
                break 'l1;
            }
            *data = 3;
        }
        if fall || sw1 == 3 {
            fall = true;
            menuId = GetCurrentMenuCB();
            if menuId == POKENAV_MENU_FUNC_EXIT as u32 {
                ShutdownPokenav();
                *data = 5;
            } else if menuId >= POKENAV_MENU_IDS_START {
                PokenavMenuCallbacks[(*gPokenavResources).currentMenuIndex]
                    .free2
                    .unwrap_unchecked()();
                PokenavMenuCallbacks[(*gPokenavResources).currentMenuIndex]
                    .free1
                    .unwrap_unchecked()();
                if SetActivePokenavMenu(menuId) != 0 {
                    *data = 4;
                } else {
                    ShutdownPokenav();
                    *data = 5;
                }
            } else if menuId != 0 {
                RunMainMenuLoopedTask(menuId);
                if IsActiveMenuLoopTaskActive() != 0 {
                    *data = 2;
                }
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if IsActiveMenuLoopTaskActive_() == 0 {
                *data = 3;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if WaitForPokenavShutdownFade() == 0 {
                let mut calledFromScript: u32 =
                    ((*gPokenavResources).mode != POKENAV_MODE_NORMAL) as u32;
                FreeMenuHandlerSubstruct1();
                FreePokenavResources();
                if calledFromScript != 0 {
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                } else {
                    SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
                }
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetActivePokenavMenu(menuId: u32) -> u32 {
    let mut index: u32 = menuId - POKENAV_MENU_IDS_START;
    InitKeys_();
    if PokenavMenuCallbacks[index].init.unwrap_unchecked()() == 0 {
        return FALSE as u32;
    }
    if PokenavMenuCallbacks[index].open.unwrap_unchecked()() == 0 {
        return FALSE as u32;
    }
    SetActiveMenuLoopTasks(
        core::mem::transmute::<Option<unsafe extern "C" fn(i32)>, *mut c_void>(
            PokenavMenuCallbacks[index].createLoopTask,
        ),
        core::mem::transmute::<Option<unsafe extern "C" fn() -> u32>, *mut c_void>(
            PokenavMenuCallbacks[index].isLoopTaskActive,
        ),
    );
    (*gPokenavResources).currentMenuCb1 = PokenavMenuCallbacks[index].callback;
    (*gPokenavResources).currentMenuIndex = index;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn IsActiveMenuLoopTaskActive_() -> u32 {
    return IsActiveMenuLoopTaskActive();
}
pub(crate) unsafe extern "C" fn GetCurrentMenuCB() -> u32 {
    return (*gPokenavResources).currentMenuCb1.unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitKeys_() {
    InitKeys();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVBlankCallback_(callback: Option<unsafe extern "C" fn()>) {
    SetVBlankCallback(callback);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokenavVBlankCallback() {
    SetVBlankCallback(Some(VBlankCB_Pokenav));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocSubstruct(index: u32, size: u32) -> *mut c_void {
    (*gPokenavResources).substructPtrs[index] = Alloc(size);
    return (*gPokenavResources).substructPtrs[index];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSubstructPtr(index: u32) -> *mut c_void {
    return (*gPokenavResources).substructPtrs[index];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreePokenavSubstruct(index: u32) {
    if !(*gPokenavResources).substructPtrs[index].is_null() {
        Free((*gPokenavResources).substructPtrs[index]);
        (*gPokenavResources).substructPtrs[index] = null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokenavMode() -> u32 {
    return (*gPokenavResources).mode as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokenavMode(mode: u16) {
    (*gPokenavResources).mode = mode;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSelectedConditionSearch(cursorPos: u32) {
    let mut searchId: u32 = cursorPos;
    if searchId > 4 {
        searchId = 0;
    }
    (*gPokenavResources).conditionSearchId = searchId as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedConditionSearch() -> u32 {
    return (*gPokenavResources).conditionSearchId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanViewRibbonsMenu() -> u32 {
    return (*gPokenavResources).hasAnyRibbons;
}
