//! Translated from `src/pokenav.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    clippy::while_immutable_condition,
    unused_assignments
)]

use crate::agb_main::{InitKeys, SetVBlankCallback};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::field_weather::FadeScreen;
use crate::overworld::{
    CB2_ReturnToFieldContinueScriptPlayMapMusic, CB2_ReturnToFieldWithOpenMenu,
    IsOverworldLinkActive, Overworld_IsRecvQueueAtMax,
};
use crate::palette::{TransferPlttBuffer, UpdatePaletteFade, gPaletteFade};
use crate::pokemon::{GetMonData2, gPlayerParty};
use crate::pokemon_storage_system::{CheckBoxMonSanityAt, GetBoxMonDataAt};
use crate::pokenav_main_menu::{
    InitPokenavMainMenu, IsActiveMenuLoopTaskActive, PokenavMainMenuLoopedTaskIsActive,
    RunMainMenuLoopedTask, SetActiveMenuLoopTasks, ShutdownPokenav, WaitForPokenavShutdownFade,
};
use crate::pokenav_menu_handler::FreeMenuHandlerSubstruct1;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::task::gTasks;
use crate::task::{DestroyTask, GetWordTaskArg, ResetTasks, RunTasks, SetWordTaskArg};
use crate::task::{task_data_ptr, task_func, task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// Data tables (translate with cdata.py): PokenavMenuCallbacks

/// `struct PokenavResources`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokenavResources {
    pub currentMenuCb1: Option<unsafe fn() -> u32>,
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
    pub init: Option<unsafe fn() -> u32>,
    pub callback: Option<unsafe fn() -> u32>,
    pub open: Option<unsafe fn() -> u32>,
    pub createLoopTask: Option<unsafe fn(i32)>,
    pub isLoopTaskActive: Option<unsafe fn() -> u32>,
    pub free1: Option<unsafe fn()>,
    pub free2: Option<unsafe fn()>,
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

#[unsafe(link_section = "ewram_data")]
pub static gNextLoopedTaskId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gPokenavResources: *mut PokenavResources = null_mut();

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CreateLoopedTask(loopedTask: Option<unsafe fn(i32) -> u32>, priority: u32) -> u32 {
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
    task_set(taskId, 3, gNextLoopedTaskId.get() as i16);
    (({
        let t1 = gNextLoopedTaskId.get();
        gNextLoopedTaskId.set(gNextLoopedTaskId.get() + 1);
        t1
    }) as u32)
        << 16
        | taskId as u32
}
pub unsafe fn IsLoopedTaskActive(taskId: u32) -> u32 {
    let primaryId: u32 = taskId & 0xFFFF;
    let secondaryId: u32 = taskId >> 16;
    if (*gTasks.as_ptr())[primaryId].isActive != 0
        && (task_func(primaryId) == Some(Task_RunLoopedTask as unsafe fn(u8))
            || task_func(primaryId) == Some(Task_RunLoopedTask_LinkMode as unsafe fn(u8)))
        && task_get(primaryId, 3) as u32 == secondaryId
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FuncIsActiveLoopedTask(func: Option<unsafe fn(i32) -> u32>) -> u32 {
    for i in 0..NUM_TASKS {
        if (*gTasks.as_ptr())[i].isActive != 0
            && (task_func(i) == Some(Task_RunLoopedTask as unsafe fn(u8))
                || task_func(i) == Some(Task_RunLoopedTask_LinkMode as unsafe fn(u8)))
            && core::mem::transmute::<usize, Option<unsafe fn(i32) -> u32>>(GetWordTaskArg(
                i as u8, 1,
            ) as usize)
                == func
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub(crate) unsafe fn Task_RunLoopedTask(taskId: u8) {
    let loopedTask: Option<unsafe fn(i32) -> u32> = core::mem::transmute::<
        usize,
        Option<unsafe fn(i32) -> u32>,
    >(GetWordTaskArg(taskId, 1) as usize);
    let state: *mut i16 = task_data_ptr(taskId, 0);
    let exitLoop: u32 = FALSE as u32;
    while exitLoop == 0 {
        let action: u32 = loopedTask.unwrap_unchecked()(*state as i32);
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
pub(crate) unsafe fn Task_RunLoopedTask_LinkMode(taskId: u8) {
    let mut task: Option<unsafe fn(i32) -> u32> = None;
    if Overworld_IsRecvQueueAtMax() != 0 {
        return;
    }
    task = core::mem::transmute::<usize, Option<unsafe fn(i32) -> u32>>(
        GetWordTaskArg(taskId, 1) as usize
    );
    let state: *mut i16 = task_data_ptr(taskId, 0);
    let action: u32 = task.unwrap_unchecked()(*state as i32);
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
pub unsafe fn CB2_InitPokeNav() {
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
pub unsafe fn OpenPokenavForTutorial() {
    SetMainCallback2(Some(CB2_InitPokenavForTutorial));
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe fn CB2_InitPokenavForTutorial() {
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
unsafe fn FreePokenavResources() {
    for i in 0..POKENAV_SUBSTRUCT_COUNT {
        FreePokenavSubstruct(i as u32);
    }
    Free(gPokenavResources as *mut c_void);
    gPokenavResources = null_mut();
    InitKeys();
}
unsafe fn InitPokenavResources(resources: *mut PokenavResources) {
    for i in 0..POKENAV_SUBSTRUCT_COUNT {
        (*resources).substructPtrs[i] = null_mut();
    }
    (*resources).mode = POKENAV_MODE_NORMAL;
    (*resources).currentMenuIndex = 0;
    (*resources).hasAnyRibbons = AnyMonHasRibbon();
    (*resources).currentMenuCb1 = None;
}
unsafe fn AnyMonHasRibbon() -> u32 {
    let mut i: i32 = 0;
    while i < PARTY_SIZE {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_HAS_SPECIES) != 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_RIBBON_COUNT) != 0
        {
            return TRUE as u32;
        }
        i += 1;
    }
    for j in 0..(TOTAL_BOXES_COUNT as i32) {
        for i in 0..IN_BOX_COUNT {
            if CheckBoxMonSanityAt(j as u32, i as u32) != 0
                && GetBoxMonDataAt(j as u8, i as u8, MON_DATA_RIBBON_COUNT) != 0
            {
                return TRUE as u32;
            }
        }
    }
    FALSE as u32
}
pub(crate) unsafe fn CB2_Pokenav() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_Pokenav() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe fn Task_Pokenav(taskId: u8) {
    let mut menuId: u32 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            InitPokenavMainMenu();
            *data = 1;
            break 'l1;
        }
        if sw1 == 1 {
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
            if IsActiveMenuLoopTaskActive_() == 0 {
                *data = 3;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if WaitForPokenavShutdownFade() == 0 {
                let calledFromScript: u32 =
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
unsafe fn SetActivePokenavMenu(menuId: u32) -> u32 {
    let index: u32 = menuId - POKENAV_MENU_IDS_START;
    InitKeys_();
    if PokenavMenuCallbacks[index].init.unwrap_unchecked()() == 0 {
        return FALSE as u32;
    }
    if PokenavMenuCallbacks[index].open.unwrap_unchecked()() == 0 {
        return FALSE as u32;
    }
    SetActiveMenuLoopTasks(
        core::mem::transmute::<Option<unsafe fn(i32)>, *mut c_void>(
            PokenavMenuCallbacks[index].createLoopTask,
        ),
        core::mem::transmute::<Option<unsafe fn() -> u32>, *mut c_void>(
            PokenavMenuCallbacks[index].isLoopTaskActive,
        ),
    );
    (*gPokenavResources).currentMenuCb1 = PokenavMenuCallbacks[index].callback;
    (*gPokenavResources).currentMenuIndex = index;
    TRUE as u32
}
unsafe fn IsActiveMenuLoopTaskActive_() -> u32 {
    IsActiveMenuLoopTaskActive()
}
unsafe fn GetCurrentMenuCB() -> u32 {
    (*gPokenavResources).currentMenuCb1.unwrap_unchecked()()
}
unsafe fn InitKeys_() {
    InitKeys();
}
pub unsafe fn SetVBlankCallback_(callback: Option<unsafe fn()>) {
    SetVBlankCallback(callback);
}
pub unsafe fn SetPokenavVBlankCallback() {
    SetVBlankCallback(Some(VBlankCB_Pokenav));
}
pub unsafe fn AllocSubstruct(index: u32, size: u32) -> *mut c_void {
    (*gPokenavResources).substructPtrs[index] = Alloc(size);
    (*gPokenavResources).substructPtrs[index]
}
pub unsafe fn GetSubstructPtr(index: u32) -> *mut c_void {
    (*gPokenavResources).substructPtrs[index]
}
pub unsafe fn FreePokenavSubstruct(index: u32) {
    if !(*gPokenavResources).substructPtrs[index].is_null() {
        Free((*gPokenavResources).substructPtrs[index]);
        (*gPokenavResources).substructPtrs[index] = null_mut();
    }
}
pub unsafe fn GetPokenavMode() -> u32 {
    (*gPokenavResources).mode as u32
}
pub unsafe fn SetPokenavMode(mode: u16) {
    (*gPokenavResources).mode = mode;
}
pub unsafe fn SetSelectedConditionSearch(cursorPos: u32) {
    let mut searchId: u32 = cursorPos;
    if searchId > 4 {
        searchId = 0;
    }
    (*gPokenavResources).conditionSearchId = searchId as u16;
}
pub unsafe fn GetSelectedConditionSearch() -> u32 {
    (*gPokenavResources).conditionSearchId as u32
}
pub unsafe fn CanViewRibbonsMenu() -> u32 {
    (*gPokenavResources).hasAnyRibbons
}
