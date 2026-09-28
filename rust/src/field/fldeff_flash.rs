//! Translated from `src/fldeff_flash.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sTransitionTypes sCaveTransitionPalette_White sCaveTransitionPalette_Black sCaveTransitionPalette_Enter sCaveTransitionTilemap sCaveTransitionTiles

/// `struct FlashStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FlashStruct {
    pub fromType: u8,
    pub toType: u8,
    pub isEnter: u8,
    pub isExit: u8,
    pub func: Option<unsafe extern "C" fn()>,
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

unsafe extern "C" {
    static EventScript_UseFlash: CArray<u8, 0>;
    static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gPostMenuFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gSpecialVar_Result: u16;
    static mut gTasks: CArray<Task, 0>;
    fn AnimateSprites();
    fn BuildOamBuffer();
    fn CreateFieldMoveTask() -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn GetCurrentMapType() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetLastUsedWarpMapType() -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetUpPuzzleEffectRegisteel();
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShouldDoBrailleRegisteelEffect() -> u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_Flash() -> u8 {
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
    return FALSE;
}
pub(crate) unsafe extern "C" fn FieldCallback_Flash() {
    let mut taskId: u8 = CreateFieldMoveTask();
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    gTasks[taskId].data[8] = (FldEff_UseFlash as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[9] = FldEff_UseFlash as *const () as usize as u32 as i16;
}
pub(crate) unsafe extern "C" fn FldEff_UseFlash() {
    PlaySE(SE_M_REFLECT);
    FlagSet(FLAG_SYS_USE_FLASH);
    ScriptContext_SetupScript(EventScript_UseFlash.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn CB2_ChangeMapMain() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBC_ChangeMapVBlank() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoChangeMap() {
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        83886082 as usize as *mut c_void as usize as u32,
                    );
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
        let mut imeTemp: u16 = 0;
        imeTemp = (67109384 as usize as *mut u16).read_volatile();
        volatile_write(67109384 as usize as *mut u16, 0);
        volatile_write(
            0x4000200 as usize as *mut u16,
            (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384 as usize as *mut u16, imeTemp);
    }
    SetVBlankCallback(Some(VBC_ChangeMapVBlank));
    SetMainCallback2(Some(CB2_ChangeMapMain));
    if TryDoMapTransition() == 0 {
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn TryDoMapTransition() -> u8 {
    let mut i: u8 = 0;
    let mut fromType: u8 = GetLastUsedWarpMapType();
    let mut toType: u8 = GetCurrentMapType();
    i = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            sTransitionTypes[i].func.unwrap_unchecked()();
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapPairFadeToType(_fromType: u8, _toType: u8) -> u8 {
    let mut i: u8 = 0;
    let mut fromType: u8 = _fromType;
    let mut toType: u8 = _toType;
    i = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            return sTransitionTypes[i].isEnter;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapPairFadeFromType(_fromType: u8, _toType: u8) -> u8 {
    let mut i: u8 = 0;
    let mut fromType: u8 = _fromType;
    let mut toType: u8 = _toType;
    i = 0;
    while sTransitionTypes[i].fromType != 0 {
        if sTransitionTypes[i].fromType == fromType && sTransitionTypes[i].toType == toType {
            return sTransitionTypes[i].isExit;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DoExitCaveTransition() {
    CreateTask(Some(Task_ExitCaveTransition1), 0);
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition1(taskId: u8) {
    gTasks[taskId].func = Some(Task_ExitCaveTransition2);
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition2(taskId: u8) {
    SetGpuReg(0x0, 0);
    LZ77UnCompVram(
        sCaveTransitionTiles.as_ptr().cast_mut(),
        0x600c000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sCaveTransitionTilemap.as_ptr().cast_mut(),
        0x600f800 as usize as *mut c_void,
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
    gTasks[taskId].func = Some(Task_ExitCaveTransition3);
    gTasks[taskId].data[0] = 16;
    gTasks[taskId].data[1] = 0;
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition3(taskId: u8) {
    let mut count: u16 = gTasks[taskId].data[1] as u16;
    let mut blend: u16 = count + 0x1000;
    SetGpuReg(REG_OFFSET_BLDALPHA, blend);
    if count <= 16 {
        gTasks[taskId].data[1] += 1;
    } else {
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].func = Some(Task_ExitCaveTransition4);
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition4(taskId: u8) {
    let mut count: u16 = 0;
    SetGpuReg(REG_OFFSET_BLDALPHA, 4112);
    count = gTasks[taskId].data[2] as u16;
    if count < 8 {
        gTasks[taskId].data[2] += 1;
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
        gTasks[taskId].func = Some(Task_ExitCaveTransition5);
        gTasks[taskId].data[2] = 8;
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition5(taskId: u8) {
    if gTasks[taskId].data[2] != 0 {
        gTasks[taskId].data[2] -= 1;
    } else {
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn DoEnterCaveTransition() {
    CreateTask(Some(Task_EnterCaveTransition1), 0);
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition1(taskId: u8) {
    gTasks[taskId].func = Some(Task_EnterCaveTransition2);
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition2(taskId: u8) {
    SetGpuReg(0x0, 0);
    LZ77UnCompVram(
        sCaveTransitionTiles.as_ptr().cast_mut(),
        0x600c000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sCaveTransitionTilemap.as_ptr().cast_mut(),
        0x600f800 as usize as *mut c_void,
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
    gTasks[taskId].func = Some(Task_EnterCaveTransition3);
    gTasks[taskId].data[0] = 16;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[2] = 0;
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition3(taskId: u8) {
    let mut count: u16 = gTasks[taskId].data[2] as u16;
    if count < 16 {
        gTasks[taskId].data[2] += 1;
        gTasks[taskId].data[2] += 1;
        LoadPalette(
            (&raw const sCaveTransitionPalette_Enter[15 - count as i32]).cast_mut() as *mut c_void,
            224,
            (count + 1) * 2,
        );
    } else {
        SetGpuReg(REG_OFFSET_BLDALPHA, 4112);
        SetGpuReg(REG_OFFSET_BLDCNT, 15937);
        gTasks[taskId].func = Some(Task_EnterCaveTransition4);
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition4(taskId: u8) {
    let mut count: u16 = 16 - gTasks[taskId].data[1] as u16;
    let mut blend: u16 = count + 0x1000;
    SetGpuReg(REG_OFFSET_BLDALPHA, blend);
    if count != 0 {
        gTasks[taskId].data[1] += 1;
    } else {
        LoadPalette(
            sCaveTransitionPalette_Black.as_ptr().cast_mut() as *mut c_void,
            0,
            32,
        );
        SetMainCallback2(gMain.savedCallback);
    }
}
