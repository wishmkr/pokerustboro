//! Translated from `src/battle_factory_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPokeballGray_Pal sPokeballSelected_Pal sInterface_Pal sPokeball_Gfx sArrow_Gfx sMenuHighlightLeft_Gfx sMenuHighlightRight_Gfx sActionBoxLeft_Gfx sActionBoxRight_Gfx sActionHighlightLeft_Gfx sActionHighlightMiddle_Gfx sActionHighlightRight_Gfx sMonPicBgAnim_Gfx sMonPicBg_Tilemap sMonPicBg_Gfx sMonPicBg_Pal sSelect_SpriteSheets sSelect_BallGfx sSelect_SpritePalettes sSelect_MenuOptionFuncs sSelect_BgTemplates sSelect_WindowTemplates sSelectText_Pal sMenuOptionTextColors sSpeciesNameTextColors sOam_Select_Pokeball sOam_Select_Arrow sOam_Select_MenuHighlight sOam_Select_MonPicBgAnim sAnim_Select_Interface sAnim_Select_MonPicBgAnim sAnim_Select_Pokeball_Still sAnim_Select_Pokeball_Moving sAnims_Select_Interface sAnims_Select_MonPicBgAnim sAnims_Select_Pokeball sAffineAnim_Select_MonPicBg_Opening sAffineAnim_Select_MonPicBg_Closing sAffineAnim_Select_MonPicBg_Open sAffineAnims_Select_MonPicBgAnim sSpriteTemplate_Select_Pokeball sSpriteTemplate_Select_Arrow sSpriteTemplate_Select_MenuHighlightLeft sSpriteTemplate_Select_MenuHighlightRight sSpriteTemplate_Select_MonPicBgAnim sSwap_SpriteSheets sSwap_BallGfx sSwap_SpritePalettes sOam_Swap_Pokeball sOam_Swap_Arrow sOam_Swap_MenuHighlight sOam_Swap_MonPicBgAnim sAnim_Swap_Interface sAnim_Swap_MonPicBgAnim sAnim_Swap_Pokeball_Still sAnim_Swap_Pokeball_Moving sAnims_Swap_Interface sAnims_Swap_MonPicBgAnim sAnims_Swap_Pokeball sAffineAnim_Swap_MonPicBg_Opening sAffineAnim_Swap_MonPicBg_Closing sAffineAnim_Swap_MonPicBg_Open sAffineAnims_Swap_MonPicBgAnim sSpriteTemplate_Swap_Pokeball sSpriteTemplate_Swap_Arrow sSpriteTemplate_Swap_MenuHighlightLeft sSpriteTemplate_Swap_MenuHighlightRight sSpriteTemplate_Swap_MonPicBgAnim sSwap_MenuOptionFuncs sSwap_BgTemplates sSwap_WindowTemplates sSwapText_Pal sSwapMenuOptionsTextColors sSwapSpeciesNameTextColors sSwap_PlayerScreenActions sSwap_EnemyScreenActions
#[allow(unused_imports)]
use crate::data::battle_factory_screen::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMenuTilesetBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMonPicBgTilesetBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMenuTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMonPicBgTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFactorySelectMons: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMenuTilesetBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMonPicBgTilesetBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMenuTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMonPicBgTilemapBuffer: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sFactorySelectScreen: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sSwap_CurrentOptionFunc: Option<unsafe extern "C" fn(u8)> = None;
pub(crate) static mut sFactorySwapScreen: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFactorySelect_CurrentOptionFunc: Option<unsafe extern "C" fn() -> u8> = None;

unsafe extern "C" {
    static mut gBattleFrontierHeldItems: u8;
    static mut gBattleFrontierMons: u8;
    static mut gEnemyParty: u8;
    static mut gFacilityTrainerMons: u8;
    static mut gFrontierFactoryMenu_Gfx: u8;
    static mut gFrontierFactoryMenu_Pal: u8;
    static mut gFrontierFactoryMenu_Tilemap: u8;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSlateportBattleTentMons: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AcceptThisPkmn: u8;
    static mut gText_Cancel3: u8;
    static mut gText_CantSelectSamePkmn: u8;
    static mut gText_Deselect: u8;
    static mut gText_No2: u8;
    static mut gText_No3: u8;
    static mut gText_Others2: u8;
    static mut gText_PkmnForSwap: u8;
    static mut gText_PkmnSwap: u8;
    static mut gText_QuitSwapping: u8;
    static mut gText_Rechoose: u8;
    static mut gText_Rent: u8;
    static mut gText_RentalPkmn2: u8;
    static mut gText_SamePkmnInPartyAlready: u8;
    static mut gText_SelectFirstPkmn: u8;
    static mut gText_SelectPkmnToAccept: u8;
    static mut gText_SelectPkmnToSwap: u8;
    static mut gText_SelectSecondPkmn: u8;
    static mut gText_SelectThirdPkmn: u8;
    static mut gText_Summary: u8;
    static mut gText_Summary2: u8;
    static mut gText_Swap: u8;
    static mut gText_TheseThreePkmnOkay: u8;
    static mut gText_Yes2: u8;
    static mut gText_Yes3: u8;
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
    fn CB2_ReturnToFieldContinueScript();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearWindowTilemap(a0: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMonCategoryText(a0: i32, a1: *mut u8);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonPicSprite_HandleDeoxys(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetFactoryMonFixedIV(a0: u8, a1: u8) -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetNumPastRentalsRank(a0: u8, a1: u8) -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheets(a0: *mut u8);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveAvoidReturn(a0: *mut u8, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
}

pub(crate) unsafe extern "C" fn SpriteCB_Pokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
            == ((IndexOfSpritePaletteTag(101u16)) as i32)
        {
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
                    let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p1).write(((__p1).read()).wrapping_sub(1));
                } else {
                    if crate::c::rem_i32(((Random()) as i32), 5i32) == 0i32 {
                        StartSpriteAnim(sprite, 0u8);
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(32i16);
                    } else {
                        StartSpriteAnim(sprite, 1u8);
                    }
                }
            } else {
                StartSpriteAnimIfDifferent(sprite, 1u8);
            }
        } else {
            StartSpriteAnimIfDifferent(sprite, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_SelectScreen() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_SelectScreen() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattleFactorySelectScreen() {
    unsafe {
        ((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        SetMainCallback2(Some(CB2_InitSelectScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_InitSelectScreen() {
    unsafe {
        let mut taskId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>()).read())
                    as usize)
                    != 0usize
                {
                    Free(((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                SetHBlankCallback(None);
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
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sSelect_BgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                InitWindows(
                    ((&raw const sSelect_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sSelectMenuTilesetBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(1088u32));
                ((&raw mut sSelectMonPicBgTilesetBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(1088u32));
                ((&raw mut sSelectMenuTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(2048u32));
                ((&raw mut sSelectMonPicBgTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(2048u32));
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(76u8, 0u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 0u16);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                ResetSpriteData();
                ResetTasks();
                FreeAllSpritePalettes();
                'l6: loop {
                    'l7: {
                        'l8: loop {
                            'l9: {
                                CpuSet(
                                    (((&raw mut gFrontierFactoryMenu_Gfx).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSelectMenuTilesetBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            1088u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
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
                'l10: loop {
                    'l11: {
                        'l12: loop {
                            'l13: {
                                CpuSet(
                                    (((&raw const sMonPicBg_Gfx)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSelectMonPicBgTilesetBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            96u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l12;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l10;
                    }
                }
                LoadBgTiles(
                    1u8,
                    ((&raw mut sSelectMenuTilesetBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    1088u16,
                    0u16,
                );
                LoadBgTiles(
                    3u8,
                    ((&raw mut sSelectMonPicBgTilesetBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    96u16,
                    0u16,
                );
                'l14: loop {
                    'l15: {
                        'l16: loop {
                            'l17: {
                                CpuSet(
                                    (((&raw mut gFrontierFactoryMenu_Tilemap).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSelectMenuTilemapBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    ((0i32
                                        | (crate::c::div_i32(
                                            2048i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l16;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l14;
                    }
                }
                LoadBgTilemap(
                    1u8,
                    ((&raw mut sSelectMenuTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    2048u16,
                    0u16,
                );
                LoadPalette(
                    (((&raw mut gFrontierFactoryMenu_Pal).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    0u16,
                    64u16,
                );
                LoadPalette(
                    (((&raw const sSelectText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    8u16,
                );
                LoadPalette(
                    (((&raw const sSelectText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    224u16,
                    10u16,
                );
                if (!(((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .is_null())
                    && ((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .read())
                        != 0)
                {
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(228))
                    .write(
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(676)
                        .cast::<u16>())
                        .read(),
                    );
                }
                LoadPalette(
                    (((&raw const sMonPicBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    32u16,
                    4u16,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetBgTilemapBuffer(
                    3u8,
                    ((&raw mut sSelectMonPicBgTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                CopyToBgTilemapBufferRect(
                    3u8,
                    ((&raw const sMonPicBg_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    11u8,
                    4u8,
                    8u8,
                    8u8,
                );
                CopyToBgTilemapBufferRect(
                    3u8,
                    ((&raw const sMonPicBg_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    2u8,
                    4u8,
                    8u8,
                    8u8,
                );
                CopyToBgTilemapBufferRect(
                    3u8,
                    ((&raw const sMonPicBg_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    20u8,
                    4u8,
                    8u8,
                    8u8,
                );
                CopyBgTilemapBufferToVram(3u8);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadSpritePalettes(
                    ((&raw const sSelect_SpritePalettes).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadSpriteSheets(
                    ((&raw const sSelect_SpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSelect_BallGfx).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                ShowBg(0u8);
                ShowBg(1u8);
                SetVBlankCallback(Some(VBlankCB_SelectScreen));
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetGpuReg(0u8, 4928u16);
                if (!(((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .is_null())
                    && ((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .read())
                        != 0)
                {
                    Select_SetWinRegs(88i16, 152i16, 32i16, 96i16);
                    ShowBg(3u8);
                    SetGpuReg(80u8, 4680u16);
                    SetGpuReg(82u8, 1035u16);
                } else {
                    HideBg(3u8);
                }
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (!(((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .is_null())
                    && ((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .read())
                        != 0)
                {
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(3))
                    .write(((&raw mut gLastViewedMonIndex).cast::<u8>()).read());
                }
                Select_InitMonsData();
                Select_InitAllSprites();
                if ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read()) as i32)
                    == 1i32
                {
                    Select_ReshowMonSprite();
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                Select_PrintSelectMonString();
                PutWindowTilemap(2u8);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                Select_PrintMonCategory();
                PutWindowTilemap(5u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                Select_PrintMonSpecies();
                PutWindowTilemap(1u8);
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                Select_PrintRentalPkmnString();
                PutWindowTilemap(0u8);
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(673))
                .write(CreateTask(Some(Select_Task_FadeSpeciesName), 0u8));
                if !((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read())
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(673))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    taskId = CreateTask(Some(Select_Task_HandleChooseMons), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(673))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(674))
                    .write(0u8);
                    taskId = CreateTask(Some(Select_Task_HandleMenu), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(13i16);
                }
                SetMainCallback2(Some(CB2_SelectScreen));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_InitMonsData() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            return;
        }
        ((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(684u32));
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .write(0u8);
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .write(1u8);
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(6))
        .write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 108))
                    .wrapping_add(4))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 2i32
        {
            CreateFrontierFactorySelectableMons(0u8);
        } else {
            CreateSlateportTentSelectableMons(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Select_InitAllSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut cursorPos: u8 = 0u8;
        let mut x: i16 = 0i16;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 108))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        ((CreateSprite(
                            (&raw const sSpriteTemplate_Select_Pokeball)
                                .cast::<u8>()
                                .cast_mut(),
                            ((((35i32).wrapping_mul(((i) as i32))).wrapping_add(32i32)) as i16),
                            64i16,
                            1u8,
                        )) as u16),
                    );
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 108))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(0i16);
                    Select_SetBallSpritePaletteNum(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        cursorPos = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        x = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(((cursorPos) as i32) as isize * 108))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .read();
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_Arrow)
                .cast::<u8>()
                .cast_mut(),
            x,
            88i16,
            0u8,
        ));
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MenuHighlightLeft)
                .cast::<u8>()
                .cast_mut(),
            176i16,
            112i16,
            0u8,
        ));
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(2))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MenuHighlightRight)
                .cast::<u8>()
                .cast_mut(),
            176i16,
            144i16,
            0u8,
        ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
    }
}
pub(crate) unsafe extern "C" fn Select_DestroyAllSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 108))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn Select_UpdateBallCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        let mut cursorPos: u8 = 0u8;
        if ((direction) as i32) > 0i32 {
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32)
                != 5i32
            {
                let __p1 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3))
                .write(0u8);
            }
        } else {
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3))
                .write(5u8);
            }
        }
        cursorPos = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(((cursorPos) as i32) as isize * 108))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Select_UpdateMenuCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        if ((direction) as i32) > 0i32 {
            if (((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as u32)
                != (crate::c::div_u32(12u32, 4u32)).wrapping_sub(1u32)
            {
                let __p1 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .write(0u8);
            }
        } else {
            if (((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read());
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .write((((crate::c::div_u32(12u32, 4u32)).wrapping_sub(1u32)) as u8));
            }
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Select_UpdateYesNoCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        if ((direction) as i32) > 0i32 {
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7))
            .read()) as i32)
                != 1i32
            {
                let __p1 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7))
                .write(0u8);
            }
        } else {
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7))
            .read()) as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(7))
                .write(1u8);
            }
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7))
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(7))
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Select_HandleMonSelectionChange() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut paletteNum: u8 = 0u8;
        let mut cursorPos: u8 = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        if ((((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(((cursorPos) as i32) as isize * 108))
        .wrapping_add(4))
        .read())
            != 0
        {
            paletteNum = IndexOfSpritePaletteTag(100u16);
            if (((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5))
            .read()) as i32)
                == 3i32)
                && ((((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(((cursorPos) as i32) as isize * 108))
                .wrapping_add(4))
                .read()) as i32)
                    == 1i32)
            {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            if (((((((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 108))
                            .wrapping_add(4))
                            .read()) as i32)
                                == 2i32
                            {
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((i) as i32) == 6i32 {
                    return;
                } else {
                    (((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 108))
                    .wrapping_add(4))
                    .write(1u8);
                }
            }
            (((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(((cursorPos) as i32) as isize * 108))
            .wrapping_add(4))
            .write(0u8);
            let __p1 = (((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            paletteNum = IndexOfSpritePaletteTag(101u16);
            (((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(((cursorPos) as i32) as isize * 108))
            .wrapping_add(4))
            .write(
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5))
                .read(),
            );
            let __p2 = (((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(((cursorPos) as i32) as isize * 108))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((paletteNum) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Select_SetBallSpritePaletteNum(id: u8) {
    unsafe {
        let mut id = id;
        let mut palNum: u8 = 0u8;
        if ((((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(((id) as i32) as isize * 108))
        .wrapping_add(4))
        .read())
            != 0
        {
            palNum = IndexOfSpritePaletteTag(101u16);
        } else {
            palNum = IndexOfSpritePaletteTag(100u16);
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 108))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            ((palNum) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Select_Task_OpenSummaryScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut currMonId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 6i32 {
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(228))
                .write(
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(228))
                    .read(),
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(7i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroyTask(
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(673))
                        .read(),
                    );
                    HideMonPic(
                        (((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(4)
                        .cast::<crate::c::Rec4<4>>()
                        .read_unaligned(),
                        (((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(672),
                    );
                    Select_DestroyAllSprites();
                    {
                        Free(
                            ((&raw mut sSelectMenuTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMenuTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSelectMonPicBgTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMonPicBgTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSelectMenuTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMenuTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSelectMonPicBgTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMonPicBgTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(8i16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(676)
                .cast::<u16>())
                .write(
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(228))
                    .read(),
                );
                DestroyTask(taskId);
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .write(1u8);
                currMonId = ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3))
                .read();
                ((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(600u32));
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            (((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .write_unaligned(
                                    ((((((&raw mut sFactorySelectScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 108))
                                    .wrapping_add(8)
                                    .cast::<crate::c::Rec4<100>>()
                                    .read_unaligned(),
                                );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ShowPokemonSummaryScreen(
                    1u8,
                    ((&raw mut sFactorySelectMons).cast::<u8>().cast::<*mut u8>()).read(),
                    currMonId,
                    5u8,
                    Some(CB2_InitSelectScreen),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_Task_Exit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    Select_CopyMonsToPlayerParty();
                    DestroyTask(
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(673))
                        .read(),
                    );
                    Select_DestroyAllSprites();
                    {
                        Free(
                            ((&raw mut sSelectMenuTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMenuTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSelectMenuTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMenuTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSelectMonPicBgTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSelectMonPicBgTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScript));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_Task_HandleYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 10i32 {
                Select_ShowChosenMons();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(4i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                Select_ShowYesNoOptions();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(5i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    if ((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(7))
                    .read()) as i32)
                        == 0i32
                    {
                        Select_HideChosenMons();
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(0i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Select_Task_Exit));
                    } else {
                        Select_ErasePopupMenu(4u8);
                        Select_DeclineChosenMons();
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(674))
                        .write(1u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Select_Task_HandleChooseMons));
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        PlaySE(5u16);
                        Select_ErasePopupMenu(4u8);
                        Select_DeclineChosenMons();
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(674))
                        .write(1u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Select_Task_HandleChooseMons));
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            PlaySE(5u16);
                            Select_UpdateYesNoCursorPosition((-1i8));
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                PlaySE(5u16);
                                Select_UpdateYesNoCursorPosition(1i8);
                            }
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_Task_HandleMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 2i32 {
                if !((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(6))
                .read())
                    != 0)
                {
                    OpenMonPic(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(4))
                        .wrapping_add(1),
                        (((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(672),
                        0u8,
                    );
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(9i16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(672))
                .read()) as i32)
                    != 1i32
                {
                    Select_ShowMenuOptions();
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    let mut retVal: u8 = 0u8;
                    PlaySE(5u16);
                    retVal = Select_RunMenuOptionFunc();
                    if ((retVal) as i32) == 1i32 {
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(674))
                        .write(1u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Select_Task_HandleChooseMons));
                    } else {
                        if ((retVal) as i32) == 2i32 {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(10i16);
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Select_Task_HandleYesNo));
                        } else {
                            if ((retVal) as i32) == 3i32 {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(11i16);
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Select_Task_HandleChooseMons));
                            } else {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(6i16);
                                ((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(Select_Task_OpenSummaryScreen));
                            }
                        }
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        PlaySE(5u16);
                        CloseMonPic(
                            (((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(660))
                            .cast::<u8>())
                            .wrapping_offset(4)
                            .cast::<crate::c::Rec4<4>>()
                            .read_unaligned(),
                            (((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(672),
                            0u8,
                        );
                        Select_ErasePopupMenu(3u8);
                        ((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(674))
                        .write(1u8);
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Select_Task_HandleChooseMons));
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            PlaySE(5u16);
                            Select_UpdateMenuCursorPosition((-1i8));
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                PlaySE(5u16);
                                Select_UpdateMenuCursorPosition(1i8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .read()) as i32)
                        == 1i32
                    {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(228))
                        .write(
                            ((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(676)
                            .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(228))
                        .write(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(244))
                            .read(),
                        );
                    }
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(6))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                Select_ShowMenuOptions();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(12i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_Task_HandleChooseMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(674))
                    .write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(674))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Select_Task_HandleMenu));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0
                    {
                        PlaySE(5u16);
                        Select_UpdateBallCursorPosition((-1i8));
                        Select_PrintMonCategory();
                        Select_PrintMonSpecies();
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0
                        {
                            PlaySE(5u16);
                            Select_UpdateBallCursorPosition(1i8);
                            Select_PrintMonCategory();
                            Select_PrintMonSpecies();
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    CloseMonPic(
                        (((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(4)
                        .cast::<crate::c::Rec4<4>>()
                        .read_unaligned(),
                        (((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(672),
                        0u8,
                    );
                    Select_PrintSelectMonString();
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(674))
                    .write(1u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateFrontierFactorySelectableMons(firstMonId: u8) {
    unsafe {
        let mut firstMonId = firstMonId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut ivs: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut friendship: u8 = 0u8;
        let mut otId: u32 = 0u32;
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut challengeNum: u8 = ((crate::c::div_i32(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1942))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        )) as u8);
        let mut rentalRank: u8 = 0u8;
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierMons).cast::<u8>());
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32
        {
            level = 100u8;
        } else {
            level = 50u8;
        }
        rentalRank = GetNumPastRentalsRank(battleMode, lvlMode);
        otId = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut monId: u16 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .read();
                    (((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                    ))
                    .cast::<u16>())
                    .write(monId);
                    if ((i) as i32) < ((rentalRank) as i32) {
                        ivs = GetFactoryMonFixedIV(
                            ((((challengeNum) as i32).wrapping_add(1i32)) as u8),
                            0u8,
                        );
                    } else {
                        ivs = GetFactoryMonFixedIV(challengeNum, 0u8);
                    }
                    CreateMonWithEVSpreadNatureOTID(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                        level,
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(12))
                        .read(),
                        ivs,
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read(),
                        otId,
                    );
                    friendship = 0u8;
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                SetMonMoveAvoidReturn(
                                    ((((((&raw mut sFactorySelectScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize
                                            * 108,
                                    ))
                                    .wrapping_add(8),
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                    j,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        32i32,
                        &raw mut friendship,
                    );
                    SetMonData(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        12i32,
                        ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSlateportTentSelectableMons(firstMonId: u8) {
    unsafe {
        let mut firstMonId = firstMonId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut ivs: u8 = 0u8;
        let mut level: u8 = 30u8;
        let mut friendship: u8 = 0u8;
        let mut otId: u32 = 0u32;
        ((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
            .write((&raw mut gSlateportBattleTentMons).cast::<u8>());
        otId = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut monId: u16 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(1612))
                    .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                    .cast::<u16>())
                    .read();
                    (((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                    ))
                    .cast::<u16>())
                    .write(monId);
                    CreateMonWithEVSpreadNatureOTID(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                        level,
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(12))
                        .read(),
                        ivs,
                        (((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read(),
                        otId,
                    );
                    friendship = 0u8;
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                SetMonMoveAvoidReturn(
                                    ((((((&raw mut sFactorySelectScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize
                                            * 108,
                                    ))
                                    .wrapping_add(8),
                                    (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                    j,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        32i32,
                        &raw mut friendship,
                    );
                    SetMonData(
                        ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((i) as i32).wrapping_add(((firstMonId) as i32))) as isize * 108,
                        ))
                        .wrapping_add(8),
                        12i32,
                        ((((&raw mut gBattleFrontierHeldItems).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_CopyMonsToPlayerParty() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(12))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 108))
                                .wrapping_add(4))
                                .read()) as i32)
                                    == ((i) as i32).wrapping_add(1i32)
                                {
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100)
                                        .cast::<crate::c::Rec4<100>>()
                                        .write_unaligned(
                                            ((((((&raw mut sFactorySelectScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(12))
                                            .cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 108))
                                            .wrapping_add(8)
                                            .cast::<crate::c::Rec4<100>>()
                                            .read_unaligned(),
                                        );
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                    .cast::<u16>())
                                    .write(
                                        (((((((&raw mut sFactorySelectScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(12))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 108))
                                        .cast::<u16>())
                                        .read(),
                                    );
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(4)
                                    .cast::<u32>())
                                    .write(GetMonData3(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 100),
                                        0i32,
                                        core::ptr::null_mut(),
                                    ));
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(9))
                                    .write(
                                        ((GetBoxMonData3(
                                            (((&raw mut gPlayerParty).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 100)),
                                            46i32,
                                            core::ptr::null_mut(),
                                        )) as u8),
                                    );
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(8))
                                    .write(
                                        ((GetBoxMonData3(
                                            (((&raw mut gPlayerParty).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 100)),
                                            40i32,
                                            core::ptr::null_mut(),
                                        )) as u8),
                                    );
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CalculatePlayerPartyCount();
    }
}
pub(crate) unsafe extern "C" fn Select_ShowMenuOptions() {
    unsafe {
        if !((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(6))
        .read())
            != 0)
        {
            (((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .write(0u8);
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(176i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(208i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        Select_PrintMenuOptions();
    }
}
pub(crate) unsafe extern "C" fn Select_ShowYesNoOptions() {
    unsafe {
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(7))
        .write(0u8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(176i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(112i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(208i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(112i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        Select_PrintYesNoOptions();
    }
}
pub(crate) unsafe extern "C" fn Select_ErasePopupMenu(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        FillWindowPixelBuffer(windowId, 0u8);
        CopyWindowToVram(windowId, 2u8);
        ClearWindowTilemap(windowId);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintRentalPkmnString() {
    unsafe {
        FillWindowPixelBuffer(0u8, 0u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_RentalPkmn2).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintMonSpecies() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut x: u8 = 0u8;
        let mut monId: u8 = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        FillWindowPixelBuffer(1u8, 0u8);
        species = ((GetMonData3(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(((monId) as i32) as isize * 108))
            .wrapping_add(8),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        StringCopy(
            (&raw mut gStringVar4).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 11))
            .cast::<u8>(),
        );
        x = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 86i32)) as u8);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            x,
            1u8,
            ((&raw const sSpeciesNameTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
        CopyWindowToVram(1u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintSelectMonString() {
    unsafe {
        let mut str: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(2u8, 0u8);
        if ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .read()) as i32)
            == 1i32
        {
            str = (&raw mut gText_SelectFirstPkmn).cast::<u8>();
        } else {
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5))
            .read()) as i32)
                == 2i32
            {
                str = (&raw mut gText_SelectSecondPkmn).cast::<u8>();
            } else {
                if ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5))
                .read()) as i32)
                    == 3i32
                {
                    str = (&raw mut gText_SelectThirdPkmn).cast::<u8>();
                } else {
                    str = (&raw mut gText_TheseThreePkmnOkay).cast::<u8>();
                }
            }
        }
        AddTextPrinterParameterized(2u8, 1u8, str, 2u8, 5u8, 0u8, None);
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintCantSelectSameMon() {
    unsafe {
        FillWindowPixelBuffer(2u8, 0u8);
        AddTextPrinterParameterized(
            2u8,
            1u8,
            (&raw mut gText_CantSelectSamePkmn).cast::<u8>(),
            2u8,
            5u8,
            0u8,
            None,
        );
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintMenuOptions() {
    unsafe {
        let mut selectedId: u8 = (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32) as isize
                * 108,
        ))
        .wrapping_add(4))
        .read();
        PutWindowTilemap(3u8);
        FillWindowPixelBuffer(3u8, 0u8);
        AddTextPrinterParameterized3(
            3u8,
            1u8,
            7u8,
            1u8,
            ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_Summary).cast::<u8>(),
        );
        if ((selectedId) as i32) != 0i32 {
            AddTextPrinterParameterized3(
                3u8,
                1u8,
                7u8,
                17u8,
                ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                0i8,
                (&raw mut gText_Deselect).cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                3u8,
                1u8,
                7u8,
                17u8,
                ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                0i8,
                (&raw mut gText_Rent).cast::<u8>(),
            );
        }
        AddTextPrinterParameterized3(
            3u8,
            1u8,
            7u8,
            33u8,
            ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_Others2).cast::<u8>(),
        );
        CopyWindowToVram(3u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Select_PrintYesNoOptions() {
    unsafe {
        PutWindowTilemap(4u8);
        FillWindowPixelBuffer(4u8, 0u8);
        AddTextPrinterParameterized3(
            4u8,
            1u8,
            7u8,
            1u8,
            ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_Yes2).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            4u8,
            1u8,
            7u8,
            17u8,
            ((&raw const sMenuOptionTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_No2).cast::<u8>(),
        );
        CopyWindowToVram(4u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Select_RunMenuOptionFunc() -> u8 {
    unsafe {
        ((&raw mut gFactorySelect_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(
            ((((&raw const sSelect_MenuOptionFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .wrapping_offset(
                (((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize,
            ))
            .read(),
        );
        return (((&raw mut gFactorySelect_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn Select_OptionRentDeselect() -> u8 {
    unsafe {
        let mut selectedId: u8 = (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32) as isize
                * 108,
        ))
        .wrapping_add(4))
        .read();
        let mut monId: u16 = (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32) as isize
                * 108,
        ))
        .cast::<u16>())
        .read();
        if (((selectedId) as i32) == 0i32) && (!((Select_AreSpeciesValid(monId)) != 0)) {
            Select_PrintCantSelectSameMon();
            Select_ErasePopupMenu(3u8);
            return 3u8;
        } else {
            CloseMonPic(
                (((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(660))
                .cast::<u8>())
                .wrapping_offset(4)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
                (((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(672),
                0u8,
            );
            Select_HandleMonSelectionChange();
            Select_PrintSelectMonString();
            Select_ErasePopupMenu(3u8);
            if ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5))
            .read()) as i32)
                > 3i32
            {
                return 2u8;
            } else {
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Select_DeclineChosenMons() -> u8 {
    unsafe {
        Select_HideChosenMons();
        Select_HandleMonSelectionChange();
        Select_PrintSelectMonString();
        Select_ErasePopupMenu(3u8);
        if ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .read()) as i32)
            > 3i32
        {
            return 2u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Select_OptionSummary() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Select_OptionOthers() -> u8 {
    unsafe {
        CloseMonPic(
            (((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4)
            .cast::<crate::c::Rec4<4>>()
            .read_unaligned(),
            (((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(672),
            0u8,
        );
        Select_ErasePopupMenu(3u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Select_PrintMonCategory() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut text = crate::ffi::Align4([0u8; 30]);
        let mut x: u8 = 0u8;
        let mut monId: u8 = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        if ((monId) as i32) < 6i32 {
            PutWindowTilemap(5u8);
            FillWindowPixelBuffer(5u8, 0u8);
            species = ((GetMonData3(
                ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(((monId) as i32) as isize * 108))
                .wrapping_add(8),
                11i32,
                core::ptr::null_mut(),
            )) as u16);
            CopyMonCategoryText(
                ((SpeciesToNationalPokedexNum(species)) as i32),
                (&raw mut text).cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 118i32)) as u8);
            AddTextPrinterParameterized(5u8, 1u8, (&raw mut text).cast::<u8>(), x, 1u8, 0u8, None);
            CopyWindowToVram(5u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Select_CreateMonSprite() {
    unsafe {
        let mut monId: u8 = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(3))
        .read();
        let mut mon: *mut u8 = ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 108))
        .wrapping_add(8);
        let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
        let mut otId: u32 = GetMonData3(mon, 1i32, core::ptr::null_mut());
        ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(4))
        .write(
            ((CreateMonPicSprite_HandleDeoxys(
                species,
                otId,
                personality,
                1u8,
                88i16,
                32i16,
                15u8,
                65535u16,
            )) as u8),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Select_SetMonPicAnimating(animating: u8) {
    unsafe {
        let mut animating = animating;
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .write(animating);
    }
}
pub(crate) unsafe extern "C" fn Select_ReshowMonSprite() {
    unsafe {
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(4))
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            64i16,
            1u8,
        ));
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(660))
                .cast::<u8>())
                .wrapping_offset(4))
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ),
            2u8,
        );
        mon = ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(12))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(3))
            .read()) as i32) as isize
                * 108,
        ))
        .wrapping_add(8);
        species = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        personality = GetMonData3(mon, 0i32, core::ptr::null_mut());
        otId = GetMonData3(mon, 1i32, core::ptr::null_mut());
        ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(4))
        .write(
            ((CreateMonPicSprite_HandleDeoxys(
                species,
                otId,
                personality,
                1u8,
                88i16,
                32i16,
                15u8,
                65535u16,
            )) as u8),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(660))
                .cast::<u8>())
                .wrapping_offset(4))
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Select_CreateChosenMonsSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(12))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 108))
                                .wrapping_add(4))
                                .read()) as i32)
                                    == ((i) as i32).wrapping_add(1i32)
                                {
                                    let mut mon: *mut u8 =
                                        ((((((&raw mut sFactorySelectScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(12))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 108))
                                        .wrapping_add(8);
                                    let mut species: u16 =
                                        ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
                                    let mut personality: u32 =
                                        GetMonData3(mon, 0i32, core::ptr::null_mut());
                                    let mut otId: u32 =
                                        GetMonData3(mon, 1i32, core::ptr::null_mut());
                                    ((((((&raw mut sFactorySelectScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(660))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .write(
                                        ((CreateMonPicSprite_HandleDeoxys(
                                            species,
                                            otId,
                                            personality,
                                            1u8,
                                            (((((i) as i32).wrapping_mul(72i32))
                                                .wrapping_add(16i32))
                                                as i16),
                                            32i16,
                                            ((((i) as i32).wrapping_add(13i32)) as u8),
                                            65535u16,
                                        )) as u8),
                                    );
                                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sFactorySelectScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(660))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(40)
                                    .cast::<i8>())
                                    .write(0i8);
                                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sFactorySelectScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(660))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(41)
                                    .cast::<i8>())
                                    .write(0i8);
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OpenChosenMonPics(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut taskId: u8 = 0u8;
        if (((crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0)
            && ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                5,
                1,
                false,
            ) as u16)
                != 0))
            && ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                5,
                1,
                false,
            ) as u16)
                != 0)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            taskId = CreateTask(Some(Select_Task_OpenChosenMonPics), 1u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CloseChosenMonPics(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0)
            && ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                5,
                1,
                false,
            ) as u16)
                != 0))
            && ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                5,
                1,
                false,
            ) as u16)
                != 0)
        {
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            FreeOamMatrix(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(3),
                    1,
                    5,
                    false,
                ) as u32) as u8),
            );
            FreeOamMatrix(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(3),
                    1,
                    5,
                    false,
                ) as u32) as u8),
            );
            ((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(672))
            .write(0u8);
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Select_Task_OpenChosenMonPics(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(224i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(64i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(65i16);
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 55u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ShowBg(3u8);
                SetGpuReg(80u8, 4680u16);
                SetGpuReg(82u8, 1035u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as i16));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    <= 32i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                        >= 96i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(32i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(96i16);
                    ClearGpuRegBits(0u8, 8192u16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != 32i32
                {
                    return;
                }
                break 'l1;
            }
            if !__matched {
                DestroyTask(taskId);
                Select_CreateChosenMonsSprites();
                return;
            }
        }
        let __p4 = ((task).wrapping_add(8)).cast::<i16>();
        (__p4).write(((__p4).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Select_Task_CloseChosenMonPics(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(224i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(32i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(96i16);
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 55u16);
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    >= 64i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                        <= 65i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(64i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(65i16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 64i32
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                HideBg(3u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(4))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(4))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_CloseChosenMonPics));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(660))
                    .cast::<u8>())
                    .wrapping_offset(8))
                    .wrapping_add(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(4))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(8))
                        .wrapping_add(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                ClearGpuRegBits(0u8, 8192u16);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Select_ShowChosenMons() {
    unsafe {
        (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(4))
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            64i16,
            1u8,
        ));
        ((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            44i16,
            64i16,
            1u8,
        ));
        (((((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(660))
        .cast::<u8>())
        .wrapping_offset(8))
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Select_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            196i16,
            64i16,
            1u8,
        ));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_OpenChosenMonPics));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(8))
            .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Select_HideChosenMons() {
    unsafe {
        let mut taskId: u8 = 0u8;
        FreeAndDestroyMonPicSprite(
            (((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .read()) as u16),
        );
        FreeAndDestroyMonPicSprite(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as u16),
        );
        FreeAndDestroyMonPicSprite(
            ((((((((&raw mut sFactorySelectScreen)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(660))
            .cast::<u8>())
            .wrapping_offset(8))
            .read()) as u16),
        );
        taskId = CreateTask(Some(Select_Task_CloseChosenMonPics), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
        ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(672))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Select_SetWinRegs(
    mWin0H: i16,
    nWin0H: i16,
    mWin0V: i16,
    nWin0V: i16,
) {
    unsafe {
        let mut mWin0H = mWin0H;
        let mut nWin0H = nWin0H;
        let mut mWin0V = mWin0V;
        let mut nWin0V = nWin0V;
        SetGpuRegBits(0u8, 8192u16);
        SetGpuReg(
            64u8,
            (((((mWin0H) as i32) << 8) | ((nWin0H) as i32)) as u16),
        );
        SetGpuReg(
            68u8,
            (((((mWin0V) as i32) << 8) | ((nWin0V) as i32)) as u16),
        );
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 55u16);
    }
}
pub(crate) unsafe extern "C" fn Select_AreSpeciesValid(monId: u16) -> u32 {
    unsafe {
        let mut monId = monId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut species: u32 = (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>()).read())
            .wrapping_offset(((monId) as i32) as isize * 16))
        .cast::<u16>())
        .read()) as u32);
        let mut selectState: u8 = ((((&raw mut sFactorySelectScreen)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_add(5))
        .read();
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < ((selectState) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(12))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 108))
                                .wrapping_add(4))
                                .read()) as i32)
                                    == ((i) as i32)
                                {
                                    if (((((((&raw mut gFacilityTrainerMons).cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(
                                        (((((((((&raw mut sFactorySelectScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(12))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 108))
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize
                                            * 16,
                                    ))
                                    .cast::<u16>())
                                    .read()) as u32)
                                        == species
                                    {
                                        return 0u32;
                                    }
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Select_Task_FadeSpeciesName(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(679))
                .write(0u8);
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(680))
                .write(0u8);
                ((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(678))
                .write(1u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(674))
                .read())
                    != 0
                {
                    if (((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(681))
                    .read())
                        != 0
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(2i16);
                    } else {
                        let __p2 = (((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(679);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        if ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(679))
                        .read()) as i32)
                            > 6i32
                        {
                            ((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(679))
                            .write(0u8);
                            if !((((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(678))
                            .read())
                                != 0)
                            {
                                let __p3 = (((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(680);
                                (__p3).write(((__p3).read()).wrapping_sub(1));
                            } else {
                                let __p4 = (((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(680);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            }
                        }
                        BlendPalettes(
                            16384u32,
                            ((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(680))
                            .read(),
                            0u16,
                        );
                        if ((((((&raw mut sFactorySelectScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(680))
                        .read()) as i32)
                            > 5i32
                        {
                            ((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(678))
                            .write(0u8);
                        } else {
                            if ((((((&raw mut sFactorySelectScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(680))
                            .read()) as i32)
                                == 0i32
                            {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(2i16);
                                ((((&raw mut sFactorySelectScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(678))
                                .write(1u8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sFactorySelectScreen)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(681))
                .read()) as i32)
                    > 14i32
                {
                    ((((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(681))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    let __p5 = (((&raw mut sFactorySelectScreen)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(681);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_CB2() {
    unsafe {
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
        RunTasks();
    }
}
pub(crate) unsafe extern "C" fn Swap_VblankCb() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CopySwappedMonData() {
    unsafe {
        let mut friendship: u8 = 0u8;
        ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .read()) as i32) as isize
                    * 100,
            )
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(19))
                        .read()) as i32) as isize
                            * 100,
                    )
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        friendship = 0u8;
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .read()) as i32) as isize
                    * 100,
            ),
            32i32,
            &raw mut friendship,
        );
        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18))
            .read()) as i32) as isize
                * 12,
        ))
        .cast::<u16>())
        .write(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2084))
            .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(19))
                .read()) as i32)
                    .wrapping_add(3i32)) as isize
                    * 12,
            ))
            .cast::<u16>())
            .read(),
        );
        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(8))
        .write(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2084))
            .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(19))
                .read()) as i32)
                    .wrapping_add(3i32)) as isize
                    * 12,
            ))
            .wrapping_add(8))
            .read(),
        );
        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(4)
        .cast::<u32>())
        .write(GetMonData3(
            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(19))
                .read()) as i32) as isize
                    * 100,
            ),
            0i32,
            core::ptr::null_mut(),
        ));
        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(9))
        .write(
            ((GetBoxMonData3(
                (((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .read()) as i32) as isize
                        * 100,
                )),
                46i32,
                core::ptr::null_mut(),
            )) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_OpenSummaryScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 6i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(7i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroyTask(
                        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read(),
                    );
                    HideMonPic(
                        (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(44)
                            .cast::<crate::c::Rec4<4>>()
                            .read_unaligned(),
                        (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(48),
                    );
                    Swap_DestroyAllSprites();
                    {
                        Free(
                            ((&raw mut sSwapMenuTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMenuTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMonPicBgTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMonPicBgTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMenuTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMenuTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMonPicBgTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMonPicBgTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(8i16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                DestroyTask(taskId);
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .write(1u8);
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<u16>())
                .write(
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(244))
                    .read(),
                );
                ShowPokemonSummaryScreen(
                    0u8,
                    (&raw mut gPlayerParty).cast::<u8>(),
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .read(),
                    2u8,
                    Some(CB2_InitSwapScreen),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_Exit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32))
                .read()) as i32)
                    == 1i32
                {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32))
                .read()) as i32)
                    == 1i32
                {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(19))
                    .write(
                        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read(),
                    );
                    CopySwappedMonData();
                }
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((UpdatePaletteFade()) != 0) {
                    DestroyTask(
                        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read(),
                    );
                    Swap_DestroyAllSprites();
                    {
                        Free(
                            ((&raw mut sSwapMenuTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMenuTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMonPicBgTilesetBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMonPicBgTilesetBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMenuTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMenuTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(
                            ((&raw mut sSwapMonPicBgTilemapBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                        );
                        ((&raw mut sSwapMonPicBgTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                    }
                    {
                        Free(((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScript));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut loPtr: u16 = 0u16;
        let mut hiPtr: u16 = 0u16;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 4i32 {
                Swap_ShowYesNoOptions();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(5i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(22))
                    .read()) as i32)
                        == 0i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(1i16);
                        hiPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as u16);
                        loPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .read()) as u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                            (((((hiPtr) as i32) << 16) | ((loPtr) as i32)) as usize as *mut u8),
                        ));
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        Swap_ErasePopupMenu(4u8);
                        hiPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as u16);
                        loPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .read()) as u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                            (((((hiPtr) as i32) << 16) | ((loPtr) as i32)) as usize as *mut u8),
                        ));
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        PlaySE(5u16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                        Swap_ErasePopupMenu(4u8);
                        hiPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as u16);
                        loPtr = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .read()) as u16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                            (((((hiPtr) as i32) << 16) | ((loPtr) as i32)) as usize as *mut u8),
                        ));
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            PlaySE(5u16);
                            Swap_UpdateYesNoCursorPosition((-1i8));
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 128i32)
                                != 0
                            {
                                PlaySE(5u16);
                                Swap_UpdateYesNoCursorPosition(1i8);
                            }
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_HandleQuitSwappingResponse(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 1i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_Exit));
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((Swap_Task_HandleChooseMons as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((Swap_Task_HandleChooseMons as *const () as usize as u32) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_ScreenInfoTransitionIn));
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_AskQuitSwapping(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            Swap_PrintOnInfoWindow((&raw mut gText_QuitSwapping).cast::<u8>());
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32))
            .write(0u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(4i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((Swap_HandleQuitSwappingResponse as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((Swap_HandleQuitSwappingResponse as *const () as usize as u32) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_HandleYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_HandleAcceptMonResponse(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CloseMonPic(
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48),
            1u8,
        );
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 1i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_Exit));
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((Swap_Task_HandleChooseMons as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((Swap_Task_HandleChooseMons as *const () as usize as u32) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_ScreenInfoTransitionIn));
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_AskAcceptMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            OpenMonPic(
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(44))
                .wrapping_add(1),
                (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48),
                1u8,
            );
            Swap_PrintOnInfoWindow((&raw mut gText_AcceptThisPkmn).cast::<u8>());
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32))
            .write(1u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(4i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((Swap_HandleAcceptMonResponse as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((Swap_HandleAcceptMonResponse as *const () as usize as u32) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Swap_Task_HandleYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 2i32 {
                if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read())
                    != 0)
                {
                    OpenMonPic(
                        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(44))
                        .wrapping_add(1),
                        (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(48),
                        1u8,
                    );
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(9i16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48))
                .read()) as i32)
                    != 1i32
                {
                    Swap_ShowMenuOptions();
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48))
                .read()) as i32)
                    != 1i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        PlaySE(5u16);
                        Swap_RunMenuOptionFunc(taskId);
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            PlaySE(5u16);
                            CloseMonPic(
                                (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(44)
                                .cast::<crate::c::Rec4<4>>()
                                .read_unaligned(),
                                (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(48),
                                1u8,
                            );
                            Swap_ErasePopupMenu(3u8);
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(0i16);
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(
                                (((Swap_Task_HandleChooseMons as *const () as usize as u32) >> 16)
                                    as i16),
                            );
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(7))
                            .write(
                                ((Swap_Task_HandleChooseMons as *const () as usize as u32) as i16),
                            );
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(5))
                            .write(1i16);
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .write(Some(Swap_Task_ScreenInfoTransitionIn));
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 64i32)
                                != 0
                            {
                                Swap_UpdateMenuCursorPosition((-1i8));
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 128i32)
                                    != 0
                                {
                                    Swap_UpdateMenuCursorPosition(1i8);
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleChooseMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(34))
                    .write(1u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(34))
                    .write(0u8);
                    Swap_PrintMonSpeciesAtFade();
                    Swap_EraseSpeciesWindow();
                    Swap_RunActionFunc(taskId);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        PlaySE(5u16);
                        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(34))
                        .write(0u8);
                        Swap_PrintMonSpeciesAtFade();
                        Swap_EraseSpeciesWindow();
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .write(
                            (((Swap_AskQuitSwapping as *const () as usize as u32) >> 16) as i16),
                        );
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7))
                        .write(((Swap_AskQuitSwapping as *const () as usize as u32) as i16));
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Swap_Task_ScreenInfoTransitionOut));
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0
                        {
                            Swap_UpdateBallCursorPosition((-1i8));
                            Swap_PrintMonCategory();
                            Swap_PrintMonSpecies();
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0
                            {
                                Swap_UpdateBallCursorPosition(1i8);
                                Swap_PrintMonCategory();
                                Swap_PrintMonSpecies();
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(48)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 128i32)
                                    != 0
                                {
                                    Swap_UpdateActionCursorPosition(1i8);
                                    Swap_PrintMonCategory();
                                    Swap_PrintMonSpecies();
                                } else {
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(48)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 64i32)
                                        != 0
                                    {
                                        Swap_UpdateActionCursorPosition((-1i8));
                                        Swap_PrintMonCategory();
                                        Swap_PrintMonSpecies();
                                    }
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_FadeSpeciesName(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(39))
                .write(0u8);
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40))
                .write(0u8);
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(38))
                .write(1u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(34))
                .read())
                    != 0
                {
                    if (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(41))
                    .read())
                        != 0
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(2i16);
                    } else {
                        let __p2 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(39);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(39))
                        .read()) as i32)
                            > 6i32
                        {
                            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(39))
                            .write(0u8);
                            if !((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(38))
                            .read())
                                != 0)
                            {
                                let __p3 = (((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(40);
                                (__p3).write(((__p3).read()).wrapping_sub(1));
                            } else {
                                let __p4 = (((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(40);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            }
                        }
                        BlendPalettes(
                            16384u32,
                            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .read(),
                            0u16,
                        );
                        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(40))
                        .read()) as i32)
                            > 5i32
                        {
                            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(38))
                            .write(0u8);
                        } else {
                            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .read()) as i32)
                                == 0i32
                            {
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(2i16);
                                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(38))
                                .write(1u8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(41))
                .read()) as i32)
                    > 14i32
                {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(41))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    let __p5 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(41);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_FadeOutSpeciesName(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(39))
                .write(0u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(0i16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadPalette(
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(240))
                    .cast::<u8>(),
                    224u16,
                    10u16,
                );
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40))
                .read()) as i32)
                    > 15i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(1i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                let __p5 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(39);
                (__p5).write(((__p5).read()).wrapping_add(1));
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(39))
                .read()) as i32)
                    > 3i32
                {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(39))
                    .write(0u8);
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(244))
                    .write(
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(228))
                        .read(),
                    );
                    let __p6 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(40);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                BlendPalettes(
                    16384u32,
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .read(),
                    0u16,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_SlideCycleBalls(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i8 = 0i8;
        let mut lastX: u8 = 0u8;
        let mut finished: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                lastX = 0u8;
                {
                    i = 2i8;
                    'l2: loop {
                        if !(((i) as i32) >= 0i32) {
                            break 'l2;
                        }
                        'l3: {
                            if ((i) as i32) != 2i32 {
                                let mut posX: u8 = ((((lastX) as i32).wrapping_sub(
                                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(5))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(32)
                                    .cast::<i16>())
                                    .read()) as i32),
                                )) as u8);
                                if (((posX) as i32) == 16i32)
                                    || (((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(
                                        ((((i) as i32).wrapping_add(1i32)).wrapping_add(1i32))
                                            as isize,
                                    ))
                                    .read()) as i32)
                                        == 1i32)
                                {
                                    lastX = ((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                    .wrapping_add(32)
                                    .cast::<i16>())
                                    .read()) as u8);
                                    let __p2 = (((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(
                                            ((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                    .wrapping_add(32)
                                    .cast::<i16>();
                                    (__p2).write(
                                        (((((__p2).read()) as i32).wrapping_add(10i32)) as i16),
                                    );
                                } else {
                                    if ((posX) as i32) > 16i32 {
                                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(32)
                                        .cast::<i16>())
                                        .write(
                                            ((((((((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((((&raw mut sFactorySwapScreen)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(5))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        (((i) as i32).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(32)
                                            .cast::<i16>())
                                            .read())
                                                as i32)
                                                .wrapping_sub(48i32))
                                                as i16),
                                        );
                                    }
                                }
                            } else {
                                lastX = ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .read()) as u8);
                                let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>();
                                (__p3)
                                    .write((((((__p3).read()) as i32).wrapping_add(10i32)) as i16));
                            }
                            if ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                            .read()) as i32)
                                == 1i32
                            {
                                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .read()) as i32)
                                    > (((i) as i32).wrapping_mul(48i32)).wrapping_add(72i32)
                                {
                                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(5))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(32)
                                    .cast::<i16>())
                                    .write(
                                        (((((i) as i32).wrapping_mul(48i32)).wrapping_add(72i32))
                                            as i16),
                                    );
                                    finished = 1u8;
                                } else {
                                    if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(5))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(32)
                                    .cast::<i16>())
                                    .read()) as i32)
                                        == (((i) as i32).wrapping_mul(48i32)).wrapping_add(72i32)
                                    {
                                        finished = 1u8;
                                    } else {
                                        finished = 0u8;
                                    }
                                }
                            } else {
                                finished = 0u8;
                            }
                            if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(5))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_sub(16i32)
                                > 240i32
                            {
                                lastX = ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .read()) as u8);
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write((-16i16));
                                if ((((((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20))
                                .read()) as i32)
                                    == 1i32
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(5),
                                        4,
                                        4,
                                        ((IndexOfSpritePaletteTag(101u16)) as u16) as i32,
                                    );
                                } else {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(5),
                                        4,
                                        4,
                                        ((IndexOfSpritePaletteTag(100u16)) as u16) as i32,
                                    );
                                }
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .write(1i16);
                            }
                        }
                        i = (i).wrapping_sub(1);
                    }
                }
                if ((finished) as i32) == 1i32 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_SlideButtonOnOffScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut posX: i32 = 0i32;
        let mut deltaX: i8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i8);
        let mut sliding: u8 = 0u8;
        let mut currPosX: i16 = 0i16;
        let mut prevTaskId: u8 = 0u8;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 1i32
        {
            deltaX = ((((deltaX) as i32).wrapping_mul((-1i32))) as i8);
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                currPosX = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .cast::<u8>())
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read();
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0)
                {
                    if ((currPosX) as i32).wrapping_add(((deltaX) as i32)) < 240i32 {
                        sliding = 1u8;
                    } else {
                        sliding = 0u8;
                        posX = 240i32;
                    }
                } else {
                    if ((currPosX) as i32).wrapping_add(((deltaX) as i32)) > 160i32 {
                        sliding = 1u8;
                    } else {
                        sliding = 0u8;
                        posX = 160i32;
                    }
                }
                if ((sliding) as i32) == 1i32 {
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                                break 'l2;
                            }
                            'l3: {
                                {
                                    j = 0u8;
                                    'l4: loop {
                                        if !(((j) as u32) < crate::c::div_u32(6u32, 3u32)) {
                                            break 'l4;
                                        }
                                        'l5: {
                                            let __p2 = (((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((((((&raw mut sFactorySwapScreen)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(8))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((j) as i32) as isize * 3))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(32)
                                            .cast::<i16>();
                                            (__p2).write(
                                                (((((__p2).read()) as i32)
                                                    .wrapping_add(((deltaX) as i32)))
                                                    as i16),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    {
                        j = 0u8;
                        'l6: loop {
                            if !(((j) as u32) < crate::c::div_u32(6u32, 3u32)) {
                                break 'l6;
                            }
                            'l7: {
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 3))
                                    .cast::<u8>())
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write(((posX) as i16));
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 3))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write((((posX).wrapping_add(16i32)) as i16));
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 3))
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write((((posX).wrapping_add(48i32)) as i16));
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    prevTaskId = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((prevTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(1i16);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                currPosX = ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .cast::<u8>())
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read();
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read())
                    != 0)
                {
                    if ((currPosX) as i32).wrapping_add(((deltaX) as i32)) < 240i32 {
                        sliding = 1u8;
                    } else {
                        sliding = 0u8;
                        posX = 240i32;
                    }
                } else {
                    if ((currPosX) as i32).wrapping_add(((deltaX) as i32)) > 192i32 {
                        sliding = 1u8;
                    } else {
                        sliding = 0u8;
                        posX = 192i32;
                    }
                }
                if ((sliding) as i32) == 1i32 {
                    {
                        i = 0u8;
                        'l8: loop {
                            if !(((i) as u32) < crate::c::div_u32(4u32, 2u32)) {
                                break 'l8;
                            }
                            'l9: {
                                {
                                    j = 0u8;
                                    'l10: loop {
                                        if !(((j) as u32) < crate::c::div_u32(2u32, 1u32)) {
                                            break 'l10;
                                        }
                                        'l11: {
                                            let __p3 = (((&raw mut gSprites).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((((((((&raw mut sFactorySwapScreen)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(14))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((j) as i32) as isize * 2))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 68,
                                                ))
                                            .wrapping_add(32)
                                            .cast::<i16>();
                                            (__p3).write(
                                                (((((__p3).read()) as i32)
                                                    .wrapping_add(((deltaX) as i32)))
                                                    as i16),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    {
                        j = 0u8;
                        'l12: loop {
                            if !(((j) as u32) < crate::c::div_u32(4u32, 2u32)) {
                                break 'l12;
                            }
                            'l13: {
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(14))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write(((posX) as i16));
                                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(14))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(32)
                                .cast::<i16>())
                                .write((((posX).wrapping_add(16i32)) as i16));
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    prevTaskId = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((prevTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(1i16);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_ScreenInfoTransitionOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut slideTaskId: u8 = 0u8;
        let mut hiPtr: u16 = 0u16;
        let mut loPtr: u16 = 0u16;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                LoadPalette(
                    (((&raw const sSwapText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    224u16,
                    10u16,
                );
                Swap_PrintActionStrings();
                PutWindowTilemap(5u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Swap_ErasePopupMenu(3u8);
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                BeginNormalPaletteFade(
                    16384u32,
                    0i8,
                    0u8,
                    16u8,
                    ((((&raw const sPokeballGray_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(37))
                    .read(),
                );
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FillWindowPixelBuffer(5u8, 0u8);
                    CopyWindowToVram(5u8, 2u8);
                    if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .read()) as i32)
                        == 1i32
                    {
                        slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((taskId) as i16));
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(6i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(5i16);
                        let __p5 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    } else {
                        slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(1i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((taskId) as i16));
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(0i16);
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(6i16);
                        let __p6 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p6).write((((((__p6).read()) as i32).wrapping_add(2i32)) as i16));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(6i16);
                    let __p7 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                } else {
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p8).write(((__p8).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    == 1i32)
                    && (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        == 1i32)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read(),
                    );
                    hiPtr = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u16);
                    loPtr = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read()) as u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                        (((((hiPtr) as i32) << 16) | ((loPtr) as i32)) as usize as *mut u8),
                    ));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_ScreenInfoTransitionIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut slideTaskId: u8 = 0u8;
        let mut hiPtr: u16 = 0u16;
        let mut loPtr: u16 = 0u16;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read()) as i32)
                    == 1i32
                {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(6i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(10i16);
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(6i16);
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(((taskId) as i16));
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((slideTaskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(6i16);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                } else {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    == 1i32)
                    && (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        == 1i32)
                {
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(226))
                    .write(
                        ((((&raw const sPokeballGray_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(37))
                        .read(),
                    );
                    Swap_PrintActionStrings();
                    PutWindowTilemap(5u8);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(
                    16384u32,
                    0i8,
                    16u8,
                    0u8,
                    ((((&raw const sPokeballGray_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(37))
                    .read(),
                );
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    Swap_PrintOneActionString(0u8);
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                Swap_PrintOneActionString(1u8);
                PutWindowTilemap(3u8);
                let __p9 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                FillWindowPixelBuffer(5u8, 0u8);
                CopyWindowToVram(5u8, 2u8);
                let __p10 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read())
                    != 0)
                {
                    Swap_PrintOnInfoWindow((&raw mut gText_SelectPkmnToSwap).cast::<u8>());
                } else {
                    Swap_PrintOnInfoWindow((&raw mut gText_SelectPkmnToAccept).cast::<u8>());
                }
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    < 3i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                Swap_PrintMonCategory();
                let __p11 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                Swap_PrintMonSpeciesForTransition();
                Swap_EraseSpeciesAtFadeWindow();
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(34))
                .write(1u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read(),
                );
                hiPtr = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u16);
                loPtr = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                    (((((hiPtr) as i32) << 16) | ((loPtr) as i32)) as usize as *mut u8),
                ));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_SwitchPartyScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                Swap_PrintMonSpeciesForTransition();
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                Swap_EraseSpeciesAtFadeWindow();
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                CreateTask(Some(Swap_Task_SlideCycleBalls), 0u8);
                ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33))
                    .read()) as i32) as isize
                        * 40,
                ))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Swap_Task_FadeOutSpeciesName));
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (!((FuncIsActiveTask(Some(Swap_Task_SlideCycleBalls))) != 0))
                    && (((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        == 1i32)
                {
                    Swap_EraseSpeciesWindow();
                    if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(20))
                    .read())
                        != 0)
                    {
                        Swap_InitActions(1u8);
                    } else {
                        Swap_InitActions(0u8);
                        {
                            i = 0u8;
                            'l2: loop {
                                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                                    break 'l2;
                                }
                                'l3: {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((((((&raw mut sFactorySwapScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8))
                                            .cast::<u8>())
                                            .wrapping_offset(3))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(62),
                                        2,
                                        1,
                                        (1u16) as i32,
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(5))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(3))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Swap_Task_FadeSpeciesName));
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(39))
                    .write(0u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .write(6u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(38))
                    .write(0u8);
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write((((Swap_Task_HandleChooseMons as *const () as usize as u32) >> 16) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((Swap_Task_HandleChooseMons as *const () as usize as u32) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Swap_Task_ScreenInfoTransitionIn));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_InitStruct() {
    unsafe {
        if ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()) as usize)
            == 0usize
        {
            ((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                .write(AllocZeroed(52u32));
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .write(0u8);
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48))
            .write(0u8);
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(21))
            .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattleFactorySwapScreen() {
    unsafe {
        ((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        SetMainCallback2(Some(CB2_InitSwapScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_InitSwapScreen() {
    unsafe {
        let mut taskId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetHBlankCallback(None);
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
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sSwap_BgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                InitWindows(
                    ((&raw const sSwap_WindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sSwapMenuTilesetBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(1088u32));
                ((&raw mut sSwapMonPicBgTilesetBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(1088u32));
                ((&raw mut sSwapMenuTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(Alloc(2048u32));
                ((&raw mut sSwapMonPicBgTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(2048u32));
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(76u8, 0u16);
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(66u8, 0u16);
                SetGpuReg(70u8, 0u16);
                SetGpuReg(72u8, 0u16);
                SetGpuReg(74u8, 0u16);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                ResetSpriteData();
                ResetTasks();
                FreeAllSpritePalettes();
                ResetAllPicSprites();
                'l6: loop {
                    'l7: {
                        'l8: loop {
                            'l9: {
                                CpuSet(
                                    (((&raw mut gFrontierFactoryMenu_Gfx).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSwapMenuTilesetBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            1088u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
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
                'l10: loop {
                    'l11: {
                        'l12: loop {
                            'l13: {
                                CpuSet(
                                    (((&raw const sMonPicBg_Gfx)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSwapMonPicBgTilesetBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            96u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l12;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l10;
                    }
                }
                LoadBgTiles(
                    1u8,
                    ((&raw mut sSwapMenuTilesetBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    1088u16,
                    0u16,
                );
                LoadBgTiles(
                    3u8,
                    ((&raw mut sSwapMonPicBgTilesetBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    96u16,
                    0u16,
                );
                'l14: loop {
                    'l15: {
                        'l16: loop {
                            'l17: {
                                CpuSet(
                                    (((&raw mut gFrontierFactoryMenu_Tilemap).cast::<u16>())
                                        .cast::<u16>())
                                    .cast::<u8>(),
                                    ((&raw mut sSwapMenuTilemapBuffer)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read(),
                                    ((0i32
                                        | (crate::c::div_i32(
                                            2048i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l16;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l14;
                    }
                }
                LoadBgTilemap(
                    1u8,
                    ((&raw mut sSwapMenuTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    2048u16,
                    0u16,
                );
                LoadPalette(
                    (((&raw mut gFrontierFactoryMenu_Pal).cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    0u16,
                    64u16,
                );
                LoadPalette(
                    (((&raw const sSwapText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    240u16,
                    10u16,
                );
                LoadPalette(
                    (((&raw const sSwapText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    224u16,
                    10u16,
                );
                LoadPalette(
                    (((&raw const sMonPicBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    32u16,
                    4u16,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetBgTilemapBuffer(
                    3u8,
                    ((&raw mut sSwapMonPicBgTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                CopyToBgTilemapBufferRect(
                    3u8,
                    ((&raw const sMonPicBg_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    11u8,
                    4u8,
                    8u8,
                    8u8,
                );
                CopyBgTilemapBufferToVram(3u8);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadSpritePalettes(
                    ((&raw const sSwap_SpritePalettes).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadSpriteSheets(
                    ((&raw const sSwap_SpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                LoadCompressedSpriteSheet(
                    ((&raw const sSwap_BallGfx).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                SetVBlankCallback(Some(Swap_VblankCb));
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (!(((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .is_null())
                    && ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .read())
                        != 0)
                {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(((&raw mut gLastViewedMonIndex).cast::<u8>()).read());
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                Swap_InitStruct();
                Swap_InitAllSprites();
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read()) as i32)
                    == 1i32
                {
                    Swap_ShowSummaryMonSprite();
                }
                Swap_InitActions(0u8);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                Swap_PrintOnInfoWindow((&raw mut gText_SelectPkmnToSwap).cast::<u8>());
                PutWindowTilemap(2u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                Swap_PrintMonCategory();
                PutWindowTilemap(8u8);
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read())
                    != 0)
                {
                    Swap_PrintMonSpecies();
                }
                PutWindowTilemap(1u8);
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                Swap_PrintPkmnSwap();
                PutWindowTilemap(0u8);
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read())
                    != 0
                {
                    Swap_PrintMonSpeciesAtFade();
                }
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                Swap_PrintActionStrings2();
                PutWindowTilemap(3u8);
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetGpuReg(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read()) as i32)
                    == 1i32
                {
                    ShowBg(3u8);
                    SetGpuReg(80u8, 4680u16);
                    SetGpuReg(82u8, 1035u16);
                } else {
                    HideBg(3u8);
                }
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33))
                .write(CreateTask(Some(Swap_Task_FadeSpeciesName), 0u8));
                if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(21))
                .read())
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    taskId = CreateTask(Some(Swap_Task_HandleChooseMons), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                } else {
                    Swap_EraseActionFadeWindow();
                    (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33))
                        .read()) as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(34))
                    .write(0u8);
                    taskId = CreateTask(Some(Swap_Task_HandleMenu), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                SetMainCallback2(Some(Swap_CB2));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_InitAllSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_Pokeball)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(101u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(CreateSprite(
                        (&raw mut spriteTemplate).cast::<u8>(),
                        ((((48i32).wrapping_mul(((i) as i32))).wrapping_add(72i32)) as i16),
                        64i16,
                        1u8,
                    ));
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(5))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(CreateSprite(
                (&raw const sSpriteTemplate_Swap_Arrow)
                    .cast::<u8>()
                    .cast_mut(),
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
                88i16,
                0u8,
            ));
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(CreateSprite(
                (&raw const sSpriteTemplate_Swap_MenuHighlightLeft)
                    .cast::<u8>()
                    .cast_mut(),
                176i16,
                112i16,
                0u8,
            ));
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(CreateSprite(
                (&raw const sSpriteTemplate_Swap_MenuHighlightRight)
                    .cast::<u8>()
                    .cast_mut(),
                176i16,
                144i16,
                0u8,
            ));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(21))
        .read()) as i32)
            == 1i32
        {
            x = 240u8;
        } else {
            x = 192u8;
        }
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_Arrow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(104u16);
        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .cast::<u8>())
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            240i16,
            120i16,
            10u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_MenuHighlightLeft)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(105u16);
        (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            256i16,
            120i16,
            10u8,
        ));
        (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .cast::<u8>())
        .wrapping_offset(2))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            288i16,
            120i16,
            10u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_Arrow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(106u16);
        (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(3))
        .cast::<u8>())
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            240i16,
            120i16,
            1u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_MenuHighlightLeft)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(107u16);
        ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(3))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            256i16,
            120i16,
            1u8,
        ));
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(108u16);
        ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(3))
        .cast::<u8>())
        .wrapping_offset(2))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            288i16,
            120i16,
            1u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_Arrow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(104u16);
        ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14))
        .cast::<u8>())
        .cast::<u8>())
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            ((x) as i16),
            144i16,
            10u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_MenuHighlightLeft)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(105u16);
        (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14))
        .cast::<u8>())
        .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            ((((x) as i32).wrapping_add(16i32)) as i16),
            144i16,
            10u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_Arrow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(106u16);
        (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14))
        .cast::<u8>())
        .wrapping_offset(2))
        .cast::<u8>())
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            ((x) as i16),
            144i16,
            1u8,
        ));
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_Swap_MenuHighlightLeft)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>()).write(108u16);
        ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14))
        .cast::<u8>())
        .wrapping_offset(2))
        .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateSprite(
            (&raw mut spriteTemplate).cast::<u8>(),
            ((((x) as i32).wrapping_add(16i32)) as i16),
            144i16,
            1u8,
        ));
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sFactorySwapScreen)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(41)
                    .cast::<i8>())
                    .write(0i8);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 3))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(14))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(14))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 2))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .cast::<u8>())
                .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .cast::<u8>())
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .cast::<u8>())
                .cast::<u8>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .cast::<u8>())
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .cast::<u8>())
                .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Swap_DestroyAllSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(5))
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
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(6u32, 3u32)) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as u32) < crate::c::div_u32(3u32, 1u32)) {
                                break 'l5;
                            }
                            'l6: {
                                DestroySprite(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 3))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 2u32)) {
                    break 'l7;
                }
                'l8: {
                    {
                        j = 0u8;
                        'l9: loop {
                            if !(((j) as u32) < crate::c::div_u32(2u32, 1u32)) {
                                break 'l9;
                            }
                            'l10: {
                                DestroySprite(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(14))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 2))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_HandleActionCursorChange(cursorId: u8) {
    unsafe {
        let mut cursorId = cursorId;
        if ((cursorId) as i32) < 3i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            Swap_HideActionButtonHighlights();
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .cast::<u8>())
                    .wrapping_offset(((cursorId) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read(),
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            Swap_HighlightActionButton(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((cursorId) as i32) as isize * 8))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_UpdateBallCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        let mut cursorPos: u8 = 0u8;
        PlaySE(5u16);
        if ((direction) as i32) > 0i32 {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                .wrapping_add(1i32)
                != ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(23))
                .read()) as i32)
            {
                let __p1 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .write(0u8);
            }
        } else {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .write(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            }
        }
        cursorPos = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read();
        Swap_HandleActionCursorChange(cursorPos);
    }
}
pub(crate) unsafe extern "C" fn Swap_UpdateActionCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        let mut cursorPos: u8 = 0u8;
        PlaySE(5u16);
        if ((direction) as i32) > 0i32 {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                < 3i32
            {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .write(3u8);
            } else {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    .wrapping_add(1i32)
                    != ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .read()) as i32)
                {
                    let __p1 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(3);
                    (__p1).write(((__p1).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(0u8);
                }
            }
        } else {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                < 3i32
            {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .write(
                    ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            } else {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    != 0i32
                {
                    let __p2 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(3);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(
                        ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(23))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                }
            }
        }
        cursorPos = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read();
        Swap_HandleActionCursorChange(cursorPos);
    }
}
pub(crate) unsafe extern "C" fn Swap_UpdateYesNoCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        if ((direction) as i32) > 0i32 {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                != 1i32
            {
                let __p1 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .write(0u8);
            }
        } else {
            if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22))
                .write(1u8);
            }
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            (((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22))
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Swap_UpdateMenuCursorPosition(direction: i8) {
    unsafe {
        let mut direction = direction;
        PlaySE(5u16);
        if ((direction) as i32) > 0i32 {
            if (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as u32)
                != (crate::c::div_u32(12u32, 4u32)).wrapping_sub(1u32)
            {
                let __p1 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read());
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
            }
        } else {
            if (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                != 0i32
            {
                let __p2 = (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .write((((crate::c::div_u32(12u32, 4u32)).wrapping_sub(1u32)) as u8));
            }
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Swap_HighlightActionButton(actionId: u8) {
    unsafe {
        let mut actionId = actionId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((actionId) as i32) == 2i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(8))
                                .cast::<u8>())
                                .wrapping_offset(3))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                        if ((i) as u32) < crate::c::div_u32(2u32, 1u32) {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(14))
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        }
                    } else {
                        if ((actionId) as i32) == 3i32 {
                            if ((i) as u32) < crate::c::div_u32(2u32, 1u32) {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((((&raw mut sFactorySwapScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(14))
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(62),
                                    2,
                                    1,
                                    (0u16) as i32,
                                );
                            }
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sFactorySwapScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8))
                                    .cast::<u8>())
                                    .wrapping_offset(3))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_HideActionButtonHighlights() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut sFactorySwapScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    if ((i) as u32) < crate::c::div_u32(2u32, 1u32) {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((((&raw mut sFactorySwapScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(14))
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_ShowMenuOptions() {
    unsafe {
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(21))
        .read()) as i32)
            == 1i32
        {
            ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(21))
            .write(0u8);
        } else {
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(176i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(208i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                .wrapping_mul(16i32))
            .wrapping_add(112i32)) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        Swap_PrintMenuOptions();
    }
}
pub(crate) unsafe extern "C" fn Swap_ShowYesNoOptions() {
    unsafe {
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(22))
            .write(0u8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(176i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(112i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(32)
        .cast::<i16>())
        .write(208i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>())
        .write(112i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        Swap_PrintYesNoOptions();
    }
}
pub(crate) unsafe extern "C" fn Swap_ErasePopupMenu(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        FillWindowPixelBuffer(windowId, 0u8);
        CopyWindowToVram(windowId, 2u8);
        ClearWindowTilemap(windowId);
    }
}
pub(crate) unsafe extern "C" fn Swap_EraseSpeciesWindow() {
    unsafe {
        PutWindowTilemap(1u8);
        FillWindowPixelBuffer(1u8, 0u8);
        CopyWindowToVram(1u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_EraseSpeciesAtFadeWindow() {
    unsafe {
        PutWindowTilemap(7u8);
        FillWindowPixelBuffer(7u8, 0u8);
        CopyWindowToVram(7u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_EraseActionFadeWindow() {
    unsafe {
        Swap_EraseSpeciesWindow();
        PutWindowTilemap(5u8);
        FillWindowPixelBuffer(5u8, 0u8);
        CopyWindowToVram(5u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintPkmnSwap() {
    unsafe {
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_PkmnSwap).cast::<u8>(),
            2u8,
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintMonSpecies() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut x: u8 = 0u8;
        FillWindowPixelBuffer(1u8, 0u8);
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read()) as i32)
            >= 3i32
        {
            CopyWindowToVram(1u8, 2u8);
        } else {
            let mut monId: u8 = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(3))
            .read();
            if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read())
                != 0)
            {
                species = ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            } else {
                species = ((GetMonData3(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            }
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 86i32))
                as u8);
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                x,
                1u8,
                ((&raw const sSwapSpeciesNameTextColors)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                0i8,
                (&raw mut gStringVar4).cast::<u8>(),
            );
            CopyWindowToVram(1u8, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintOnInfoWindow(str: *mut u8) {
    unsafe {
        let mut str = str;
        FillWindowPixelBuffer(2u8, 0u8);
        AddTextPrinterParameterized(2u8, 1u8, str, 2u8, 5u8, 0u8, None);
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintMenuOptions() {
    unsafe {
        PutWindowTilemap(3u8);
        FillWindowPixelBuffer(3u8, 0u8);
        AddTextPrinterParameterized3(
            3u8,
            1u8,
            15u8,
            1u8,
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            (&raw mut gText_Summary2).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            3u8,
            1u8,
            15u8,
            17u8,
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            (&raw mut gText_Swap).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            3u8,
            1u8,
            15u8,
            33u8,
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            (&raw mut gText_Rechoose).cast::<u8>(),
        );
        CopyWindowToVram(3u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintYesNoOptions() {
    unsafe {
        PutWindowTilemap(4u8);
        FillWindowPixelBuffer(4u8, 0u8);
        AddTextPrinterParameterized3(
            4u8,
            1u8,
            7u8,
            1u8,
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            (&raw mut gText_Yes3).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            4u8,
            1u8,
            7u8,
            17u8,
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            (&raw mut gText_No3).cast::<u8>(),
        );
        CopyWindowToVram(4u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintActionString(str: *mut u8, y: u32, windowId: u32) {
    unsafe {
        let mut str = str;
        let mut y = y;
        let mut windowId = windowId;
        let mut x: i32 = GetStringRightAlignXOffset(0i32, str, 70i32);
        AddTextPrinterParameterized3(
            ((windowId) as u8),
            0u8,
            ((x) as u8),
            ((y) as u8),
            ((&raw const sSwapMenuOptionsTextColors)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            0i8,
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintActionStrings() {
    unsafe {
        FillWindowPixelBuffer(5u8, 0u8);
        'l1: {
            let __sw1 = ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                Swap_PrintActionString((&raw mut gText_PkmnForSwap).cast::<u8>(), 0u32, 5u32);
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                Swap_PrintActionString((&raw mut gText_Cancel3).cast::<u8>(), 24u32, 5u32);
                break 'l1;
            }
        }
        CopyWindowToVram(5u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintActionStrings2() {
    unsafe {
        FillWindowPixelBuffer(3u8, 0u8);
        'l1: {
            let __sw1 = ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                Swap_PrintActionString((&raw mut gText_PkmnForSwap).cast::<u8>(), 8u32, 3u32);
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                Swap_PrintActionString((&raw mut gText_Cancel3).cast::<u8>(), 32u32, 3u32);
                break 'l1;
            }
        }
        CopyWindowToVram(3u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintOneActionString(which: u8) {
    unsafe {
        let mut which = which;
        'l1: {
            let __sw1 = ((which) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read()) as i32)
                    == 1i32
                {
                    Swap_PrintActionString((&raw mut gText_PkmnForSwap).cast::<u8>(), 8u32, 3u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                Swap_PrintActionString((&raw mut gText_Cancel3).cast::<u8>(), 32u32, 3u32);
                break 'l1;
            }
        }
        CopyWindowToVram(3u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintMonSpeciesAtFade() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut x: u8 = 0u8;
        let mut pal = crate::ffi::Align4([0u8; 10]);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sSwapText_Pal)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .cast::<u8>(),
                            ((&raw mut pal).cast::<u16>()).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(8i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(21))
        .read())
            != 0)
        {
            (((&raw mut pal).cast::<u16>()).wrapping_offset(4)).write(
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(228))
                    .read(),
            );
        } else {
            (((&raw mut pal).cast::<u16>()).wrapping_offset(4)).write(
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<u16>())
                .read(),
            );
        }
        LoadPalette(((&raw mut pal).cast::<u16>()).cast::<u8>(), 240u16, 10u16);
        PutWindowTilemap(7u8);
        FillWindowPixelBuffer(7u8, 0u8);
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read()) as i32)
            >= 3i32
        {
            CopyWindowToVram(7u8, 3u8);
        } else {
            let mut monId: u8 = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(3))
            .read();
            if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read())
                != 0)
            {
                species = ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            } else {
                species = ((GetMonData3(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            }
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 86i32))
                as u8);
            AddTextPrinterParameterized3(
                7u8,
                1u8,
                x,
                1u8,
                ((&raw const sSwapSpeciesNameTextColors)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                0i8,
                (&raw mut gStringVar4).cast::<u8>(),
            );
            CopyWindowToVram(7u8, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintMonSpeciesForTransition() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut x: u8 = 0u8;
        LoadPalette(
            (((&raw const sSwapText_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            10u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(240))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(224))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    10u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read()) as i32)
            >= 3i32
        {
            CopyWindowToVram(1u8, 2u8);
        } else {
            let mut monId: u8 = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(3))
            .read();
            if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read())
                != 0)
            {
                species = ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            } else {
                species = ((GetMonData3(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            }
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 86i32))
                as u8);
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                x,
                1u8,
                ((&raw const sSwapSpeciesNameTextColors)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                0i8,
                (&raw mut gStringVar4).cast::<u8>(),
            );
            CopyWindowToVram(1u8, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_PrintMonCategory() {
    unsafe {
        let mut species: u16 = 0u16;
        let mut text = crate::ffi::Align4([0u8; 30]);
        let mut x: u8 = 0u8;
        let mut monId: u8 = ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(3))
        .read();
        FillWindowPixelBuffer(8u8, 0u8);
        if ((monId) as i32) >= 3i32 {
            CopyWindowToVram(8u8, 2u8);
        } else {
            PutWindowTilemap(8u8);
            if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .read())
                != 0)
            {
                species = ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            } else {
                species = ((GetMonData3(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16);
            }
            CopyMonCategoryText(
                ((SpeciesToNationalPokedexNum(species)) as i32),
                (&raw mut text).cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 118i32)) as u8);
            AddTextPrinterParameterized(8u8, 1u8, (&raw mut text).cast::<u8>(), x, 1u8, 0u8, None);
            CopyWindowToVram(8u8, 2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_InitActions(id: u8) {
    unsafe {
        let mut id = id;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(21))
        .read()) as i32)
            != 1i32
        {
            'l1: {
                let __sw1 = ((id) as i32);
                if __sw1 == 0i32 {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .write(0u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(0u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .write(((crate::c::div_u32(32u32, 8u32)) as u8));
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sSwap_PlayerScreenActions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .write(1u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .write(0u8);
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .write(((crate::c::div_u32(40u32, 8u32)) as u8));
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sSwap_EnemyScreenActions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_RunMenuOptionFunc(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut sSwap_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(
            ((((&raw const sSwap_MenuOptionFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32) as isize,
            ))
            .read(),
        );
        (((&raw mut sSwap_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn Swap_OptionSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CloseMonPic(
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48),
            1u8,
        );
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read(),
            );
        Swap_ErasePopupMenu(3u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_SwitchPartyScreen));
    }
}
pub(crate) unsafe extern "C" fn Swap_OptionSummary(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(6i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_OpenSummaryScreen));
    }
}
pub(crate) unsafe extern "C" fn Swap_OptionRechoose(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CloseMonPic(
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48),
            1u8,
        );
        Swap_ErasePopupMenu(3u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write((((Swap_Task_HandleChooseMons as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((Swap_Task_HandleChooseMons as *const () as usize as u32) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(1i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_ScreenInfoTransitionIn));
    }
}
pub(crate) unsafe extern "C" fn Swap_RunActionFunc(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut sSwap_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(
            (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32) as isize
                    * 8,
            ))
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read(),
        );
        (((&raw mut sSwap_CurrentOptionFunc)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn Swap_ActionCancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write((((Swap_AskQuitSwapping as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((Swap_AskQuitSwapping as *const () as usize as u32) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_ScreenInfoTransitionOut));
    }
}
pub(crate) unsafe extern "C" fn Swap_ActionPkmnForSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write((((Swap_Task_SwitchPartyScreen as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((Swap_Task_SwitchPartyScreen as *const () as usize as u32) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_ScreenInfoTransitionOut));
    }
}
pub(crate) unsafe extern "C" fn Swap_ActionMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20))
        .read())
            != 0)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((Swap_Task_HandleMenu as *const () as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((Swap_Task_HandleMenu as *const () as usize as u32) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(2i16);
        } else {
            if ((Swap_AlreadyHasSameSpecies(
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read(),
            )) as i32)
                == 1i32
            {
                OpenMonPic(
                    ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(44))
                    .wrapping_add(1),
                    (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(48),
                    1u8,
                );
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(1i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Swap_TaskCantHaveSameMons));
                return;
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write((((Swap_AskAcceptMon as *const () as usize as u32) >> 16) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((Swap_AskAcceptMon as *const () as usize as u32) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(0i16);
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Swap_Task_ScreenInfoTransitionOut));
    }
}
pub(crate) unsafe extern "C" fn OpenMonPic(spriteId: *mut u8, animating: *mut u8, swapScreen: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut animating = animating;
        let mut swapScreen = swapScreen;
        (spriteId).write(CreateSprite(
            (&raw const sSpriteTemplate_Swap_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            64i16,
            1u8,
        ));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteId).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_OpenMonPic));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteId).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((swapScreen) as i16));
        (animating).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_ShowSummaryMonSprite() {
    unsafe {
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        (((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(44))
        .wrapping_add(1))
        .write(CreateSprite(
            (&raw const sSpriteTemplate_Swap_MonPicBgAnim)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            64i16,
            1u8,
        ));
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(44))
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ),
            2u8,
        );
        mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32) as isize
                * 100,
        );
        species = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        personality = GetMonData3(mon, 0i32, core::ptr::null_mut());
        otId = GetMonData3(mon, 1i32, core::ptr::null_mut());
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .write(
                ((CreateMonPicSprite_HandleDeoxys(
                    species,
                    personality,
                    otId,
                    1u8,
                    88i16,
                    32i16,
                    15u8,
                    65535u16,
                )) as u8),
            );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(44))
                .wrapping_add(1))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn CloseMonPic(
    pic__v: crate::c::Rec4<4>,
    animating: *mut u8,
    swapScreen: u8,
) {
    unsafe {
        let mut pic = crate::ffi::Align4([0u8; 4]);
        (&raw mut pic)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(pic__v);
        let mut animating = animating;
        let mut swapScreen = swapScreen;
        let mut taskId: u8 = 0u8;
        FreeAndDestroyMonPicSprite(((((&raw mut pic).cast::<u8>()).read()) as u16));
        taskId = CreateTask(Some(Task_CloseMonPic), 1u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((swapScreen) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write((((((&raw mut pic).cast::<u8>()).wrapping_add(1)).read()) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
        (animating).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn HideMonPic(pic__v: crate::c::Rec4<4>, animating: *mut u8) {
    unsafe {
        let mut pic = crate::ffi::Align4([0u8; 4]);
        (&raw mut pic)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(pic__v);
        let mut animating = animating;
        FreeAndDestroyMonPicSprite(((((&raw mut pic).cast::<u8>()).read()) as u16));
        FreeOamMatrix(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut pic).cast::<u8>()).wrapping_add(1)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(3),
                1,
                5,
                false,
            ) as u32) as u8),
        );
        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut pic).cast::<u8>()).wrapping_add(1)).read()) as i32) as isize * 68,
        ));
        (animating).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Swap_TaskCantHaveSameMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48))
        .read()) as i32)
            == 1i32
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                Swap_PrintOnInfoWindow((&raw mut gText_SamePkmnInPartyAlready).cast::<u8>());
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32))
                .write(0u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
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
                    CloseMonPic(
                        (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(44)
                            .cast::<crate::c::Rec4<4>>()
                            .read_unaligned(),
                        (((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(48),
                        1u8,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48))
                .read()) as i32)
                    != 1i32
                {
                    FillWindowPixelBuffer(5u8, 0u8);
                    CopyWindowToVram(5u8, 2u8);
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                Swap_PrintOnInfoWindow((&raw mut gText_SelectPkmnToAccept).cast::<u8>());
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                Swap_PrintMonSpeciesForTransition();
                Swap_EraseSpeciesAtFadeWindow();
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(34))
                .write(1u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read(),
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Swap_Task_HandleChooseMons));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_AlreadyHasSameSpecies(monId: u8) -> u8 {
    unsafe {
        let mut monId = monId;
        let mut i: u8 = 0u8;
        let mut species: u16 = ((GetMonData3(
            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(((monId) as i32) as isize * 100),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if (((i) as i32)
                        != ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(18))
                        .read()) as i32))
                        && ((((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16) as i32)
                            == ((species) as i32))
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OpenMonPic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut taskId: u8 = 0u8;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            taskId = CreateTask(Some(Task_OpenMonPic), 1u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CloseMonPic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 1i32
            {
                ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48))
                .write(0u8);
            } else {
                Select_SetMonPicAnimating(0u8);
            }
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenMonPic(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(88i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(152i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(64i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(65i16);
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 55u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ShowBg(3u8);
                SetGpuReg(80u8, 4680u16);
                SetGpuReg(82u8, 1035u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as i16));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    <= 32i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                        >= 96i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(32i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(96i16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != 32i32
                {
                    return;
                }
                break 'l1;
            }
            if !__matched {
                DestroyTask(taskId);
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as i32)
                    == 1i32
                {
                    Swap_CreateMonSprite();
                } else {
                    Select_CreateMonSprite();
                }
                return;
            }
        }
        let __p4 = ((task).wrapping_add(8)).cast::<i16>();
        (__p4).write(((__p4).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_CloseMonPic(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(88i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(152i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(32i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(96i16);
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 55u16);
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    >= 64i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                        <= 65i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(64i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(65i16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 64i32
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                HideBg(3u8);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read());
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_CloseMonPic));
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                ClearGpuRegBits(0u8, 8192u16);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_CreateMonSprite() {
    unsafe {
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        if !((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20))
        .read())
            != 0)
        {
            mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32) as isize
                    * 100,
            );
        } else {
            mon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32) as isize
                    * 100,
            );
        }
        species = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
        personality = GetMonData3(mon, 0i32, core::ptr::null_mut());
        otId = GetMonData3(mon, 1i32, core::ptr::null_mut());
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .write(
                ((CreateMonPicSprite_HandleDeoxys(
                    species,
                    otId,
                    personality,
                    1u8,
                    88i16,
                    32i16,
                    15u8,
                    65535u16,
                )) as u8),
            );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(40)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(41)
        .cast::<i8>())
        .write(0i8);
        ((((&raw mut sFactorySwapScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(48))
            .write(0u8);
    }
}
