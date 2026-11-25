//! Translated from `src/dodrio_berry_picking.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::self_assignment,
    clippy::too_many_arguments,
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
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    FillBgTilemapBufferRect_Palette0, IsDma3ManagerBusyWithBgCopy, ResetBgsAndClearDma3BusyFlags,
    ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_Reset;
use crate::ffi::gSpecialVar_Result;
use crate::gpu_regs::SetGpuReg;
use crate::item::{AddBagItem, CheckBagHasSpace, CopyItemName};
use crate::link::{
    GetBlockReceivedStatus, GetLinkPlayerCount, GetLinkPlayerCountAsBitFlags, GetMultiplayerId,
    IsLinkTaskFinished, ResetBlockReceivedFlags, SendBlock, SetCloseLinkCallback, gLinkPlayers,
    gReceivedRemoteLinkPlayers,
};
use crate::link::{gBlockRecvBuffer, gRecvCmds};
use crate::link_rfu_2::{ClearRecvCommands, Rfu_SendPacket, Rfu_SetLinkStandbyCallback};
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::m4aSongNumStop;
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized3, DecompressAndCopyTileDataToVram,
    DrawDialogueFrame, FreeTempTileDataBuffersIfPossible, InitStandardTextBoxWindows,
    InitTextBoxGfxAndPrinters, ResetTempTileDataBuffers,
};
use crate::minigame_countdown::{IsMinigameCountdownRunning, StartMinigameCountdown};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::pokemon::{GetMonData2, IsMonShiny, gPlayerParty};
use crate::random::Random;
use crate::save::Task_LinkFullSave;
use crate::script::ScriptContext_Enable;
use crate::sound::{
    FadeOutAndFadeInNewMapMusic, FadeOutAndPlayNewMapMusic, IsSEPlaying, PlayFanfareByFanfareNum,
    PlayNewMapMusic, PlaySE, StopMapMusic, WaitFanfare,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar4};
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::text_window::{DrawTextBorderOuter, LoadUserWindowBorderGfx_, rbox_fill_rectangle};
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
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
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
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
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
const sTimer: usize = 1;
const sUnused1: usize = 2;
const sUnused2: usize = 3;
const sUnused3: usize = 4;
// Data tables (translate with cdata.py): sActiveColumnMap sDodrioHeadToColumnMap sDodrioNeighborMap sPlayerIdAtColumn sUnsharedColumns sDuplicateGfx sBerryFallDelays sTreeBorderXPos sDifficultyThresholds sPrizeBerryIds sLeaderFuncs sMemberFuncs sBerryScoreMultipliers sWindowTemplates_Records sRecordsTexts sRecordNumMaxDigits sRecordTextYCoords sRecordNumYCoords sDebug_BerryResults sJPText_Vowels sText_Letters sText_Digits sDebug_PlayerNames sBgTemplates sWindowTemplate_Dummy sWindowTemplates_Results sWindowTemplate_Prize sWindowTemplates_PlayAgain sWindowTemplate_DroppedOut sWindowTemplate_CommStandby sActiveColumnMap_Duplicate sDodrioHeadToColumnMap_Duplicate sDodrioNeighborMap_Duplicate sPlayerIdAtColumn_Duplicate sUnsharedColumns_Duplicate sBg_Pal sDodrioNormal_Pal sDodrioShiny_Pal sStatus_Pal sBerries_Pal sBerries_Gfx sCloud_Pal sBg_Gfx sTreeBorder_Gfx sStatus_Gfx sCloud_Gfx sDodrio_Gfx sBg_Tilemap sTreeBorderRight_Tilemap sTreeBorderLeft_Tilemap sOamData_Dodrio sOamData_16x16_Priority0 sOamData_Berry sOamData_Cloud sAnim_Dodrio_Normal sAnim_Dodrio_PickRight sAnim_Dodrio_PickMiddle sAnim_Dodrio_PickLeft sAnim_Dodrio_Down sAnims_Dodrio sAnims_StatusBar_Yellow sAnims_StatusBar_Gray sAnims_StatusBar_Red sAnims_StatusBar sAnim_Berry_Blue sAnim_Berry_Green sAnim_Berry_Gold sAnim_Berry_BlueSquished sAnim_Berry_GreenSquished sAnim_Berry_GoldSquished sAnim_Berry_Eaten sAnim_Berry_Empty1 sAnim_Berry_Empty2 sAnims_Berry sAnim_Cloud sAnims_Cloud sUnusedSounds sBerryIconXCoords sCloudStartCoords sTextColorTable sNameWindowCoords_1Player sNameWindowCoords_2Players sNameWindowCoords_3Players sNameWindowCoords_4Players sNameWindowCoords_5Players sNameWindowCoords sRankingTexts sResultsXCoords sResultsYCoords sRankingYCoords sGfxFuncs moveDelays.0

/// `struct DodrioGame`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DodrioGame {
    pub exitCallback: Option<unsafe fn()>,
    pub taskId: u8,
    _pad0: [u8; 3],
    pub playersReceived: u8,
    _pad1: [u8; 3],
    pub startState: u8,
    _pad2: [u8; 3],
    pub state: u8,
    _pad3: [u8; 3],
    pub timer: u8,
    _pad4: [u8; 3],
    pub funcId: u8,
    _pad5: [u8; 3],
    pub prevFuncId: u8,
    _pad6: [u8; 3],
    pub isLeader: u8,
    _pad7: [u8; 3],
    pub numPlayers: u8,
    _pad8: [u8; 3],
    pub multiplayerId: u8,
    pub unused1: CArray<u8, 7>,
    pub countdownEndDelay: u8,
    _pad9: [u8; 3],
    pub posToPlayerId: CArray<u8, 5>,
    _pad10: [u8; 3],
    pub unused2: u8,
    _pad11: [u8; 3],
    pub numGraySquares: u8,
    _pad12: [u8; 3],
    pub berryColStart: u8,
    _pad13: [u8; 3],
    pub berryColEnd: u8,
    pub berryResults: CArray<CArray<u16, 6>, 5>,
    pub berriesEaten: CArray<u16, 5>,
    pub difficulty: CArray<u8, 5>,
    _pad14: [u8; 3],
    pub pickStateQueue: CArray<u8, 4>,
    pub eatTimer: CArray<u8, 11>,
    _pad15: [u8; 1],
    pub inputState: CArray<u8, 5>,
    _pad16: [u8; 3],
    pub inputDelay: CArray<u8, 5>,
    _pad17: [u8; 3],
    pub berryEatenBy: CArray<u8, 11>,
    _pad18: [u8; 1],
    pub berryState: CArray<u8, 11>,
    _pad19: [u8; 1],
    pub fallTimer: CArray<u8, 11>,
    _pad20: [u8; 1],
    pub newBerryTimer: CArray<u8, 11>,
    _pad21: [u8; 1],
    pub prevBerryIds: CArray<u8, 11>,
    _pad22: [u8; 1],
    pub playersAttemptingPick: CArray<CArray<u8, 2>, 11>,
    _pad23: [u8; 2],
    pub playAgainStates: CArray<u8, 5>,
    pub berriesPickedInRow: u16,
    pub maxBerriesPickedInRow: u16,
    pub startCountdown: u32,
    pub startGame: u32,
    pub berriesFalling: u32,
    pub clearRecvCmdTimer: u8,
    _pad24: [u8; 3],
    pub clearRecvCmds: u8,
    pub allReadyToEnd: u32,
    pub readyToEnd: CArray<u32, 5>,
    pub playingPickSound: u8,
    _pad25: [u8; 3],
    pub playingSquishSound: CArray<u8, 11>,
    _pad26: [u8; 1],
    pub endSoundState: u8,
    _pad27: [u8; 3],
    pub readyToStart: CArray<u8, 5>,
    pub gfx: DodrioGame_Gfx,
    pub monInfo: CArray<DodrioGame_MonInfo, 5>,
    pub players: CArray<DodrioGame_Player, 5>,
    pub player: DodrioGame_Player,
    pub scoreResults: CArray<DodrioGame_ScoreResults, 5>,
}

unsafe impl Sync for DodrioGame {}

/// `struct StatusBar`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct StatusBar {
    pub unused: CArray<u8, 12>,
    pub entered: CArray<u8, 10>,
    pub yChange: CArray<i16, 10>,
    pub spriteIds: CArray<u16, 10>,
    pub flashTimer: u16,
}

unsafe impl Sync for StatusBar {}

/// `struct DodrioGame_Gfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DodrioGame_Gfx {
    pub tilemapBuffers: CArray<CArray<u16, 2048>, 3>,
    pub finished: u32,
    pub taskId: u8,
    _pad0: [u8; 3],
    pub windowIds: CArray<u8, 10>,
    _pad1: [u8; 2],
    pub state: u8,
    _pad2: [u8; 3],
    pub loadState: u8,
    _pad3: [u8; 3],
    pub timer: u16,
    _pad4: [u8; 2],
    pub cursorSelection: u8,
    _pad5: [u8; 3],
    pub playAgainState: u8,
    pub func: Option<unsafe fn()>,
}

unsafe impl Sync for DodrioGame_Gfx {}

/// `struct DodrioGame_Player`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DodrioGame_Player {
    pub name: CArray<u8, 16>,
    pub receivedGameStatePacket: u32,
    pub berries: DodrioGame_Berries,
    pub comm: DodrioGame_PlayerCommData,
    pub unused: u32,
}

unsafe impl Sync for DodrioGame_Player {}

/// `struct DodrioGame_MonInfo`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DodrioGame_MonInfo {
    pub isShiny: u8,
}

unsafe impl Sync for DodrioGame_MonInfo {}

/// `struct DodrioGame_PlayerCommData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DodrioGame_PlayerCommData {
    pub pickState: u8,
    _pad0: [u8; 3],
    pub ateBerry: u8,
    _pad1: [u8; 3],
    pub missedBerry: u8,
}

unsafe impl Sync for DodrioGame_PlayerCommData {}

/// `struct DodrioGame_Berries`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DodrioGame_Berries {
    pub ids: CArray<u8, 11>,
    pub fallDist: CArray<u8, 11>,
}

unsafe impl Sync for DodrioGame_Berries {}

/// `struct DodrioGame_ScoreResults`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DodrioGame_ScoreResults {
    pub ranking: u8,
    pub score: u32,
}

unsafe impl Sync for DodrioGame_ScoreResults {}

/// `struct ReadyToStartPacket`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ReadyToStartPacket {
    pub id: u8,
    _pad0: [u8; 3],
    pub ready: u8,
}

unsafe impl Sync for ReadyToStartPacket {}

/// `struct GameStatePacket`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct GameStatePacket {
    pub id: u8,
    bits_1: u8,
    bits_2: u8,
    bits_3: u8,
    bits_4: u8,
    bits_5: u8,
    bits_6: u8,
    bits_7: u8,
    bits_8: u8,
    bits_9: u8,
    bits_10: u8,
    bits_11: u8,
}

impl GameStatePacket {
    #[inline(always)]
    pub fn fallDist_Col0(&self) -> u8 {
        ((self.bits_1 as u32) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_fallDist_Col0(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !0xf) | (v & 0xf);
    }
    #[inline(always)]
    pub fn fallDist_Col1(&self) -> u8 {
        ((self.bits_1 as u32 >> 4) & 0xf) as u8
    }
    #[inline(always)]
    pub fn set_fallDist_Col1(&mut self, v: u8) {
        self.bits_1 = (self.bits_1 & !(0xf << 4)) | ((v & 0xf) << 4);
    }
    #[inline(always)]
    pub fn fallDist_Col2(&self) -> u16 {
        ((self.bits_2 as u32) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col2(&mut self, v: u16) {
        self.bits_2 = (self.bits_2 & !0xf) | (v as u8 & 0xf);
    }
    #[inline(always)]
    pub fn fallDist_Col3(&self) -> u16 {
        ((self.bits_2 as u32 >> 4) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col3(&mut self, v: u16) {
        self.bits_2 = (self.bits_2 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn fallDist_Col4(&self) -> u16 {
        ((self.bits_3 as u32) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col4(&mut self, v: u16) {
        self.bits_3 = (self.bits_3 & !0xf) | (v as u8 & 0xf);
    }
    #[inline(always)]
    pub fn fallDist_Col5(&self) -> u16 {
        ((self.bits_3 as u32 >> 4) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col5(&mut self, v: u16) {
        self.bits_3 = (self.bits_3 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn fallDist_Col6(&self) -> u16 {
        ((self.bits_4 as u32) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col6(&mut self, v: u16) {
        self.bits_4 = (self.bits_4 & !0xf) | (v as u8 & 0xf);
    }
    #[inline(always)]
    pub fn fallDist_Col7(&self) -> u16 {
        ((self.bits_4 as u32 >> 4) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col7(&mut self, v: u16) {
        self.bits_4 = (self.bits_4 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn fallDist_Col8(&self) -> u16 {
        ((self.bits_5 as u32) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col8(&mut self, v: u16) {
        self.bits_5 = (self.bits_5 & !0xf) | (v as u8 & 0xf);
    }
    #[inline(always)]
    pub fn fallDist_Col9(&self) -> u16 {
        ((self.bits_5 as u32 >> 4) & 0xf) as u16
    }
    #[inline(always)]
    pub fn set_fallDist_Col9(&mut self, v: u16) {
        self.bits_5 = (self.bits_5 & !(0xf << 4)) | ((v as u8 & 0xf) << 4);
    }
    #[inline(always)]
    pub fn berryId_Col0(&self) -> u16 {
        ((self.bits_6 as u32) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col0(&mut self, v: u16) {
        self.bits_6 = (self.bits_6 & !0x3) | (v as u8 & 0x3);
    }
    #[inline(always)]
    pub fn berryId_Col1(&self) -> u16 {
        ((self.bits_6 as u32 >> 2) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col1(&mut self, v: u16) {
        self.bits_6 = (self.bits_6 & !(0x3 << 2)) | ((v as u8 & 0x3) << 2);
    }
    #[inline(always)]
    pub fn berryId_Col2(&self) -> u16 {
        ((self.bits_6 as u32 >> 4) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col2(&mut self, v: u16) {
        self.bits_6 = (self.bits_6 & !(0x3 << 4)) | ((v as u8 & 0x3) << 4);
    }
    #[inline(always)]
    pub fn berryId_Col3(&self) -> u16 {
        ((self.bits_6 as u32 >> 6) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col3(&mut self, v: u16) {
        self.bits_6 = (self.bits_6 & !(0x3 << 6)) | ((v as u8 & 0x3) << 6);
    }
    #[inline(always)]
    pub fn berryId_Col4(&self) -> u16 {
        ((self.bits_7 as u32) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col4(&mut self, v: u16) {
        self.bits_7 = (self.bits_7 & !0x3) | (v as u8 & 0x3);
    }
    #[inline(always)]
    pub fn berryId_Col5(&self) -> u16 {
        ((self.bits_7 as u32 >> 2) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col5(&mut self, v: u16) {
        self.bits_7 = (self.bits_7 & !(0x3 << 2)) | ((v as u8 & 0x3) << 2);
    }
    #[inline(always)]
    pub fn berryId_Col6(&self) -> u16 {
        ((self.bits_7 as u32 >> 4) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col6(&mut self, v: u16) {
        self.bits_7 = (self.bits_7 & !(0x3 << 4)) | ((v as u8 & 0x3) << 4);
    }
    #[inline(always)]
    pub fn berryId_Col7(&self) -> u16 {
        ((self.bits_7 as u32 >> 6) & 0x3) as u16
    }
    #[inline(always)]
    pub fn set_berryId_Col7(&mut self, v: u16) {
        self.bits_7 = (self.bits_7 & !(0x3 << 6)) | ((v as u8 & 0x3) << 6);
    }
    #[inline(always)]
    pub fn berryId_Col8(&self) -> u8 {
        ((self.bits_8 as u32) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_berryId_Col8(&mut self, v: u8) {
        self.bits_8 = (self.bits_8 & !0x3) | (v & 0x3);
    }
    #[inline(always)]
    pub fn berryId_Col9(&self) -> u8 {
        ((self.bits_8 as u32 >> 2) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_berryId_Col9(&mut self, v: u8) {
        self.bits_8 = (self.bits_8 & !(0x3 << 2)) | ((v & 0x3) << 2);
    }
    #[inline(always)]
    pub fn pickState_Player1(&self) -> u8 {
        ((self.bits_8 as u32 >> 4) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_pickState_Player1(&mut self, v: u8) {
        self.bits_8 = (self.bits_8 & !(0x3 << 4)) | ((v & 0x3) << 4);
    }
    #[inline(always)]
    pub fn pickState_Player2(&self) -> u8 {
        ((self.bits_8 as u32 >> 6) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_pickState_Player2(&mut self, v: u8) {
        self.bits_8 = (self.bits_8 & !(0x3 << 6)) | ((v & 0x3) << 6);
    }
    #[inline(always)]
    pub fn pickState_Player3(&self) -> u8 {
        ((self.bits_9 as u32) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_pickState_Player3(&mut self, v: u8) {
        self.bits_9 = (self.bits_9 & !0x3) | (v & 0x3);
    }
    #[inline(always)]
    pub fn pickState_Player4(&self) -> u8 {
        ((self.bits_9 as u32 >> 2) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_pickState_Player4(&mut self, v: u8) {
        self.bits_9 = (self.bits_9 & !(0x3 << 2)) | ((v & 0x3) << 2);
    }
    #[inline(always)]
    pub fn pickState_Player5(&self) -> u8 {
        ((self.bits_9 as u32 >> 4) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_pickState_Player5(&mut self, v: u8) {
        self.bits_9 = (self.bits_9 & !(0x3 << 4)) | ((v & 0x3) << 4);
    }
    #[inline(always)]
    pub fn ateBerry_Player1(&self) -> u8 {
        ((self.bits_9 as u32 >> 6) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ateBerry_Player1(&mut self, v: u8) {
        self.bits_9 = (self.bits_9 & !(0x1 << 6)) | ((v & 0x1) << 6);
    }
    #[inline(always)]
    pub fn ateBerry_Player2(&self) -> u8 {
        ((self.bits_9 as u32 >> 7) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ateBerry_Player2(&mut self, v: u8) {
        self.bits_9 = (self.bits_9 & !(0x1 << 7)) | ((v & 0x1) << 7);
    }
    #[inline(always)]
    pub fn ateBerry_Player3(&self) -> u8 {
        ((self.bits_10 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ateBerry_Player3(&mut self, v: u8) {
        self.bits_10 = (self.bits_10 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn ateBerry_Player4(&self) -> u8 {
        ((self.bits_10 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ateBerry_Player4(&mut self, v: u8) {
        self.bits_10 = (self.bits_10 & !(0x1 << 1)) | ((v & 0x1) << 1);
    }
    #[inline(always)]
    pub fn ateBerry_Player5(&self) -> u8 {
        ((self.bits_10 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_ateBerry_Player5(&mut self, v: u8) {
        self.bits_10 = (self.bits_10 & !(0x1 << 2)) | ((v & 0x1) << 2);
    }
    #[inline(always)]
    pub fn numGraySquares(&self) -> u8 {
        ((self.bits_10 as u32 >> 3) & 0x1f) as u8
    }
    #[inline(always)]
    pub fn set_numGraySquares(&mut self, v: u8) {
        self.bits_10 = (self.bits_10 & !(0x1f << 3)) | ((v & 0x1f) << 3);
    }
    #[inline(always)]
    pub fn allReadyToEnd(&self) -> u8 {
        ((self.bits_11 as u32) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_allReadyToEnd(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn berriesFalling(&self) -> u8 {
        ((self.bits_11 as u32 >> 1) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_berriesFalling(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 1)) | ((v & 0x1) << 1);
    }
    #[inline(always)]
    pub fn missedBerry_Player1(&self) -> u8 {
        ((self.bits_11 as u32 >> 2) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_missedBerry_Player1(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 2)) | ((v & 0x1) << 2);
    }
    #[inline(always)]
    pub fn missedBerry_Player2(&self) -> u8 {
        ((self.bits_11 as u32 >> 3) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_missedBerry_Player2(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 3)) | ((v & 0x1) << 3);
    }
    #[inline(always)]
    pub fn missedBerry_Player3(&self) -> u8 {
        ((self.bits_11 as u32 >> 4) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_missedBerry_Player3(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 4)) | ((v & 0x1) << 4);
    }
    #[inline(always)]
    pub fn missedBerry_Player4(&self) -> u8 {
        ((self.bits_11 as u32 >> 5) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_missedBerry_Player4(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 5)) | ((v & 0x1) << 5);
    }
    #[inline(always)]
    pub fn missedBerry_Player5(&self) -> u8 {
        ((self.bits_11 as u32 >> 6) & 0x1) as u8
    }
    #[inline(always)]
    pub fn set_missedBerry_Player5(&mut self, v: u8) {
        self.bits_11 = (self.bits_11 & !(0x1 << 6)) | ((v & 0x1) << 6);
    }
}

unsafe impl Sync for GameStatePacket {}

/// `struct PickStatePacket`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PickStatePacket {
    pub id: u8,
    _pad0: [u8; 3],
    pub pickState: u8,
}

unsafe impl Sync for PickStatePacket {}

/// `struct ReadyToEndPacket`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReadyToEndPacket {
    pub id: u8,
    pub ready: u32,
}

unsafe impl Sync for ReadyToEndPacket {}

/// `struct WinCoords`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct WinCoords {
    pub left: u8,
    pub top: u8,
}

unsafe impl Sync for WinCoords {}

/// `__typeof__(sGfxFuncs[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sGfxFuncs_0_t {
    pub id: u8,
    pub func: Option<unsafe fn()>,
}

unsafe impl Sync for sGfxFuncs_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<DodrioGame>() == 13104);
    assert!(offset_of!(DodrioGame, exitCallback) == 0);
    assert!(offset_of!(DodrioGame, taskId) == 4);
    assert!(offset_of!(DodrioGame, playersReceived) == 8);
    assert!(offset_of!(DodrioGame, startState) == 12);
    assert!(offset_of!(DodrioGame, state) == 16);
    assert!(offset_of!(DodrioGame, timer) == 20);
    assert!(offset_of!(DodrioGame, funcId) == 24);
    assert!(offset_of!(DodrioGame, prevFuncId) == 28);
    assert!(offset_of!(DodrioGame, isLeader) == 32);
    assert!(offset_of!(DodrioGame, numPlayers) == 36);
    assert!(offset_of!(DodrioGame, multiplayerId) == 40);
    assert!(offset_of!(DodrioGame, unused1) == 41);
    assert!(offset_of!(DodrioGame, countdownEndDelay) == 48);
    assert!(offset_of!(DodrioGame, posToPlayerId) == 52);
    assert!(offset_of!(DodrioGame, unused2) == 60);
    assert!(offset_of!(DodrioGame, numGraySquares) == 64);
    assert!(offset_of!(DodrioGame, berryColStart) == 68);
    assert!(offset_of!(DodrioGame, berryColEnd) == 72);
    assert!(offset_of!(DodrioGame, berryResults) == 74);
    assert!(offset_of!(DodrioGame, berriesEaten) == 134);
    assert!(offset_of!(DodrioGame, difficulty) == 144);
    assert!(offset_of!(DodrioGame, pickStateQueue) == 152);
    assert!(offset_of!(DodrioGame, eatTimer) == 156);
    assert!(offset_of!(DodrioGame, inputState) == 168);
    assert!(offset_of!(DodrioGame, inputDelay) == 176);
    assert!(offset_of!(DodrioGame, berryEatenBy) == 184);
    assert!(offset_of!(DodrioGame, berryState) == 196);
    assert!(offset_of!(DodrioGame, fallTimer) == 208);
    assert!(offset_of!(DodrioGame, newBerryTimer) == 220);
    assert!(offset_of!(DodrioGame, prevBerryIds) == 232);
    assert!(offset_of!(DodrioGame, playersAttemptingPick) == 244);
    assert!(offset_of!(DodrioGame, playAgainStates) == 268);
    assert!(offset_of!(DodrioGame, berriesPickedInRow) == 274);
    assert!(offset_of!(DodrioGame, maxBerriesPickedInRow) == 276);
    assert!(offset_of!(DodrioGame, startCountdown) == 280);
    assert!(offset_of!(DodrioGame, startGame) == 284);
    assert!(offset_of!(DodrioGame, berriesFalling) == 288);
    assert!(offset_of!(DodrioGame, clearRecvCmdTimer) == 292);
    assert!(offset_of!(DodrioGame, clearRecvCmds) == 296);
    assert!(offset_of!(DodrioGame, allReadyToEnd) == 300);
    assert!(offset_of!(DodrioGame, readyToEnd) == 304);
    assert!(offset_of!(DodrioGame, playingPickSound) == 324);
    assert!(offset_of!(DodrioGame, playingSquishSound) == 328);
    assert!(offset_of!(DodrioGame, endSoundState) == 340);
    assert!(offset_of!(DodrioGame, readyToStart) == 344);
    assert!(offset_of!(DodrioGame, gfx) == 352);
    assert!(offset_of!(DodrioGame, monInfo) == 12684);
    assert!(offset_of!(DodrioGame, players) == 12704);
    assert!(offset_of!(DodrioGame, player) == 13004);
    assert!(offset_of!(DodrioGame, scoreResults) == 13064);
    assert!(size_of::<StatusBar>() == 64);
    assert!(offset_of!(StatusBar, unused) == 0);
    assert!(offset_of!(StatusBar, entered) == 12);
    assert!(offset_of!(StatusBar, yChange) == 22);
    assert!(offset_of!(StatusBar, spriteIds) == 42);
    assert!(offset_of!(StatusBar, flashTimer) == 62);
    assert!(size_of::<DodrioGame_Gfx>() == 12332);
    assert!(offset_of!(DodrioGame_Gfx, tilemapBuffers) == 0);
    assert!(offset_of!(DodrioGame_Gfx, finished) == 12288);
    assert!(offset_of!(DodrioGame_Gfx, taskId) == 12292);
    assert!(offset_of!(DodrioGame_Gfx, windowIds) == 12296);
    assert!(offset_of!(DodrioGame_Gfx, state) == 12308);
    assert!(offset_of!(DodrioGame_Gfx, loadState) == 12312);
    assert!(offset_of!(DodrioGame_Gfx, timer) == 12316);
    assert!(offset_of!(DodrioGame_Gfx, cursorSelection) == 12320);
    assert!(offset_of!(DodrioGame_Gfx, playAgainState) == 12324);
    assert!(offset_of!(DodrioGame_Gfx, func) == 12328);
    assert!(size_of::<DodrioGame_Player>() == 60);
    assert!(offset_of!(DodrioGame_Player, name) == 0);
    assert!(offset_of!(DodrioGame_Player, receivedGameStatePacket) == 16);
    assert!(offset_of!(DodrioGame_Player, berries) == 20);
    assert!(offset_of!(DodrioGame_Player, comm) == 44);
    assert!(offset_of!(DodrioGame_Player, unused) == 56);
    assert!(size_of::<DodrioGame_MonInfo>() == 4);
    assert!(offset_of!(DodrioGame_MonInfo, isShiny) == 0);
    assert!(size_of::<DodrioGame_PlayerCommData>() == 12);
    assert!(offset_of!(DodrioGame_PlayerCommData, pickState) == 0);
    assert!(offset_of!(DodrioGame_PlayerCommData, ateBerry) == 4);
    assert!(offset_of!(DodrioGame_PlayerCommData, missedBerry) == 8);
    assert!(size_of::<DodrioGame_Berries>() == 24);
    assert!(offset_of!(DodrioGame_Berries, ids) == 0);
    assert!(offset_of!(DodrioGame_Berries, fallDist) == 11);
    assert!(size_of::<DodrioGame_ScoreResults>() == 8);
    assert!(offset_of!(DodrioGame_ScoreResults, ranking) == 0);
    assert!(offset_of!(DodrioGame_ScoreResults, score) == 4);
    assert!(size_of::<ReadyToStartPacket>() == 8);
    assert!(offset_of!(ReadyToStartPacket, id) == 0);
    assert!(offset_of!(ReadyToStartPacket, ready) == 4);
    assert!(size_of::<GameStatePacket>() == 12);
    assert!(offset_of!(GameStatePacket, id) == 0);
    assert!(offset_of!(GameStatePacket, bits_1) == 1);
    assert!(offset_of!(GameStatePacket, bits_2) == 2);
    assert!(offset_of!(GameStatePacket, bits_3) == 3);
    assert!(offset_of!(GameStatePacket, bits_4) == 4);
    assert!(offset_of!(GameStatePacket, bits_5) == 5);
    assert!(offset_of!(GameStatePacket, bits_6) == 6);
    assert!(offset_of!(GameStatePacket, bits_7) == 7);
    assert!(offset_of!(GameStatePacket, bits_8) == 8);
    assert!(offset_of!(GameStatePacket, bits_9) == 9);
    assert!(offset_of!(GameStatePacket, bits_10) == 10);
    assert!(offset_of!(GameStatePacket, bits_11) == 11);
    assert!(size_of::<PickStatePacket>() == 8);
    assert!(offset_of!(PickStatePacket, id) == 0);
    assert!(offset_of!(PickStatePacket, pickState) == 4);
    assert!(size_of::<ReadyToEndPacket>() == 8);
    assert!(offset_of!(ReadyToEndPacket, id) == 0);
    assert!(offset_of!(ReadyToEndPacket, ready) == 4);
    assert!(size_of::<WinCoords>() == 4);
    assert!(offset_of!(WinCoords, left) == 0);
    assert!(offset_of!(WinCoords, top) == 1);
    assert!(size_of::<sGfxFuncs_0_t>() == 8);
    assert!(offset_of!(sGfxFuncs_0_t, id) == 0);
    assert!(offset_of!(sGfxFuncs_0_t, func) == 4);
};

const BERRYSTATE_EATEN: u8 = 2;
const BERRYSTATE_NONE: u8 = 0;
const BERRYSTATE_PICKED: u8 = 1;
const BERRYSTATE_SQUISHED: u8 = 3;
const BERRY_BLUE: u8 = 0;
const BERRY_GOLD: u8 = 2;
const BERRY_GREEN: u8 = 1;
const BERRY_IN_ROW: i32 = 5;
const BERRY_MISSED: u8 = 3;
const BERRY_PRIZE: i32 = 4;
const BG_INTERFACE: u8 = 0;
const BG_SCENERY: u8 = 3;
const BG_TREE_LEFT: u8 = 1;
const BG_TREE_RIGHT: u8 = 2;
const COLORID_BLUE: u8 = 2;
const COLORID_GRAY: u8 = 0;
const COLORID_RED: i32 = 1;
const EAT_FALL_DIST: u8 = 7;
const FRAMES_PER_STATE: i32 = 13;
const FUNC_ASK_PLAY_AGAIN: u8 = 7;
const FUNC_COUNTDOWN: u8 = 2;
const FUNC_END_LINK: u8 = 8;
const FUNC_EXIT: u8 = 9;
const FUNC_INIT_COUNTDOWN: u8 = 1;
const FUNC_INIT_RESULTS: u8 = 5;
const FUNC_INTRO: u8 = 0;
const FUNC_PLAY_GAME: u8 = 4;
const FUNC_RESET_GAME: u8 = 10;
const FUNC_RESULTS: u8 = 6;
const FUNC_WAIT_END_GAME: u8 = 11;
const FUNC_WAIT_START: u8 = 3;
const GFXFUNC_ERASE_MSG: u8 = 6;
const GFXFUNC_IDLE: u8 = 9;
const GFXFUNC_MSG_COMM_STANDBY: u8 = 5;
const GFXFUNC_MSG_PLAYER_DROPPED: u8 = 7;
const GFXFUNC_MSG_PLAY_AGAIN: u8 = 3;
const GFXFUNC_MSG_SAVING: u8 = 4;
const GFXFUNC_SHOW_NAMES: u8 = 1;
const GFXFUNC_SHOW_RESULTS: u8 = 2;
const GFXFUNC_STOP: u8 = 8;
const GFXTAG_BERRIES: u16 = 2;
const GFXTAG_CLOUD: u16 = 5;
const GFXTAG_COUNTDOWN: u16 = 7;
const GFXTAG_DODRIO: u16 = 0;
const GFXTAG_STATUS: u16 = 1;
const INPUTSTATE_ATE_BERRY: u8 = 3;
const INPUTSTATE_BAD_MISS: u8 = 4;
const INPUTSTATE_NONE: u8 = 0;
const INPUTSTATE_PICKED: u8 = 2;
const INPUTSTATE_TRY_PICK: u8 = 1;
const MAX_BERRIES: u32 = 9999;
const MAX_FALL_DIST: u8 = 10;
const MAX_SCORE: u32 = 0xf4236;
const NO_PRIZE: u8 = 3;
const NUM_BERRY_COLUMNS: u8 = 11;
const NUM_BERRY_TYPES: u8 = 4;
const NUM_CLOUDS: u8 = 2;
const NUM_DIFFICULTIES: i32 = 7;
const NUM_RECORD_TYPES: i32 = 3;
const NUM_STATUS_SQUARES: u8 = 10;
const PACKET_GAME_STATE: u8 = 2;
const PACKET_PICK_STATE: u8 = 3;
const PACKET_READY_END: u8 = 4;
const PACKET_READY_START: u8 = 1;
const PALTAG_BERRIES: u16 = 3;
const PALTAG_CLOUD: u16 = 6;
const PALTAG_COUNTDOWN: u16 = 8;
const PALTAG_DODRIO_NORMAL: u16 = 0;
const PALTAG_DODRIO_SHINY: u16 = 1;
const PALTAG_STATUS: u16 = 2;
const PICK_DISABLED: u8 = 4;
const PICK_LEFT: u8 = 3;
const PICK_MIDDLE: u8 = 2;
const PICK_NONE: u8 = 0;
const PICK_RIGHT: u8 = 1;
const PLAYER_NONE: u8 = 255;
const PLAY_AGAIN_DROPPED: u8 = 5;
const PLAY_AGAIN_NO: u8 = 2;
const PLAY_AGAIN_NONE: u8 = 0;
const PLAY_AGAIN_YES: u8 = 1;
const PRIZE_FILLED_BAG: u8 = 1;
const PRIZE_NO_ROOM: u8 = 2;
const PRIZE_RECEIVED: u8 = 0;
const PRIZE_SCORE: u32 = 3000;
const STATUS_GRAY: u8 = 1;
const STATUS_RED: u8 = 2;
const STATUS_YELLOW: u8 = 0;
const WIN_PLAY_AGAIN: i32 = 0;
const WIN_YES_NO: i32 = 1;

static moveDelays_0: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::dodrio_berry_picking::moveDelays_0).cast());
static sActiveColumnMap: Table<CArray<CArray<CArray<u8, 11>, 5>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sActiveColumnMap).cast());
static sAnims_Berry: Table<CArray<*mut AnimCmd, 9>> =
    Table((&raw const crate::data::dodrio_berry_picking::sAnims_Berry).cast());
static sAnims_Cloud: Table<CArray<*mut AnimCmd, 1>> =
    Table((&raw const crate::data::dodrio_berry_picking::sAnims_Cloud).cast());
static sAnims_Dodrio: Table<CArray<*mut AnimCmd, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sAnims_Dodrio).cast());
static sAnims_StatusBar: Table<CArray<*mut AnimCmd, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sAnims_StatusBar).cast());
static sBerries_Gfx: Table<CArray<u32, 109>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBerries_Gfx).cast());
static sBerries_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBerries_Pal).cast());
static sBerryFallDelays: Table<CArray<CArray<u8, 3>, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBerryFallDelays).cast());
static sBerryIconXCoords: Table<CArray<i16, 4>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBerryIconXCoords).cast());
static sBerryScoreMultipliers: Table<CArray<i16, 4>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBerryScoreMultipliers).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBgTemplates).cast());
static sBg_Gfx: Table<CArray<u32, 548>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBg_Gfx).cast());
static sBg_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBg_Pal).cast());
static sBg_Tilemap: Table<CArray<u32, 155>> =
    Table((&raw const crate::data::dodrio_berry_picking::sBg_Tilemap).cast());
static sCloudStartCoords: Table<CArray<CArray<i16, 2>, 2>> =
    Table((&raw const crate::data::dodrio_berry_picking::sCloudStartCoords).cast());
static sCloud_Gfx: Table<CArray<u32, 82>> =
    Table((&raw const crate::data::dodrio_berry_picking::sCloud_Gfx).cast());
static sCloud_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::dodrio_berry_picking::sCloud_Pal).cast());
static sDebug_BerryResults: Table<CArray<CArray<u16, 4>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDebug_BerryResults).cast());
static sDebug_PlayerNames: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDebug_PlayerNames).cast());
static sDifficultyThresholds: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDifficultyThresholds).cast());
static sDodrioHeadToColumnMap: Table<CArray<CArray<CArray<u8, 3>, 5>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDodrioHeadToColumnMap).cast());
static sDodrioNeighborMap: Table<CArray<CArray<CArray<u8, 3>, 5>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDodrioNeighborMap).cast());
static sDodrioNormal_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDodrioNormal_Pal).cast());
static sDodrioShiny_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDodrioShiny_Pal).cast());
static sDodrio_Gfx: Table<CArray<u32, 1159>> =
    Table((&raw const crate::data::dodrio_berry_picking::sDodrio_Gfx).cast());
static sGfxFuncs: Table<CArray<sGfxFuncs_0_t, 10>> =
    Table((&raw const crate::data::dodrio_berry_picking::sGfxFuncs).cast());
static sLeaderFuncs: Table<CArray<Option<unsafe fn()>, 12>> =
    Table((&raw const crate::data::dodrio_berry_picking::sLeaderFuncs).cast());
static sMemberFuncs: Table<CArray<Option<unsafe fn()>, 12>> =
    Table((&raw const crate::data::dodrio_berry_picking::sMemberFuncs).cast());
static sNameWindowCoords: Table<CArray<*mut WinCoords, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sNameWindowCoords).cast());
static sOamData_16x16_Priority0: Table<OamData> =
    Table((&raw const crate::data::dodrio_berry_picking::sOamData_16x16_Priority0).cast());
static sOamData_Berry: Table<OamData> =
    Table((&raw const crate::data::dodrio_berry_picking::sOamData_Berry).cast());
static sOamData_Cloud: Table<OamData> =
    Table((&raw const crate::data::dodrio_berry_picking::sOamData_Cloud).cast());
static sOamData_Dodrio: Table<OamData> =
    Table((&raw const crate::data::dodrio_berry_picking::sOamData_Dodrio).cast());
static sPlayerIdAtColumn: Table<CArray<CArray<u8, 11>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sPlayerIdAtColumn).cast());
static sPrizeBerryIds: Table<CArray<CArray<u8, 10>, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sPrizeBerryIds).cast());
static sRankingTexts: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRankingTexts).cast());
static sRankingYCoords: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRankingYCoords).cast());
static sRecordNumMaxDigits: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRecordNumMaxDigits).cast());
static sRecordNumYCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRecordNumYCoords).cast());
static sRecordTextYCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRecordTextYCoords).cast());
static sRecordsTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::dodrio_berry_picking::sRecordsTexts).cast());
static sResultsXCoords: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::dodrio_berry_picking::sResultsXCoords).cast());
static sResultsYCoords: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sResultsYCoords).cast());
static sStatus_Gfx: Table<CArray<u32, 37>> =
    Table((&raw const crate::data::dodrio_berry_picking::sStatus_Gfx).cast());
static sStatus_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::dodrio_berry_picking::sStatus_Pal).cast());
static sTextColorTable: Table<CArray<CArray<u8, 3>, 4>> =
    Table((&raw const crate::data::dodrio_berry_picking::sTextColorTable).cast());
static sTreeBorderLeft_Tilemap: Table<CArray<u32, 147>> =
    Table((&raw const crate::data::dodrio_berry_picking::sTreeBorderLeft_Tilemap).cast());
static sTreeBorderRight_Tilemap: Table<CArray<u32, 148>> =
    Table((&raw const crate::data::dodrio_berry_picking::sTreeBorderRight_Tilemap).cast());
static sTreeBorderXPos: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sTreeBorderXPos).cast());
static sTreeBorder_Gfx: Table<CArray<u32, 883>> =
    Table((&raw const crate::data::dodrio_berry_picking::sTreeBorder_Gfx).cast());
static sUnsharedColumns: Table<CArray<CArray<u8, 5>, 5>> =
    Table((&raw const crate::data::dodrio_berry_picking::sUnsharedColumns).cast());
static sWindowTemplate_CommStandby: Table<WindowTemplate> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplate_CommStandby).cast());
static sWindowTemplate_DroppedOut: Table<WindowTemplate> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplate_DroppedOut).cast());
static sWindowTemplate_Prize: Table<WindowTemplate> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplate_Prize).cast());
static sWindowTemplates_PlayAgain: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplates_PlayAgain).cast());
static sWindowTemplates_Records: Table<WindowTemplate> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplates_Records).cast());
static sWindowTemplates_Results: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::dodrio_berry_picking::sWindowTemplates_Results).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sGame: crate::global::Global<*mut DodrioGame> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sDodrioSpriteIds: crate::global::Global<CArray<*mut u16, 5>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sCloudSpriteIds: crate::global::Global<CArray<*mut u16, 2>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBerrySpriteIds: crate::global::Global<CArray<*mut u16, 11>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBerryIconSpriteIds: crate::global::Global<CArray<*mut u16, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sStatusBar: crate::global::Global<*mut StatusBar> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sGfx: crate::global::Global<*mut DodrioGame_Gfx> =
    crate::global::Global::new(null_mut());
pub(crate) static sExitingGame: crate::global::Global<u32> = crate::global::Global::new(0);

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
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `GetWindowFrameTilesPal` with this module's view of its types.
#[inline]
unsafe fn GetWindowFrameTilesPal(a0: u8) -> *mut TilesPal {
    unsafe { crate::text_window::GetWindowFrameTilesPal(a0) as *mut TilesPal }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn StartDodrioBerryPicking(partyId: u16, exitCallback: Option<unsafe fn()>) {
    sExitingGame.set(FALSE as u32);
    if gReceivedRemoteLinkPlayers.get() != 0
        && !({
            crate::gl!(sGame = AllocZeroed(13104) as *mut DodrioGame);
            crate::gl!(sGame)
        })
        .is_null()
    {
        ResetTasksAndSprites();
        InitDodrioGame(crate::gl!(sGame));
        (*crate::gl!(sGame)).exitCallback = exitCallback;
        (*crate::gl!(sGame)).multiplayerId = GetMultiplayerId();
        (*crate::gl!(sGame)).player =
            (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId];
        InitMonInfo(
            &raw mut (*crate::gl!(sGame)).monInfo[(*crate::gl!(sGame)).multiplayerId],
            crate::gl!(&raw gPlayerParty[partyId]),
        );
        CreateTask(Some(Task_StartDodrioGame), 1);
        SetMainCallback2(Some(CB2_DodrioGame));
        SetRandomPrize();
        GetActiveBerryColumns(
            (*crate::gl!(sGame)).numPlayers,
            &raw mut (*crate::gl!(sGame)).berryColStart,
            &raw mut (*crate::gl!(sGame)).berryColEnd,
        );
        StopMapMusic();
        PlayNewMapMusic(MUS_RG_BERRY_PICK);
    } else {
        SetMainCallback2(exitCallback);
    }
}
unsafe fn ResetTasksAndSprites() {
    ResetTasks();
    ResetSpriteData();
    FreeAllSpritePalettes();
}
unsafe fn InitDodrioGame(game: *mut DodrioGame) {
    (*game).startState = 0;
    (*game).state = 0;
    (*game).timer = 0;
    (*game).funcId = FUNC_INTRO;
    (*game).prevFuncId = FUNC_INTRO;
    (*game).startGame = FALSE as u32;
    (*game).berriesFalling = FALSE as u32;
    (*game).countdownEndDelay = 0;
    (*game).numGraySquares = 0;
    (*game).unused2 = 0;
    (*game).allReadyToEnd = FALSE as u32;
    for i in 0..4u8 {
        (*game).pickStateQueue[i] = PICK_NONE;
    }
    let mut i: u8 = 0;
    while i < MAX_RFU_PLAYERS as u8 {
        (*game).inputState[i] = INPUTSTATE_NONE;
        (*game).inputDelay[i] = 0;
        (*game).berryResults[i][0] = 0;
        (*game).berryResults[i][1] = 0;
        (*game).berryResults[i][2] = 0;
        (*game).berryResults[i][3] = 0;
        (*game).berryResults[i][5] = 0;
        (*game).playAgainStates[i] = PLAY_AGAIN_NONE;
        (*game).readyToEnd[i] = FALSE as u32;
        i += 1;
    }
    for i in 0..NUM_BERRY_COLUMNS {
        (*game).fallTimer[i] = 0;
        (*game).newBerryTimer[i] = 0;
        (*game).berryState[i] = BERRYSTATE_NONE;
        (*game).playersAttemptingPick[i][0] = PLAYER_NONE;
        (*game).playersAttemptingPick[i][1] = PLAYER_NONE;
    }
    (*game).isLeader = (if GetMultiplayerId() == 0 {
        TRUE as i32
    } else {
        0
    }) as u8;
    (*game).numPlayers = GetLinkPlayerCount();
    (*game).posToPlayerId[0] = GetMultiplayerId();
    i = 1;
    while i < (*game).numPlayers {
        (*game).posToPlayerId[i] = (*game).posToPlayerId[i as i32 - 1] + 1;
        if (*game).posToPlayerId[i] as i32 > (*game).numPlayers as i32 - 1 {
            (*game).posToPlayerId[i] =
                rem_i32((*game).posToPlayerId[i] as i32, (*game).numPlayers as i32) as u8;
        }
        i += 1;
    }
}
pub(crate) unsafe fn Task_StartDodrioGame(taskId: u8) {
    let mut numPlayers: u8 = 0;
    match (*crate::gl!(sGame)).startState {
        0 => {
            SetVBlankCallback(None);
            CreateTask_(Some(Task_CommunicateMonInfo), 4);
            (*crate::gl!(sGame)).startState += 1;
        }
        1 => {
            if FuncIsActiveTask(Some(Task_CommunicateMonInfo)) == 0 {
                InitGameGfx(&raw mut (*crate::gl!(sGame)).gfx);
                (*crate::gl!(sGame)).startState += 1;
            }
        }
        2 => {
            if IsGfxFuncActive() == 0 {
                Rfu_SetLinkStandbyCallback();
                (*crate::gl!(sGame)).startState += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                if gReceivedRemoteLinkPlayers.get() != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                }
                (*crate::gl!(sGame)).startState += 1;
            }
        }
        4 => {
            numPlayers = (*crate::gl!(sGame)).numPlayers;
            LoadDodrioGfx();
            for i in 0..numPlayers {
                CreateDodrioSprite(
                    &raw mut (*crate::gl!(sGame)).monInfo[(*crate::gl!(sGame)).posToPlayerId[i]],
                    i,
                    (*crate::gl!(sGame)).posToPlayerId[i],
                    (*crate::gl!(sGame)).numPlayers,
                );
            }
            SetAllDodrioInvisibility(FALSE, (*crate::gl!(sGame)).numPlayers);
            (*crate::gl!(sGame)).startState += 1;
        }
        5 => {
            LoadBerryGfx();
            CreateBerrySprites();
            CreateCloudSprites();
            CreateStatusBarSprites();
            (*crate::gl!(sGame)).startState += 1;
        }
        6 => {
            BlendPalettes(PALETTES_ALL, 0x10, 0x00);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB_DodrioGame));
            (*crate::gl!(sGame)).startState += 1;
        }
        7 => {
            UpdatePaletteFade();
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sGame)).startState += 1;
            }
        }
        _ => {
            DestroyTask(taskId);
            CreateDodrioGameTask(Some(Task_NewGameIntro));
        }
    }
}
pub(crate) unsafe fn Task_DodrioGame_Leader(taskId: u8) {
    RecvLinkData_Leader();
    sLeaderFuncs[(*crate::gl!(sGame)).funcId].unwrap_unchecked()();
    if sExitingGame.get() == 0 {
        UpdateGame_Leader();
    }
    SendLinkData_Leader();
}
pub(crate) unsafe fn Task_DodrioGame_Member(taskId: u8) {
    RecvLinkData_Member();
    sMemberFuncs[(*crate::gl!(sGame)).funcId].unwrap_unchecked()();
    if sExitingGame.get() == 0 {
        UpdateGame_Member();
    }
    SendLinkData_Member();
}
pub(crate) unsafe fn DoGameIntro() {
    match (*crate::gl!(sGame)).state {
        0 => {
            StartDodrioIntroAnim(1);
            SetGfxFuncById(GFXFUNC_SHOW_NAMES);
            (*crate::gl!(sGame)).state += 1;
        }
        1 if IsGfxFuncActive() == 0 => {
            SetGameFunc(FUNC_INIT_COUNTDOWN);
        }
        _ => {}
    }
}
pub(crate) unsafe fn InitCountdown() {
    match (*crate::gl!(sGame)).state {
        0 => {
            InitFirstWaveOfBerries();
            (*crate::gl!(sGame)).state += 1;
        }
        _ => {
            (*crate::gl!(sGame)).startCountdown = TRUE as u32;
            SetGameFunc(FUNC_COUNTDOWN);
        }
    }
}
pub(crate) unsafe fn DoCountdown() {
    match (*crate::gl!(sGame)).state {
        0 => {
            StartMinigameCountdown(GFXTAG_COUNTDOWN, PALTAG_COUNTDOWN, 120, 80, 0);
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            Rfu_SetLinkStandbyCallback();
            (*crate::gl!(sGame)).state += 1;
        }
        2 => {
            if IsLinkTaskFinished() != 0 {
                (*crate::gl!(sGame)).state += 1;
                (*crate::gl!(sGame)).countdownEndDelay = 0;
            }
        }
        3 => {
            if IsMinigameCountdownRunning() == 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        4 => {
            if ({
                (*crate::gl!(sGame)).countdownEndDelay += 1;
                (*crate::gl!(sGame)).countdownEndDelay
            }) > 5
            {
                Rfu_SetLinkStandbyCallback();
                (*crate::gl!(sGame)).state += 1;
            }
        }
        5 if IsLinkTaskFinished() != 0 => {
            SetGameFunc(FUNC_WAIT_START);
        }
        _ => {}
    }
}
pub(crate) unsafe fn WaitGameStart() {
    if (*crate::gl!(sGame)).state == 0 && (*crate::gl!(sGame)).startGame != 0 {
        SetGameFunc(FUNC_PLAY_GAME);
    }
}
pub(crate) unsafe fn PlayGame_Leader() {
    if (*crate::gl!(sGame)).state == 0 {
        if (*crate::gl!(sGame)).numGraySquares < NUM_STATUS_SQUARES {
            if (*crate::gl!(sGame)).inputState[0] == INPUTSTATE_NONE {
                if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
                    if (*crate::gl!(sGame)).players[0].comm.pickState == PICK_NONE {
                        (*crate::gl!(sGame)).players[0].comm.ateBerry = 0;
                        (*crate::gl!(sGame)).players[0].comm.pickState =
                            UpdatePickStateQueue(PICK_MIDDLE);
                    }
                } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
                    if (*crate::gl!(sGame)).players[0].comm.pickState == PICK_NONE {
                        (*crate::gl!(sGame)).players[0].comm.ateBerry = 0;
                        (*crate::gl!(sGame)).players[0].comm.pickState =
                            UpdatePickStateQueue(PICK_RIGHT);
                    }
                } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
                    if (*crate::gl!(sGame)).players[0].comm.pickState == PICK_NONE {
                        (*crate::gl!(sGame)).players[0].comm.ateBerry = 0;
                        (*crate::gl!(sGame)).players[0].comm.pickState =
                            UpdatePickStateQueue(PICK_LEFT);
                    }
                } else {
                    (*crate::gl!(sGame)).players[0].comm.pickState =
                        UpdatePickStateQueue(PICK_NONE);
                }
            }
        } else {
            SetGameFunc(FUNC_WAIT_END_GAME);
        }
        UpdateFallingBerries();
        HandleSound_Leader();
    }
}
pub(crate) unsafe fn PlayGame_Member() {
    if (*crate::gl!(sGame)).numGraySquares < NUM_STATUS_SQUARES {
        if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
            if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState
                == PICK_NONE
            {
                (*crate::gl!(sGame)).player.comm.pickState = PICK_MIDDLE;
            }
        } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
            if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState
                == PICK_NONE
            {
                (*crate::gl!(sGame)).player.comm.pickState = PICK_RIGHT;
            }
        } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
            if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState
                == PICK_NONE
            {
                (*crate::gl!(sGame)).player.comm.pickState = PICK_LEFT;
            }
        } else {
            (*crate::gl!(sGame)).player.comm.pickState = PICK_NONE;
        }
    } else {
        SetGameFunc(FUNC_WAIT_END_GAME);
    }
    HandleSound_Member();
}
pub(crate) unsafe fn WaitEndGame_Leader() {
    let mut i: u8 = 0;
    UpdateFallingBerries();
    HandleSound_Leader();
    if ReadyToEndGame_Leader() == TRUE as u32 {
        SetMaxBerriesPickedInRow();
        SetGameFunc(FUNC_INIT_RESULTS);
    } else {
        (*crate::gl!(sGame)).allReadyToEnd = TRUE as u32;
        i = 1;
        while i < (*crate::gl!(sGame)).numPlayers {
            if (*crate::gl!(sGame)).readyToEnd[i] != TRUE as u32 {
                (*crate::gl!(sGame)).allReadyToEnd = FALSE as u32;
                break;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe fn WaitEndGame_Member() {
    HandleSound_Member();
    if ReadyToEndGame_Member() == TRUE as u32 {
        SetGameFunc(FUNC_INIT_RESULTS);
    }
}
unsafe fn AllLinkBlocksReceived() -> u32 {
    let recvStatus: u8 = GetBlockReceivedStatus();
    let playerFlags: u8 = GetLinkPlayerCountAsBitFlags();
    if recvStatus == playerFlags {
        ResetBlockReceivedFlags();
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn InitResults_Leader() {
    match (*crate::gl!(sGame)).state {
        0 => {
            if SendBlock(
                0,
                (*crate::gl!(sGame)).berryResults.as_mut_ptr() as *mut c_void,
                60,
            ) != 0
            {
                (*crate::gl!(sGame)).playersReceived = 0;
                (*crate::gl!(sGame)).state += 1;
            }
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            if AllLinkBlocksReceived() != 0 {
                (*crate::gl!(sGame)).playersReceived = (*crate::gl!(sGame)).numPlayers;
            }
            if (*crate::gl!(sGame)).playersReceived >= (*crate::gl!(sGame)).numPlayers {
                (*crate::gl!(sGame)).timer += 1;
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            if WaitFanfare(TRUE) != 0 {
                SetGameFunc(FUNC_RESULTS);
                FadeOutAndPlayNewMapMusic(MUS_RG_VICTORY_WILD, 4);
            }
        }
    }
}
pub(crate) unsafe fn InitResults_Member() {
    let mut i: u8 = 0;
    match (*crate::gl!(sGame)).state {
        0 => {
            if SendBlock(
                0,
                (*crate::gl!(sGame)).berryResults[(*crate::gl!(sGame)).timer].as_mut_ptr()
                    as *mut c_void,
                60,
            ) != 0
            {
                (*crate::gl!(sGame)).playersReceived = 0;
                (*crate::gl!(sGame)).state += 1;
            }
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            if AllLinkBlocksReceived() != 0 {
                i = 0;
                while i < (*crate::gl!(sGame)).numPlayers {
                    memcpy(
                        (*crate::gl!(sGame)).berryResults.as_mut_ptr() as *mut u8,
                        (*gBlockRecvBuffer.as_ptr()).as_mut_ptr() as *mut u8,
                        60,
                    );
                    (*crate::gl!(sGame)).playersReceived = (*crate::gl!(sGame)).numPlayers;
                    i += 1;
                }
            }
            if (*crate::gl!(sGame)).playersReceived >= (*crate::gl!(sGame)).numPlayers {
                (*crate::gl!(sGame)).timer += 1;
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            if WaitFanfare(TRUE) != 0 {
                (*crate::gl!(sGame)).maxBerriesPickedInRow =
                    (*crate::gl!(sGame)).berryResults[(*crate::gl!(sGame)).multiplayerId][5];
                SetGameFunc(FUNC_RESULTS);
                FadeOutAndPlayNewMapMusic(MUS_RG_VICTORY_WILD, 4);
            }
        }
    }
}
pub(crate) unsafe fn DoResults() {
    let mut playAgainState: u8 = PLAY_AGAIN_YES;
    let mut i: u8 = 0;
    match (*crate::gl!(sGame)).state {
        0 => {
            TryUpdateRecords();
            SetStatusBarInvisibility(TRUE);
            ResetCloudPos();
            SetCloudInvisibility(TRUE);
            SetGfxFuncById(GFXFUNC_SHOW_RESULTS);
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            if IsGfxFuncActive() == 0 {
                SetGfxFuncById(GFXFUNC_MSG_COMM_STANDBY);
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            playAgainState = GetPlayAgainState();
            if SendBlock(0, &raw mut playAgainState as *mut c_void, 1) != 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                (*crate::gl!(sGame)).state += 1;
                (*crate::gl!(sGame)).playersReceived = 0;
            }
        }
        4 => {
            if AllLinkBlocksReceived() != 0 {
                i = 0;
                while i < (*crate::gl!(sGame)).numPlayers {
                    (*crate::gl!(sGame)).playAgainStates[i] =
                        *((*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8);
                    (*crate::gl!(sGame)).playersReceived = (*crate::gl!(sGame)).numPlayers;
                    i += 1;
                }
            }
            if (*crate::gl!(sGame)).playersReceived >= (*crate::gl!(sGame)).numPlayers
                && ({
                    (*crate::gl!(sGame)).timer += 1;
                    (*crate::gl!(sGame)).timer
                }) >= 120
            {
                SetGfxFuncById(GFXFUNC_ERASE_MSG);
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            if IsGfxFuncActive() == 0 {
                SetGameFunc(FUNC_ASK_PLAY_AGAIN);
            }
        }
    }
}
pub(crate) unsafe fn AskPlayAgain() {
    let mut playAgainState: u8 = 0;
    let mut i: u8 = 0;
    match (*crate::gl!(sGame)).state {
        0 => {
            if GetHighestScore() >= PRIZE_SCORE {
                SetGfxFuncById(GFXFUNC_MSG_SAVING);
            }
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            if IsGfxFuncActive() == 0 {
                SetGfxFuncById(GFXFUNC_MSG_PLAY_AGAIN);
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            ResetBerryAndStatusBarSprites();
            ResetForPlayAgainPrompt();
            (*crate::gl!(sGame)).state += 1;
        }
        3 => {
            if ({
                playAgainState = GetPlayAgainState();
                playAgainState
            }) != PLAY_AGAIN_NONE
            {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        4 => {
            if IsGfxFuncActive() == 0 {
                SetGfxFuncById(GFXFUNC_MSG_COMM_STANDBY);
                (*crate::gl!(sGame)).state += 1;
            }
        }
        5 => {
            playAgainState = GetPlayAgainState();
            if SendBlock(0, &raw mut playAgainState as *mut c_void, 1) != 0 {
                (*crate::gl!(sGame)).playersReceived = 0;
                (*crate::gl!(sGame)).state += 1;
            }
        }
        6 => {
            if IsLinkTaskFinished() != 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        7 => {
            if AllLinkBlocksReceived() != 0 {
                i = 0;
                while i < (*crate::gl!(sGame)).numPlayers {
                    (*crate::gl!(sGame)).playAgainStates[i] =
                        *((*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8);
                    (*crate::gl!(sGame)).playersReceived = (*crate::gl!(sGame)).numPlayers;
                    i += 1;
                }
            }
            if (*crate::gl!(sGame)).playersReceived >= (*crate::gl!(sGame)).numPlayers {
                if ({
                    (*crate::gl!(sGame)).timer += 1;
                    (*crate::gl!(sGame)).timer
                }) >= 120
                {
                    ResetPickState();
                    SetGfxFuncById(GFXFUNC_ERASE_MSG);
                    (*crate::gl!(sGame)).state += 1;
                }
            } else {
                HandleWaitPlayAgainInput();
            }
        }
        _ => {
            if IsGfxFuncActive() == 0 {
                i = 0;
                while i < (*crate::gl!(sGame)).numPlayers {
                    if (*crate::gl!(sGame)).playAgainStates[i] == PLAY_AGAIN_NO {
                        SetGameFunc(FUNC_END_LINK);
                        return;
                    }
                    i += 1;
                }
                SetGameFunc(FUNC_RESET_GAME);
            }
        }
    }
}
pub(crate) unsafe fn EndLink() {
    match (*crate::gl!(sGame)).state {
        0 => {
            SetCloseLinkCallback();
            SetGfxFuncById(GFXFUNC_MSG_PLAYER_DROPPED);
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            if IsGfxFuncActive() == 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            if GetPlayAgainState() == PLAY_AGAIN_DROPPED {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            if gReceivedRemoteLinkPlayers.get() == 0 {
                SetGameFunc(FUNC_EXIT);
            }
        }
    }
}
pub(crate) unsafe fn ExitGame() {
    match (*crate::gl!(sGame)).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            UpdatePaletteFade();
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            FreeBerrySprites();
            FreeStatusBar();
            FreeDodrioSprites((*crate::gl!(sGame)).numPlayers);
            FreeCloudSprites();
            sExitingGame.set(TRUE as u32);
            SetGfxFuncById(GFXFUNC_STOP);
            (*crate::gl!(sGame)).state += 1;
        }
        _ => {
            if IsGfxFuncActive() == 0 {
                SetMainCallback2((*crate::gl!(sGame)).exitCallback);
                DestroyTask((*crate::gl!(sGame)).taskId);
                Free(crate::gl!(sGame) as *mut c_void);
                FreeAllWindowBuffers();
            }
        }
    }
}
pub(crate) unsafe fn ResetGame() {
    match (*crate::gl!(sGame)).state {
        0 => {
            SetGfxFuncById(GFXFUNC_IDLE);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sGame)).state += 1;
        }
        1 => {
            UpdatePaletteFade();
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        2 => {
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            (*crate::gl!(sGame)).state += 1;
        }
        3 => {
            StopMapMusic();
            (*crate::gl!(sGame)).state += 1;
        }
        4 => {
            PlayNewMapMusic(MUS_RG_BERRY_PICK);
            StartCloudMovement();
            (*crate::gl!(sGame)).state += 1;
        }
        5 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*crate::gl!(sGame)).state += 1;
        }
        6 => {
            UpdatePaletteFade();
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            DestroyTask((*crate::gl!(sGame)).taskId);
            CreateDodrioGameTask(Some(Task_NewGameIntro));
            ResetGfxState();
            InitDodrioGame(crate::gl!(sGame));
            if gReceivedRemoteLinkPlayers.get() == 0 {
                (*crate::gl!(sGame)).numPlayers = 1;
            }
            SetRandomPrize();
            SetCloudInvisibility(FALSE);
        }
    }
}
pub(crate) unsafe fn Task_NewGameIntro(taskId: u8) {
    match (*crate::gl!(sGame)).state {
        0 => {
            if SlideTreeBordersOut() == TRUE as u32 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        1 => {
            InitStatusBarPos();
            (*crate::gl!(sGame)).state += 1;
        }
        2 => {
            if DoStatusBarIntro() == TRUE as u32 {
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {
            if (*crate::gl!(sGame)).isLeader != 0 {
                CreateDodrioGameTask(Some(Task_DodrioGame_Leader));
            } else {
                CreateDodrioGameTask(Some(Task_DodrioGame_Member));
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe fn Task_CommunicateMonInfo(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut i: u8 = 0;
    match *data {
        0 => {
            if SendBlock(
                0,
                &raw mut (*crate::gl!(sGame)).monInfo[(*crate::gl!(sGame)).multiplayerId].isShiny
                    as *mut c_void,
                1,
            ) != 0
            {
                (*crate::gl!(sGame)).playersReceived = 0;
                *data += 1;
            }
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                *data += 1;
            }
        }
        2 => {
            if AllLinkBlocksReceived() != 0 {
                i = 0;
                while i < (*crate::gl!(sGame)).numPlayers {
                    *(&raw mut (*crate::gl!(sGame)).monInfo[i] as *mut u8) =
                        *((*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8);
                    (*crate::gl!(sGame)).playersReceived = (*crate::gl!(sGame)).numPlayers;
                    i += 1;
                }
            }
            if (*crate::gl!(sGame)).playersReceived >= (*crate::gl!(sGame)).numPlayers {
                DestroyTask(taskId);
                SetGfxFuncById(GFXFUNC_ERASE_MSG);
                (*crate::gl!(sGame)).state += 1;
            }
        }
        _ => {}
    }
}
unsafe fn RecvLinkData_Gameplay() {
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    (*crate::gl!(sGame)).players[0].receivedGameStatePacket = RecvPacket_GameState(
        0,
        &raw mut (*crate::gl!(sGame)).players[0],
        &raw mut (*crate::gl!(sGame)).players[0].comm,
        &raw mut (*crate::gl!(sGame)).players[1].comm,
        &raw mut (*crate::gl!(sGame)).players[2].comm,
        &raw mut (*crate::gl!(sGame)).players[3].comm,
        &raw mut (*crate::gl!(sGame)).players[4].comm,
        &raw mut (*crate::gl!(sGame)).numGraySquares,
        &raw mut (*crate::gl!(sGame)).berriesFalling,
        &raw mut (*crate::gl!(sGame)).allReadyToEnd,
    );
    (*crate::gl!(sGame)).clearRecvCmds = TRUE;
    let mut i: u8 = 1;
    while i < numPlayers {
        if (*crate::gl!(sGame)).inputState[i] == INPUTSTATE_NONE
            && RecvPacket_PickState(
                i as u32,
                &raw mut (*crate::gl!(sGame)).players[i].comm.pickState,
            ) == 0
        {
            (*crate::gl!(sGame)).players[i].comm.pickState = PICK_NONE;
            (*crate::gl!(sGame)).clearRecvCmds = FALSE;
        }
        i += 1;
    }
    if ({
        (*crate::gl!(sGame)).clearRecvCmdTimer += 1;
        (*crate::gl!(sGame)).clearRecvCmdTimer
    }) >= 60
    {
        if (*crate::gl!(sGame)).clearRecvCmds != 0 {
            ClearRecvCommands();
            (*crate::gl!(sGame)).clearRecvCmdTimer = 0;
        } else if (*crate::gl!(sGame)).clearRecvCmdTimer > 70 {
            ClearRecvCommands();
            (*crate::gl!(sGame)).clearRecvCmdTimer = 0;
        }
    }
    for i in 0..numPlayers {
        if (*crate::gl!(sGame)).players[i].comm.pickState != PICK_NONE
            && (*crate::gl!(sGame)).inputState[i] == INPUTSTATE_NONE
        {
            (*crate::gl!(sGame)).inputState[i] = INPUTSTATE_TRY_PICK;
        }
        match (*crate::gl!(sGame)).inputState[i] {
            INPUTSTATE_TRY_PICK | INPUTSTATE_PICKED | INPUTSTATE_ATE_BERRY => {
                if ({
                    (*crate::gl!(sGame)).inputDelay[i] += 1;
                    (*crate::gl!(sGame)).inputDelay[i]
                }) >= 6
                {
                    (*crate::gl!(sGame)).inputDelay[i] = 0;
                    (*crate::gl!(sGame)).inputState[i] = INPUTSTATE_NONE;
                    (*crate::gl!(sGame)).players[i].comm.pickState = PICK_NONE;
                    (*crate::gl!(sGame)).players[i].comm.ateBerry = FALSE;
                    (*crate::gl!(sGame)).players[i].comm.missedBerry = FALSE;
                }
            }
            INPUTSTATE_BAD_MISS
                if ({
                    (*crate::gl!(sGame)).inputDelay[i] += 1;
                    (*crate::gl!(sGame)).inputDelay[i]
                }) >= 40 =>
            {
                (*crate::gl!(sGame)).inputDelay[i] = 0;
                (*crate::gl!(sGame)).inputState[i] = INPUTSTATE_NONE;
                (*crate::gl!(sGame)).players[i].comm.pickState = PICK_NONE;
                (*crate::gl!(sGame)).players[i].comm.ateBerry = FALSE;
                (*crate::gl!(sGame)).players[i].comm.missedBerry = FALSE;
            }
            _ => {}
        }
    }
}
unsafe fn RecvLinkData_ReadyToEnd() {
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    (*crate::gl!(sGame)).players[0].receivedGameStatePacket = RecvPacket_GameState(
        0,
        &raw mut (*crate::gl!(sGame)).players[0],
        &raw mut (*crate::gl!(sGame)).players[0].comm,
        &raw mut (*crate::gl!(sGame)).players[1].comm,
        &raw mut (*crate::gl!(sGame)).players[2].comm,
        &raw mut (*crate::gl!(sGame)).players[3].comm,
        &raw mut (*crate::gl!(sGame)).players[4].comm,
        &raw mut (*crate::gl!(sGame)).numGraySquares,
        &raw mut (*crate::gl!(sGame)).berriesFalling,
        &raw mut (*crate::gl!(sGame)).allReadyToEnd,
    );
    (*crate::gl!(sGame)).clearRecvCmds = TRUE;
    for i in 1..numPlayers {
        if RecvPacket_ReadyToEnd(i as u32) != 0 {
            (*crate::gl!(sGame)).readyToEnd[i] = TRUE as u32;
            (*crate::gl!(sGame)).clearRecvCmds = FALSE;
        }
    }
    if ({
        (*crate::gl!(sGame)).clearRecvCmdTimer += 1;
        (*crate::gl!(sGame)).clearRecvCmdTimer
    }) >= 60
    {
        if (*crate::gl!(sGame)).clearRecvCmds != 0 {
            ClearRecvCommands();
            (*crate::gl!(sGame)).clearRecvCmdTimer = 0;
        } else if (*crate::gl!(sGame)).clearRecvCmdTimer > 70 {
            ClearRecvCommands();
            (*crate::gl!(sGame)).clearRecvCmdTimer = 0;
        }
    }
}
pub(crate) unsafe fn RecvLinkData_Leader() {
    match (*crate::gl!(sGame)).funcId {
        FUNC_WAIT_START => {
            if AllPlayersReadyToStart() == TRUE as u32 {
                ResetReadyToStart();
                (*crate::gl!(sGame)).startGame = TRUE as u32;
            }
        }
        FUNC_PLAY_GAME => {
            RecvLinkData_Gameplay();
        }
        FUNC_WAIT_END_GAME => {
            RecvLinkData_ReadyToEnd();
        }
        _ => {}
    }
}
pub(crate) unsafe fn SendLinkData_Leader() {
    match (*crate::gl!(sGame)).funcId {
        FUNC_PLAY_GAME => {
            SendPacket_GameState(
                &raw mut (*crate::gl!(sGame)).player,
                &raw mut (*crate::gl!(sGame)).players[0].comm,
                &raw mut (*crate::gl!(sGame)).players[1].comm,
                &raw mut (*crate::gl!(sGame)).players[2].comm,
                &raw mut (*crate::gl!(sGame)).players[3].comm,
                &raw mut (*crate::gl!(sGame)).players[4].comm,
                (*crate::gl!(sGame)).numGraySquares,
                (*crate::gl!(sGame)).berriesFalling,
                (*crate::gl!(sGame)).allReadyToEnd,
            );
        }
        FUNC_WAIT_END_GAME => {
            SendPacket_GameState(
                &raw mut (*crate::gl!(sGame)).player,
                &raw mut (*crate::gl!(sGame)).players[0].comm,
                &raw mut (*crate::gl!(sGame)).players[1].comm,
                &raw mut (*crate::gl!(sGame)).players[2].comm,
                &raw mut (*crate::gl!(sGame)).players[3].comm,
                &raw mut (*crate::gl!(sGame)).players[4].comm,
                (*crate::gl!(sGame)).numGraySquares,
                (*crate::gl!(sGame)).berriesFalling,
                (*crate::gl!(sGame)).allReadyToEnd,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn RecvLinkData_Member() {
    match (*crate::gl!(sGame)).funcId {
        FUNC_PLAY_GAME => {
            RecvPacket_GameState(
                (*crate::gl!(sGame)).multiplayerId as u32,
                &raw mut (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId],
                &raw mut (*crate::gl!(sGame)).players[0].comm,
                &raw mut (*crate::gl!(sGame)).players[1].comm,
                &raw mut (*crate::gl!(sGame)).players[2].comm,
                &raw mut (*crate::gl!(sGame)).players[3].comm,
                &raw mut (*crate::gl!(sGame)).players[4].comm,
                &raw mut (*crate::gl!(sGame)).numGraySquares,
                &raw mut (*crate::gl!(sGame)).berriesFalling,
                &raw mut (*crate::gl!(sGame)).allReadyToEnd,
            );
        }
        FUNC_WAIT_END_GAME => {
            RecvPacket_GameState(
                (*crate::gl!(sGame)).multiplayerId as u32,
                &raw mut (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId],
                &raw mut (*crate::gl!(sGame)).players[0].comm,
                &raw mut (*crate::gl!(sGame)).players[1].comm,
                &raw mut (*crate::gl!(sGame)).players[2].comm,
                &raw mut (*crate::gl!(sGame)).players[3].comm,
                &raw mut (*crate::gl!(sGame)).players[4].comm,
                &raw mut (*crate::gl!(sGame)).numGraySquares,
                &raw mut (*crate::gl!(sGame)).berriesFalling,
                &raw mut (*crate::gl!(sGame)).allReadyToEnd,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn SendLinkData_Member() {
    match (*crate::gl!(sGame)).funcId {
        FUNC_WAIT_START => {
            SendPacket_ReadyToStart(TRUE as u32);
            (*crate::gl!(sGame)).startGame = TRUE as u32;
        }
        FUNC_PLAY_GAME => {
            if (*crate::gl!(sGame)).player.comm.pickState != PICK_NONE {
                SendPacket_PickState((*crate::gl!(sGame)).player.comm.pickState);
            }
        }
        FUNC_WAIT_END_GAME
            if (*crate::gl!(sGame)).berriesFalling == 0
                && (*crate::gl!(sGame)).allReadyToEnd == 0 =>
        {
            SendPacket_ReadyToEnd(TRUE as u32);
        }
        _ => {}
    }
}
unsafe fn HandleSound_Leader() {
    if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .pickState
        == PICK_NONE
    {
        if IsSEPlaying() == 0 {
            (*crate::gl!(sGame)).playingPickSound = FALSE;
        }
    } else if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .ateBerry
        == TRUE
    {
        if (*crate::gl!(sGame)).playingPickSound == 0 {
            m4aSongNumStop(SE_SUCCESS);
            PlaySE(SE_SUCCESS);
            (*crate::gl!(sGame)).playingPickSound = TRUE;
        }
    } else if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .missedBerry
        == TRUE
        && (*crate::gl!(sGame)).playingPickSound == 0
        && IsSEPlaying() == 0
    {
        PlaySE(SE_BOO);
        StartDodrioMissedAnim(1);
        (*crate::gl!(sGame)).playingPickSound = TRUE;
    }
    if (*crate::gl!(sGame)).endSoundState == 0
        && (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES
    {
        StopMapMusic();
        (*crate::gl!(sGame)).endSoundState = 1;
    } else if (*crate::gl!(sGame)).endSoundState == 1 {
        PlayFanfareByFanfareNum(FANFARE_TOO_BAD);
        (*crate::gl!(sGame)).endSoundState = 2;
    }
}
unsafe fn HandleSound_Member() {
    let berryStart: u8 = (*crate::gl!(sGame)).berryColStart;
    let berryEnd: u8 = (*crate::gl!(sGame)).berryColEnd;
    if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .pickState
        == PICK_NONE
    {
        if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
            .comm
            .ateBerry
            != TRUE
            && (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .missedBerry
                != TRUE
        {
            (*crate::gl!(sGame)).playingPickSound = 0;
        }
    } else if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .ateBerry
        == TRUE
    {
        if (*crate::gl!(sGame)).playingPickSound == 0 {
            m4aSongNumStop(SE_SUCCESS);
            PlaySE(SE_SUCCESS);
            (*crate::gl!(sGame)).playingPickSound = TRUE;
        }
    } else if (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .missedBerry
        == TRUE
        && (*crate::gl!(sGame)).playingPickSound == 0
        && IsSEPlaying() == 0
    {
        PlaySE(SE_BOO);
        StartDodrioMissedAnim(1);
        (*crate::gl!(sGame)).playingPickSound = TRUE;
    }
    for i in berryStart..berryEnd {
        let berries: *mut DodrioGame_Berries =
            &raw mut (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId].berries;
        if (*berries).fallDist[i] >= MAX_FALL_DIST {
            if (*crate::gl!(sGame)).playingSquishSound[i] == 0 {
                PlaySE(SE_BALLOON_RED + (*berries).ids[i] as u16);
                (*crate::gl!(sGame)).playingSquishSound[i] = TRUE;
            }
        } else {
            (*crate::gl!(sGame)).playingSquishSound[i] = FALSE;
        }
    }
    if (*crate::gl!(sGame)).endSoundState == 0
        && (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES
    {
        StopMapMusic();
        (*crate::gl!(sGame)).endSoundState = 1;
    } else if (*crate::gl!(sGame)).endSoundState == 1 {
        PlayFanfareByFanfareNum(FANFARE_TOO_BAD);
        (*crate::gl!(sGame)).endSoundState = 2;
    }
}
pub(crate) unsafe fn CB2_DodrioGame() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_DodrioGame() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
unsafe fn InitMonInfo(monInfo: *mut DodrioGame_MonInfo, mon: *mut Pokemon) {
    (*monInfo).isShiny = IsMonShiny(mon);
}
unsafe fn CreateTask_(func: Option<unsafe fn(u8)>, priority: u8) {
    CreateTask(func, priority);
}
unsafe fn CreateDodrioGameTask(func: Option<unsafe fn(u8)>) {
    (*crate::gl!(sGame)).taskId = CreateTask(func, 1);
    (*crate::gl!(sGame)).state = 0;
    (*crate::gl!(sGame)).startState = 0;
    (*crate::gl!(sGame)).timer = 0;
}
unsafe fn SetGameFunc(funcId: u8) {
    (*crate::gl!(sGame)).prevFuncId = (*crate::gl!(sGame)).funcId;
    (*crate::gl!(sGame)).funcId = funcId;
    (*crate::gl!(sGame)).state = 0;
    (*crate::gl!(sGame)).timer = 0;
}
unsafe fn SlideTreeBordersOut() -> u32 {
    let x: u8 = ((*crate::gl!(sGame)).timer as i32 / 4) as u8;
    (*crate::gl!(sGame)).timer += 1;
    if x != 0 && (*crate::gl!(sGame)).timer as i32 % 4 == 0 {
        if x < sTreeBorderXPos[(*crate::gl!(sGame)).numPlayers as i32 - 1] {
            SetGpuReg(REG_OFFSET_BG1HOFS, x as u16 * 8);
            SetGpuReg(REG_OFFSET_BG2HOFS, (x as u16 * 8).wrapping_neg());
            return FALSE as u32;
        } else {
            return TRUE as u32;
        }
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn InitFirstWaveOfBerries() {
    let berryStart: u8 = (*crate::gl!(sGame)).berryColStart;
    let berryEnd: u8 = (*crate::gl!(sGame)).berryColEnd;
    for i in berryStart..berryEnd {
        let berries: *mut DodrioGame_Berries = &raw mut (*crate::gl!(sGame)).player.berries;
        (*berries).fallDist[i] = (if i as i32 % 2 == 0 { 1 } else { 0 }) as u8;
        (*berries).ids[i] = BERRY_BLUE;
    }
}
unsafe fn HandlePickBerries() {
    let berryStart: u8 = (*crate::gl!(sGame)).berryColStart;
    let berryEnd: u8 = (*crate::gl!(sGame)).berryColEnd;
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    let mut column: u8 = 0;
    if (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES {
        return;
    }
    let mut i: u8 = 0;
    while i < numPlayers {
        let pickState: *mut u8 = &raw mut (*crate::gl!(sGame)).players[i].comm.pickState;
        if *pickState != PICK_NONE && (*crate::gl!(sGame)).inputState[i] == INPUTSTATE_TRY_PICK {
            for j in berryStart..berryEnd {
                column = sActiveColumnMap[0][0][j];
                if (*crate::gl!(sGame)).playersAttemptingPick[column][0] == i
                    || (*crate::gl!(sGame)).playersAttemptingPick[column][1] == i
                {
                    break;
                }
                if TryPickBerry(i, *pickState, column) == TRUE as u32 {
                    for k in 0..2u8 {
                        if (*crate::gl!(sGame)).playersAttemptingPick[column][k] == PLAYER_NONE {
                            (*crate::gl!(sGame)).playersAttemptingPick[column][k] = i;
                            (*crate::gl!(sGame)).inputState[i] = INPUTSTATE_PICKED;
                            (*crate::gl!(sGame)).berryState[column] = BERRYSTATE_PICKED;
                            break;
                        }
                    }
                    break;
                }
                if (*crate::gl!(sGame)).players[i].comm.missedBerry == TRUE {
                    break;
                }
            }
        }
        i += 1;
    }
    for j in berryStart..berryEnd {
        'l4: {
            let mut playerIdMissed: u8 = PLAYER_NONE;
            column = sActiveColumnMap[0][0][j];
            if (*crate::gl!(sGame)).berryState[column] == BERRYSTATE_PICKED {
                let mut playerIdPicked: u8 = 0;
                let mut delayStage: u8 =
                    ((*crate::gl!(sGame)).difficulty[GetPlayerIdAtColumn(column)] as i32 / 7) as u8;
                if delayStage >= 2 {
                    delayStage = 2;
                }
                let delayRemaining: i32 = sBerryFallDelays[delayStage]
                    [(*crate::gl!(sGame)).players[0].berries.ids[column]]
                    as i32
                    - (*crate::gl!(sGame)).fallTimer[column] as i32;
                if delayRemaining < 6 {
                    (*crate::gl!(sGame)).eatTimer[column] += delayRemaining as u8;
                }
                if ({
                    (*crate::gl!(sGame)).eatTimer[column] += 1;
                    (*crate::gl!(sGame)).eatTimer[column]
                }) >= 6
                {
                    (*crate::gl!(sGame)).eatTimer[column] = 0;
                    if (*crate::gl!(sGame)).playersAttemptingPick[column][0] == PLAYER_NONE
                        && (*crate::gl!(sGame)).playersAttemptingPick[column][1] == PLAYER_NONE
                    {
                        break 'l4;
                    } else if (*crate::gl!(sGame)).playersAttemptingPick[column][0] != PLAYER_NONE
                        && (*crate::gl!(sGame)).playersAttemptingPick[column][1] == PLAYER_NONE
                    {
                        playerIdPicked = (*crate::gl!(sGame)).playersAttemptingPick[column][0];
                    } else {
                        let playerId1: u8 = (*crate::gl!(sGame)).playersAttemptingPick[column][0];
                        i = (*crate::gl!(sGame)).playersAttemptingPick[column][1];
                        if Random() as i32 & 1 == 0 {
                            playerIdPicked = playerId1;
                            playerIdMissed = i;
                        } else {
                            playerIdPicked = i;
                            playerIdMissed = playerId1;
                        }
                    }
                    (*crate::gl!(sGame)).player.berries.fallDist[column] = EAT_FALL_DIST;
                    (*crate::gl!(sGame)).berryState[column] = BERRYSTATE_EATEN;
                    (*crate::gl!(sGame)).inputState[playerIdPicked] = INPUTSTATE_ATE_BERRY;
                    (*crate::gl!(sGame)).berryEatenBy[column] = playerIdPicked;
                    (*crate::gl!(sGame)).players[playerIdPicked].comm.ateBerry = TRUE;
                    if playerIdMissed != PLAYER_NONE {
                        (*crate::gl!(sGame)).players[playerIdMissed]
                            .comm
                            .missedBerry = TRUE;
                    }
                    (*crate::gl!(sGame)).berriesEaten[playerIdPicked] += 1;
                    IncrementBerryResult(0, column, playerIdPicked);
                    UpdateBerriesPickedInRow(TRUE as u32);
                    TryIncrementDifficulty(playerIdPicked);
                    (*crate::gl!(sGame)).prevBerryIds[column] =
                        (*crate::gl!(sGame)).player.berries.ids[column];
                    (*crate::gl!(sGame)).player.berries.ids[column] = BERRY_MISSED;
                    (*crate::gl!(sGame)).playersAttemptingPick[column][0] = PLAYER_NONE;
                    (*crate::gl!(sGame)).playersAttemptingPick[column][1] = PLAYER_NONE;
                }
            }
        }
    }
}
unsafe fn TryPickBerry(playerId: u8, pickState: u8, column: u8) -> u32 {
    let mut pick: i32 = 0;
    let numPlayersIdx: u8 = (*crate::gl!(sGame)).numPlayers - 1;
    let berries: *mut DodrioGame_Berries = &raw mut (*crate::gl!(sGame)).player.berries;
    match pickState {
        PICK_MIDDLE => {
            pick = 1;
        }
        PICK_RIGHT => {
            pick = 2;
        }
        _ => {
            pick = 0;
        }
    }
    if (*berries).fallDist[column] == 6 || (*berries).fallDist[column] == EAT_FALL_DIST {
        if column == sDodrioHeadToColumnMap[numPlayersIdx][playerId][pick] {
            if (*crate::gl!(sGame)).berryState[column] == BERRYSTATE_PICKED
                || (*crate::gl!(sGame)).berryState[column] == BERRYSTATE_EATEN
            {
                (*crate::gl!(sGame)).players[playerId].comm.missedBerry = TRUE;
                return FALSE as u32;
            } else {
                return TRUE as u32;
            }
        }
    } else {
        if column == sDodrioHeadToColumnMap[numPlayersIdx][playerId][pick] {
            (*crate::gl!(sGame)).inputState[playerId] = INPUTSTATE_BAD_MISS;
            (*crate::gl!(sGame)).players[playerId].comm.missedBerry = TRUE;
        }
    }
    FALSE as u32
}
unsafe fn UpdateFallingBerries() {
    let berryStart: u8 = (*crate::gl!(sGame)).berryColStart;
    let berryEnd: u8 = (*crate::gl!(sGame)).berryColEnd;
    let mut delayStage: u8 = 0;
    let mut otherBerryMissed: u8 = 0;
    (*crate::gl!(sGame)).berriesFalling = FALSE as u32;
    let mut i: u8 = berryStart;
    while (i as i32) < berryEnd as i32 - 1 {
        let game: *mut DodrioGame = crate::gl!(sGame);
        if (*crate::gl!(sGame)).berryState[i] == BERRYSTATE_NONE
            || (*crate::gl!(sGame)).berryState[i] == BERRYSTATE_PICKED
        {
            (*crate::gl!(sGame)).berriesFalling = TRUE as u32;
            if (*game).player.berries.fallDist[i] >= MAX_FALL_DIST {
                (*game).player.berries.fallDist[i] = MAX_FALL_DIST;
                (*crate::gl!(sGame)).berryState[i] = BERRYSTATE_SQUISHED;
                if (*crate::gl!(sGame)).playingSquishSound[i] == 0 {
                    (*crate::gl!(sGame)).playingSquishSound[i] = TRUE;
                    PlaySE(SE_BALLOON_RED + (*game).player.berries.ids[i] as u16);
                }
                if (*crate::gl!(sGame)).numGraySquares < NUM_STATUS_SQUARES
                    || otherBerryMissed == TRUE
                {
                    otherBerryMissed = TRUE;
                    (*crate::gl!(sGame)).playingSquishSound[i] = FALSE;
                    if (*crate::gl!(sGame)).numGraySquares < NUM_STATUS_SQUARES {
                        (*crate::gl!(sGame)).numGraySquares += 1;
                    }
                    IncrementBerryResult(BERRY_MISSED, i, 0);
                    UpdateBerriesPickedInRow(FALSE as u32);
                }
            } else {
                delayStage =
                    ((*crate::gl!(sGame)).difficulty[GetPlayerIdAtColumn(i)] as i32 / 7) as u8;
                if delayStage >= 2 {
                    delayStage = 2;
                }
                let delay: u8 = sBerryFallDelays[delayStage][(*game).player.berries.ids[i]];
                if ({
                    (*crate::gl!(sGame)).fallTimer[i] += 1;
                    (*crate::gl!(sGame)).fallTimer[i]
                }) >= delay
                {
                    (*game).player.berries.fallDist[i] += 1;
                    (*crate::gl!(sGame)).fallTimer[i] = 0;
                }
                HandlePickBerries();
            }
        } else if (*crate::gl!(sGame)).berryState[i] == BERRYSTATE_EATEN {
            (*crate::gl!(sGame)).berriesFalling = TRUE as u32;
            if ({
                (*crate::gl!(sGame)).newBerryTimer[i] += 1;
                (*crate::gl!(sGame)).newBerryTimer[i]
            }) >= 20
            {
                (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).berryEatenBy[i]]
                    .comm
                    .ateBerry = FALSE;
                (*crate::gl!(sGame)).newBerryTimer[i] = 0;
                (*crate::gl!(sGame)).fallTimer[i] = 0;
                (*crate::gl!(sGame)).berryState[i] = BERRYSTATE_NONE;
                (*game).player.berries.fallDist[i] = 1;
                (*game).player.berries.ids[i] = GetNewBerryId(GetPlayerIdAtColumn(i), i);
            }
        } else if (*crate::gl!(sGame)).berryState[i] == BERRYSTATE_SQUISHED
            && ({
                (*crate::gl!(sGame)).newBerryTimer[i] += 1;
                (*crate::gl!(sGame)).newBerryTimer[i]
            }) >= 20
            && (*crate::gl!(sGame)).numGraySquares < NUM_STATUS_SQUARES
        {
            (*crate::gl!(sGame)).newBerryTimer[i] = 0;
            (*crate::gl!(sGame)).fallTimer[i] = 0;
            (*crate::gl!(sGame)).berryState[i] = BERRYSTATE_NONE;
            (*game).player.berries.fallDist[i] = 1;
            (*crate::gl!(sGame)).prevBerryIds[i] = (*game).player.berries.ids[i];
            (*game).player.berries.ids[i] = GetNewBerryId(GetPlayerIdAtColumn(i), i);
        }
        i += 1;
    }
}
unsafe fn UpdateBerrySprites() {
    let berryStart: u8 = (*crate::gl!(sGame)).berryColStart;
    let berryEnd: u8 = (*crate::gl!(sGame)).berryColEnd;
    for i in berryStart..berryEnd {
        let player: *mut DodrioGame_Player =
            &raw mut (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId];
        let column: u8 = sActiveColumnMap[(*crate::gl!(sGame)).numPlayers as i32 - 1]
            [(*crate::gl!(sGame)).multiplayerId][i];
        if (*player).berries.fallDist[column] != 0 {
            SetBerryInvisibility(i, FALSE);
        } else {
            SetBerryInvisibility(i, TRUE);
        }
        if (*player).berries.fallDist[column] >= MAX_FALL_DIST {
            SetBerryAnim(i as u16, (*player).berries.ids[column] + BERRY_MISSED);
            SetBerryYPos(i, (*player).berries.fallDist[column] * 2 - 1);
        } else if (*player).berries.ids[column] == 3 {
            (*player).berries.fallDist[column] = EAT_FALL_DIST;
            SetBerryAnim(i as u16, 6);
            SetBerryYPos(i, (*player).berries.fallDist[column] * 2 - 1);
        } else {
            SetBerryAnim(i as u16, (*player).berries.ids[column]);
            SetBerryYPos(i, (*player).berries.fallDist[column] * 2);
        }
    }
}
unsafe fn UpdateAllDodrioAnims() {
    let mut numPlayers: u8 = 0;
    numPlayers = (*crate::gl!(sGame)).numPlayers;
    for i in 0..numPlayers {
        let player: *mut DodrioGame_Player = &raw mut (*crate::gl!(sGame)).players[i];
        SetDodrioAnim(i, (*player).comm.pickState);
    }
}
unsafe fn SetAllDodrioDisabled() {
    let mut numPlayers: u8 = 0;
    numPlayers = (*crate::gl!(sGame)).numPlayers;
    for i in 0..numPlayers {
        SetDodrioAnim(i, PICK_DISABLED);
    }
}
unsafe fn UpdateGame_Leader() {
    UpdateBerrySprites();
    if (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES {
        SetAllDodrioDisabled();
    } else {
        UpdateAllDodrioAnims();
    }
    UpdateStatusBarAnim((*crate::gl!(sGame)).numGraySquares);
}
unsafe fn UpdateGame_Member() {
    UpdateBerrySprites();
    if (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES {
        SetAllDodrioDisabled();
    } else {
        UpdateAllDodrioAnims();
    }
    UpdateStatusBarAnim((*crate::gl!(sGame)).numGraySquares);
}
unsafe fn GetActiveBerryColumns(numPlayers: u8, start: *mut u8, end: *mut u8) {
    match numPlayers {
        1 => {
            *start = 4;
            *end = 7;
        }
        2 => {
            *start = 3;
            *end = 8;
        }
        3 => {
            *start = 2;
            *end = 9;
        }
        4 => {
            *start = 1;
            *end = 10;
        }
        5 => {
            *start = 0;
            *end = 11;
        }
        _ => {}
    }
}
unsafe fn AllPlayersReadyToStart() -> u32 {
    let mut numPlayers: u8 = 0;
    numPlayers = (*crate::gl!(sGame)).numPlayers;
    let mut i: u8 = 1;
    while i < numPlayers {
        if (*crate::gl!(sGame)).readyToStart[i] == FALSE {
            (*crate::gl!(sGame)).readyToStart[i] = RecvPacket_ReadyToStart(i as u32) as u8;
        }
        i += 1;
    }
    numPlayers = numPlayers;
    while i < numPlayers {
        if (*crate::gl!(sGame)).readyToStart[i] == FALSE {
            return FALSE as u32;
        }
        i += 1;
    }
    TRUE as u32
}
unsafe fn ResetReadyToStart() {
    for i in 0..(MAX_RFU_PLAYERS as u8) {
        (*crate::gl!(sGame)).readyToStart[i] = FALSE;
    }
}
unsafe fn ReadyToEndGame_Leader() -> u32 {
    if (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES
        && (*crate::gl!(sGame)).berriesFalling == 0
    {
        (*crate::gl!(sGame)).numGraySquares = NUM_STATUS_SQUARES;
        if (*crate::gl!(sGame)).allReadyToEnd != 0 {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn ReadyToEndGame_Member() -> u32 {
    let mut berryStart: u8 = 0;
    let mut berryEnd: u8 = 0;
    if (*crate::gl!(sGame)).numGraySquares >= NUM_STATUS_SQUARES {
        berryStart = (*crate::gl!(sGame)).berryColStart;
        berryEnd = (*crate::gl!(sGame)).berryColEnd;
        (*crate::gl!(sGame)).numGraySquares = NUM_STATUS_SQUARES;
        if (*crate::gl!(sGame)).allReadyToEnd != 0 {
            for i in berryStart..berryEnd {
                let player: *mut DodrioGame_Player =
                    &raw mut (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId];
                let column: u8 = sActiveColumnMap[(*crate::gl!(sGame)).numPlayers as i32 - 1]
                    [(*crate::gl!(sGame)).multiplayerId][i];
                if (*player).berries.fallDist[column] != MAX_FALL_DIST {
                    return FALSE as u32;
                }
            }
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn TryIncrementDifficulty(playerId: u8) {
    let threshold: u8 = sDifficultyThresholds[(*crate::gl!(sGame)).difficulty[playerId] as i32 % 7]
        + ((*crate::gl!(sGame)).difficulty[playerId] as i32 / 7) as u8 * 100;
    if (*crate::gl!(sGame)).berriesEaten[playerId] >= threshold as u16 {
        (*crate::gl!(sGame)).difficulty[playerId] += 1;
    }
}
unsafe fn GetPlayerIdAtColumn(column: u8) -> u8 {
    sPlayerIdAtColumn[(*crate::gl!(sGame)).numPlayers as i32 - 1][column]
}
unsafe fn GetNewBerryId(playerId: u8, column: u8) -> u8 {
    let mut highestDifficulty: u8 = 0;
    let numPlayersIdx: u8 = (*crate::gl!(sGame)).numPlayers - 1;
    let leftPlayer: u8 = sDodrioNeighborMap[numPlayersIdx][playerId][0];
    let middlePlayer: u8 = sDodrioNeighborMap[numPlayersIdx][playerId][1];
    let rightPlayer: u8 = sDodrioNeighborMap[numPlayersIdx][playerId][2];
    let mut i: u8 = 0;
    while sUnsharedColumns[numPlayersIdx][i] != 0 {
        if column == sUnsharedColumns[numPlayersIdx][i] {
            return GetNewBerryIdByDifficulty(
                (*crate::gl!(sGame)).difficulty[middlePlayer],
                column,
            );
        }
        i += 1;
    }
    if (*crate::gl!(sGame)).difficulty[leftPlayer] > (*crate::gl!(sGame)).difficulty[middlePlayer] {
        highestDifficulty = (*crate::gl!(sGame)).difficulty[leftPlayer];
    } else {
        highestDifficulty = (*crate::gl!(sGame)).difficulty[middlePlayer];
    }
    if (*crate::gl!(sGame)).difficulty[rightPlayer] > highestDifficulty {
        highestDifficulty = (*crate::gl!(sGame)).difficulty[rightPlayer];
    }
    GetNewBerryIdByDifficulty(highestDifficulty, column)
}
unsafe fn GetNewBerryIdByDifficulty(difficulty: u8, column: u8) -> u8 {
    let prevBerryId: u8 = (*crate::gl!(sGame)).prevBerryIds[column];
    'l1: {
        let sw1: i32 = difficulty as i32 % 7;
        let matched =
            sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 6;
        let fall = false;
        if !matched {
            return BERRY_BLUE;
        }
        if sw1 == 0 {
            return BERRY_BLUE;
        }
        if sw1 == 1 {
            return BERRY_GREEN;
        }
        if sw1 == 2 {
            return BERRY_GOLD;
        }
        if sw1 == 3 {
            if prevBerryId == BERRY_BLUE {
                return BERRY_GREEN;
            } else {
                return BERRY_BLUE;
            }
        }
        if fall || sw1 == 4 {
            if prevBerryId == BERRY_BLUE {
                return BERRY_GOLD;
            } else {
                return BERRY_BLUE;
            }
        }
        if fall || sw1 == 5 {
            if prevBerryId == BERRY_GOLD {
                return BERRY_GREEN;
            } else {
                return BERRY_GOLD;
            }
        }
        if fall || sw1 == 6 {
            if prevBerryId == BERRY_BLUE {
                return BERRY_GREEN;
            } else if prevBerryId == BERRY_GREEN {
                return BERRY_GOLD;
            } else {
                return BERRY_BLUE;
            }
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsTotalBerriesMissedOver10(berryResults: *mut CArray<u16, 6>) -> u32 {
    let mut missed: i32 = 0;
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount() as i32 {
        missed += (*berryResults.at(i))[3] as i32;
        i += 1;
    }
    if missed > 10 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IncrementBerryResult(berryIdArg: u8, column: u8, playerId: u8) {
    let mut berryId: u8 = 0;
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    'l1: {
        match berryIdArg {
            BERRY_BLUE | BERRY_GREEN | BERRY_GOLD => {
                berryId = (*crate::gl!(sGame)).players[0].berries.ids[column];
                (*crate::gl!(sGame)).berryResults[playerId][berryId] = IncrementWithLimit(
                    (*crate::gl!(sGame)).berryResults[playerId][berryId] as u32,
                    20000,
                ) as u16;
            }
            BERRY_MISSED => {
                if IsTotalBerriesMissedOver10((*crate::gl!(sGame)).berryResults.as_mut_ptr()) != 0 {
                    break 'l1;
                }
                match numPlayers {
                    5 => match column {
                        0 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                        }
                        1 => {
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                        }
                        2 => {
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                            (*crate::gl!(sGame)).berryResults[4][3] += 1;
                        }
                        3 => {
                            (*crate::gl!(sGame)).berryResults[4][3] += 1;
                        }
                        4 => {
                            (*crate::gl!(sGame)).berryResults[4][3] += 1;
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        5 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        6 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        7 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        8 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        9 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        _ => {}
                    },
                    4 => match column {
                        1 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                        }
                        2 => {
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                        }
                        3 => {
                            (*crate::gl!(sGame)).berryResults[3][3] += 1;
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        4 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        5 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        6 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        7 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        8 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        _ => {}
                    },
                    3 => match column {
                        2 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        3 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                        }
                        4 => {
                            (*crate::gl!(sGame)).berryResults[2][3] += 1;
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        5 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        6 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        7 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        _ => {}
                    },
                    2 => match column {
                        3 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        4 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                        }
                        5 => {
                            (*crate::gl!(sGame)).berryResults[0][3] += 1;
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        6 => {
                            (*crate::gl!(sGame)).berryResults[1][3] += 1;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
unsafe fn UpdateBerriesPickedInRow(picked: u32) {
    if (*crate::gl!(sGame)).numPlayers != MAX_RFU_PLAYERS as u8 {
        return;
    }
    if picked == TRUE as u32 {
        if ({
            (*crate::gl!(sGame)).berriesPickedInRow += 1;
            (*crate::gl!(sGame)).berriesPickedInRow
        }) > (*crate::gl!(sGame)).maxBerriesPickedInRow
        {
            (*crate::gl!(sGame)).maxBerriesPickedInRow = (*crate::gl!(sGame)).berriesPickedInRow;
        }
        if (*crate::gl!(sGame)).berriesPickedInRow > MAX_BERRIES as u16 {
            (*crate::gl!(sGame)).berriesPickedInRow = MAX_BERRIES as u16;
        }
    } else {
        if (*crate::gl!(sGame)).berriesPickedInRow > (*crate::gl!(sGame)).maxBerriesPickedInRow {
            (*crate::gl!(sGame)).maxBerriesPickedInRow = (*crate::gl!(sGame)).berriesPickedInRow;
        }
        (*crate::gl!(sGame)).berriesPickedInRow = 0;
    }
}
unsafe fn SetMaxBerriesPickedInRow() {
    let mut i: u8 = 0;
    while i < (*crate::gl!(sGame)).numPlayers {
        (*crate::gl!(sGame)).berryResults[i][5] = (*crate::gl!(sGame)).maxBerriesPickedInRow;
        i += 1;
    }
}
unsafe fn ResetForPlayAgainPrompt() {
    for i in 0..(MAX_RFU_PLAYERS as u8) {
        for j in 0..NUM_BERRY_COLUMNS {
            (*crate::gl!(sGame)).players[i].berries.fallDist[j] = 0;
        }
        (*crate::gl!(sGame)).players[i].comm.pickState = PICK_NONE;
        (*crate::gl!(sGame)).players[i].comm.ateBerry = FALSE;
        (*crate::gl!(sGame)).difficulty[i] = 0;
        (*crate::gl!(sGame)).berriesEaten[i] = 0;
        (*crate::gl!(sGame)).scoreResults[i].ranking = 0;
        (*crate::gl!(sGame)).scoreResults[i].score = 0;
        (*crate::gl!(sGame)).berryResults[i][0] = 0;
        (*crate::gl!(sGame)).berryResults[i][1] = 0;
        (*crate::gl!(sGame)).berryResults[i][2] = 0;
        (*crate::gl!(sGame)).berryResults[i][3] = 0;
        (*crate::gl!(sGame)).berryResults[i][4] = 0;
        (*crate::gl!(sGame)).berryResults[i][5] = 0;
    }
    (*crate::gl!(sGame)).endSoundState = 0;
    (*crate::gl!(sGame)).berriesPickedInRow = 0;
    (*crate::gl!(sGame)).numGraySquares = 0;
    UpdateAllDodrioAnims();
    UpdateBerrySprites();
}
unsafe fn SetRandomPrize() {
    let mut prizeSet: u8 = 0;
    match (*crate::gl!(sGame)).numPlayers {
        4 => {
            prizeSet = 1;
        }
        5 => {
            prizeSet = 2;
        }
        _ => {}
    }
    let prizeIdx: u8 = (Random() % 10) as u8;
    for i in 0..(MAX_RFU_PLAYERS as u8) {
        (*crate::gl!(sGame)).berryResults[i][4] = sPrizeBerryIds[prizeSet][prizeIdx] as u16;
    }
}
unsafe fn GetBerriesPicked(playerId: u8) -> u32 {
    let sum: u32 = (*crate::gl!(sGame)).berryResults[playerId][0] as u32
        + (*crate::gl!(sGame)).berryResults[playerId][1] as u32
        + (*crate::gl!(sGame)).berryResults[playerId][2] as u32;
    if sum < MAX_BERRIES { sum } else { MAX_BERRIES }
}
pub(crate) unsafe fn TryUpdateRecords() {
    let berriesPicked: u32 = Min(
        GetBerriesPicked((*crate::gl!(sGame)).multiplayerId),
        MAX_BERRIES,
    );
    let score: u32 = Min(GetScore((*crate::gl!(sGame)).multiplayerId), MAX_SCORE);
    if (*crate::gl!(gSaveBlock2Ptr)).berryPick.bestScore < score {
        (*crate::gl!(gSaveBlock2Ptr)).berryPick.bestScore = score;
    }
    if ((*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPicked as u32) < berriesPicked {
        (*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPicked = berriesPicked as u16;
    }
    if (*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPickedInRow
        < (*crate::gl!(sGame)).maxBerriesPickedInRow
    {
        (*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPickedInRow =
            (*crate::gl!(sGame)).maxBerriesPickedInRow;
    }
}
unsafe fn UpdatePickStateQueue(pickState: u8) -> u8 {
    let nextState: u8 = (*crate::gl!(sGame)).pickStateQueue[3];
    let mut i: u8 = 3;
    while i != 0 {
        (*crate::gl!(sGame)).pickStateQueue[i] = (*crate::gl!(sGame)).pickStateQueue[i as i32 - 1];
        i -= 1;
    }
    (*crate::gl!(sGame)).pickStateQueue[0] = pickState;
    nextState
}
unsafe fn HandleWaitPlayAgainInput() {
    if (*crate::gl!(sGame)).inputDelay[(*crate::gl!(sGame)).multiplayerId] == 0 {
        if crate::gl!(gMain.newKeys) as i32 & DPAD_UP != 0 {
            (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState = PICK_MIDDLE;
            (*crate::gl!(sGame)).inputDelay[(*crate::gl!(sGame)).multiplayerId] = 6;
            PlaySE(SE_M_CHARM);
        } else if crate::gl!(gMain.newKeys) as i32 & DPAD_LEFT != 0 {
            (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState = PICK_LEFT;
            (*crate::gl!(sGame)).inputDelay[(*crate::gl!(sGame)).multiplayerId] = 6;
            PlaySE(SE_M_CHARM);
        } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0 {
            (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState = PICK_RIGHT;
            (*crate::gl!(sGame)).inputDelay[(*crate::gl!(sGame)).multiplayerId] = 6;
            PlaySE(SE_M_CHARM);
        } else {
            (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
                .comm
                .pickState = PICK_NONE;
        }
    } else {
        (*crate::gl!(sGame)).inputDelay[(*crate::gl!(sGame)).multiplayerId] -= 1;
    }
}
unsafe fn ResetPickState() {
    (*crate::gl!(sGame)).players[(*crate::gl!(sGame)).multiplayerId]
        .comm
        .pickState = PICK_NONE;
}
pub(crate) unsafe fn GetPrizeItemId() -> u16 {
    (*crate::gl!(sGame)).berryResults[(*crate::gl!(sGame)).multiplayerId][4] + ITEM_CHERI_BERRY
}
unsafe fn GetNumPlayers() -> u8 {
    (*crate::gl!(sGame)).numPlayers
}
unsafe fn GetPlayerName(id: u8) -> *mut u8 {
    if gReceivedRemoteLinkPlayers.get() != 0 {
        return (*gLinkPlayers.as_ptr())[id].name.as_mut_ptr();
    } else {
        return (*crate::gl!(sGame)).players[id].name.as_mut_ptr();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
unsafe fn GetBerryResult(playerId: u8, berryId: u8) -> u16 {
    (*crate::gl!(sGame)).berryResults[playerId][berryId]
}
unsafe fn GetScore(playerId: u8) -> u32 {
    let mut score: u32 = 0;
    for i in 0..BERRY_MISSED {
        score += (*crate::gl!(sGame)).berryResults[playerId][i] as u32
            * sBerryScoreMultipliers[i] as u32;
    }
    let scoreLost: u32 =
        (*crate::gl!(sGame)).berryResults[playerId][3] as u32 * sBerryScoreMultipliers[3] as u32;
    if score <= scoreLost {
        return 0;
    } else {
        return score - scoreLost;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetHighestScore() -> u32 {
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    let mut maxScore: u32 = GetScore(0);
    for i in 1..numPlayers {
        let score: u32 = GetScore(i);
        if score > maxScore {
            maxScore = score;
        }
    }
    Min(maxScore, MAX_SCORE)
}
unsafe fn GetHighestBerryResult(berryId: u8) -> u32 {
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    let mut maxScore: u16 = (*crate::gl!(sGame)).berryResults[0][berryId];
    for i in 0..numPlayers {
        let score: u16 = (*crate::gl!(sGame)).berryResults[i][berryId];
        if score > maxScore {
            maxScore = score;
        }
    }
    maxScore as u32
}
unsafe fn GetScoreByRanking(ranking: u8) -> u32 {
    let mut scores: CArray<u32, 5> = zeroed();
    let mut temp: u32 = 0;
    let mut unsorted: i16 = TRUE as i16;
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    let mut i: u8 = 0;
    while i < numPlayers {
        scores[i] = {
            temp = GetScore(i);
            temp
        };
        i += 1;
    }
    while unsorted != 0 {
        unsorted = FALSE as i16;
        i = 0;
        while (i as i32) < numPlayers as i32 - 1 {
            if scores[i] < scores[i as i32 + 1] {
                temp = scores[i];
                scores[i] = scores[i as i32 + 1];
                scores[i as i32 + 1] = temp;
                unsorted = TRUE as i16;
            }
            i += 1;
        }
    }
    scores[ranking]
}
unsafe fn SetScoreResults() -> u32 {
    let mut i: u8 = 0;
    let mut ranking: u8 = 0;
    let mut nextRanking: u8 = 0;
    let mut playersRanked: u8 = 0;
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    GetHighestScore();
    if GetHighestScore() == 0 {
        for i in 0..numPlayers {
            (*crate::gl!(sGame)).scoreResults[i].ranking = 4;
            (*crate::gl!(sGame)).scoreResults[i].score = 0;
        }
    }
    i = 0;
    while i < numPlayers {
        (*crate::gl!(sGame)).scoreResults[i].score = Min(GetScore(i), MAX_SCORE);
        i += 1;
    }
    loop {
        let score: u32 = GetScoreByRanking(ranking);
        let curRanking: u8 = nextRanking;
        for i in 0..numPlayers {
            if score == (*crate::gl!(sGame)).scoreResults[i].score {
                (*crate::gl!(sGame)).scoreResults[i].ranking = curRanking;
                nextRanking += 1;
                playersRanked += 1;
            }
        }
        ranking = nextRanking;
        if playersRanked >= numPlayers {
            break;
        }
    }
    0
}
unsafe fn GetScoreResults(dst: *mut DodrioGame_ScoreResults, playerId: u8) {
    *dst = (*crate::gl!(sGame)).scoreResults[playerId];
}
unsafe fn GetScoreRanking(playerId: u8) -> u8 {
    let mut ranking: u8 = 0;
    let numPlayers: u8 = (*crate::gl!(sGame)).numPlayers;
    let mut scores: CArray<u32, 5> = CArray([0, 0, 0, 0, 0]);
    let mut i: u8 = 0;
    while i < numPlayers {
        scores[i] = GetScore(i);
        i += 1;
    }
    let playersScore: u32 = scores[playerId];
    for i in 0..(MAX_RFU_PLAYERS as u8) {
        if i != playerId && playersScore < scores[i] {
            ranking += 1;
        }
    }
    ranking
}
pub(crate) unsafe fn TryGivePrize() -> u8 {
    let multiplayerId: u8 = (*crate::gl!(sGame)).multiplayerId;
    let itemId: u16 = GetPrizeItemId();
    if GetScore(multiplayerId) != GetHighestScore() {
        return NO_PRIZE;
    }
    if CheckBagHasSpace(itemId, 1) == 0 {
        return PRIZE_NO_ROOM;
    }
    AddBagItem(itemId, 1);
    if CheckBagHasSpace(itemId, 1) == 0 {
        return PRIZE_FILLED_BAG;
    }
    PRIZE_RECEIVED
}
unsafe fn IncrementWithLimit(num: u32, max: u32) -> u32 {
    if num < max {
        return num + 1;
    } else {
        return max;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn Min(a: u32, b: u32) -> u32 {
    if a < b {
        return a;
    } else {
        return b;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetPlayerIdByPos(id: u8) -> u8 {
    (*crate::gl!(sGame)).posToPlayerId[id]
}
#[unsafe(no_mangle)]
pub unsafe fn IsDodrioInParty() {
    for i in 0..PARTY_SIZE {
        if GetMonData2(
            crate::gl!(&raw gPlayerParty[i]),
            MON_DATA_SANITY_HAS_SPECIES,
        ) != 0
            && GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES_OR_EGG)
                == SPECIES_DODRIO
        {
            gSpecialVar_Result = TRUE as u16;
            return;
        }
    }
    gSpecialVar_Result = FALSE as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn ShowDodrioBerryPickingRecords() {
    let taskId: u8 = CreateTask(Some(Task_ShowDodrioBerryPickingRecords), 0);
    Task_ShowDodrioBerryPickingRecords(taskId);
}
pub(crate) unsafe fn Task_ShowDodrioBerryPickingRecords(taskId: u8) {
    let mut window: WindowTemplate = zeroed();
    let mut width: i32 = 0;
    let mut widthCurr: i32 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            window = *sWindowTemplates_Records;
            width = GetStringWidth(
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_BerryPickingRecords)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
            );
            for i in 0..3i32 {
                widthCurr = GetStringWidth(FONT_NORMAL, sRecordsTexts[i], 0) + 50;
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
            PrintRecordsText(*data.at(1) as u8, width);
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
pub(crate) unsafe fn PrintRecordsText(windowId: u8, width: i32) {
    let mut x: i32 = 0;
    let mut numWidth: i32 = 0;
    let mut recordNums: CArray<i32, 3> = zeroed();
    recordNums[0] = (*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPicked as i32;
    recordNums[1] = (*crate::gl!(gSaveBlock2Ptr)).berryPick.bestScore as i32;
    recordNums[2] = (*crate::gl!(gSaveBlock2Ptr)).berryPick.berriesPickedInRow as i32;
    LoadUserWindowBorderGfx_(windowId, 0x21D, 208);
    DrawTextBorderOuter(windowId, 0x21D, 13);
    FillWindowPixelBuffer(windowId, 17);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_BerryPickingRecords).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_BerryPickingRecords).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            width * 8,
        ) as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    for i in 0..NUM_RECORD_TYPES {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            recordNums[i],
            STR_CONV_MODE_LEFT_ALIGN,
            sRecordNumMaxDigits[i],
        );
        numWidth = GetStringWidth(FONT_NORMAL, gStringVar1.as_mut_ptr(), -1);
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            sRecordsTexts[i],
            0,
            sRecordTextYCoords[i][0],
            TEXT_SKIP_DRAW,
            None,
        );
        x = width * 8 - numWidth;
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            x as u8,
            sRecordNumYCoords[i][0],
            TEXT_SKIP_DRAW,
            None,
        );
    }
    PutWindowTilemap(windowId);
}
unsafe fn Debug_UpdateNumPlayers() {
    (*crate::gl!(sGame)).numPlayers = GetLinkPlayerCount();
}
unsafe fn Debug_SetPlayerNamesAndResults() {
    let mut playerId: u8 = (*crate::gl!(sGame)).numPlayers;
    while playerId < 5 {
        StringCopy(
            (*gLinkPlayers.as_ptr())[playerId].name.as_mut_ptr(),
            sDebug_PlayerNames[playerId],
        );
        playerId += 1;
    }
    (*crate::gl!(sGame)).numPlayers = MAX_RFU_PLAYERS as u8;
    for i in 0..NUM_BERRY_TYPES {
        playerId = 0;
        while playerId < (*crate::gl!(sGame)).numPlayers {
            (*crate::gl!(sGame)).berryResults[playerId][i] = sDebug_BerryResults[playerId][i];
            playerId += 1;
        }
    }
}
unsafe fn SendPacket_ReadyToStart(ready: u32) {
    let mut packet: ReadyToStartPacket = zeroed();
    packet.id = PACKET_READY_START;
    packet.ready = ready as u8;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_ReadyToStart(playerId: u32) -> u32 {
    if crate::gl!(gRecvCmds[0][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    let packet: *mut ReadyToStartPacket =
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[playerId][1] as *mut c_void as *mut ReadyToStartPacket;
    if (*packet).id == PACKET_READY_START {
        return (*packet).ready as u32;
    }
    FALSE as u32
}
unsafe fn SendPacket_GameState(
    player: *mut DodrioGame_Player,
    player1: *mut DodrioGame_PlayerCommData,
    player2: *mut DodrioGame_PlayerCommData,
    player3: *mut DodrioGame_PlayerCommData,
    player4: *mut DodrioGame_PlayerCommData,
    player5: *mut DodrioGame_PlayerCommData,
    numGraySquares: u8,
    berriesFalling: u32,
    allReadyToEnd: u32,
) {
    let mut packet: GameStatePacket = zeroed();
    let berries: *mut DodrioGame_Berries = &raw mut (*player).berries;
    packet.id = PACKET_GAME_STATE;
    packet.set_fallDist_Col0((*berries).fallDist[0]);
    packet.set_fallDist_Col1((*berries).fallDist[1]);
    packet.set_fallDist_Col2((*berries).fallDist[2] as u16);
    packet.set_fallDist_Col3((*berries).fallDist[3] as u16);
    packet.set_fallDist_Col4((*berries).fallDist[4] as u16);
    packet.set_fallDist_Col5((*berries).fallDist[5] as u16);
    packet.set_fallDist_Col6((*berries).fallDist[6] as u16);
    packet.set_fallDist_Col7((*berries).fallDist[7] as u16);
    packet.set_fallDist_Col8((*berries).fallDist[8] as u16);
    packet.set_fallDist_Col9((*berries).fallDist[9] as u16);
    packet.set_berryId_Col0((*berries).ids[0] as u16);
    packet.set_berryId_Col1((*berries).ids[1] as u16);
    packet.set_berryId_Col2((*berries).ids[2] as u16);
    packet.set_berryId_Col3((*berries).ids[3] as u16);
    packet.set_berryId_Col4((*berries).ids[4] as u16);
    packet.set_berryId_Col5((*berries).ids[5] as u16);
    packet.set_berryId_Col6((*berries).ids[6] as u16);
    packet.set_berryId_Col7((*berries).ids[7] as u16);
    packet.set_berryId_Col8((*berries).ids[8]);
    packet.set_berryId_Col9((*berries).ids[9]);
    packet.set_pickState_Player1((*player1).pickState);
    packet.set_pickState_Player2((*player2).pickState);
    packet.set_pickState_Player3((*player3).pickState);
    packet.set_pickState_Player4((*player4).pickState);
    packet.set_pickState_Player5((*player5).pickState);
    packet.set_ateBerry_Player1((*player1).ateBerry);
    packet.set_ateBerry_Player2((*player2).ateBerry);
    packet.set_ateBerry_Player3((*player3).ateBerry);
    packet.set_ateBerry_Player4((*player4).ateBerry);
    packet.set_ateBerry_Player5((*player5).ateBerry);
    packet.set_missedBerry_Player1((*player1).missedBerry);
    packet.set_missedBerry_Player2((*player2).missedBerry);
    packet.set_missedBerry_Player3((*player3).missedBerry);
    packet.set_missedBerry_Player4((*player4).missedBerry);
    packet.set_missedBerry_Player5((*player5).missedBerry);
    packet.set_numGraySquares(numGraySquares);
    packet.set_berriesFalling(berriesFalling as u8);
    packet.set_allReadyToEnd(allReadyToEnd as u8);
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_GameState(
    playerId: u32,
    player: *mut DodrioGame_Player,
    player1: *mut DodrioGame_PlayerCommData,
    player2: *mut DodrioGame_PlayerCommData,
    player3: *mut DodrioGame_PlayerCommData,
    player4: *mut DodrioGame_PlayerCommData,
    player5: *mut DodrioGame_PlayerCommData,
    numGraySquares: *mut u8,
    berriesFalling: *mut u32,
    allReadyToEnd: *mut u32,
) -> u32 {
    let berries: *mut DodrioGame_Berries = &raw mut (*player).berries;
    if crate::gl!(gRecvCmds[0][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    let packet: *mut GameStatePacket =
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[0][1] as *mut c_void as *mut GameStatePacket;
    if (*packet).id == PACKET_GAME_STATE {
        (*berries).fallDist[0] = (*packet).fallDist_Col0();
        (*berries).fallDist[1] = (*packet).fallDist_Col1();
        (*berries).fallDist[2] = (*packet).fallDist_Col2() as u8;
        (*berries).fallDist[3] = (*packet).fallDist_Col3() as u8;
        (*berries).fallDist[4] = (*packet).fallDist_Col4() as u8;
        (*berries).fallDist[5] = (*packet).fallDist_Col5() as u8;
        (*berries).fallDist[6] = (*packet).fallDist_Col6() as u8;
        (*berries).fallDist[7] = (*packet).fallDist_Col7() as u8;
        (*berries).fallDist[8] = (*packet).fallDist_Col8() as u8;
        (*berries).fallDist[9] = (*packet).fallDist_Col9() as u8;
        (*berries).fallDist[10] = (*packet).fallDist_Col0();
        (*berries).ids[0] = (*packet).berryId_Col0() as u8;
        (*berries).ids[1] = (*packet).berryId_Col1() as u8;
        (*berries).ids[2] = (*packet).berryId_Col2() as u8;
        (*berries).ids[3] = (*packet).berryId_Col3() as u8;
        (*berries).ids[4] = (*packet).berryId_Col4() as u8;
        (*berries).ids[5] = (*packet).berryId_Col5() as u8;
        (*berries).ids[6] = (*packet).berryId_Col6() as u8;
        (*berries).ids[7] = (*packet).berryId_Col7() as u8;
        (*berries).ids[8] = (*packet).berryId_Col8();
        (*berries).ids[9] = (*packet).berryId_Col9();
        (*berries).ids[10] = (*packet).berryId_Col0() as u8;
        (*player1).pickState = (*packet).pickState_Player1();
        (*player1).ateBerry = (*packet).ateBerry_Player1();
        (*player1).missedBerry = (*packet).missedBerry_Player1();
        (*player2).pickState = (*packet).pickState_Player2();
        (*player2).ateBerry = (*packet).ateBerry_Player2();
        (*player2).missedBerry = (*packet).missedBerry_Player2();
        (*player3).pickState = (*packet).pickState_Player3();
        (*player3).ateBerry = (*packet).ateBerry_Player3();
        (*player3).missedBerry = (*packet).missedBerry_Player3();
        (*player4).pickState = (*packet).pickState_Player4();
        (*player4).ateBerry = (*packet).ateBerry_Player4();
        (*player4).missedBerry = (*packet).missedBerry_Player4();
        (*player5).pickState = (*packet).pickState_Player5();
        (*player5).ateBerry = (*packet).ateBerry_Player5();
        (*player5).missedBerry = (*packet).missedBerry_Player5();
        *numGraySquares = (*packet).numGraySquares();
        *berriesFalling = (*packet).berriesFalling() as u32;
        *allReadyToEnd = (*packet).allReadyToEnd() as u32;
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn SendPacket_PickState(pickState: u8) {
    let mut packet: PickStatePacket = zeroed();
    packet.id = PACKET_PICK_STATE;
    packet.pickState = pickState;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_PickState(playerId: u32, pickState: *mut u8) -> u32 {
    if crate::gl!(gRecvCmds[0][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    let packet: *mut PickStatePacket =
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[playerId][1] as *mut c_void as *mut PickStatePacket;
    if (*packet).id == PACKET_PICK_STATE {
        *pickState = (*packet).pickState;
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn SendPacket_ReadyToEnd(ready: u32) {
    let mut packet: ReadyToEndPacket = zeroed();
    packet.id = PACKET_READY_END;
    packet.ready = ready;
    Rfu_SendPacket(&raw mut packet as *mut c_void);
}
unsafe fn RecvPacket_ReadyToEnd(playerId: u32) -> u32 {
    if crate::gl!(gRecvCmds[0][0]) as i32 & RFUCMD_MASK != RFUCMD_SEND_PACKET {
        return FALSE as u32;
    }
    let packet: *mut ReadyToEndPacket =
        &raw mut (*(crate::gl!(&raw @at crate::link::gRecvCmds;).cast_const())
            .cast::<CArray<CArray<u16, 8>, 5>>()
            .cast_mut())[playerId][1] as *mut c_void as *mut ReadyToEndPacket;
    if (*packet).id == PACKET_READY_END {
        return (*packet).ready;
    }
    FALSE as u32
}
unsafe fn LoadDodrioGfx() {
    let ptr: *mut c_void = AllocZeroed(0x3000);
    let mut normal: SpritePalette = zeroed();
    normal.data = sDodrioNormal_Pal.as_ptr().cast_mut();
    normal.tag = PALTAG_DODRIO_NORMAL;
    let mut shiny: SpritePalette = zeroed();
    shiny.data = sDodrioShiny_Pal.as_ptr().cast_mut();
    shiny.tag = PALTAG_DODRIO_SHINY;
    LZ77UnCompWram(sDodrio_Gfx.as_ptr().cast_mut(), ptr);
    if !ptr.is_null() {
        let mut sheet: SpriteSheet = zeroed();
        sheet.data = ptr;
        sheet.size = 0x3000;
        sheet.tag = GFXTAG_DODRIO;
        LoadSpriteSheet(&raw mut sheet);
        Free(ptr);
    }
    LoadSpritePalette(&raw mut normal);
    LoadSpritePalette(&raw mut shiny);
}
unsafe fn CreateDodrioSprite(
    monInfo: *mut DodrioGame_MonInfo,
    playerId: u8,
    id: u8,
    numPlayers: u8,
) {
    let mut template: SpriteTemplate = zeroed();
    template.tileTag = GFXTAG_DODRIO;
    template.paletteTag = (*monInfo).isShiny as u16;
    template.oam = (&raw const *sOamData_Dodrio).cast_mut();
    template.anims = sAnims_Dodrio.as_ptr().cast_mut();
    template.images = null_mut();
    template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    template.callback = Some(SpriteCB_Dodrio);
    crate::gl!(sDodrioSpriteIds[id] = AllocZeroed(4) as *mut u16);
    *crate::gl!(sDodrioSpriteIds[id]) = CreateSprite(
        &raw mut template,
        GetDodrioXPos(playerId, numPlayers),
        136,
        3,
    ) as u16;
    SetDodrioInvisibility(TRUE, id);
}
pub(crate) unsafe fn SpriteCB_Dodrio(sprite: *mut Sprite) {
    match (*sprite).data[sState] {
        0 => {}
        1 => {
            DoDodrioMissedAnim(sprite);
        }
        2 => {
            DoDodrioIntroAnim(sprite);
        }
        _ => {}
    }
}
unsafe fn StartDodrioMissedAnim(unused: u8) {
    let sprite: *mut Sprite = {
        let gl_i0 = *{
            let gl_i0 = GetMultiplayerId();
            crate::gl!(sDodrioSpriteIds[gl_i0])
        };
        crate::gl!(&raw gSprites[gl_i0])
    };
    (*sprite).data[sState] = 1;
    (*sprite).data[sTimer] = 0;
    (*sprite).data[sUnused1] = 0;
    (*sprite).data[sUnused2] = 0;
    (*sprite).data[sUnused3] = 0;
}
unsafe fn StartDodrioIntroAnim(unused: u8) {
    let sprite: *mut Sprite = {
        let gl_i1 = *{
            let gl_i1 = GetMultiplayerId();
            crate::gl!(sDodrioSpriteIds[gl_i1])
        };
        crate::gl!(&raw gSprites[gl_i1])
    };
    (*sprite).data[sState] = 2;
    (*sprite).data[sTimer] = 0;
    (*sprite).data[sUnused1] = 0;
    (*sprite).data[sUnused2] = 0;
    (*sprite).data[sUnused3] = 0;
}
unsafe fn DoDodrioMissedAnim(sprite: *mut Sprite) -> u32 {
    let mut x: i8 = 0;
    let state: u8 = (({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) / 2
        % 4) as u8;
    if (*sprite).data[sTimer] >= 3 {
        match state {
            1 | 2 => {
                x = -1;
            }
            _ => {
                x = 1;
            }
        }
        (*sprite).x += x as i16;
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) >= 40
        {
            (*sprite).data[sState] = 0;
            (*sprite).x = GetDodrioXPos(0, GetNumPlayers());
        }
    }
    0
}
unsafe fn DoDodrioIntroAnim(sprite: *mut Sprite) -> u32 {
    let mut pickState: u8 = (({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) / 13
        % 4) as u8;
    if (*sprite).data[sTimer] % 13 == 0 && pickState != PICK_NONE {
        PlaySE(SE_M_CHARM);
    }
    if (*sprite).data[sTimer] >= 104 {
        (*sprite).data[sState] = 0;
        pickState = PICK_NONE;
    }
    SetDodrioAnim(GetMultiplayerId(), pickState);
    0
}
unsafe fn FreeDodrioSprites(numPlayers: u8) {
    for i in 0..numPlayers {
        let sprite: *mut Sprite = {
            let gl_i2 = *crate::gl!(sDodrioSpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i2])
        };
        if !sprite.is_null() {
            DestroySpriteAndFreeResources(sprite);
        }
    }
}
unsafe fn SetDodrioInvisibility(invisible: u8, id: u8) {
    (*gSprites.as_ptr())[*crate::gl!(sDodrioSpriteIds[id])].set_invisible(invisible as u16);
}
unsafe fn SetAllDodrioInvisibility(invisible: u8, count: u8) {
    for i in 0..count {
        SetDodrioInvisibility(invisible, i);
    }
}
unsafe fn SetDodrioAnim(id: u8, pickState: u8) {
    StartSpriteAnim(
        {
            let gl_i3 = *crate::gl!(sDodrioSpriteIds[id]);
            crate::gl!(&raw gSprites[gl_i3])
        },
        pickState,
    );
}
pub(crate) fn SpriteCB_Status(sprite: *mut Sprite) {}
unsafe fn InitStatusBarPos() {
    for i in 0..NUM_STATUS_SQUARES {
        let sprite: *mut Sprite = {
            let gl_i4 = (*crate::gl!(sStatusBar)).spriteIds[i];
            crate::gl!(&raw gSprites[gl_i4])
        };
        (*sprite).x = i as i16 * 16 + 48;
        (*sprite).y = -8 - i as i16 * 8;
        (*crate::gl!(sStatusBar)).entered[i] = FALSE;
    }
}
unsafe fn CreateStatusBarSprites() {
    let ptr: *mut c_void = AllocZeroed(0x180);
    let mut pal: SpritePalette = zeroed();
    pal.data = sStatus_Pal.as_ptr().cast_mut();
    pal.tag = PALTAG_STATUS;
    LZ77UnCompWram(sStatus_Gfx.as_ptr().cast_mut(), ptr);
    if !ptr.is_null() {
        let mut sheet: SpriteSheet = zeroed();
        sheet.data = ptr;
        sheet.size = 0x180;
        sheet.tag = GFXTAG_STATUS;
        let mut template: SpriteTemplate = zeroed();
        template.tileTag = GFXTAG_STATUS;
        template.paletteTag = PALTAG_STATUS;
        template.oam = (&raw const *sOamData_16x16_Priority0).cast_mut();
        template.anims = sAnims_StatusBar.as_ptr().cast_mut();
        template.images = null_mut();
        template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
        template.callback = Some(SpriteCB_Status);
        crate::gl!(sStatusBar = AllocZeroed(64) as *mut StatusBar);
        LoadSpriteSheet(&raw mut sheet);
        LoadSpritePalette(&raw mut pal);
        for i in 0..NUM_STATUS_SQUARES {
            (*crate::gl!(sStatusBar)).spriteIds[i] =
                CreateSprite(&raw mut template, i as i16 * 16 + 48, -8 - i as i16 * 8, 0) as u16;
        }
    }
    Free(ptr);
}
unsafe fn FreeStatusBar() {
    for i in 0..NUM_STATUS_SQUARES {
        let sprite: *mut Sprite = {
            let gl_i5 = (*crate::gl!(sStatusBar)).spriteIds[i];
            crate::gl!(&raw gSprites[gl_i5])
        };
        if !sprite.is_null() {
            DestroySpriteAndFreeResources(sprite);
        }
    }
    Free(crate::gl!(sStatusBar) as *mut c_void);
    crate::gl!(sStatusBar = null_mut());
}
unsafe fn DoStatusBarIntro() -> u32 {
    let mut animActive: u32 = FALSE as u32;
    for i in 0..NUM_STATUS_SQUARES {
        'l1: {
            let sprite: *mut Sprite = {
                let gl_i6 = (*crate::gl!(sStatusBar)).spriteIds[i];
                crate::gl!(&raw gSprites[gl_i6])
            };
            (*crate::gl!(sStatusBar)).yChange[i] = 2;
            if (*crate::gl!(sStatusBar)).entered[i] != 0 && (*sprite).y == 8 {
                break 'l1;
            }
            animActive = TRUE as u32;
            if (*sprite).y == 8 {
                if (*crate::gl!(sStatusBar)).entered[i] != 0 {
                    break 'l1;
                }
                (*crate::gl!(sStatusBar)).entered[i] = TRUE;
                (*crate::gl!(sStatusBar)).yChange[i] = -16;
                PlaySE(SE_CLICK);
            }
            (*sprite).y += (*crate::gl!(sStatusBar)).yChange[i];
        }
    }
    if animActive != 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn UpdateStatusBarAnim(numEmpty: u8) {
    let mut i: u8 = 0;
    if numEmpty > NUM_STATUS_SQUARES {
        for i in 0..NUM_STATUS_SQUARES {
            StartSpriteAnim(
                {
                    let gl_i7 = (*crate::gl!(sStatusBar)).spriteIds[i];
                    crate::gl!(&raw gSprites[gl_i7])
                },
                STATUS_GRAY,
            );
        }
    } else {
        i = 0;
        while (i as i32) < NUM_STATUS_SQUARES as i32 - numEmpty as i32 {
            if numEmpty > 6 {
                (*crate::gl!(sStatusBar)).flashTimer += numEmpty as u16 - 6;
                if (*crate::gl!(sStatusBar)).flashTimer > 30 {
                    (*crate::gl!(sStatusBar)).flashTimer = 0;
                } else if (*crate::gl!(sStatusBar)).flashTimer > 10 {
                    StartSpriteAnim(
                        {
                            let gl_i8 = (*crate::gl!(sStatusBar)).spriteIds[i];
                            crate::gl!(&raw gSprites[gl_i8])
                        },
                        STATUS_RED,
                    );
                } else {
                    StartSpriteAnim(
                        {
                            let gl_i9 = (*crate::gl!(sStatusBar)).spriteIds[i];
                            crate::gl!(&raw gSprites[gl_i9])
                        },
                        STATUS_YELLOW,
                    );
                }
            } else {
                StartSpriteAnim(
                    {
                        let gl_i10 = (*crate::gl!(sStatusBar)).spriteIds[i];
                        crate::gl!(&raw gSprites[gl_i10])
                    },
                    STATUS_YELLOW,
                );
            }
            i += 1;
        }
        while i < NUM_STATUS_SQUARES {
            StartSpriteAnim(
                {
                    let gl_i11 = (*crate::gl!(sStatusBar)).spriteIds[i];
                    crate::gl!(&raw gSprites[gl_i11])
                },
                STATUS_GRAY,
            );
            i += 1;
        }
    }
}
unsafe fn SetStatusBarInvisibility(invisible: u8) {
    for i in 0..NUM_STATUS_SQUARES {
        (*gSprites.as_ptr())[(*crate::gl!(sStatusBar)).spriteIds[i]]
            .set_invisible(invisible as u16);
    }
}
pub(crate) unsafe fn LoadBerryGfx() {
    let ptr: *mut c_void = AllocZeroed(0x480);
    let mut pal: SpritePalette = zeroed();
    pal.data = sBerries_Pal.as_ptr().cast_mut();
    pal.tag = PALTAG_BERRIES;
    LZ77UnCompWram(sBerries_Gfx.as_ptr().cast_mut(), ptr);
    if !ptr.is_null() {
        let mut sheet: SpriteSheet = zeroed();
        sheet.data = ptr;
        sheet.size = 0x480;
        sheet.tag = GFXTAG_BERRIES;
        LoadSpriteSheet(&raw mut sheet);
    }
    LoadSpritePalette(&raw mut pal);
    Free(ptr);
}
pub(crate) unsafe fn CreateBerrySprites() {
    let mut x: i16 = 0;
    let mut berry: SpriteTemplate = zeroed();
    berry.tileTag = GFXTAG_BERRIES;
    berry.paletteTag = PALTAG_BERRIES;
    berry.oam = (&raw const *sOamData_Berry).cast_mut();
    berry.anims = sAnims_Berry.as_ptr().cast_mut();
    berry.images = null_mut();
    berry.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    berry.callback = Some(SpriteCallbackDummy);
    let mut berryIcon: SpriteTemplate = zeroed();
    berryIcon.tileTag = GFXTAG_BERRIES;
    berryIcon.paletteTag = PALTAG_BERRIES;
    berryIcon.oam = (&raw const *sOamData_16x16_Priority0).cast_mut();
    berryIcon.anims = sAnims_Berry.as_ptr().cast_mut();
    berryIcon.images = null_mut();
    berryIcon.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    berryIcon.callback = Some(SpriteCallbackDummy);
    let mut i: u8 = 0;
    while i < NUM_BERRY_COLUMNS {
        crate::gl!(sBerrySpriteIds[i] = AllocZeroed(4) as *mut u16);
        x = i as i16 * 16;
        *crate::gl!(sBerrySpriteIds[i]) =
            CreateSprite(&raw mut berry, x + i as i16 * 8, 8, 1) as u16;
        SetBerryInvisibility(i, TRUE);
        i += 1;
    }
    for i in 0..NUM_BERRY_TYPES {
        crate::gl!(sBerryIconSpriteIds[i] = AllocZeroed(4) as *mut u16);
        if i == BERRY_MISSED {
            *crate::gl!(sBerryIconSpriteIds[i]) =
                CreateSprite(&raw mut berryIcon, sBerryIconXCoords[i], 49, 0) as u16;
        } else {
            *crate::gl!(sBerryIconSpriteIds[i]) =
                CreateSprite(&raw mut berryIcon, sBerryIconXCoords[i], 52, 0) as u16;
        }
        StartSpriteAnim(
            {
                let gl_i12 = *crate::gl!(sBerryIconSpriteIds[i]);
                crate::gl!(&raw gSprites[gl_i12])
            },
            i,
        );
    }
    SetBerryIconsInvisibility(TRUE);
}
unsafe fn FreeBerrySprites() {
    let mut sprite: *mut Sprite = null_mut();
    let mut i: u8 = 0;
    while i < NUM_BERRY_COLUMNS {
        sprite = {
            let gl_i13 = *crate::gl!(sBerrySpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i13])
        };
        if !sprite.is_null() {
            DestroySprite(sprite);
        }
        Free(crate::gl!(sBerrySpriteIds[i]) as *mut c_void);
        crate::gl!(sBerrySpriteIds[i] = null_mut());
        i += 1;
    }
    for i in 0..NUM_BERRY_TYPES {
        sprite = {
            let gl_i14 = *crate::gl!(sBerryIconSpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i14])
        };
        if !sprite.is_null() {
            DestroySprite(sprite);
        }
        Free(crate::gl!(sBerryIconSpriteIds[i]) as *mut c_void);
        crate::gl!(sBerryIconSpriteIds[i] = null_mut());
    }
}
unsafe fn SetBerryInvisibility(id: u8, invisible: u8) {
    (*gSprites.as_ptr())[*crate::gl!(sBerrySpriteIds[id])].set_invisible(invisible as u16);
}
unsafe fn SetBerryIconsInvisibility(invisible: u8) {
    for i in 0..NUM_BERRY_TYPES {
        (*gSprites.as_ptr())[*crate::gl!(sBerryIconSpriteIds[i])].set_invisible(invisible as u16);
    }
}
unsafe fn SetBerryYPos(id: u8, y: u8) {
    {
        let gl_i15 = *crate::gl!(sBerrySpriteIds[id]);
        crate::gl!(gSprites[gl_i15].y = y as i16 * 8)
    };
}
unsafe fn SetBerryAnim(id: u16, animNum: u8) {
    StartSpriteAnim(
        {
            let gl_i16 = *crate::gl!(sBerrySpriteIds[id]);
            crate::gl!(&raw gSprites[gl_i16])
        },
        animNum,
    );
}
fn UnusedSetSpritePos(spriteId: u8) {
    crate::gl!(gSprites[spriteId].x = 20 * spriteId as i16 + 50);
    crate::gl!(gSprites[spriteId].y = 50);
}
pub(crate) unsafe fn SpriteCB_Cloud(sprite: *mut Sprite) {
    if (*sprite).data[1] != TRUE as i16 {
        for i in 0..NUM_CLOUDS {
            if ({
                *(*sCloudSpriteIds.as_ptr())[i].at(1) += 1;
                *(*sCloudSpriteIds.as_ptr())[i].at(1)
            }) > moveDelays_0[i] as u16
            {
                (*sprite).x -= 1;
                *(*sCloudSpriteIds.as_ptr())[i].at(1) = 0;
            }
        }
    }
}
pub(crate) unsafe fn CreateCloudSprites() {
    let ptr: *mut c_void = AllocZeroed(0x400);
    let mut pal: SpritePalette = zeroed();
    pal.data = sCloud_Pal.as_ptr().cast_mut();
    pal.tag = PALTAG_CLOUD;
    LZ77UnCompWram(sCloud_Gfx.as_ptr().cast_mut(), ptr);
    if !ptr.is_null() {
        let mut sheet: SpriteSheet = zeroed();
        sheet.data = ptr;
        sheet.size = 0x400;
        sheet.tag = GFXTAG_CLOUD;
        let mut template: SpriteTemplate = zeroed();
        template.tileTag = GFXTAG_CLOUD;
        template.paletteTag = PALTAG_CLOUD;
        template.oam = (&raw const *sOamData_Cloud).cast_mut();
        template.anims = sAnims_Cloud.as_ptr().cast_mut();
        template.images = null_mut();
        template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
            .cast::<CArray<*mut AffineAnimCmd, 0>>())
        .as_ptr()
        .cast_mut();
        template.callback = Some(SpriteCB_Cloud);
        LoadSpriteSheet(&raw mut sheet);
        LoadSpritePalette(&raw mut pal);
        for i in 0..NUM_CLOUDS {
            crate::gl!(sCloudSpriteIds[i] = AllocZeroed(4) as *mut u16);
            *crate::gl!(sCloudSpriteIds[i]) = CreateSprite(
                &raw mut template,
                sCloudStartCoords[i][0],
                sCloudStartCoords[i][1],
                4,
            ) as u16;
        }
    }
    Free(ptr);
}
unsafe fn ResetCloudPos() {
    for i in 0..NUM_CLOUDS {
        let sprite: *mut Sprite = {
            let gl_i17 = *crate::gl!(sCloudSpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i17])
        };
        (*sprite).data[1] = TRUE as i16;
        (*sprite).x = sCloudStartCoords[i][0];
        (*sprite).y = sCloudStartCoords[i][1];
    }
}
unsafe fn StartCloudMovement() {
    for i in 0..NUM_CLOUDS {
        let sprite: *mut Sprite = {
            let gl_i18 = *crate::gl!(sCloudSpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i18])
        };
        (*sprite).data[1] = FALSE as i16;
    }
}
unsafe fn FreeCloudSprites() {
    for i in 0..NUM_CLOUDS {
        let sprite: *mut Sprite = {
            let gl_i19 = *crate::gl!(sCloudSpriteIds[i]);
            crate::gl!(&raw gSprites[gl_i19])
        };
        if !sprite.is_null() {
            DestroySprite(sprite);
        }
        Free(crate::gl!(sCloudSpriteIds[i]) as *mut c_void);
        crate::gl!(sCloudSpriteIds[i] = null_mut());
    }
}
unsafe fn SetCloudInvisibility(invisible: u8) {
    for i in 0..NUM_CLOUDS {
        (*gSprites.as_ptr())[*crate::gl!(sCloudSpriteIds[i])].set_invisible(invisible as u16);
    }
}
unsafe fn GetDodrioXPos(playerId: u8, numPlayers: u8) -> i16 {
    let mut x: i16 = 0;
    match numPlayers {
        1 => {
            x = 15;
        }
        2 => match playerId {
            0 => {
                x = 12;
            }
            1 => {
                x = 18;
            }
            _ => {}
        },
        3 => match playerId {
            0 => {
                x = 15;
            }
            1 => {
                x = 21;
            }
            2 => {
                x = 9;
            }
            _ => {}
        },
        4 => match playerId {
            0 => {
                x = 12;
            }
            1 => {
                x = 18;
            }
            2 => {
                x = 24;
            }
            3 => {
                x = 6;
            }
            _ => {}
        },
        5 => match playerId {
            0 => {
                x = 15;
            }
            1 => {
                x = 21;
            }
            2 => {
                x = 27;
            }
            3 => {
                x = 3;
            }
            4 => {
                x = 9;
            }
            _ => {}
        },
        _ => {}
    }
    x * 8
}
unsafe fn ResetBerryAndStatusBarSprites() {
    for i in 0..NUM_BERRY_COLUMNS {
        SetBerryInvisibility(i, TRUE);
        SetBerryYPos(i, 1);
    }
    SetStatusBarInvisibility(FALSE);
}
unsafe fn LoadWindowFrameGfx(frameId: u8) {
    LoadBgTiles(
        BG_INTERFACE,
        (*GetWindowFrameTilesPal(frameId)).tiles as *mut c_void,
        0x120,
        1,
    );
    LoadPalette(
        (*GetWindowFrameTilesPal(frameId)).pal as *mut c_void,
        160,
        32,
    );
}
unsafe fn LoadUserWindowFrameGfx() {
    LoadUserWindowBorderGfx_(0, 0xA, 176);
}
unsafe fn ResetGfxState() {
    (*crate::gl!(sGfx)).finished = FALSE as u32;
    (*crate::gl!(sGfx)).state = 0;
    (*crate::gl!(sGfx)).loadState = 0;
    (*crate::gl!(sGfx)).cursorSelection = 0;
    (*crate::gl!(sGfx)).playAgainState = PLAY_AGAIN_NONE;
}
unsafe fn DrawYesNoMessageWindow(template: *mut WindowTemplate) {
    let pal: u8 = 10;
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        1,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop - 1,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        2,
        (*template).tilemapLeft,
        (*template).tilemapTop - 1,
        (*template).width,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        3,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop - 1,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        4,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop,
        1,
        (*template).height,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        6,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop,
        1,
        (*template).height,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        7,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        8,
        (*template).tilemapLeft,
        (*template).tilemapTop + (*template).height,
        (*template).width,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        9,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        pal,
    );
}
unsafe fn DrawMessageWindow(template: *mut WindowTemplate) {
    let pal: u8 = 11;
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        10,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop - 1,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        11,
        (*template).tilemapLeft,
        (*template).tilemapTop - 1,
        (*template).width,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        12,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop - 1,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        13,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop,
        1,
        (*template).height,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        15,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop,
        1,
        (*template).height,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        16,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        17,
        (*template).tilemapLeft,
        (*template).tilemapTop + (*template).height,
        (*template).width,
        1,
        pal,
    );
    FillBgTilemapBufferRect(
        BG_INTERFACE,
        18,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        pal,
    );
}
unsafe fn InitGameGfx(ptr: *mut DodrioGame_Gfx) {
    crate::gl!(sGfx = ptr);
    (*crate::gl!(sGfx)).finished = FALSE as u32;
    (*crate::gl!(sGfx)).state = 0;
    (*crate::gl!(sGfx)).loadState = 0;
    (*crate::gl!(sGfx)).cursorSelection = 0;
    (*crate::gl!(sGfx)).playAgainState = PLAY_AGAIN_NONE;
    (*crate::gl!(sGfx)).taskId = CreateTask(Some(Task_TryRunGfxFunc), 3);
    SetGfxFunc(Some(LoadGfx));
}
unsafe fn FreeAllWindowBuffers_() {
    FreeAllWindowBuffers();
}
unsafe fn SetGfxFuncById(funcId: u8) {
    for i in 0..10u8 {
        if sGfxFuncs[i].id == funcId {
            SetGfxFunc(sGfxFuncs[i].func);
        }
    }
}
pub(crate) unsafe fn Task_TryRunGfxFunc(taskId: u8) {
    if (*crate::gl!(sGfx)).finished == 0 {
        GetGfxFunc().unwrap_unchecked()();
    }
}
pub(crate) unsafe fn LoadGfx() {
    match (*crate::gl!(sGfx)).state {
        0 => {
            InitBgs();
            (*crate::gl!(sGfx)).state += 1;
        }
        1 => {
            if LoadBgGfx() == TRUE as u32 {
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        2 => {
            CopyToBgTilemapBuffer(
                BG_SCENERY,
                sBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                BG_TREE_LEFT,
                sTreeBorderLeft_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                BG_TREE_RIGHT,
                sTreeBorderRight_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(BG_SCENERY);
            CopyBgTilemapBufferToVram(BG_TREE_LEFT);
            CopyBgTilemapBufferToVram(BG_TREE_RIGHT);
            (*crate::gl!(sGfx)).state += 1;
        }
        3 => {
            ShowBg(BG_INTERFACE);
            ShowBg(BG_SCENERY);
            ShowBg(BG_TREE_LEFT);
            ShowBg(BG_TREE_RIGHT);
            (*crate::gl!(sGfx)).state += 1;
        }
        4 => {
            LoadWindowFrameGfx((*crate::gl!(gSaveBlock2Ptr)).optionsWindowFrameType() as u8);
            LoadUserWindowFrameGfx();
            (*crate::gl!(sGfx)).state += 1;
        }
        _ => {
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn ShowNames() {
    let mut numPlayers: u8 = 0;
    let mut playerId: u8 = 0;
    let mut colorsId: u8 = 0;
    let mut name: *mut u8 = null_mut();
    let mut left: u32 = 0;
    let mut window: WindowTemplate = zeroed();
    let mut coords: *mut WinCoords = null_mut();
    match (*crate::gl!(sGfx)).state {
        0 => {
            numPlayers = GetNumPlayers();
            coords = sNameWindowCoords[numPlayers as i32 - 1];
            window.bg = BG_INTERFACE;
            window.width = 7;
            window.height = 2;
            window.paletteNum = 13;
            window.baseBlock = 0x13;
            for i in 0..numPlayers {
                colorsId = COLORID_GRAY;
                playerId = GetPlayerIdByPos(i);
                left = (56 - GetStringWidth(FONT_NORMAL, GetPlayerName(playerId), -1) as u32) / 2;
                window.tilemapLeft = (*coords).left;
                window.tilemapTop = (*coords).top;
                (*crate::gl!(sGfx)).windowIds[i] = AddWindow(&raw mut window) as u8;
                ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[i]);
                FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[i], 17);
                if playerId == GetMultiplayerId() {
                    colorsId = COLORID_BLUE;
                }
                name = GetPlayerName(playerId);
                AddTextPrinterParameterized3(
                    (*crate::gl!(sGfx)).windowIds[i],
                    FONT_NORMAL,
                    left as u8,
                    1,
                    sTextColorTable[colorsId].as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    name,
                );
                CopyWindowToVram((*crate::gl!(sGfx)).windowIds[i], COPYWIN_GFX);
                window.baseBlock += 0xE;
                DrawMessageWindow(&raw mut window);
                coords = coords.at(1);
            }
            (*crate::gl!(sGfx)).state += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                numPlayers = GetNumPlayers();
                for i in 0..numPlayers {
                    PutWindowTilemap((*crate::gl!(sGfx)).windowIds[i]);
                }
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        _ => {
            if ({
                (*crate::gl!(sGfx)).state += 1;
                (*crate::gl!(sGfx)).state
            }) > 180
            {
                numPlayers = GetNumPlayers();
                for i in 0..numPlayers {
                    ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[i]);
                    RemoveWindow((*crate::gl!(sGfx)).windowIds[i]);
                }
                FillBgTilemapBufferRect_Palette0(
                    BG_INTERFACE,
                    0,
                    0,
                    0,
                    DISPLAY_TILE_WIDTH,
                    DISPLAY_TILE_HEIGHT,
                );
                CopyBgTilemapBufferToVram(BG_INTERFACE);
                (*crate::gl!(sGfx)).finished = TRUE as u32;
            }
        }
    }
}
unsafe fn PrintRankedScores(numPlayers_: u8) {
    let mut ranking: u8 = 0;
    let mut rankedPlayers: u8 = 0;
    let numPlayers: u8 = numPlayers_;
    let mut name: *mut u8 = null_mut();
    let mut numWidth: u32 = 0;
    let mut numString: CArray<u8, 32> = zeroed();
    let mut playersByRanking: CArray<u8, 5> = CArray([0, 1, 2, 3, 4]);
    let mut temp: DodrioGame_ScoreResults = zeroed();
    let mut scoreResults: CArray<DodrioGame_ScoreResults, 5> = zeroed();
    let mut i: u8 = 0;
    while i < numPlayers {
        playersByRanking[i] = i;
        GetScoreResults(&raw mut temp, i);
        scoreResults[i] = temp;
        i += 1;
    }
    if GetHighestScore() != 0 {
        loop {
            for i in 0..numPlayers {
                if scoreResults[i].ranking == ranking {
                    playersByRanking[rankedPlayers] = i;
                    rankedPlayers += 1;
                }
            }
            ranking = rankedPlayers;
            if rankedPlayers >= numPlayers {
                break;
            }
        }
    }
    i = 0;
    while i < numPlayers {
        if scoreResults[i].score == 0 {
            scoreResults[i].ranking = numPlayers - 1;
        }
        i += 1;
    }
    let x: u32 = 216
        - GetStringWidth(
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_SpacePoints).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
        ) as u32;
    for i in 0..numPlayers {
        let mut colorsId: u8 = COLORID_GRAY;
        let playerId: u8 = playersByRanking[i];
        let points: u32 = scoreResults[playerId].score;
        AddTextPrinterParameterized(
            (*crate::gl!(sGfx)).windowIds[1],
            FONT_NORMAL,
            sRankingTexts[scoreResults[playerId].ranking],
            8,
            sRankingYCoords[i] as u8,
            TEXT_SKIP_DRAW,
            None,
        );
        if playerId == GetMultiplayerId() {
            colorsId = COLORID_BLUE;
        }
        name = GetPlayerName(playerId);
        AddTextPrinterParameterized3(
            (*crate::gl!(sGfx)).windowIds[1],
            FONT_NORMAL,
            28,
            sRankingYCoords[i] as u8,
            sTextColorTable[colorsId].as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            name,
        );
        ConvertIntToDecimalStringN(
            numString.as_mut_ptr(),
            points as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            7,
        );
        numWidth = GetStringWidth(FONT_NORMAL, numString.as_mut_ptr(), -1) as u32;
        AddTextPrinterParameterized(
            (*crate::gl!(sGfx)).windowIds[1],
            FONT_NORMAL,
            numString.as_mut_ptr(),
            x as u8 - numWidth as u8,
            sRankingYCoords[i] as u8,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            (*crate::gl!(sGfx)).windowIds[1],
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_SpacePoints).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            x as u8,
            sRankingYCoords[i] as u8,
            TEXT_SKIP_DRAW,
            None,
        );
    }
}
pub(crate) unsafe fn ShowResults() {
    let mut prizeState: u8 = 0;
    let numPlayers: u8 = GetNumPlayers();
    let mut name: *mut u8 = null_mut();
    let mut strWidth: u32 = 0;
    let mut x: u32 = 0;
    match (*crate::gl!(sGfx)).state {
        0 => {
            SetScoreResults();
            (*crate::gl!(sGfx)).timer = 0;
            (*crate::gl!(sGfx)).state += 1;
        }
        1 => {
            (*crate::gl!(sGfx)).windowIds[0] =
                AddWindow((&raw const sWindowTemplates_Results[0]).cast_mut()) as u8;
            (*crate::gl!(sGfx)).windowIds[1] =
                AddWindow((&raw const sWindowTemplates_Results[1]).cast_mut()) as u8;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            DrawMessageWindow((&raw const sWindowTemplates_Results[0]).cast_mut());
            DrawMessageWindow((&raw const sWindowTemplates_Results[1]).cast_mut());
            (*crate::gl!(sGfx)).state += 1;
        }
        2 => {
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[1], 17);
            strWidth = GetStringWidth(
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_BerryPickingResults)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                -1,
            ) as u32;
            x = (224 - strWidth) / 2;
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_BerryPickingResults)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                x as u8,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_10P30P50P50P).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                68,
                17,
                TEXT_SKIP_DRAW,
                None,
            );
            for i in 0..numPlayers {
                let mut colorsId: u8 = COLORID_GRAY;
                if i == GetMultiplayerId() {
                    colorsId = COLORID_BLUE;
                }
                name = GetPlayerName(i);
                AddTextPrinterParameterized3(
                    (*crate::gl!(sGfx)).windowIds[1],
                    FONT_NORMAL,
                    0,
                    sResultsYCoords[i] as u8,
                    sTextColorTable[colorsId].as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    name,
                );
                for j in 0..4u8 {
                    let berriesPicked: u16 = Min(GetBerryResult(i, j) as u32, MAX_BERRIES) as u16;
                    let maxBerriesPicked: u16 = Min(GetHighestBerryResult(j), MAX_BERRIES) as u16;
                    ConvertIntToDecimalStringN(
                        gStringVar4.as_mut_ptr(),
                        berriesPicked as i32,
                        STR_CONV_MODE_LEFT_ALIGN,
                        4,
                    );
                    let width: u32 =
                        GetStringWidth(FONT_NORMAL, gStringVar4.as_mut_ptr(), -1) as u32;
                    if maxBerriesPicked == berriesPicked && maxBerriesPicked != 0 {
                        AddTextPrinterParameterized3(
                            (*crate::gl!(sGfx)).windowIds[1],
                            FONT_NORMAL,
                            sResultsXCoords[j] as u8 - width as u8,
                            sResultsYCoords[i] as u8,
                            sTextColorTable[1].as_ptr().cast_mut(),
                            TEXT_SKIP_DRAW as i8,
                            gStringVar4.as_mut_ptr(),
                        );
                    } else {
                        AddTextPrinterParameterized(
                            (*crate::gl!(sGfx)).windowIds[1],
                            FONT_NORMAL,
                            gStringVar4.as_mut_ptr(),
                            sResultsXCoords[j] as u8 - width as u8,
                            sResultsYCoords[i] as u8,
                            TEXT_SKIP_DRAW,
                            None,
                        );
                    }
                }
            }
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[1], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            SetBerryIconsInvisibility(FALSE);
            (*crate::gl!(sGfx)).state += 1;
        }
        4 => {
            if ({
                (*crate::gl!(sGfx)).timer += 1;
                (*crate::gl!(sGfx)).timer
            }) >= 30
                && crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
            {
                (*crate::gl!(sGfx)).timer = 0;
                PlaySE(SE_SELECT);
                SetBerryIconsInvisibility(TRUE);
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        5 => {
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[1], 17);
            strWidth = GetStringWidth(
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_AnnouncingRankings)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                -1,
            ) as u32;
            x = (224 - strWidth) / 2;
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_AnnouncingRankings)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                x as u8,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            (*crate::gl!(sGfx)).state += 1;
        }
        6 => {
            PrintRankedScores(numPlayers);
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[1], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        7 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).state += 1;
        }
        8 => {
            if ({
                (*crate::gl!(sGfx)).timer += 1;
                (*crate::gl!(sGfx)).timer
            }) >= 30
                && crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
            {
                (*crate::gl!(sGfx)).timer = 0;
                PlaySE(SE_SELECT);
                if GetHighestScore() < PRIZE_SCORE {
                    (*crate::gl!(sGfx)).state = 127;
                } else {
                    StopMapMusic();
                    (*crate::gl!(sGfx)).state += 1;
                }
                FillBgTilemapBufferRect_Palette0(BG_INTERFACE, 0, 0, 5, DISPLAY_TILE_WIDTH, 15);
                RemoveWindow((*crate::gl!(sGfx)).windowIds[1]);
                (*crate::gl!(sGfx)).windowIds[1] =
                    AddWindow((&raw const *sWindowTemplate_Prize).cast_mut()) as u8;
                ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
                DrawMessageWindow((&raw const *sWindowTemplate_Prize).cast_mut());
            }
        }
        9 => {
            PlayNewMapMusic(MUS_LEVEL_UP);
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[1], 17);
            strWidth = GetStringWidth(
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_AnnouncingPrizes)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                -1,
            ) as u32;
            x = (224 - strWidth) / 2;
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_AnnouncingPrizes)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                x as u8,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            DynamicPlaceholderTextUtil_Reset();
            CopyItemName(GetPrizeItemId(), gStringVar1.as_mut_ptr());
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_FirstPlacePrize).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            prizeState = TryGivePrize();
            if prizeState != PRIZE_RECEIVED && prizeState != NO_PRIZE {
                DynamicPlaceholderTextUtil_Reset();
                CopyItemName(GetPrizeItemId(), gStringVar1.as_mut_ptr());
                DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gStringVar1.as_mut_ptr());
                if prizeState == PRIZE_NO_ROOM {
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::strings::gText_CantHoldAnyMore)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                } else if prizeState == PRIZE_FILLED_BAG {
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::strings::gText_FilledStorageSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                AddTextPrinterParameterized(
                    (*crate::gl!(sGfx)).windowIds[1],
                    FONT_NORMAL,
                    gStringVar4.as_mut_ptr(),
                    0,
                    41,
                    TEXT_SKIP_DRAW,
                    None,
                );
            }
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[1], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        10 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            FadeOutAndFadeInNewMapMusic(MUS_RG_VICTORY_WILD, 20, 10);
            (*crate::gl!(sGfx)).state += 1;
        }
        11 => {
            if ({
                (*crate::gl!(sGfx)).timer += 1;
                (*crate::gl!(sGfx)).timer
            }) >= 30
                && crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
            {
                (*crate::gl!(sGfx)).timer = 0;
                PlaySE(SE_SELECT);
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        _ => {
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            RemoveWindow((*crate::gl!(sGfx)).windowIds[0]);
            RemoveWindow((*crate::gl!(sGfx)).windowIds[1]);
            FillBgTilemapBufferRect_Palette0(
                BG_INTERFACE,
                0,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn Msg_WantToPlayAgain() {
    let mut y: u8 = 0;
    match (*crate::gl!(sGfx)).state {
        0 => {
            (*crate::gl!(sGfx)).windowIds[0] =
                AddWindow((&raw const sWindowTemplates_PlayAgain[0]).cast_mut()) as u8;
            (*crate::gl!(sGfx)).windowIds[1] =
                AddWindow((&raw const sWindowTemplates_PlayAgain[1]).cast_mut()) as u8;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            DrawMessageWindow((&raw const sWindowTemplates_PlayAgain[0]).cast_mut());
            DrawYesNoMessageWindow((&raw const sWindowTemplates_PlayAgain[1]).cast_mut());
            (*crate::gl!(sGfx)).state += 1;
            (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_NONE;
            (*crate::gl!(sGfx)).playAgainState = PLAY_AGAIN_NONE;
        }
        1 => {
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[1], 17);
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_WantToPlayAgain).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                5,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_Yes).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_No).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                17,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[1], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).state += 1;
        }
        3 => {
            y = (*crate::gl!(sGfx)).cursorSelection;
            if y == PLAY_AGAIN_NONE {
                y = PLAY_AGAIN_YES;
            }
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[1], 17);
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_Yes).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_No).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                17,
                TEXT_SKIP_DRAW,
                None,
            );
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[1],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                (y - 1) * 16 + 1,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[1], COPYWIN_FULL);
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*crate::gl!(sGfx)).cursorSelection == PLAY_AGAIN_NONE {
                    (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_YES;
                }
                (*crate::gl!(sGfx)).state += 1;
            } else if crate::gl!(gMain.newKeys) as i32 & 192 != 0 {
                PlaySE(SE_SELECT);
                match (*crate::gl!(sGfx)).cursorSelection {
                    PLAY_AGAIN_NONE => {
                        (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_NO;
                    }
                    PLAY_AGAIN_YES => {
                        (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_NO;
                    }
                    PLAY_AGAIN_NO => {
                        (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_YES;
                    }
                    _ => {}
                }
            } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*crate::gl!(sGfx)).cursorSelection = PLAY_AGAIN_NO;
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        _ => {
            (*crate::gl!(sGfx)).playAgainState = (*crate::gl!(sGfx)).cursorSelection;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[1]);
            RemoveWindow((*crate::gl!(sGfx)).windowIds[0]);
            RemoveWindow((*crate::gl!(sGfx)).windowIds[1]);
            FillBgTilemapBufferRect_Palette0(
                BG_INTERFACE,
                0,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn Msg_SavingDontTurnOff() {
    match (*crate::gl!(sGfx)).state {
        0 => {
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
            (*crate::gl!(sGfx)).state += 1;
        }
        1 => {
            CopyWindowToVram(0, COPYWIN_FULL);
            (*crate::gl!(sGfx)).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateTask(Some(Task_LinkFullSave), 0);
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        3 => {
            if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        _ => {
            FillBgTilemapBufferRect_Palette0(
                BG_INTERFACE,
                0,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn Msg_CommunicationStandby() {
    match (*crate::gl!(sGfx)).state {
        0 => {
            (*crate::gl!(sGfx)).windowIds[0] =
                AddWindow((&raw const *sWindowTemplate_CommStandby).cast_mut()) as u8;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            DrawMessageWindow((&raw const *sWindowTemplate_CommStandby).cast_mut());
            (*crate::gl!(sGfx)).state += 1;
        }
        1 => {
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_CommunicationStandby3)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                5,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).state += 1;
        }
        _ => {
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn EraseMessage() {
    ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
    RemoveWindow((*crate::gl!(sGfx)).windowIds[0]);
    FillBgTilemapBufferRect_Palette0(
        BG_INTERFACE,
        0,
        0,
        0,
        DISPLAY_TILE_WIDTH,
        DISPLAY_TILE_HEIGHT,
    );
    CopyBgTilemapBufferToVram(BG_INTERFACE);
    (*crate::gl!(sGfx)).finished = TRUE as u32;
}
pub(crate) unsafe fn Msg_SomeoneDroppedOut() {
    match (*crate::gl!(sGfx)).state {
        0 => {
            (*crate::gl!(sGfx)).windowIds[0] =
                AddWindow((&raw const *sWindowTemplate_DroppedOut).cast_mut()) as u8;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            DrawMessageWindow((&raw const *sWindowTemplate_DroppedOut).cast_mut());
            (*crate::gl!(sGfx)).state += 1;
            (*crate::gl!(sGfx)).timer = 0;
            (*crate::gl!(sGfx)).cursorSelection = 0;
            (*crate::gl!(sGfx)).playAgainState = PLAY_AGAIN_NONE;
        }
        1 => {
            FillWindowPixelBuffer((*crate::gl!(sGfx)).windowIds[0], 17);
            AddTextPrinterParameterized(
                (*crate::gl!(sGfx)).windowIds[0],
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_SomeoneDroppedOut)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                5,
                TEXT_SKIP_DRAW,
                None,
            );
            CopyWindowToVram((*crate::gl!(sGfx)).windowIds[0], COPYWIN_GFX);
            (*crate::gl!(sGfx)).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PutWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            }
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).state += 1;
        }
        3 => {
            if ({
                (*crate::gl!(sGfx)).timer += 1;
                (*crate::gl!(sGfx)).timer
            }) >= 120
            {
                (*crate::gl!(sGfx)).state += 1;
            }
        }
        _ => {
            (*crate::gl!(sGfx)).playAgainState = PLAY_AGAIN_DROPPED;
            ClearWindowTilemap((*crate::gl!(sGfx)).windowIds[0]);
            RemoveWindow((*crate::gl!(sGfx)).windowIds[0]);
            FillBgTilemapBufferRect_Palette0(
                BG_INTERFACE,
                0,
                0,
                0,
                DISPLAY_TILE_WIDTH,
                DISPLAY_TILE_HEIGHT,
            );
            CopyBgTilemapBufferToVram(BG_INTERFACE);
            (*crate::gl!(sGfx)).finished = TRUE as u32;
        }
    }
}
pub(crate) unsafe fn StopGfxFuncs() {
    DestroyTask((*crate::gl!(sGfx)).taskId);
    (*crate::gl!(sGfx)).finished = TRUE as u32;
}
pub(crate) fn GfxIdle() {}
unsafe fn SetGfxFunc(func: Option<unsafe fn()>) {
    (*crate::gl!(sGfx)).state = 0;
    (*crate::gl!(sGfx)).finished = FALSE as u32;
    (*crate::gl!(sGfx)).func = func;
}
unsafe fn GetGfxFunc() -> Option<unsafe fn()> {
    (*crate::gl!(sGfx)).func
}
unsafe fn IsGfxFuncActive() -> u32 {
    if (*crate::gl!(sGfx)).finished == TRUE as u32 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetPlayAgainState() -> u8 {
    (*crate::gl!(sGfx)).playAgainState
}
unsafe fn InitBgs() {
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
    SetGpuReg(0x0, 0);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    InitStandardTextBoxWindows();
    InitTextBoxGfxAndPrinters();
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetBgTilemapBuffer(
        BG_SCENERY,
        (*crate::gl!(sGfx)).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        BG_TREE_LEFT,
        (*crate::gl!(sGfx)).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        BG_TREE_RIGHT,
        (*crate::gl!(sGfx)).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
}
unsafe fn LoadBgGfx() -> u32 {
    match (*crate::gl!(sGfx)).loadState {
        0 => {
            LoadPalette(sBg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 64);
        }
        1 => {
            ResetTempTileDataBuffers();
        }
        2 => {
            DecompressAndCopyTileDataToVram(
                BG_SCENERY,
                sBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        3 => {
            DecompressAndCopyTileDataToVram(
                BG_TREE_LEFT,
                sTreeBorder_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() == TRUE {
                return FALSE as u32;
            }
        }
        5 => {
            LoadPalette(GetTextWindowPalette(3) as *mut c_void, 208, 32);
        }
        _ => {
            (*crate::gl!(sGfx)).loadState = 0;
            return TRUE as u32;
        }
    }
    (*crate::gl!(sGfx)).loadState += 1;
    FALSE as u32
}
