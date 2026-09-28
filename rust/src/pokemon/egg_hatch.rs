//! Translated from `src/egg_hatch.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static gBattleTextboxPalette: CArray<u32, 0>;
    static gBattleTextboxTilemap: CArray<u32, 0>;
    static gBattleTextboxTiles: CArray<u32, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_HatchedFromEgg: CArray<u8, 0>;
    static gText_NicknameHatchPrompt: CArray<u8, 0>;
    static gTradeGba2_Pal: CArray<u16, 0>;
    static gTradeGba_Gfx: CArray<u8, 0>;
    static gTradePlatform_Tilemap: CArray<u16, 0>;
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
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CalculateMonStats(a0: *mut Pokemon);
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CleanupOverworldWindowsAndTilemaps();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountPartyAliveNonEggMonsExcept(a0: u8) -> u8;
    fn CountStorageNonEggMons() -> u32;
    fn CreateMon(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoMonFrontSpriteAnimation(a0: *mut Sprite, a1: u16, a2: u8, a3: u8);
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeMonSpritesGfx();
    fn GetBoxMonNickname(a0: *mut BoxPokemon, a1: *mut u8) -> *mut u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetCurrentRegionMapSectionId() -> u8;
    fn GetMonAbility(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonNickname2(a0: *mut Pokemon, a1: *mut u8) -> *mut u8;
    fn GetMonSpritePalStruct(a0: *mut Pokemon) -> *mut CompressedSpritePalette;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetSpeciesName(a0: *mut u8, a1: u16);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsFanfareTaskInactive() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MonRestorePP(a0: *mut Pokemon);
    fn PlayBGM(a0: u16);
    fn PlayFanfare(a0: u16);
    fn PlayRainStoppingSoundEffect();
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StopMapMusic();
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn m4aSoundVSyncOn();
}

pub(crate) unsafe extern "C" fn CreateHatchedMon(egg: *mut Pokemon, temp: *mut Pokemon) {
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut pokerus: u32 = 0;
    let mut i: u8 = 0;
    let mut friendship: u8 = 0;
    let mut language: u8 = 0;
    let mut gameMet: u8 = 0;
    let mut markings: u8 = 0;
    let mut isModernFatefulEncounter: u8 = 0;
    let mut moves: CArray<u16, 4> = zeroed();
    let mut ivs: CArray<u32, 6> = zeroed();
    species = GetMonData2(egg, MON_DATA_SPECIES) as u16;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        moves[i] = GetMonData2(egg, MON_DATA_MOVE1 + i as i32) as u16;
        i += 1;
    }
    personality = GetMonData2(egg, MON_DATA_PERSONALITY);
    i = 0;
    while i < NUM_STATS as u8 {
        ivs[i] = GetMonData2(egg, MON_DATA_HP_IV + i as i32);
        i += 1;
    }
    language = GetMonData2(egg, MON_DATA_LANGUAGE) as u8;
    gameMet = GetMonData2(egg, MON_DATA_MET_GAME) as u8;
    markings = GetMonData2(egg, MON_DATA_MARKINGS) as u8;
    pokerus = GetMonData2(egg, MON_DATA_POKERUS);
    isModernFatefulEncounter = GetMonData2(egg, MON_DATA_MODERN_FATEFUL_ENCOUNTER) as u8;
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
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        SetMonData(
            temp,
            MON_DATA_MOVE1 + i as i32,
            &raw mut moves[i] as *mut c_void,
        );
        i += 1;
    }
    i = 0;
    while i < NUM_STATS as u8 {
        SetMonData(
            temp,
            MON_DATA_HP_IV + i as i32,
            &raw mut ivs[i] as *mut c_void,
        );
        i += 1;
    }
    language = GAME_LANGUAGE;
    SetMonData(temp, MON_DATA_LANGUAGE, &raw mut language as *mut c_void);
    SetMonData(temp, MON_DATA_MET_GAME, &raw mut gameMet as *mut c_void);
    SetMonData(temp, MON_DATA_MARKINGS, &raw mut markings as *mut c_void);
    friendship = 120;
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
pub(crate) unsafe extern "C" fn AddHatchedMonToParty(id: u8) {
    let mut isEgg: u8 = 0x46;
    let mut species: u16 = 0;
    let mut name: CArray<u8, 11> = zeroed();
    let mut ball: u16 = 0;
    let mut metLevel: u16 = 0;
    let mut metLocation: u8 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[id];
    CreateHatchedMon(mon, &raw mut gEnemyParty[0]);
    SetMonData(mon, MON_DATA_IS_EGG, &raw mut isEgg as *mut c_void);
    species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    GetSpeciesName(name.as_mut_ptr(), species);
    SetMonData(mon, MON_DATA_NICKNAME, name.as_mut_ptr() as *mut c_void);
    species = SpeciesToNationalPokedexNum(species);
    GetSetPokedexFlag(species, FLAG_SET_SEEN);
    GetSetPokedexFlag(species, FLAG_SET_CAUGHT);
    GetMonNickname2(mon, gStringVar1.as_mut_ptr());
    ball = ITEM_POKE_BALL;
    SetMonData(mon, MON_DATA_POKEBALL, &raw mut ball as *mut c_void);
    metLevel = 0;
    SetMonData(mon, MON_DATA_MET_LEVEL, &raw mut metLevel as *mut c_void);
    metLocation = GetCurrentRegionMapSectionId();
    SetMonData(
        mon,
        MON_DATA_MET_LOCATION,
        &raw mut metLocation as *mut c_void,
    );
    MonRestorePP(mon);
    CalculateMonStats(mon);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptHatchMon() {
    AddHatchedMonToParty(gSpecialVar_0x8004 as u8);
}
pub(crate) unsafe extern "C" fn _CheckDaycareMonReceivedMail(
    daycare: *mut DayCare,
    daycareId: u8,
) -> u8 {
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut daycareMon: *mut DaycareMon = &raw mut (*daycare).mons[daycareId];
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckDaycareMonReceivedMail() -> u8 {
    return _CheckDaycareMonReceivedMail(
        &raw mut (*gSaveBlock1Ptr).daycare,
        gSpecialVar_0x8004 as u8,
    );
}
pub(crate) unsafe extern "C" fn EggHatchCreateMonSprite(
    useAlt: u8,
    state: u8,
    partyId: u8,
    speciesLoc: *mut u16,
) -> u8 {
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
            let mut species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            let mut pid: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                (&raw const gMonFrontPicTable[species]).cast_mut(),
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
    return spriteId;
}
pub(crate) unsafe extern "C" fn VBlankCB_EggHatch() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EggHatch() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_EggHatch), 10);
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe extern "C" fn Task_EggHatch(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_LoadEggHatch));
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadEggHatch() {
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
                gBattleTextboxTiles.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                0,
                gBattleTextboxTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            LoadCompressedPalette(gBattleTextboxPalette.as_ptr().cast_mut(), 0, 32);
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
            LoadPalette(gTradeGba2_Pal.as_ptr().cast_mut() as *mut c_void, 16, 160);
            LoadBgTiles(
                1,
                gTradeGba_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x1420,
                0,
            );
            CopyToBgTilemapBuffer(
                1,
                gTradePlatform_Tilemap.as_ptr().cast_mut() as *mut c_void,
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
pub(crate) unsafe extern "C" fn EggHatchSetMonNickname() {
    SetMonData(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar3.as_mut_ptr() as *mut c_void,
    );
    FreeMonSpritesGfx();
    Free(sEggHatchData as *mut c_void);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn Task_EggHatchPlayBGM(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        StopMapMusic();
        PlayRainStoppingSoundEffect();
    }
    if gTasks[taskId].data[0] == 1 {
        PlayBGM(MUS_EVOLUTION_INTRO);
    }
    if gTasks[taskId].data[0] > 60 {
        PlayBGM(MUS_EVOLUTION);
        DestroyTask(taskId);
    }
    gTasks[taskId].data[0] += 1;
}
pub(crate) unsafe extern "C" fn CB2_EggHatch() {
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
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
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
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
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
                gText_HatchedFromEgg.as_ptr().cast_mut(),
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
                gText_NicknameHatchPrompt.as_ptr().cast_mut(),
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
        12 => {
            if gPaletteFade.active() == 0 {
                FreeMonSpritesGfx();
                RemoveWindow((*sEggHatchData).windowId);
                UnsetBgTilemapBuffer(0);
                UnsetBgTilemapBuffer(1);
                Free(sEggHatchData as *mut c_void);
                SetMainCallback2(Some(CB2_ReturnToField));
            }
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 20
    {
        (*sprite).callback = Some(SpriteCB_Egg_Shake2);
        (*sprite).data[0] = 0;
    } else {
        (*sprite).data[1] = (*sprite).data[1] + 20 & 0xFF;
        (*sprite).x2 = Sin((*sprite).data[1], 1);
        if (*sprite).data[0] == 15 {
            PlaySE(SE_BALL);
            StartSpriteAnim(sprite, EGG_ANIM_CRACKED_1);
            CreateRandomEggShardSprite();
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake2(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) > 30
    {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) > 20
        {
            (*sprite).callback = Some(SpriteCB_Egg_Shake3);
            (*sprite).data[0] = 0;
            (*sprite).data[2] = 0;
        } else {
            (*sprite).data[1] = (*sprite).data[1] + 20 & 0xFF;
            (*sprite).x2 = Sin((*sprite).data[1], 2);
            if (*sprite).data[0] == 15 {
                PlaySE(SE_BALL);
                StartSpriteAnim(sprite, EGG_ANIM_CRACKED_2);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Shake3(sprite: *mut Sprite) {
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) > 30
    {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) > 38
        {
            let mut species: u16 = 0;
            (*sprite).callback = Some(SpriteCB_Egg_WaitHatch);
            (*sprite).data[0] = 0;
            species = GetMonData2(
                &raw mut gPlayerParty[(*sEggHatchData).eggPartyId],
                MON_DATA_SPECIES,
            ) as u16;
            gSprites[(*sEggHatchData).monSpriteId].x2 = 0;
            gSprites[(*sEggHatchData).monSpriteId].y2 = 0;
        } else {
            (*sprite).data[1] = (*sprite).data[1] + 20 & 0xFF;
            (*sprite).x2 = Sin((*sprite).data[1], 2);
            if (*sprite).data[0] == 15 {
                PlaySE(SE_BALL);
                StartSpriteAnim(sprite, EGG_ANIM_CRACKED_2);
                CreateRandomEggShardSprite();
                CreateRandomEggShardSprite();
            }
            if (*sprite).data[0] == 30 {
                PlaySE(SE_BALL);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_WaitHatch(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 50
    {
        (*sprite).callback = Some(SpriteCB_Egg_Hatch);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Hatch(sprite: *mut Sprite) {
    let mut i: i16 = 0;
    if (*sprite).data[0] == 0 {
        BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 65535);
    }
    if ((*sprite).data[0] as u32) < 4 {
        i = 0;
        while i < 4 {
            CreateRandomEggShardSprite();
            i += 1;
        }
    }
    (*sprite).data[0] += 1;
    if gPaletteFade.active() == 0 {
        PlaySE(SE_EGG_HATCH);
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCB_Egg_Reveal);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Egg_Reveal(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        gSprites[(*sEggHatchData).monSpriteId].set_invisible(FALSE as u16);
        StartSpriteAffineAnim(
            &raw mut gSprites[(*sEggHatchData).monSpriteId],
            BATTLER_AFFINE_EMERGE,
        );
    }
    if (*sprite).data[0] == 8 {
        BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 65535);
    }
    if (*sprite).data[0] <= 9 {
        gSprites[(*sEggHatchData).monSpriteId].y -= 1;
    }
    if (*sprite).data[0] > 40 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
    (*sprite).data[0] += 1;
}
pub(crate) unsafe extern "C" fn SpriteCB_EggShard(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[1];
    (*sprite).data[5] += (*sprite).data[2];
    (*sprite).x2 = (*sprite).data[4] / 256;
    (*sprite).y2 = (*sprite).data[5] / 256;
    (*sprite).data[2] += (*sprite).data[3];
    if (*sprite).y as i32 + (*sprite).y2 as i32 > (*sprite).y as i32 + 20 && (*sprite).data[2] > 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateRandomEggShardSprite() {
    let mut spriteAnimIndex: u16 = 0;
    let mut velocityX: i16 = sEggShardVelocities[(*sEggHatchData).eggShardVelocityId][0];
    let mut velocityY: i16 = sEggShardVelocities[(*sEggHatchData).eggShardVelocityId][1];
    (*sEggHatchData).eggShardVelocityId += 1;
    spriteAnimIndex = Random() % 4;
    CreateEggShardSprite(
        EGG_X as u8,
        60,
        velocityX,
        velocityY,
        100,
        spriteAnimIndex as u8,
    );
}
pub(crate) unsafe extern "C" fn CreateEggShardSprite(
    x: u8,
    y: u8,
    velocityX: i16,
    velocityY: i16,
    acceleration: i16,
    spriteAnimIndex: u8,
) {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_EggShard).cast_mut(),
        x as i16,
        y as i16,
        4,
    );
    gSprites[spriteId].data[1] = velocityX;
    gSprites[spriteId].data[2] = velocityY;
    gSprites[spriteId].data[3] = acceleration;
    StartSpriteAnim(&raw mut gSprites[spriteId], spriteAnimIndex);
}
pub(crate) unsafe extern "C" fn EggHatchPrintMessage(
    windowId: u8,
    string: *mut u8,
    x: u8,
    y: u8,
    speed: u8,
) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEggCyclesToSubtract() -> u8 {
    let mut count: u8 = 0;
    let mut i: u8 = 0;
    count = CalculatePlayerPartyCount();
    i = 0;
    while i < count {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0 {
            let mut ability: u8 = GetMonAbility(&raw mut gPlayerParty[i]);
            if ability == ABILITY_MAGMA_ARMOR || ability == ABILITY_FLAME_BODY {
                return 2;
            }
        }
        i += 1;
    }
    return 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMons() -> u16 {
    let mut aliveNonEggMonsCount: u16 = CountStorageNonEggMons() as u16;
    aliveNonEggMonsCount += CountPartyAliveNonEggMonsExcept(PARTY_SIZE as u8) as u16;
    return aliveNonEggMonsCount;
}
