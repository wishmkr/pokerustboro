//! Translated from `src/fldeff_flash.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTransitionTypes sCaveTransitionPalette_White sCaveTransitionPalette_Black sCaveTransitionPalette_Enter sCaveTransitionTilemap sCaveTransitionTiles
#[allow(unused_imports)]
use crate::data::fldeff_flash::*;

unsafe extern "C" {
    static mut EventScript_UseFlash: u8;
    static mut gFieldCallback2: u8;
    static mut gFieldEffectArguments: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gPostMenuFieldCallback: u8;
    static mut gSpecialVar_Result: u8;
    static mut gTasks: u8;
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
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
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
    unsafe {
        if (ShouldDoBrailleRegisteelEffect()) != 0 {
            ((&raw mut gSpecialVar_Result).cast::<u16>())
                .write(((GetCursorSelectionMonId()) as u16));
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SetUpPuzzleEffectRegisteel));
            return 1u8;
        } else {
            if ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(21)).read()) as i32) == 1i32)
                && (!((FlagGet(2184u16)) != 0))
            {
                ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .write(Some(FieldCallback_PrepareFadeInFromMenu));
                ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCallback_Flash));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_Flash() {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((FldEff_UseFlash as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((FldEff_UseFlash as *const () as usize as u32) as i16));
    }
}
pub(crate) unsafe extern "C" fn FldEff_UseFlash() {
    unsafe {
        PlaySE(207u16);
        FlagSet(2184u16);
        ScriptContext_SetupScript((&raw mut EventScript_UseFlash).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CB2_ChangeMapMain() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBC_ChangeMapVBlank() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoChangeMap() {
    unsafe {
        SetVBlankCallback(None);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((100663296i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((117440512i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2063597568i32)
                                        | crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((83886082i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            1022i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        ResetPaletteFade();
        ResetTasks();
        ResetSpriteData();
        {
            let mut imeTemp: u16 = 0u16;
            imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p1 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 1i32) as u16));
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
        }
        SetVBlankCallback(Some(VBC_ChangeMapVBlank));
        SetMainCallback2(Some(CB2_ChangeMapMain));
        if !((TryDoMapTransition()) != 0) {
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoMapTransition() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut fromType: u8 = GetLastUsedWarpMapType();
        let mut toType: u8 = GetCurrentMapType();
        {
            i = 0u8;
            'l1: loop {
                if !((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                .read())
                    != 0)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        == ((fromType) as i32))
                        && ((((((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(1))
                        .read()) as i32)
                            == ((toType) as i32))
                    {
                        ((((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked()();
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapPairFadeToType(_fromType: u8, _toType: u8) -> u8 {
    unsafe {
        let mut _fromType = _fromType;
        let mut _toType = _toType;
        let mut i: u8 = 0u8;
        let mut fromType: u8 = _fromType;
        let mut toType: u8 = _toType;
        {
            i = 0u8;
            'l1: loop {
                if !((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                .read())
                    != 0)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        == ((fromType) as i32))
                        && ((((((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(1))
                        .read()) as i32)
                            == ((toType) as i32))
                    {
                        return (((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(2))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapPairFadeFromType(_fromType: u8, _toType: u8) -> u8 {
    unsafe {
        let mut _fromType = _fromType;
        let mut _toType = _toType;
        let mut i: u8 = 0u8;
        let mut fromType: u8 = _fromType;
        let mut toType: u8 = _toType;
        {
            i = 0u8;
            'l1: loop {
                if !((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                .read())
                    != 0)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sTransitionTypes).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        == ((fromType) as i32))
                        && ((((((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(1))
                        .read()) as i32)
                            == ((toType) as i32))
                    {
                        return (((((&raw const sTransitionTypes).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(3))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoExitCaveTransition() {
    unsafe {
        CreateTask(Some(Task_ExitCaveTransition1), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitCaveTransition2));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(0u8, 0u16);
        LZ77UnCompVram(
            ((&raw const sCaveTransitionTiles)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100712448i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sCaveTransitionTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100726784i32) as usize as *mut u8),
        );
        LoadPalette(
            (((&raw const sCaveTransitionPalette_White)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            ((((&raw const sCaveTransitionPalette_Enter)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(8))
            .cast::<u8>(),
            224u16,
            16u16,
        );
        SetGpuReg(80u8, 15937u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(8u8, 7948u16);
        SetGpuReg(0u8, 4416u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitCaveTransition3));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(16i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut count: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut blend: u16 = ((((count) as i32).wrapping_add(4096i32)) as u16);
        SetGpuReg(82u8, blend);
        if ((count) as i32) <= 16i32 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ExitCaveTransition4));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition4(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut count: u16 = 0u16;
        SetGpuReg(82u8, 4112u16);
        count = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u16);
        if ((count) as i32) < 8i32 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            LoadPalette(
                ((((&raw const sCaveTransitionPalette_Enter)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((8i32).wrapping_add(((count) as i32))) as isize))
                .cast::<u8>(),
                224u16,
                (((16u32).wrapping_sub(((count) as u32).wrapping_mul(2u32))) as u16),
            );
        } else {
            LoadPalette(
                (((&raw const sCaveTransitionPalette_White)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                0u16,
                32u16,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ExitCaveTransition5));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(8i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitCaveTransition5(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DoEnterCaveTransition() {
    unsafe {
        CreateTask(Some(Task_EnterCaveTransition1), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_EnterCaveTransition2));
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(0u8, 0u16);
        LZ77UnCompVram(
            ((&raw const sCaveTransitionTiles)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100712448i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sCaveTransitionTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100726784i32) as usize as *mut u8),
        );
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(8u8, 7948u16);
        SetGpuReg(0u8, 4416u16);
        LoadPalette(
            (((&raw const sCaveTransitionPalette_White)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sCaveTransitionPalette_Black)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_EnterCaveTransition3));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(16i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut count: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u16);
        if ((count) as i32) < 16i32 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
            LoadPalette(
                ((((&raw const sCaveTransitionPalette_Enter)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((15i32).wrapping_sub(((count) as i32))) as isize))
                .cast::<u8>(),
                224u16,
                ((((((count) as i32).wrapping_add(1i32)) as u32).wrapping_mul(2u32)) as u16),
            );
        } else {
            SetGpuReg(82u8, 4112u16);
            SetGpuReg(80u8, 15937u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_EnterCaveTransition4));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCaveTransition4(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut count: u16 = (((16i32).wrapping_sub(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32),
        )) as u16);
        let mut blend: u16 = ((((count) as i32).wrapping_add(4096i32)) as u16);
        SetGpuReg(82u8, blend);
        if (count) != 0 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            LoadPalette(
                (((&raw const sCaveTransitionPalette_Black)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .cast::<u8>(),
                0u16,
                32u16,
            );
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
