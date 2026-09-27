//! Translated from `src/use_pokeblock.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMonFrame_Pal sMonFrame_Gfx sMonFrame_Tilemap sGraphData_Tilemap sConditionToMonData sConditionToFlavor sNatureTextColors sBgTemplates sWindowTemplates sUsePokeblockYesNoWinTemplate sConditionNames sSpriteSheet_UpDown sSpritePalette_UpDown sUpDownCoordsOnGraph sOam_UpDown sAnim_Up sAnim_Down sAnims_UpDown sSpriteTemplate_UpDown sOam_Condition sAnim_Condition_0 sAnim_Condition_1 sAnim_Condition_2 sAnims_Condition sSpriteTemplate_Condition sSpritePalette_Condition
#[allow(unused_imports)]
use crate::data::use_pokeblock::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInfo: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sExitCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblock: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokeblockMonId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokeblockGain: i16 = 0i16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGraph_Tilemap: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGraph_Gfx: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMonFrame_TilemapPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gConditionGraphData_Pal: u8;
    static mut gConditionText_Pal: u8;
    static mut gKeyRepeatStartDelay: u8;
    static mut gMain: u8;
    static mut gNatureNamePointers: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gScanlineEffect: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gText_GetsAPokeBlockQuestion: u8;
    static mut gText_NatureSlash: u8;
    static mut gText_NothingChanged: u8;
    static mut gText_WasEnhanced: u8;
    static mut gText_WontEatAnymore: u8;
    static mut gUsePokeblockCondition_Gfx: u8;
    static mut gUsePokeblockGraph_Gfx: u8;
    static mut gUsePokeblockGraph_Pal: u8;
    static mut gUsePokeblockGraph_Tilemap: u8;
    static mut gUsePokeblockNatureWin_Pal: u8;
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
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearWindowTilemap(a0: u8);
    fn ConditionGraph_CalcPositions(a0: *mut u8, a1: *mut u8);
    fn ConditionGraph_Draw(a0: *mut u8);
    fn ConditionGraph_Init(a0: *mut u8);
    fn ConditionGraph_InitResetScanline(a0: *mut u8);
    fn ConditionGraph_InitWindow(a0: u8);
    fn ConditionGraph_ResetScanline(a0: *mut u8) -> u8;
    fn ConditionGraph_SetNewPositions(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn ConditionGraph_TryUpdate(a0: *mut u8) -> u8;
    fn ConditionGraph_Update(a0: *mut u8);
    fn ConditionMenu_UpdateMonEnter(a0: *mut u8, a1: *mut i16) -> u8;
    fn ConditionMenu_UpdateMonExit(a0: *mut u8, a1: *mut i16) -> u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateConditionSparkleSprites(a0: *mut *mut u8, a1: u8, a2: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroyConditionSparkleSprites(a0: *mut *mut u8);
    fn DestroySprite(a0: *mut u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeConditionSparkles(a0: *mut *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBoxOrPartyMonData(a0: u16, a1: u16, a2: i32, a3: *mut u8) -> i32;
    fn GetConditionMenuMonConditions(
        a0: *mut u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u16,
        a7: u8,
    );
    fn GetConditionMenuMonGfx(a0: *mut u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16, a6: u8);
    fn GetConditionMenuMonNameAndLocString(
        a0: *mut u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: u16,
        a6: u8,
    );
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonFlavorRelation(a0: *mut u8, a1: u8) -> i8;
    fn GetNature(a0: *mut u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadConditionMonPicTemplate(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn LoadConditionSelectionIcons(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn LoadConditionSparkle(a0: *mut u8, a1: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveConditionMonOffscreen(a0: *mut i16) -> u8;
    fn MoveConditionMonOnscreen(a0: *mut i16) -> u8;
    fn PlaySE(a0: u16);
    fn PreparePokeblockFeedScene();
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetConditionSparkleSprites(a0: *mut *mut u8);
    fn ResetSpriteData();
    fn RunTextPrinters();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryClearPokeblock(a0: u8) -> u32;
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonToGivePokeblock(
    pokeblock: *mut u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut callback = callback;
        ((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(32876u32));
        ((&raw mut sInfo).cast::<u8>().cast::<*mut u8>())
            .write((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720));
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(pokeblock);
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
        SetMainCallback2(Some(CB2_UsePokeblockMenu));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnAndChooseMonToGivePokeblock() {
    unsafe {
        ((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(32876u32));
        ((&raw mut sInfo).cast::<u8>().cast::<*mut u8>())
            .write((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720));
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(((&raw mut sPokeblock).cast::<u8>().cast::<*mut u8>()).read());
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(
            ((&raw mut sExitCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        ((&raw mut gPokeblockMonId).cast::<u8>().cast::<u8>()).write(GetSelectionIdFromPartyId(
            ((&raw mut gPokeblockMonId).cast::<u8>().cast::<u8>()).read(),
        ));
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(86)).write(
            ((if ((((&raw mut gPokeblockMonId).cast::<u8>().cast::<u8>()).read()) as i32)
                <= crate::c::div_i32(6i32, 2i32)
            {
                0i32
            } else {
                1i32
            }) as u8),
        );
        SetUsePokeblockCallback(Some(LoadUsePokeblockMenu));
        SetMainCallback2(Some(CB2_ReturnToUsePokeblockMenu));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToUsePokeblockMenu() {
    unsafe {
        (((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .read())
        .unwrap_unchecked()();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        if core::mem::transmute::<_, usize>(
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == (ShowUsePokeblockMenu as *const () as usize)
        {
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                .write(0u8);
            SetMainCallback2(Some(CB2_ShowUsePokeblockMenuForResults));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowUsePokeblockMenuForResults() {
    unsafe {
        ShowUsePokeblockMenuForResults();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_UsePokeblockMenu() {
    unsafe {
        (((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .read())
        .unwrap_unchecked()();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_UsePokeblockMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ConditionGraph_Draw(
            (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
        );
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn SetUsePokeblockCallback(func: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut func = func;
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(func);
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn LoadUsePokeblockMenu() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31504))
                    .write(255u8);
                ConditionGraph_Init(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                );
                let __p2 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetSpriteData();
                FreeAllSpritePalettes();
                let __p3 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetVBlankCallback(None);
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((100663296i32) as usize as *mut u8),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                98304i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                let __p4 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
                DeactivateAllTextPrinters();
                LoadUserWindowBorderGfx(0u8, 151u16, 224u8);
                let __p5 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p6 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((LoadConditionTitle()) != 0) {
                    let __p7 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(20u16);
                LoadPartyInfo();
                let __p8 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((LoadUsePokeblockMenuGfx()) != 0) {
                    let __p9 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                UpdateMonPic(0u8);
                LoadAndCreateSelectionIcons();
                let __p10 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((MoveConditionMonOnscreen(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31502)
                        .cast::<i16>(),
                )) != 0)
                {
                    let __p11 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                let __p12 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                ConditionGraph_CalcPositions(
                    (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31832))
                    .cast::<u8>())
                    .cast::<u8>(),
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31832))
                    .wrapping_add(20))
                    .cast::<u8>())
                    .cast::<u8>(),
                );
                ConditionGraph_InitResetScanline(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                );
                let __p13 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((ConditionGraph_ResetScanline(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                )) != 0)
                {
                    ConditionGraph_SetNewPositions(
                        (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31832),
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31832))
                        .wrapping_add(20))
                        .cast::<u8>())
                        .cast::<u8>(),
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31832))
                        .wrapping_add(20))
                        .cast::<u8>())
                        .cast::<u8>(),
                    );
                    let __p14 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                ConditionGraph_Update(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                );
                let __p15 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                PutWindowTilemap(0u8);
                PutWindowTilemap(1u8);
                UpdateMonInfoText(0u16, 1u8);
                let __p16 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                SetUsePokeblockCallback(Some(ShowUsePokeblockMenu));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowUsePokeblockMenu() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(3u8);
                ShowBg(2u8);
                let __p2 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ResetConditionSparkleSprites(
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31516))
                        .cast::<*mut u8>(),
                    );
                    if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        != (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32720))
                        .wrapping_add(112))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                    {
                        let mut numSparkles: u8 =
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(32688))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(32691)
                                    .cast::<i8>())
                                .read()) as i32) as isize,
                            ))
                            .read();
                        CreateConditionSparkleSprites(
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31516))
                            .cast::<*mut u8>(),
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31504))
                            .read(),
                            numSparkles,
                        );
                    }
                    SetUsePokeblockCallback(Some(UsePokeblockMenu));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UsePokeblockMenu() {
    unsafe {
        let mut loading: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    PlaySE(5u16);
                    UpdateSelection(1u8);
                    DestroyConditionSparkleSprites(
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31516))
                        .cast::<*mut u8>(),
                    );
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(1u8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        PlaySE(5u16);
                        UpdateSelection(0u8);
                        DestroyConditionSparkleSprites(
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31516))
                            .cast::<*mut u8>(),
                        );
                        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .write(1u8);
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            PlaySE(5u16);
                            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(80))
                            .write(3u8);
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                PlaySE(5u16);
                                if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(32720))
                                .wrapping_add(113))
                                .read()) as i32)
                                    == (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(32720))
                                    .wrapping_add(112))
                                    .read()) as i32)
                                        .wrapping_sub(1i32)
                                {
                                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(80))
                                    .write(3u8);
                                } else {
                                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                        .wrapping_add(80))
                                    .write(5u8);
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                loading = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(116)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read())
                .unwrap_unchecked()();
                if !((loading) != 0) {
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
                break 'l1;
            }
            if __sw1 == 4i32 {
                break 'l1;
            }
            if __sw1 == 5i32 {
                AskUsePokeblock();
                let __p2 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                'l2: {
                    let __sw3 = ((HandleAskUsePokeblockInput()) as i32);
                    if __sw3 == 1i32 || __sw3 == (-1i32) {
                        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .write(0u8);
                        break 'l2;
                    }
                    if __sw3 == 0i32 {
                        if (IsSheenMaxed()) != 0 {
                            PrintWontEatAnymore();
                            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(80))
                            .write(7u8);
                        } else {
                            SetUsePokeblockCallback(Some(FeedPokeblockToMon));
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    EraseMenuWindow();
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(0u8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FeedPokeblockToMon() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gPokeblockMonId).cast::<u8>().cast::<u8>()).write(
                    GetPartyIdFromSelectionId(
                        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32720))
                        .wrapping_add(113))
                        .read(),
                    ),
                );
                ((&raw mut sExitCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                ((&raw mut sPokeblock).cast::<u8>().cast::<*mut u8>()).write(
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read(),
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetVBlankCallback(None);
                    {
                        Free(((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    {
                        Free(((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sMonFrame_TilemapPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sMonFrame_TilemapPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_ReturnAndChooseMonToGivePokeblock));
                    PreparePokeblockFeedScene();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowUsePokeblockMenuForResults() {
    unsafe {
        let mut loading: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    != ((((&raw mut gPokeblockMonId).cast::<u8>().cast::<u8>()).read()) as i32)
                {
                    UpdateSelection(
                        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(86))
                        .read(),
                    );
                    let __p2 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                loading = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(116)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read())
                .unwrap_unchecked()();
                if !((loading) != 0) {
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if __sw1 == 3i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p3 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(3u8);
                ShowBg(2u8);
                let __p4 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetVBlankCallback(Some(VBlankCB_UsePokeblockMenu));
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p5 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ResetConditionSparkleSprites(
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31516))
                        .cast::<*mut u8>(),
                    );
                    SetUsePokeblockCallback(Some(ShowPokeblockResults));
                    SetMainCallback2(Some(CB2_UsePokeblockMenu));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPokeblockResults() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .write((&raw mut gPlayerParty).cast::<u8>());
                let __p2 = (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>();
                (__p2).write(
                    ((__p2).read()).wrapping_offset(
                        (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32696))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(32720))
                            .wrapping_add(113))
                            .read()) as i32) as isize
                                * 4,
                        ))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 100,
                    ),
                );
                DestroyConditionSparkleSprites(
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31516))
                    .cast::<*mut u8>(),
                );
                let __p3 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    let __p4 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                CalculateConditionEnhancements();
                ConditionGraph_CalcPositions(
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(92))
                        .cast::<u8>(),
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31832))
                    .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .cast::<u8>(),
                );
                ConditionGraph_SetNewPositions(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31832))
                    .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32691)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 20,
                    ))
                    .cast::<u8>(),
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31832))
                    .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .cast::<u8>(),
                );
                LoadAndCreateUpDownSprites();
                let __p5 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((ConditionGraph_TryUpdate(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                )) != 0)
                {
                    CalculateNumAdditionalSparkles(GetPartyIdFromSelectionId(
                        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32720))
                        .wrapping_add(113))
                        .read(),
                    ));
                    if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        != (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32720))
                        .wrapping_add(112))
                        .read()) as i32)
                            .wrapping_sub(1i32)
                    {
                        let mut numSparkles: u8 =
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(32688))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(32691)
                                    .cast::<i8>())
                                .read()) as i32) as isize,
                            ))
                            .read();
                        CreateConditionSparkleSprites(
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31516))
                            .cast::<*mut u8>(),
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31504))
                            .read(),
                            numSparkles,
                        );
                    }
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(82))
                        .write(0u8);
                    let __p6 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p7 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(82);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    > 16i32
                {
                    PrintFirstEnhancement();
                    let __p9 =
                        (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0)
                    && (!((TryPrintNextEnhancement()) != 0))
                {
                    TryClearPokeblock(
                        ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8),
                    );
                    SetUsePokeblockCallback(Some(CloseUsePokeblockMenu));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CloseUsePokeblockMenu() {
    unsafe {
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
                        .write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                ScanlineEffect_InitHBlankDmaTransfer();
                let __p3 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetMainCallback2(
                    ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
                FreeConditionSparkles(
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31516))
                    .cast::<*mut u8>(),
                );
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as u32) < crate::c::div_u32(7u32, 1u32)) {
                            break 'l2;
                        }
                        'l3: {
                            DestroySprite(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(31494))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                FreeSpriteTilesByTag(0u16);
                FreeSpriteTilesByTag(1u16);
                FreeSpritePaletteByTag(0u16);
                FreeSpritePaletteByTag(1u16);
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                            break 'l4;
                        }
                        'l5: {
                            DestroySprite(
                                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(31556))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31504))
                .read()) as i32)
                    != 255i32
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(31504))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                SetVBlankCallback(None);
                {
                    Free(((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                {
                    Free(((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                {
                    Free(
                        ((&raw mut sMonFrame_TilemapPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((&raw mut sMonFrame_TilemapPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                {
                    Free(((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
                }
                FreeAllWindowBuffers();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AskUsePokeblock() {
    unsafe {
        let mut stringBuffer = crate::ffi::Align4([0u8; 64]);
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((GetPartyIdFromSelectionId(
                    (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read(),
                )) as i32) as isize
                    * 100,
            ),
            2i32,
            (&raw mut stringBuffer).cast::<u8>(),
        );
        StringGet_Nickname((&raw mut stringBuffer).cast::<u8>());
        StringAppend(
            (&raw mut stringBuffer).cast::<u8>(),
            (&raw mut gText_GetsAPokeBlockQuestion).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut stringBuffer).cast::<u8>(),
        );
        FillWindowPixelBuffer(2u8, 17u8);
        DrawTextBorderOuter(2u8, 151u16, 14u8);
        AddTextPrinterParameterized(
            2u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(2u8);
        CopyWindowToVram(2u8, 3u8);
        CreateYesNoMenu(
            (&raw const sUsePokeblockYesNoWinTemplate)
                .cast::<u8>()
                .cast_mut(),
            151u16,
            14u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn HandleAskUsePokeblockInput() -> i8 {
    unsafe {
        let mut menuItem: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        'l1: {
            let __sw1 = ((menuItem) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                PlaySE(5u16);
                rbox_fill_rectangle(2u8);
                ClearWindowTilemap(2u8);
                break 'l1;
            }
        }
        return menuItem;
    }
}
pub(crate) unsafe extern "C" fn PrintFirstEnhancement() {
    unsafe {
        DrawTextBorderOuter(2u8, 151u16, 14u8);
        FillWindowPixelBuffer(2u8, 17u8);
        {
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                .write(0u8);
            'l1: loop {
                if !(((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(83))
                .read()) as i32)
                    < 5i32)
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(97))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(83))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        != 0i32
                    {
                        break 'l1;
                    }
                }
                let __p1 =
                    (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83);
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        if ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83)).read())
            as i32)
            < 5i32
        {
            BufferEnhancedText(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                    .read(),
                ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(97))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                        .read()) as i32) as isize,
                ))
                .read()) as i16),
            );
        } else {
            BufferEnhancedText(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                    .read(),
                0i16,
            );
        }
        PrintMenuWindowText((&raw mut gStringVar4).cast::<u8>());
        PutWindowTilemap(2u8);
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn TryPrintNextEnhancement() -> u8 {
    unsafe {
        FillWindowPixelBuffer(2u8, 17u8);
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            let __p1 = (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                .read()) as i32)
                < 5i32
            {
                if ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(97))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                        .read()) as i32) as isize,
                ))
                .read()) as i32)
                    != 0i32
                {
                    break 'l1;
                }
            } else {
                ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                    .write(5u8);
                return 0u8;
            }
        }
        BufferEnhancedText(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83)).read(),
            ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(97))
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(83))
                    .read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        PrintMenuWindowText((&raw mut gStringVar4).cast::<u8>());
        CopyWindowToVram(2u8, 2u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn PrintWontEatAnymore() {
    unsafe {
        FillWindowPixelBuffer(2u8, 17u8);
        DrawTextBorderOuter(2u8, 151u16, 14u8);
        AddTextPrinterParameterized(
            2u8,
            1u8,
            (&raw mut gText_WontEatAnymore).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(2u8);
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn EraseMenuWindow() {
    unsafe {
        rbox_fill_rectangle(2u8);
        ClearWindowTilemap(2u8);
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuWindowText(message: *mut u8) {
    unsafe {
        let mut message = message;
        AddTextPrinterParameterized(
            2u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferEnhancedText(dest: *mut u8, condition: u8, enhancement: i16) {
    unsafe {
        let mut dest = dest;
        let mut condition = condition;
        let mut enhancement = enhancement;
        'l1: {
            let __sw1 = ((enhancement) as i32);
            let mut __fall = false;
            if (1i32..=32767i32).contains(&__sw1) {
                __fall = true;
                enhancement = 0i16;
            }
            if __fall || ((-32768i32)..=(-1i32)).contains(&__sw1) {
                __fall = true;
                if (enhancement) != 0 {
                    let __p2 = (dest).wrapping_offset((((enhancement) as u16) as i32) as isize);
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(0i32)) as u8));
                }
                StringCopy(
                    dest,
                    ((((&raw const sConditionNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((condition) as i32) as isize))
                    .read(),
                );
                StringAppend(dest, (&raw mut gText_WasEnhanced).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 0i32 {
                __fall = true;
                StringCopy(dest, (&raw mut gText_NothingChanged).cast::<u8>());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonConditions(mon: *mut u8, data: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut data = data;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((data).wrapping_offset(((i) as i32) as isize)).write(
                        ((GetMonData2(
                            mon,
                            ((((((&raw const sConditionToMonData)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddPokeblockToConditions(pokeblock: *mut u8, mon: *mut u8) {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut mon = mon;
        let mut i: u16 = 0u16;
        let mut stat: i16 = 0i16;
        let mut data: u8 = 0u8;
        if GetMonData2(mon, 48i32) != 255u32 {
            CalculatePokeblockEffectiveness(pokeblock, mon);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        data = ((GetMonData2(
                            mon,
                            ((((((&raw const sConditionToMonData)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as u8);
                        stat = ((((data) as i32).wrapping_add(
                            ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(102))
                            .cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as i16);
                        if ((stat) as i32) < 0i32 {
                            stat = 0i16;
                        }
                        if ((stat) as i32) > 255i32 {
                            stat = 255i16;
                        }
                        data = ((stat) as u8);
                        SetMonData(
                            mon,
                            ((((((&raw const sConditionToMonData)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                            &raw mut data,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            stat = (((((GetMonData2(mon, 48i32)) as u8) as i32)
                .wrapping_add(((((pokeblock).wrapping_add(6)).read()) as i32)))
                as i16);
            if ((stat) as i32) > 255i32 {
                stat = 255i16;
            }
            data = ((stat) as u8);
            SetMonData(mon, 48i32, &raw mut data);
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateConditionEnhancements() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut mon: *mut u8 = (&raw mut gPlayerParty).cast::<u8>();
        mon = (mon).wrapping_offset(
            (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32696))
            .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(1))
            .read()) as i32) as isize
                * 100,
        );
        GetMonConditions(
            mon,
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(87))
                .cast::<u8>(),
        );
        AddPokeblockToConditions(
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
            mon,
        );
        GetMonConditions(
            mon,
            ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(92))
                .cast::<u8>(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(97))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(92))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(87))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CalculatePokeblockEffectiveness(pokeblock: *mut u8, mon: *mut u8) {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut mon = mon;
        let mut i: i8 = 0i8;
        let mut direction: i8 = 0i8;
        let mut flavor: i8 = 0i8;
        (((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(102))
            .cast::<i16>())
        .write(((((pokeblock).wrapping_add(1)).read()) as i16));
        ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(102))
            .cast::<i16>())
        .wrapping_offset(1))
        .write(((((pokeblock).wrapping_add(5)).read()) as i16));
        ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(102))
            .cast::<i16>())
        .wrapping_offset(2))
        .write(((((pokeblock).wrapping_add(4)).read()) as i16));
        ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(102))
            .cast::<i16>())
        .wrapping_offset(3))
        .write(((((pokeblock).wrapping_add(3)).read()) as i16));
        ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(102))
            .cast::<i16>())
        .wrapping_offset(4))
        .write(((((pokeblock).wrapping_add(2)).read()) as i16));
        if ((((&raw mut gPokeblockGain).cast::<u8>().cast::<i16>()).read()) as i32) > 0i32 {
            direction = 1i8;
        } else {
            if ((((&raw mut gPokeblockGain).cast::<u8>().cast::<i16>()).read()) as i32) < 0i32 {
                direction = (-1i8);
            } else {
                return;
            }
        }
        {
            i = 0i8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut amount: i16 = ((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(102))
                    .cast::<i16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    let mut boost: i8 = ((crate::c::div_i32(((amount) as i32), 10i32)) as i8);
                    if crate::c::rem_i32(((amount) as i32), 10i32) >= 5i32 {
                        boost = (boost).wrapping_add(1);
                    }
                    flavor = GetMonFlavorRelation(
                        mon,
                        ((((&raw const sConditionToFlavor).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    if ((flavor) as i32) == ((direction) as i32) {
                        let __p1 = (((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(102))
                        .cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p1).write(
                            (((((__p1).read()) as i32)
                                .wrapping_add(((boost) as i32).wrapping_mul(((flavor) as i32))))
                                as i16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsSheenMaxed() -> u8 {
    unsafe {
        if GetBoxOrPartyMonData(
            ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32696))
                .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32) as isize
                    * 4,
            ))
            .read()) as u16),
            (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32696))
            .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(1))
            .read()) as u16),
            48i32,
            core::ptr::null_mut(),
        ) == 255i32
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetPartyIdFromSelectionId(selectionId: u8) -> u8 {
    unsafe {
        let mut selectionId = selectionId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if !((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        45i32,
                    )) != 0)
                    {
                        if ((selectionId) as i32) == 0i32 {
                            return i;
                        }
                        selectionId = (selectionId).wrapping_sub(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetSelectionIdFromPartyId(partyId: u8) -> u8 {
    unsafe {
        let mut partyId = partyId;
        let mut i: u8 = 0u8;
        let mut numEggs: u8 = 0u8;
        {
            i = 0u8;
            numEggs = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((partyId) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        45i32,
                    )) != 0
                    {
                        numEggs = (numEggs).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((((partyId) as i32).wrapping_sub(((numEggs) as i32))) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetPartyIdFromSelectionId_(selectionId: u8) -> u8 {
    unsafe {
        let mut selectionId = selectionId;
        return GetPartyIdFromSelectionId(selectionId);
    }
}
pub(crate) unsafe extern "C" fn LoadAndCreateUpDownSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        LoadSpriteSheet((&raw const sSpriteSheet_UpDown).cast::<u8>().cast_mut());
        LoadSpritePalette((&raw const sSpritePalette_UpDown).cast::<u8>().cast_mut());
        ((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(84)).write(0u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(97))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        let mut spriteId: u16 = ((CreateSprite(
                            (&raw const sSpriteTemplate_UpDown).cast::<u8>().cast_mut(),
                            (((((&raw const sUpDownCoordsOnGraph).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .read(),
                            ((((((&raw const sUpDownCoordsOnGraph).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read(),
                            0u8,
                        )) as u16);
                        if ((spriteId) as i32) != 64i32 {
                            if ((((((((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(97))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 0i32
                            {
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(SpriteCB_UpDown));
                            }
                            let __p1 = (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(84);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 6i32 {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        } else {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 12i32 {
                let __p2 = (sprite).wrapping_add(38).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
            }
        }
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 60i32
        {
            DestroySprite(sprite);
            let __p5 = (((&raw mut sInfo).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(84);
            (__p5).write(((__p5).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPartyInfo() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut numMons: u16 = 0u16;
        {
            i = 0u16;
            numMons = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((CalculatePlayerPartyCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        45i32,
                    )) != 0)
                    {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32696))
                        .cast::<u8>())
                        .wrapping_offset(((numMons) as i32) as isize * 4))
                        .write(14u8);
                        (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32696))
                        .cast::<u8>())
                        .wrapping_offset(((numMons) as i32) as isize * 4))
                        .wrapping_add(1))
                        .write(((i) as u8));
                        (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32696))
                        .cast::<u8>())
                        .wrapping_offset(((numMons) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(0u16);
                        numMons = (numMons).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(113))
        .write(0u8);
        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(112))
        .write(((((numMons) as i32).wrapping_add(1i32)) as u8));
        LoadInitialMonInfo();
    }
}
pub(crate) unsafe extern "C" fn LoadInitialMonInfo() {
    unsafe {
        let mut nextSelection: i16 = 0i16;
        let mut prevSelection: i16 = 0i16;
        LoadMonInfo(
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(113))
            .read()) as i16),
            0u8,
        );
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32691)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32692)
            .cast::<i8>())
        .write(1i8);
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32693)
            .cast::<i8>())
        .write(2i8);
        nextSelection = (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32720))
        .wrapping_add(113))
        .read()) as i32)
            .wrapping_add(1i32)) as i16);
        if ((nextSelection) as i32)
            >= (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(112))
            .read()) as i32)
        {
            nextSelection = 0i16;
        }
        prevSelection = (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32720))
        .wrapping_add(113))
        .read()) as i32)
            .wrapping_sub(1i32)) as i16);
        if ((prevSelection) as i32) < 0i32 {
            prevSelection = (((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(112))
            .read()) as i32)
                .wrapping_sub(1i32)) as i16);
        }
        LoadMonInfo(nextSelection, 1u8);
        LoadMonInfo(prevSelection, 2u8);
    }
}
pub(crate) unsafe extern "C" fn LoadMonInfo(partyId: i16, loadId: u8) {
    unsafe {
        let mut partyId = partyId;
        let mut loadId = loadId;
        let mut boxId: u8 = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32696))
        .cast::<u8>())
        .wrapping_offset(((partyId) as i32) as isize * 4))
        .read();
        let mut monId: u8 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32696))
        .cast::<u8>())
        .wrapping_offset(((partyId) as i32) as isize * 4))
        .wrapping_add(1))
        .read();
        let mut numSelections: u8 = (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32720))
        .wrapping_add(112))
        .read();
        let mut excludesCancel: u8 = 0u8;
        GetConditionMenuMonNameAndLocString(
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31565))
                .cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>(),
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31637))
                .cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 64))
            .cast::<u8>(),
            ((boxId) as u16),
            ((monId) as u16),
            ((partyId) as u16),
            ((numSelections) as u16),
            excludesCancel,
        );
        GetConditionMenuMonConditions(
            (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32688))
                .cast::<u8>(),
            ((boxId) as u16),
            ((monId) as u16),
            ((partyId) as u16),
            ((loadId) as u16),
            ((numSelections) as u16),
            excludesCancel,
        );
        GetConditionMenuMonGfx(
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(772))
                .cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 8192))
            .cast::<u8>(),
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 128))
            .cast::<u16>())
            .cast::<u8>(),
            ((boxId) as u16),
            ((monId) as u16),
            ((partyId) as u16),
            ((numSelections) as u16),
            excludesCancel,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateMonPic(loadId: u8) {
    unsafe {
        let mut loadId = loadId;
        let mut spriteId: u8 = 0u8;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePal = crate::ffi::Align4([0u8; 8]);
        if ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31504))
            .read()) as i32)
            == 255i32
        {
            LoadConditionMonPicTemplate(
                (&raw mut spriteSheet).cast::<u8>(),
                (&raw mut spriteTemplate).cast::<u8>(),
                (&raw mut spritePal).cast::<u8>(),
            );
            (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u8>()).write(
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(772))
                    .cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 8192))
                .cast::<u8>(),
            );
            (((&raw mut spritePal).cast::<u8>()).cast::<*mut u16>()).write(
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 128))
                .cast::<u16>(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(31506)
                .cast::<u16>())
            .write(((LoadSpritePalette((&raw mut spritePal).cast::<u8>())) as u16));
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(31508)
                .cast::<u16>())
            .write(LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>()));
            spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), 38i16, 104i16, 0u8);
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31504))
                .write(spriteId);
            if ((spriteId) as i32) == 64i32 {
                FreeSpriteTilesByTag(100u16);
                FreeSpritePaletteByTag(100u16);
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31504))
                    .write(255u8);
            } else {
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31504))
                    .write(spriteId);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31504))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MonPic));
                let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31504))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(34i32)) as i16));
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31512)
                    .cast::<*mut u8>())
                .write(
                    (((100728832i32).wrapping_add(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31508)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_mul(32i32),
                    )) as usize as *mut u8),
                );
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31506)
                    .cast::<u16>())
                .write(
                    (((256i32).wrapping_add(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31506)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                );
            }
        } else {
            {
                let mut _src: *mut u8 = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(772))
                .cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 8192))
                .cast::<u8>();
                let mut _dest: *mut u8 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(31512)
                .cast::<*mut u8>())
                .read();
                let mut _size: u32 = ((crate::c::div_i32(4096i32, 2i32)) as u32);
                'l1: loop {
                    if !((1i32) != 0) {
                        break 'l1;
                    }
                    if _size <= 4096u32 {
                        'l2: loop {
                            'l3: {
                                'l4: loop {
                                    'l5: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
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
                        break 'l1;
                    }
                    'l6: loop {
                        'l7: {
                            'l8: loop {
                                'l9: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2147483648i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l8;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    _src = (_src).wrapping_offset(4096);
                    _dest = (_dest).wrapping_offset(4096);
                    _size = (_size).wrapping_sub(4096u32);
                }
            }
            LoadPalette(
                (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 128))
                .cast::<u16>())
                .cast::<u8>(),
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31506)
                    .cast::<u16>())
                .read(),
                32u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadAndCreateSelectionIcons() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut spriteId: u16 = 0u16;
        let mut spriteSheets = crate::ffi::Align4([0u8; 32]);
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spritePals = crate::ffi::Align4([0u8; 24]);
        let mut spriteSheet2 = crate::ffi::Align4([0u8; 8]);
        let mut spritePal2 = crate::ffi::Align4([0u8; 8]);
        LoadConditionSelectionIcons(
            (&raw mut spriteSheets).cast::<u8>(),
            (&raw mut spriteTemplate).cast::<u8>(),
            (&raw mut spritePals).cast::<u8>(),
        );
        LoadSpriteSheets((&raw mut spriteSheets).cast::<u8>());
        LoadSpritePalettes((&raw mut spritePals).cast::<u8>());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    spriteId = ((CreateSprite(
                        (&raw mut spriteTemplate).cast::<u8>(),
                        226i16,
                        (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
                        0u8,
                    )) as u16);
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31494))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(((spriteId) as u8));
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((i) as i16));
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_SelectionIconPokeball));
                    } else {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31494))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(103u16);
        {
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    spriteId = ((CreateSprite(
                        (&raw mut spriteTemplate).cast::<u8>(),
                        230i16,
                        (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
                        0u8,
                    )) as u16);
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31494))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(((spriteId) as u8));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(3),
                            6,
                            2,
                            (0u32) as i32,
                        );
                    } else {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31494))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(102u16);
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SelectionIconCancel));
        spriteId = ((CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            222i16,
            (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
            0u8,
        )) as u16);
        if ((spriteId) as i32) != 64i32 {
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31494))
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .write(((spriteId) as u8));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                6,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                6,
                2,
                (2u32) as i32,
            );
        } else {
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31494))
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .write(255u8);
        }
        LoadConditionSparkle(
            (&raw mut spriteSheet2).cast::<u8>(),
            (&raw mut spritePal2).cast::<u8>(),
        );
        LoadSpriteSheet((&raw mut spriteSheet2).cast::<u8>());
        LoadSpritePalette((&raw mut spritePal2).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn LoadUsePokeblockMenuGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(120))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32;
            if __sw1 == 0i32 {
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 8704i32, 0u8);
                SetGpuReg(0u8, 28736u16);
                SetGpuReg(80u8, 580u16);
                SetGpuReg(82u8, 1035u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>()).write(Alloc(6656u32));
                ((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>()).write(Alloc(1280u32));
                ((&raw mut sMonFrame_TilemapPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(1280u32));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LZ77UnCompVram(
                    ((&raw const sMonFrame_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((&raw mut sMonFrame_TilemapPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadBgTiles(
                    3u8,
                    (((&raw const sMonFrame_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    224u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadBgTilemap(
                    3u8,
                    ((&raw mut sMonFrame_TilemapPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    1280u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadPalette(
                    (((&raw const sMonFrame_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    208u16,
                    32u16,
                );
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31502)
                    .cast::<i16>())
                .write((-80i16));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LZ77UnCompVram(
                    ((&raw mut gUsePokeblockGraph_Gfx).cast::<u32>()).cast::<u32>(),
                    ((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>()).read(),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                LZ77UnCompVram(
                    ((&raw mut gUsePokeblockGraph_Tilemap).cast::<u32>()).cast::<u32>(),
                    ((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>()).read(),
                );
                LoadPalette(
                    (((&raw mut gUsePokeblockGraph_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    32u16,
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadBgTiles(
                    1u8,
                    ((&raw mut sGraph_Gfx).cast::<u8>().cast::<*mut u8>()).read(),
                    6656u16,
                    640u16,
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetBgTilemapBuffer(
                    1u8,
                    ((&raw mut sGraph_Tilemap).cast::<u8>().cast::<*mut u8>()).read(),
                );
                CopyToBgTilemapBufferRect(
                    1u8,
                    (((&raw mut gUsePokeblockNatureWin_Pal).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    0u8,
                    13u8,
                    12u8,
                    4u8,
                );
                CopyBgTilemapBufferToVram(1u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                LZ77UnCompVram(
                    ((&raw const sGraphData_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(29444))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                LoadBgTilemap(
                    2u8,
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(29444))
                    .cast::<u8>(),
                    1280u16,
                    0u16,
                );
                LoadPalette(
                    (((&raw mut gConditionGraphData_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    48u16,
                    32u16,
                );
                LoadPalette(
                    (((&raw mut gConditionText_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    240u16,
                    32u16,
                );
                ConditionGraph_InitWindow(2u8);
                break 'l1;
            }
            if !__matched {
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(120))
                .write(0u8);
                return 0u8;
            }
        }
        let __p2 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(120);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateMonInfoText(loadId: u16, firstPrint: u8) {
    unsafe {
        let mut loadId = loadId;
        let mut firstPrint = firstPrint;
        let mut partyIndex: u8 = 0u8;
        let mut nature: u8 = 0u8;
        let mut str: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(0u8, 0u8);
        FillWindowPixelBuffer(1u8, 0u8);
        if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(113))
        .read()) as i32)
            != (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(112))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            AddTextPrinterParameterized(
                0u8,
                1u8,
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(31637))
                .cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 64))
                .cast::<u8>(),
                0u8,
                1u8,
                0u8,
                None,
            );
            partyIndex = GetPartyIdFromSelectionId(
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(113))
                .read(),
            );
            nature = GetNature(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIndex) as i32) as isize * 100),
            );
            str = StringCopy(
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(122))
                .cast::<u8>(),
                (&raw mut gText_NatureSlash).cast::<u8>(),
            );
            str = StringCopy(
                str,
                ((((&raw mut gNatureNamePointers).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(((nature) as i32) as isize))
                .read(),
            );
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                2u8,
                1u8,
                ((&raw const sNatureTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                0i8,
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(122))
                .cast::<u8>(),
            );
        }
        if (firstPrint) != 0 {
            CopyWindowToVram(0u8, 3u8);
            CopyWindowToVram(1u8, 3u8);
        } else {
            CopyWindowToVram(0u8, 2u8);
            CopyWindowToVram(1u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateSelection(up: u8) {
    unsafe {
        let mut up = up;
        let mut newLoadId: u16 = 0u16;
        let mut startedOnMon: u32 = 0u32;
        let mut endedOnMon: u32 = 0u32;
        if (up) != 0 {
            newLoadId = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32693)
                .cast::<i8>())
            .read()) as u16);
        } else {
            newLoadId = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32692)
                .cast::<i8>())
            .read()) as u16);
        }
        ConditionGraph_SetNewPositions(
            (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832))
                .wrapping_add(20))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32691)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 20,
            ))
            .cast::<u8>(),
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832))
                .wrapping_add(20))
            .cast::<u8>())
            .wrapping_offset(((newLoadId) as i32) as isize * 20))
            .cast::<u8>(),
        );
        if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(113))
        .read()) as i32)
            == (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(112))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            startedOnMon = 0u32;
        } else {
            startedOnMon = 1u32;
        }
        if (up) != 0 {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32693)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32692)
                    .cast::<i8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32692)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32691)
                    .cast::<i8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32691)
                .cast::<i8>())
            .write(((newLoadId) as i8));
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32694)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32693)
                    .cast::<i8>())
                .read(),
            );
            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(113))
            .write(
                ((if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    == 0i32
                {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                } else {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                }) as u8),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31564)).write(
                ((if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    == 0i32
                {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                } else {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                }) as u8),
            );
        } else {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32692)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32693)
                    .cast::<i8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32693)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32691)
                    .cast::<i8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32691)
                .cast::<i8>())
            .write(((newLoadId) as i8));
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32694)
                .cast::<i8>())
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32692)
                    .cast::<i8>())
                .read(),
            );
            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(113))
            .write(
                ((if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    < (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        .wrapping_add(1i32)
                } else {
                    0i32
                }) as u8),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31564)).write(
                ((if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    < (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(113))
                    .read()) as i32)
                        .wrapping_add(1i32)
                } else {
                    0i32
                }) as u8),
            );
        }
        if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(113))
        .read()) as i32)
            == (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(112))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            endedOnMon = 0u32;
        } else {
            endedOnMon = 1u32;
        }
        DestroyConditionSparkleSprites(
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31516))
                .cast::<*mut u8>(),
        );
        if !((startedOnMon) != 0) {
            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(116)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(LoadNewSelection_CancelToMon));
        } else {
            if !((endedOnMon) != 0) {
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(116)
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(LoadNewSelection_MonToCancel));
            } else {
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(116)
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(LoadNewSelection_MonToMon));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadNewSelection_CancelToMon() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(120))
            .read()) as i32);
            if __sw1 == 0i32 {
                UpdateMonPic(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32691)
                        .cast::<i8>())
                    .read()) as u8),
                );
                let __p2 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdateMonInfoText(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32691)
                        .cast::<i8>())
                    .read()) as u16),
                    0u8,
                );
                let __p3 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((ConditionMenu_UpdateMonEnter(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31502)
                        .cast::<i16>(),
                )) != 0)
                {
                    LoadMonInfo(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31564))
                        .read()) as i16),
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32694)
                            .cast::<i8>())
                        .read()) as u8),
                    );
                    let __p4 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(120);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetConditionSparkleSprites(
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31516))
                    .cast::<*mut u8>(),
                );
                if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    != (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    let mut numSparkles: u8 =
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32688))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(32691)
                                .cast::<i8>())
                            .read()) as i32) as isize,
                        ))
                        .read();
                    CreateConditionSparkleSprites(
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31516))
                        .cast::<*mut u8>(),
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31504))
                        .read(),
                        numSparkles,
                    );
                }
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(120))
                .write(0u8);
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LoadNewSelection_MonToCancel() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(120))
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((ConditionMenu_UpdateMonExit(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31502)
                        .cast::<i16>(),
                )) != 0)
                {
                    let __p2 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(120);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdateMonInfoText(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32691)
                        .cast::<i8>())
                    .read()) as u16),
                    0u8,
                );
                let __p3 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadMonInfo(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31564))
                    .read()) as i16),
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32694)
                        .cast::<i8>())
                    .read()) as u8),
                );
                let __p4 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(120))
                .write(0u8);
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn LoadNewSelection_MonToMon() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(120))
            .read()) as i32);
            if __sw1 == 0i32 {
                ConditionGraph_TryUpdate(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                );
                if !((MoveConditionMonOffscreen(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31502)
                        .cast::<i16>(),
                )) != 0)
                {
                    UpdateMonPic(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32691)
                            .cast::<i8>())
                        .read()) as u8),
                    );
                    let __p2 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(120);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                UpdateMonInfoText(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32691)
                        .cast::<i8>())
                    .read()) as u16),
                    0u8,
                );
                let __p3 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((ConditionMenu_UpdateMonEnter(
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31832),
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31502)
                        .cast::<i16>(),
                )) != 0)
                {
                    LoadMonInfo(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31564))
                        .read()) as i16),
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32694)
                            .cast::<i8>())
                        .read()) as u8),
                    );
                    let __p4 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(120);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetConditionSparkleSprites(
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31516))
                    .cast::<*mut u8>(),
                );
                if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(113))
                .read()) as i32)
                    != (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32720))
                    .wrapping_add(112))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    let mut numSparkles: u8 =
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(32688))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(32691)
                                .cast::<i8>())
                            .read()) as i32) as isize,
                        ))
                        .read();
                    CreateConditionSparkleSprites(
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31516))
                        .cast::<*mut u8>(),
                        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(31504))
                        .read(),
                        numSparkles,
                    );
                }
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(120))
                .write(0u8);
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonPic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(31502)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(38i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionIconPokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(113))
            .read()) as i32)
        {
            StartSpriteAnim(sprite, 0u8);
        } else {
            StartSpriteAnim(sprite, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionIconCancel(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
            .wrapping_add(113))
        .read()) as i32)
            == (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                .wrapping_add(112))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                4,
                4,
                ((IndexOfSpritePaletteTag(101u16)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                4,
                4,
                ((IndexOfSpritePaletteTag(102u16)) as u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CalculateNumAdditionalSparkles(monIndex: u8) {
    unsafe {
        let mut monIndex = monIndex;
        let mut sheen: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((monIndex) as i32) as isize * 100),
            48i32,
        )) as u8);
        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32688))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32691)
                .cast::<i8>())
            .read()) as i32) as isize,
        ))
        .write(
            ((if ((sheen) as i32) != 255i32 {
                crate::c::div_u32(
                    ((sheen) as u32),
                    (crate::c::div_u32(255u32, 9u32)).wrapping_add(1u32),
                )
            } else {
                9u32
            }) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadConditionGfx() {
    unsafe {
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePalette = crate::ffi::Align4([0u8; 8]);
        (&raw mut spritePalette)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sSpritePalette_Condition)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u32>())
            .write(((&raw mut gUsePokeblockCondition_Gfx).cast::<u32>()).cast::<u32>());
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(2048u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(1u16);
        LoadCompressedSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        LoadSpritePalette((&raw mut spritePalette).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateConditionSprite() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut xDiff: i16 = 0i16;
        let mut xStart: i16 = 0i16;
        let mut yStart: i32 = 17i32;
        let mut speed: i32 = 8i32;
        let mut sprites: *mut *mut u8 =
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31556))
                .cast::<*mut u8>();
        let mut template: *mut u8 = (&raw const sSpriteTemplate_Condition)
            .cast::<u8>()
            .cast_mut();
        {
            i = 0u16;
            xDiff = 64i16;
            xStart = (-96i16);
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        template,
                        (((((i) as i32).wrapping_mul(((xDiff) as i32)))
                            .wrapping_add(((xStart) as i32))) as i16),
                        ((yStart) as i16),
                        0u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((speed) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((((i) as i32).wrapping_mul(((xDiff) as i32)) | 32i32) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((i) as i16));
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((i) as u8),
                        );
                        ((sprites).wrapping_offset(((i) as i32) as isize)).write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadConditionTitle() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32720))
            .wrapping_add(120))
            .read()) as i32);
            if __sw1 == 0i32 {
                LoadConditionGfx();
                let __p2 = ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32720))
                .wrapping_add(120);
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 1u8;
            }
            if __sw1 == 1i32 {
                CreateConditionSprite();
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(32720))
                    .wrapping_add(120))
                .write(0u8);
                return 0u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Condition(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut prevX: i16 = ((sprite).wrapping_add(32).cast::<i16>()).read();
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        if ((((prevX) as i32)
            <= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32))
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)))
            || ((((prevX) as i32)
                >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32))
                && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    <= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)))
        {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
