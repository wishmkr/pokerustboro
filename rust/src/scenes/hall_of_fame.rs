//! Translated from `src/hall_of_fame.c` by tools/rustport/c2rs.py.
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
        ((self.bits_8 as u32 >> 0) & 0x1ff) as u16
    }
    #[inline(always)]
    pub fn set_species(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x1ff << 0)) | ((v as u16 & 0x1ff) << 0);
    }
    #[inline(always)]
    pub fn lvl(&self) -> u16 {
        ((self.bits_8 as u32 >> 9) & 0x7f) as u16
    }
    #[inline(always)]
    pub fn set_lvl(&mut self, v: u16) {
        self.bits_8 = (self.bits_8 & !(0x7f << 9)) | ((v as u16 & 0x7f) << 9);
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
pub(crate) static mut sHofFadePalettes: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofMonPtr: *mut HallofFameTeam = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofGfxPtr: *mut HofGfx = null_mut();

unsafe extern "C" {
    static mut gDamagedSaveSectors: u32;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gGameContinueCallback: Option<unsafe extern "C" fn()>;
    static mut gHasHallOfFameRecords: u8;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gReservedSpritePaletteCount: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AButtonExit: CArray<u8, 0>;
    static gText_HOFCorrupted: CArray<u8, 0>;
    static gText_HOFNumber: CArray<u8, 0>;
    static gText_IDNumber: CArray<u8, 0>;
    static gText_LeagueChamp: CArray<u8, 0>;
    static gText_Level: CArray<u8, 0>;
    static gText_Name: CArray<u8, 0>;
    static gText_Number: CArray<u8, 0>;
    static gText_PickCancel: CArray<u8, 0>;
    static gText_PickNextCancel: CArray<u8, 0>;
    static gText_SavingDontTurnOffPower: CArray<u8, 0>;
    static gText_Time: CArray<u8, 0>;
    static gText_WelcomeToHOF: CArray<u8, 0>;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
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
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlendPalettesUnfaded(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_StartCreditsSequence();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ComputerScreenCloseEffect(a0: u16, a1: u16, a2: u8);
    fn ComputerScreenOpenEffect(a0: u16, a1: u16, a2: u8);
    fn ConfettiUtil_AddNew(
        a0: *mut OamData,
        a1: u16,
        a2: u16,
        a3: i16,
        a4: i16,
        a5: u8,
        a6: u8,
    ) -> u8;
    fn ConfettiUtil_Free() -> u32;
    fn ConfettiUtil_Init(a0: u8) -> u32;
    fn ConfettiUtil_Remove(a0: u8) -> u8;
    fn ConfettiUtil_SetCallback(a0: u8, a1: Option<unsafe extern "C" fn(*mut ConfettiUtil)>) -> u8;
    fn ConfettiUtil_SetData(a0: u8, a1: u8, a2: i16) -> u8;
    fn ConfettiUtil_Update() -> u32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
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
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerPicSprite(a0: u16, a1: u8, a2: i16, a3: i16, a4: u8, a5: u16) -> u16;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoMonFrontSpriteAnimation(a0: *mut Sprite, a1: u16, a2: u8, a3: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FadeOutBGM(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeAndDestroyTrainerPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetGameStat(a0: u8) -> u32;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn HideBg(a0: u8);
    fn HofPCTopBar_AddWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn HofPCTopBar_Print(a0: *mut u8, a1: u8, a2: u8);
    fn HofPCTopBar_PrintPair(a0: *mut u8, a1: *mut u8, a2: u8, a3: u8, a4: u8);
    fn HofPCTopBar_RemoveWindow();
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsComputerScreenCloseEffectActive() -> u8;
    fn IsComputerScreenOpenEffectActive() -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadGameSave(a0: u8) -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadWindowGfx(a0: u8, a1: u8, a2: u16, a3: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId_Debug(a0: u8, a1: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ReturnFromHallOfFamePC();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpeciesToPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopCryAndClearCrySongs();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
}

pub(crate) unsafe extern "C" fn VBlankCB_HallOfFame() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_HallOfFame() {
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn InitHallOfFameScreen() -> u8 {
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFameScreen() {
    if InitHallOfFameScreen() == 0 {
        let mut taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0);
        gTasks[taskId].data[0] = FALSE as i16;
        sHofMonPtr = AllocZeroed(120) as *mut HallofFameTeam;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFameScreenDontSaveData() {
    if InitHallOfFameScreen() == 0 {
        let mut taskId: u8 = CreateTask(Some(Task_Hof_InitMonData), 0);
        gTasks[taskId].data[0] = TRUE as i16;
        sHofMonPtr = AllocZeroed(120) as *mut HallofFameTeam;
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_InitMonData(taskId: u8) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    gTasks[taskId].data[2] = 0;
    i = 0;
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
            j = 0;
            while j < POKEMON_NAME_LENGTH as u16 {
                (*sHofMonPtr).mon[i].nickname[j] = nickname[j];
                j += 1;
            }
            gTasks[taskId].data[2] += 1;
        } else {
            (*sHofMonPtr).mon[i].set_species(SPECIES_NONE);
            (*sHofMonPtr).mon[i].tid = 0;
            (*sHofMonPtr).mon[i].personality = 0;
            (*sHofMonPtr).mon[i].set_lvl(0);
            (*sHofMonPtr).mon[i].nickname[0] = EOS;
        }
        i += 1;
    }
    sHofFadePalettes = 0;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[4] = SPRITE_NONE as i16;
    i = 0;
    while i < PARTY_SIZE as u16 {
        gTasks[taskId].data[i as i32 + 5] = SPRITE_NONE as i16;
        i += 1;
    }
    if gTasks[taskId].data[0] != 0 {
        gTasks[taskId].func = Some(Task_Hof_SetMonDisplayTask);
    } else {
        gTasks[taskId].func = Some(Task_Hof_InitTeamSaveData);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_InitTeamSaveData(taskId: u8) {
    let mut i: u16 = 0;
    let mut lastSavedTeam: *mut HallofFameTeam =
        gDecompressionBuffer.as_mut_ptr() as *mut HallofFameTeam;
    if gHasHallOfFameRecords == 0 {
        memset(gDecompressionBuffer.as_mut_ptr(), 0, 8192);
    } else {
        if LoadGameSave(SAVE_HALL_OF_FAME) != SAVE_STATUS_OK {
            memset(gDecompressionBuffer.as_mut_ptr(), 0, 8192);
        }
    }
    i = 0;
    while i < HALL_OF_FAME_MAX_TEAMS {
        if (*lastSavedTeam).mon[0].species() == 0 {
            break;
        }
        i += 1;
        lastSavedTeam = lastSavedTeam.at(1);
    }
    if i >= HALL_OF_FAME_MAX_TEAMS {
        let mut afterTeam: *mut HallofFameTeam =
            gDecompressionBuffer.as_mut_ptr() as *mut HallofFameTeam;
        let mut beforeTeam: *mut HallofFameTeam =
            gDecompressionBuffer.as_mut_ptr() as *mut HallofFameTeam;
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
        gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(0, COPYWIN_FULL);
    gTasks[taskId].func = Some(Task_Hof_TrySaveData);
}
pub(crate) unsafe extern "C" fn Task_Hof_TrySaveData(taskId: u8) {
    gGameContinueCallback = Some(CB2_DoHallOfFameScreenDontSaveData);
    if TrySavingData(SAVE_HALL_OF_FAME) == SAVE_STATUS_ERROR && gDamagedSaveSectors != 0 {
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
        gTasks[taskId].func = Some(Task_Hof_WaitToDisplayMon);
        gTasks[taskId].data[3] = 32;
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitToDisplayMon(taskId: u8) {
    if gTasks[taskId].data[3] != 0 {
        gTasks[taskId].data[3] -= 1;
    } else {
        gTasks[taskId].func = Some(Task_Hof_SetMonDisplayTask);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_SetMonDisplayTask(taskId: u8) {
    gTasks[taskId].func = Some(Task_Hof_DisplayMon);
}
pub(crate) unsafe extern "C" fn Task_Hof_DisplayMon(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut startX: i16 = 0;
    let mut startY: i16 = 0;
    let mut destX: i16 = 0;
    let mut destY: i16 = 0;
    let mut currMonId: u16 = gTasks[taskId].data[1] as u16;
    let mut currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currMonId];
    if gTasks[taskId].data[2] > 3 {
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
    spriteId = CreateMonPicSprite_Affine(
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
    gSprites[spriteId].data[7] = (*currMon).species() as i16;
    gSprites[spriteId].callback = Some(SpriteCB_GetOnScreenAndAnimate);
    gTasks[taskId].data[currMonId as i32 + 5] = spriteId as i16;
    ClearDialogWindowAndFrame(0, TRUE);
    gTasks[taskId].func = Some(Task_Hof_PrintMonInfoAfterAnimating);
}
pub(crate) unsafe extern "C" fn Task_Hof_PrintMonInfoAfterAnimating(taskId: u8) {
    let mut currMonId: u16 = gTasks[taskId].data[1] as u16;
    let mut currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currMonId];
    let mut monSprite: *mut Sprite = &raw mut gSprites[gTasks[taskId].data[currMonId as i32 + 5]];
    if (*monSprite).callback == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite)) {
        (*monSprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
        HallOfFame_PrintMonInfo(currMon, 0, 14);
        gTasks[taskId].data[3] = 120;
        gTasks[taskId].func = Some(Task_Hof_TryDisplayAnotherMon);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_TryDisplayAnotherMon(taskId: u8) {
    let mut currPokeID: u16 = gTasks[taskId].data[1] as u16;
    let mut currMon: *mut HallofFameMon = &raw mut (*sHofMonPtr).mon[currPokeID];
    if gTasks[taskId].data[3] != 0 {
        gTasks[taskId].data[3] -= 1;
    } else {
        sHofFadePalettes |= shl_i32(
            0x10000,
            gSprites[gTasks[taskId].data[currPokeID as i32 + 5]]
                .oam
                .paletteNum() as u32,
        ) as u32;
        if gTasks[taskId].data[1] < 5 && (*currMon.at(1)).species() != SPECIES_NONE {
            gTasks[taskId].data[1] += 1;
            BeginNormalPaletteFade(sHofFadePalettes, 0, 12, 12, 25520);
            gSprites[gTasks[taskId].data[currPokeID as i32 + 5]]
                .oam
                .set_priority(1);
            gTasks[taskId].func = Some(Task_Hof_DisplayMon);
        } else {
            gTasks[taskId].func = Some(Task_Hof_PaletteFadeAndPrintWelcomeText);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_PaletteFadeAndPrintWelcomeText(taskId: u8) {
    let mut i: u16 = 0;
    BeginNormalPaletteFade(PALETTES_OBJECTS, 0, 0, 0, 0);
    i = 0;
    while i < PARTY_SIZE as u16 {
        if gTasks[taskId].data[i as i32 + 5] != SPRITE_NONE as i16 {
            gSprites[gTasks[taskId].data[i as i32 + 5]]
                .oam
                .set_priority(0);
        }
        i += 1;
    }
    HallOfFame_PrintWelcomeText(0, 15);
    PlaySE(SE_APPLAUSE);
    gTasks[taskId].data[3] = 400;
    gTasks[taskId].func = Some(Task_Hof_DoConfetti);
}
pub(crate) unsafe extern "C" fn Task_Hof_DoConfetti(taskId: u8) {
    if gTasks[taskId].data[3] != 0 {
        gTasks[taskId].data[3] -= 1;
        if gTasks[taskId].data[3] as i32 & 3 == 0 && gTasks[taskId].data[3] > 110 {
            CreateHofConfettiSprite();
        }
    } else {
        let mut i: u16 = 0;
        i = 0;
        while i < PARTY_SIZE as u16 {
            if gTasks[taskId].data[i as i32 + 5] != SPRITE_NONE as i16 {
                gSprites[gTasks[taskId].data[i as i32 + 5]]
                    .oam
                    .set_priority(1);
            }
            i += 1;
        }
        BeginNormalPaletteFade(sHofFadePalettes, 0, 12, 12, 25520);
        FillWindowPixelBuffer(0, 0);
        CopyWindowToVram(0, COPYWIN_FULL);
        gTasks[taskId].data[3] = 7;
        gTasks[taskId].func = Some(Task_Hof_WaitToDisplayPlayer);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitToDisplayPlayer(taskId: u8) {
    if gTasks[taskId].data[3] >= 16 {
        gTasks[taskId].func = Some(Task_Hof_DisplayPlayer);
    } else {
        gTasks[taskId].data[3] += 1;
        SetGpuReg(REG_OFFSET_BLDALPHA, gTasks[taskId].data[3] as u16 * 256);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_DisplayPlayer(taskId: u8) {
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(3);
    gTasks[taskId].data[4] = CreateTrainerPicSprite(
        PlayerGenderToFrontTrainerPicId_Debug((*gSaveBlock2Ptr).playerGender, TRUE),
        TRUE,
        120,
        72,
        6,
        TAG_NONE,
    ) as i16;
    AddWindow((&raw const *sHof_WindowTemplate).cast_mut());
    LoadWindowGfx(
        1,
        (*gSaveBlock2Ptr).optionsWindowFrameType() as u8,
        0x21D,
        208,
    );
    LoadPalette(GetTextWindowPalette(1) as *mut c_void, 224, 32);
    gTasks[taskId].data[3] = 120;
    gTasks[taskId].func = Some(Task_Hof_WaitAndPrintPlayerInfo);
}
pub(crate) unsafe extern "C" fn Task_Hof_WaitAndPrintPlayerInfo(taskId: u8) {
    if gTasks[taskId].data[3] != 0 {
        gTasks[taskId].data[3] -= 1;
    } else if gSprites[gTasks[taskId].data[4]].x != 192 {
        gSprites[gTasks[taskId].data[4]].x += 1;
    } else {
        FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 0x20, 0x20);
        HallOfFame_PrintPlayerInfo(1, 2);
        DrawDialogueFrame(0, 0);
        AddTextPrinterParameterized2(
            0,
            FONT_NORMAL,
            gText_LeagueChamp.as_ptr().cast_mut(),
            0,
            None,
            TEXT_COLOR_DARK_GRAY,
            TEXT_COLOR_WHITE,
            TEXT_COLOR_LIGHT_GRAY,
        );
        CopyWindowToVram(0, COPYWIN_FULL);
        gTasks[taskId].func = Some(Task_Hof_ExitOnKeyPressed);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_ExitOnKeyPressed(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        FadeOutBGM(4);
        gTasks[taskId].func = Some(Task_Hof_HandlePaletteOnExit);
    }
}
pub(crate) unsafe extern "C" fn Task_Hof_HandlePaletteOnExit(taskId: u8) {
    CpuSet(
        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
        gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
        512,
    );
    BeginNormalPaletteFade(PALETTES_ALL, 8, 0, 0x10, 0);
    gTasks[taskId].func = Some(Task_Hof_HandleExit);
}
pub(crate) unsafe extern "C" fn Task_Hof_HandleExit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut i: i32 = 0;
        i = 0;
        while i < PARTY_SIZE {
            let mut spriteId: u8 = gTasks[taskId].data[i + 5] as u8;
            if spriteId != SPRITE_NONE {
                FreeOamMatrix(gSprites[spriteId].oam.matrixNum() as u8);
                FreeAndDestroyMonPicSprite(spriteId as u16);
            }
            i += 1;
        }
        FreeAndDestroyTrainerPicSprite(gTasks[taskId].data[4] as u16);
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
pub(crate) unsafe extern "C" fn StartCredits() {
    SetMainCallback2(Some(CB2_StartCreditsSequence));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_DoHallOfFamePC() {
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
                let mut fameTeam: *mut HallofFameTeam =
                    gDecompressionBuffer.as_mut_ptr() as *mut HallofFameTeam;
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
            let mut taskId: u8 = 0;
            let mut i: u8 = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1808);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            taskId = CreateTask(Some(Task_HofPC_CopySaveData), 0);
            i = 0;
            while i < PARTY_SIZE as u8 {
                gTasks[taskId].data[i as i32 + 5] = SPRITE_NONE as i16;
                i += 1;
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
pub(crate) unsafe extern "C" fn Task_HofPC_CopySaveData(taskId: u8) {
    HofPCTopBar_AddWindow(0, 30, 0, 12, 0x226);
    if LoadGameSave(SAVE_HALL_OF_FAME) != SAVE_STATUS_OK {
        gTasks[taskId].func = Some(Task_HofPC_PrintDataIsCorrupted);
    } else {
        let mut i: u16 = 0;
        let mut savedTeams: *mut HallofFameTeam = null_mut();
        CpuSet(
            gDecompressionBuffer.as_mut_ptr() as *mut c_void,
            sHofMonPtr as *mut c_void,
            SECTOR_SIZE,
        );
        savedTeams = sHofMonPtr;
        i = 0;
        while i < HALL_OF_FAME_MAX_TEAMS {
            if (*savedTeams).mon[0].species() == 0 {
                break;
            }
            i += 1;
            savedTeams = savedTeams.at(1);
        }
        if i < HALL_OF_FAME_MAX_TEAMS {
            gTasks[taskId].data[0] = i as i16 - 1;
        } else {
            gTasks[taskId].data[0] = 49;
        }
        gTasks[taskId].data[1] = GetGameStat(GAME_STAT_ENTERED_HOF) as i16;
        gTasks[taskId].func = Some(Task_HofPC_DrawSpritesPrintText);
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_DrawSpritesPrintText(taskId: u8) {
    let mut savedTeams: *mut HallofFameTeam = sHofMonPtr;
    let mut currMon: *mut HallofFameMon = null_mut();
    let mut i: u16 = 0;
    i = 0;
    while (i as i32) < gTasks[taskId].data[0] as i32 {
        savedTeams = savedTeams.at(1);
        i += 1;
    }
    currMon = &raw mut (*savedTeams).mon[0];
    sHofFadePalettes = 0;
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].data[4] = 0;
    i = 0;
    while i < PARTY_SIZE as u16 {
        if (*currMon).species() != 0 {
            gTasks[taskId].data[4] += 1;
        }
        i += 1;
        currMon = currMon.at(1);
    }
    currMon = &raw mut (*savedTeams).mon[0];
    i = 0;
    while i < PARTY_SIZE as u16 {
        if (*currMon).species() != 0 {
            let mut spriteId: u16 = 0;
            let mut posX: i16 = 0;
            let mut posY: i16 = 0;
            if gTasks[taskId].data[4] > 3 {
                posX = sHallOfFame_MonFullTeamPositions[i][2];
                posY = sHallOfFame_MonFullTeamPositions[i][3];
            } else {
                posX = sHallOfFame_MonHalfTeamPositions[i][2];
                posY = sHallOfFame_MonHalfTeamPositions[i][3];
            }
            if (*currMon).species() == SPECIES_EGG as u16 {
                posY += 10;
            }
            spriteId = CreateMonPicSprite_HandleDeoxys(
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
            gTasks[taskId].data[i as i32 + 5] = spriteId as i16;
        } else {
            gTasks[taskId].data[i as i32 + 5] = SPRITE_NONE as i16;
        }
        i += 1;
        currMon = currMon.at(1);
    }
    BlendPalettes(PALETTES_OBJECTS, 0xC, 25520);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        gTasks[taskId].data[1] as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_HOFNumber.as_ptr().cast_mut(),
    );
    if gTasks[taskId].data[0] <= 0 {
        HofPCTopBar_PrintPair(
            gStringVar4.as_mut_ptr(),
            gText_PickCancel.as_ptr().cast_mut(),
            0,
            0,
            TRUE,
        );
    } else {
        HofPCTopBar_PrintPair(
            gStringVar4.as_mut_ptr(),
            gText_PickNextCancel.as_ptr().cast_mut(),
            0,
            0,
            TRUE,
        );
    }
    gTasks[taskId].func = Some(Task_HofPC_PrintMonInfo);
}
pub(crate) unsafe extern "C" fn Task_HofPC_PrintMonInfo(taskId: u8) {
    let mut savedTeams: *mut HallofFameTeam = sHofMonPtr;
    let mut currMon: *mut HallofFameMon = null_mut();
    let mut i: u16 = 0;
    let mut currMonID: u16 = 0;
    i = 0;
    while (i as i32) < gTasks[taskId].data[0] as i32 {
        savedTeams = savedTeams.at(1);
        i += 1;
    }
    i = 0;
    while i < PARTY_SIZE as u16 {
        let mut spriteId: u16 = gTasks[taskId].data[i as i32 + 5] as u16;
        if spriteId != SPRITE_NONE as u16 {
            gSprites[spriteId].oam.set_priority(1);
        }
        i += 1;
    }
    currMonID = gTasks[taskId].data[gTasks[taskId].data[2] as i32 + 5] as u16;
    gSprites[currMonID].oam.set_priority(0);
    sHofFadePalettes =
        shl_i32(0x10000, gSprites[currMonID].oam.paletteNum() as u32) as u32 ^ PALETTES_OBJECTS;
    BlendPalettesUnfaded(sHofFadePalettes, 0xC, 25520);
    currMon = &raw mut (*savedTeams).mon[gTasks[taskId].data[2]];
    if (*currMon).species() != SPECIES_EGG as u16 {
        StopCryAndClearCrySongs();
        PlayCry_Normal((*currMon).species(), 0);
    }
    HallOfFame_PrintMonInfo(currMon, 0, 14);
    gTasks[taskId].func = Some(Task_HofPC_HandleInput);
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandleInput(taskId: u8) {
    let mut i: u16 = 0;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if gTasks[taskId].data[0] != 0 {
            gTasks[taskId].data[0] -= 1;
            i = 0;
            while i < PARTY_SIZE as u16 {
                let mut spriteId: u8 = gTasks[taskId].data[i as i32 + 5] as u8;
                if spriteId != SPRITE_NONE {
                    FreeAndDestroyMonPicSprite(spriteId as u16);
                    gTasks[taskId].data[i as i32 + 5] = SPRITE_NONE as i16;
                }
                i += 1;
            }
            if gTasks[taskId].data[1] != 0 {
                gTasks[taskId].data[1] -= 1;
            }
            gTasks[taskId].func = Some(Task_HofPC_DrawSpritesPrintText);
        } else {
            if IsCryPlayingOrClearCrySongs() != 0 {
                StopCryAndClearCrySongs();
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
            }
            gTasks[taskId].func = Some(Task_HofPC_HandlePaletteOnExit);
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        if IsCryPlayingOrClearCrySongs() != 0 {
            StopCryAndClearCrySongs();
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0x100);
        }
        gTasks[taskId].func = Some(Task_HofPC_HandlePaletteOnExit);
    } else if gMain.newKeys as i32 & DPAD_UP != 0 && gTasks[taskId].data[2] != 0 {
        gTasks[taskId].data[2] -= 1;
        gTasks[taskId].func = Some(Task_HofPC_PrintMonInfo);
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (gTasks[taskId].data[2] as i32) < gTasks[taskId].data[4] as i32 - 1
    {
        gTasks[taskId].data[2] += 1;
        gTasks[taskId].func = Some(Task_HofPC_PrintMonInfo);
    }
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandlePaletteOnExit(taskId: u8) {
    let mut fameTeam: *mut HallofFameTeam = null_mut();
    CpuSet(
        gPlttBufferFaded.as_mut_ptr() as *mut c_void,
        gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
        512,
    );
    fameTeam = gDecompressionBuffer.as_mut_ptr() as *mut HallofFameTeam;
    (*fameTeam).mon[0] = *sDummyFameMon;
    ComputerScreenCloseEffect(0, 0, 0);
    gTasks[taskId].func = Some(Task_HofPC_HandleExit);
}
pub(crate) unsafe extern "C" fn Task_HofPC_HandleExit(taskId: u8) {
    if IsComputerScreenCloseEffectActive() == 0 {
        let mut i: u8 = 0;
        i = 0;
        while i < PARTY_SIZE as u8 {
            let mut spriteId: u16 = gTasks[taskId].data[i as i32 + 5] as u16;
            if spriteId != SPRITE_NONE as u16 {
                FreeAndDestroyMonPicSprite(spriteId);
                gTasks[taskId].data[i as i32 + 5] = SPRITE_NONE as i16;
            }
            i += 1;
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
pub(crate) unsafe extern "C" fn Task_HofPC_PrintDataIsCorrupted(taskId: u8) {
    HofPCTopBar_Print(gText_AButtonExit.as_ptr().cast_mut(), 8, TRUE);
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gText_HOFCorrupted.as_ptr().cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(0, COPYWIN_FULL);
    gTasks[taskId].func = Some(Task_HofPC_ExitOnButtonPress);
}
pub(crate) unsafe extern "C" fn Task_HofPC_ExitOnButtonPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        gTasks[taskId].func = Some(Task_HofPC_HandlePaletteOnExit);
    }
}
pub(crate) unsafe extern "C" fn HallOfFame_PrintWelcomeText(
    unusedPossiblyWindowId: u8,
    unused2: u8,
) {
    FillWindowPixelBuffer(0, 0);
    PutWindowTilemap(0);
    AddTextPrinterParameterized3(
        0,
        FONT_NORMAL,
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            gText_WelcomeToHOF.as_ptr().cast_mut(),
            0xD0,
        ) as u8,
        1,
        sMonInfoTextColors.as_ptr().cast_mut(),
        0,
        gText_WelcomeToHOF.as_ptr().cast_mut(),
    );
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn HallOfFame_PrintMonInfo(
    currMon: *mut HallofFameMon,
    unused1: u8,
    unused2: u8,
) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut stringPtr: *mut u8 = null_mut();
    let mut dexNumber: i32 = 0;
    let mut width: i32 = 0;
    FillWindowPixelBuffer(0, 0);
    PutWindowTilemap(0);
    if (*currMon).species() != SPECIES_EGG as u16 {
        stringPtr = StringCopy(text.as_mut_ptr(), gText_Number.as_ptr().cast_mut());
        dexNumber = SpeciesToPokedexNum((*currMon).species()) as i32;
        if dexNumber != 0xFFFF {
            *stringPtr = (dexNumber / 100) as u8 + CHAR_0;
            stringPtr = stringPtr.at(1);
            dexNumber = dexNumber % 100;
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
            gSpeciesNames[(*currMon).species()].as_ptr().cast_mut(),
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
        stringPtr = StringCopy(text.as_mut_ptr(), gText_Level.as_ptr().cast_mut());
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
        stringPtr = StringCopy(text.as_mut_ptr(), gText_IDNumber.as_ptr().cast_mut());
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
pub(crate) unsafe extern "C" fn HallOfFame_PrintPlayerInfo(unused1: u8, unused2: u8) {
    let mut text: CArray<u8, 20> = zeroed();
    let mut width: u32 = 0;
    let mut trainerId: u16 = 0;
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
        gText_Name.as_ptr().cast_mut(),
    );
    width = GetStringRightAlignXOffset(
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
    trainerId = (*gSaveBlock2Ptr).playerTrainerId[0] as u16
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u16) << 8;
    AddTextPrinterParameterized3(
        1,
        FONT_NORMAL,
        0,
        0x11,
        sPlayerInfoTextColors.as_ptr().cast_mut(),
        0,
        gText_IDNumber.as_ptr().cast_mut(),
    );
    text[0] = (trainerId as i32 % 100000 / 10000) as u8 + CHAR_0;
    text[1] = (trainerId as i32 % 10000 / 1000) as u8 + CHAR_0;
    text[2] = (trainerId as i32 % 1000 / 100) as u8 + CHAR_0;
    text[3] = (trainerId as i32 % 100 / 10) as u8 + CHAR_0;
    text[4] = (trainerId as i32 % 10 / 1) as u8 + CHAR_0;
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
        gText_Time.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn ClearVramOamPltt_LoadHofPal() {
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
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
pub(crate) unsafe extern "C" fn LoadHofGfx() {
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
pub(crate) unsafe extern "C" fn InitHofBgs() {
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
pub(crate) unsafe extern "C" fn LoadHofBgs() -> u8 {
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn SpriteCB_GetOnScreenAndAnimate(sprite: *mut Sprite) {
    if (*sprite).x != (*sprite).data[1] || (*sprite).y != (*sprite).data[2] {
        if (*sprite).x < (*sprite).data[1] {
            (*sprite).x += 15;
        }
        if (*sprite).x > (*sprite).data[1] {
            (*sprite).x -= 15;
        }
        if (*sprite).y < (*sprite).data[2] {
            (*sprite).y += 10;
        }
        if (*sprite).y > (*sprite).data[2] {
            (*sprite).y -= 10;
        }
    } else {
        let mut species: i16 = (*sprite).data[7];
        if species == SPECIES_EGG as i16 {
            DoMonFrontSpriteAnimation(sprite, species as u16, TRUE, 3);
        } else {
            DoMonFrontSpriteAnimation(sprite, species as u16, FALSE, 3);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HofConfetti(sprite: *mut Sprite) {
    if (*sprite).y2 > 120 {
        DestroySprite(sprite);
    } else {
        let mut rand: u16 = 0;
        let mut sineIdx: u8 = 0;
        (*sprite).y2 += 1;
        (*sprite).y2 += (*sprite).data[1];
        sineIdx = (*sprite).data[0] as u8;
        rand = (Random() as i32 % 4) as u16 + 8;
        (*sprite).x2 = (rand as i32 * gSineTable[sineIdx] as i32 / 256) as i16;
        (*sprite).data[0] += 4;
    }
}
pub(crate) unsafe extern "C" fn CreateHofConfettiSprite() -> u8 {
    let mut spriteID: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    let mut posX: i16 = (Random() as i32 % 240) as i16;
    let mut posY: i16 = -((Random() as i32 % 8) as i16);
    spriteID = CreateSprite(
        (&raw const *sSpriteTemplate_HofConfetti).cast_mut(),
        posX,
        posY,
        0,
    );
    sprite = &raw mut gSprites[spriteID];
    StartSpriteAnim(sprite, (Random() % 17) as u8);
    if Random() as i32 & 3 != 0 {
        (*sprite).data[1] = 0;
    } else {
        (*sprite).data[1] = 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDomeConfetti() {
    let mut taskId: u8 = 0;
    gSpecialVar_0x8004 = 180;
    taskId = CreateTask(Some(Task_DoDomeConfetti), 0);
    if taskId != TASK_NONE {
        gTasks[taskId].data[1] = gSpecialVar_0x8004 as i16;
        gSpecialVar_0x8005 = taskId as u16;
    }
}
pub(crate) unsafe extern "C" fn StopDomeConfetti() {
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
pub(crate) unsafe extern "C" fn UpdateDomeConfetti(util: *mut ConfettiUtil) {
    if (*util).yDelta > 110 {
        gTasks[(*util).data[7]].data[15] -= 1;
        ConfettiUtil_Remove((*util).id);
    } else {
        let mut sineIdx: u8 = 0;
        let mut rand: i32 = 0;
        (*util).yDelta += 1;
        (*util).yDelta += (*util).data[1];
        sineIdx = (*util).data[0] as u8;
        rand = Random() as i32;
        rand &= 3;
        rand += 8;
        (*util).xDelta = (rand * gSineTable[sineIdx] as i32 / 256) as i16;
        (*util).data[0] += 4;
    }
}
pub(crate) unsafe extern "C" fn Task_DoDomeConfetti(taskId: u8) {
    let mut id: u32 = 0;
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
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
