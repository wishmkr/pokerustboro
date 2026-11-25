//! Translated from `src/mail.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::CopyToBgTilemapBuffer;
use crate::bg::{
    CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags,
    ShowBg, UnsetBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::easy_chat::{ConvertEasyChatWordsToString, CopyEasyChatWord};
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::ConvertInternationalPlayerName;
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::load_save::gSaveBlock2Ptr;
use crate::mail_data::MailSpeciesToSpecies;
use crate::menu::{
    AddTextPrinterParameterized3, DecompressAndCopyTileDataToVram,
    FreeTempTileDataBuffersIfPossible, ResetTempTileDataBuffers,
};
use crate::menu_helpers::MenuHelpers_IsLinkActive;
use crate::overworld::Overworld_IsRecvQueueAtMax;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon_icon::{
    CreateMonIconNoPersonality, FreeAndDestroyMonIconSprite, FreeMonIconPalette,
    GetIconSpeciesNoPersonality, LoadMonIconPalette,
};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{StringCopy, StringLength};
use crate::task::ResetTasks;
use crate::text::{DeactivateAllTextPrinters, RunTextPrinters};
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
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
// Data tables (translate with cdata.py): sBgTemplates sWindowTemplates sTextColors sBgColors sMailGraphics sLineLayouts_Wide sMailLayouts_Wide sLineLayouts_Tall sMailLayouts_Tall

/// `struct MailRead`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MailRead {
    pub message: CArray<CArray<u8, 64>, 8>,
    pub playerName: CArray<u8, 12>,
    pub exitCallback: Option<unsafe fn()>,
    pub callback: Option<unsafe fn()>,
    pub mail: *mut Mail,
    pub hasText: u8,
    pub signatureWidth: u8,
    pub mailType: u8,
    pub iconType: u8,
    pub monIconSpriteId: u8,
    pub language: u8,
    pub international: u8,
    pub parserSingle: Option<unsafe fn(*mut u8, u16) -> *mut u8>,
    pub parserMultiple: Option<unsafe fn(*mut u8, *mut u16, u16, u16) -> *mut u8>,
    pub layout: *mut MailLayout,
    pub bg1TilemapBuffer: CArray<u8, 4096>,
    pub bg2TilemapBuffer: CArray<u8, 4096>,
}

unsafe impl Sync for MailRead {}

/// `struct MailLayout`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MailLayout {
    pub numLines: u8,
    pub signatureYPos: u8,
    pub signatureWidth: u8,
    pub wordsYPos: u8,
    pub wordsXPos: u8,
    pub lines: *mut MailLineLayout,
}

unsafe impl Sync for MailLayout {}

/// `struct MailGraphics`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MailGraphics {
    pub palette: *mut u16,
    pub tiles: *mut u32,
    pub tileMap: *mut u32,
    pub unused: u32,
    pub textColor: u16,
    pub textShadow: u16,
}

unsafe impl Sync for MailGraphics {}

/// `struct MailLineLayout`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MailLineLayout {
    bits_0: u8,
    pub height: u8,
}

impl MailLineLayout {
    #[inline(always)]
    pub fn numEasyChatWords(&self) -> u8 {
        ((self.bits_0 as u32) & 0x3) as u8
    }
    #[inline(always)]
    pub fn set_numEasyChatWords(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !0x3) | (v & 0x3);
    }
    #[inline(always)]
    pub fn xOffset(&self) -> u8 {
        ((self.bits_0 as u32 >> 2) & 0x3f) as u8
    }
    #[inline(always)]
    pub fn set_xOffset(&mut self, v: u8) {
        self.bits_0 = (self.bits_0 & !(0x3f << 2)) | ((v & 0x3f) << 2);
    }
}

unsafe impl Sync for MailLineLayout {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MailRead>() == 8748);
    assert!(offset_of!(MailRead, message) == 0);
    assert!(offset_of!(MailRead, playerName) == 512);
    assert!(offset_of!(MailRead, exitCallback) == 524);
    assert!(offset_of!(MailRead, callback) == 528);
    assert!(offset_of!(MailRead, mail) == 532);
    assert!(offset_of!(MailRead, hasText) == 536);
    assert!(offset_of!(MailRead, signatureWidth) == 537);
    assert!(offset_of!(MailRead, mailType) == 538);
    assert!(offset_of!(MailRead, iconType) == 539);
    assert!(offset_of!(MailRead, monIconSpriteId) == 540);
    assert!(offset_of!(MailRead, language) == 541);
    assert!(offset_of!(MailRead, international) == 542);
    assert!(offset_of!(MailRead, parserSingle) == 544);
    assert!(offset_of!(MailRead, parserMultiple) == 548);
    assert!(offset_of!(MailRead, layout) == 552);
    assert!(offset_of!(MailRead, bg1TilemapBuffer) == 556);
    assert!(offset_of!(MailRead, bg2TilemapBuffer) == 4652);
    assert!(size_of::<MailLayout>() == 12);
    assert!(offset_of!(MailLayout, numLines) == 0);
    assert!(offset_of!(MailLayout, signatureYPos) == 1);
    assert!(offset_of!(MailLayout, signatureWidth) == 2);
    assert!(offset_of!(MailLayout, wordsYPos) == 3);
    assert!(offset_of!(MailLayout, wordsXPos) == 4);
    assert!(offset_of!(MailLayout, lines) == 8);
    assert!(size_of::<MailGraphics>() == 20);
    assert!(offset_of!(MailGraphics, palette) == 0);
    assert!(offset_of!(MailGraphics, tiles) == 4);
    assert!(offset_of!(MailGraphics, tileMap) == 8);
    assert!(offset_of!(MailGraphics, unused) == 12);
    assert!(offset_of!(MailGraphics, textColor) == 16);
    assert!(offset_of!(MailGraphics, textShadow) == 18);
    assert!(size_of::<MailLineLayout>() == 4);
    assert!(offset_of!(MailLineLayout, bits_0) == 0);
    assert!(offset_of!(MailLineLayout, height) == 1);
};

const ICON_TYPE_BEAD: u8 = 1;
const ICON_TYPE_DREAM: u8 = 2;
const ICON_TYPE_NONE: u8 = 0;

static sBgColors: Table<CArray<CArray<u16, 2>, 2>> =
    Table((&raw const crate::data::mail::sBgColors).cast());
static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::mail::sBgTemplates).cast());
static sMailGraphics: Table<CArray<MailGraphics, 12>> =
    Table((&raw const crate::data::mail::sMailGraphics).cast());
static sMailLayouts_Tall: Table<CArray<MailLayout, 12>> =
    Table((&raw const crate::data::mail::sMailLayouts_Tall).cast());
static sMailLayouts_Wide: Table<CArray<MailLayout, 12>> =
    Table((&raw const crate::data::mail::sMailLayouts_Wide).cast());
static sTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::mail::sTextColors).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::mail::sWindowTemplates).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMailRead: crate::global::Global<*mut MailRead> =
    crate::global::Global::new(null_mut());

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetOverworldTextboxPalettePtr` with this module's view of its types.
#[inline]
fn GetOverworldTextboxPalettePtr() -> *mut u16 {
    unsafe { crate::text_window::GetOverworldTextboxPalettePtr() as *mut u16 }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn ReadMail(mail: *mut Mail, exitCallback: Option<unsafe fn()>, mut hasText: u8) {
    let mut buffer: CArray<u16, 2> = zeroed();
    let mut species: u16 = 0;
    crate::gl!(sMailRead = AllocZeroed(8748) as *mut MailRead);
    (*crate::gl!(sMailRead)).language = GAME_LANGUAGE;
    (*crate::gl!(sMailRead)).international = TRUE;
    (*crate::gl!(sMailRead)).parserSingle = Some(CopyEasyChatWord);
    (*crate::gl!(sMailRead)).parserMultiple = Some(ConvertEasyChatWordsToString);
    if (*mail).itemId == ITEM_ORANGE_MAIL
        || (*mail).itemId == ITEM_HARBOR_MAIL
        || (*mail).itemId == ITEM_GLITTER_MAIL
        || (*mail).itemId == ITEM_MECH_MAIL
        || (*mail).itemId == ITEM_WOOD_MAIL
        || (*mail).itemId == ITEM_WAVE_MAIL
        || (*mail).itemId == ITEM_BEAD_MAIL
        || (*mail).itemId == ITEM_SHADOW_MAIL
        || (*mail).itemId == ITEM_TROPIC_MAIL
        || (*mail).itemId == ITEM_DREAM_MAIL
        || (*mail).itemId == ITEM_FAB_MAIL
        || (*mail).itemId == ITEM_RETRO_MAIL
    {
        (*crate::gl!(sMailRead)).mailType = (*mail).itemId as u8 - ITEM_ORANGE_MAIL as u8;
    } else {
        (*crate::gl!(sMailRead)).mailType = 0;
        hasText = FALSE;
    }
    match (*crate::gl!(sMailRead)).international {
        TRUE => {
            (*crate::gl!(sMailRead)).layout =
                (&raw const sMailLayouts_Tall[(*crate::gl!(sMailRead)).mailType]).cast_mut();
        }
        _ => {
            (*crate::gl!(sMailRead)).layout =
                (&raw const sMailLayouts_Wide[(*crate::gl!(sMailRead)).mailType]).cast_mut();
        }
    }
    species = MailSpeciesToSpecies((*mail).species, buffer.as_mut_ptr());
    if species > SPECIES_NONE && species < NUM_SPECIES {
        match (*crate::gl!(sMailRead)).mailType {
            6 => {
                (*crate::gl!(sMailRead)).iconType = ICON_TYPE_BEAD;
            }
            9 => {
                (*crate::gl!(sMailRead)).iconType = ICON_TYPE_DREAM;
            }
            _ => {
                (*crate::gl!(sMailRead)).iconType = ICON_TYPE_NONE;
            }
        }
    } else {
        (*crate::gl!(sMailRead)).iconType = ICON_TYPE_NONE;
    }
    (*crate::gl!(sMailRead)).mail = mail;
    (*crate::gl!(sMailRead)).exitCallback = exitCallback;
    (*crate::gl!(sMailRead)).hasText = hasText;
    SetMainCallback2(Some(CB2_InitMailRead));
}
unsafe fn MailReadBuildGraphics() -> u8 {
    let mut icon: u16 = 0;
    match crate::gl!(gMain.state) {
        0 => {
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            SetGpuReg(0x0, 0);
        }
        1 => {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                OAM as i32 as usize as *mut c_void,
                0x1000200,
            );
        }
        2 => {
            ResetPaletteFade();
        }
        3 => {
            ResetTasks();
        }
        4 => {
            ResetSpriteData();
        }
        5 => {
            FreeAllSpritePalettes();
            ResetTempTileDataBuffers();
            SetGpuReg(REG_OFFSET_BG0HOFS, 0);
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            SetGpuReg(REG_OFFSET_BG1HOFS, 0);
            SetGpuReg(REG_OFFSET_BG1VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2HOFS, 0);
            SetGpuReg(REG_OFFSET_BG3HOFS, 0);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        }
        6 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 3);
            SetBgTilemapBuffer(
                1,
                (*crate::gl!(sMailRead)).bg1TilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                2,
                (*crate::gl!(sMailRead)).bg2TilemapBuffer.as_mut_ptr() as *mut c_void,
            );
        }
        7 => {
            InitWindows(sWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
        }
        8 => {
            DecompressAndCopyTileDataToVram(
                1,
                sMailGraphics[(*crate::gl!(sMailRead)).mailType].tiles as *mut c_void,
                0,
                0,
                0,
            );
        }
        9 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return FALSE;
            }
        }
        10 => {
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            FillBgTilemapBufferRect_Palette0(2, 1, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            CopyToBgTilemapBuffer(
                1,
                sMailGraphics[(*crate::gl!(sMailRead)).mailType].tileMap as *mut c_void,
                0,
                0,
            );
        }
        11 => {
            CopyBgTilemapBufferToVram(0);
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(2);
        }
        12 => {
            LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 240, 32);
            crate::gl!(
                gPlttBufferUnfaded[250] =
                    sMailGraphics[(*crate::gl!(sMailRead)).mailType].textColor
            );
            crate::gl!(
                gPlttBufferFaded[250] = sMailGraphics[(*crate::gl!(sMailRead)).mailType].textColor
            );
            crate::gl!(
                gPlttBufferUnfaded[251] =
                    sMailGraphics[(*crate::gl!(sMailRead)).mailType].textShadow
            );
            crate::gl!(
                gPlttBufferFaded[251] = sMailGraphics[(*crate::gl!(sMailRead)).mailType].textShadow
            );
            LoadPalette(
                sMailGraphics[(*crate::gl!(sMailRead)).mailType].palette as *mut c_void,
                0,
                32,
            );
            crate::gl!(
                gPlttBufferUnfaded[10] = sBgColors[(*crate::gl!(gSaveBlock2Ptr)).playerGender][0]
            );
            crate::gl!(
                gPlttBufferFaded[10] = sBgColors[(*crate::gl!(gSaveBlock2Ptr)).playerGender][0]
            );
            crate::gl!(
                gPlttBufferUnfaded[11] = sBgColors[(*crate::gl!(gSaveBlock2Ptr)).playerGender][1]
            );
            crate::gl!(
                gPlttBufferFaded[11] = sBgColors[(*crate::gl!(gSaveBlock2Ptr)).playerGender][1]
            );
        }
        13 => {
            if (*crate::gl!(sMailRead)).hasText != 0 {
                BufferMailText();
            }
        }
        14 => {
            if (*crate::gl!(sMailRead)).hasText != 0 {
                PrintMailText();
                RunTextPrinters();
            }
        }
        15 => {
            if Overworld_IsRecvQueueAtMax() == TRUE as u32 {
                return FALSE;
            }
        }
        16 => {
            SetVBlankCallback(Some(VBlankCB_MailRead));
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
        }
        17 => {
            icon = GetIconSpeciesNoPersonality((*(*crate::gl!(sMailRead)).mail).species);
            match (*crate::gl!(sMailRead)).iconType {
                ICON_TYPE_BEAD => {
                    LoadMonIconPalette(icon);
                    (*crate::gl!(sMailRead)).monIconSpriteId =
                        CreateMonIconNoPersonality(icon, Some(SpriteCallbackDummy), 96, 128, 0, 0);
                }
                ICON_TYPE_DREAM => {
                    LoadMonIconPalette(icon);
                    (*crate::gl!(sMailRead)).monIconSpriteId =
                        CreateMonIconNoPersonality(icon, Some(SpriteCallbackDummy), 40, 128, 0, 0);
                }
                _ => {}
            }
        }
        18 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            (*crate::gl!(sMailRead)).callback = Some(CB2_WaitForPaletteExitOnKeyPress);
            return TRUE;
        }
        _ => {
            return FALSE;
        }
    }
    crate::gl!(gMain.state += 1);
    FALSE
}
pub(crate) unsafe fn CB2_InitMailRead() {
    loop {
        if MailReadBuildGraphics() == TRUE {
            SetMainCallback2(Some(CB2_MailRead));
            break;
        }
        if MenuHelpers_IsLinkActive() == TRUE {
            break;
        }
    }
}
unsafe fn BufferMailText() {
    let mut numWords: u8 = 0;
    let mut i: u16 = 0;
    while i < (*(*crate::gl!(sMailRead)).layout).numLines as u16 {
        ConvertEasyChatWordsToString(
            (*crate::gl!(sMailRead)).message[i].as_mut_ptr(),
            &raw mut (*(*crate::gl!(sMailRead)).mail).words[numWords],
            (*(*(*crate::gl!(sMailRead)).layout).lines.at(i)).numEasyChatWords() as u16,
            1,
        );
        numWords += (*(*(*crate::gl!(sMailRead)).layout).lines.at(i)).numEasyChatWords();
        i += 1;
    }
    let ptr: *mut u8 = StringCopy(
        (*crate::gl!(sMailRead)).playerName.as_mut_ptr(),
        (*(*crate::gl!(sMailRead)).mail).playerName.as_mut_ptr(),
    );
    if (*crate::gl!(sMailRead)).international == 0 {
        StringCopy(
            ptr,
            (*(&raw const crate::data::strings::gText_FromSpace).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        (*crate::gl!(sMailRead)).signatureWidth = (*(*crate::gl!(sMailRead)).layout).signatureWidth
            - (StringLength((*crate::gl!(sMailRead)).playerName.as_mut_ptr()) as u8 * 8 - 96);
    } else {
        ConvertInternationalPlayerName((*crate::gl!(sMailRead)).playerName.as_mut_ptr());
        (*crate::gl!(sMailRead)).signatureWidth = (*(*crate::gl!(sMailRead)).layout).signatureWidth;
    }
}
unsafe fn PrintMailText() {
    let mut signature: CArray<u8, 32> = zeroed();
    let mut y: u8 = 0;
    PutWindowTilemap(0);
    PutWindowTilemap(1);
    FillWindowPixelBuffer(0, 0);
    FillWindowPixelBuffer(1, 0);
    let mut i: u16 = 0;
    while i < (*(*crate::gl!(sMailRead)).layout).numLines as u16 {
        'l1: {
            if (*crate::gl!(sMailRead)).message[i][0] == EOS
                || (*crate::gl!(sMailRead)).message[i][0] == 0x00
            {
                break 'l1;
            }
            AddTextPrinterParameterized3(
                0,
                FONT_NORMAL,
                (*(*(*crate::gl!(sMailRead)).layout).lines.at(i)).xOffset()
                    + (*(*crate::gl!(sMailRead)).layout).wordsXPos,
                y + (*(*crate::gl!(sMailRead)).layout).wordsYPos,
                sTextColors.as_ptr().cast_mut(),
                0,
                (*crate::gl!(sMailRead)).message[i].as_mut_ptr(),
            );
            y += (*(*(*crate::gl!(sMailRead)).layout).lines.at(i)).height;
        }
        i += 1;
    }
    let bufptr: *mut u8 = StringCopy(
        signature.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_FromSpace).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringCopy(bufptr, (*crate::gl!(sMailRead)).playerName.as_mut_ptr());
    let box_x: i32 = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        signature.as_mut_ptr(),
        (*crate::gl!(sMailRead)).signatureWidth as i32,
    ) + 104;
    let box_y: i32 = (*(*crate::gl!(sMailRead)).layout).signatureYPos as i32 + 88;
    AddTextPrinterParameterized3(
        0,
        FONT_NORMAL,
        box_x as u8,
        box_y as u8,
        sTextColors.as_ptr().cast_mut(),
        0,
        signature.as_mut_ptr(),
    );
    CopyWindowToVram(0, COPYWIN_FULL);
    CopyWindowToVram(1, COPYWIN_FULL);
}
pub(crate) unsafe fn VBlankCB_MailRead() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_MailRead() {
    if (*crate::gl!(sMailRead)).iconType != ICON_TYPE_NONE {
        AnimateSprites();
        BuildOamBuffer();
    }
    (*crate::gl!(sMailRead)).callback.unwrap_unchecked()();
}
pub(crate) unsafe fn CB2_WaitForPaletteExitOnKeyPress() {
    if UpdatePaletteFade() == 0 {
        (*crate::gl!(sMailRead)).callback = Some(CB2_ExitOnKeyPress);
    }
}
pub(crate) unsafe fn CB2_ExitOnKeyPress() {
    if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        (*crate::gl!(sMailRead)).callback = Some(CB2_ExitMailReadFreeVars);
    }
}
pub(crate) unsafe fn CB2_ExitMailReadFreeVars() {
    if UpdatePaletteFade() == 0 {
        SetMainCallback2((*crate::gl!(sMailRead)).exitCallback);
        match (*crate::gl!(sMailRead)).iconType {
            ICON_TYPE_BEAD | ICON_TYPE_DREAM => {
                FreeMonIconPalette(GetIconSpeciesNoPersonality(
                    (*(*crate::gl!(sMailRead)).mail).species,
                ));
                FreeAndDestroyMonIconSprite({
                    let gl_i0 = (*crate::gl!(sMailRead)).monIconSpriteId;
                    crate::gl!(&raw gSprites[gl_i0])
                });
            }
            _ => {}
        }
        memset(crate::gl!(sMailRead) as *mut u8, 0, 8748);
        ResetPaletteFade();
        UnsetBgTilemapBuffer(0);
        UnsetBgTilemapBuffer(1);
        ResetBgsAndClearDma3BusyFlags(0);
        FreeAllWindowBuffers();
        Free(crate::gl!(sMailRead) as *mut c_void);
        crate::gl!(sMailRead = null_mut());
    }
}
