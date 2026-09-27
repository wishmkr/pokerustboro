//! Translated from `src/wireless_communication_status_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPalettes sBgTiles_Gfx sBgTiles_Tilemap sBgTemplates sWindowTemplates sHeaderTexts sActivityGroupInfo
#[allow(unused_imports)]
use crate::data::wireless_communication_status_screen::*;

pub(crate) static mut sStatusScreen: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_ListenToWireless() -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn DynamicPlaceholderTextUtil_Reset();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn Menu_LoadStdPalAt(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aSoundVSyncOn();
}

pub(crate) unsafe extern "C" fn CB2_RunWirelessCommunicationScreen() {
    unsafe {
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            RunTasks();
            RunTextPrinters();
            AnimateSprites();
            BuildOamBuffer();
            UpdatePaletteFade();
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_WirelessCommunicationScreen() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowWirelessCommunicationScreen() {
    unsafe {
        SetMainCallback2(Some(CB2_InitWirelessCommunicationScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_InitWirelessCommunicationScreen() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        ((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(108u32));
        SetVBlankCallback(None);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(1u8, Alloc(2048u32));
        SetBgTilemapBuffer(0u8, Alloc(2048u32));
        DecompressAndLoadBgGfxUsingHeap(
            1u8,
            (((&raw const sBgTiles_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw const sBgTiles_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u16,
            0u16,
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        ResetPaletteFade();
        ResetSpriteData();
        ResetTasks();
        ScanlineEffect_Stop();
        m4aSoundVSyncOn();
        SetVBlankCallback(Some(VBlankCB_WirelessCommunicationScreen));
        ((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(96))
            .write(CreateTask(Some(Task_WirelessCommunicationScreen), 0u8));
        ((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(97))
            .write(CreateTask_ListenToWireless());
        ((((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
            .cast::<u32>())
        .wrapping_offset(3))
        .write(1u32);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        LoadPalette(
            ((&raw const sPalettes).cast::<u8>().cast_mut()).cast::<u8>(),
            0u16,
            32u16,
        );
        Menu_LoadStdPalAt(240u16);
        DynamicPlaceholderTextUtil_Reset();
        FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 32u8, 15u8);
        CopyBgTilemapBufferToVram(1u8);
        SetMainCallback2(Some(CB2_RunWirelessCommunicationScreen));
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_ExitWirelessCommunicationStatusScreen() {
    unsafe {
        let mut i: i32 = 0i32;
        FreeAllWindowBuffers();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    Free(GetBgTilemapBuffer(((i) as u8)));
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read());
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
pub(crate) unsafe extern "C" fn CyclePalette(counter: *mut i16, palIdx: *mut i16) {
    unsafe {
        let mut counter = counter;
        let mut palIdx = palIdx;
        let mut idx: i32 = 0i32;
        if (({
            let __t1 = ((counter).read()).wrapping_add(1);
            (counter).write(__t1);
            __t1
        }) as i32)
            > 5i32
        {
            if (({
                let __t2 = ((palIdx).read()).wrapping_add(1);
                (palIdx).write(__t2);
                __t2
            }) as i32)
                == ((crate::c::div_u32(512u32, 32u32)) as i32).wrapping_sub(2i32)
            {
                (palIdx).write(0i16);
            }
            (counter).write(0i16);
        }
        idx = (((palIdx).read()) as i32).wrapping_add(2i32);
        LoadPalette(
            (((((&raw const sPalettes).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((idx) as isize * 32))
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            16u16,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintHeaderTexts() {
    unsafe {
        let mut i: i32 = 0i32;
        FillWindowPixelBuffer(0u8, 0u8);
        FillWindowPixelBuffer(1u8, 0u8);
        FillWindowPixelBuffer(2u8, 0u8);
        WCSS_AddTextPrinterParameterized(
            0u8,
            1u8,
            (((&raw const sHeaderTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .read(),
            ((GetStringCenterAlignXOffset(
                1i32,
                (((&raw const sHeaderTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .read(),
                192i32,
            )) as u8),
            6u8,
            3u8,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    WCSS_AddTextPrinterParameterized(
                        1u8,
                        1u8,
                        ((((&raw const sHeaderTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                        .read(),
                        0u8,
                        ((((30i32).wrapping_mul(i)).wrapping_add(8i32)) as u8),
                        1u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        WCSS_AddTextPrinterParameterized(
            1u8,
            1u8,
            ((((&raw const sHeaderTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((i).wrapping_add(1i32)) as isize))
            .read(),
            0u8,
            ((((30i32).wrapping_mul(i)).wrapping_add(8i32)) as u8),
            2u8,
        );
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 2u8);
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Task_WirelessCommunicationScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintHeaderTexts();
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                ShowBg(1u8);
                CopyBgTilemapBufferToVram(0u8);
                ShowBg(0u8);
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (UpdateCommunicationCounts(
                    (((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>(),
                    ((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .cast::<u32>(),
                    ((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<u32>(),
                    ((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(97))
                    .read(),
                )) != 0
                {
                    FillWindowPixelBuffer(2u8, 0u8);
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                ConvertIntToDecimalStringN(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    (((((((&raw mut sStatusScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32),
                                    1i32,
                                    2u8,
                                );
                                if i != 3i32 {
                                    WCSS_AddTextPrinterParameterized(
                                        2u8,
                                        1u8,
                                        (&raw mut gStringVar4).cast::<u8>(),
                                        12u8,
                                        ((((30i32).wrapping_mul(i)).wrapping_add(8i32)) as u8),
                                        1u8,
                                    );
                                } else {
                                    WCSS_AddTextPrinterParameterized(
                                        2u8,
                                        1u8,
                                        (&raw mut gStringVar4).cast::<u8>(),
                                        12u8,
                                        98u8,
                                        2u8,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    PutWindowTilemap(2u8);
                    CopyWindowToVram(2u8, 3u8);
                }
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
                    || (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0)
                {
                    PlaySE(5u16);
                    ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sStatusScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(97))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .write(255i16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                CyclePalette(
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7),
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_ExitWirelessCommunicationStatusScreen));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WCSS_AddTextPrinterParameterized(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    mode: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut mode = mode;
        let mut color = crate::ffi::Align4([0u8; 3]);
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(2u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(4u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(5u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(7u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(6u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(2u8);
                break 'l1;
            }
        }
        AddTextPrinterParameterized4(
            windowId,
            fontId,
            x,
            y,
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn CountPlayersInGroupAndGetActivity(
    player: *mut u8,
    groupCounts: *mut u32,
) -> u32 {
    unsafe {
        let mut player = player;
        let mut groupCounts = groupCounts;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut activity: u32 =
            ((crate::c::bf_read((player).wrapping_add(10), 0, 7, false) as u8) as u32);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(93u32, 3u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const sActivityGroupInfo).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 3))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 255i32
                    {
                        break 'l2;
                    }
                    if (activity
                        == (((((((&raw const sActivityGroupInfo).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 3))
                        .cast::<u8>())
                        .read()) as u32))
                        && (((crate::c::bf_read((player).wrapping_add(26), 0, 2, false) as u8)
                            as i32)
                            == 1i32)
                    {
                        if ((((((((&raw const sActivityGroupInfo).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            == 0i32
                        {
                            k = 0i32;
                            {
                                j = 0i32;
                                'l3: loop {
                                    if !(j < 4i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if ((((((player).wrapping_add(4)).cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            != 0i32
                                        {
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            k = (k).wrapping_add(1);
                            let __p1 = (groupCounts).wrapping_offset(
                                ((((((((&raw const sActivityGroupInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 3))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32) as isize,
                            );
                            (__p1).write(((__p1).read()).wrapping_add(((k) as u32)));
                        } else {
                            let __p2 = (groupCounts).wrapping_offset(
                                ((((((((&raw const sActivityGroupInfo).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 3))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32) as isize,
                            );
                            (__p2).write(
                                ((__p2).read()).wrapping_add(
                                    ((((((((&raw const sActivityGroupInfo)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 3))
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as u32),
                                ),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return activity;
    }
}
pub(crate) unsafe extern "C" fn HaveCountsChanged(
    currCounts: *mut u32,
    prevCounts: *mut u32,
) -> u32 {
    unsafe {
        let mut currCounts = currCounts;
        let mut prevCounts = prevCounts;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((currCounts).wrapping_offset((i) as isize)).read()
                        != ((prevCounts).wrapping_offset((i) as isize)).read()
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
pub(crate) unsafe extern "C" fn UpdateCommunicationCounts(
    groupCounts: *mut u32,
    prevGroupCounts: *mut u32,
    activities: *mut u32,
    taskId: u8,
) -> u32 {
    unsafe {
        let mut groupCounts = groupCounts;
        let mut prevGroupCounts = prevGroupCounts;
        let mut activities = activities;
        let mut taskId = taskId;
        let mut activitiesChanged: u32 = 0u32;
        let mut groupCountBuffer = crate::ffi::Align4([0u8; 16]);
        (&raw mut groupCountBuffer)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u32>()
            .write(0u32);
        (&raw mut groupCountBuffer)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u32>()
            .write(0u32);
        (&raw mut groupCountBuffer)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<u32>()
            .write(0u32);
        (&raw mut groupCountBuffer)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<u32>()
            .write(0u32);
        let mut players: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut activity: u32 = CountPlayersInGroupAndGetActivity(
                        ((players).read()).wrapping_offset((i) as isize * 32),
                        (&raw mut groupCountBuffer).cast::<u32>(),
                    );
                    if activity != ((activities).wrapping_offset((i) as isize)).read() {
                        ((activities).wrapping_offset((i) as isize)).write(activity);
                        activitiesChanged = 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((HaveCountsChanged((&raw mut groupCountBuffer).cast::<u32>(), prevGroupCounts)) != 0) {
            if activitiesChanged == 1u32 {
                return 1u32;
            } else {
                return 0u32;
            }
        } else {
            crate::c::memcpy(
                (groupCounts).cast::<u8>(),
                ((&raw mut groupCountBuffer).cast::<u32>()).cast::<u8>(),
                16u32,
            );
            crate::c::memcpy(
                (prevGroupCounts).cast::<u8>(),
                ((&raw mut groupCountBuffer).cast::<u32>()).cast::<u8>(),
                16u32,
            );
            ((groupCounts).wrapping_offset(3)).write(
                ((((groupCounts).read()).wrapping_add(((groupCounts).wrapping_offset(1)).read()))
                    .wrapping_add(((groupCounts).wrapping_offset(2)).read()))
                .wrapping_add(((groupCounts).wrapping_offset(3)).read()),
            );
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
