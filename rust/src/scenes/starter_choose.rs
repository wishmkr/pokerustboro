//! Translated from `src/starter_choose.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{ChangeBgX, ChangeBgY, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::gSpecialVar_Result;
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::international_string_util::{CopyMonCategoryText, GetStringCenterAlignXOffset};
use crate::menu::{
    AddTextPrinterParameterized3, ClearScheduledBgCopiesToVram, CreateYesNoMenu,
    DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    Menu_ProcessInputNoWrapClearOnChoose, ScheduleBgCopyTilemapToVram,
};
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
};
use crate::pokemon::SpeciesToNationalPokedexNum;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{PlayCry_Normal, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::task::{ResetTasks, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trainer_pokemon_sprites::{
    CreateMonPicSprite_Affine, FreeAndDestroyMonPicSprite, ResetAllPicSprites,
};
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{ClearWindowTilemap, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const tStarterSelection: usize = 0;
const sBallId: usize = 1;
const tPkmnSpriteId: usize = 1;
const tCircleSpriteId: usize = 2;
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

pub(crate) static sStarterLabelWindowId: crate::global::Global<u16> = crate::global::Global::new(0);

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `GetOverworldTextboxPalettePtr` with this module's view of its types.
#[inline]
unsafe fn GetOverworldTextboxPalettePtr() -> *mut u16 {
    unsafe { crate::text_window::GetOverworldTextboxPalettePtr() as *mut u16 }
}
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

pub unsafe fn GetStarterPokemon(mut chosenStarterId: u16) -> u16 {
    if chosenStarterId > STARTER_MON_COUNT {
        chosenStarterId = 0;
    }
    sStarterMon[chosenStarterId]
}
pub(crate) unsafe fn VblankCB_StarterChoose() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub unsafe fn CB2_ChooseStarter() {
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
        0x6003000_usize as *mut c_void,
    );
    LZ77UnCompVram(
        gBirchGrassTilemap.as_ptr().cast_mut(),
        0x6003800_usize as *mut c_void,
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
    let taskId: u8 = CreateTask(Some(Task_StarterChoose), 0);
    task_set(taskId, 0, 1);
    let mut spriteId: u8 = CreateSprite((&raw const *sSpriteTemplate_Hand).cast_mut(), 120, 56, 2);
    gSprites[spriteId].data[0] = taskId as i16;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[0][0] as i16,
        sPokeballCoords[0][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[sBallId] = 0;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[1][0] as i16,
        sPokeballCoords[1][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[sBallId] = 1;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
        sPokeballCoords[2][0] as i16,
        sPokeballCoords[2][1] as i16,
        2,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[sBallId] = 2;
    sStarterLabelWindowId.set(WINDOW_NONE as u16);
}
pub(crate) unsafe fn CB2_StarterChoose() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn Task_StarterChoose(taskId: u8) {
    CreateStarterPokemonLabel(task_get(taskId, tStarterSelection) as u8);
    DrawStdFrameWithCustomTileAndPalette(0, 0, 0x2A8, 0xD);
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_BirchInTrouble).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(0);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(Task_HandleStarterChooseInput));
}
pub(crate) unsafe fn Task_HandleStarterChooseInput(taskId: u8) {
    let selection: u8 = task_get(taskId, tStarterSelection) as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        ClearStarterLabel();
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_StarterCircle).cast_mut(),
            sPokeballCoords[selection][0] as i16,
            sPokeballCoords[selection][1] as i16,
            1,
        );
        task_set(taskId, tCircleSpriteId, spriteId as i16);
        spriteId = CreatePokemonFrontSprite(
            GetStarterPokemon(task_get(taskId, tStarterSelection) as u16),
            sPokeballCoords[selection][0],
            sPokeballCoords[selection][1],
        );
        gSprites[spriteId].affineAnims = (&raw const *sAffineAnims_StarterPokemon).cast_mut();
        gSprites[spriteId].callback = Some(SpriteCB_StarterPokemon);
        task_set(taskId, tPkmnSpriteId, spriteId as i16);
        task_set_func(taskId, Some(Task_WaitForStarterSprite));
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 && selection > 0 {
        task_set(
            taskId,
            tStarterSelection,
            task_get(taskId, tStarterSelection) - 1,
        );
        task_set_func(taskId, Some(Task_MoveStarterChooseCursor));
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 && selection < 2 {
        task_set(
            taskId,
            tStarterSelection,
            task_get(taskId, tStarterSelection) + 1,
        );
        task_set_func(taskId, Some(Task_MoveStarterChooseCursor));
    }
}
pub(crate) unsafe fn Task_WaitForStarterSprite(taskId: u8) {
    if gSprites[task_get(taskId, tCircleSpriteId)].affineAnimEnded() != 0
        && gSprites[task_get(taskId, tCircleSpriteId)].x == STARTER_PKMN_POS_X
        && gSprites[task_get(taskId, tCircleSpriteId)].y == STARTER_PKMN_POS_Y
    {
        task_set_func(taskId, Some(Task_AskConfirmStarter));
    }
}
pub(crate) unsafe fn Task_AskConfirmStarter(taskId: u8) {
    PlayCry_Normal(
        GetStarterPokemon(task_get(taskId, tStarterSelection) as u16),
        0,
    );
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ConfirmStarterChoice).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
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
    task_set_func(taskId, Some(Task_HandleConfirmStarterInput));
}
pub(crate) unsafe fn Task_HandleConfirmStarterInput(taskId: u8) {
    let mut spriteId: u8 = 0;
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            gSpecialVar_Result = task_get(taskId, tStarterSelection) as u16;
            ResetAllPicSprites();
            SetMainCallback2(gMain.savedCallback);
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            spriteId = task_get(taskId, tPkmnSpriteId) as u8;
            FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
            FreeAndDestroyMonPicSprite(spriteId as u16);
            spriteId = task_get(taskId, tCircleSpriteId) as u8;
            FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
            DestroySprite(&raw mut gSprites[spriteId]);
            task_set_func(taskId, Some(Task_DeclineStarter));
        }
        _ => {}
    }
}
pub(crate) fn Task_DeclineStarter(taskId: u8) {
    task_set_func(taskId, Some(Task_StarterChoose));
}
unsafe fn CreateStarterPokemonLabel(selection: u8) {
    let mut categoryText: CArray<u8, 32> = zeroed();
    let species: u16 = GetStarterPokemon(selection as u16);
    CopyMonCategoryText(
        SpeciesToNationalPokedexNum(species) as i32,
        categoryText.as_mut_ptr(),
    );
    let speciesName: *mut u8 = (*(&raw const crate::data::data_tables::gSpeciesNames)
        .cast::<CArray<CArray<u8, 11>, 0>>())[species]
        .as_ptr()
        .cast_mut();
    let mut winTemplate: WindowTemplate = *sWindowTemplate_StarterLabel;
    winTemplate.tilemapLeft = sStarterLabelCoords[selection][0];
    winTemplate.tilemapTop = sStarterLabelCoords[selection][1];
    sStarterLabelWindowId.set(AddWindow(&raw mut winTemplate));
    FillWindowPixelBuffer(sStarterLabelWindowId.get() as u8, 0);
    let mut width: i32 =
        GetStringCenterAlignXOffset(FONT_NARROW as i32, categoryText.as_mut_ptr(), 0x68);
    AddTextPrinterParameterized3(
        sStarterLabelWindowId.get() as u8,
        FONT_NARROW,
        width as u8,
        1,
        sTextColors.as_ptr().cast_mut(),
        0,
        categoryText.as_mut_ptr(),
    );
    width = GetStringCenterAlignXOffset(FONT_NORMAL as i32, speciesName, 0x68);
    AddTextPrinterParameterized3(
        sStarterLabelWindowId.get() as u8,
        FONT_NORMAL,
        width as u8,
        17,
        sTextColors.as_ptr().cast_mut(),
        0,
        speciesName,
    );
    PutWindowTilemap(sStarterLabelWindowId.get() as u8);
    ScheduleBgCopyTilemapToVram(0);
    let labelLeft: u8 = sStarterLabelCoords[selection][0] * 8 - 4;
    let labelRight: u8 = (sStarterLabelCoords[selection][0] + 13) * 8 + 4;
    let labelTop: u8 = sStarterLabelCoords[selection][1] * 8;
    let labelBottom: u8 = (sStarterLabelCoords[selection][1] + 4) * 8;
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (labelLeft as u16) << 8 | labelRight as u16,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (labelTop as u16) << 8 | labelBottom as u16,
    );
}
unsafe fn ClearStarterLabel() {
    FillWindowPixelBuffer(sStarterLabelWindowId.get() as u8, 0);
    ClearWindowTilemap(sStarterLabelWindowId.get() as u8);
    RemoveWindow(sStarterLabelWindowId.get() as u8);
    sStarterLabelWindowId.set(WINDOW_NONE as u16);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe fn Task_MoveStarterChooseCursor(taskId: u8) {
    ClearStarterLabel();
    task_set_func(taskId, Some(Task_CreateStarterLabel));
}
pub(crate) unsafe fn Task_CreateStarterLabel(taskId: u8) {
    CreateStarterPokemonLabel(task_get(taskId, tStarterSelection) as u8);
    task_set_func(taskId, Some(Task_HandleStarterChooseInput));
}
unsafe fn CreatePokemonFrontSprite(species: u16, x: u8, y: u8) -> u8 {
    let spriteId: u8 = CreateMonPicSprite_Affine(
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
    spriteId
}
pub(crate) unsafe fn SpriteCB_SelectionHand(sprite: *mut Sprite) {
    (*sprite).x = sCursorCoords[task_get((*sprite).data[0], 0)][0] as i16;
    (*sprite).y = sCursorCoords[task_get((*sprite).data[0], 0)][1] as i16;
    (*sprite).y2 = Sin((*sprite).data[1], 8);
    (*sprite).data[1] = (*sprite).data[1] as u8 as i16 + 4;
}
pub(crate) unsafe fn SpriteCB_Pokeball(sprite: *mut Sprite) {
    if task_get((*sprite).data[0], 0) == (*sprite).data[sBallId] {
        StartSpriteAnimIfDifferent(sprite, 1);
    } else {
        StartSpriteAnimIfDifferent(sprite, 0);
    }
}
pub(crate) unsafe fn SpriteCB_StarterPokemon(sprite: *mut Sprite) {
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
