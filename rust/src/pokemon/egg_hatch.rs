//! Translated from `src/egg_hatch.c` by tools/rustport/c2rs.py.
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
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_main::gMonSpritesGfxPtr;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, ResetBgsAndClearDma3BusyFlags, SetBgAttribute,
    ShowBg, UnsetBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::daycare::{GetBoxMonNickname, GetMonNickname2};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005};
use crate::field_screen_effect::FieldCB_ContinueScriptHandleMusic;
use crate::field_weather::{FadeScreen, PlayRainStoppingSoundEffect};
use crate::gpu_regs::SetGpuReg;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::m4aSoundVSyncOn;
use crate::menu::{
    AddTextPrinterParameterized4, CreateYesNoMenu, DecompressAndLoadBgGfxUsingHeap,
    Menu_ProcessInputNoWrapClearOnChoose, ResetTempTileDataBuffers,
};
use crate::naming_screen::DoNamingScreen;
use crate::overworld::{
    CB2_ReturnToField, CleanupOverworldWindowsAndTilemaps, GetCurrentRegionMapSectionId,
    gFieldCallback,
};
use crate::palette::{
    BeginNormalPaletteFade, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    CalculateMonStats, CalculatePlayerPartyCount, CreateMon, DoMonFrontSpriteAnimation,
    GetMonAbility, GetMonData2, GetMonData3, GetMonGender, GetMonSpritePalStruct, GetSpeciesName,
    MonRestorePP, SetMonData, SetMultiuseSpriteTemplateToPokemon, SpeciesToNationalPokedexNum,
    gEnemyParty, gMultiuseSpriteTemplate, gPlayerParty,
};
use crate::pokemon_storage_system::{CountPartyAliveNonEggMonsExcept, CountStorageNonEggMons};
use crate::random::Random;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::LockPlayerFieldControls;
use crate::sound::{
    GetCurrentMapMusic, IsFanfareTaskInactive, PlayBGM, PlayFanfare, PlaySE, StopMapMusic,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_get, task_set};
use crate::text::{DeactivateAllTextPrinters, IsTextPrinterActive, RunTextPrinters};
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
    }
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
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
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
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
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringCompareWithoutExtCtrlCodes` with this module's view of its types.
#[inline]
unsafe fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompareWithoutExtCtrlCodes(a0 as _, a1 as _) }
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
/// `TVShowConvertInternationalString` with this module's view of its types.
#[inline]
unsafe fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32) {
    unsafe {
        crate::international_string_util::TVShowConvertInternationalString(a0 as _, a1 as _, a2);
    }
}
// The C's names for task and sprite data slots.
const sTimer: usize = 0;
const tTimer: usize = 0;
const sSinIdx: usize = 1;
const sVelocX: usize = 1;
const sDelayTimer: usize = 2;
const sVelocY: usize = 2;
const sAccelY: usize = 3;
const sDeltaX: usize = 4;
const sDeltaY: usize = 5;
// Data tables (translate with cdata.py): sEggPalette sEggHatchTiles sEggShardTiles sOamData_Egg sSpriteAnim_Egg_Normal sSpriteAnim_Egg_Cracked1 sSpriteAnim_Egg_Cracked2 sSpriteAnim_Egg_Cracked3 sSpriteAnimTable_Egg sEggHatch_Sheet sEggShards_Sheet sEgg_SpritePalette sSpriteTemplate_Egg sOamData_EggShard sSpriteAnim_EggShard0 sSpriteAnim_EggShard1 sSpriteAnim_EggShard2 sSpriteAnim_EggShard3 sSpriteAnimTable_EggShard sSpriteTemplate_EggShard sBgTemplates_EggHatch sWinTemplates_EggHatch sYesNoWinTemplate sEggShardVelocities

/// `struct EggHatchData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct EggHatchData {
    pub eggSpriteId: u8,
    pub monSpriteId: u8,
    pub state: u8,
    pub delayTimer: u8,
    pub eggPartyId: u8,
    pub unused_5: u8,
    pub unused_6: u8,
    pub eggShardVelocityId: u8,
    pub windowId: u8,
    pub unused_9: u8,
    pub unused_A: u8,
    pub species: u16,
    pub textColor: CArray<u8, 3>,
}

unsafe impl Sync for EggHatchData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<EggHatchData>() == 20);
    assert!(offset_of!(EggHatchData, eggSpriteId) == 0);
    assert!(offset_of!(EggHatchData, monSpriteId) == 1);
    assert!(offset_of!(EggHatchData, state) == 2);
    assert!(offset_of!(EggHatchData, delayTimer) == 3);
    assert!(offset_of!(EggHatchData, eggPartyId) == 4);
    assert!(offset_of!(EggHatchData, unused_5) == 5);
    assert!(offset_of!(EggHatchData, unused_6) == 6);
    assert!(offset_of!(EggHatchData, eggShardVelocityId) == 7);
    assert!(offset_of!(EggHatchData, windowId) == 8);
    assert!(offset_of!(EggHatchData, unused_9) == 9);
    assert!(offset_of!(EggHatchData, unused_A) == 10);
    assert!(offset_of!(EggHatchData, species) == 12);
    assert!(offset_of!(EggHatchData, textColor) == 14);
};

const EGG_ANIM_CRACKED_1: u8 = 1;
const EGG_ANIM_CRACKED_2: u8 = 2;
const EGG_X: i16 = 120;
const EGG_Y: i16 = 75;

static sBgTemplates_EggHatch: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::egg_hatch::sBgTemplates_EggHatch).cast());
static sEggHatch_Sheet: Table<SpriteSheet> =
    Table((&raw const crate::data::egg_hatch::sEggHatch_Sheet).cast());
static sEggShardVelocities: Table<CArray<CArray<i16, 2>, 19>> =
    Table((&raw const crate::data::egg_hatch::sEggShardVelocities).cast());
static sEggShards_Sheet: Table<SpriteSheet> =
    Table((&raw const crate::data::egg_hatch::sEggShards_Sheet).cast());
static sEgg_SpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::egg_hatch::sEgg_SpritePalette).cast());
static sSpriteAnimTable_EggShard: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::egg_hatch::sSpriteAnimTable_EggShard).cast());
static sSpriteTemplate_Egg: Table<SpriteTemplate> =
    Table((&raw const crate::data::egg_hatch::sSpriteTemplate_Egg).cast());
static sSpriteTemplate_EggShard: Table<SpriteTemplate> =
    Table((&raw const crate::data::egg_hatch::sSpriteTemplate_EggShard).cast());
static sWinTemplates_EggHatch: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::egg_hatch::sWinTemplates_EggHatch).cast());
static sYesNoWinTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::egg_hatch::sYesNoWinTemplate).cast());

pub(crate) static mut sEggHatchData: *mut EggHatchData = null_mut();

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn CreateHatchedMon(egg: *mut Pokemon, temp: *mut Pokemon) {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut ivs: CArray<u32, 6> = zeroed();
    let species: u16 = GetMonData2(egg, MON_DATA_SPECIES) as u16;
    for i in 0..(MAX_MON_MOVES as u8) {
        moves[i] = GetMonData2(egg, MON_DATA_MOVE1 + i as i32) as u16;
    }
    let personality: u32 = GetMonData2(egg, MON_DATA_PERSONALITY);
    for i in 0..(NUM_STATS as u8) {
        ivs[i] = GetMonData2(egg, MON_DATA_HP_IV + i as i32);
    }
    let mut language: u8 = GetMonData2(egg, MON_DATA_LANGUAGE) as u8;
    let mut gameMet: u8 = GetMonData2(egg, MON_DATA_MET_GAME) as u8;
    let mut markings: u8 = GetMonData2(egg, MON_DATA_MARKINGS) as u8;
    let mut pokerus: u32 = GetMonData2(egg, MON_DATA_POKERUS);
    let mut isModernFatefulEncounter: u8 =
        GetMonData2(egg, MON_DATA_MODERN_FATEFUL_ENCOUNTER) as u8;
    CreateMon(
        temp,
        species,
        EGG_HATCH_LEVEL,
        USE_RANDOM_IVS,
        TRUE,
        personality,
        0,
        0,
    );
    let mut i: u8 = 0;
    while i < MAX_MON_MOVES as u8 {
        SetMonData(
            temp,
            MON_DATA_MOVE1 + i as i32,
            &raw mut moves[i] as *mut c_void,
        );
        i += 1;
    }
    for i in 0..(NUM_STATS as u8) {
        SetMonData(
            temp,
            MON_DATA_HP_IV + i as i32,
            &raw mut ivs[i] as *mut c_void,
        );
    }
    language = GAME_LANGUAGE;
    SetMonData(temp, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    SetMonData(temp, MON_DATA_MET_GAME, &raw mut gameMet as *mut c_void);
    SetMonData(temp, MON_DATA_MARKINGS, &raw mut markings as *mut c_void);
    let mut friendship: u8 = 120;
    SetMonData(
        temp,
        MON_DATA_FRIENDSHIP,
        &raw mut friendship as *mut c_void,
    );
    SetMonData(temp, MON_DATA_POKERUS, &raw mut pokerus as *mut c_void);
    SetMonData(
        temp,
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        &raw mut isModernFatefulEncounter as *mut c_void,
    );
    *egg = *temp;
}
unsafe fn AddHatchedMonToParty(id: u8) {
    let mut isEgg: u8 = 0x46;
    let mut name: CArray<u8, 11> = zeroed();
    let mon: *mut Pokemon = &raw mut gPlayerParty[id];
    CreateHatchedMon(mon, &raw mut gEnemyParty[0]);
    SetMonData(mon, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
    let mut species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    GetSpeciesName(name.as_mut_ptr(), species);
    SetMonData(mon, MON_DATA_NICKNAME, name.as_mut_ptr() as *mut c_void);
    species = SpeciesToNationalPokedexNum(species);
    GetSetPokedexFlag(species, FLAG_SET_SEEN);
    GetSetPokedexFlag(species, FLAG_SET_CAUGHT);
    GetMonNickname2(mon, gStringVar1.as_mut_ptr());
    let mut ball: u16 = ITEM_POKE_BALL;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    let mut metLevel: u16 = 0;
    SetMonData(mon, MON_DATA_MET_LEVEL, &raw mut metLevel as *mut c_void);
    let mut metLocation: u8 = GetCurrentRegionMapSectionId();
    SetMonData(
        mon,
        MON_DATA_MET_LOCATION,
        &raw mut metLocation as *mut c_void,
    );
    MonRestorePP(mon);
    CalculateMonStats(mon);
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptHatchMon() {
    AddHatchedMonToParty(gSpecialVar_0x8004 as u8);
}
unsafe fn _CheckDaycareMonReceivedMail(daycare: *mut DayCare, daycareId: u8) -> u8 {
    let mut nickname: CArray<u8, 32> = zeroed();
    let daycareMon: *mut DaycareMon = &raw mut (*daycare).mons[daycareId];
    GetBoxMonNickname(&raw mut (*daycareMon).mon, nickname.as_mut_ptr());
    if (*daycareMon).mail.message.itemId != ITEM_NONE
        && (StringCompareWithoutExtCtrlCodes(
            nickname.as_mut_ptr(),
            (*daycareMon).mail.monName.as_mut_ptr(),
        ) != 0
            || StringCompareWithoutExtCtrlCodes(
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                (*daycareMon).mail.otName.as_mut_ptr(),
            ) != 0)
    {
        StringCopy(gStringVar1.as_mut_ptr(), nickname.as_mut_ptr());
        TVShowConvertInternationalString(
            gStringVar2.as_mut_ptr(),
            (*daycareMon).mail.otName.as_mut_ptr(),
            (*daycareMon).mail.gameLanguage() as i32,
        );
        TVShowConvertInternationalString(
            gStringVar3.as_mut_ptr(),
            (*daycareMon).mail.monName.as_mut_ptr(),
            (*daycareMon).mail.monLanguage() as i32,
        );
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn CheckDaycareMonReceivedMail() -> u8 {
    _CheckDaycareMonReceivedMail(&raw mut (*gSaveBlock1Ptr).daycare, gSpecialVar_0x8004 as u8)
}
unsafe fn EggHatchCreateMonSprite(useAlt: u8, state: u8, partyId: u8, speciesLoc: *mut u16) -> u8 {
    let mut position: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut mon: *mut Pokemon = null_mut();
    if useAlt == FALSE {
        mon = &raw mut gPlayerParty[partyId];
        position = B_POSITION_OPPONENT_LEFT;
    }
    if useAlt == TRUE {
        mon = &raw mut gPlayerParty[partyId];
        position = B_POSITION_OPPONENT_RIGHT;
    }
    match state {
        0 => {
            let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            let pid: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                    .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                    .cast_mut(),
                (*gMonSpritesGfxPtr).sprites.ptr
                    [useAlt as i32 * 2 + B_POSITION_OPPONENT_LEFT as i32],
                species as i32,
                pid,
            );
            LoadCompressedSpritePalette(GetMonSpritePalStruct(mon));
            *speciesLoc = species;
        }
        1 => {
            SetMultiuseSpriteTemplateToPokemon((*GetMonSpritePalStruct(mon)).tag, position);
            spriteId = CreateSprite(&raw mut gMultiuseSpriteTemplate, EGG_X, EGG_Y, 6);
            gSprites[spriteId].set_invisible(TRUE as u16);
            gSprites[spriteId].callback = Some(SpriteCallbackDummy);
        }
        _ => {}
    }
    spriteId
}
pub(crate) unsafe fn VBlankCB_EggHatch() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe fn EggHatch() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_EggHatch), 10);
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe fn Task_EggHatch(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_LoadEggHatch));
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_LoadEggHatch() {
    match gMain.state {
        0 => {
            SetGpuReg(0x0, 0);
            sEggHatchData = Alloc(20) as *mut EggHatchData;
            AllocateMonSpritesGfx();
            (*sEggHatchData).eggPartyId = gSpecialVar_0x8004 as u8;
            (*sEggHatchData).eggShardVelocityId = 0;
            SetVBlankCallback(Some(VBlankCB_EggHatch));
            gSpecialVar_0x8005 = GetCurrentMapMusic();
            ResetTempTileDataBuffers();
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates_EggHatch.as_ptr().cast_mut(), 2);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            SetBgAttribute(1, BG_ATTR_PRIORITY, 2);
            SetBgTilemapBuffer(1, Alloc(0x1000));
            SetBgTilemapBuffer(0, Alloc(0x2000));
            DeactivateAllTextPrinters();
            ResetPaletteFade();
            FreeAllSpritePalettes();
            ResetSpriteData();
            ResetTasks();
            ScanlineEffect_Stop();
            m4aSoundVSyncOn();
            gMain.state += 1;
        }
        1 => {
            InitWindows(sWinTemplates_EggHatch.as_ptr().cast_mut());
            (*sEggHatchData).windowId = 0;
            gMain.state += 1;
        }
        2 => {
            DecompressAndLoadBgGfxUsingHeap(
                0,
                (*(&raw const crate::data::graphics::gBattleTextboxTiles).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                0,
                (*(&raw const crate::data::graphics::gBattleTextboxTilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
            );
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattleTextboxPalette)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                32,
            );
            gMain.state += 1;
        }
        3 => {
            LoadSpriteSheet((&raw const *sEggHatch_Sheet).cast_mut());
            LoadSpriteSheet((&raw const *sEggShards_Sheet).cast_mut());
            LoadSpritePalette((&raw const *sEgg_SpritePalette).cast_mut());
            gMain.state += 1;
        }
        4 => {
            CopyBgTilemapBufferToVram(0);
            AddHatchedMonToParty((*sEggHatchData).eggPartyId);
            gMain.state += 1;
        }
        5 => {
            EggHatchCreateMonSprite(
                0,
                0,
                (*sEggHatchData).eggPartyId,
                &raw mut (*sEggHatchData).species,
            );
            gMain.state += 1;
        }
        6 => {
            (*sEggHatchData).monSpriteId = EggHatchCreateMonSprite(
                FALSE,
                1,
                (*sEggHatchData).eggPartyId,
                &raw mut (*sEggHatchData).species,
            );
            gMain.state += 1;
        }
        7 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            LoadPalette(
                (*(&raw const crate::data::graphics::gTradeGba2_Pal).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                16,
                160,
            );
            LoadBgTiles(
                1,
                (*(&raw const crate::data::graphics::gTradeGba_Gfx).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0x1420,
                0,
            );
            CopyToBgTilemapBuffer(
                1,
                (*(&raw const crate::data::trade::gTradePlatform_Tilemap).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0x1000,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            gMain.state += 1;
        }
        8 => {
            SetMainCallback2(Some(CB2_EggHatch));
            (*sEggHatchData).state = 0;
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn EggHatchSetMonNickname() {
    SetMonData(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_NICKNAME,
        gStringVar3.as_mut_ptr() as *mut c_void,
    );
    FreeMonSpritesGfx();
    Free(sEggHatchData as *mut c_void);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn Task_EggHatchPlayBGM(taskId: u8) {
    if task_get(taskId, tTimer) == 0 {
        StopMapMusic();
        PlayRainStoppingSoundEffect();
    }
    if task_get(taskId, tTimer) == 1 {
        PlayBGM(MUS_EVOLUTION_INTRO);
    }
    if task_get(taskId, tTimer) > 60 {
        PlayBGM(MUS_EVOLUTION);
        DestroyTask(taskId);
    }
    task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
}
pub(crate) unsafe fn CB2_EggHatch() {
    let mut species: u16 = 0;
    let mut gender: u8 = 0;
    let mut personality: u32 = 0;
    match (*sEggHatchData).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sEggHatchData).eggSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Egg).cast_mut(),
                EGG_X,
                EGG_Y,
                5,
            );
            ShowBg(0);
            ShowBg(1);
            (*sEggHatchData).state += 1;
            CreateTask(Some(Task_EggHatchPlayBGM), 5);
        }
        1 => {
            if gPaletteFade.active() == 0 {
                FillWindowPixelBuffer((*sEggHatchData).windowId, 0);
                (*sEggHatchData).delayTimer = 0;
                (*sEggHatchData).state += 1;
            }
        }
        2 => {
            if ({
                (*sEggHatchData).delayTimer += 1;
                (*sEggHatchData).delayTimer
            }) > 30
            {
                (*sEggHatchData).state += 1;
                gSprites[(*sEggHatchData).eggSpriteId].callback = Some(SpriteCB_Egg_Shake1);
            }
        }
        3 => {
            if gSprites[(*sEggHatchData).eggSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                species = GetMonData2(
                    &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                    MON_DATA_SPECIES,
                ) as u16;
                DoMonFrontSpriteAnimation(
                    &raw mut gSprites[(*sEggHatchData).monSpriteId],
                    species,
                    FALSE,
                    1,
                );
                (*sEggHatchData).state += 1;
            }
        }
        4 => {
            if gSprites[(*sEggHatchData).monSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                (*sEggHatchData).state += 1;
            }
        }
        5 => {
            GetMonNickname2(
                &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_HatchedFromEgg).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            EggHatchPrintMessage(
                (*sEggHatchData).windowId,
                gStringVar4.as_mut_ptr(),
                0,
                3,
                TEXT_SKIP_DRAW,
            );
            PlayFanfare(MUS_EVOLVED);
            (*sEggHatchData).state += 1;
            PutWindowTilemap((*sEggHatchData).windowId);
            CopyWindowToVram((*sEggHatchData).windowId, COPYWIN_FULL);
        }
        6 => {
            if IsFanfareTaskInactive() != 0 {
                (*sEggHatchData).state += 1;
            }
        }
        7 => {
            if IsFanfareTaskInactive() != 0 {
                (*sEggHatchData).state += 1;
            }
        }
        8 => {
            GetMonNickname2(
                &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_NicknameHatchPrompt)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            EggHatchPrintMessage((*sEggHatchData).windowId, gStringVar4.as_mut_ptr(), 0, 2, 1);
            (*sEggHatchData).state += 1;
        }
        9 => {
            if IsTextPrinterActive((*sEggHatchData).windowId) == 0 {
                LoadUserWindowBorderGfx((*sEggHatchData).windowId, 0x140, 224);
                CreateYesNoMenu((&raw const *sYesNoWinTemplate).cast_mut(), 0x140, 0xE, 0);
                (*sEggHatchData).state += 1;
            }
        }
        10 => match Menu_ProcessInputNoWrapClearOnChoose() {
            0 => {
                GetMonNickname2(
                    &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                    gStringVar3.as_mut_ptr(),
                );
                species = GetMonData2(
                    &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                    MON_DATA_SPECIES,
                ) as u16;
                gender = GetMonGender(&raw mut gPlayerParty[(*sEggHatchData).eggPartyId]);
                personality = GetMonData3(
                    &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                    MON_DATA_PERSONALITY,
                    null_mut(),
                );
                DoNamingScreen(
                    NAMING_SCREEN_NICKNAME,
                    gStringVar3.as_mut_ptr(),
                    species,
                    gender as u16,
                    personality,
                    Some(EggHatchSetMonNickname),
                );
            }
            1 | MENU_B_PRESSED => {
                (*sEggHatchData).state += 1;
            }
            _ => {}
        },
        11 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sEggHatchData).state += 1;
        }
        12 if gPaletteFade.active() == 0 => {
            FreeMonSpritesGfx();
            RemoveWindow((*sEggHatchData).windowId);
            UnsetBgTilemapBuffer(0);
            UnsetBgTilemapBuffer(1);
            Free(sEggHatchData as *mut c_void);
            SetMainCallback2(Some(CB2_ReturnToField));
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn SpriteCB_Egg_Shake1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 20
    {
        (*sprite).callback = Some(SpriteCB_Egg_Shake2);
        (*sprite).data[sTimer] = 0;
    } else {
        (*sprite).data[sSinIdx] = ((*sprite).data[sSinIdx] + 20) & 0xFF;
        (*sprite).x2 = Sin((*sprite).data[sSinIdx], 1);
        if (*sprite).data[sTimer] == 15 {
            PlaySE(SE_BALL);
            StartSpriteAnim(sprite, EGG_ANIM_CRACKED_1);
            CreateRandomEggShardSprite();
        }
    }
}
pub(crate) unsafe fn SpriteCB_Egg_Shake2(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sDelayTimer] += 1;
        (*sprite).data[sDelayTimer]
    }) > 30
    {
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) > 20
        {
            (*sprite).callback = Some(SpriteCB_Egg_Shake3);
            (*sprite).data[sTimer] = 0;
            (*sprite).data[sDelayTimer] = 0;
        } else {
            (*sprite).data[sSinIdx] = ((*sprite).data[sSinIdx] + 20) & 0xFF;
            (*sprite).x2 = Sin((*sprite).data[sSinIdx], 2);
            if (*sprite).data[sTimer] == 15 {
                PlaySE(SE_BALL);
                StartSpriteAnim(sprite, EGG_ANIM_CRACKED_2);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_Egg_Shake3(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sDelayTimer] += 1;
        (*sprite).data[sDelayTimer]
    }) > 30
    {
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) > 38
        {
            (*sprite).callback = Some(SpriteCB_Egg_WaitHatch);
            (*sprite).data[sTimer] = 0;
            let species: u16 = GetMonData2(
                &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                MON_DATA_SPECIES,
            ) as u16;
            gSprites[(*sEggHatchData).monSpriteId].x2 = 0;
            gSprites[(*sEggHatchData).monSpriteId].y2 = 0;
        } else {
            (*sprite).data[sSinIdx] = ((*sprite).data[sSinIdx] + 20) & 0xFF;
            (*sprite).x2 = Sin((*sprite).data[sSinIdx], 2);
            if (*sprite).data[sTimer] == 15 {
                PlaySE(SE_BALL);
                StartSpriteAnim(sprite, EGG_ANIM_CRACKED_2);
                CreateRandomEggShardSprite();
                CreateRandomEggShardSprite();
            }
            if (*sprite).data[sTimer] == 30 {
                PlaySE(SE_BALL);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_Egg_WaitHatch(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 50
    {
        (*sprite).callback = Some(SpriteCB_Egg_Hatch);
        (*sprite).data[sTimer] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_Egg_Hatch(sprite: *mut Sprite) {
    if (*sprite).data[sTimer] == 0 {
        BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 65535);
    }
    if ((*sprite).data[sTimer] as u32) < 4 {
        for i in 0..4i16 {
            CreateRandomEggShardSprite();
        }
    }
    (*sprite).data[sTimer] += 1;
    if gPaletteFade.active() == 0 {
        PlaySE(SE_EGG_HATCH);
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCB_Egg_Reveal);
        (*sprite).data[sTimer] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_Egg_Reveal(sprite: *mut Sprite) {
    if (*sprite).data[sTimer] == 0 {
        gSprites[(*sEggHatchData).monSpriteId].set_invisible(FALSE as u16);
        StartSpriteAffineAnim(
            &raw mut gSprites[(*sEggHatchData).monSpriteId],
            BATTLER_AFFINE_EMERGE,
        );
    }
    if (*sprite).data[sTimer] == 8 {
        BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 65535);
    }
    if (*sprite).data[sTimer] <= 9 {
        gSprites[(*sEggHatchData).monSpriteId].y -= 1;
    }
    if (*sprite).data[sTimer] > 40 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
    (*sprite).data[sTimer] += 1;
}
pub(crate) unsafe fn SpriteCB_EggShard(sprite: *mut Sprite) {
    (*sprite).data[sDeltaX] += (*sprite).data[sVelocX];
    (*sprite).data[sDeltaY] += (*sprite).data[sVelocY];
    (*sprite).x2 = (*sprite).data[sDeltaX] / 256;
    (*sprite).y2 = (*sprite).data[sDeltaY] / 256;
    (*sprite).data[sVelocY] += (*sprite).data[sAccelY];
    if (*sprite).y as i32 + (*sprite).y2 as i32 > (*sprite).y as i32 + 20
        && (*sprite).data[sVelocY] > 0
    {
        DestroySprite(sprite);
    }
}
unsafe fn CreateRandomEggShardSprite() {
    let velocityX: i16 = sEggShardVelocities[(*sEggHatchData).eggShardVelocityId][0];
    let velocityY: i16 = sEggShardVelocities[(*sEggHatchData).eggShardVelocityId][1];
    (*sEggHatchData).eggShardVelocityId += 1;
    let spriteAnimIndex: u16 = Random() % 4;
    CreateEggShardSprite(
        EGG_X as u8,
        60,
        velocityX,
        velocityY,
        100,
        spriteAnimIndex as u8,
    );
}
unsafe fn CreateEggShardSprite(
    x: u8,
    y: u8,
    velocityX: i16,
    velocityY: i16,
    acceleration: i16,
    spriteAnimIndex: u8,
) {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_EggShard).cast_mut(),
        x as i16,
        y as i16,
        4,
    );
    gSprites[spriteId].data[sVelocX] = velocityX;
    gSprites[spriteId].data[sVelocY] = velocityY;
    gSprites[spriteId].data[sAccelY] = acceleration;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimIndex);
}
unsafe fn EggHatchPrintMessage(windowId: u8, string: *mut u8, x: u8, y: u8, speed: u8) {
    FillWindowPixelBuffer(windowId, 255);
    (*sEggHatchData).textColor[0] = 0;
    (*sEggHatchData).textColor[1] = 5;
    (*sEggHatchData).textColor[2] = 6;
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        0,
        0,
        (*sEggHatchData).textColor.as_mut_ptr(),
        speed as i8,
        string,
    );
}
pub unsafe fn GetEggCyclesToSubtract() -> u8 {
    let count: u8 = CalculatePlayerPartyCount();
    for i in 0..count {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0 {
            let ability: u8 = GetMonAbility(&raw mut gPlayerParty[i]);
            if ability == ABILITY_MAGMA_ARMOR || ability == ABILITY_FLAME_BODY {
                return 2;
            }
        }
    }
    1
}
#[unsafe(no_mangle)]
pub unsafe fn CountPartyAliveNonEggMons() -> u16 {
    let mut aliveNonEggMonsCount: u16 = CountStorageNonEggMons() as u16;
    aliveNonEggMonsCount += CountPartyAliveNonEggMonsExcept(PARTY_SIZE as u8) as u16;
    aliveNonEggMonsCount
}
