//! Translated from `src/pokemon_jump.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_labels,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, HideBg,
    IsDma3ManagerBusyWithBgCopy, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::digit_obj_util::{
    DigitObjUtil_CreatePrinter, DigitObjUtil_Free, DigitObjUtil_Init, DigitObjUtil_PrintNumOn,
};
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_Reset;
use crate::ffi::gSpecialVar_Result;
use crate::item::{AddBagItem, CheckBagHasSpace, CopyItemName, CopyItemNameHandlePlural};
use crate::link::gRecvCmds;
use crate::link::{
    GetLinkPlayerCount, GetMultiplayerId, IsLinkTaskFinished, SetCloseLinkCallback, gLinkPlayers,
    gReceivedRemoteLinkPlayers,
};
use crate::link_rfu_2::{Rfu_SendPacket, gRfu};
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterParameterized3, CreateYesNoMenu, DecompressAndCopyTileDataToVram,
    EraseYesNoWindow, FreeTempTileDataBuffersIfPossible, Menu_ProcessInputNoWrapClearOnChoose,
    ResetBgPositions, ResetTempTileDataBuffers,
};
use crate::minigame_countdown::{IsMinigameCountdownRunning, StartMinigameCountdown};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::pokemon::{GetMonData2, GetMonSpritePalFromSpeciesAndPersonality, gPlayerParty};
use crate::random::Random;
use crate::save::Task_LinkFullSave;
use crate::script::ScriptContext_Enable;
use crate::sound::{
    FadeOutAndPlayNewMapMusic, FadeOutMapMusic, IsFanfareTaskInactive, IsNotWaitingForBGMStop,
    PlayFanfare, PlaySE,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, IndexOfSpritePaletteTag, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::gStringVar1;
use crate::task::gTasks;
use crate::task::{DestroyTask, GetWordTaskArg, ResetTasks, RunTasks, SetWordTaskArg};
use crate::text_window::{
    DrawTextBorderOuter, LoadUserWindowBorderGfx_, LoadUserWindowBorderGfxOnBg, rbox_fill_rectangle,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap, RemoveWindow,
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
/// `DynamicPlaceholderTextUtil_ExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe {
        crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_ExpandPlaceholders(
            a0 as _, a1 as _,
        ) as *mut u8
    }
}
/// `DynamicPlaceholderTextUtil_SetPlaceholderPtr` with this module's view of its types.
#[inline]
unsafe fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8) {
    unsafe {
        crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            a0, a1 as _,
        );
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `HandleLoadSpecialPokePic` with this module's view of its types.
#[inline]
fn HandleLoadSpecialPokePic(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32, a3: u32) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic(a0 as _, a1 as _, a2, a3);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
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
// The C's names for task and sprite data slots.
const sState: usize = 0;
const sHopPos: usize = 1;
const sNumHops: usize = 2;
const sNumShakes: usize = 2;
const sOffset: usize = 7;
// Data tables (translate with cdata.py): sPokeJumpMons sPokeJumpLeaderFuncs sPokeJumpMemberFuncs sVineBaseSpeeds sVineSpeedDelays sSoundEffects sJumpOffsets sScoreBonuses sPrizeItems sPrizeQuantityData sPokeJumpPal1 sPokeJumpPal2 sVine1_Gfx sVine2_Gfx sVine3_Gfx sVine4_Gfx sStar_Gfx sCompressedSpriteSheets sSpritePalettes sOamData_JumpMon sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sSpriteTemplate_JumpMon sVineYCoords sVineXCoords sSpriteTemplates_Vine sOamData_JumpMon sOamData_Vine16x32 sOamData_Vine32x32 sOamData_Vine32x16 sAnims_Vine_Highest sAnims_Vine_Higher sAnims_Vine_High sAnims_Vine_Low sAnims_Vine_Lower sAnims_Vine_Lowest sAnims_VineTall_Highest sAnims_VineTall_Higher sAnims_VineTall_High sAnims_VineTall_Low sAnims_VineTall_Lower sAnims_VineTall_Lowest sAnims_Vine sAnims_VineTall sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sOamData_Star sAnim_Star_Still sAnim_Star_Spinning sAnims_Star sSpriteTemplate_Star sInterface_Pal sBg_Pal sBg_Gfx sBg_Tilemap sVenusaur_Pal sVenusaur_Gfx sVenusaur_Tilemap sBonuses_Pal sBonuses_Gfx sBonuses_Tilemap sBgTemplates sWindowTemplates sPokeJumpGfxFuncs sVenusaurStates sSpriteSheet_Digits sSpritePalette_Digits sPlayerNameWindowCoords_2Players sPlayerNameWindowCoords_3Players sPlayerNameWindowCoords_4Players sPlayerNameWindowCoords_5Players sPlayerNameWindowCoords sMonXCoords_2Players sMonXCoords_3Players sMonXCoords_4Players sMonXCoords_5Players sMonXCoords sWindowTemplate_Records sRecordsTexts

/// `struct PokemonJump`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump {
    pub exitCallback: Option<unsafe fn()>,
    pub taskId: u8,
    pub numPlayers: u8,
    pub multiplayerId: u8,
    pub startDelayTimer: u8,
    pub mainState: u16,
    pub helperState: u16,
    pub excellentsInRow: u16,
    pub excellentsInRowRecord: u16,
    pub gameOver: u32,
    pub vineState: u32,
    pub prevVineState: u32,
    pub vineSpeed: i32,
    pub vineSpeedAccel: u32,
    pub rngSeed: u32,
    pub nextVineSpeed: u32,
    pub linkTimer: i32,
    pub linkTimerLimit: u32,
    pub vineStateTimer: u16,
    pub ignoreJumpInput: u16,
    pub unused1: u16,
    pub unused2: u16,
    pub timer: u16,
    pub prizeItemId: u16,
    pub prizeItemQuantity: u16,
    pub playAgainComm: u16,
    pub unused3: u8,
    pub playAgainState: u8,
    pub allowVineUpdates: u8,
    pub isLeader: u8,
    pub funcActive: u8,
    pub allPlayersReady: u8,
    pub vineTimer: u16,
    pub nextFuncId: u8,
    pub showBonus: u8,
    pub vineSpeedDelay: u16,
    pub vineBaseSpeedIdx: u8,
    pub vineSpeedStage: u8,
    pub numPlayersAtPeak: i32,
    pub initScoreUpdate: u32,
    pub updateScore: u32,
    pub unused4: u32,
    pub giveBonus: u32,
    pub skipJumpUpdate: u32,
    pub atMaxSpeedStage: u32,
    pub comm: PokemonJump_CommData,
    pub atJumpPeak: CArray<u8, 5>,
    pub atJumpPeak2: CArray<u8, 5>,
    pub atJumpPeak3: CArray<u8, 5>,
    pub memberFuncIds: CArray<u8, 5>,
    pub playAgainStates: CArray<u16, 5>,
    pub jumpTimeStarts: CArray<u16, 5>,
    pub jumpGfx: PokemonJumpGfx,
    pub monInfo: CArray<PokemonJump_MonInfo, 5>,
    pub players: CArray<PokemonJump_Player, 5>,
    pub player: *mut PokemonJump_Player,
}

unsafe impl Sync for PokemonJump {}

/// `struct PokemonJumpGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJumpGfx {
    pub funcFinished: u32,
    pub mainState: u16,
    pub taskId: u8,
    pub unused1: CArray<u8, 3>,
    pub resetVineState: u8,
    pub resetVineTimer: u8,
    pub vineState: u8,
    pub msgWindowState: u8,
    pub vinePalNumDownswing: u8,
    pub vinePalNumUpswing: u8,
    pub unused2: u16,
    pub msgWindowId: u16,
    pub fanfare: u16,
    pub bonusTimer: u32,
    pub nameWindowIds: CArray<u16, 5>,
    pub itemName: CArray<u8, 64>,
    pub itemQuantityStr: CArray<u8, 64>,
    pub prizeMsg: CArray<u8, 256>,
    pub tilemapBuffer: CArray<u16, 16384>,
    pub monSprites: CArray<*mut Sprite, 5>,
    pub starSprites: CArray<*mut Sprite, 5>,
    pub vineSprites: CArray<*mut Sprite, 8>,
    pub unused3: CArray<u8, 12>,
    pub monSpriteSubpriorities: CArray<u8, 5>,
}

unsafe impl Sync for PokemonJumpGfx {}

/// `struct PokemonJump_MonInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_MonInfo {
    pub species: u16,
    pub otId: u32,
    pub personality: u32,
}

unsafe impl Sync for PokemonJump_MonInfo {}

/// `struct PokemonJump_Player`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_Player {
    pub jumpOffset: i32,
    pub jumpOffsetIdx: i32,
    pub unused: u32,
    pub monJumpType: u16,
    pub jumpTimeStart: u16,
    pub monState: u16,
    pub prevMonState: u16,
    pub jumpState: i32,
    pub funcFinished: u32,
    pub name: CArray<u8, 11>,
}

unsafe impl Sync for PokemonJump_Player {}

/// `struct PokemonJump_CommData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonJump_CommData {
    pub funcId: u8,
    pub receivedBonusFlags: u8,
    pub data: u16,
    pub jumpsInRow: u16,
    pub jumpScore: u32,
}

unsafe impl Sync for PokemonJump_CommData {}

/// `struct MonInfoPacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MonInfoPacket {
    pub id: u8,
    pub species: u16,
    pub personality: u32,
    pub otId: u32,
}

unsafe impl Sync for MonInfoPacket {}

/// `struct UnusedPacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnusedPacket {
    pub id: u8,
    pub data: u32,
    pub filler: u32,
}

unsafe impl Sync for UnusedPacket {}

/// `struct LeaderStatePacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LeaderStatePacket {
    pub id: u8,
    pub funcId: u8,
    pub monState: u8,
    bits_3: u8,
    pub jumpTimeStart: u16,
    pub vineTimer: u16,
    bits_8: u32,
}

impl LeaderStatePacket {
    #[inline(always)]
    pub fn receivedBonusFlags(&self) -> u8 {
        ((self.bits_3 as u32) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_receivedBonusFlags(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !0x1f) | (v & 0x1f);
    }
    #[inline(always)]
    pub fn jumpState(&self) -> u8 {
        ((self.bits_3 as u32 >> 5) & 0x7) as u8
    }
    #[inline(always)]
    pub fn set_jumpState(&mut self, v: u8) {
        self.bits_3 = (self.bits_3 & !(0x7 << 5)) | ((v & 0x7) << 5);
    }
    #[inline(always)]
    pub fn jumpsInRow(&self) -> u32 {
        self.bits_8 & 0x7fff
    }
    #[inline(always)]
    pub fn set_jumpsInRow(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !0x7fff) | (v & 0x7fff);
    }
    #[inline(always)]
    pub fn jumpScore(&self) -> u32 {
        (self.bits_8 >> 15) & 0x1ffff
    }
    #[inline(always)]
    pub fn set_jumpScore(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x1ffff << 15)) | ((v & 0x1ffff) << 15);
    }
}

unsafe impl Sync for LeaderStatePacket {}

/// `struct MemberStatePacket`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MemberStatePacket {
    pub id: u8,
    pub monState: u8,
    pub jumpState: u8,
    pub funcFinished: u8,
    pub jumpTimeStart: u16,
    pub funcId: u8,
    pub playAgainState: u16,
}

unsafe impl Sync for MemberStatePacket {}

/// `__typeof__(sPokeJumpGfxFuncs[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPokeJumpGfxFuncs_0_t {
    pub id: i32,
    pub func: Option<unsafe fn()>,
}

unsafe impl Sync for sPokeJumpGfxFuncs_0_t {}

/// `struct PokemonJumpMons`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PokemonJumpMons {
    pub species: u16,
    pub jumpType: u16,
}

unsafe impl Sync for PokemonJumpMons {}

/// `__typeof__(sPrizeQuantityData[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPrizeQuantityData_0_t {
    pub score: u32,
    pub quantity: u32,
}

unsafe impl Sync for sPrizeQuantityData_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokemonJump>() == 33712);
    assert!(offset_of!(PokemonJump, exitCallback) == 0);
    assert!(offset_of!(PokemonJump, taskId) == 4);
    assert!(offset_of!(PokemonJump, numPlayers) == 5);
    assert!(offset_of!(PokemonJump, multiplayerId) == 6);
    assert!(offset_of!(PokemonJump, startDelayTimer) == 7);
    assert!(offset_of!(PokemonJump, mainState) == 8);
    assert!(offset_of!(PokemonJump, helperState) == 10);
    assert!(offset_of!(PokemonJump, excellentsInRow) == 12);
    assert!(offset_of!(PokemonJump, excellentsInRowRecord) == 14);
    assert!(offset_of!(PokemonJump, gameOver) == 16);
    assert!(offset_of!(PokemonJump, vineState) == 20);
    assert!(offset_of!(PokemonJump, prevVineState) == 24);
    assert!(offset_of!(PokemonJump, vineSpeed) == 28);
    assert!(offset_of!(PokemonJump, vineSpeedAccel) == 32);
    assert!(offset_of!(PokemonJump, rngSeed) == 36);
    assert!(offset_of!(PokemonJump, nextVineSpeed) == 40);
    assert!(offset_of!(PokemonJump, linkTimer) == 44);
    assert!(offset_of!(PokemonJump, linkTimerLimit) == 48);
    assert!(offset_of!(PokemonJump, vineStateTimer) == 52);
    assert!(offset_of!(PokemonJump, ignoreJumpInput) == 54);
    assert!(offset_of!(PokemonJump, unused1) == 56);
    assert!(offset_of!(PokemonJump, unused2) == 58);
    assert!(offset_of!(PokemonJump, timer) == 60);
    assert!(offset_of!(PokemonJump, prizeItemId) == 62);
    assert!(offset_of!(PokemonJump, prizeItemQuantity) == 64);
    assert!(offset_of!(PokemonJump, playAgainComm) == 66);
    assert!(offset_of!(PokemonJump, unused3) == 68);
    assert!(offset_of!(PokemonJump, playAgainState) == 69);
    assert!(offset_of!(PokemonJump, allowVineUpdates) == 70);
    assert!(offset_of!(PokemonJump, isLeader) == 71);
    assert!(offset_of!(PokemonJump, funcActive) == 72);
    assert!(offset_of!(PokemonJump, allPlayersReady) == 73);
    assert!(offset_of!(PokemonJump, vineTimer) == 74);
    assert!(offset_of!(PokemonJump, nextFuncId) == 76);
    assert!(offset_of!(PokemonJump, showBonus) == 77);
    assert!(offset_of!(PokemonJump, vineSpeedDelay) == 78);
    assert!(offset_of!(PokemonJump, vineBaseSpeedIdx) == 80);
    assert!(offset_of!(PokemonJump, vineSpeedStage) == 81);
    assert!(offset_of!(PokemonJump, numPlayersAtPeak) == 84);
    assert!(offset_of!(PokemonJump, initScoreUpdate) == 88);
    assert!(offset_of!(PokemonJump, updateScore) == 92);
    assert!(offset_of!(PokemonJump, unused4) == 96);
    assert!(offset_of!(PokemonJump, giveBonus) == 100);
    assert!(offset_of!(PokemonJump, skipJumpUpdate) == 104);
    assert!(offset_of!(PokemonJump, atMaxSpeedStage) == 108);
    assert!(offset_of!(PokemonJump, comm) == 112);
    assert!(offset_of!(PokemonJump, atJumpPeak) == 124);
    assert!(offset_of!(PokemonJump, atJumpPeak2) == 129);
    assert!(offset_of!(PokemonJump, atJumpPeak3) == 134);
    assert!(offset_of!(PokemonJump, memberFuncIds) == 139);
    assert!(offset_of!(PokemonJump, playAgainStates) == 144);
    assert!(offset_of!(PokemonJump, jumpTimeStarts) == 154);
    assert!(offset_of!(PokemonJump, jumpGfx) == 164);
    assert!(offset_of!(PokemonJump, monInfo) == 33448);
    assert!(offset_of!(PokemonJump, players) == 33508);
    assert!(offset_of!(PokemonJump, player) == 33708);
    assert!(size_of::<PokemonJumpGfx>() == 33284);
    assert!(offset_of!(PokemonJumpGfx, funcFinished) == 0);
    assert!(offset_of!(PokemonJumpGfx, mainState) == 4);
    assert!(offset_of!(PokemonJumpGfx, taskId) == 6);
    assert!(offset_of!(PokemonJumpGfx, unused1) == 7);
    assert!(offset_of!(PokemonJumpGfx, resetVineState) == 10);
    assert!(offset_of!(PokemonJumpGfx, resetVineTimer) == 11);
    assert!(offset_of!(PokemonJumpGfx, vineState) == 12);
    assert!(offset_of!(PokemonJumpGfx, msgWindowState) == 13);
    assert!(offset_of!(PokemonJumpGfx, vinePalNumDownswing) == 14);
    assert!(offset_of!(PokemonJumpGfx, vinePalNumUpswing) == 15);
    assert!(offset_of!(PokemonJumpGfx, unused2) == 16);
    assert!(offset_of!(PokemonJumpGfx, msgWindowId) == 18);
    assert!(offset_of!(PokemonJumpGfx, fanfare) == 20);
    assert!(offset_of!(PokemonJumpGfx, bonusTimer) == 24);
    assert!(offset_of!(PokemonJumpGfx, nameWindowIds) == 28);
    assert!(offset_of!(PokemonJumpGfx, itemName) == 38);
    assert!(offset_of!(PokemonJumpGfx, itemQuantityStr) == 102);
    assert!(offset_of!(PokemonJumpGfx, prizeMsg) == 166);
    assert!(offset_of!(PokemonJumpGfx, tilemapBuffer) == 422);
    assert!(offset_of!(PokemonJumpGfx, monSprites) == 33192);
    assert!(offset_of!(PokemonJumpGfx, starSprites) == 33212);
    assert!(offset_of!(PokemonJumpGfx, vineSprites) == 33232);
    assert!(offset_of!(PokemonJumpGfx, unused3) == 33264);
    assert!(offset_of!(PokemonJumpGfx, monSpriteSubpriorities) == 33276);
    assert!(size_of::<PokemonJump_MonInfo>() == 12);
    assert!(offset_of!(PokemonJump_MonInfo, species) == 0);
    assert!(offset_of!(PokemonJump_MonInfo, otId) == 4);
    assert!(offset_of!(PokemonJump_MonInfo, personality) == 8);
    assert!(size_of::<PokemonJump_Player>() == 40);
    assert!(offset_of!(PokemonJump_Player, jumpOffset) == 0);
    assert!(offset_of!(PokemonJump_Player, jumpOffsetIdx) == 4);
    assert!(offset_of!(PokemonJump_Player, unused) == 8);
    assert!(offset_of!(PokemonJump_Player, monJumpType) == 12);
    assert!(offset_of!(PokemonJump_Player, jumpTimeStart) == 14);
    assert!(offset_of!(PokemonJump_Player, monState) == 16);
    assert!(offset_of!(PokemonJump_Player, prevMonState) == 18);
    assert!(offset_of!(PokemonJump_Player, jumpState) == 20);
    assert!(offset_of!(PokemonJump_Player, funcFinished) == 24);
    assert!(offset_of!(PokemonJump_Player, name) == 28);
    assert!(size_of::<PokemonJump_CommData>() == 12);
    assert!(offset_of!(PokemonJump_CommData, funcId) == 0);
    assert!(offset_of!(PokemonJump_CommData, receivedBonusFlags) == 1);
    assert!(offset_of!(PokemonJump_CommData, data) == 2);
    assert!(offset_of!(PokemonJump_CommData, jumpsInRow) == 4);
    assert!(offset_of!(PokemonJump_CommData, jumpScore) == 8);
    assert!(size_of::<MonInfoPacket>() == 12);
    assert!(offset_of!(MonInfoPacket, id) == 0);
    assert!(offset_of!(MonInfoPacket, species) == 2);
    assert!(offset_of!(MonInfoPacket, personality) == 4);
    assert!(offset_of!(MonInfoPacket, otId) == 8);
    assert!(size_of::<UnusedPacket>() == 12);
    assert!(offset_of!(UnusedPacket, id) == 0);
    assert!(offset_of!(UnusedPacket, data) == 4);
    assert!(offset_of!(UnusedPacket, filler) == 8);
    assert!(size_of::<LeaderStatePacket>() == 12);
    assert!(offset_of!(LeaderStatePacket, id) == 0);
    assert!(offset_of!(LeaderStatePacket, funcId) == 1);
    assert!(offset_of!(LeaderStatePacket, monState) == 2);
    assert!(offset_of!(LeaderStatePacket, bits_3) == 3);
    assert!(offset_of!(LeaderStatePacket, jumpTimeStart) == 4);
    assert!(offset_of!(LeaderStatePacket, vineTimer) == 6);
    assert!(offset_of!(LeaderStatePacket, bits_8) == 8);
    assert!(size_of::<MemberStatePacket>() == 12);
    assert!(offset_of!(MemberStatePacket, id) == 0);
    assert!(offset_of!(MemberStatePacket, monState) == 1);
    assert!(offset_of!(MemberStatePacket, jumpState) == 2);
    assert!(offset_of!(MemberStatePacket, funcFinished) == 3);
    assert!(offset_of!(MemberStatePacket, jumpTimeStart) == 4);
    assert!(offset_of!(MemberStatePacket, funcId) == 6);
    assert!(offset_of!(MemberStatePacket, playAgainState) == 8);
    assert!(size_of::<sPokeJumpGfxFuncs_0_t>() == 8);
    assert!(offset_of!(sPokeJumpGfxFuncs_0_t, id) == 0);
    assert!(offset_of!(sPokeJumpGfxFuncs_0_t, func) == 4);
    assert!(size_of::<PokemonJumpMons>() == 4);
    assert!(offset_of!(PokemonJumpMons, species) == 0);
    assert!(offset_of!(PokemonJumpMons, jumpType) == 2);
    assert!(size_of::<sPrizeQuantityData_0_t>() == 8);
    assert!(offset_of!(sPrizeQuantityData_0_t, score) == 0);
    assert!(offset_of!(sPrizeQuantityData_0_t, quantity) == 4);
};

const BG_BONUSES: u8 = 1;
const BG_INTERFACE: u8 = 0;
const BG_SCENERY: u8 = 3;
const BG_VENUSAUR: u8 = 2;
const DATAIDX_GAME_STRUCT: u8 = 14;
const FUNC_ASK_PLAY_AGAIN: u8 = 4;
const FUNC_EXIT: u8 = 6;
const FUNC_GAME_INTRO: u8 = 0;
const FUNC_GAME_OVER: u8 = 3;
const FUNC_GAME_ROUND: u8 = 2;
const FUNC_GIVE_PRIZE: u8 = 7;
const FUNC_NONE: u8 = 9;
const FUNC_RESET_GAME: u8 = 5;
const FUNC_SAVE: u8 = 8;
const FUNC_WAIT_ROUND: u8 = 1;
const F_SE_FAIL: i32 = 2;
const F_SE_JUMP: i32 = 1;
const GFXFUNC_COUNTDOWN: i32 = 9;
const GFXFUNC_ERASE_MSG: i32 = 6;
const GFXFUNC_ERASE_NAMES: i32 = 3;
const GFXFUNC_MSG_COMM_STANDBY: i32 = 8;
const GFXFUNC_MSG_PLAYER_DROPPED: i32 = 7;
const GFXFUNC_MSG_PLAY_AGAIN: i32 = 4;
const GFXFUNC_MSG_SAVING: i32 = 5;
const GFXFUNC_SHOW_NAMES: i32 = 1;
const GFXFUNC_SHOW_NAMES_HIGHLIGHT: i32 = 2;
const GFXTAG_COUNTDOWN: u16 = 9;
const JUMPSTATE_FAILURE: i32 = 2;
const JUMPSTATE_NONE: i32 = 0;
const JUMPSTATE_SUCCESS: i32 = 1;
const JUMP_PEAK: i32 = -30;
const LINK_INTERVAL_LONG: i32 = 5;
const LINK_INTERVAL_MEDIUM: i32 = 4;
const LINK_INTERVAL_NONE: i32 = 0;
const LINK_INTERVAL_SHORT: i32 = 3;
const LINK_TIMER_STOPPED: u32 = 4369;
const MAX_JUMPS: u16 = 9999;
const MAX_JUMP_SCORE: u32 = 0x18696;
const MONSTATE_HIT: u16 = 2;
const MONSTATE_JUMP: u16 = 1;
const MONSTATE_NORMAL: u16 = 0;
const NUM_VINESTATES: i32 = 10;
const NUM_WINDOWS: u32 = 2;
const PACKET_LEADER_STATE: u8 = 3;
const PACKET_MEMBER_STATE: u8 = 4;
const PACKET_MON_INFO: u8 = 1;
const PACKET_UNUSED: u8 = 2;
const PALTAG_1: u16 = 5;
const PALTAG_2: u16 = 6;
const PALTAG_COUNTDOWN: u16 = 7;
const PLAY_AGAIN_NO: u8 = 1;
const PLAY_AGAIN_YES: u8 = 2;
const VINE_HIGHEST: u32 = 0;
const VINE_LOWEST: i32 = 5;
const VINE_SPRITES_PER_SIDE: i32 = 4;
const VINE_UPSWING_HIGH: u32 = 8;
const VINE_UPSWING_LOW: u32 = 7;
const VINE_UPSWING_LOWER: u32 = 6;
const WIN_POINTS: u8 = 0;
const WIN_TIMES: u8 = 1;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokemon_jump::sBgTemplates).cast());
static sBg_Gfx: Table<CArray<u32, 139>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Gfx).cast());
static sBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Pal).cast());
static sBg_Tilemap: Table<CArray<u32, 108>> =
    Table((&raw const crate::data::pokemon_jump::sBg_Tilemap).cast());
static sBonuses_Gfx: Table<CArray<u32, 684>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Gfx).cast());
static sBonuses_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Pal).cast());
static sBonuses_Tilemap: Table<CArray<u32, 331>> =
    Table((&raw const crate::data::pokemon_jump::sBonuses_Tilemap).cast());
static sCompressedSpriteSheets: Table<CArray<CompressedSpriteSheet, 5>> =
    Table((&raw const crate::data::pokemon_jump::sCompressedSpriteSheets).cast());
static sInterface_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sInterface_Pal).cast());
static sJumpOffsets: Table<CArray<CArray<i8, 48>, 3>> =
    Table((&raw const crate::data::pokemon_jump::sJumpOffsets).cast());
static sMonXCoords: Table<CArray<*mut i16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sMonXCoords).cast());
static sPlayerNameWindowCoords: Table<CArray<*mut u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sPlayerNameWindowCoords).cast());
static sPokeJumpGfxFuncs: Table<CArray<sPokeJumpGfxFuncs_0_t, 10>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpGfxFuncs).cast());
static sPokeJumpLeaderFuncs: Table<CArray<Option<unsafe fn() -> u32>, 9>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpLeaderFuncs).cast());
static sPokeJumpMemberFuncs: Table<CArray<Option<unsafe fn() -> u32>, 9>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpMemberFuncs).cast());
static sPokeJumpMons: Table<CArray<PokemonJumpMons, 100>> =
    Table((&raw const crate::data::pokemon_jump::sPokeJumpMons).cast());
static sPrizeItems: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sPrizeItems).cast());
static sPrizeQuantityData: Table<CArray<sPrizeQuantityData_0_t, 5>> =
    Table((&raw const crate::data::pokemon_jump::sPrizeQuantityData).cast());
static sRecordsTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::pokemon_jump::sRecordsTexts).cast());
static sScoreBonuses: Table<CArray<i32, 6>> =
    Table((&raw const crate::data::pokemon_jump::sScoreBonuses).cast());
static sSoundEffects: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sSoundEffects).cast());
static sSpritePalette_Digits: Table<SpritePalette> =
    Table((&raw const crate::data::pokemon_jump::sSpritePalette_Digits).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokemon_jump::sSpritePalettes).cast());
static sSpriteSheet_Digits: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokemon_jump::sSpriteSheet_Digits).cast());
static sSpriteTemplate_JumpMon: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplate_JumpMon).cast());
static sSpriteTemplate_Star: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplate_Star).cast());
static sSpriteTemplates_Vine: Table<CArray<*mut SpriteTemplate, 4>> =
    Table((&raw const crate::data::pokemon_jump::sSpriteTemplates_Vine).cast());
static sVenusaurStates: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaurStates).cast());
static sVenusaur_Gfx: Table<CArray<u32, 596>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Gfx).cast());
static sVenusaur_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Pal).cast());
static sVenusaur_Tilemap: Table<CArray<u32, 238>> =
    Table((&raw const crate::data::pokemon_jump::sVenusaur_Tilemap).cast());
static sVineBaseSpeeds: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sVineBaseSpeeds).cast());
static sVineSpeedDelays: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::pokemon_jump::sVineSpeedDelays).cast());
static sVineXCoords: Table<CArray<i16, 8>> =
    Table((&raw const crate::data::pokemon_jump::sVineXCoords).cast());
static sVineYCoords: Table<CArray<CArray<i16, 10>, 4>> =
    Table((&raw const crate::data::pokemon_jump::sVineYCoords).cast());
static sWindowTemplate_Records: Table<WindowTemplate> =
    Table((&raw const crate::data::pokemon_jump::sWindowTemplate_Records).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::pokemon_jump::sWindowTemplates).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokemonJump: crate::global::Global<*mut PokemonJump> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokemonJumpGfx: crate::global::Global<*mut PokemonJumpGfx> =
    crate::global::Global::new(null_mut());

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

pub unsafe fn StartPokemonJump(partyId: u16, exitCallback: Option<unsafe fn()>) {
    let mut taskId: u8 = 0;
    if gReceivedRemoteLinkPlayers.get() != 0 {
        crate::gl!(sPokemonJump = Alloc(33712) as *mut PokemonJump);
        if !(*sPokemonJump.as_ptr()).is_null() {
            ResetTasks();
            taskId = CreateTask(Some(Task_StartPokemonJump), 1);
            (*crate::gl!(sPokemonJump)).mainState = 0;
            (*crate::gl!(sPokemonJump)).exitCallback = exitCallback;
            (*crate::gl!(sPokemonJump)).taskId = taskId;
            (*crate::gl!(sPokemonJump)).multiplayerId = GetMultiplayerId();
            InitJumpMonInfo(
                &raw mut (*crate::gl!(sPokemonJump)).monInfo
                    [(*crate::gl!(sPokemonJump)).multiplayerId],
                crate::gl!(&raw gPlayerParty[partyId]),
            );
            InitGame(crate::gl!(sPokemonJump));
            SetWordTaskArg(taskId, 2, crate::gl!(sPokemonJump) as usize as u32);
            SetMainCallback2(Some(CB2_PokemonJump));
            return;
        }
    }
    SetMainCallback2(exitCallback);
}
unsafe fn FreePokemonJump() {
    FreeWindowsAndDigitObj();
    Free(crate::gl!(sPokemonJump) as *mut c_void);
}
unsafe fn InitGame(jump: *mut PokemonJump) {
    (*jump).numPlayers = GetLinkPlayerCount();
    (*jump).comm.funcId = FUNC_RESET_GAME;
    (*jump).comm.data = 0;
    InitPlayerAndJumpTypes();
    ResetForNewGame(jump);
    if (*jump).numPlayers == MAX_RFU_PLAYERS as u8 {
        IncrementGamesWithMaxPlayers();
    }
}
unsafe fn ResetForNewGame(jump: *mut PokemonJump) {
    (*jump).vineState = VINE_UPSWING_LOWER;
    (*jump).prevVineState = VINE_UPSWING_LOWER;
    (*jump).vineTimer = 0;
    (*jump).vineSpeed = 0;
    (*jump).updateScore = FALSE as u32;
    (*jump).isLeader = (GetMultiplayerId() == 0) as u8;
    (*jump).mainState = 0;
    (*jump).helperState = 0;
    (*jump).excellentsInRow = 0;
    (*jump).excellentsInRowRecord = 0;
    (*jump).initScoreUpdate = FALSE as u32;
    (*jump).unused2 = 0;
    (*jump).unused3 = 0;
    (*jump).numPlayersAtPeak = 0;
    (*jump).allowVineUpdates = FALSE;
    (*jump).allPlayersReady = FALSE;
    (*jump).funcActive = TRUE;
    (*jump).comm.jumpScore = 0;
    (*jump).comm.receivedBonusFlags = 0;
    (*jump).comm.jumpsInRow = 0;
    (*jump).unused4 = TRUE as u32;
    (*jump).showBonus = FALSE;
    (*jump).skipJumpUpdate = FALSE as u32;
    (*jump).giveBonus = FALSE as u32;
    (*jump).linkTimer = 0;
    (*jump).linkTimerLimit = 0;
    ResetPlayersForNewGame();
    ResetPlayersJumpStates();
    for i in 0..MAX_RFU_PLAYERS {
        (*jump).atJumpPeak[i] = FALSE;
        (*jump).jumpTimeStarts[i] = 0;
    }
}
unsafe fn InitPlayerAndJumpTypes() {
    let mut index: i32 = 0;
    for i in 0..MAX_RFU_PLAYERS {
        index = GetPokemonJumpSpeciesIdx((*crate::gl!(sPokemonJump)).monInfo[i].species) as i32;
        (*crate::gl!(sPokemonJump)).players[i].monJumpType = sPokeJumpMons[index].jumpType;
    }
    (*crate::gl!(sPokemonJump)).player =
        &raw mut (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId];
}
unsafe fn ResetPlayersForNewGame() {
    for i in 0..MAX_RFU_PLAYERS {
        (*crate::gl!(sPokemonJump)).players[i].jumpTimeStart = 0;
        (*crate::gl!(sPokemonJump)).players[i].monState = MONSTATE_NORMAL;
        (*crate::gl!(sPokemonJump)).players[i].prevMonState = MONSTATE_NORMAL;
        (*crate::gl!(sPokemonJump)).players[i].jumpOffset = 0;
        (*crate::gl!(sPokemonJump)).players[i].jumpOffsetIdx = 2147483647;
        (*crate::gl!(sPokemonJump)).players[i].jumpState = JUMPSTATE_NONE;
        (*crate::gl!(sPokemonJump)).memberFuncIds[i] = FUNC_NONE;
    }
}
unsafe fn GetPokemonJumpSpeciesIdx(species: u16) -> i16 {
    for i in 0..100u32 {
        if sPokeJumpMons[i].species == species {
            return i as i16;
        }
    }
    -1
}
unsafe fn InitJumpMonInfo(monInfo: *mut PokemonJump_MonInfo, mon: *mut Pokemon) {
    (*monInfo).species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
    (*monInfo).otId = GetMonData2(mon, MON_DATA_OT_ID);
    (*monInfo).personality = GetMonData2(mon, MON_DATA_PERSONALITY);
}
pub(crate) unsafe fn VBlankCB_PokemonJump() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe fn CB2_PokemonJump() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn SetPokeJumpTask(func: Option<unsafe fn(u8)>) {
    (*crate::gl!(sPokemonJump)).taskId = CreateTask(func, 1);
    (*crate::gl!(sPokemonJump)).mainState = 0;
}
pub(crate) unsafe fn Task_StartPokemonJump(taskId: u8) {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetTaskWithPokeJumpStruct(Some(Task_CommunicateMonInfo), 5);
            FadeOutMapMusic(4);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        1 => {
            if FuncIsActiveTask(Some(Task_CommunicateMonInfo)) == 0 {
                StartPokeJumpGfx(&raw mut (*crate::gl!(sPokemonJump)).jumpGfx);
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        2 => {
            if IsPokeJumpGfxFuncFinished() == 0 && IsNotWaitingForBGMStop() == TRUE {
                FadeOutAndPlayNewMapMusic(MUS_RG_POKE_JUMP, 8);
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                BlendPalettes(PALETTES_ALL, 16, 0);
                BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
                SetVBlankCallback(Some(VBlankCB_PokemonJump));
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        4 => {
            UpdatePaletteFade();
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sPokemonJump)).startDelayTimer = 0;
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        5 => {
            (*crate::gl!(sPokemonJump)).startDelayTimer += 1;
            if (*crate::gl!(sPokemonJump)).startDelayTimer >= 20 {
                if (*crate::gl!(sPokemonJump)).isLeader != 0 {
                    SetPokeJumpTask(Some(Task_PokemonJump_Leader));
                } else {
                    SetPokeJumpTask(Some(Task_PokemonJump_Member));
                }
                InitVineState();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
unsafe fn SetLinkTimeInterval(intervalId: i32) {
    if intervalId == LINK_INTERVAL_NONE {
        (*crate::gl!(sPokemonJump)).linkTimerLimit = LINK_TIMER_STOPPED;
        (*crate::gl!(sPokemonJump)).linkTimer = 1;
    } else {
        (*crate::gl!(sPokemonJump)).linkTimerLimit = shl_i32(1, intervalId as u32 - 1) as u32 - 1;
        (*crate::gl!(sPokemonJump)).linkTimer = 0;
    }
}
unsafe fn SetFunc_Leader(funcId: u8) {
    (*crate::gl!(sPokemonJump)).comm.funcId = funcId;
    (*crate::gl!(sPokemonJump)).mainState = 0;
    (*crate::gl!(sPokemonJump)).helperState = 0;
    (*crate::gl!(sPokemonJump)).funcActive = TRUE;
    (*crate::gl!(sPokemonJump)).allPlayersReady = FALSE;
    let mut i: i32 = 1;
    while i < (*crate::gl!(sPokemonJump)).numPlayers as i32 {
        (*crate::gl!(sPokemonJump)).players[i].funcFinished = FALSE as u32;
        i += 1;
    }
}
pub(crate) unsafe fn RecvLinkData_Leader() {
    let mut monState: u16 = 0;
    let mut funcId: u8 = 0;
    let mut playAgainState: u16 = 0;
    let mut i: i32 = 1;
    let mut numReady: i32 = 0;
    while i < (*crate::gl!(sPokemonJump)).numPlayers as i32 {
        monState = (*crate::gl!(sPokemonJump)).players[i].monState;
        if RecvPacket_MemberStateToLeader(
            &raw mut (*crate::gl!(sPokemonJump)).players[i],
            i,
            &raw mut funcId,
            &raw mut playAgainState,
        ) != 0
        {
            (*crate::gl!(sPokemonJump)).playAgainStates[i] = playAgainState;
            (*crate::gl!(sPokemonJump)).memberFuncIds[i] = funcId;
            (*crate::gl!(sPokemonJump)).players[i].prevMonState = monState;
        }
        if (*crate::gl!(sPokemonJump)).players[i].funcFinished != 0
            && (*crate::gl!(sPokemonJump)).memberFuncIds[i]
                == (*crate::gl!(sPokemonJump)).comm.funcId
        {
            numReady += 1;
        }
        i += 1;
    }
    if numReady == (*crate::gl!(sPokemonJump)).numPlayers as i32 - 1 {
        (*crate::gl!(sPokemonJump)).allPlayersReady = TRUE;
    }
}
pub(crate) unsafe fn Task_PokemonJump_Leader(taskId: u8) {
    RecvLinkData_Leader();
    TryUpdateScore();
    if (*crate::gl!(sPokemonJump)).funcActive == 0
        && (*crate::gl!(sPokemonJump)).allPlayersReady != 0
    {
        SetFunc_Leader((*crate::gl!(sPokemonJump)).nextFuncId);
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
    }
    if (*crate::gl!(sPokemonJump)).funcActive == TRUE
        && sPokeJumpLeaderFuncs[(*crate::gl!(sPokemonJump)).comm.funcId].unwrap_unchecked()() == 0
    {
        (*crate::gl!(sPokemonJump)).funcActive = FALSE;
        (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId]
            .funcFinished = TRUE as u32;
    }
    UpdateGame();
    SendLinkData_Leader();
}
pub(crate) unsafe fn SendLinkData_Leader() {
    if (*crate::gl!(sPokemonJump)).linkTimer == 0 {
        SendPacket_LeaderState(
            (*crate::gl!(sPokemonJump)).players.as_mut_ptr(),
            &raw mut (*crate::gl!(sPokemonJump)).comm,
        );
    }
    if (*crate::gl!(sPokemonJump)).linkTimerLimit != LINK_TIMER_STOPPED {
        (*crate::gl!(sPokemonJump)).linkTimer += 1;
        (*crate::gl!(sPokemonJump)).linkTimer &= (*crate::gl!(sPokemonJump)).linkTimerLimit as i32;
    }
}
unsafe fn SetFunc_Member(funcId: u8) {
    (*crate::gl!(sPokemonJump)).comm.funcId = funcId;
    (*crate::gl!(sPokemonJump)).mainState = 0;
    (*crate::gl!(sPokemonJump)).helperState = 0;
    (*crate::gl!(sPokemonJump)).funcActive = TRUE;
    (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId].funcFinished =
        FALSE as u32;
}
pub(crate) unsafe fn RecvLinkData_Member() {
    let mut monState: u16 = 0;
    let mut leaderData: PokemonJump_CommData = zeroed();
    monState = (*crate::gl!(sPokemonJump)).players[0].monState;
    if RecvPacket_LeaderState(
        (*crate::gl!(sPokemonJump)).players.as_mut_ptr(),
        &raw mut leaderData,
    ) != 0
    {
        if (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId]
            .funcFinished
            == TRUE as u32
            && leaderData.funcId != (*crate::gl!(sPokemonJump)).comm.funcId
        {
            SetFunc_Member(leaderData.funcId);
        }
        if (*crate::gl!(sPokemonJump)).comm.jumpScore != leaderData.jumpScore {
            (*crate::gl!(sPokemonJump)).comm.jumpScore = leaderData.jumpScore;
            (*crate::gl!(sPokemonJump)).updateScore = TRUE as u32;
            (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags = leaderData.receivedBonusFlags;
            if (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags != 0 {
                (*crate::gl!(sPokemonJump)).showBonus = TRUE;
            } else {
                (*crate::gl!(sPokemonJump)).showBonus = FALSE;
            }
        }
        (*crate::gl!(sPokemonJump)).comm.data = leaderData.data;
        (*crate::gl!(sPokemonJump)).comm.jumpsInRow = leaderData.jumpsInRow;
        (*crate::gl!(sPokemonJump)).players[0].prevMonState = monState;
    }
    let mut i: i32 = 1;
    while i < (*crate::gl!(sPokemonJump)).numPlayers as i32 {
        if i != (*crate::gl!(sPokemonJump)).multiplayerId as i32 {
            monState = (*crate::gl!(sPokemonJump)).players[i].monState;
            if RecvPacket_MemberStateToMember(&raw mut (*crate::gl!(sPokemonJump)).players[i], i)
                != 0
            {
                (*crate::gl!(sPokemonJump)).players[i].prevMonState = monState;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe fn Task_PokemonJump_Member(taskId: u8) {
    RecvLinkData_Member();
    if (*crate::gl!(sPokemonJump)).funcActive != 0
        && sPokeJumpMemberFuncs[(*crate::gl!(sPokemonJump)).comm.funcId].unwrap_unchecked()() == 0
    {
        (*crate::gl!(sPokemonJump)).funcActive = FALSE;
        (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId]
            .funcFinished = TRUE as u32;
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
    }
    UpdateGame();
    SendLinkData_Member();
}
pub(crate) unsafe fn SendLinkData_Member() {
    if (*crate::gl!(sPokemonJump)).linkTimer == 0 {
        SendPacket_MemberState(
            &raw mut (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId],
            (*crate::gl!(sPokemonJump)).comm.funcId,
            (*crate::gl!(sPokemonJump)).playAgainComm,
        );
    }
    if (*crate::gl!(sPokemonJump)).linkTimerLimit != LINK_TIMER_STOPPED {
        (*crate::gl!(sPokemonJump)).linkTimer += 1;
        (*crate::gl!(sPokemonJump)).linkTimer &= (*crate::gl!(sPokemonJump)).linkTimerLimit as i32;
    }
}
pub(crate) unsafe fn GameIntro_Leader() -> u32 {
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        if fall || sw1 == 1 {
            if DoGameIntro() == 0 {
                (*crate::gl!(sPokemonJump)).comm.data = (*crate::gl!(sPokemonJump)).vineTimer;
                (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_WAIT_ROUND;
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn GameIntro_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*crate::gl!(sPokemonJump)).rngSeed = (*crate::gl!(sPokemonJump)).comm.data as u32;
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        if fall || sw1 == 1 {
            return DoGameIntro();
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn WaitRound_Leader() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            ResetPlayersJumpStates();
            SetLinkTimeInterval(LINK_INTERVAL_LONG);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        1 if (*crate::gl!(sPokemonJump)).allPlayersReady != 0 => {
            (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_GAME_ROUND;
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn WaitRound_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            ResetPlayersJumpStates();
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*crate::gl!(sPokemonJump)).vineTimer = (*crate::gl!(sPokemonJump)).comm.data;
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        if fall || sw1 == 1 {
            if AreLinkQueuesEmpty() != 0 {
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn GameRound_Leader() -> u32 {
    if HandleSwingRound() == 0 {
        (*crate::gl!(sPokemonJump)).comm.data = (*crate::gl!(sPokemonJump)).vineTimer;
        (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_WAIT_ROUND;
    } else if UpdateVineHitStates() != 0 {
        return TRUE as u32;
    } else {
        ResetVineAfterHit();
        (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_GAME_OVER;
    }
    FALSE as u32
}
pub(crate) unsafe fn GameRound_Member() -> u32 {
    if HandleSwingRound() == 0 {
    } else if UpdateVineHitStates() != 0 {
        return TRUE as u32;
    } else {
        ResetVineAfterHit();
    }
    FALSE as u32
}
pub(crate) unsafe fn GameOver_Leader() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            UpdateVineHitStates();
            if AllPlayersJumpedOrHit() != 0 {
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        1 => {
            if DoVineHitEffect() == 0 {
                if HasEnoughScoreForPrize() != 0 {
                    (*crate::gl!(sPokemonJump)).comm.data = GetPrizeData();
                    (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_GIVE_PRIZE;
                } else if (*crate::gl!(sPokemonJump)).comm.jumpsInRow >= 200 {
                    (*crate::gl!(sPokemonJump)).comm.data =
                        (*crate::gl!(sPokemonJump)).excellentsInRowRecord;
                    (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_SAVE;
                } else {
                    (*crate::gl!(sPokemonJump)).comm.data =
                        (*crate::gl!(sPokemonJump)).excellentsInRowRecord;
                    (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_ASK_PLAY_AGAIN;
                }
                (*crate::gl!(sPokemonJump)).mainState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn GameOver_Member() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            if UpdateVineHitStates() == 0 {
                ResetVineAfterHit();
            }
            if AllPlayersJumpedOrHit() != 0 {
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        1 => {
            if DoVineHitEffect() == 0 {
                (*crate::gl!(sPokemonJump)).mainState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn AskPlayAgain_Leader() -> u32 {
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_MEDIUM);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        if fall || sw1 == 1 {
            if DoPlayAgainPrompt() == 0 {
                TryUpdateRecords(
                    (*crate::gl!(sPokemonJump)).comm.jumpScore,
                    (*crate::gl!(sPokemonJump)).comm.jumpsInRow,
                    (*crate::gl!(sPokemonJump)).comm.data,
                );
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if (*crate::gl!(sPokemonJump)).allPlayersReady != 0 {
                if ShouldPlayAgain() != 0 {
                    (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_RESET_GAME;
                } else {
                    (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_EXIT;
                }
                (*crate::gl!(sPokemonJump)).mainState += 1;
                return FALSE as u32;
            }
            break 'l1;
        }
        if sw1 == 3 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn AskPlayAgain_Member() -> u32 {
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).mainState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        if fall || sw1 == 1 {
            if DoPlayAgainPrompt() == 0 {
                TryUpdateRecords(
                    (*crate::gl!(sPokemonJump)).comm.jumpScore,
                    (*crate::gl!(sPokemonJump)).comm.jumpsInRow,
                    (*crate::gl!(sPokemonJump)).comm.data,
                );
                (*crate::gl!(sPokemonJump)).playAgainComm =
                    (*crate::gl!(sPokemonJump)).playAgainState as u16;
                return FALSE as u32;
            }
            break 'l1;
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn ResetGame_Leader() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            if CloseMessageAndResetScore() == 0 {
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        1 if (*crate::gl!(sPokemonJump)).allPlayersReady != 0 => {
            ResetForNewGame(crate::gl!(sPokemonJump));
            (*crate::gl!(sPokemonJump)).rngSeed = Random() as u32;
            (*crate::gl!(sPokemonJump)).comm.data = (*crate::gl!(sPokemonJump)).rngSeed as u16;
            (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_GAME_INTRO;
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn ResetGame_Member() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            if CloseMessageAndResetScore() == 0 {
                ResetForNewGame(crate::gl!(sPokemonJump));
                (*crate::gl!(sPokemonJump)).mainState += 1;
                return FALSE as u32;
            }
        }
        1 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn ExitGame() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            (*crate::gl!(sPokemonJump)).mainState = 1;
        }
        1 => {
            SetLinkTimeInterval(LINK_INTERVAL_NONE);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        2 if ClosePokeJumpLink() == 0 => {
            SetMainCallback2((*crate::gl!(sPokemonJump)).exitCallback);
            FreePokemonJump();
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn GivePrize_Leader() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            SetLinkTimeInterval(LINK_INTERVAL_MEDIUM);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        1 if TryGivePrize() == 0 => {
            (*crate::gl!(sPokemonJump)).comm.data =
                (*crate::gl!(sPokemonJump)).excellentsInRowRecord;
            (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_SAVE;
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn GivePrize_Member() -> u32 {
    SetLinkTimeInterval(LINK_INTERVAL_NONE);
    if TryGivePrize() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn SavePokeJump() -> u32 {
    match (*crate::gl!(sPokemonJump)).mainState {
        0 => {
            TryUpdateRecords(
                (*crate::gl!(sPokemonJump)).comm.jumpScore,
                (*crate::gl!(sPokemonJump)).comm.jumpsInRow,
                (*crate::gl!(sPokemonJump)).comm.data,
            );
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_SAVING);
            (*crate::gl!(sPokemonJump)).mainState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                SetLinkTimeInterval(LINK_INTERVAL_NONE);
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        2 => {
            if AreLinkQueuesEmpty() != 0 {
                CreateTask(Some(Task_LinkFullSave), 6);
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        3 => {
            if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
                ClearMessageWindow();
                (*crate::gl!(sPokemonJump)).mainState += 1;
            }
        }
        4 if RemoveMessageWindow() == 0 => {
            (*crate::gl!(sPokemonJump)).nextFuncId = FUNC_ASK_PLAY_AGAIN;
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn DoGameIntro() -> u32 {
    match (*crate::gl!(sPokemonJump)).helperState {
        0 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_SHOW_NAMES_HIGHLIGHT);
            ResetMonSpriteSubpriorities();
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                StartMonIntroBounce((*crate::gl!(sPokemonJump)).multiplayerId as i32);
                (*crate::gl!(sPokemonJump)).timer = 0;
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        2 => {
            if ({
                (*crate::gl!(sPokemonJump)).timer += 1;
                (*crate::gl!(sPokemonJump)).timer
            }) > 120
            {
                SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_NAMES);
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() != TRUE as u32 && IsMonIntroBounceActive() != TRUE as i32
            {
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        4 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_COUNTDOWN);
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        5 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                DisallowVineUpdates();
                SetUpResetVineGfx();
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        6 => {
            if ResetVineGfx() == 0 {
                AllowVineUpdates();
                ResetVineState();
                (*crate::gl!(sPokemonJump)).helperState += 1;
                return FALSE as u32;
            }
        }
        7 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
unsafe fn HandleSwingRound() -> u32 {
    UpdateVineState();
    if (*crate::gl!(sPokemonJump)).ignoreJumpInput != 0 {
        (*crate::gl!(sPokemonJump)).ignoreJumpInput = FALSE as u16;
        return FALSE as u32;
    }
    'l1: {
        let sw1: u16 = (*crate::gl!(sPokemonJump)).helperState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsPlayersMonState(MONSTATE_NORMAL) != 0 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                SetMonStateJump();
                SetLinkTimeInterval(LINK_INTERVAL_SHORT);
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if IsPlayersMonState(MONSTATE_JUMP) == TRUE as u32 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if IsPlayersMonState(MONSTATE_NORMAL) == TRUE as u32 {
                (*crate::gl!(sPokemonJump)).helperState = 0;
            }
            break 'l1;
        }
    }
    TRUE as u32
}
unsafe fn DoVineHitEffect() -> u32 {
    let mut i: i32 = 0;
    match (*crate::gl!(sPokemonJump)).helperState {
        0 => {
            i = 0;
            while i < (*crate::gl!(sPokemonJump)).numPlayers as i32 {
                if IsMonHitShakeActive(i) == TRUE as i32 {
                    return TRUE as u32;
                }
                i += 1;
            }
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        1 => {
            i = 0;
            while i < (*crate::gl!(sPokemonJump)).numPlayers as i32 {
                if (*crate::gl!(sPokemonJump)).players[i].monState == MONSTATE_HIT {
                    StartMonHitFlash(i as u8);
                }
                i += 1;
            }
            SetUpPokeJumpGfxFuncById(GFXFUNC_SHOW_NAMES);
            (*crate::gl!(sPokemonJump)).timer = 0;
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        2 => {
            if ({
                (*crate::gl!(sPokemonJump)).timer += 1;
                (*crate::gl!(sPokemonJump)).timer
            }) > 100
            {
                SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_NAMES);
                (*crate::gl!(sPokemonJump)).timer = 0;
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                StopMonHitFlash();
                (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags = 0;
                ResetPlayersMonState();
                (*crate::gl!(sPokemonJump)).helperState += 1;
                return FALSE as u32;
            }
        }
        4 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn TryGivePrize() -> u32 {
    'l1: {
        match (*crate::gl!(sPokemonJump)).helperState {
            0 => {
                UnpackPrizeData(
                    (*crate::gl!(sPokemonJump)).comm.data,
                    &raw mut (*crate::gl!(sPokemonJump)).prizeItemId,
                    &raw mut (*crate::gl!(sPokemonJump)).prizeItemQuantity,
                );
                PrintPrizeMessage(
                    (*crate::gl!(sPokemonJump)).prizeItemId,
                    (*crate::gl!(sPokemonJump)).prizeItemQuantity,
                );
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
            1 | 4 => {
                if DoPrizeMessageAndFanfare() == 0 {
                    (*crate::gl!(sPokemonJump)).timer = 0;
                    (*crate::gl!(sPokemonJump)).helperState += 1;
                }
            }
            2 | 5 => {
                (*crate::gl!(sPokemonJump)).timer += 1;
                if crate::gl!(gMain.newKeys) as i32 & 3 != 0
                    || (*crate::gl!(sPokemonJump)).timer > 180
                {
                    ClearMessageWindow();
                    (*crate::gl!(sPokemonJump)).helperState += 1;
                }
            }
            3 => {
                if RemoveMessageWindow() == 0 {
                    (*crate::gl!(sPokemonJump)).prizeItemQuantity = GetQuantityLimitedByBag(
                        (*crate::gl!(sPokemonJump)).prizeItemId,
                        (*crate::gl!(sPokemonJump)).prizeItemQuantity,
                    );
                    if (*crate::gl!(sPokemonJump)).prizeItemQuantity != 0
                        && AddBagItem(
                            (*crate::gl!(sPokemonJump)).prizeItemId,
                            (*crate::gl!(sPokemonJump)).prizeItemQuantity,
                        ) != 0
                    {
                        if CheckBagHasSpace((*crate::gl!(sPokemonJump)).prizeItemId, 1) == 0 {
                            PrintPrizeFilledBagMessage((*crate::gl!(sPokemonJump)).prizeItemId);
                            (*crate::gl!(sPokemonJump)).helperState = 4;
                        } else {
                            (*crate::gl!(sPokemonJump)).helperState = 6;
                            break 'l1;
                        }
                    } else {
                        PrintNoRoomForPrizeMessage((*crate::gl!(sPokemonJump)).prizeItemId);
                        (*crate::gl!(sPokemonJump)).helperState = 4;
                    }
                }
            }
            6 if RemoveMessageWindow() == 0 => {
                return FALSE as u32;
            }
            _ => {}
        }
    }
    TRUE as u32
}
unsafe fn DoPlayAgainPrompt() -> u32 {
    let mut input: i8 = 0;
    match (*crate::gl!(sPokemonJump)).helperState {
        0 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_PLAY_AGAIN);
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        1 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        2 => {
            input = HandlePlayAgainInput();
            match input {
                MENU_B_PRESSED | 1 => {
                    (*crate::gl!(sPokemonJump)).playAgainState = PLAY_AGAIN_NO;
                    SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_MSG);
                    (*crate::gl!(sPokemonJump)).helperState += 1;
                }
                0 => {
                    (*crate::gl!(sPokemonJump)).playAgainState = PLAY_AGAIN_YES;
                    SetUpPokeJumpGfxFuncById(GFXFUNC_ERASE_MSG);
                    (*crate::gl!(sPokemonJump)).helperState += 1;
                }
                _ => {}
            }
        }
        3 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        4 => {
            SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_COMM_STANDBY);
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        5 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
                return FALSE as u32;
            }
        }
        6 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
unsafe fn ClosePokeJumpLink() -> u32 {
    match (*crate::gl!(sPokemonJump)).helperState {
        0 => {
            ClearMessageWindow();
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        1 => {
            if RemoveMessageWindow() == 0 {
                SetUpPokeJumpGfxFuncById(GFXFUNC_MSG_PLAYER_DROPPED);
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        2 => {
            if IsPokeJumpGfxFuncFinished() == 0 {
                (*crate::gl!(sPokemonJump)).timer = 0;
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        3 => {
            if ({
                (*crate::gl!(sPokemonJump)).timer += 1;
                (*crate::gl!(sPokemonJump)).timer
            }) > 120
            {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        4 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                SetCloseLinkCallback();
                (*crate::gl!(sPokemonJump)).helperState += 1;
            }
        }
        5 if gReceivedRemoteLinkPlayers.get() == 0 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
unsafe fn CloseMessageAndResetScore() -> u32 {
    match (*crate::gl!(sPokemonJump)).helperState {
        0 => {
            ClearMessageWindow();
            PrintScore(0);
            (*crate::gl!(sPokemonJump)).helperState += 1;
        }
        1 => {
            if RemoveMessageWindow() == 0 {
                (*crate::gl!(sPokemonJump)).helperState += 1;
                return FALSE as u32;
            }
        }
        2 => {
            return FALSE as u32;
        }
        _ => {}
    }
    TRUE as u32
}
pub(crate) unsafe fn Task_CommunicateMonInfo(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let jump: *mut PokemonJump =
        GetWordTaskArg(taskId, DATAIDX_GAME_STRUCT) as usize as *mut PokemonJump;
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            for i in 0..MAX_RFU_PLAYERS {
                *data.at(i + 2) = FALSE as i16;
            }
            *data += 1;
        }
        if fall || sw1 == 1 {
            SendPacket_MonInfo(&raw mut (*jump).monInfo[(*jump).multiplayerId]);
            for i in 0..MAX_RFU_PLAYERS {
                if *data.at(i + 2) == 0 && RecvPacket_MonInfo(i, &raw mut (*jump).monInfo[i]) != 0 {
                    StringCopy(
                        (*jump).players[i].name.as_mut_ptr(),
                        (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                    );
                    *data.at(i + 2) = TRUE as i16;
                    *data.at(1) += 1;
                    if *data.at(1) == (*jump).numPlayers as i16 {
                        InitPlayerAndJumpTypes();
                        DestroyTask(taskId);
                        break;
                    }
                }
            }
            break 'l1;
        }
    }
}
unsafe fn SetTaskWithPokeJumpStruct(func: Option<unsafe fn(u8)>, taskPriority: u8) {
    let taskId: u8 = CreateTask(func, taskPriority);
    SetWordTaskArg(
        taskId,
        DATAIDX_GAME_STRUCT,
        crate::gl!(sPokemonJump) as usize as u32,
    );
}
unsafe fn InitVineState() {
    (*crate::gl!(sPokemonJump)).vineTimer = 0;
    (*crate::gl!(sPokemonJump)).vineState = VINE_UPSWING_LOWER;
    (*crate::gl!(sPokemonJump)).vineStateTimer = 0;
    (*crate::gl!(sPokemonJump)).vineSpeed = 0;
    (*crate::gl!(sPokemonJump)).ignoreJumpInput = FALSE as u16;
    (*crate::gl!(sPokemonJump)).gameOver = FALSE as u32;
}
unsafe fn ResetVineState() {
    (*crate::gl!(sPokemonJump)).vineTimer = 0;
    (*crate::gl!(sPokemonJump)).vineStateTimer = 1791;
    (*crate::gl!(sPokemonJump)).vineState = VINE_UPSWING_LOW;
    (*crate::gl!(sPokemonJump)).ignoreJumpInput = FALSE as u16;
    (*crate::gl!(sPokemonJump)).gameOver = FALSE as u32;
    (*crate::gl!(sPokemonJump)).vineSpeedStage = 0;
    (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx = 0;
    (*crate::gl!(sPokemonJump)).vineSpeedAccel = 0;
    (*crate::gl!(sPokemonJump)).vineSpeedDelay = 0;
    (*crate::gl!(sPokemonJump)).atMaxSpeedStage = FALSE as u32;
    UpdateVineSpeed();
}
unsafe fn UpdateVineState() {
    if (*crate::gl!(sPokemonJump)).allowVineUpdates != 0 {
        (*crate::gl!(sPokemonJump)).vineTimer += 1;
        (*crate::gl!(sPokemonJump)).vineStateTimer += GetVineSpeed() as u16;
        if (*crate::gl!(sPokemonJump)).vineStateTimer >= 2559 {
            (*crate::gl!(sPokemonJump)).vineStateTimer -= 2559;
        }
        (*crate::gl!(sPokemonJump)).prevVineState = (*crate::gl!(sPokemonJump)).vineState;
        (*crate::gl!(sPokemonJump)).vineState =
            ((*crate::gl!(sPokemonJump)).vineStateTimer >> 8) as u32;
        if (*crate::gl!(sPokemonJump)).vineState > VINE_UPSWING_LOWER
            && (*crate::gl!(sPokemonJump)).prevVineState < VINE_UPSWING_LOW
        {
            (*crate::gl!(sPokemonJump)).ignoreJumpInput += 1;
            UpdateVineSpeed();
        }
    }
}
unsafe fn GetVineSpeed() -> i32 {
    if (*crate::gl!(sPokemonJump)).gameOver != 0 {
        return 0;
    }
    let mut speed: i32 = (*crate::gl!(sPokemonJump)).vineSpeed;
    if (*crate::gl!(sPokemonJump)).vineStateTimer <= 1535 {
        (*crate::gl!(sPokemonJump)).vineSpeedAccel += 80;
        speed += ((*crate::gl!(sPokemonJump)).vineSpeedAccel / 256) as i32;
    }
    speed
}
unsafe fn UpdateVineSpeed() {
    let mut baseSpeed: i32 = 0;
    (*crate::gl!(sPokemonJump)).vineSpeedAccel = 0;
    if (*crate::gl!(sPokemonJump)).vineSpeedDelay != 0 {
        (*crate::gl!(sPokemonJump)).vineSpeedDelay -= 1;
        if (*crate::gl!(sPokemonJump)).atMaxSpeedStage != 0 {
            if PokeJumpRandom() % 4 != 0 {
                (*crate::gl!(sPokemonJump)).vineSpeed =
                    (*crate::gl!(sPokemonJump)).nextVineSpeed as i32;
            } else {
                if (*crate::gl!(sPokemonJump)).nextVineSpeed > 54 {
                    (*crate::gl!(sPokemonJump)).vineSpeed = 30;
                } else {
                    (*crate::gl!(sPokemonJump)).vineSpeed = 82;
                }
            }
        }
    } else {
        if (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx as u32 & 8 == 0 {
            (*crate::gl!(sPokemonJump)).nextVineSpeed =
                sVineBaseSpeeds[(*crate::gl!(sPokemonJump)).vineBaseSpeedIdx] as u32
                    + (*crate::gl!(sPokemonJump)).vineSpeedStage as u32 * 7;
            (*crate::gl!(sPokemonJump)).vineSpeedDelay = sVineSpeedDelays[PokeJumpRandom() % 4] + 2;
            (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx += 1;
        } else {
            if (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx == 8 {
                if (*crate::gl!(sPokemonJump)).vineSpeedStage < 3 {
                    (*crate::gl!(sPokemonJump)).vineSpeedStage += 1;
                } else {
                    (*crate::gl!(sPokemonJump)).atMaxSpeedStage = TRUE as u32;
                }
            }
            baseSpeed =
                sVineBaseSpeeds[15 - (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx as i32] as i32;
            (*crate::gl!(sPokemonJump)).nextVineSpeed =
                baseSpeed as u32 + (*crate::gl!(sPokemonJump)).vineSpeedStage as u32 * 7;
            if ({
                (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx += 1;
                (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx
            }) > 15
            {
                if PokeJumpRandom() % 4 == 0 {
                    (*crate::gl!(sPokemonJump)).nextVineSpeed -= 5;
                }
                (*crate::gl!(sPokemonJump)).vineBaseSpeedIdx = 0;
            }
        }
        (*crate::gl!(sPokemonJump)).vineSpeed = (*crate::gl!(sPokemonJump)).nextVineSpeed as i32;
    }
}
unsafe fn PokeJumpRandom() -> i32 {
    (*crate::gl!(sPokemonJump)).rngSeed = 0x41c64e6d * (*crate::gl!(sPokemonJump)).rngSeed + 24691;
    ((*crate::gl!(sPokemonJump)).rngSeed >> 16) as i32
}
unsafe fn ResetVineAfterHit() {
    (*crate::gl!(sPokemonJump)).gameOver = TRUE as u32;
    (*crate::gl!(sPokemonJump)).vineState = VINE_UPSWING_LOWER;
    (*crate::gl!(sPokemonJump)).vineStateTimer = 1535;
    AllowVineUpdates();
}
unsafe fn IsGameOver() -> i32 {
    (*crate::gl!(sPokemonJump)).gameOver as i32
}
unsafe fn ResetPlayersJumpStates() {
    for i in 0..MAX_RFU_PLAYERS {
        (*crate::gl!(sPokemonJump)).players[i].jumpState = JUMPSTATE_NONE;
    }
}
unsafe fn ResetPlayersMonState() {
    (*(*crate::gl!(sPokemonJump)).player).monState = MONSTATE_NORMAL;
    (*(*crate::gl!(sPokemonJump)).player).prevMonState = MONSTATE_NORMAL;
}
unsafe fn IsPlayersMonState(monState: u16) -> u32 {
    if (*crate::gl!(sPokemonJump)).players[(*crate::gl!(sPokemonJump)).multiplayerId].monState
        == monState
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SetMonStateJump() {
    (*(*crate::gl!(sPokemonJump)).player).jumpTimeStart = (*crate::gl!(sPokemonJump)).vineTimer;
    (*(*crate::gl!(sPokemonJump)).player).prevMonState =
        (*(*crate::gl!(sPokemonJump)).player).monState;
    (*(*crate::gl!(sPokemonJump)).player).monState = MONSTATE_JUMP;
}
unsafe fn SetMonStateHit() {
    (*(*crate::gl!(sPokemonJump)).player).prevMonState =
        (*(*crate::gl!(sPokemonJump)).player).monState;
    (*(*crate::gl!(sPokemonJump)).player).monState = MONSTATE_HIT;
    (*(*crate::gl!(sPokemonJump)).player).jumpTimeStart = (*crate::gl!(sPokemonJump)).vineTimer;
    (*(*crate::gl!(sPokemonJump)).player).jumpState = JUMPSTATE_FAILURE;
}
unsafe fn SetMonStateNormal() {
    (*(*crate::gl!(sPokemonJump)).player).prevMonState =
        (*(*crate::gl!(sPokemonJump)).player).monState;
    (*(*crate::gl!(sPokemonJump)).player).monState = MONSTATE_NORMAL;
}
pub(crate) unsafe fn UpdateGame() {
    if (*crate::gl!(sPokemonJump)).updateScore != 0 {
        PrintScore((*crate::gl!(sPokemonJump)).comm.jumpScore as i32);
        (*crate::gl!(sPokemonJump)).updateScore = FALSE as u32;
        if (*crate::gl!(sPokemonJump)).showBonus != 0 {
            let numPlayers: i32 =
                DoSameJumpTimeBonus((*crate::gl!(sPokemonJump)).comm.receivedBonusFlags);
            PlaySE(sSoundEffects[numPlayers - 2]);
            (*crate::gl!(sPokemonJump)).showBonus = FALSE;
        }
    }
    PrintJumpsInRow((*crate::gl!(sPokemonJump)).comm.jumpsInRow);
    HandleMonState();
    TryUpdateVineSwing();
}
unsafe fn TryUpdateVineSwing() {
    if (*crate::gl!(sPokemonJump)).allowVineUpdates != 0 {
        UpdateVineSwing((*crate::gl!(sPokemonJump)).vineState as i32);
    }
}
unsafe fn DisallowVineUpdates() {
    (*crate::gl!(sPokemonJump)).allowVineUpdates = FALSE;
}
unsafe fn AllowVineUpdates() {
    (*crate::gl!(sPokemonJump)).allowVineUpdates = TRUE;
}
unsafe fn HandleMonState() {
    let mut soundFlags: i32 = 0;
    let numPlayers: i32 = (*crate::gl!(sPokemonJump)).numPlayers as i32;
    for i in 0..numPlayers {
        match (*crate::gl!(sPokemonJump)).players[i].monState {
            MONSTATE_NORMAL => {
                SetMonSpriteY(i as u32, 0);
            }
            MONSTATE_JUMP => {
                if (*crate::gl!(sPokemonJump)).players[i].prevMonState != MONSTATE_JUMP
                    || (*crate::gl!(sPokemonJump)).players[i].jumpTimeStart
                        != (*crate::gl!(sPokemonJump)).jumpTimeStarts[i]
                {
                    if i == (*crate::gl!(sPokemonJump)).multiplayerId as i32 {
                        (*crate::gl!(sPokemonJump)).players[i].prevMonState = MONSTATE_JUMP;
                    }
                    soundFlags |= F_SE_JUMP;
                    (*crate::gl!(sPokemonJump)).players[i].jumpOffsetIdx = 2147483647;
                    (*crate::gl!(sPokemonJump)).jumpTimeStarts[i] =
                        (*crate::gl!(sPokemonJump)).players[i].jumpTimeStart;
                }
                UpdateJump(i);
            }
            MONSTATE_HIT if (*crate::gl!(sPokemonJump)).players[i].prevMonState != MONSTATE_HIT => {
                if i == (*crate::gl!(sPokemonJump)).multiplayerId as i32 {
                    (*crate::gl!(sPokemonJump)).players[i].prevMonState = MONSTATE_HIT;
                }
                soundFlags |= F_SE_FAIL;
                StartMonHitShake(i as u8);
            }
            _ => {}
        }
    }
    if soundFlags & F_SE_FAIL != 0 {
        PlaySE(SE_RG_POKE_JUMP_FAILURE);
    } else if soundFlags & F_SE_JUMP != 0 {
        PlaySE(SE_LEDGE);
    }
}
unsafe fn UpdateJump(multiplayerId: i32) {
    let mut jumpOffsetIdx: i32 = 0;
    let mut jumpOffset: i32 = 0;
    if (*crate::gl!(sPokemonJump)).skipJumpUpdate != 0 {
        return;
    }
    let player: *mut PokemonJump_Player =
        &raw mut (*crate::gl!(sPokemonJump)).players[multiplayerId];
    if (*player).jumpOffsetIdx != 2147483647 {
        (*player).jumpOffsetIdx += 1;
        jumpOffsetIdx = (*player).jumpOffsetIdx;
    } else {
        jumpOffsetIdx =
            (*crate::gl!(sPokemonJump)).vineTimer as i32 - (*player).jumpTimeStart as i32;
        if jumpOffsetIdx >= 65000 {
            jumpOffsetIdx -= 65000;
            jumpOffsetIdx += (*crate::gl!(sPokemonJump)).vineTimer as i32;
        }
        (*player).jumpOffsetIdx = jumpOffsetIdx;
    }
    if jumpOffsetIdx < 4 {
        return;
    }
    jumpOffsetIdx -= 4;
    if jumpOffsetIdx < 48 {
        jumpOffset = sJumpOffsets[(*player).monJumpType][jumpOffsetIdx] as i32;
    } else {
        jumpOffset = 0;
    }
    SetMonSpriteY(multiplayerId as u32, jumpOffset as i16);
    if jumpOffset == 0 && multiplayerId == (*crate::gl!(sPokemonJump)).multiplayerId as i32 {
        SetMonStateNormal();
    }
    (*player).jumpOffset = jumpOffset;
}
unsafe fn TryUpdateScore() {
    if (*crate::gl!(sPokemonJump)).vineState == VINE_UPSWING_HIGH
        && (*crate::gl!(sPokemonJump)).prevVineState == VINE_UPSWING_LOW
    {
        if (*crate::gl!(sPokemonJump)).initScoreUpdate == 0 {
            ClearUnreadField();
            (*crate::gl!(sPokemonJump)).numPlayersAtPeak = 0;
            (*crate::gl!(sPokemonJump)).initScoreUpdate = TRUE as u32;
            (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags = 0;
        } else {
            if (*crate::gl!(sPokemonJump)).numPlayersAtPeak == MAX_RFU_PLAYERS {
                (*crate::gl!(sPokemonJump)).excellentsInRow += 1;
                TryUpdateExcellentsRecord((*crate::gl!(sPokemonJump)).excellentsInRow);
            } else {
                (*crate::gl!(sPokemonJump)).excellentsInRow = 0;
            }
            if (*crate::gl!(sPokemonJump)).numPlayersAtPeak > 1 {
                (*crate::gl!(sPokemonJump)).giveBonus = TRUE as u32;
                memcpy(
                    (*crate::gl!(sPokemonJump)).atJumpPeak3.as_mut_ptr(),
                    (*crate::gl!(sPokemonJump)).atJumpPeak2.as_mut_ptr(),
                    MAX_RFU_PLAYERS as u32,
                );
            }
            ClearUnreadField();
            (*crate::gl!(sPokemonJump)).numPlayersAtPeak = 0;
            (*crate::gl!(sPokemonJump)).initScoreUpdate = TRUE as u32;
            (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags = 0;
            if (*crate::gl!(sPokemonJump)).comm.jumpsInRow < MAX_JUMPS {
                (*crate::gl!(sPokemonJump)).comm.jumpsInRow += 1;
            }
            AddJumpScore(10);
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        }
    }
    if (*crate::gl!(sPokemonJump)).giveBonus != 0
        && (DidAllPlayersClearVine() == TRUE as u32
            || (*crate::gl!(sPokemonJump)).vineState == VINE_HIGHEST)
    {
        let numPlayers: i32 =
            GetNumPlayersForBonus((*crate::gl!(sPokemonJump)).atJumpPeak3.as_mut_ptr());
        AddJumpScore(GetScoreBonus(numPlayers));
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        (*crate::gl!(sPokemonJump)).giveBonus = FALSE as u32;
    }
    if (*crate::gl!(sPokemonJump)).initScoreUpdate != 0 {
        let numAtPeak: i32 = GetPlayersAtJumpPeak();
        if numAtPeak > (*crate::gl!(sPokemonJump)).numPlayersAtPeak {
            (*crate::gl!(sPokemonJump)).numPlayersAtPeak = numAtPeak;
            memcpy(
                (*crate::gl!(sPokemonJump)).atJumpPeak2.as_mut_ptr(),
                (*crate::gl!(sPokemonJump)).atJumpPeak.as_mut_ptr(),
                MAX_RFU_PLAYERS as u32,
            );
        }
    }
}
unsafe fn UpdateVineHitStates() -> u32 {
    if (*crate::gl!(sPokemonJump)).vineState == VINE_UPSWING_LOWER
        && (*(*crate::gl!(sPokemonJump)).player).jumpOffset == 0
    {
        if (*(*crate::gl!(sPokemonJump)).player).prevMonState == MONSTATE_JUMP
            && IsGameOver() == TRUE as i32
        {
            (*(*crate::gl!(sPokemonJump)).player).jumpState = JUMPSTATE_SUCCESS;
        } else {
            SetMonStateHit();
            SetLinkTimeInterval(LINK_INTERVAL_SHORT);
        }
    }
    if (*crate::gl!(sPokemonJump)).vineState == VINE_UPSWING_LOW
        && (*crate::gl!(sPokemonJump)).prevVineState == VINE_UPSWING_LOWER
        && (*(*crate::gl!(sPokemonJump)).player).monState != MONSTATE_HIT
    {
        (*(*crate::gl!(sPokemonJump)).player).jumpState = JUMPSTATE_SUCCESS;
        SetLinkTimeInterval(LINK_INTERVAL_SHORT);
    }
    for i in 0..((*crate::gl!(sPokemonJump)).numPlayers as i32) {
        if (*crate::gl!(sPokemonJump)).players[i].monState == MONSTATE_HIT {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn AllPlayersJumpedOrHit() -> u32 {
    let numPlayers: i32 = (*crate::gl!(sPokemonJump)).numPlayers as i32;
    let mut numJumpedOrHit: i32 = 0;
    for i in 0..numPlayers {
        if (*crate::gl!(sPokemonJump)).players[i].jumpState != JUMPSTATE_NONE {
            numJumpedOrHit += 1;
        }
    }
    (numJumpedOrHit == numPlayers) as u32
}
unsafe fn DidAllPlayersClearVine() -> u32 {
    for i in 0..((*crate::gl!(sPokemonJump)).numPlayers as i32) {
        if (*crate::gl!(sPokemonJump)).players[i].jumpState != JUMPSTATE_SUCCESS {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn ShouldPlayAgain() -> u32 {
    if (*crate::gl!(sPokemonJump)).playAgainState == PLAY_AGAIN_NO {
        return FALSE as u32;
    }
    for i in 1..((*crate::gl!(sPokemonJump)).numPlayers as i32) {
        if (*crate::gl!(sPokemonJump)).playAgainStates[i] == PLAY_AGAIN_NO as u16 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn AddJumpScore(score: i32) {
    (*crate::gl!(sPokemonJump)).comm.jumpScore += score as u32;
    (*crate::gl!(sPokemonJump)).updateScore = TRUE as u32;
    if (*crate::gl!(sPokemonJump)).comm.jumpScore >= MAX_JUMP_SCORE {
        (*crate::gl!(sPokemonJump)).comm.jumpScore = MAX_JUMP_SCORE;
    }
}
unsafe fn GetPlayersAtJumpPeak() -> i32 {
    let mut numAtPeak: i32 = 0;
    let numPlayers: i32 = (*crate::gl!(sPokemonJump)).numPlayers as i32;
    for i in 0..numPlayers {
        if (*crate::gl!(sPokemonJump)).players[i].jumpOffset == JUMP_PEAK {
            (*crate::gl!(sPokemonJump)).atJumpPeak[i] = TRUE;
            numAtPeak += 1;
        } else {
            (*crate::gl!(sPokemonJump)).atJumpPeak[i] = FALSE;
        }
    }
    numAtPeak
}
unsafe fn AreLinkQueuesEmpty() -> u32 {
    ((crate::gl!(&raw gRfu.recvQueue.count)).read_volatile() == 0
        && (crate::gl!(&raw gRfu.sendQueue.count)).read_volatile() == 0) as u32
}
unsafe fn GetNumPlayersForBonus(atJumpPeak: *mut u8) -> i32 {
    let mut flags: i32 = 0;
    let mut count: i32 = 0;
    for i in 0..MAX_RFU_PLAYERS {
        if *atJumpPeak.at(i) != 0 {
            flags |= shl_i32(1, i as u32);
            count += 1;
        }
    }
    (*crate::gl!(sPokemonJump)).comm.receivedBonusFlags = flags as u8;
    if flags != 0 {
        (*crate::gl!(sPokemonJump)).showBonus = TRUE;
    }
    count
}
unsafe fn ClearUnreadField() {
    (*crate::gl!(sPokemonJump)).unused3 = 0;
}
fn GetScoreBonus(numPlayers: i32) -> i32 {
    sScoreBonuses[numPlayers]
}
unsafe fn TryUpdateExcellentsRecord(excellentsInRow: u16) {
    if excellentsInRow > (*crate::gl!(sPokemonJump)).excellentsInRowRecord {
        (*crate::gl!(sPokemonJump)).excellentsInRowRecord = excellentsInRow;
    }
}
unsafe fn HasEnoughScoreForPrize() -> u32 {
    if (*crate::gl!(sPokemonJump)).comm.jumpScore >= sPrizeQuantityData[0].score {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetPrizeData() -> u16 {
    let itemId: u16 = GetPrizeItemId();
    let quantity: u16 = GetPrizeQuantity();
    quantity << 12 | itemId & 0xFFF
}
unsafe fn UnpackPrizeData(data: u16, itemId: *mut u16, quantity: *mut u16) {
    *quantity = data >> 12;
    *itemId = data & 0xFFF;
}
pub(crate) unsafe fn GetPrizeItemId() -> u16 {
    let index: u16 = Random() % 8;
    sPrizeItems[index]
}
unsafe fn GetPrizeQuantity() -> u16 {
    let mut quantity: u32 = 0;
    for i in 0..5u32 {
        if (*crate::gl!(sPokemonJump)).comm.jumpScore >= sPrizeQuantityData[i].score {
            quantity = sPrizeQuantityData[i].quantity;
        } else {
            break;
        }
    }
    quantity as u16
}
unsafe fn GetQuantityLimitedByBag(item: u16, mut quantity: u16) -> u16 {
    while quantity != 0 && CheckBagHasSpace(item, quantity) == 0 {
        quantity -= 1;
    }
    quantity
}
unsafe fn GetNumPokeJumpPlayers() -> u16 {
    GetLinkPlayerCount() as u16
}
unsafe fn GetPokeJumpMultiplayerId() -> u16 {
    (*crate::gl!(sPokemonJump)).multiplayerId as u16
}
unsafe fn GetMonInfoByMultiplayerId(multiplayerId: u8) -> *mut PokemonJump_MonInfo {
    &raw mut (*crate::gl!(sPokemonJump)).monInfo[multiplayerId]
}
unsafe fn GetPokeJumpPlayerName(multiplayerId: u8) -> *mut u8 {
    (*crate::gl!(sPokemonJump)).players[multiplayerId]
        .name
        .as_mut_ptr()
}
pub unsafe fn IsSpeciesAllowedInPokemonJump(species: u16) -> u32 {
    (GetPokemonJumpSpeciesIdx(species) > -1) as u32
}
#[unsafe(no_mangle)]
pub unsafe fn IsPokemonJumpSpeciesInParty() {
    for i in 0..PARTY_SIZE {
        if GetMonData2(
            crate::gl!(&raw gPlayerParty[i]),
            MON_DATA_SANITY_HAS_SPECIES,
        ) != 0
        {
            let species: u16 =
                GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG) as u16;
            if IsSpeciesAllowedInPokemonJump(species) != 0 {
                gSpecialVar_Result = TRUE as u16;
                return;
            }
        }
    }
    gSpecialVar_Result = FALSE as u16;
}
unsafe fn LoadSpriteSheetsAndPalettes(jumpGfx: *mut PokemonJumpGfx) {
    for i in 0..5i32 {
        LoadCompressedSpriteSheet((&raw const sCompressedSpriteSheets[i]).cast_mut());
    }
    for i in 0..2i32 {
        LoadSpritePalette((&raw const sSpritePalettes[i]).cast_mut());
    }
    (*jumpGfx).vinePalNumDownswing = IndexOfSpritePaletteTag(PALTAG_1);
    (*jumpGfx).vinePalNumUpswing = IndexOfSpritePaletteTag(PALTAG_2);
}
unsafe fn ResetPokeJumpSpriteData(sprite: *mut Sprite) {
    for i in 0..8i32 {
        (*sprite).data[i] = 0;
    }
}
unsafe fn CreateJumpMonSprite(
    jumpGfx: *mut PokemonJumpGfx,
    monInfo: *mut PokemonJump_MonInfo,
    x: i16,
    y: i16,
    multiplayerId: u8,
) {
    let mut spriteSheet: SpriteSheet = zeroed();
    let mut spritePalette: CompressedSpritePalette = zeroed();
    let mut subpriority: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplate_JumpMon;
    let buffer: *mut u8 = Alloc(8192) as *mut u8;
    let unusedBuffer: *mut u8 = Alloc(MON_PIC_SIZE as u32) as *mut u8;
    if multiplayerId as u16 == GetPokeJumpMultiplayerId() {
        subpriority = 3;
    } else {
        subpriority = multiplayerId + 4;
    }
    if !buffer.is_null() && !unusedBuffer.is_null() {
        HandleLoadSpecialPokePic(
            (&raw const (*(&raw const crate::data::data_tables::gMonStillFrontPicTable)
                .cast::<CArray<CompressedSpriteSheet, 0>>())[(*monInfo).species])
                .cast_mut(),
            buffer as *mut c_void,
            (*monInfo).species as i32,
            (*monInfo).personality,
        );
        spriteSheet.data = buffer as *mut c_void;
        spriteSheet.tag = multiplayerId as u16;
        spriteSheet.size = MON_PIC_SIZE;
        LoadSpriteSheet(&raw mut spriteSheet);
        spritePalette.data = GetMonSpritePalFromSpeciesAndPersonality(
            (*monInfo).species,
            (*monInfo).otId,
            (*monInfo).personality,
        );
        spritePalette.tag = multiplayerId as u16;
        LoadCompressedSpritePalette(&raw mut spritePalette);
        Free(buffer as *mut c_void);
        Free(unusedBuffer as *mut c_void);
        spriteTemplate.tileTag += multiplayerId as u16;
        spriteTemplate.paletteTag += multiplayerId as u16;
        spriteId = CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
        if spriteId != MAX_SPRITES {
            (*jumpGfx).monSprites[multiplayerId] = crate::gl!(&raw gSprites[spriteId]);
            (*jumpGfx).monSpriteSubpriorities[multiplayerId] = subpriority;
            return;
        }
    }
    (*jumpGfx).monSprites[multiplayerId] = null_mut();
}
unsafe fn DoStarAnim(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) {
    ResetPokeJumpSpriteData((*jumpGfx).starSprites[multiplayerId]);
    (*(*jumpGfx).starSprites[multiplayerId]).data[sOffset] =
        (((*jumpGfx).monSprites[multiplayerId] as usize)
            .wrapping_sub((*gSprites.as_ptr()).as_mut_ptr() as usize) as i32
            / 68) as i16;
    (*(*jumpGfx).starSprites[multiplayerId]).set_invisible(FALSE as u16);
    (*(*jumpGfx).starSprites[multiplayerId]).y = 96;
    (*(*jumpGfx).starSprites[multiplayerId]).callback = Some(SpriteCB_Star);
    StartSpriteAnim((*jumpGfx).starSprites[multiplayerId], 1);
}
pub(crate) unsafe fn SpriteCB_Star(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {
            if (*sprite).animEnded() != 0 {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
        1 => {
            (*sprite).y -= 1;
            (*sprite).data[1] += 1;
            if (*sprite).y <= 72 {
                (*sprite).y = 72;
                (*sprite).data[sState] += 1;
            }
        }
        2 if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) >= 48 =>
        {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
        _ => {}
    }
}
unsafe fn Gfx_StartMonHitShake(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) {
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonHitShake);
    (*(*jumpGfx).monSprites[multiplayerId]).y2 = 0;
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
}
unsafe fn Gfx_IsMonHitShakeActive(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) -> u32 {
    ((*(*jumpGfx).monSprites[multiplayerId]).callback
        == Some(SpriteCB_MonHitShake as unsafe fn(*mut Sprite))) as u32
}
pub(crate) unsafe fn SpriteCB_MonHitShake(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        if ({
            (*sprite).data[sNumShakes] += 1;
            (*sprite).data[sNumShakes]
        }) as i32
            & 1
            != 0
        {
            (*sprite).y2 = 2;
        } else {
            (*sprite).y2 = -2;
        }
        (*sprite).data[1] = 0;
    }
    if (*sprite).data[sNumShakes] > 12 {
        (*sprite).y2 = 0;
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn Gfx_StartMonHitFlash(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) {
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonHitFlash);
}
unsafe fn Gfx_StopMonHitFlash(jumpGfx: *mut PokemonJumpGfx) {
    let numPlayers: u16 = GetNumPokeJumpPlayers();
    for i in 0..(numPlayers as i32) {
        if (*(*jumpGfx).monSprites[i]).callback
            == Some(SpriteCB_MonHitFlash as unsafe fn(*mut Sprite))
        {
            (*(*jumpGfx).monSprites[i]).set_invisible(FALSE as u16);
            (*(*jumpGfx).monSprites[i]).callback = Some(SpriteCallbackDummy);
            (*(*jumpGfx).monSprites[i]).subpriority = 10;
        }
    }
}
pub(crate) unsafe fn SpriteCB_MonHitFlash(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
}
unsafe fn Gfx_ResetMonSpriteSubpriorities(jumpGfx: *mut PokemonJumpGfx) {
    let numPlayers: u16 = GetNumPokeJumpPlayers();
    for i in 0..(numPlayers as i32) {
        (*(*jumpGfx).monSprites[i]).subpriority = (*jumpGfx).monSpriteSubpriorities[i];
    }
}
unsafe fn Gfx_StartMonIntroBounce(jumpGfx: *mut PokemonJumpGfx, multiplayerId: i32) {
    ResetPokeJumpSpriteData((*jumpGfx).monSprites[multiplayerId]);
    (*(*jumpGfx).monSprites[multiplayerId]).callback = Some(SpriteCB_MonIntroBounce);
}
unsafe fn Gfx_IsMonIntroBounceActive(jumpGfx: *mut PokemonJumpGfx) -> u32 {
    let numPlayers: u16 = GetNumPokeJumpPlayers();
    for i in 0..(numPlayers as i32) {
        if (*(*jumpGfx).monSprites[i]).callback
            == Some(SpriteCB_MonIntroBounce as unsafe fn(*mut Sprite))
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub(crate) unsafe fn SpriteCB_MonIntroBounce(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PlaySE(SE_BIKE_HOP);
            (*sprite).data[sHopPos] = 0;
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            (*sprite).data[sHopPos] += 4;
            if (*sprite).data[sHopPos] > 127 {
                (*sprite).data[sHopPos] = 0;
            }
            (*sprite).y2 = -((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sprite).data[sHopPos]]
                >> 3);
            if (*sprite).data[sHopPos] == 0 {
                if ({
                    (*sprite).data[sNumHops] += 1;
                    (*sprite).data[sNumHops]
                }) < 2
                {
                    (*sprite).data[sState] = 0;
                } else {
                    (*sprite).callback = Some(SpriteCallbackDummy);
                }
            }
            break 'l1;
        }
    }
}
unsafe fn CreateStarSprite(jumpGfx: *mut PokemonJumpGfx, x: i16, y: i16, multiplayerId: u8) {
    let spriteId: u8 = CreateSprite((&raw const *sSpriteTemplate_Star).cast_mut(), x, y, 1);
    if spriteId != MAX_SPRITES {
        (*gSprites.as_ptr())[spriteId].set_invisible(TRUE as u16);
        (*jumpGfx).starSprites[multiplayerId] = crate::gl!(&raw gSprites[spriteId]);
    }
}
unsafe fn CreateVineSprites(jumpGfx: *mut PokemonJumpGfx) {
    let mut spriteId: u8 = 0;
    let mut count: i32 = 0;
    for i in 0..VINE_SPRITES_PER_SIDE {
        spriteId = CreateSprite(
            sSpriteTemplates_Vine[i],
            sVineXCoords[count],
            sVineYCoords[i][0],
            2,
        );
        (*jumpGfx).vineSprites[count] = crate::gl!(&raw gSprites[spriteId]);
        count += 1;
    }
    let mut i: i32 = 3;
    while i >= 0 {
        spriteId = CreateSprite(
            sSpriteTemplates_Vine[i],
            sVineXCoords[count],
            sVineYCoords[i][0],
            2,
        );
        (*jumpGfx).vineSprites[count] = crate::gl!(&raw gSprites[spriteId]);
        (*(*jumpGfx).vineSprites[count]).set_hFlip(TRUE as u16);
        count += 1;
        i -= 1;
    }
}
unsafe fn UpdateVineAnim(jumpGfx: *mut PokemonJumpGfx, mut vineState: i32) {
    let mut palNum: i32 = 0;
    let mut priority: i32 = 0;
    if vineState > VINE_LOWEST {
        vineState = NUM_VINESTATES - vineState;
        priority = 3;
        palNum = (*jumpGfx).vinePalNumUpswing as i32;
    } else {
        priority = 2;
        palNum = (*jumpGfx).vinePalNumDownswing as i32;
    }
    let mut count: i32 = 0;
    for i in 0..VINE_SPRITES_PER_SIDE {
        (*(*jumpGfx).vineSprites[count]).y = sVineYCoords[i][vineState];
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_priority(priority as u16);
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_paletteNum(palNum as u16);
        StartSpriteAnim((*jumpGfx).vineSprites[count], vineState as u8);
        count += 1;
    }
    let mut i: i32 = 3;
    while i >= 0 {
        (*(*jumpGfx).vineSprites[count]).y = sVineYCoords[i][vineState];
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_priority(priority as u16);
        (*(*jumpGfx).vineSprites[count])
            .oam
            .set_paletteNum(palNum as u16);
        StartSpriteAnim((*jumpGfx).vineSprites[count], vineState as u8);
        count += 1;
        i -= 1;
    }
}
unsafe fn StartPokeJumpCountdown(jumpGfx: *mut PokemonJumpGfx) {
    StartMinigameCountdown(GFXTAG_COUNTDOWN, PALTAG_COUNTDOWN, 120, 80, 0);
    Gfx_ResetMonSpriteSubpriorities(jumpGfx);
}
unsafe fn IsPokeJumpCountdownRunning() -> u32 {
    IsMinigameCountdownRunning()
}
unsafe fn StartPokeJumpGfx(jumpGfx: *mut PokemonJumpGfx) {
    crate::gl!(sPokemonJumpGfx = jumpGfx);
    InitPokeJumpGfx(crate::gl!(sPokemonJumpGfx));
    let taskId: u8 = CreateTask(Some(Task_RunPokeJumpGfxFunc), 3);
    (*crate::gl!(sPokemonJumpGfx)).taskId = taskId;
    SetWordTaskArg(
        (*crate::gl!(sPokemonJumpGfx)).taskId,
        2,
        crate::gl!(sPokemonJumpGfx) as usize as u32,
    );
    SetUpPokeJumpGfxFunc(Some(LoadPokeJumpGfx));
}
unsafe fn FreeWindowsAndDigitObj() {
    FreeAllWindowBuffers();
    DigitObjUtil_Free();
}
unsafe fn InitPokeJumpGfx(jumpGfx: *mut PokemonJumpGfx) {
    (*jumpGfx).mainState = 0;
    (*jumpGfx).funcFinished = FALSE as u32;
    (*jumpGfx).msgWindowId = WINDOW_NONE as u16;
}
unsafe fn SetUpPokeJumpGfxFuncById(id: i32) {
    for i in 0..10i32 {
        if sPokeJumpGfxFuncs[i].id == id {
            SetUpPokeJumpGfxFunc(sPokeJumpGfxFuncs[i].func);
        }
    }
}
unsafe fn IsPokeJumpGfxFuncFinished() -> u32 {
    ((*crate::gl!(sPokemonJumpGfx)).funcFinished != TRUE as u32) as u32
}
unsafe fn SetUpPokeJumpGfxFunc(func: Option<unsafe fn()>) {
    SetWordTaskArg(
        (*crate::gl!(sPokemonJumpGfx)).taskId,
        0,
        core::mem::transmute::<_, usize>(func) as u32,
    );
    (*crate::gl!(sPokemonJumpGfx)).mainState = 0;
    (*crate::gl!(sPokemonJumpGfx)).funcFinished = FALSE as u32;
}
pub(crate) unsafe fn Task_RunPokeJumpGfxFunc(taskId: u8) {
    if (*crate::gl!(sPokemonJumpGfx)).funcFinished == 0 {
        let func: Option<unsafe fn()> = core::mem::transmute::<_, Option<unsafe fn()>>(
            GetWordTaskArg(taskId, 0) as usize as *mut c_void,
        );
        func.unwrap_unchecked()();
    }
}
pub(crate) unsafe fn LoadPokeJumpGfx() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sWindowTemplates.as_ptr().cast_mut());
            ResetTempTileDataBuffers();
            LoadSpriteSheetsAndPalettes(crate::gl!(sPokemonJumpGfx));
            InitDigitPrinters();
            LoadPalette(sBg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
            DecompressAndCopyTileDataToVram(
                BG_SCENERY,
                sBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_SCENERY,
                sBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sVenusaur_Pal.as_ptr().cast_mut() as *mut c_void, 48, 32);
            DecompressAndCopyTileDataToVram(
                BG_VENUSAUR,
                sVenusaur_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_VENUSAUR,
                sVenusaur_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sBonuses_Pal.as_ptr().cast_mut() as *mut c_void, 16, 32);
            DecompressAndCopyTileDataToVram(
                BG_BONUSES,
                sBonuses_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                BG_BONUSES,
                sBonuses_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            LoadPalette(sInterface_Pal.as_ptr().cast_mut() as *mut c_void, 32, 32);
            SetBgTilemapBuffer(
                BG_INTERFACE,
                (*crate::gl!(sPokemonJumpGfx)).tilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            FillBgTilemapBufferRect_Palette0(BG_INTERFACE, 0, 0, 0, 0x20, 0x20);
            PrintScoreSuffixes();
            PrintScore(0);
            LoadUserWindowBorderGfxOnBg(0, 1, 224);
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            CopyBgTilemapBufferToVram(BG_VENUSAUR);
            CopyBgTilemapBufferToVram(BG_BONUSES);
            ResetBgPositions();
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                CreateJumpMonSprites();
                CreateVineSprites(crate::gl!(sPokemonJumpGfx));
                UpdateVineAnim(crate::gl!(sPokemonJumpGfx), VINE_UPSWING_LOWER as i32);
                ShowBg(BG_SCENERY);
                ShowBg(BG_INTERFACE);
                ShowBg(BG_VENUSAUR);
                HideBg(BG_BONUSES);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn PrintPlayerNamesNoHighlight() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            AddPlayerNameWindows();
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PrintPokeJumpPlayerNames(FALSE as u32);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                DrawPlayerNameWindows();
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        3 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn PrintPlayerNamesWithHighlight() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            AddPlayerNameWindows();
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PrintPokeJumpPlayerNames(TRUE as u32);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                DrawPlayerNameWindows();
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        3 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn ErasePlayerNames() {
    let numPlayers: i32 = GetNumPokeJumpPlayers() as i32;
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            for i in 0..numPlayers {
                ClearWindowTilemap((*crate::gl!(sPokemonJumpGfx)).nameWindowIds[i] as u8);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            for i in 0..numPlayers {
                RemoveWindow((*crate::gl!(sPokemonJumpGfx)).nameWindowIds[i] as u8);
            }
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn Msg_WantToPlayAgain() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(1, 8, 20, 2) as u16;
            AddTextPrinterParameterized(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_WantToPlayAgain2)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                COPYWIN_GFX,
            );
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                DrawTextBorderOuter((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8, 1, 14);
                CreatePokeJumpYesNoMenu(23, 7, 0);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn Msg_SavingDontTurnOff() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(2, 7, 26, 4) as u16;
            AddTextPrinterParameterized(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                FONT_NORMAL,
                (*crate::asmdata::gText_SavingDontTurnOffPower.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                COPYWIN_GFX,
            );
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                DrawTextBorderOuter((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn EraseMessage() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            ClearMessageWindow();
            EraseYesNoWindow();
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 if RemoveMessageWindow() == 0 && IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn Msg_SomeoneDroppedOut() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(2, 8, 22, 4) as u16;
            AddTextPrinterParameterized(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_SomeoneDroppedOut2)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                COPYWIN_GFX,
            );
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                DrawTextBorderOuter((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn Msg_CommunicationStandby() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(7, 10, 16, 2) as u16;
            AddTextPrinterParameterized(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_CommunicationStandby4)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram(
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
                COPYWIN_GFX,
            );
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                DrawTextBorderOuter((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
            }
        }
        2 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
pub(crate) unsafe fn DoPokeJumpCountdown() {
    match (*crate::gl!(sPokemonJumpGfx)).mainState {
        0 => {
            StartPokeJumpCountdown(crate::gl!(sPokemonJumpGfx));
            (*crate::gl!(sPokemonJumpGfx)).mainState += 1;
        }
        1 if IsPokeJumpCountdownRunning() == 0 => {
            (*crate::gl!(sPokemonJumpGfx)).funcFinished = TRUE as u32;
        }
        _ => {}
    }
}
unsafe fn SetUpResetVineGfx() {
    (*crate::gl!(sPokemonJumpGfx)).resetVineState = 0;
    (*crate::gl!(sPokemonJumpGfx)).resetVineTimer = 0;
    (*crate::gl!(sPokemonJumpGfx)).vineState = VINE_UPSWING_LOWER as u8;
    UpdateVineSwing((*crate::gl!(sPokemonJumpGfx)).vineState as i32);
}
unsafe fn ResetVineGfx() -> u32 {
    'l1: {
        let sw1: u8 = (*crate::gl!(sPokemonJumpGfx)).resetVineState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*crate::gl!(sPokemonJumpGfx)).resetVineTimer += 1;
            if (*crate::gl!(sPokemonJumpGfx)).resetVineTimer > 10 {
                (*crate::gl!(sPokemonJumpGfx)).resetVineTimer = 0;
                (*crate::gl!(sPokemonJumpGfx)).vineState += 1;
                if (*crate::gl!(sPokemonJumpGfx)).vineState >= NUM_VINESTATES as u8 {
                    (*crate::gl!(sPokemonJumpGfx)).vineState = VINE_HIGHEST as u8;
                    (*crate::gl!(sPokemonJumpGfx)).resetVineState += 1;
                }
            }
            UpdateVineSwing((*crate::gl!(sPokemonJumpGfx)).vineState as i32);
            if (*crate::gl!(sPokemonJumpGfx)).vineState != VINE_UPSWING_LOW as u8 {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn PrintPrizeMessage(itemId: u16, quantity: u16) {
    CopyItemNameHandlePlural(
        itemId,
        (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr(),
        quantity as u32,
    );
    ConvertIntToDecimalStringN(
        (*crate::gl!(sPokemonJumpGfx)).itemQuantityStr.as_mut_ptr(),
        quantity as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        0,
        (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr(),
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        1,
        (*crate::gl!(sPokemonJumpGfx)).itemQuantityStr.as_mut_ptr(),
    );
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_AwesomeWonF701F700).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(4, 8, 22, 4) as u16;
    AddTextPrinterParameterized(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        FONT_NORMAL,
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        COPYWIN_GFX,
    );
    (*crate::gl!(sPokemonJumpGfx)).fanfare = MUS_LEVEL_UP;
    (*crate::gl!(sPokemonJumpGfx)).msgWindowState = 0;
}
unsafe fn PrintPrizeFilledBagMessage(itemId: u16) {
    CopyItemName(itemId, (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        0,
        (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr(),
    );
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_FilledStorageSpace2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(4, 8, 22, 4) as u16;
    AddTextPrinterParameterized(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        FONT_NORMAL,
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        COPYWIN_GFX,
    );
    (*crate::gl!(sPokemonJumpGfx)).fanfare = MUS_DUMMY;
    (*crate::gl!(sPokemonJumpGfx)).msgWindowState = 0;
}
unsafe fn PrintNoRoomForPrizeMessage(itemId: u16) {
    CopyItemName(itemId, (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr());
    DynamicPlaceholderTextUtil_Reset();
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
        0,
        (*crate::gl!(sPokemonJumpGfx)).itemName.as_mut_ptr(),
    );
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_CantHoldMore).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*crate::gl!(sPokemonJumpGfx)).msgWindowId = AddMessageWindow(4, 9, 22, 2) as u16;
    AddTextPrinterParameterized(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        FONT_NORMAL,
        (*crate::gl!(sPokemonJumpGfx)).prizeMsg.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(
        (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
        COPYWIN_GFX,
    );
    (*crate::gl!(sPokemonJumpGfx)).fanfare = MUS_DUMMY;
    (*crate::gl!(sPokemonJumpGfx)).msgWindowState = 0;
}
unsafe fn DoPrizeMessageAndFanfare() -> u32 {
    'l1: {
        let sw1: u8 = (*crate::gl!(sPokemonJumpGfx)).msgWindowState;
        let mut fall = false;
        if sw1 == 0 {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                DrawTextBorderOuter((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8, 1, 14);
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sPokemonJumpGfx)).msgWindowState += 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                break 'l1;
            }
            if (*crate::gl!(sPokemonJumpGfx)).fanfare == MUS_DUMMY {
                (*crate::gl!(sPokemonJumpGfx)).msgWindowState += 2;
                return FALSE as u32;
            }
            PlayFanfare((*crate::gl!(sPokemonJumpGfx)).fanfare);
            (*crate::gl!(sPokemonJumpGfx)).msgWindowState += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            if IsFanfareTaskInactive() == 0 {
                break 'l1;
            }
            (*crate::gl!(sPokemonJumpGfx)).msgWindowState += 1;
        }
        if fall || sw1 == 3 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn ClearMessageWindow() {
    if (*crate::gl!(sPokemonJumpGfx)).msgWindowId != WINDOW_NONE as u16 {
        rbox_fill_rectangle((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
        CopyWindowToVram(
            (*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8,
            COPYWIN_MAP,
        );
        (*crate::gl!(sPokemonJumpGfx)).msgWindowState = 0;
    }
}
unsafe fn RemoveMessageWindow() -> u32 {
    if (*crate::gl!(sPokemonJumpGfx)).msgWindowId == WINDOW_NONE as u16 {
        return FALSE as u32;
    }
    'l1: {
        let sw1: u8 = (*crate::gl!(sPokemonJumpGfx)).msgWindowState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                RemoveWindow((*crate::gl!(sPokemonJumpGfx)).msgWindowId as u8);
                (*crate::gl!(sPokemonJumpGfx)).msgWindowId = WINDOW_NONE as u16;
                (*crate::gl!(sPokemonJumpGfx)).msgWindowState += 1;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 1 {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn HandlePlayAgainInput() -> i8 {
    Menu_ProcessInputNoWrapClearOnChoose()
}
unsafe fn AddMessageWindow(left: u32, top: u32, width: u32, height: u32) -> u32 {
    let mut window: WindowTemplate = zeroed();
    window.bg = BG_INTERFACE;
    window.tilemapLeft = left as u8;
    window.tilemapTop = top as u8;
    window.width = width as u8;
    window.height = height as u8;
    window.paletteNum = 15;
    window.baseBlock = 0x43;
    let windowId: u32 = AddWindow(&raw mut window) as u32;
    FillWindowPixelBuffer(windowId as u8, 0x11);
    windowId
}
unsafe fn CreatePokeJumpYesNoMenu(left: u16, top: u16, cursorPos: u8) {
    let mut window: WindowTemplate = zeroed();
    window.bg = BG_INTERFACE;
    window.tilemapLeft = left as u8;
    window.tilemapTop = top as u8;
    window.width = 6;
    window.height = 4;
    window.paletteNum = 2;
    window.baseBlock = 0x2B;
    CreateYesNoMenu(&raw mut window, 1, 0xD, cursorPos);
}
unsafe fn PrintScoreSuffixes() {
    let mut color: CArray<u8, 3> = CArray([0, 2, 3]);
    PutWindowTilemap(WIN_POINTS);
    PutWindowTilemap(WIN_TIMES);
    FillWindowPixelBuffer(WIN_POINTS, 0);
    FillWindowPixelBuffer(WIN_TIMES, 0);
    AddTextPrinterParameterized3(
        WIN_POINTS,
        FONT_SMALL,
        0,
        1,
        color.as_mut_ptr(),
        0,
        (*(&raw const crate::data::strings::gText_SpacePoints2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        WIN_TIMES,
        FONT_SMALL,
        0,
        1,
        color.as_mut_ptr(),
        0,
        (*(&raw const crate::data::strings::gText_SpaceTimes3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn CreateJumpMonSprites() {
    let mut y: i32 = 0;
    let playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    let mut xCoords: *mut i16 = sMonXCoords[playersCount - 2];
    for i in 0..playersCount {
        let monInfo: *mut PokemonJump_MonInfo = GetMonInfoByMultiplayerId(i as u8);
        y = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
            .cast::<CArray<MonCoords, 0>>())[(*monInfo).species]
            .y_offset as i32;
        CreateJumpMonSprite(
            crate::gl!(sPokemonJumpGfx),
            monInfo,
            *xCoords,
            y as i16 + 112,
            i as u8,
        );
        CreateStarSprite(crate::gl!(sPokemonJumpGfx), *xCoords, 112, i as u8);
        xCoords = xCoords.at(1);
    }
}
unsafe fn SetMonSpriteY(id: u32, y: i16) {
    (*(*crate::gl!(sPokemonJumpGfx)).monSprites[id]).y2 = y;
}
unsafe fn UpdateVineSwing(vineState: i32) {
    UpdateVineAnim(crate::gl!(sPokemonJumpGfx), vineState);
    ChangeBgY(
        BG_VENUSAUR,
        (sVenusaurStates[vineState] as i32 * 5) << 13,
        BG_COORD_SET,
    );
}
unsafe fn DoSameJumpTimeBonus(mut flags: u8) -> i32 {
    let mut numPlayers: i32 = 0;
    for i in 0..MAX_RFU_PLAYERS {
        if flags as i32 & 1 != 0 {
            DoStarAnim(crate::gl!(sPokemonJumpGfx), i);
            numPlayers += 1;
        }
        flags >>= 1;
    }
    ShowBonus(numPlayers as u8 - 2);
    numPlayers
}
unsafe fn InitDigitPrinters() {
    let mut template: DigitObjUtilTemplate = zeroed();
    template.set_shape(0);
    template.set_size(0);
    template.set_strConvMode(0);
    template.set_priority(1);
    template.oamCount = 5;
    template.xDelta = 8;
    template.x = 108;
    template.y = 6;
    template.spriteSheet =
        (&raw const *sSpriteSheet_Digits).cast_mut() as *mut c_void as *mut SpriteSheet;
    template.spritePal = (&raw const *sSpritePalette_Digits).cast_mut();
    DigitObjUtil_Init(NUM_WINDOWS);
    DigitObjUtil_CreatePrinter(WIN_POINTS as u32, 0, &raw mut template);
    template.oamCount = 4;
    template.x = 30;
    template.y = 6;
    DigitObjUtil_CreatePrinter(WIN_TIMES as u32, 0, &raw mut template);
}
unsafe fn PrintScore(num: i32) {
    DigitObjUtil_PrintNumOn(WIN_POINTS as u32, num);
}
unsafe fn PrintJumpsInRow(num: u16) {
    DigitObjUtil_PrintNumOn(WIN_TIMES as u32, num as i32);
}
unsafe fn StartMonHitShake(multiplayerId: u8) {
    Gfx_StartMonHitShake(crate::gl!(sPokemonJumpGfx), multiplayerId as i32);
}
unsafe fn StartMonHitFlash(multiplayerId: u8) {
    Gfx_StartMonHitFlash(crate::gl!(sPokemonJumpGfx), multiplayerId as i32);
}
unsafe fn IsMonHitShakeActive(multiplayerId: i32) -> i32 {
    Gfx_IsMonHitShakeActive(crate::gl!(sPokemonJumpGfx), multiplayerId) as i32
}
unsafe fn StopMonHitFlash() {
    Gfx_StopMonHitFlash(crate::gl!(sPokemonJumpGfx));
}
unsafe fn ResetMonSpriteSubpriorities() {
    Gfx_ResetMonSpriteSubpriorities(crate::gl!(sPokemonJumpGfx));
}
unsafe fn StartMonIntroBounce(multiplayerId: i32) {
    Gfx_StartMonIntroBounce(crate::gl!(sPokemonJumpGfx), multiplayerId);
}
unsafe fn IsMonIntroBounceActive() -> i32 {
    Gfx_IsMonIntroBounceActive(crate::gl!(sPokemonJumpGfx)) as i32
}
unsafe fn AddPlayerNameWindows() {
    let mut window: WindowTemplate = zeroed();
    let playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    let mut winCoords: *mut u16 = sPlayerNameWindowCoords[playersCount - 2];
    window.bg = BG_INTERFACE;
    window.width = 8;
    window.height = 2;
    window.paletteNum = 2;
    window.baseBlock = 0x2B;
    for i in 0..playersCount {
        window.tilemapLeft = *winCoords as u8;
        window.tilemapTop = *winCoords.at(1) as u8;
        (*crate::gl!(sPokemonJumpGfx)).nameWindowIds[i] = AddWindow(&raw mut window);
        ClearWindowTilemap((*crate::gl!(sPokemonJumpGfx)).nameWindowIds[i] as u8);
        window.baseBlock += 0x10;
        winCoords = winCoords.at(2);
    }
    CopyBgTilemapBufferToVram(BG_INTERFACE);
}
unsafe fn PrintPokeJumpPlayerName(multiplayerId: i32, bgColor: u8, fgColor: u8, shadow: u8) {
    let mut colors: CArray<u8, 3> = zeroed();
    colors[0] = bgColor;
    colors[1] = fgColor;
    colors[2] = shadow;
    FillWindowPixelBuffer(
        (*crate::gl!(sPokemonJumpGfx)).nameWindowIds[multiplayerId] as u8,
        0,
    );
    let mut x: u32 =
        64 - GetStringWidth(FONT_NORMAL, GetPokeJumpPlayerName(multiplayerId as u8), -1) as u32;
    x /= 2;
    AddTextPrinterParameterized3(
        (*crate::gl!(sPokemonJumpGfx)).nameWindowIds[multiplayerId] as u8,
        FONT_NORMAL,
        x as u8,
        1,
        colors.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        GetPokeJumpPlayerName(multiplayerId as u8),
    );
    CopyWindowToVram(
        (*crate::gl!(sPokemonJumpGfx)).nameWindowIds[multiplayerId] as u8,
        COPYWIN_GFX,
    );
}
unsafe fn PrintPokeJumpPlayerNames(highlightSelf: u32) {
    let mut multiplayerId: i32 = 0;
    let playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    if highlightSelf == 0 {
        for i in 0..playersCount {
            PrintPokeJumpPlayerName(
                i,
                TEXT_COLOR_TRANSPARENT,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
    } else {
        multiplayerId = GetPokeJumpMultiplayerId() as i32;
        for i in 0..playersCount {
            if multiplayerId != i {
                PrintPokeJumpPlayerName(
                    i,
                    TEXT_COLOR_TRANSPARENT,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_LIGHT_GRAY,
                );
            } else {
                PrintPokeJumpPlayerName(
                    i,
                    TEXT_COLOR_TRANSPARENT,
                    TEXT_COLOR_RED,
                    TEXT_COLOR_LIGHT_RED,
                );
            }
        }
    }
}
pub(crate) unsafe fn DrawPlayerNameWindows() {
    let playersCount: i32 = GetNumPokeJumpPlayers() as i32;
    for i in 0..playersCount {
        PutWindowTilemap((*crate::gl!(sPokemonJumpGfx)).nameWindowIds[i] as u8);
    }
    CopyBgTilemapBufferToVram(BG_INTERFACE);
}
unsafe fn ShowBonus(bonusId: u8) {
    (*crate::gl!(sPokemonJumpGfx)).bonusTimer = 0;
    ChangeBgX(BG_BONUSES, bonusId as i32 / 2 * 256 * 256, BG_COORD_SET);
    ChangeBgY(
        BG_BONUSES,
        (bonusId as i32 % 2 * 256 - 40) * 256,
        BG_COORD_SET,
    );
    ShowBg(BG_BONUSES);
    CreateTask(Some(Task_UpdateBonus), 4);
}
unsafe fn UpdateBonus() -> u32 {
    if (*crate::gl!(sPokemonJumpGfx)).bonusTimer >= 32 {
        return FALSE as u32;
    } else {
        ChangeBgY(BG_BONUSES, 128, BG_COORD_ADD);
        if ({
            (*crate::gl!(sPokemonJumpGfx)).bonusTimer += 1;
            (*crate::gl!(sPokemonJumpGfx)).bonusTimer
        }) >= 32
        {
            HideBg(BG_BONUSES);
        }
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_UpdateBonus(taskId: u8) {
    if UpdateBonus() == 0 {
        DestroyTask(taskId);
    }
}
unsafe fn SendPacket_MonInfo(monInfo: *mut PokemonJump_MonInfo) {
    let mut packet: MonInfoPacket = zeroed();
    packet.id = PACKET_MON_INFO;
    packet.species = (*monInfo).species;
    packet.otId = (*monInfo).otId;
    packet.personality = (*monInfo).personality;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_MonInfo(multiplayerId: i32, monInfo: *mut PokemonJump_MonInfo) -> u32 {
    let mut packet: MonInfoPacket = zeroed();
    if crate::gl!(gRecvCmds[multiplayerId][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id == PACKET_MON_INFO {
        (*monInfo).species = packet.species;
        (*monInfo).otId = packet.otId;
        (*monInfo).personality = packet.personality;
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn SendPacket_Unused(data: u32) {
    let mut packet: UnusedPacket = zeroed();
    packet.id = PACKET_UNUSED;
    packet.data = data;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn SendPacket_LeaderState(player: *mut PokemonJump_Player, comm: *mut PokemonJump_CommData) {
    let mut packet: LeaderStatePacket = zeroed();
    packet.id = PACKET_LEADER_STATE;
    packet.set_jumpScore((*comm).jumpScore);
    packet.set_receivedBonusFlags((*comm).receivedBonusFlags);
    packet.funcId = (*comm).funcId;
    packet.vineTimer = (*comm).data;
    packet.set_jumpsInRow((*comm).jumpsInRow as u32);
    packet.monState = (*player).monState as u8;
    packet.set_jumpState((*player).jumpState as u8);
    packet.jumpTimeStart = (*player).jumpTimeStart;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_LeaderState(
    player: *mut PokemonJump_Player,
    comm: *mut PokemonJump_CommData,
) -> u32 {
    let mut packet: LeaderStatePacket = zeroed();
    if crate::gl!(gRecvCmds[0][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[0][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_LEADER_STATE {
        return FALSE as u32;
    }
    (*comm).jumpScore = packet.jumpScore();
    (*comm).receivedBonusFlags = packet.receivedBonusFlags();
    (*comm).funcId = packet.funcId;
    (*comm).data = packet.vineTimer;
    (*comm).jumpsInRow = packet.jumpsInRow() as u16;
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState() as i32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    TRUE as u32
}
unsafe fn SendPacket_MemberState(player: *mut PokemonJump_Player, funcId: u8, playAgainState: u16) {
    let mut packet: MemberStatePacket = zeroed();
    packet.id = PACKET_MEMBER_STATE;
    packet.monState = (*player).monState as u8;
    packet.jumpState = (*player).jumpState as u8;
    packet.funcFinished = (*player).funcFinished as u8;
    packet.jumpTimeStart = (*player).jumpTimeStart;
    packet.funcId = funcId;
    packet.playAgainState = playAgainState;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_MemberStateToLeader(
    player: *mut PokemonJump_Player,
    multiplayerId: i32,
    funcId: *mut u8,
    playAgainState: *mut u16,
) -> u32 {
    let mut packet: MemberStatePacket = zeroed();
    if crate::gl!(gRecvCmds[multiplayerId][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_MEMBER_STATE {
        return FALSE as u32;
    }
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState as i32;
    (*player).funcFinished = packet.funcFinished as u32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    *funcId = packet.funcId;
    *playAgainState = packet.playAgainState;
    TRUE as u32
}
unsafe fn RecvPacket_MemberStateToMember(
    player: *mut PokemonJump_Player,
    multiplayerId: i32,
) -> u32 {
    let mut packet: MemberStatePacket = zeroed();
    if crate::gl!(gRecvCmds[multiplayerId][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    memcpy(
        &raw mut packet as *mut u8,
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[multiplayerId][1] as *mut u8,
        12,
    );
    if packet.id != PACKET_MEMBER_STATE {
        return FALSE as u32;
    }
    (*player).monState = packet.monState as u16;
    (*player).jumpState = packet.jumpState as i32;
    (*player).funcFinished = packet.funcFinished as u32;
    (*player).jumpTimeStart = packet.jumpTimeStart;
    TRUE as u32
}
unsafe fn GetPokeJumpRecords() -> *mut PokemonJumpRecords {
    &raw mut (*crate::gl!(gSaveBlock2Ptr)).pokeJump
}
#[unsafe(no_mangle)]
pub unsafe fn ResetPokemonJumpRecords() {
    let records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    (*records).jumpsInRow = 0;
    (*records).bestJumpScore = 0;
    (*records).excellentsInRow = 0;
    (*records).gamesWithMaxPlayers = 0;
    (*records).unused2 = 0;
    (*records).unused1 = 0;
}
pub(crate) unsafe fn TryUpdateRecords(
    jumpScore: u32,
    jumpsInRow: u16,
    excellentsInRow: u16,
) -> u32 {
    let records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    let mut newRecord: u32 = FALSE as u32;
    if (*records).bestJumpScore < jumpScore && jumpScore <= MAX_JUMP_SCORE {
        (*records).bestJumpScore = jumpScore;
        newRecord = TRUE as u32;
    }
    if (*records).jumpsInRow < jumpsInRow && jumpsInRow <= MAX_JUMPS {
        (*records).jumpsInRow = jumpsInRow;
        newRecord = TRUE as u32;
    }
    if (*records).excellentsInRow < excellentsInRow && excellentsInRow <= MAX_JUMPS {
        (*records).excellentsInRow = excellentsInRow;
        newRecord = TRUE as u32;
    }
    newRecord
}
unsafe fn IncrementGamesWithMaxPlayers() {
    let records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    if (*records).gamesWithMaxPlayers < 9999 {
        (*records).gamesWithMaxPlayers += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShowPokemonJumpRecords() {
    let taskId: u8 = CreateTask(Some(Task_ShowPokemonJumpRecords), 0);
    Task_ShowPokemonJumpRecords(taskId);
}
pub(crate) unsafe fn Task_ShowPokemonJumpRecords(taskId: u8) {
    let mut window: WindowTemplate = zeroed();
    let mut width: i32 = 0;
    let mut widthCurr: i32 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            window = *sWindowTemplate_Records;
            width = GetStringWidth(
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_PkmnJumpRecords).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
            );
            for i in 0..3i32 {
                widthCurr = GetStringWidth(FONT_NORMAL, sRecordsTexts[i], 0) + 38;
                if widthCurr > width {
                    width = widthCurr;
                }
            }
            width = (width + 7) / 8;
            if width & 1 != 0 {
                width += 1;
            }
            window.tilemapLeft = ((30 - width) / 2) as u8;
            window.width = width as u8;
            *data.at(1) = AddWindow(&raw mut window) as i16;
            PrintRecordsText(*data.at(1) as u16, width);
            CopyWindowToVram(*data.at(1) as u8, COPYWIN_FULL);
            *data += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                *data += 1;
            }
        }
        2 => {
            if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
                rbox_fill_rectangle(*data.at(1) as u8);
                CopyWindowToVram(*data.at(1) as u8, COPYWIN_MAP);
                *data += 1;
            }
        }
        3 if IsDma3ManagerBusyWithBgCopy() == 0 => {
            RemoveWindow(*data.at(1) as u8);
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
        _ => {}
    }
}
pub(crate) unsafe fn PrintRecordsText(windowId: u16, width: i32) {
    let mut x: i32 = 0;
    let mut recordNums: CArray<i32, 3> = zeroed();
    let records: *mut PokemonJumpRecords = GetPokeJumpRecords();
    recordNums[0] = (*records).jumpsInRow as i32;
    recordNums[1] = (*records).bestJumpScore as i32;
    recordNums[2] = (*records).excellentsInRow as i32;
    LoadUserWindowBorderGfx_(windowId as u8, 0x21D, 208);
    DrawTextBorderOuter(windowId as u8, 0x21D, 13);
    FillWindowPixelBuffer(windowId as u8, 17);
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_PkmnJumpRecords).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_PkmnJumpRecords).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            width * 8,
        ) as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    for i in 0..3i32 {
        AddTextPrinterParameterized(
            windowId as u8,
            FONT_NORMAL,
            sRecordsTexts[i],
            0,
            25 + i as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            recordNums[i],
            STR_CONV_MODE_LEFT_ALIGN,
            5,
        );
        TruncateToFirstWordOnly(gStringVar1.as_mut_ptr());
        x = width * 8 - GetStringWidth(FONT_NORMAL, gStringVar1.as_mut_ptr(), 0);
        AddTextPrinterParameterized(
            windowId as u8,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            x as u8,
            25 + i as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    PutWindowTilemap(windowId as u8);
}
unsafe fn TruncateToFirstWordOnly(mut str: *mut u8) {
    while *str != EOS {
        if *str == CHAR_SPACE {
            *str = EOS;
            break;
        }
        str = str.at(1);
    }
}
