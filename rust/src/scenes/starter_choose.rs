//! Translated from `src/starter_choose.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gBirchBagGrass_Pal sPokeballSelection_Pal sStarterCircle_Pal gBirchBagTilemap gBirchGrassTilemap gBirchBagGrass_Gfx gPokeballSelection_Gfx sStarterCircle_Gfx sWindowTemplates sWindowTemplate_ConfirmStarter sWindowTemplate_StarterLabel sPokeballCoords sStarterLabelCoords sStarterMon sBgTemplates sTextColors sOam_Hand sOam_Pokeball sOam_StarterCircle sCursorCoords sAnim_Hand sAnim_Pokeball_Still sAnim_Pokeball_Moving sAnim_StarterCircle sAnims_Hand sAnims_Pokeball sAnims_StarterCircle sAffineAnim_StarterPokemon sAffineAnim_StarterCircle sAffineAnims_StarterPokemon sAffineAnims_StarterCircle sSpriteSheet_PokeballSelect sSpriteSheet_StarterCircle sSpritePalettes_StarterChoose sSpriteTemplate_Hand sSpriteTemplate_Pokeball sSpriteTemplate_StarterCircle
#[allow(unused_imports)]
use crate::data::starter_choose::*;

pub(crate) static mut sStarterLabelWindowId: u16 = 0u16;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_BirchInTrouble: u8;
    static mut gText_ConfirmStarterChoice: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearScheduledBgCopiesToVram();
    fn ClearWindowTilemap(a0: u8);
    fn CopyMonCategoryText(a0: i32, a1: *mut u8);
    fn CreateMonPicSprite_Affine(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeAllSpritePalettes();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStarterPokemon(chosenStarterId: u16) -> u16 {
    unsafe {
        let mut chosenStarterId = chosenStarterId;
        if ((chosenStarterId) as i32) > 3i32 {
            chosenStarterId = 0u16;
        }
        return ((((&raw const sStarterMon)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((chosenStarterId) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn VblankCB_StarterChoose() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ChooseStarter() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        SetVBlankCallback(None);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
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
                                    100663296u32,
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
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write((dmaRegs).wrapping_offset(1), 83886080u32);
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            1024i32,
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
        LZ77UnCompVram(
            ((&raw const gBirchBagGrass_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const gBirchBagTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100675584i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const gBirchGrassTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100677632i32) as usize as *mut u8),
        );
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 680u16, 208u8);
        ClearScheduledBgCopiesToVram();
        ScanlineEffect_Stop();
        ResetTasks();
        ResetSpriteData();
        ResetPaletteFade();
        FreeAllSpritePalettes();
        ResetAllPicSprites();
        LoadPalette(
            (GetOverworldTextboxPalettePtr()).cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw const gBirchBagGrass_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            64u16,
        );
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_PokeballSelect)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadCompressedSpriteSheet(
            ((&raw const sSpriteSheet_StarterCircle)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadSpritePalettes(
            ((&raw const sSpritePalettes_StarterChoose)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        EnableInterrupts(1u16);
        SetVBlankCallback(Some(VblankCB_StarterChoose));
        SetMainCallback2(Some(CB2_StarterChoose));
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 31u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(80u8, 254u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 7u16);
        SetGpuReg(0u8, 12352u16);
        ShowBg(0u8);
        ShowBg(2u8);
        ShowBg(3u8);
        taskId = CreateTask(Some(Task_StarterChoose), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Hand).cast::<u8>().cast_mut(),
            120i16,
            56i16,
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Pokeball)
                .cast::<u8>()
                .cast_mut(),
            ((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>())
                .read()) as i16),
            (((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i16),
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Pokeball)
                .cast::<u8>()
                .cast_mut(),
            (((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(2))
            .cast::<u8>())
            .read()) as i16),
            ((((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i16),
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_Pokeball)
                .cast::<u8>()
                .cast_mut(),
            (((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(4))
            .cast::<u8>())
            .read()) as i16),
            ((((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(4))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i16),
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(2i16);
        ((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).write(255u16);
    }
}
pub(crate) unsafe extern "C" fn CB2_StarterChoose() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Task_StarterChoose(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreateStarterPokemonLabel(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
        );
        DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 680u16, 13u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_BirchInTrouble).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(0u8);
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleStarterChooseInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleStarterChooseInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selection: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            let mut spriteId: u8 = 0u8;
            ClearStarterLabel();
            spriteId = CreateSprite(
                (&raw const sSpriteTemplate_StarterCircle)
                    .cast::<u8>()
                    .cast_mut(),
                (((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selection) as i32) as isize * 2))
                .cast::<u8>())
                .read()) as i16),
                ((((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selection) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i16),
                1u8,
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((spriteId) as i16));
            spriteId = CreatePokemonFrontSprite(
                GetStarterPokemon(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u16),
                ),
                (((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selection) as i32) as isize * 2))
                .cast::<u8>())
                .read(),
                ((((((&raw const sPokeballCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((selection) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read(),
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                (&raw const sAffineAnims_StarterPokemon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>(),
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_StarterPokemon));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((spriteId) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WaitForStarterSprite));
        } else {
            if (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 32i32)
                != 0)
                && (((selection) as i32) > 0i32)
            {
                let __p1 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_MoveStarterChooseCursor));
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0)
                    && (((selection) as i32) < 2i32)
                {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_MoveStarterChooseCursor));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForStarterSprite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0)
            && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                == crate::c::div_i32(240i32, 2i32)))
            && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                == 64i32)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_AskConfirmStarter));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_AskConfirmStarter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlayCry_Normal(
            GetStarterPokemon(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
            ),
            0i8,
        );
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_ConfirmStarterChoice).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        CreateYesNoMenu(
            (&raw const sWindowTemplate_ConfirmStarter)
                .cast::<u8>()
                .cast_mut(),
            680u16,
            13u8,
            0u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleConfirmStarterInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleConfirmStarterInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u16),
                );
                ResetAllPicSprites();
                SetMainCallback2(
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == (-1i32) {
                PlaySE(5u16);
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8);
                FreeOamMatrix(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(3),
                        1,
                        5,
                        false,
                    ) as u32) as u8),
                );
                FreeAndDestroyMonPicSprite(((spriteId) as u16));
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8);
                FreeOamMatrix(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(3),
                        1,
                        5,
                        false,
                    ) as u32) as u8),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_DeclineStarter));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DeclineStarter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StarterChoose));
    }
}
pub(crate) unsafe extern "C" fn CreateStarterPokemonLabel(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut categoryText = crate::ffi::Align4([0u8; 32]);
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        let mut speciesName: *mut u8 = core::ptr::null_mut();
        let mut width: i32 = 0i32;
        let mut labelLeft: u8 = 0u8;
        let mut labelRight: u8 = 0u8;
        let mut labelTop: u8 = 0u8;
        let mut labelBottom: u8 = 0u8;
        let mut species: u16 = GetStarterPokemon(((selection) as u16));
        CopyMonCategoryText(
            ((SpeciesToNationalPokedexNum(species)) as i32),
            (&raw mut categoryText).cast::<u8>(),
        );
        speciesName = (((&raw mut gSpeciesNames).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize * 11))
        .cast::<u8>();
        (&raw mut winTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sWindowTemplate_StarterLabel)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(1)).write(
            (((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((selection) as i32) as isize * 2))
            .cast::<u8>())
            .read(),
        );
        (((&raw mut winTemplate).cast::<u8>()).wrapping_add(2)).write(
            ((((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((selection) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read(),
        );
        ((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>())
            .write(AddWindow((&raw mut winTemplate).cast::<u8>()));
        FillWindowPixelBuffer(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
            0u8,
        );
        width = GetStringCenterAlignXOffset(7i32, (&raw mut categoryText).cast::<u8>(), 104i32);
        AddTextPrinterParameterized3(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
            7u8,
            ((width) as u8),
            1u8,
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut categoryText).cast::<u8>(),
        );
        width = GetStringCenterAlignXOffset(1i32, speciesName, 104i32);
        AddTextPrinterParameterized3(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
            1u8,
            ((width) as u8),
            17u8,
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            speciesName,
        );
        PutWindowTilemap(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        labelLeft = ((((((((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((selection) as i32) as isize * 2))
        .cast::<u8>())
        .read()) as i32)
            .wrapping_mul(8i32))
        .wrapping_sub(4i32)) as u8);
        labelRight = (((((((((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((selection) as i32) as isize * 2))
        .cast::<u8>())
        .read()) as i32)
            .wrapping_add(13i32))
        .wrapping_mul(8i32))
        .wrapping_add(4i32)) as u8);
        labelTop = ((((((((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((selection) as i32) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_mul(8i32)) as u8);
        labelBottom = (((((((((((&raw const sStarterLabelCoords).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((selection) as i32) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_add(4i32))
        .wrapping_mul(8i32)) as u8);
        SetGpuReg(
            64u8,
            (((((labelLeft) as i32) << 8) | ((labelRight) as i32)) as u16),
        );
        SetGpuReg(
            68u8,
            (((((labelTop) as i32) << 8) | ((labelBottom) as i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ClearStarterLabel() {
    unsafe {
        FillWindowPixelBuffer(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
            0u8,
        );
        ClearWindowTilemap(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
        );
        RemoveWindow(
            ((((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).read()) as u8),
        );
        ((&raw mut sStarterLabelWindowId).cast::<u8>().cast::<u16>()).write(255u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_MoveStarterChooseCursor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearStarterLabel();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CreateStarterLabel));
    }
}
pub(crate) unsafe extern "C" fn Task_CreateStarterLabel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreateStarterPokemonLabel(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleStarterChooseInput));
    }
}
pub(crate) unsafe extern "C" fn CreatePokemonFrontSprite(species: u16, x: u8, y: u8) -> u8 {
    unsafe {
        let mut species = species;
        let mut x = x;
        let mut y = y;
        let mut spriteId: u8 = 0u8;
        spriteId = ((CreateMonPicSprite_Affine(
            species,
            8u32,
            0u32,
            1u8,
            ((x) as i16),
            ((y) as i16),
            14u8,
            65535u16,
        )) as u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((((&raw const sCursorCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 2,
                ))
            .cast::<u8>())
            .read()) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((&raw const sCursorCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 2,
                ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            8i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8)
                as i32)
                .wrapping_add(4i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Pokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            StartSpriteAnimIfDifferent(sprite, 1u8);
        } else {
            StartSpriteAnimIfDifferent(sprite, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StarterPokemon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            > crate::c::div_i32(240i32, 2i32)
        {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        }
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            < crate::c::div_i32(240i32, 2i32)
        {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 64i32 {
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 64i32 {
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
