//! Translated from `src/fldeff_flash.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::braille_puzzles::{SetUpPuzzleEffectRegisteel, ShouldDoBrailleRegisteelEffect};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, FlagSet};
use crate::ffi::gSpecialVar_Result;
use crate::field_effect::gFieldEffectArguments;
use crate::fieldmap::gMapHeader;
use crate::fldeff_rocksmash::CreateFieldMoveTask;
use crate::gpu_regs::SetGpuReg;
use crate::overworld::{GetCurrentMapType, GetLastUsedWarpMapType, gFieldCallback2};
use crate::palette::{LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade};
use crate::party_menu::{
    FieldCallback_PrepareFadeInFromMenu, GetCursorSelectionMonId, gPostMenuFieldCallback,
};
use crate::script::ScriptContext_SetupScript;
use crate::sound::PlaySE;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::task::{ResetTasks, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
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
// Data tables (translate with cdata.py): sTransitionTypes sCaveTransitionPalette_White sCaveTransitionPalette_Black sCaveTransitionPalette_Enter sCaveTransitionTilemap sCaveTransitionTiles

/// `struct FlashStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlashStruct {
    pub fromType: u8,
    pub toType: u8,
    pub isEnter: u8,
    pub isExit: u8,
    pub func: Option<unsafe fn()>,
}

unsafe impl Sync for FlashStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FlashStruct>() == 8);
    assert!(offset_of!(FlashStruct, fromType) == 0);
    assert!(offset_of!(FlashStruct, toType) == 1);
    assert!(offset_of!(FlashStruct, isEnter) == 2);
    assert!(offset_of!(FlashStruct, isExit) == 3);
    assert!(offset_of!(FlashStruct, func) == 4);
};

static sCaveTransitionPalette_Black: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::fldeff_flash::sCaveTransitionPalette_Black).cast());
static sCaveTransitionPalette_Enter: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::fldeff_flash::sCaveTransitionPalette_Enter).cast());
static sCaveTransitionPalette_White: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::fldeff_flash::sCaveTransitionPalette_White).cast());
static sCaveTransitionTilemap: Table<CArray<u32, 120>> =
    Table((&raw const crate::data::fldeff_flash::sCaveTransitionTilemap).cast());
static sCaveTransitionTiles: Table<CArray<u32, 45>> =
    Table((&raw const crate::data::fldeff_flash::sCaveTransitionTiles).cast());
static sTransitionTypes: Table<CArray<FlashStruct, 17>> =
    Table((&raw const crate::data::fldeff_flash::sTransitionTypes).cast());

/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn SetUpFieldMove_Flash() -> u8 {
    if ShouldDoBrailleRegisteelEffect() != 0 {
        gSpecialVar_Result = GetCursorSelectionMonId() as u16;
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(SetUpPuzzleEffectRegisteel);
        return TRUE;
    } else if gMapHeader.cave == TRUE && FlagGet(FLAG_SYS_USE_FLASH) == 0 {
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_Flash);
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn FieldCallback_Flash() {
    let taskId: u8 = CreateFieldMoveTask();
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    task_set(
        taskId,
        8,
        (FldEff_UseFlash as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        FldEff_UseFlash as *const () as usize as u32 as i16,
    );
}
pub(crate) unsafe fn FldEff_UseFlash() {
    PlaySE(SE_M_REFLECT);
    FlagSet(FLAG_SYS_USE_FLASH);
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_UseFlash.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn CB2_ChangeMapMain() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBC_ChangeMapVBlank() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub unsafe fn CB2_DoChangeMap() {
    SetVBlankCallback(None);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x8100c000);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        OAM as i32 as usize as *mut c_void as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x85000100);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), 83886082_usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x810001ff);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    ResetPaletteFade();
    ResetTasks();
    ResetSpriteData();
    {
        let imeTemp: u16 = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        volatile_write(
            0x4000200_usize as *mut u16,
            (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384_usize as *mut u16, imeTemp);
    }
    SetVBlankCallback(Some(VBC_ChangeMapVBlank));
    SetMainCallback2(Some(CB2_ChangeMapMain));
    if TryDoMapTransition() == 0 {
        SetMainCallback2(gMain.savedCallback);
    }
}
unsafe fn TryDoMapTransition() -> u8 {
    let fromType: u8 = GetLastUsedWarpMapType();
    let toType: u8 = GetCurrentMapType();
    let mut i: u8 = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            sTransitionTypes[i].func.unwrap_unchecked()();
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
pub unsafe fn GetMapPairFadeToType(_fromType: u8, _toType: u8) -> u8 {
    let fromType: u8 = _fromType;
    let toType: u8 = _toType;
    let mut i: u8 = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            return sTransitionTypes[i].isEnter;
        }
        i += 1;
    }
    FALSE
}
pub unsafe fn GetMapPairFadeFromType(_fromType: u8, _toType: u8) -> u8 {
    let fromType: u8 = _fromType;
    let toType: u8 = _toType;
    let mut i: u8 = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            return sTransitionTypes[i].isExit;
        }
        i += 1;
    }
    FALSE
}
pub(crate) unsafe fn DoExitCaveTransition() {
    CreateTask(Some(Task_ExitCaveTransition1), 0);
}
pub(crate) unsafe fn Task_ExitCaveTransition1(taskId: u8) {
    task_set_func(taskId, Some(Task_ExitCaveTransition2));
}
pub(crate) unsafe fn Task_ExitCaveTransition2(taskId: u8) {
    SetGpuReg(0x0, 0);
    LZ77UnCompVram(
        sCaveTransitionTiles.as_ptr().cast_mut(),
        0x600c000_usize as *mut c_void,
    );
    LZ77UnCompVram(
        sCaveTransitionTilemap.as_ptr().cast_mut(),
        0x600f800_usize as *mut c_void,
    );
    LoadPalette(
        sCaveTransitionPalette_White.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
    LoadPalette(
        (&raw const sCaveTransitionPalette_Enter[8]).cast_mut() as *mut c_void,
        224,
        16,
    );
    SetGpuReg(REG_OFFSET_BLDCNT, 15937);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 7948);
    SetGpuReg(0x0, 4416);
    task_set_func(taskId, Some(Task_ExitCaveTransition3));
    task_set(taskId, 0, 16);
    task_set(taskId, 1, 0);
}
pub(crate) unsafe fn Task_ExitCaveTransition3(taskId: u8) {
    let count: u16 = task_get(taskId, 1) as u16;
    let blend: u16 = count + 0x1000;
    SetGpuReg(REG_OFFSET_BLDALPHA, blend);
    if count <= 16 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        task_set(taskId, 2, 0);
        task_set_func(taskId, Some(Task_ExitCaveTransition4));
    }
}
pub(crate) unsafe fn Task_ExitCaveTransition4(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDALPHA, 4112);
    let count: u16 = task_get(taskId, 2) as u16;
    if count < 8 {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
        LoadPalette(
            (&raw const sCaveTransitionPalette_Enter[8 + count as i32]).cast_mut() as *mut c_void,
            224,
            16 - count * 2,
        );
    } else {
        LoadPalette(
            sCaveTransitionPalette_White.as_ptr().cast_mut() as *mut c_void,
            0,
            32,
        );
        task_set_func(taskId, Some(Task_ExitCaveTransition5));
        task_set(taskId, 2, 8);
    }
}
pub(crate) unsafe fn Task_ExitCaveTransition5(taskId: u8) {
    if task_get(taskId, 2) != 0 {
        task_set(taskId, 2, task_get(taskId, 2) - 1);
    } else {
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe fn DoEnterCaveTransition() {
    CreateTask(Some(Task_EnterCaveTransition1), 0);
}
pub(crate) unsafe fn Task_EnterCaveTransition1(taskId: u8) {
    task_set_func(taskId, Some(Task_EnterCaveTransition2));
}
pub(crate) unsafe fn Task_EnterCaveTransition2(taskId: u8) {
    SetGpuReg(0x0, 0);
    LZ77UnCompVram(
        sCaveTransitionTiles.as_ptr().cast_mut(),
        0x600c000_usize as *mut c_void,
    );
    LZ77UnCompVram(
        sCaveTransitionTilemap.as_ptr().cast_mut(),
        0x600f800_usize as *mut c_void,
    );
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 7948);
    SetGpuReg(0x0, 4416);
    LoadPalette(
        sCaveTransitionPalette_White.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
    LoadPalette(
        sCaveTransitionPalette_Black.as_ptr().cast_mut() as *mut c_void,
        0,
        32,
    );
    task_set_func(taskId, Some(Task_EnterCaveTransition3));
    task_set(taskId, 0, 16);
    task_set(taskId, 1, 0);
    task_set(taskId, 2, 0);
}
pub(crate) unsafe fn Task_EnterCaveTransition3(taskId: u8) {
    let count: u16 = task_get(taskId, 2) as u16;
    if count < 16 {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
        task_set(taskId, 2, task_get(taskId, 2) + 1);
        LoadPalette(
            (&raw const sCaveTransitionPalette_Enter[15 - count as i32]).cast_mut() as *mut c_void,
            224,
            (count + 1) * 2,
        );
    } else {
        SetGpuReg(REG_OFFSET_BLDALPHA, 4112);
        SetGpuReg(REG_OFFSET_BLDCNT, 15937);
        task_set_func(taskId, Some(Task_EnterCaveTransition4));
    }
}
pub(crate) unsafe fn Task_EnterCaveTransition4(taskId: u8) {
    let count: u16 = 16 - task_get(taskId, 1) as u16;
    let blend: u16 = count + 0x1000;
    SetGpuReg(REG_OFFSET_BLDALPHA, blend);
    if count != 0 {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
    } else {
        LoadPalette(
            sCaveTransitionPalette_Black.as_ptr().cast_mut() as *mut c_void,
            0,
            32,
        );
        SetMainCallback2(gMain.savedCallback);
    }
}
