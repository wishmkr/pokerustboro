//! Translated from `src/title_screen.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sUnusedUnknownPal sTitleScreenRayquazaGfx sTitleScreenRayquazaTilemap sTitleScreenLogoShineGfx sTitleScreenCloudsGfx gTitleScreenAlphaBlend sVersionBannerLeftOamData sVersionBannerRightOamData sVersionBannerLeftAnimSequence sVersionBannerRightAnimSequence sVersionBannerLeftAnimTable sVersionBannerRightAnimTable sVersionBannerLeftSpriteTemplate sVersionBannerRightSpriteTemplate sSpriteSheet_EmeraldVersion sOamData_CopyrightBanner sAnim_PressStart_0 sAnim_PressStart_1 sAnim_PressStart_2 sAnim_PressStart_3 sAnim_PressStart_4 sAnim_Copyright_0 sAnim_Copyright_1 sAnim_Copyright_2 sAnim_Copyright_3 sAnim_Copyright_4 sStartCopyrightBannerAnimTable sStartCopyrightBannerSpriteTemplate sSpriteSheet_PressStart sSpritePalette_PressStart sPokemonLogoShineOamData sPokemonLogoShineAnimSequence sPokemonLogoShineAnimTable sPokemonLogoShineSpriteTemplate sPokemonLogoShineSpriteSheet

const A_B_START_SELECT: i32 = 15;
const BERRY_UPDATE_BUTTON_COMBO: i32 = 6;
const CLEAR_SAVE_BUTTON_COMBO: i32 = 70;
const NUM_COPYRIGHT_FRAMES: u8 = 5;
const NUM_PRESS_START_FRAMES: u8 = 5;
const RESET_RTC_BUTTON_COMBO: i32 = 38;
const SHINE_MODE_DOUBLE: u8 = 1;
const SHINE_MODE_SINGLE: u8 = 2;
const SHINE_MODE_SINGLE_NO_BG_COLOR: u8 = 0;
const SHINE_SPEED: i16 = 4;
const START_BANNER_X: i16 = 128;
const VERSION_BANNER_LEFT_X: i16 = 98;
const VERSION_BANNER_RIGHT_X: i16 = 162;
const VERSION_BANNER_Y: i16 = 2;
const VERSION_BANNER_Y_GOAL: i16 = 66;

static gTitleScreenAlphaBlend: Table<CArray<u16, 64>> =
    Table((&raw const crate::data::title_screen::gTitleScreenAlphaBlend).cast());
static sPokemonLogoShineSpriteSheet: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::title_screen::sPokemonLogoShineSpriteSheet).cast());
static sPokemonLogoShineSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::title_screen::sPokemonLogoShineSpriteTemplate).cast());
static sSpritePalette_PressStart: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::title_screen::sSpritePalette_PressStart).cast());
static sSpriteSheet_EmeraldVersion: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::title_screen::sSpriteSheet_EmeraldVersion).cast());
static sSpriteSheet_PressStart: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::title_screen::sSpriteSheet_PressStart).cast());
static sStartCopyrightBannerSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::title_screen::sStartCopyrightBannerSpriteTemplate).cast());
static sTitleScreenCloudsGfx: Table<CArray<u32, 185>> =
    Table((&raw const crate::data::title_screen::sTitleScreenCloudsGfx).cast());
static sTitleScreenRayquazaGfx: Table<CArray<u32, 505>> =
    Table((&raw const crate::data::title_screen::sTitleScreenRayquazaGfx).cast());
static sTitleScreenRayquazaTilemap: Table<CArray<u32, 192>> =
    Table((&raw const crate::data::title_screen::sTitleScreenRayquazaTilemap).cast());
static sVersionBannerLeftSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::title_screen::sVersionBannerLeftSpriteTemplate).cast());
static sVersionBannerRightSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::title_screen::sVersionBannerRightSpriteTemplate).cast());

unsafe extern "C" {
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gTitleScreenBgPalettes: CArray<u16, 0>;
    static gTitleScreenCloudsTilemap: CArray<u32, 0>;
    static gTitleScreenEmeraldVersionPal: CArray<u16, 0>;
    static gTitleScreenPokemonLogoGfx: CArray<u32, 0>;
    static gTitleScreenPokemonLogoTilemap: CArray<u32, 0>;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_InitBerryFixProgram();
    fn CB2_InitClearSaveDataScreen();
    fn CB2_InitCopyrightScreenAfterTitleScreen();
    fn CB2_InitMainMenu();
    fn CB2_InitResetRtcScreen();
    fn CanResetRTC() -> u32;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn EnableInterrupts(a0: u16);
    fn FadeOutBGM(a0: u8);
    fn FreeAllSpritePalettes();
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn PanFadeAndZoomScreen(a0: u16, a1: u16, a2: u16, a3: u16);
    fn ProcessSpriteCopyRequests();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_InitWave(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayAllStop();
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn SpriteCB_VersionBannerLeft(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[1]].data[1] != 0 {
        (*sprite).oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
        (*sprite).y = VERSION_BANNER_Y_GOAL;
    } else {
        if (*sprite).y != VERSION_BANNER_Y_GOAL {
            (*sprite).y += 1;
        }
        if (*sprite).data[0] != 0 {
            (*sprite).data[0] -= 1;
        }
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            gTitleScreenAlphaBlend[(*sprite).data[0]],
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_VersionBannerRight(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[1]].data[1] != 0 {
        (*sprite).oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
        (*sprite).y = VERSION_BANNER_Y_GOAL;
    } else {
        if (*sprite).y != VERSION_BANNER_Y_GOAL {
            (*sprite).y += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PressStartCopyrightBanner(sprite: *mut Sprite) {
    if (*sprite).data[0] == TRUE as i16 {
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) as i32
            & 16
            != 0
        {
            (*sprite).set_invisible(FALSE as u16);
        } else {
            (*sprite).set_invisible(TRUE as u16);
        }
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn CreatePressStartBanner(mut x: i16, y: i16) {
    let mut i: u8 = 0;
    let mut spriteId: u8 = 0;
    x -= 64;
    i = 0;
    while i < NUM_PRESS_START_FRAMES {
        spriteId = CreateSprite(
            (&raw const *sStartCopyrightBannerSpriteTemplate).cast_mut(),
            x,
            y,
            0,
        );
        StartSpriteAnim(&raw mut gSprites[spriteId], i);
        gSprites[spriteId].data[0] = TRUE as i16;
        i += 1;
        x += 32;
    }
}
pub(crate) unsafe extern "C" fn CreateCopyrightBanner(mut x: i16, y: i16) {
    let mut i: u8 = 0;
    let mut spriteId: u8 = 0;
    x -= 64;
    i = 0;
    while i < NUM_COPYRIGHT_FRAMES {
        spriteId = CreateSprite(
            (&raw const *sStartCopyrightBannerSpriteTemplate).cast_mut(),
            x,
            y,
            0,
        );
        StartSpriteAnim(&raw mut gSprites[spriteId], i + NUM_PRESS_START_FRAMES);
        i += 1;
        x += 32;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokemonLogoShine(sprite: *mut Sprite) {
    if (*sprite).x < 272 {
        if (*sprite).data[0] != SHINE_MODE_SINGLE_NO_BG_COLOR as i16 {
            let mut backgroundColor: u16 = 0;
            if (*sprite).x < 120 {
                if (*sprite).data[1] < 31 {
                    (*sprite).data[1] += 1;
                }
                if (*sprite).data[1] < 31 {
                    (*sprite).data[1] += 1;
                }
            } else {
                if (*sprite).data[1] != 0 {
                    (*sprite).data[1] -= 1;
                }
                if (*sprite).data[1] != 0 {
                    (*sprite).data[1] -= 1;
                }
            }
            backgroundColor = (((*sprite).data[1] as u16 & 0x1F) << 10)
                + (((*sprite).data[1] as u16 & 0x1F) << 5)
                + ((*sprite).data[1] as u16 & 0x1F);
            if (*sprite).x == 132 || (*sprite).x == 136 || (*sprite).x == 140 || (*sprite).x == 144
            {
                gPlttBufferFaded[0] = 13304;
            } else {
                gPlttBufferFaded[0] = backgroundColor;
            }
        }
        (*sprite).x += SHINE_SPEED;
    } else {
        gPlttBufferFaded[0] = 0;
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokemonLogoShine_Fast(sprite: *mut Sprite) {
    if (*sprite).x < 272 {
        (*sprite).x += 8;
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn StartPokemonLogoShine(mode: u8) {
    let mut spriteId: u8 = 0;
    match mode {
        SHINE_MODE_SINGLE_NO_BG_COLOR | SHINE_MODE_SINGLE => {
            spriteId = CreateSprite(
                (&raw const *sPokemonLogoShineSpriteTemplate).cast_mut(),
                0,
                68,
                0,
            );
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
            gSprites[spriteId].data[0] = mode as i16;
        }
        SHINE_MODE_DOUBLE => {
            spriteId = CreateSprite(
                (&raw const *sPokemonLogoShineSpriteTemplate).cast_mut(),
                0,
                68,
                0,
            );
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
            gSprites[spriteId].data[0] = mode as i16;
            gSprites[spriteId].set_invisible(TRUE as u16);
            spriteId = CreateSprite(
                (&raw const *sPokemonLogoShineSpriteTemplate).cast_mut(),
                0,
                68,
                0,
            );
            gSprites[spriteId].callback = Some(SpriteCB_PokemonLogoShine_Fast);
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
            spriteId = CreateSprite(
                (&raw const *sPokemonLogoShineSpriteTemplate).cast_mut(),
                -80,
                68,
                0,
            );
            gSprites[spriteId].callback = Some(SpriteCB_PokemonLogoShine_Fast);
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_WINDOW);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    ScanlineEffect_InitHBlankDmaTransfer();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitTitleScreen() {
    'l1: {
        match gMain.state {
            1 => {
                LZ77UnCompVram(
                    gTitleScreenPokemonLogoGfx.as_ptr().cast_mut(),
                    0x6000000 as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    gTitleScreenPokemonLogoTilemap.as_ptr().cast_mut(),
                    0x6004800 as usize as *mut c_void,
                );
                LoadPalette(
                    gTitleScreenBgPalettes.as_ptr().cast_mut() as *mut c_void,
                    0,
                    480,
                );
                LZ77UnCompVram(
                    sTitleScreenRayquazaGfx.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    sTitleScreenRayquazaTilemap.as_ptr().cast_mut(),
                    0x600d000 as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    sTitleScreenCloudsGfx.as_ptr().cast_mut(),
                    0x600c000 as usize as *mut c_void,
                );
                LZ77UnCompVram(
                    gTitleScreenCloudsTilemap.as_ptr().cast_mut(),
                    0x600d800 as usize as *mut c_void,
                );
                ScanlineEffect_Stop();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                gReservedSpritePaletteCount = 9;
                LoadCompressedSpriteSheet((&raw const sSpriteSheet_EmeraldVersion[0]).cast_mut());
                LoadCompressedSpriteSheet((&raw const sSpriteSheet_PressStart[0]).cast_mut());
                LoadCompressedSpriteSheet((&raw const sPokemonLogoShineSpriteSheet[0]).cast_mut());
                LoadPalette(
                    gTitleScreenEmeraldVersionPal.as_ptr().cast_mut() as *mut c_void,
                    256,
                    32,
                );
                LoadSpritePalette((&raw const sSpritePalette_PressStart[0]).cast_mut());
                gMain.state = 2;
            }
            2 => {
                let mut taskId: u8 = CreateTask(Some(Task_TitleScreenPhase1), 0);
                gTasks[taskId].data[0] = 256;
                gTasks[taskId].data[1] = FALSE as i16;
                gTasks[taskId].data[2] = -16;
                gTasks[taskId].data[3] = -32;
                gMain.state = 3;
                break 'l1;
            }
            3 => {
                BeginNormalPaletteFade(PALETTES_ALL, 1, 16, 0, 65535);
                SetVBlankCallback(Some(VBlankCB));
                gMain.state = 4;
            }
            4 => {
                PanFadeAndZoomScreen(120, 80, 0x100, 0);
                SetGpuReg(REG_OFFSET_BG2X_L, 58112);
                SetGpuReg(REG_OFFSET_BG2X_H, 65535);
                SetGpuReg(REG_OFFSET_BG2Y_L, 57344);
                SetGpuReg(REG_OFFSET_BG2Y_H, 65535);
                SetGpuReg(REG_OFFSET_WIN0H, 0);
                SetGpuReg(REG_OFFSET_WIN0V, 0);
                SetGpuReg(REG_OFFSET_WIN1H, 0);
                SetGpuReg(REG_OFFSET_WIN1V, 0);
                SetGpuReg(REG_OFFSET_WININ, 7967);
                SetGpuReg(REG_OFFSET_WINOUT, 16159);
                SetGpuReg(REG_OFFSET_BLDCNT, 132);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                SetGpuReg(REG_OFFSET_BLDY, 12);
                SetGpuReg(REG_OFFSET_BG0CNT, 6667);
                SetGpuReg(REG_OFFSET_BG1CNT, 6926);
                SetGpuReg(REG_OFFSET_BG2CNT, 18817);
                EnableInterrupts(INTR_FLAG_VBLANK);
                SetGpuReg(REG_OFFSET_DISPCNT, 46145);
                m4aSongNumStart(MUS_TITLE);
                gMain.state = 5;
            }
            5 => {
                if UpdatePaletteFade() == 0 {
                    StartPokemonLogoShine(SHINE_MODE_SINGLE_NO_BG_COLOR);
                    ScanlineEffect_InitWave(0, DISPLAY_HEIGHT as u8, 4, 4, 0, 4, TRUE);
                    SetMainCallback2(Some(MainCB2));
                }
            }
            _ => {
                SetVBlankCallback(None);
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                SetGpuReg(REG_OFFSET_BLDY, 0);
                *(PLTT as i32 as usize as *mut u16) = 32767;
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
                                volatile_write(
                                    dmaRegs.at(1),
                                    VRAM as usize as *mut c_void as usize as u32,
                                );
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
                gMain.state = 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MainCB2() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase1(taskId: u8) {
    if gMain.newKeys as i32 & A_B_START_SELECT != 0 || gTasks[taskId].data[1] != 0 {
        gTasks[taskId].data[1] = TRUE as i16;
        gTasks[taskId].data[0] = 0;
    }
    if gTasks[taskId].data[0] != 0 {
        let mut frameNum: u16 = gTasks[taskId].data[0] as u16;
        if frameNum == 176 {
            StartPokemonLogoShine(SHINE_MODE_DOUBLE);
        } else if frameNum == 64 {
            StartPokemonLogoShine(SHINE_MODE_SINGLE);
        }
        gTasks[taskId].data[0] -= 1;
    } else {
        let mut spriteId: u8 = 0;
        SetGpuReg(REG_OFFSET_DISPCNT, 5185);
        SetGpuReg(REG_OFFSET_WININ, 0);
        SetGpuReg(REG_OFFSET_WINOUT, 0);
        SetGpuReg(REG_OFFSET_BLDCNT, 16208);
        SetGpuReg(REG_OFFSET_BLDALPHA, 16);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        spriteId = CreateSprite(
            (&raw const *sVersionBannerLeftSpriteTemplate).cast_mut(),
            VERSION_BANNER_LEFT_X,
            VERSION_BANNER_Y,
            0,
        );
        gSprites[spriteId].data[0] = 64;
        gSprites[spriteId].data[1] = taskId as i16;
        spriteId = CreateSprite(
            (&raw const *sVersionBannerRightSpriteTemplate).cast_mut(),
            VERSION_BANNER_RIGHT_X,
            VERSION_BANNER_Y,
            0,
        );
        gSprites[spriteId].data[1] = taskId as i16;
        gTasks[taskId].data[0] = 144;
        gTasks[taskId].func = Some(Task_TitleScreenPhase2);
    }
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase2(taskId: u8) {
    let mut yPos: u32 = 0;
    if gMain.newKeys as i32 & A_B_START_SELECT != 0 || gTasks[taskId].data[1] != 0 {
        gTasks[taskId].data[1] = TRUE as i16;
        gTasks[taskId].data[0] = 0;
    }
    if gTasks[taskId].data[0] != 0 {
        gTasks[taskId].data[0] -= 1;
    } else {
        gTasks[taskId].data[1] = TRUE as i16;
        SetGpuReg(REG_OFFSET_BLDCNT, 8514);
        SetGpuReg(REG_OFFSET_BLDALPHA, 3846);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        SetGpuReg(REG_OFFSET_DISPCNT, 5953);
        CreatePressStartBanner(START_BANNER_X, 108);
        CreateCopyrightBanner(START_BANNER_X, 148);
        gTasks[taskId].data[4] = 0;
        gTasks[taskId].func = Some(Task_TitleScreenPhase3);
    }
    if gTasks[taskId].data[0] as i32 & 3 == 0 && gTasks[taskId].data[2] != 0 {
        gTasks[taskId].data[2] += 1;
    }
    if gTasks[taskId].data[0] as i32 & 1 == 0 && gTasks[taskId].data[3] != 0 {
        gTasks[taskId].data[3] += 1;
    }
    yPos = gTasks[taskId].data[3] as u32 * 256;
    SetGpuReg(REG_OFFSET_BG2Y_L, yPos as u16);
    SetGpuReg(REG_OFFSET_BG2Y_H, (yPos / 0x10000) as u16);
    gTasks[taskId].data[5] = 15;
    gTasks[taskId].data[6] = 6;
}
pub(crate) unsafe extern "C" fn Task_TitleScreenPhase3(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & START_BUTTON != 0 {
        FadeOutBGM(4);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
        SetMainCallback2(Some(CB2_GoToMainMenu));
    } else if gMain.heldKeys as i32 & CLEAR_SAVE_BUTTON_COMBO == CLEAR_SAVE_BUTTON_COMBO {
        SetMainCallback2(Some(CB2_GoToClearSaveDataScreen));
    } else if gMain.heldKeys as i32 & RESET_RTC_BUTTON_COMBO == RESET_RTC_BUTTON_COMBO
        && CanResetRTC() == TRUE as u32
    {
        FadeOutBGM(4);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_GoToResetRtcScreen));
    } else if gMain.heldKeys as i32 & BERRY_UPDATE_BUTTON_COMBO == BERRY_UPDATE_BUTTON_COMBO {
        FadeOutBGM(4);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        SetMainCallback2(Some(CB2_GoToBerryFixScreen));
    } else {
        SetGpuReg(REG_OFFSET_BG2Y_L, 0);
        SetGpuReg(REG_OFFSET_BG2Y_H, 0);
        if ({
            gTasks[taskId].data[0] += 1;
            gTasks[taskId].data[0]
        }) as i32
            & 1
            != 0
        {
            gTasks[taskId].data[4] += 1;
            gBattle_BG1_Y = (gTasks[taskId].data[4] / 2) as u16;
            gBattle_BG1_X = 0;
        }
        UpdateLegendaryMarkingColor(gTasks[taskId].data[0] as u8);
        if gMPlayInfo_BGM.status & 0xFFFF == 0 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 65535);
            SetMainCallback2(Some(CB2_GoToCopyrightScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToMainMenu() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitMainMenu));
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToCopyrightScreen() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitCopyrightScreenAfterTitleScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToClearSaveDataScreen() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitClearSaveDataScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToResetRtcScreen() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2(Some(CB2_InitResetRtcScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_GoToBerryFixScreen() {
    if UpdatePaletteFade() == 0 {
        m4aMPlayAllStop();
        SetMainCallback2(Some(CB2_InitBerryFixProgram));
    }
}
pub(crate) unsafe extern "C" fn UpdateLegendaryMarkingColor(frameNum: u8) {
    if frameNum as i32 % 4 == 0 {
        let mut intensity: i32 = Cos(frameNum as i16, (0.5f32 as f32 * 256 as f32) as i16) as i32
            + (0.5f32 as f32 * 256 as f32) as i16 as i32;
        let mut r: u32 = 31 - (intensity * 31 / 256) as u32;
        let mut g: u32 = 31 - (intensity * 22 / 256) as u32;
        let mut b: u32 = 12;
        let mut color: u16 = r as u16 | (g as u16) << 5 | (b as u16) << 10;
        LoadPalette(&raw mut color as *mut c_void, 239, 2);
    }
}
