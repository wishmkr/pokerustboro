//! Translated from `src/berry_fix_program.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sText_BerryProgramUpdate sText_RubySapphire sText_Emerald sText_BerryProgramWillBeUpdatedPressA sText_EnsureGBAConnectionMatches sText_TurnOffPowerHoldingStartSelect sText_TransmittingPleaseWait sText_PleaseFollowInstructionsOnScreen sText_TransmissionFailureTryAgain sBerryFixBgTemplates sBerryFixWindowTemplates sText_Pal sBerryProgramTextColors sGameTitleTextColors sBerryProgramTexts sBerryFixGraphics
#[allow(unused_imports)]
use crate::data::berry_fix_program::*;

pub(crate) static mut sBerryFix: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gMain: u8;
    static mut gMultiBootProgram_BerryGlitchFix_End: u8;
    static mut gMultiBootProgram_BerryGlitchFix_Start: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn DeactivateAllTextPrinters();
    fn DisableInterrupts(a0: u16);
    fn DoSoftReset();
    fn EnableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn MultiBootCheckComplete(a0: *mut u8) -> i32;
    fn MultiBootInit(a0: *mut u8);
    fn MultiBootMain(a0: *mut u8) -> i32;
    fn MultiBootStartMaster(a0: *mut u8, a1: *mut u8, a2: i32, a3: u8, a4: i8);
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn m4aSoundVSyncOff();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitBerryFixProgram() {
    unsafe {
        DisableInterrupts(65535u16);
        EnableInterrupts(1u16);
        m4aSoundVSyncOff();
        SetVBlankCallback(None);
        ResetSpriteData();
        ResetTasks();
        ScanlineEffect_Stop();
        SetGpuReg(0u8, 0u16);
        ((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(80u32));
        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1)).write(6u8);
        SetMainCallback2(Some(BerryFix_Main));
    }
}
pub(crate) unsafe extern "C" fn BerryFix_Main() {
    unsafe {
        'l1: {
            let __sw1 =
                (((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                BerryFix_GpuSet();
                (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (BerryFix_TrySetScene(5i32) == 5i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (BerryFix_TrySetScene(0i32) == 0i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if BerryFix_TrySetScene(1i32) == 1i32 {
                    (((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                    .write((&raw mut gMultiBootProgram_BerryGlitchFix_Start).cast::<u8>());
                    (((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(75))
                    .write(0u8);
                    MultiBootInit(
                        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                    );
                    ((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .write(0u16);
                    (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                MultiBootMain(
                    (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4),
                );
                if ((((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .wrapping_add(24))
                .read()) as i32)
                    != 0i32)
                    || ((!(((((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(29))
                    .read()) as i32)
                        & 2i32)
                        != 0))
                        || (!(((((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .wrapping_add(30))
                        .read()) as i32)
                            & 2i32)
                            != 0)))
                {
                    ((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .write(0u16);
                } else {
                    if (({
                        let __p2 = (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<u16>();
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        > 180i32
                    {
                        MultiBootStartMaster(
                            (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4),
                            ((&raw mut gMultiBootProgram_BerryGlitchFix_Start).cast::<u8>())
                                .wrapping_offset(192),
                            ((((((&raw mut gMultiBootProgram_BerryGlitchFix_End).cast::<u8>())
                                as usize)
                                .wrapping_sub(
                                    (((&raw mut gMultiBootProgram_BerryGlitchFix_Start)
                                        .cast::<u8>())
                                    .wrapping_offset(192))
                                        as usize,
                                ) as i32
                                / 1) as u32) as i32),
                            4u8,
                            1i8,
                        );
                        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(5u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if BerryFix_TrySetScene(2i32) == 2i32 {
                    MultiBootMain(
                        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                    );
                    if (MultiBootCheckComplete(
                        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4),
                    )) != 0
                    {
                        (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(6u8);
                    } else {
                        if !(((((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .wrapping_add(30))
                        .read()) as i32)
                            & 2i32)
                            != 0)
                        {
                            (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read())
                                .write(7u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (BerryFix_TrySetScene(3i32) == 3i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    DoSoftReset();
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (BerryFix_TrySetScene(4i32) == 4i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0)
                {
                    (((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BerryFix_GpuSet() {
    unsafe {
        let mut width: i32 = 0i32;
        let mut left: i32 = 0i32;
        SetGpuReg(8u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(80u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    100663296u32,
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2063597568i32)
                                        | crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(32i32, 8i32),
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
                                    117440512u32,
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
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write((dmaRegs).wrapping_offset(1), 83886080u32);
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
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBerryFixBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        InitWindows(
            ((&raw const sBerryFixWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        'l13: loop {
            'l14: {
                'l15: loop {
                    'l16: {
                        {
                            let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                            crate::c::volatile_write(
                                dmaRegs,
                                ((((&raw const sText_Pal).cast::<u8>().cast_mut().cast::<u16>())
                                    .cast::<u16>()) as usize
                                    as u32),
                            );
                            crate::c::volatile_write((dmaRegs).wrapping_offset(1), 83886560u32);
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(2),
                                (2214592512u32
                                    | crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    )),
                            );
                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l15;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l13;
            }
        }
        SetGpuReg(0u8, 64u16);
        FillWindowPixelBuffer(2u8, 0u8);
        FillWindowPixelBuffer(3u8, 0u8);
        FillWindowPixelBuffer(0u8, 170u8);
        width = GetStringWidth(
            0u8,
            ((&raw const sText_Emerald).cast::<u8>().cast_mut()).cast::<u8>(),
            0i16,
        );
        left = crate::c::div_i32((120i32).wrapping_sub(width), 2i32);
        AddTextPrinterParameterized3(
            2u8,
            0u8,
            ((left) as u8),
            3u8,
            ((&raw const sGameTitleTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            ((&raw const sText_Emerald).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        width = GetStringWidth(
            0u8,
            ((&raw const sText_RubySapphire).cast::<u8>().cast_mut()).cast::<u8>(),
            0i16,
        );
        left = (crate::c::div_i32((120i32).wrapping_sub(width), 2i32)).wrapping_add(120i32);
        AddTextPrinterParameterized3(
            2u8,
            0u8,
            ((left) as u8),
            3u8,
            ((&raw const sGameTitleTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            ((&raw const sText_RubySapphire).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        width = GetStringWidth(
            0u8,
            ((&raw const sText_RubySapphire).cast::<u8>().cast_mut()).cast::<u8>(),
            0i16,
        );
        left = crate::c::div_i32((112i32).wrapping_sub(width), 2i32);
        AddTextPrinterParameterized3(
            3u8,
            0u8,
            ((left) as u8),
            0u8,
            ((&raw const sGameTitleTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            ((&raw const sText_RubySapphire).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        width = GetStringWidth(
            1u8,
            ((&raw const sText_BerryProgramUpdate)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i16,
        );
        left = crate::c::div_i32((208i32).wrapping_sub(width), 2i32);
        AddTextPrinterParameterized3(
            0u8,
            1u8,
            ((left) as u8),
            2u8,
            ((&raw const sBerryProgramTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            ((&raw const sText_BerryProgramUpdate)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        CopyWindowToVram(2u8, 2u8);
        CopyWindowToVram(3u8, 2u8);
        CopyWindowToVram(0u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn BerryFix_TrySetScene(scene: i32) -> i32 {
    unsafe {
        let mut scene = scene;
        if ((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == scene
        {
            return scene;
        }
        if ((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 6i32
        {
            BerryFix_SetScene(scene);
            ((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(((scene) as u8));
        } else {
            BerryFix_HideScene();
            ((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(6u8);
        }
        return ((((((&raw mut sBerryFix).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn BerryFix_SetScene(scene: i32) {
    unsafe {
        let mut scene = scene;
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        FillWindowPixelBuffer(1u8, 170u8);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            0u8,
            0u8,
            ((&raw const sBerryProgramTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            ((((&raw const sBerryProgramTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((scene) as isize))
            .read(),
        );
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 2u8);
        'l1: {
            let __sw1 = scene;
            if __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                PutWindowTilemap(2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                PutWindowTilemap(3u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                PutWindowTilemap(0u8);
                break 'l1;
            }
        }
        CopyBgTilemapBufferToVram(0u8);
        LZ77UnCompVram(
            (((((&raw const sBerryFixGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((scene) as isize * 12))
            .cast::<*mut u32>())
            .read(),
            ((100679680i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            (((((&raw const sBerryFixGraphics).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((scene) as isize * 12))
            .wrapping_add(4)
            .cast::<*mut u32>())
            .read(),
            ((100726784i32) as usize as *mut u8),
        );
        'l2: loop {
            'l3: {
                'l4: loop {
                    'l5: {
                        CpuSet(
                            ((((((&raw const sBerryFixGraphics).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((scene) as isize * 12))
                            .wrapping_add(8)
                            .cast::<*mut u16>())
                            .read())
                            .cast::<u8>(),
                            ((83886080i32) as usize as *mut u8),
                            (67108864u32
                                | (crate::c::div_u32(
                                    256u32,
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l4;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l2;
            }
        }
        ShowBg(0u8);
        ShowBg(1u8);
    }
}
pub(crate) unsafe extern "C" fn BerryFix_HideScene() {
    unsafe {
        HideBg(0u8);
        HideBg(1u8);
    }
}
