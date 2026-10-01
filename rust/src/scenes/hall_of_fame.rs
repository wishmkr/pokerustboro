//! Translated from `src/hall_of_fame.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, HideBg,
    ResetBgsAndClearDma3BusyFlags, ShowBg, UnsetBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
use crate::confetti_util::{
    ConfettiUtil_Free, ConfettiUtil_Init, ConfettiUtil_Remove, ConfettiUtil_SetData,
    ConfettiUtil_Update,
};
#[allow(unused_imports)]
use crate::consts::*;
use crate::credits::{CB2_StartCreditsSequence, gHasHallOfFameRecords};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005};
use crate::fldeff_misc::{
    ComputerScreenCloseEffect, ComputerScreenOpenEffect, IsComputerScreenCloseEffectActive,
    IsComputerScreenOpenEffectActive,
};
use crate::gpu_regs::SetGpuReg;
use crate::hof_pc::ReturnFromHallOfFamePC;
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayVolumeControl};
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized3, ClearDialogWindowAndFrame,
    DecompressAndCopyTileDataToVram, DrawDialogueFrame, DrawStdFrameWithCustomTileAndPalette,
    FreeTempTileDataBuffersIfPossible, HofPCTopBar_AddWindow, HofPCTopBar_Print,
    HofPCTopBar_PrintPair, HofPCTopBar_RemoveWindow, InitStandardTextBoxWindows,
    InitTextBoxGfxAndPrinters, ResetTempTileDataBuffers,
};
use crate::overworld::GetGameStat;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, BlendPalettesUnfaded, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::{
    DoMonFrontSpriteAnimation, GetGenderFromSpeciesAndPersonality, GetMonData2, GetMonData3,
    SpeciesToPokedexNum, gPlayerParty,
};
use crate::random::Random;
use crate::save::{LoadGameSave, TrySavingData, gDamagedSaveSectors, gGameContinueCallback};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{
    FadeOutBGM, IsCryPlayingOrClearCrySongs, PlayBGM, PlayCry_Normal, PlaySE,
    StopCryAndClearCrySongs,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, FreeSpritePaletteByTag,
    FreeSpriteTilesByTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
    gReservedSpritePaletteCount,
};
use crate::string_util::{gStringVar1, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::text::RunTextPrinters;
use crate::text_window::LoadWindowGfx;
use crate::trainer_pokemon_sprites::{
    CreateMonPicSprite_Affine, CreateMonPicSprite_HandleDeoxys, CreateTrainerPicSprite,
    FreeAndDestroyMonPicSprite, FreeAndDestroyTrainerPicSprite,
    PlayerGenderToFrontTrainerPicId_Debug, ResetAllPicSprites,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
};
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
/// `ConfettiUtil_AddNew` with this module's view of its types.
#[inline]
unsafe fn ConfettiUtil_AddNew(
    a0: *mut OamData,
    a1: u16,
    a2: u16,
    a3: i16,
    a4: i16,
    a5: u8,
    a6: u8,
) -> u8 {
    unsafe { crate::confetti_util::ConfettiUtil_AddNew(a0 as _, a1, a2, a3, a4, a5, a6) }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
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
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sSineIdx: usize = 0;
const tCurrTeamNo: usize = 0;
const tDontSaveData: usize = 0;
const sExtraY: usize = 1;
const tCurrPageNo: usize = 1;
const tDestinationX: usize = 1;
const tDisplayedMonId: usize = 1;
const tTimer: usize = 1;
const tCurrMonId: usize = 2;
const tDestinationY: usize = 2;
const tMonNumber: usize = 2;
const tFrameCount: usize = 3;
const tMonNo: usize = 4;
const tPlayerSpriteID: usize = 4;
const tSpecies: usize = 7;
const tConfettiCount: usize = 15;
// Data tables (translate with cdata.py): sHof_BgTemplates sHof_WindowTemplate sMonInfoTextColors sPlayerInfoTextColors sUnusedTextColors sSpriteSheet_Confetti sSpritePalette_Confetti sHallOfFame_MonFullTeamPositions sHallOfFame_MonHalfTeamPositions sOamData_Confetti sAnim_PinkConfettiA sAnim_RedConfettiA sAnim_BlueConfettiA sAnim_RedConfettiB sAnim_BlueConfettiB sAnim_YellowConfettiA sAnim_WhiteConfettiA sAnim_GreenConfettiA sAnim_PinkConfettiB sAnim_BlueConfettiC sAnim_YellowConfettiB sAnim_WhiteConfettiB sAnim_GreenConfettiB sAnim_PinkConfettiC sAnim_RedConfettiC sAnim_YellowConfettiC sAnim_WhiteConfettiC sAnims_Confetti sSpriteTemplate_HofConfetti sHallOfFame_Pal sHallOfFame_Gfx sDummyFameMon sHallOfFame_SlotOrder

/// `struct HallofFameTeam`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HallofFameTeam {
    pub mon: CArray<HallofFameMon, 6>,
}

unsafe impl Sync for HallofFameTeam {}

/// `struct HofGfx`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct HofGfx {
    pub state: u16,
    pub field_2: CArray<u8, 16>,
    pub tilemap1: CArray<u8, 4096>,
    pub tilemap2: CArray<u8, 4096>,
}

unsafe impl Sync for HofGfx {}

/// `struct HallofFameMon`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HallofFameMon {
    pub tid: u32,
    pub personality: u32,
    bits_8: u16,
    pub nickname: CArray<u8, 10>,
}

impl HallofFameMon {
    #[inline(always)]
    pub fn species(&self) -> u16 {
        ((self.bits_8 as u32) & 0x1ff) as u16
    }
    #[inline(always)]
    pub fn set_species(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !0x1ff) | (v & 0x1ff);
    }
    #[inline(always)]
    pub fn lvl(&self) -> u16 {
        ((self.bits_8 as u32 >> 9) & 0x7f) as u16
    }
    #[inline(always)]
    pub fn set_lvl(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x7f << 9)) | ((v & 0x7f) << 9);
    }
}

unsafe impl Sync for HallofFameMon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<HallofFameTeam>() == 120);
    assert!(offset_of!(HallofFameTeam, mon) == 0);
    assert!(size_of::<HofGfx>() == 8212);
    assert!(offset_of!(HofGfx, state) == 0);
    assert!(offset_of!(HofGfx, field_2) == 2);
    assert!(offset_of!(HofGfx, tilemap1) == 18);
    assert!(offset_of!(HofGfx, tilemap2) == 4114);
    assert!(size_of::<HallofFameMon>() == 20);
    assert!(offset_of!(HallofFameMon, tid) == 0);
    assert!(offset_of!(HallofFameMon, personality) == 4);
    assert!(offset_of!(HallofFameMon, bits_8) == 8);
    assert!(offset_of!(HallofFameMon, nickname) == 10);
};

const CONFETTI_EXTRA_Y: i32 = 1;
const CONFETTI_SINE_IDX: i32 = 0;
const CONFETTI_TASK_ID: u8 = 7;
const HALL_OF_FAME_MAX_TEAMS: u16 = 50;
const TAG_CONFETTI: u16 = 1001;

static sAnims_Confetti: Table<CArray<*mut AnimCmd, 17>> =
    Table((&raw const crate::data::hall_of_fame::sAnims_Confetti).cast());
static sDummyFameMon: Table<HallofFameMon> =
    Table((&raw const crate::data::hall_of_fame::sDummyFameMon).cast());
static sHallOfFame_Gfx: Table<CArray<u32, 115>> =
    Table((&raw const crate::data::hall_of_fame::sHallOfFame_Gfx).cast());
static sHallOfFame_MonFullTeamPositions: Table<CArray<CArray<i16, 4>, 6>> =
    Table((&raw const crate::data::hall_of_fame::sHallOfFame_MonFullTeamPositions).cast());
static sHallOfFame_MonHalfTeamPositions: Table<CArray<CArray<i16, 4>, 3>> =
    Table((&raw const crate::data::hall_of_fame::sHallOfFame_MonHalfTeamPositions).cast());
static sHallOfFame_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::hall_of_fame::sHallOfFame_Pal).cast());
static sHof_BgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::hall_of_fame::sHof_BgTemplates).cast());
static sHof_WindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::hall_of_fame::sHof_WindowTemplate).cast());
static sMonInfoTextColors: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::hall_of_fame::sMonInfoTextColors).cast());
static sOamData_Confetti: Table<OamData> =
    Table((&raw const crate::data::hall_of_fame::sOamData_Confetti).cast());
static sPlayerInfoTextColors: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::hall_of_fame::sPlayerInfoTextColors).cast());
static sSpritePalette_Confetti: Table<CArray<CompressedSpritePalette, 2>> =
    Table((&raw const crate::data::hall_of_fame::sSpritePalette_Confetti).cast());
static sSpriteSheet_Confetti: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::hall_of_fame::sSpriteSheet_Confetti).cast());
static sSpriteTemplate_HofConfetti: Table<SpriteTemplate> =
    Table((&raw const crate::data::hall_of_fame::sSpriteTemplate_HofConfetti).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sHofFadePalettes: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofMonPtr: *mut HallofFameTeam = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofGfxPtr: *mut HofGfx = null_mut();

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `ConfettiUtil_SetCallback` with this module's view of its types.
#[inline]
unsafe fn ConfettiUtil_SetCallback(a0: u8, a1: Option<unsafe fn(*mut ConfettiUtil)>) -> u8 {
    unsafe { crate::confetti_util::ConfettiUtil_SetCallback(a0, core::mem::transmute(a1)) }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
unsafe fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn VBlankCB_HallOfFame() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_HallOfFame() {
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn InitHallOfFameScreen() -> u8 {
    match gMain.state {
        0 => {
            SetVBlankCallback(None);
            ClearVramOamPltt_LoadHofPal();
            sHofGfxPtr = AllocZeroed(8212) as *mut HofGfx;
            gMain.state = 1;
        }
        1 => {
            LoadHofGfx();
            gMain.state += 1;
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1808);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            InitHofBgs();
            (*sHofGfxPtr).state = 0;
            gMain.state += 1;
        }
        3 => {
            if LoadHofBgs() == 0 {
                SetVBlankCallback(Some(VBlankCB_HallOfFame));
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
                gMain.state += 1;
            }
        }
        4 => {
            UpdatePaletteFade();
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_HallOfFame));
                PlayBGM(MUS_HALL_OF_FAME);
                return FALSE;
            }
        }
        _ => {}
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_DoHallOfFameScreen() {
    if InitHallOfFameScreen() == 0 {
        let taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0);
        task_set(taskId, tDontSaveData, FALSE as i16);
        sHofMonPtr = AllocZeroed(120) as *mut HallofFameTeam;
    }
}
pub unsafe fn CB2_DoHallOfFameScreenDontSaveData() {
    if InitHallOfFameScreen() == 0 {
        let taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0);
        task_set(taskId, tDontSaveData, TRUE as i16);
        sHofMonPtr = AllocZeroed(120) as *mut HallofFameTeam;
    }
}
pub(crate) unsafe fn Task_Hof_InitMonData(taskId: u8) {
    task_set(taskId, tMonNumber, 0);
    let mut i: u16 = 0;
    while i < PARTY_SIZE as u16 {
        let mut nickname: CArray<u8, 11> = zeroed();
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != 0 {
            (*sHofMonPtr).mon[i]
                .set_species(GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16);
            (*sHofMonPtr).mon[i].tid = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_OT_ID);
            (*sHofMonPtr).mon[i].personality =
                GetMonData2(&raw mut gPlayerParty[i], MON_DATA_PERSONALITY);
            (*sHofMonPtr).mon[i]
                .set_lvl(GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u16);
            GetMonData3(
                &raw mut gPlayerParty[i],
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
            for j in 0..(POKEMON_NAME_LENGTH as u16) {
                (*sHofMonPtr).mon[i].nickname[j] = nickname[j];
            }
            task_set(taskId, tMonNumber, task_get(taskId, tMonNumber) + 1);
        } else {
            (*sHofMonPtr).mon[i].set_species(SPECIES_NONE);
            (*sHofMonPtr).mon[i].tid = 0;
            (*sHofMonPtr).mon[i].personality = 0;
            (*sHofMonPtr).mon[i].set_lvl(0);
            (*sHofMonPtr).mon[i].nickname[0] = EOS;
        }
        i += 1;
    }
    sHofFadePalettes.set(0);
    task_set(taskId, tDisplayedMonId, 0);
    task_set(taskId, tPlayerSpriteID, SPRITE_NONE as i16);
    for i in 0..(PARTY_SIZE as u16) {
        task_set(taskId, i as i32 + 5, SPRITE_NONE as i16);
    }
    if task_get(taskId, tDontSaveData) != 0 {
        task_set_func(taskId, Some(Task_Hof_SetMonDisplayTask));
    } else {
        task_set_func(taskId, Some(Task_Hof_InitTeamSaveData));
    }
}
pub(crate) unsafe fn Task_Hof_InitTeamSaveData(taskId: u8) {
    let mut lastSavedTeam: *mut HallofFameTeam =
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut HallofFameTeam;
    if gHasHallOfFameRecords == 0 {
        memset(
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr(),
            0,
            8192,
        );
    } else {
        if LoadGameSave(SAVE_HALL_OF_FAME) != SAVE_STATUS_OK {
            memset(
                (*(&raw const crate::decompress::gDecompressionBuffer)
                    .cast::<CArray<u8, 16384>>()
                    .cast_mut())
                .as_mut_ptr(),
                0,
                8192,
            );
        }
    }
    let mut i: u16 = 0;
    while i < HALL_OF_FAME_MAX_TEAMS {
        if (*lastSavedTeam).mon[0].species() == 0 {
            break;
        }
        i += 1;
        lastSavedTeam = lastSavedTeam.at(1);
    }
    if i >= HALL_OF_FAME_MAX_TEAMS {
        let mut afterTeam: *mut HallofFameTeam =
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut HallofFameTeam;
        let mut beforeTeam: *mut HallofFameTeam =
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut HallofFameTeam;
        afterTeam = afterTeam.at(1);
        i = 0;
        while i < 49 {
            *beforeTeam = *afterTeam;
            i += 1;
            beforeTeam = beforeTeam.at(1);
            afterTeam = afterTeam.at(1);
        }
        lastSavedTeam = lastSavedTeam.at(-1);
    }
    *lastSavedTeam = *sHofMonPtr;
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*crate::asmdata::gText_SavingDontTurnOffPower.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(0, COPYWIN_FULL);
    task_set_func(taskId, Some(Task_Hof_TrySaveData));
}
pub(crate) unsafe fn Task_Hof_TrySaveData(taskId: u8) {
    gGameContinueCallback = Some(CB2_DoHallOfFameScreenDontSaveData);
    if TrySavingData(SAVE_HALL_OF_FAME) == SAVE_STATUS_ERROR && gDamagedSaveSectors.get() != 0 {
        UnsetBgTilemapBuffer(1);
        UnsetBgTilemapBuffer(3);
        FreeAllWindowBuffers();
        if !sHofGfxPtr.is_null() {
            Free(sHofGfxPtr as *mut c_void);
            sHofGfxPtr = null_mut();
        }
        if !sHofMonPtr.is_null() {
            Free(sHofMonPtr as *mut c_void);
            sHofMonPtr = null_mut();
        }
        DestroyTask(taskId);
    } else {
        PlaySE(SE_SAVE);
        task_set_func(taskId, Some(Task_Hof_WaitToDisplayMon));
        task_set(taskId, tFrameCount, 32);
    }
}
pub(crate) fn Task_Hof_WaitToDisplayMon(taskId: u8) {
    if task_get(taskId, tFrameCount) != 0 {
        task_set(taskId, tFrameCount, task_get(taskId, tFrameCount) - 1);
    } else {
        task_set_func(taskId, Some(Task_Hof_SetMonDisplayTask));
    }
}
pub(crate) fn Task_Hof_SetMonDisplayTask(taskId: u8) {
    task_set_func(taskId, Some(Task_Hof_DisplayMon));
}
pub(crate) unsafe fn Task_Hof_DisplayMon(taskId: u8) {
    let mut startX: i16 = 0;
    let mut startY: i16 = 0;
    let mut destX: i16 = 0;
    let mut destY: i16 = 0;
    let currMonId: u16 = task_get(taskId, 1) as u16;
    let currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currMonId];
    if task_get(taskId, 2) > 3 {
        startX = sHallOfFame_MonFullTeamPositions[currMonId][0];
        startY = sHallOfFame_MonFullTeamPositions[currMonId][1];
        destX = sHallOfFame_MonFullTeamPositions[currMonId][2];
        destY = sHallOfFame_MonFullTeamPositions[currMonId][3];
    } else {
        startX = sHallOfFame_MonHalfTeamPositions[currMonId][0];
        startY = sHallOfFame_MonHalfTeamPositions[currMonId][1];
        destX = sHallOfFame_MonHalfTeamPositions[currMonId][2];
        destY = sHallOfFame_MonHalfTeamPositions[currMonId][3];
    }
    if (*currMon).species() == SPECIES_EGG as u16 {
        destY += 10;
    }
    let spriteId: u8 = CreateMonPicSprite_Affine(
        (*currMon).species(),
        (*currMon).tid,
        (*currMon).personality,
        MON_PIC_AFFINE_FRONT,
        startX,
        startY,
        currMonId as u8,
        TAG_NONE,
    ) as u8;
    gSprites[spriteId].data[1] = destX;
    gSprites[spriteId].data[2] = destY;
    gSprites[spriteId].data[0] = 0;
    gSprites[spriteId].data[tSpecies] = (*currMon).species() as i16;
    gSprites[spriteId].callback = Some(SpriteCB_GetOnScreenAndAnimate);
    task_set(taskId, currMonId as i32 + 5, spriteId as i16);
    ClearDialogWindowAndFrame(0, TRUE);
    task_set_func(taskId, Some(Task_Hof_PrintMonInfoAfterAnimating));
}
pub(crate) unsafe fn Task_Hof_PrintMonInfoAfterAnimating(taskId: u8) {
    let currMonId: u16 = task_get(taskId, tDisplayedMonId) as u16;
    let currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currMonId];
    let monSprite: *mut Sprite = &raw mut gSprites[task_get(taskId, currMonId as i32 + 5)];
    if (*monSprite).callback == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite)) {
        (*monSprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
        HallOfFame_PrintMonInfo(currMon, 0, 14);
        task_set(taskId, tFrameCount, 120);
        task_set_func(taskId, Some(Task_Hof_TryDisplayAnotherMon));
    }
}
pub(crate) unsafe fn Task_Hof_TryDisplayAnotherMon(taskId: u8) {
    let currPokeID: u16 = task_get(taskId, tDisplayedMonId) as u16;
    let currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currPokeID];
    if task_get(taskId, tFrameCount) != 0 {
        task_set(taskId, tFrameCount, task_get(taskId, tFrameCount) - 1);
    } else {
        {
            let rhs = shl_i32(
                0x10000,
                gSprites[task_get(taskId, currPokeID as i32 + 5)]
                    .oam
                    .paletteNum() as u32,
            ) as u32;
            sHofFadePalettes.set(sHofFadePalettes.get() | rhs)
        };
        if task_get(taskId, tDisplayedMonId) < 5 && (*currMon.at(1)).species() != SPECIES_NONE {
            task_set(
                taskId,
                tDisplayedMonId,
                task_get(taskId, tDisplayedMonId) + 1,
            );
            BeginNormalPaletteFade(sHofFadePalettes.get(), 0, 12, 12, 25520);
            gSprites[task_get(taskId, currPokeID as i32 + 5)]
                .oam
                .set_priority(1);
            task_set_func(taskId, Some(Task_Hof_DisplayMon));
        } else {
            task_set_func(taskId, Some(Task_Hof_PaletteFadeAndPrintWelcomeText));
        }
    }
}
pub(crate) unsafe fn Task_Hof_PaletteFadeAndPrintWelcomeText(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_OBJECTS, 0, 0, 0, 0);
    for i in 0..(PARTY_SIZE as u16) {
        if task_get(taskId, i as i32 + 5) != SPRITE_NONE as i16 {
            gSprites[task_get(taskId, i as i32 + 5)].oam.set_priority(0);
        }
    }
    HallOfFame_PrintWelcomeText(0, 15);
    PlaySE(SE_APPLAUSE);
    task_set(taskId, tFrameCount, 400);
    task_set_func(taskId, Some(Task_Hof_DoConfetti));
}
pub(crate) unsafe fn Task_Hof_DoConfetti(taskId: u8) {
    if task_get(taskId, tFrameCount) != 0 {
        task_set(taskId, tFrameCount, task_get(taskId, tFrameCount) - 1);
        if task_get(taskId, tFrameCount) as i32 & 3 == 0 && task_get(taskId, tFrameCount) > 110 {
            CreateHofConfettiSprite();
        }
    } else {
        for i in 0..(PARTY_SIZE as u16) {
            if task_get(taskId, i as i32 + 5) != SPRITE_NONE as i16 {
                gSprites[task_get(taskId, i as i32 + 5)].oam.set_priority(1);
            }
        }
        BeginNormalPaletteFade(sHofFadePalettes.get(), 0, 12, 12, 25520);
        FillWindowPixelBuffer(0, 0);
        CopyWindowToVram(0, COPYWIN_FULL);
        task_set(taskId, tFrameCount, 7);
        task_set_func(taskId, Some(Task_Hof_WaitToDisplayPlayer));
    }
}
pub(crate) unsafe fn Task_Hof_WaitToDisplayPlayer(taskId: u8) {
    if task_get(taskId, tFrameCount) >= 16 {
        task_set_func(taskId, Some(Task_Hof_DisplayPlayer));
    } else {
        task_set(taskId, tFrameCount, task_get(taskId, tFrameCount) + 1);
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            task_get(taskId, tFrameCount) as u16 * 256,
        );
    }
}
pub(crate) unsafe fn Task_Hof_DisplayPlayer(taskId: u8) {
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(3);
    task_set(
        taskId,
        tPlayerSpriteID,
        CreateTrainerPicSprite(
            PlayerGenderToFrontTrainerPicId_Debug((*gSaveBlock2Ptr).playerGender, TRUE),
            TRUE,
            120,
            72,
            6,
            TAG_NONE,
        ) as i16,
    );
    AddWindow((&raw const *sHof_WindowTemplate).cast_mut());
    LoadWindowGfx(
        1,
        (*gSaveBlock2Ptr).optionsWindowFrameType() as u8,
        0x21D,
        208,
    );
    LoadPalette(GetTextWindowPalette(1) as *mut c_void, 224, 32);
    task_set(taskId, tFrameCount, 120);
    task_set_func(taskId, Some(Task_Hof_WaitAndPrintPlayerInfo));
}
pub(crate) unsafe fn Task_Hof_WaitAndPrintPlayerInfo(taskId: u8) {
    if task_get(taskId, tFrameCount) != 0 {
        task_set(taskId, tFrameCount, task_get(taskId, tFrameCount) - 1);
    } else if gSprites[task_get(taskId, tPlayerSpriteID)].x != 192 {
        gSprites[task_get(taskId, tPlayerSpriteID)].x += 1;
    } else {
        FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 0x20, 0x20);
        HallOfFame_PrintPlayerInfo(1, 2);
        DrawDialogueFrame(0, 0);
        AddTextPrinterParameterized2(
            0,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_LeagueChamp).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            None,
            TEXT_COLOR_DARK_GRAY,
            TEXT_COLOR_WHITE,
            TEXT_COLOR_LIGHT_GRAY,
        );
        CopyWindowToVram(0, COPYWIN_FULL);
        task_set_func(taskId, Some(Task_Hof_ExitOnKeyPressed));
    }
}
pub(crate) unsafe fn Task_Hof_ExitOnKeyPressed(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        FadeOutBGM(4);
        task_set_func(taskId, Some(Task_Hof_HandlePaletteOnExit));
    }
}
pub(crate) unsafe fn Task_Hof_HandlePaletteOnExit(taskId: u8) {
    CpuSet(
        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
        gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
        512,
    );
    BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 0x10, 0);
    task_set_func(taskId, Some(Task_Hof_HandleExit));
}
pub(crate) unsafe fn Task_Hof_HandleExit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        for i in 0..PARTY_SIZE {
            let spriteId: u8 = task_get(taskId, i + 5) as u8;
            if spriteId != SPRITE_NONE {
                FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
                FreeAndDestroyMonPicSprite(spriteId as u16);
            }
        }
        FreeAndDestroyTrainerPicSprite(task_get(taskId, tPlayerSpriteID) as u16);
        HideBg(0);
        HideBg(1);
        HideBg(3);
        FreeAllWindowBuffers();
        UnsetBgTilemapBuffer(1);
        UnsetBgTilemapBuffer(3);
        ResetBgsAndClearDma3BusyFlags(0);
        DestroyTask(taskId);
        if !sHofGfxPtr.is_null() {
            Free(sHofGfxPtr as *mut c_void);
            sHofGfxPtr = null_mut();
        }
        if !sHofMonPtr.is_null() {
            Free(sHofMonPtr as *mut c_void);
            sHofMonPtr = null_mut();
        }
        StartCredits();
    }
}
unsafe fn StartCredits() {
    SetMainCallback2(Some(CB2_StartCreditsSequence));
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_DoHallOfFamePC() {
    match gMain.state {
        1 => {
            LoadHofGfx();
            gMain.state += 1;
        }
        2 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            InitHofBgs();
            gMain.state += 1;
        }
        3 => {
            if LoadHofBgs() == 0 {
                let fameTeam: *mut HallofFameTeam =
                    (*(&raw const crate::decompress::gDecompressionBuffer)
                        .cast::<CArray<u8, 16384>>()
                        .cast_mut())
                    .as_mut_ptr() as *mut HallofFameTeam;
                (*fameTeam).mon[0] = *sDummyFameMon;
                ComputerScreenOpenEffect(0, 0, 0);
                SetVBlankCallback(Some(VBlankCB_HallOfFame));
                gMain.state += 1;
            }
        }
        4 => {
            RunTasks();
            AnimateSprites();
            BuildOamBuffer();
            UpdatePaletteFade();
            if IsComputerScreenOpenEffectActive() == 0 {
                gMain.state += 1;
            }
        }
        5 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1808);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            let taskId: u8 = CreateTask(Some(Task_HofPC_CopySaveData), 0);
            for i in 0..(PARTY_SIZE as u8) {
                task_set(taskId, i as i32 + 5, SPRITE_NONE as i16);
            }
            sHofMonPtr = AllocZeroed(8192) as *mut HallofFameTeam;
            SetMainCallback2(Some(CB2_HallOfFame));
        }
        _ => {
            SetVBlankCallback(None);
            ClearVramOamPltt_LoadHofPal();
            sHofGfxPtr = AllocZeroed(8212) as *mut HofGfx;
            gMain.state = 1;
        }
    }
}
pub(crate) unsafe fn Task_HofPC_CopySaveData(taskId: u8) {
    HofPCTopBar_AddWindow(0, 30, 0, 12, 0x226);
    if LoadGameSave(SAVE_HALL_OF_FAME) != SAVE_STATUS_OK {
        task_set_func(taskId, Some(Task_HofPC_PrintDataIsCorrupted));
    } else {
        CpuSet(
            (*(&raw const crate::decompress::gDecompressionBuffer)
                .cast::<CArray<u8, 16384>>()
                .cast_mut())
            .as_mut_ptr() as *mut c_void,
            sHofMonPtr as *mut c_void,
            SECTOR_SIZE,
        );
        let mut savedTeams: *mut HallofFameTeam = sHofMonPtr;
        let mut i: u16 = 0;
        while i < HALL_OF_FAME_MAX_TEAMS {
            if (*savedTeams).mon[0].species() == 0 {
                break;
            }
            i += 1;
            savedTeams = savedTeams.at(1);
        }
        if i < HALL_OF_FAME_MAX_TEAMS {
            task_set(taskId, tCurrTeamNo, i as i16 - 1);
        } else {
            task_set(taskId, tCurrTeamNo, 49);
        }
        task_set(
            taskId,
            tCurrPageNo,
            GetGameStat(GAME_STAT_ENTERED_HOF) as i16,
        );
        task_set_func(taskId, Some(Task_HofPC_DrawSpritesPrintText));
    }
}
pub(crate) unsafe fn Task_HofPC_DrawSpritesPrintText(taskId: u8) {
    let mut savedTeams: *mut HallofFameTeam = sHofMonPtr;
    let mut i: u16 = 0;
    while (i as i32) < task_get(taskId, tCurrTeamNo) as i32 {
        savedTeams = savedTeams.at(1);
        i += 1;
    }
    let mut currMon: *mut HallofFameMon = &raw mut (*savedTeams).mon[0];
    sHofFadePalettes.set(0);
    task_set(taskId, tCurrMonId, 0);
    task_set(taskId, tMonNo, 0);
    i = 0;
    while i < PARTY_SIZE as u16 {
        if (*currMon).species() != 0 {
            task_set(taskId, tMonNo, task_get(taskId, tMonNo) + 1);
        }
        i += 1;
        currMon = currMon.at(1);
    }
    currMon = &raw mut (*savedTeams).mon[0];
    i = 0;
    while i < PARTY_SIZE as u16 {
        if (*currMon).species() != 0 {
            let mut posX: i16 = 0;
            let mut posY: i16 = 0;
            if task_get(taskId, tMonNo) > 3 {
                posX = sHallOfFame_MonFullTeamPositions[i][2];
                posY = sHallOfFame_MonFullTeamPositions[i][3];
            } else {
                posX = sHallOfFame_MonHalfTeamPositions[i][2];
                posY = sHallOfFame_MonHalfTeamPositions[i][3];
            }
            if (*currMon).species() == SPECIES_EGG as u16 {
                posY += 10;
            }
            let spriteId: u16 = CreateMonPicSprite_HandleDeoxys(
                (*currMon).species(),
                (*currMon).tid,
                (*currMon).personality,
                TRUE,
                posX,
                posY,
                i as u8,
                TAG_NONE,
            );
            gSprites[spriteId].oam.set_priority(1);
            task_set(taskId, i as i32 + 5, spriteId as i16);
        } else {
            task_set(taskId, i as i32 + 5, SPRITE_NONE as i16);
        }
        i += 1;
        currMon = currMon.at(1);
    }
    BlendPalettes(PALETTES_OBJECTS, 0xC, 25520);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        task_get(taskId, tCurrPageNo) as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_HOFNumber).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if task_get(taskId, tCurrTeamNo) <= 0 {
        HofPCTopBar_PrintPair(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PickCancel).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            0,
            TRUE,
        );
    } else {
        HofPCTopBar_PrintPair(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PickNextCancel).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            0,
            TRUE,
        );
    }
    task_set_func(taskId, Some(Task_HofPC_PrintMonInfo));
}
pub(crate) unsafe fn Task_HofPC_PrintMonInfo(taskId: u8) {
    let mut savedTeams: *mut HallofFameTeam = sHofMonPtr;
    let mut i: u16 = 0;
    while (i as i32) < task_get(taskId, tCurrTeamNo) as i32 {
        savedTeams = savedTeams.at(1);
        i += 1;
    }
    for i in 0..(PARTY_SIZE as u16) {
        let spriteId: u16 = task_get(taskId, i as i32 + 5) as u16;
        if spriteId != SPRITE_NONE as u16 {
            gSprites[spriteId].oam.set_priority(1);
        }
    }
    let currMonID: u16 =
        (*gTasks.as_ptr())[taskId].data[task_get(taskId, tCurrMonId) as i32 + 5] as u16;
    gSprites[currMonID].oam.set_priority(0);
    sHofFadePalettes.set(
        shl_i32(0x10000, gSprites[currMonID].oam.paletteNum() as u32) as u32 ^ PALETTES_OBJECTS,
    );
    BlendPalettesUnfaded(sHofFadePalettes.get(), 0xC, 25520);
    let currMon: *mut HallofFameMon = &raw mut (*savedTeams).mon[task_get(taskId, tCurrMonId)];
    if (*currMon).species() != SPECIES_EGG as u16 {
        StopCryAndClearCrySongs();
        PlayCry_Normal((*currMon).species(), 0);
    }
    HallOfFame_PrintMonInfo(currMon, 0, 14);
    task_set_func(taskId, Some(Task_HofPC_HandleInput));
}
pub(crate) unsafe fn Task_HofPC_HandleInput(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if task_get(taskId, tCurrTeamNo) != 0 {
            task_set(taskId, tCurrTeamNo, task_get(taskId, tCurrTeamNo) - 1);
            for i in 0..(PARTY_SIZE as u16) {
                let spriteId: u8 = task_get(taskId, i as i32 + 5) as u8;
                if spriteId != SPRITE_NONE {
                    FreeAndDestroyMonPicSprite(spriteId as u16);
                    task_set(taskId, i as i32 + 5, SPRITE_NONE as i16);
                }
            }
            if task_get(taskId, tCurrPageNo) != 0 {
                task_set(taskId, tCurrPageNo, task_get(taskId, tCurrPageNo) - 1);
            }
            task_set_func(taskId, Some(Task_HofPC_DrawSpritesPrintText));
        } else {
            if IsCryPlayingOrClearCrySongs() != 0 {
                StopCryAndClearCrySongs();
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            }
            task_set_func(taskId, Some(Task_HofPC_HandlePaletteOnExit));
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        if IsCryPlayingOrClearCrySongs() != 0 {
            StopCryAndClearCrySongs();
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        }
        task_set_func(taskId, Some(Task_HofPC_HandlePaletteOnExit));
    } else if gMain.newKeys as i32 & DPAD_UP != 0 && task_get(taskId, tCurrMonId) != 0 {
        task_set(taskId, tCurrMonId, task_get(taskId, tCurrMonId) - 1);
        task_set_func(taskId, Some(Task_HofPC_PrintMonInfo));
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (task_get(taskId, tCurrMonId) as i32) < task_get(taskId, tMonNo) as i32 - 1
    {
        task_set(taskId, tCurrMonId, task_get(taskId, tCurrMonId) + 1);
        task_set_func(taskId, Some(Task_HofPC_PrintMonInfo));
    }
}
pub(crate) unsafe fn Task_HofPC_HandlePaletteOnExit(taskId: u8) {
    CpuSet(
        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
        gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
        512,
    );
    let fameTeam: *mut HallofFameTeam = (*(&raw const crate::decompress::gDecompressionBuffer)
        .cast::<CArray<u8, 16384>>()
        .cast_mut())
    .as_mut_ptr() as *mut HallofFameTeam;
    (*fameTeam).mon[0] = *sDummyFameMon;
    ComputerScreenCloseEffect(0, 0, 0);
    task_set_func(taskId, Some(Task_HofPC_HandleExit));
}
pub(crate) unsafe fn Task_HofPC_HandleExit(taskId: u8) {
    if IsComputerScreenCloseEffectActive() == 0 {
        for i in 0..(PARTY_SIZE as u8) {
            let spriteId: u16 = task_get(taskId, i as i32 + 5) as u16;
            if spriteId != SPRITE_NONE as u16 {
                FreeAndDestroyMonPicSprite(spriteId);
                task_set(taskId, i as i32 + 5, SPRITE_NONE as i16);
            }
        }
        HideBg(0);
        HideBg(1);
        HideBg(3);
        HofPCTopBar_RemoveWindow();
        FreeAllWindowBuffers();
        UnsetBgTilemapBuffer(1);
        UnsetBgTilemapBuffer(3);
        ResetBgsAndClearDma3BusyFlags(0);
        DestroyTask(taskId);
        if !sHofGfxPtr.is_null() {
            Free(sHofGfxPtr as *mut c_void);
            sHofGfxPtr = null_mut();
        }
        if !sHofMonPtr.is_null() {
            Free(sHofMonPtr as *mut c_void);
            sHofMonPtr = null_mut();
        }
        ReturnFromHallOfFamePC();
    }
}
pub(crate) unsafe fn Task_HofPC_PrintDataIsCorrupted(taskId: u8) {
    HofPCTopBar_Print(
        (*(&raw const crate::data::strings::gText_AButtonExit).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        TRUE,
    );
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_HOFCorrupted).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(0, COPYWIN_FULL);
    task_set_func(taskId, Some(Task_HofPC_ExitOnButtonPress));
}
pub(crate) unsafe fn Task_HofPC_ExitOnButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        task_set_func(taskId, Some(Task_HofPC_HandlePaletteOnExit));
    }
}
unsafe fn HallOfFame_PrintWelcomeText(unusedPossiblyWindowId: u8, unused2: u8) {
    FillWindowPixelBuffer(0, 0);
    PutWindowTilemap(0);
    AddTextPrinterParameterized3(
        0,
        FONT_NORMAL,
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_WelcomeToHOF).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0xD0,
        ) as u8,
        1,
        sMonInfoTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_WelcomeToHOF).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(0, COPYWIN_FULL);
}
unsafe fn HallOfFame_PrintMonInfo(currMon: *mut HallofFameMon, unused1: u8, unused2: u8) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut stringPtr: *mut u8 = null_mut();
    let mut dexNumber: i32 = 0;
    let mut width: i32 = 0;
    FillWindowPixelBuffer(0, 0);
    PutWindowTilemap(0);
    if (*currMon).species() != SPECIES_EGG as u16 {
        stringPtr = StringCopy(
            text.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Number).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        dexNumber = SpeciesToPokedexNum((*currMon).species()) as i32;
        if dexNumber != 0xFFFF {
            *stringPtr = (dexNumber / 100) as u8 + CHAR_0;
            stringPtr = stringPtr.at(1);
            dexNumber %= 100;
            *stringPtr = (dexNumber / 10) as u8 + CHAR_0;
            stringPtr = stringPtr.at(1);
            *stringPtr = (dexNumber % 10) as u8 + CHAR_0;
            stringPtr = stringPtr.at(1);
        } else {
            *({
                let t1 = stringPtr;
                stringPtr = stringPtr.at(1);
                t1
            }) = CHAR_QUESTION_MARK;
            *({
                let t2 = stringPtr;
                stringPtr = stringPtr.at(1);
                t2
            }) = CHAR_QUESTION_MARK;
            *({
                let t3 = stringPtr;
                stringPtr = stringPtr.at(1);
                t3
            }) = CHAR_QUESTION_MARK;
        }
        *stringPtr = EOS;
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            0x10,
            1,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
    }
    memcpy(
        text.as_mut_ptr(),
        (*currMon).nickname.as_mut_ptr(),
        POKEMON_NAME_LENGTH,
    );
    text[10] = EOS;
    if (*currMon).species() == SPECIES_EGG as u16 {
        width = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 0xD0);
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            width as u8,
            1,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        CopyWindowToVram(0, COPYWIN_FULL);
    } else {
        width = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 0x80);
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            width as u8,
            1,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        text[0] = CHAR_SLASH;
        stringPtr = StringCopy(
            text.as_mut_ptr().at(1),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*currMon).species()]
            .as_ptr()
            .cast_mut(),
        );
        if (*currMon).species() != SPECIES_NIDORAN_M && (*currMon).species() != SPECIES_NIDORAN_F {
            match GetGenderFromSpeciesAndPersonality((*currMon).species(), (*currMon).personality) {
                MON_MALE => {
                    *stringPtr = CHAR_MALE;
                    stringPtr = stringPtr.at(1);
                }
                MON_FEMALE => {
                    *stringPtr = CHAR_FEMALE;
                    stringPtr = stringPtr.at(1);
                }
                _ => {}
            }
        }
        *stringPtr = EOS;
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            0x80,
            1,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        stringPtr = StringCopy(
            text.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Level).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        ConvertIntToDecimalStringN(
            stringPtr,
            (*currMon).lvl() as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            0x24,
            0x11,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        stringPtr = StringCopy(
            text.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_IDNumber).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        ConvertIntToDecimalStringN(
            stringPtr,
            (*currMon).tid as u16 as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            5,
        );
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            0x68,
            0x11,
            sMonInfoTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
        CopyWindowToVram(0, COPYWIN_FULL);
    }
}
unsafe fn HallOfFame_PrintPlayerInfo(unused1: u8, unused2: u8) {
    let mut text: CArray<u8, 20> = zeroed();
    FillWindowPixelBuffer(1, 17);
    PutWindowTilemap(1);
    DrawStdFrameWithCustomTileAndPalette(1, FALSE, 0x21D, 0xD);
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        0,
        1,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        (*(&raw const crate::data::strings::gText_Name).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let mut width: u32 = GetStringRightAlignXOffset(
        FONT_NORMAL as i32,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        0x70,
    ) as u32;
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        width as u8,
        1,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    let trainerId: u16 = (*gSaveBlock2Ptr).playerTrainerId[0] as u16
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u16) << 8;
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        0,
        0x11,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_IDNumber).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    text[0] = (trainerId as i32 % 100000 / 10000) as u8 + CHAR_0;
    text[1] = (trainerId as i32 % 10000 / 1000) as u8 + CHAR_0;
    text[2] = (trainerId as i32 % 1000 / 100) as u8 + CHAR_0;
    text[3] = (trainerId as i32 % 100 / 10) as u8 + CHAR_0;
    text[4] = (trainerId as i32 % 10) as u8 + CHAR_0;
    text[5] = EOS;
    width = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 0x70) as u32;
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        width as u8,
        0x11,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        text.as_mut_ptr(),
    );
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        0,
        0x21,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        (*(&raw const crate::data::strings::gText_Time).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    text[0] = ((*gSaveBlock2Ptr).playTimeHours as i32 / 100) as u8 + CHAR_0;
    text[1] = ((*gSaveBlock2Ptr).playTimeHours as i32 % 100 / 10) as u8 + CHAR_0;
    text[2] = ((*gSaveBlock2Ptr).playTimeHours as i32 % 10) as u8 + CHAR_0;
    if text[0] == CHAR_0 {
        text[0] = 0x00;
    }
    if text[0] == 0x00 && text[1] == CHAR_0 {
        text[8] = CHAR_SPACE;
    }
    text[3] = CHAR_COLON;
    text[4] = ((*gSaveBlock2Ptr).playTimeMinutes as i32 % 100 / 10) as u8 + CHAR_0;
    text[5] = ((*gSaveBlock2Ptr).playTimeMinutes as i32 % 10) as u8 + CHAR_0;
    text[6] = EOS;
    width = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 0x70) as u32;
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        width as u8,
        0x21,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        text.as_mut_ptr(),
    );
    CopyWindowToVram(1, COPYWIN_FULL);
}
unsafe fn ClearVramOamPltt_LoadHofPal() {
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        {
            let mut _dest: *mut u32 = OAM as i32 as usize as *mut c_void as *mut u32;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut c_void as *mut u16;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    ResetPaletteFade();
    LoadPalette(sHallOfFame_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
}
unsafe fn LoadHofGfx() {
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    ResetTempTileDataBuffers();
    ResetAllPicSprites();
    FreeAllSpritePalettes();
    gReservedSpritePaletteCount = 8;
    LoadCompressedSpriteSheet(sSpriteSheet_Confetti.as_ptr().cast_mut());
    LoadCompressedSpritePalette(sSpritePalette_Confetti.as_ptr().cast_mut());
}
unsafe fn InitHofBgs() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sHof_BgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(1, (*sHofGfxPtr).tilemap1.as_mut_ptr() as *mut c_void);
    SetBgTilemapBuffer(3, (*sHofGfxPtr).tilemap2.as_mut_ptr() as *mut c_void);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
unsafe fn LoadHofBgs() -> u8 {
    match (*sHofGfxPtr).state {
        0 => {
            DecompressAndCopyTileDataToVram(
                1,
                sHallOfFame_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return TRUE;
            }
        }
        2 => {
            FillBgTilemapBufferRect_Palette0(1, 1, 0, 0, 0x20, 2);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 3, 0x20, 0xB);
            FillBgTilemapBufferRect_Palette0(1, 1, 0, 0xE, 0x20, 6);
            FillBgTilemapBufferRect_Palette0(3, 2, 0, 0, 0x20, 0x20);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(3);
        }
        3 => {
            InitStandardTextBoxWindows();
            InitTextBoxGfxAndPrinters();
        }
        4 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(3);
            (*sHofGfxPtr).state = 0;
            return FALSE;
        }
        _ => {}
    }
    (*sHofGfxPtr).state += 1;
    TRUE
}
pub(crate) unsafe fn SpriteCB_GetOnScreenAndAnimate(sprite: *mut Sprite) {
    if (*sprite).x != (*sprite).data[tDestinationX] || (*sprite).y != (*sprite).data[tDestinationY]
    {
        if (*sprite).x < (*sprite).data[tDestinationX] {
            (*sprite).x += 15;
        }
        if (*sprite).x > (*sprite).data[tDestinationX] {
            (*sprite).x -= 15;
        }
        if (*sprite).y < (*sprite).data[tDestinationY] {
            (*sprite).y += 10;
        }
        if (*sprite).y > (*sprite).data[tDestinationY] {
            (*sprite).y -= 10;
        }
    } else {
        let species: i16 = (*sprite).data[tSpecies];
        if species == SPECIES_EGG as i16 {
            DoMonFrontSpriteAnimation(sprite, species as u16, TRUE, 3);
        } else {
            DoMonFrontSpriteAnimation(sprite, species as u16, FALSE, 3);
        }
    }
}
pub(crate) unsafe fn SpriteCB_HofConfetti(sprite: *mut Sprite) {
    if (*sprite).y2 > 120 {
        DestroySprite(sprite);
    } else {
        (*sprite).y2 += 1;
        (*sprite).y2 += (*sprite).data[sExtraY];
        let sineIdx: u8 = (*sprite).data[sSineIdx] as u8;
        let rand: u16 = (Random() as i32 % 4) as u16 + 8;
        (*sprite).x2 = (rand as i32
            * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[sineIdx] as i32
            / 256) as i16;
        (*sprite).data[sSineIdx] += 4;
    }
}
unsafe fn CreateHofConfettiSprite() -> u8 {
    let posX: i16 = (Random() as i32 % 240) as i16;
    let posY: i16 = -((Random() as i32 % 8) as i16);
    let spriteID: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_HofConfetti).cast_mut(),
        posX,
        posY,
        0,
    );
    let sprite: *mut Sprite = &raw mut gSprites[spriteID];
    StartSpriteAnim(sprite, (Random() % 17) as u8);
    if Random() as i32 & 3 != 0 {
        (*sprite).data[sExtraY] = 0;
    } else {
        (*sprite).data[sExtraY] = 1;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn DoDomeConfetti() {
    gSpecialVar_0x8004 = 180;
    let taskId: u8 = CreateTask(Some(Task_DoDomeConfetti), 0);
    if taskId != TASK_NONE {
        task_set(taskId, tTimer, gSpecialVar_0x8004 as i16);
        gSpecialVar_0x8005 = taskId as u16;
    }
}
unsafe fn StopDomeConfetti() {
    let mut taskId: u8 = 0;
    if ({
        taskId = FindTaskIdByFunc(Some(Task_DoDomeConfetti));
        taskId
    }) != TASK_NONE
    {
        DestroyTask(taskId);
    }
    ConfettiUtil_Free();
    FreeSpriteTilesByTag(TAG_CONFETTI);
    FreeSpritePaletteByTag(TAG_CONFETTI);
}
pub(crate) unsafe fn UpdateDomeConfetti(util: *mut ConfettiUtil) {
    if (*util).yDelta > 110 {
        task_set(
            (*util).data[7],
            tConfettiCount,
            task_get((*util).data[7], tConfettiCount) - 1,
        );
        ConfettiUtil_Remove((*util).id);
    } else {
        (*util).yDelta += 1;
        (*util).yDelta += (*util).data[1];
        let sineIdx: u8 = (*util).data[0] as u8;
        let mut rand: i32 = Random() as i32;
        rand &= 3;
        rand += 8;
        (*util).xDelta = (rand
            * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[sineIdx] as i32
            / 256) as i16;
        (*util).data[0] += 4;
    }
}
pub(crate) unsafe fn Task_DoDomeConfetti(taskId: u8) {
    let mut id: u32 = 0;
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    match *data {
        0 => {
            if ConfettiUtil_Init(64) == 0 {
                DestroyTask(taskId);
                gSpecialVar_0x8004 = 0;
                gSpecialVar_0x8005 = 0xFFFF;
            }
            LoadCompressedSpriteSheet(sSpriteSheet_Confetti.as_ptr().cast_mut());
            LoadCompressedSpritePalette(sSpritePalette_Confetti.as_ptr().cast_mut());
            *data += 1;
        }
        1 => {
            if *data.at(1) != 0 && *data.at(1) as i32 % 3 == 0 {
                id = ConfettiUtil_AddNew(
                    (&raw const *sOamData_Confetti).cast_mut(),
                    TAG_CONFETTI,
                    TAG_CONFETTI,
                    (Random() as i32 % 240) as i16,
                    -((Random() as i32 % 8) as i16),
                    (Random() % 17) as u8,
                    id as u8,
                ) as u32;
                if id != 0xFF {
                    ConfettiUtil_SetCallback(id as u8, Some(UpdateDomeConfetti));
                    if Random() as i32 % 4 == 0 {
                        ConfettiUtil_SetData(id as u8, 1, 1);
                    }
                    ConfettiUtil_SetData(id as u8, CONFETTI_TASK_ID, taskId as i16);
                    *data.at(15) += 1;
                }
            }
            ConfettiUtil_Update();
            if *data.at(1) != 0 {
                *data.at(1) -= 1;
            } else if *data.at(15) == 0 {
                *data = 0xFF;
            }
        }
        255 => {
            StopDomeConfetti();
            gSpecialVar_0x8004 = 0;
            gSpecialVar_0x8005 = 0xFFFF;
        }
        _ => {}
    }
}
