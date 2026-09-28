//! Translated from `src/starter_choose.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gBirchBagGrass_Pal sPokeballSelection_Pal sStarterCircle_Pal gBirchBagTilemap gBirchGrassTilemap gBirchBagGrass_Gfx gPokeballSelection_Gfx sStarterCircle_Gfx sWindowTemplates sWindowTemplate_ConfirmStarter sWindowTemplate_StarterLabel sPokeballCoords sStarterLabelCoords sStarterMon sBgTemplates sTextColors sOam_Hand sOam_Pokeball sOam_StarterCircle sCursorCoords sAnim_Hand sAnim_Pokeball_Still sAnim_Pokeball_Moving sAnim_StarterCircle sAnims_Hand sAnims_Pokeball sAnims_StarterCircle sAffineAnim_StarterPokemon sAffineAnim_StarterCircle sAffineAnims_StarterPokemon sAffineAnims_StarterCircle sSpriteSheet_PokeballSelect sSpriteSheet_StarterCircle sSpritePalettes_StarterChoose sSpriteTemplate_Hand sSpriteTemplate_Pokeball sSpriteTemplate_StarterCircle

const STARTER_MON_COUNT: u16 = 3;
const STARTER_PKMN_POS_X: i16 = 120;
const STARTER_PKMN_POS_Y: i16 = 64;

static gBirchBagGrass_Gfx: Table<CArray<u32, 682>> =
    Table((&raw const crate::data::starter_choose::gBirchBagGrass_Gfx).cast());
static gBirchBagGrass_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::starter_choose::gBirchBagGrass_Pal).cast());
static gBirchBagTilemap: Table<CArray<u32, 99>> =
    Table((&raw const crate::data::starter_choose::gBirchBagTilemap).cast());
static gBirchGrassTilemap: Table<CArray<u32, 126>> =
    Table((&raw const crate::data::starter_choose::gBirchGrassTilemap).cast());
static sAffineAnims_StarterPokemon: Table<*mut AffineAnimCmd> =
    Table((&raw const crate::data::starter_choose::sAffineAnims_StarterPokemon).cast());
static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::starter_choose::sBgTemplates).cast());
static sCursorCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::starter_choose::sCursorCoords).cast());
static sPokeballCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::starter_choose::sPokeballCoords).cast());
static sSpritePalettes_StarterChoose: Table<CArray<SpritePalette, 3>> =
    Table((&raw const crate::data::starter_choose::sSpritePalettes_StarterChoose).cast());
static sSpriteSheet_PokeballSelect: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::starter_choose::sSpriteSheet_PokeballSelect).cast());
static sSpriteSheet_StarterCircle: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::starter_choose::sSpriteSheet_StarterCircle).cast());
static sSpriteTemplate_Hand: Table<SpriteTemplate> =
    Table((&raw const crate::data::starter_choose::sSpriteTemplate_Hand).cast());
static sSpriteTemplate_Pokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::starter_choose::sSpriteTemplate_Pokeball).cast());
static sSpriteTemplate_StarterCircle: Table<SpriteTemplate> =
    Table((&raw const crate::data::starter_choose::sSpriteTemplate_StarterCircle).cast());
static sStarterLabelCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::starter_choose::sStarterLabelCoords).cast());
static sStarterMon: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::starter_choose::sStarterMon).cast());
static sTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::starter_choose::sTextColors).cast());
static sWindowTemplate_ConfirmStarter: Table<WindowTemplate> =
    Table((&raw const crate::data::starter_choose::sWindowTemplate_ConfirmStarter).cast());
static sWindowTemplate_StarterLabel: Table<WindowTemplate> =
    Table((&raw const crate::data::starter_choose::sWindowTemplate_StarterLabel).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::starter_choose::sWindowTemplates).cast());

pub(crate) static mut sStarterLabelWindowId: u16 = 0;

unsafe extern "C" {
    static mut gMain: Main;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BirchInTrouble: CArray<u8, 0>;
    static gText_ConfirmStarterChoice: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
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
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn EnableInterrupts(a0: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeAllSpritePalettes();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
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
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStarterPokemon(mut chosenStarterId: u16) -> u16 {
    if chosenStarterId > STARTER_MON_COUNT {
        chosenStarterId = 0;
    }
    return sStarterMon[chosenStarterId];
}
pub(crate) unsafe extern "C" fn VblankCB_StarterChoose() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ChooseStarter() {
    let mut taskId: u8 = 0;
    let mut spriteId: u8 = 0;
    SetVBlankCallback(None);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as u32);
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
                    volatile_write(dmaRegs.at(1), OAM);
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
                    volatile_write(dmaRegs.at(1), PLTT);
                    volatile_write(dmaRegs.at(2), 0x81000200);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    LZ77UnCompVram(
        gBirchBagGrass_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        gBirchBagTilemap.as_ptr().cast_mut(),
        0x6003000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        gBirchGrassTilemap.as_ptr().cast_mut(),
        0x6003800 as usize as *mut c_void,
    );
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 3);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 0x2A8, 208);
    ClearScheduledBgCopiesToVram();
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    FreeAllSpritePalettes();
    ResetAllPicSprites();
    LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 224, 32);
    LoadPalette(gBirchBagGrass_Pal.as_ptr().cast_mut() as *mut c_void, 0, 64);
    LoadCompressedSpriteSheet((&raw const sSpriteSheet_PokeballSelect[0]).cast_mut());
    LoadCompressedSpriteSheet((&raw const sSpriteSheet_StarterCircle[0]).cast_mut());
    LoadSpritePalettes(sSpritePalettes_StarterChoose.as_ptr().cast_mut());
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    EnableInterrupts(DISPSTAT_VBLANK);
    SetVBlankCallback(Some(VblankCB_StarterChoose));
    SetMainCallback2(Some(CB2_StarterChoose));
    SetGpuReg(REG_OFFSET_WININ, 63);
    SetGpuReg(REG_OFFSET_WINOUT, 31);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 254);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 7);
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
    ShowBg(0);
    ShowBg(2);
    ShowBg(3);
    taskId = CreateTask(Some(Task_StarterChoose), 0);
    gTasks[taskId].data[0] = 1;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_Hand).cast_mut(), 120, 56, 2);
    gSprites[spriteId].data[0] = taskId as i16;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[0][0] as i16,
        sPokeballCoords[0][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = 0;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[1][0] as i16,
        sPokeballCoords[1][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = 1;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[2][0] as i16,
        sPokeballCoords[2][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = 2;
    sStarterLabelWindowId = WINDOW_NONE as u16;
}
pub(crate) unsafe extern "C" fn CB2_StarterChoose() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Task_StarterChoose(taskId: u8) {
    CreateStarterPokemonLabel(gTasks[taskId].data[0] as u8);
    DrawStdFrameWithCustomTileAndPalette(0, 0, 0x2A8, 0xD);
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        gText_BirchInTrouble.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(0);
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(Task_HandleStarterChooseInput);
}
pub(crate) unsafe extern "C" fn Task_HandleStarterChooseInput(taskId: u8) {
    let mut selection: u8 = gTasks[taskId].data[0] as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        let mut spriteId: u8 = 0;
        ClearStarterLabel();
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_StarterCircle).cast_mut(),
            sPokeballCoords[selection][0] as i16,
            sPokeballCoords[selection][1] as i16,
            1,
        );
        gTasks[taskId].data[2] = spriteId as i16;
        spriteId = CreatePokemonFrontSprite(
            GetStarterPokemon(gTasks[taskId].data[0] as u16),
            sPokeballCoords[selection][0],
            sPokeballCoords[selection][1],
        );
        gSprites[spriteId].affineAnims = (&raw const *sAffineAnims_StarterPokemon).cast_mut();
        gSprites[spriteId].callback = Some(SpriteCB_StarterPokemon);
        gTasks[taskId].data[1] = spriteId as i16;
        gTasks[taskId].func = Some(Task_WaitForStarterSprite);
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 && selection > 0 {
        gTasks[taskId].data[0] -= 1;
        gTasks[taskId].func = Some(Task_MoveStarterChooseCursor);
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 && selection < 2 {
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].func = Some(Task_MoveStarterChooseCursor);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForStarterSprite(taskId: u8) {
    if gSprites[gTasks[taskId].data[2]].affineAnimEnded() != 0
        && gSprites[gTasks[taskId].data[2]].x == STARTER_PKMN_POS_X
        && gSprites[gTasks[taskId].data[2]].y == STARTER_PKMN_POS_Y
    {
        gTasks[taskId].func = Some(Task_AskConfirmStarter);
    }
}
pub(crate) unsafe extern "C" fn Task_AskConfirmStarter(taskId: u8) {
    PlayCry_Normal(GetStarterPokemon(gTasks[taskId].data[0] as u16), 0);
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        gText_ConfirmStarterChoice.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    ScheduleBgCopyTilemapToVram(0);
    CreateYesNoMenu(
        (&raw const *sWindowTemplate_ConfirmStarter).cast_mut(),
        0x2A8,
        0xD,
        0,
    );
    gTasks[taskId].func = Some(Task_HandleConfirmStarterInput);
}
pub(crate) unsafe extern "C" fn Task_HandleConfirmStarterInput(taskId: u8) {
    let mut spriteId: u8 = 0;
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            gSpecialVar_Result = gTasks[taskId].data[0] as u16;
            ResetAllPicSprites();
            SetMainCallback2(gMain.savedCallback);
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            spriteId = gTasks[taskId].data[1] as u8;
            FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
            FreeAndDestroyMonPicSprite(spriteId as u16);
            spriteId = gTasks[taskId].data[2] as u8;
            FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
            DestroySprite(&raw mut gSprites[spriteId]);
            gTasks[taskId].func = Some(Task_DeclineStarter);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_DeclineStarter(taskId: u8) {
    gTasks[taskId].func = Some(Task_StarterChoose);
}
pub(crate) unsafe extern "C" fn CreateStarterPokemonLabel(selection: u8) {
    let mut categoryText: CArray<u8, 32> = zeroed();
    let mut winTemplate: WindowTemplate = zeroed();
    let mut speciesName: *mut u8 = null_mut();
    let mut width: i32 = 0;
    let mut labelLeft: u8 = 0;
    let mut labelRight: u8 = 0;
    let mut labelTop: u8 = 0;
    let mut labelBottom: u8 = 0;
    let mut species: u16 = GetStarterPokemon(selection as u16);
    CopyMonCategoryText(
        SpeciesToNationalPokedexNum(species) as i32,
        categoryText.as_mut_ptr(),
    );
    speciesName = gSpeciesNames[species].as_ptr().cast_mut();
    winTemplate = *sWindowTemplate_StarterLabel;
    winTemplate.tilemapLeft = sStarterLabelCoords[selection][0];
    winTemplate.tilemapTop = sStarterLabelCoords[selection][1];
    sStarterLabelWindowId = AddWindow(&raw mut winTemplate);
    FillWindowPixelBuffer(sStarterLabelWindowId as u8, 0);
    width = GetStringCenterAlignXOffset(FONT_NARROW as i32, categoryText.as_mut_ptr(), 0x68);
    AddTextPrinterParameterized3(
        sStarterLabelWindowId as u8,
        FONT_NARROW,
        width as u8,
        1,
        sTextColors.as_ptr().cast_mut(),
        0,
        categoryText.as_mut_ptr(),
    );
    width = GetStringCenterAlignXOffset(FONT_NORMAL as i32, speciesName, 0x68);
    AddTextPrinterParameterized3(
        sStarterLabelWindowId as u8,
        FONT_NORMAL,
        width as u8,
        17,
        sTextColors.as_ptr().cast_mut(),
        0,
        speciesName,
    );
    PutWindowTilemap(sStarterLabelWindowId as u8);
    ScheduleBgCopyTilemapToVram(0);
    labelLeft = sStarterLabelCoords[selection][0] * 8 - 4;
    labelRight = (sStarterLabelCoords[selection][0] + 13) * 8 + 4;
    labelTop = sStarterLabelCoords[selection][1] * 8;
    labelBottom = (sStarterLabelCoords[selection][1] + 4) * 8;
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (labelLeft as u16) << 8 | labelRight as u16,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (labelTop as u16) << 8 | labelBottom as u16,
    );
}
pub(crate) unsafe extern "C" fn ClearStarterLabel() {
    FillWindowPixelBuffer(sStarterLabelWindowId as u8, 0);
    ClearWindowTilemap(sStarterLabelWindowId as u8);
    RemoveWindow(sStarterLabelWindowId as u8);
    sStarterLabelWindowId = WINDOW_NONE as u16;
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn Task_MoveStarterChooseCursor(taskId: u8) {
    ClearStarterLabel();
    gTasks[taskId].func = Some(Task_CreateStarterLabel);
}
pub(crate) unsafe extern "C" fn Task_CreateStarterLabel(taskId: u8) {
    CreateStarterPokemonLabel(gTasks[taskId].data[0] as u8);
    gTasks[taskId].func = Some(Task_HandleStarterChooseInput);
}
pub(crate) unsafe extern "C" fn CreatePokemonFrontSprite(species: u16, x: u8, y: u8) -> u8 {
    let mut spriteId: u8 = 0;
    spriteId = CreateMonPicSprite_Affine(
        species,
        SHINY_ODDS,
        0,
        MON_PIC_AFFINE_FRONT,
        x as i16,
        y as i16,
        14,
        TAG_NONE,
    ) as u8;
    gSprites[spriteId].oam.set_priority(0);
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_SelectionHand(sprite: *mut Sprite) {
    (*sprite).x = sCursorCoords[gTasks[(*sprite).data[0]].data[0]][0] as i16;
    (*sprite).y = sCursorCoords[gTasks[(*sprite).data[0]].data[0]][1] as i16;
    (*sprite).y2 = Sin((*sprite).data[1], 8);
    (*sprite).data[1] = (*sprite).data[1] as u8 as i16 + 4;
}
pub(crate) unsafe extern "C" fn SpriteCB_Pokeball(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[0]].data[0] == (*sprite).data[1] {
        StartSpriteAnimIfDifferent(sprite, 1);
    } else {
        StartSpriteAnimIfDifferent(sprite, 0);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StarterPokemon(sprite: *mut Sprite) {
    if (*sprite).x > STARTER_PKMN_POS_X {
        (*sprite).x -= 4;
    }
    if (*sprite).x < STARTER_PKMN_POS_X {
        (*sprite).x += 4;
    }
    if (*sprite).y > STARTER_PKMN_POS_Y {
        (*sprite).y -= 2;
    }
    if (*sprite).y < STARTER_PKMN_POS_Y {
        (*sprite).y += 2;
    }
}
