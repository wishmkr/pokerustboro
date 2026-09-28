//! Translated from `src/trainer_card.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sTrainerCardStickers_Gfx sUnused_Pal sHoennTrainerCardBronze_Pal sKantoTrainerCardGreen_Pal sHoennTrainerCardCopper_Pal sKantoTrainerCardBronze_Pal sHoennTrainerCardSilver_Pal sKantoTrainerCardSilver_Pal sHoennTrainerCardGold_Pal sKantoTrainerCardGold_Pal sHoennTrainerCardFemaleBg_Pal sKantoTrainerCardFemaleBg_Pal sHoennTrainerCardBadges_Pal sKantoTrainerCardBadges_Pal sTrainerCardStar_Pal sTrainerCardSticker1_Pal sTrainerCardSticker2_Pal sTrainerCardSticker3_Pal sTrainerCardSticker4_Pal sHoennTrainerCardBadges_Gfx sKantoTrainerCardBadges_Gfx sTrainerCardBgTemplates sTrainerCardWindowTemplates sHoennTrainerCardPals sKantoTrainerCardPals sTrainerCardTextColors sTrainerCardStatColors sTimeColonInvisibleTextColors sTrainerPicOffset sTrainerPicFacilityClass sTrainerCardFlipTasks sTimeColonTextColors sText_HofTime sLinkBattleTexts widths.1 xOffsets.2 yOffsets.0 yOffsetsLine1.4 yOffsetsLine2.3

/// `struct TrainerCardData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrainerCardData {
    pub mainState: u8,
    pub printState: u8,
    pub gfxLoadState: u8,
    pub bgPalLoadState: u8,
    pub flipDrawState: u8,
    pub isLink: u8,
    pub timeColonBlinkTimer: u8,
    pub timeColonInvisible: u8,
    pub onBack: u8,
    pub allowDMACopy: u8,
    pub hasPokedex: u8,
    pub hasHofResult: u8,
    pub hasLinkResults: u8,
    pub hasBattleTowerWins: u8,
    pub unused_E: u8,
    pub unused_F: u8,
    pub hasTrades: u8,
    pub badgeCount: CArray<u8, 8>,
    pub easyChatProfile: CArray<CArray<u8, 13>, 4>,
    pub textPlayersCard: CArray<u8, 70>,
    pub textHofTime: CArray<u8, 70>,
    pub textLinkBattleType: CArray<u8, 140>,
    pub textLinkBattleWins: CArray<u8, 70>,
    pub textLinkBattleLosses: CArray<u8, 140>,
    pub textNumTrades: CArray<u8, 140>,
    pub textBerryCrushPts: CArray<u8, 140>,
    pub textUnionRoomStats: CArray<u8, 70>,
    pub textNumLinkPokeblocks: CArray<u8, 70>,
    pub textNumLinkContests: CArray<u8, 70>,
    pub textBattleFacilityStat: CArray<u8, 70>,
    pub monIconPal: CArray<u16, 96>,
    pub flipBlendY: i8,
    pub timeColonNeedDraw: u8,
    pub cardType: u8,
    pub isHoenn: u8,
    pub blendColor: u16,
    pub callback2: Option<unsafe extern "C" fn()>,
    pub trainerCard: TrainerCard,
    pub frontTilemap: CArray<u16, 600>,
    pub backTilemap: CArray<u16, 600>,
    pub bgTilemap: CArray<u16, 600>,
    pub badgeTiles: CArray<u8, 1024>,
    pub stickerTiles: CArray<u8, 512>,
    pub cardTiles: CArray<u8, 8960>,
    pub cardTilemapBuffer: CArray<u16, 4096>,
    pub bgTilemapBuffer: CArray<u16, 4096>,
    pub cardTop: u16,
    pub language: u8,
}

unsafe impl Sync for TrainerCardData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<TrainerCardData>() == 31916);
    assert!(offset_of!(TrainerCardData, mainState) == 0);
    assert!(offset_of!(TrainerCardData, printState) == 1);
    assert!(offset_of!(TrainerCardData, gfxLoadState) == 2);
    assert!(offset_of!(TrainerCardData, bgPalLoadState) == 3);
    assert!(offset_of!(TrainerCardData, flipDrawState) == 4);
    assert!(offset_of!(TrainerCardData, isLink) == 5);
    assert!(offset_of!(TrainerCardData, timeColonBlinkTimer) == 6);
    assert!(offset_of!(TrainerCardData, timeColonInvisible) == 7);
    assert!(offset_of!(TrainerCardData, onBack) == 8);
    assert!(offset_of!(TrainerCardData, allowDMACopy) == 9);
    assert!(offset_of!(TrainerCardData, hasPokedex) == 10);
    assert!(offset_of!(TrainerCardData, hasHofResult) == 11);
    assert!(offset_of!(TrainerCardData, hasLinkResults) == 12);
    assert!(offset_of!(TrainerCardData, hasBattleTowerWins) == 13);
    assert!(offset_of!(TrainerCardData, unused_E) == 14);
    assert!(offset_of!(TrainerCardData, unused_F) == 15);
    assert!(offset_of!(TrainerCardData, hasTrades) == 16);
    assert!(offset_of!(TrainerCardData, badgeCount) == 17);
    assert!(offset_of!(TrainerCardData, easyChatProfile) == 25);
    assert!(offset_of!(TrainerCardData, textPlayersCard) == 77);
    assert!(offset_of!(TrainerCardData, textHofTime) == 147);
    assert!(offset_of!(TrainerCardData, textLinkBattleType) == 217);
    assert!(offset_of!(TrainerCardData, textLinkBattleWins) == 357);
    assert!(offset_of!(TrainerCardData, textLinkBattleLosses) == 427);
    assert!(offset_of!(TrainerCardData, textNumTrades) == 567);
    assert!(offset_of!(TrainerCardData, textBerryCrushPts) == 707);
    assert!(offset_of!(TrainerCardData, textUnionRoomStats) == 847);
    assert!(offset_of!(TrainerCardData, textNumLinkPokeblocks) == 917);
    assert!(offset_of!(TrainerCardData, textNumLinkContests) == 987);
    assert!(offset_of!(TrainerCardData, textBattleFacilityStat) == 1057);
    assert!(offset_of!(TrainerCardData, monIconPal) == 1128);
    assert!(offset_of!(TrainerCardData, flipBlendY) == 1320);
    assert!(offset_of!(TrainerCardData, timeColonNeedDraw) == 1321);
    assert!(offset_of!(TrainerCardData, cardType) == 1322);
    assert!(offset_of!(TrainerCardData, isHoenn) == 1323);
    assert!(offset_of!(TrainerCardData, blendColor) == 1324);
    assert!(offset_of!(TrainerCardData, callback2) == 1328);
    assert!(offset_of!(TrainerCardData, trainerCard) == 1332);
    assert!(offset_of!(TrainerCardData, frontTilemap) == 1432);
    assert!(offset_of!(TrainerCardData, backTilemap) == 2632);
    assert!(offset_of!(TrainerCardData, bgTilemap) == 3832);
    assert!(offset_of!(TrainerCardData, badgeTiles) == 5032);
    assert!(offset_of!(TrainerCardData, stickerTiles) == 6056);
    assert!(offset_of!(TrainerCardData, cardTiles) == 6568);
    assert!(offset_of!(TrainerCardData, cardTilemapBuffer) == 15528);
    assert!(offset_of!(TrainerCardData, bgTilemapBuffer) == 23720);
    assert!(offset_of!(TrainerCardData, cardTop) == 31912);
    assert!(offset_of!(TrainerCardData, language) == 31914);
};

const CARD_FLIP_Y: i16 = 77;
const STATE_CLOSE_CARD: u8 = 14;
const STATE_CLOSE_CARD_LINK: u8 = 16;
const STATE_HANDLE_INPUT_BACK: u8 = 11;
const STATE_HANDLE_INPUT_FRONT: u8 = 10;
const STATE_WAIT_FLIP_TO_BACK: u8 = 12;
const STATE_WAIT_FLIP_TO_FRONT: u8 = 13;
const STATE_WAIT_LINK_PARTNER: u8 = 15;
const WIN_CARD_TEXT: u8 = 1;
const WIN_MSG: u8 = 0;
const WIN_TRAINER_PIC: u8 = 2;

static sHoennTrainerCardBadges_Gfx: Table<CArray<u32, 146>> =
    Table((&raw const crate::data::trainer_card::sHoennTrainerCardBadges_Gfx).cast());
static sHoennTrainerCardBadges_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sHoennTrainerCardBadges_Pal).cast());
static sHoennTrainerCardFemaleBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sHoennTrainerCardFemaleBg_Pal).cast());
static sHoennTrainerCardPals: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::trainer_card::sHoennTrainerCardPals).cast());
static sKantoTrainerCardBadges_Gfx: Table<CArray<u32, 168>> =
    Table((&raw const crate::data::trainer_card::sKantoTrainerCardBadges_Gfx).cast());
static sKantoTrainerCardBadges_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sKantoTrainerCardBadges_Pal).cast());
static sKantoTrainerCardFemaleBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sKantoTrainerCardFemaleBg_Pal).cast());
static sKantoTrainerCardPals: Table<CArray<*mut u16, 5>> =
    Table((&raw const crate::data::trainer_card::sKantoTrainerCardPals).cast());
static sLinkBattleTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::trainer_card::sLinkBattleTexts).cast());
static sText_HofTime: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::trainer_card::sText_HofTime).cast());
static sTimeColonTextColors: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::trainer_card::sTimeColonTextColors).cast());
static sTrainerCardBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardBgTemplates).cast());
static sTrainerCardFlipTasks: Table<CArray<Option<unsafe extern "C" fn(*mut Task) -> u8>, 6>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardFlipTasks).cast());
static sTrainerCardStar_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardStar_Pal).cast());
static sTrainerCardStatColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardStatColors).cast());
static sTrainerCardSticker1_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardSticker1_Pal).cast());
static sTrainerCardSticker2_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardSticker2_Pal).cast());
static sTrainerCardSticker3_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardSticker3_Pal).cast());
static sTrainerCardSticker4_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardSticker4_Pal).cast());
static sTrainerCardStickers_Gfx: Table<CArray<u32, 93>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardStickers_Gfx).cast());
static sTrainerCardTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardTextColors).cast());
static sTrainerCardWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::trainer_card::sTrainerCardWindowTemplates).cast());
static sTrainerPicFacilityClass: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::trainer_card::sTrainerPicFacilityClass).cast());
static sTrainerPicOffset: Table<CArray<CArray<CArray<u8, 2>, 2>, 2>> =
    Table((&raw const crate::data::trainer_card::sTrainerPicOffset).cast());
static widths_1: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trainer_card::widths_1).cast());
static xOffsets_2: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trainer_card::xOffsets_2).cast());
static yOffsetsLine1_4: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trainer_card::yOffsetsLine1_4).cast());
static yOffsetsLine2_3: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trainer_card::yOffsetsLine2_3).cast());
static yOffsets_0: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trainer_card::yOffsets_0).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerCards: CArray<TrainerCard, 4> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sData: *mut TrainerCardData = null_mut();

unsafe extern "C" {
    static gGameVersion: u8;
    static gHoennTrainerCardBack_Tilemap: CArray<u32, 0>;
    static gHoennTrainerCardBg_Tilemap: CArray<u32, 0>;
    static gHoennTrainerCardFrontLink_Tilemap: CArray<u32, 0>;
    static gHoennTrainerCardFront_Tilemap: CArray<u32, 0>;
    static gHoennTrainerCard_Gfx: CArray<u32, 0>;
    static gKantoTrainerCardBack_Tilemap: CArray<u32, 0>;
    static gKantoTrainerCardBg_Tilemap: CArray<u32, 0>;
    static gKantoTrainerCardFrontLink_Tilemap: CArray<u32, 0>;
    static gKantoTrainerCardFront_Tilemap: CArray<u32, 0>;
    static gKantoTrainerCard_Gfx: CArray<u32, 0>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static gMonIconPalettes: CArray<CArray<u16, 16>, 0>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattlePtsWon: CArray<u8, 0>;
    static gText_BattleTower: CArray<u8, 0>;
    static gText_BerryCrush: CArray<u8, 0>;
    static gText_Colon2: CArray<u8, 0>;
    static gText_EmptyString6: CArray<u8, 0>;
    static gText_HallOfFameDebut: CArray<u8, 0>;
    static gText_NumBP: CArray<u8, 0>;
    static gText_NumPokeblocks: CArray<u8, 0>;
    static gText_PokeblocksWithFriends: CArray<u8, 0>;
    static gText_PokedollarVar1: CArray<u8, 0>;
    static gText_PokemonTrades: CArray<u8, 0>;
    static gText_TrainerCardIDNo: CArray<u8, 0>;
    static gText_TrainerCardMoney: CArray<u8, 0>;
    static gText_TrainerCardName: CArray<u8, 0>;
    static gText_TrainerCardPokedex: CArray<u8, 0>;
    static gText_TrainerCardTime: CArray<u8, 0>;
    static gText_UnionTradesAndBattles: CArray<u8, 0>;
    static gText_Var1sTrainerCard: CArray<u8, 0>;
    static gText_WaitingTrainerFinishReading: CArray<u8, 0>;
    static gText_WinsLosses: CArray<u8, 0>;
    static gText_WinsStraight: CArray<u8, 0>;
    static gText_WonContestsWFriends: CArray<u8, 0>;
    static gUnionRoomFacilityClasses: CArray<u16, 0>;
    static mut gWirelessCommType: u8;
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
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReshowFrontierPass();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountPlayerMuseumPaintings() -> u8;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerCardTrainerPicSprite(a0: u16, a1: u8, a2: u16, a3: u16, a4: u8, a5: u8) -> u16;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn EnableInterrupts(a0: u16);
    fn FacilityClassToPicIndex(a0: u16) -> u16;
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetGameStat(a0: u8) -> u32;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetMonIconPaletteIndexFromSpecies(a0: u16) -> u8;
    fn GetMonIconTiles(a0: u16, a1: u32) -> *mut u8;
    fn GetMoney(a0: *mut u32) -> u32;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HasAllHoennMons() -> u16;
    fn HideBg(a0: u8);
    fn InUnionRoom() -> u32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Overworld_IsRecvQueueAtMax() -> u32;
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TintPalette_CustomTone(a0: *mut u16, a1: u16, a2: u16, a3: u16, a4: u16);
    fn TintPalette_SepiaTone(a0: *mut u16, a1: u16);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn WriteSequenceToBgTilemapBuffer(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: i16,
    );
}

pub(crate) unsafe extern "C" fn VblankCb_TrainerCard() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    BlinkTimeColon();
    if (*sData).allowDMACopy != 0 {
        {
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(
                        dmaRegs,
                        &raw mut gScanlineEffectRegBuffers[0] as usize as u32,
                    );
                    volatile_write(
                        dmaRegs.at(1),
                        &raw mut gScanlineEffectRegBuffers[1] as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x800000a0);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HblankCb_TrainerCard() {
    let mut backup: u16 = 0;
    let mut bgVOffset: u16 = 0;
    backup = (67109384 as usize as *mut u16).read_volatile();
    volatile_write(67109384 as usize as *mut u16, 0);
    bgVOffset =
        gScanlineEffectRegBuffers[1][(67108870 as usize as *mut u16).read_volatile() as i32 & 0xFF];
    volatile_write(67108882 as usize as *mut u16, bgVOffset);
    volatile_write(67109384 as usize as *mut u16, backup);
}
pub(crate) unsafe extern "C" fn CB2_TrainerCard() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn CloseTrainerCard(taskId: u8) {
    SetMainCallback2((*sData).callback2);
    FreeAllWindowBuffers();
    Free(sData as *mut c_void);
    sData = null_mut();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn Task_TrainerCard(taskId: u8) {
    match (*sData).mainState {
        0 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                FillWindowPixelBuffer(WIN_CARD_TEXT, 0);
                (*sData).mainState += 1;
            }
        }
        1 => {
            if PrintAllOnCardFront() != 0 {
                (*sData).mainState += 1;
            }
        }
        2 => {
            DrawTrainerCardWindow(WIN_CARD_TEXT);
            (*sData).mainState += 1;
        }
        3 => {
            FillWindowPixelBuffer(WIN_TRAINER_PIC, 0);
            CreateTrainerCardTrainerPic();
            DrawTrainerCardWindow(WIN_TRAINER_PIC);
            (*sData).mainState += 1;
        }
        4 => {
            DrawCardScreenBackground((*sData).bgTilemap.as_mut_ptr());
            (*sData).mainState += 1;
        }
        5 => {
            DrawCardFrontOrBack((*sData).frontTilemap.as_mut_ptr());
            (*sData).mainState += 1;
        }
        6 => {
            DrawStarsAndBadgesOnCard();
            (*sData).mainState += 1;
        }
        7 => {
            if gWirelessCommType == 1 && gReceivedRemoteLinkPlayers == 1 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(230, 150);
            }
            BlendPalettes(PALETTES_ALL, 16, (*sData).blendColor);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, (*sData).blendColor);
            SetVBlankCallback(Some(VblankCb_TrainerCard));
            (*sData).mainState += 1;
        }
        8 => {
            if UpdatePaletteFade() == 0 && IsDma3ManagerBusyWithBgCopy() == 0 {
                PlaySE(SE_RG_CARD_OPEN);
                (*sData).mainState = STATE_HANDLE_INPUT_FRONT;
            }
        }
        9 => {
            if IsSEPlaying() == 0 {
                (*sData).mainState += 1;
            }
        }
        STATE_HANDLE_INPUT_FRONT => {
            if gReceivedRemoteLinkPlayers == 0 && (*sData).timeColonNeedDraw != 0 {
                PrintTimeOnCard();
                DrawTrainerCardWindow(WIN_CARD_TEXT);
                (*sData).timeColonNeedDraw = FALSE;
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                FlipTrainerCard();
                PlaySE(SE_RG_CARD_FLIP);
                (*sData).mainState = STATE_WAIT_FLIP_TO_BACK;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                if gReceivedRemoteLinkPlayers != 0
                    && (*sData).isLink != 0
                    && InUnionRoom() == TRUE as u32
                {
                    (*sData).mainState = STATE_WAIT_LINK_PARTNER;
                } else {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, (*sData).blendColor);
                    (*sData).mainState = STATE_CLOSE_CARD;
                }
            }
        }
        STATE_WAIT_FLIP_TO_BACK => {
            if IsCardFlipTaskActive() != 0 && Overworld_IsRecvQueueAtMax() != TRUE as u32 {
                PlaySE(SE_RG_CARD_OPEN);
                (*sData).mainState = STATE_HANDLE_INPUT_BACK;
            }
        }
        STATE_HANDLE_INPUT_BACK => {
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                if gReceivedRemoteLinkPlayers != 0
                    && (*sData).isLink != 0
                    && InUnionRoom() == TRUE as u32
                {
                    (*sData).mainState = STATE_WAIT_LINK_PARTNER;
                } else if gReceivedRemoteLinkPlayers != 0 {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, (*sData).blendColor);
                    (*sData).mainState = STATE_CLOSE_CARD;
                } else {
                    FlipTrainerCard();
                    (*sData).mainState = STATE_WAIT_FLIP_TO_FRONT;
                    PlaySE(SE_RG_CARD_FLIP);
                }
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                if gReceivedRemoteLinkPlayers != 0
                    && (*sData).isLink != 0
                    && InUnionRoom() == TRUE as u32
                {
                    (*sData).mainState = STATE_WAIT_LINK_PARTNER;
                } else {
                    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, (*sData).blendColor);
                    (*sData).mainState = STATE_CLOSE_CARD;
                }
            }
        }
        STATE_WAIT_LINK_PARTNER => {
            SetCloseLinkCallback();
            DrawDialogueFrame(WIN_MSG, TRUE);
            AddTextPrinterParameterized(
                WIN_MSG,
                FONT_NORMAL,
                gText_WaitingTrainerFinishReading.as_ptr().cast_mut(),
                0,
                1,
                255,
                None,
            );
            CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
            (*sData).mainState = STATE_CLOSE_CARD_LINK;
        }
        STATE_CLOSE_CARD_LINK => {
            if gReceivedRemoteLinkPlayers == 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, (*sData).blendColor);
                (*sData).mainState = STATE_CLOSE_CARD;
            }
        }
        STATE_CLOSE_CARD => {
            if UpdatePaletteFade() == 0 {
                CloseTrainerCard(taskId);
            }
        }
        STATE_WAIT_FLIP_TO_FRONT => {
            if IsCardFlipTaskActive() != 0 && Overworld_IsRecvQueueAtMax() != TRUE as u32 {
                (*sData).mainState = STATE_HANDLE_INPUT_FRONT;
                PlaySE(SE_RG_CARD_OPEN);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn LoadCardGfx() -> u8 {
    match (*sData).gfxLoadState {
        0 => {
            if (*sData).cardType != CARD_TYPE_FRLG {
                LZ77UnCompWram(
                    gHoennTrainerCardBg_Tilemap.as_ptr().cast_mut(),
                    (*sData).bgTilemap.as_mut_ptr() as *mut c_void,
                );
            } else {
                LZ77UnCompWram(
                    gKantoTrainerCardBg_Tilemap.as_ptr().cast_mut(),
                    (*sData).bgTilemap.as_mut_ptr() as *mut c_void,
                );
            }
        }
        1 => {
            if (*sData).cardType != CARD_TYPE_FRLG {
                LZ77UnCompWram(
                    gHoennTrainerCardBack_Tilemap.as_ptr().cast_mut(),
                    (*sData).backTilemap.as_mut_ptr() as *mut c_void,
                );
            } else {
                LZ77UnCompWram(
                    gKantoTrainerCardBack_Tilemap.as_ptr().cast_mut(),
                    (*sData).backTilemap.as_mut_ptr() as *mut c_void,
                );
            }
        }
        2 => {
            if (*sData).isLink == 0 {
                if (*sData).cardType != CARD_TYPE_FRLG {
                    LZ77UnCompWram(
                        gHoennTrainerCardFront_Tilemap.as_ptr().cast_mut(),
                        (*sData).frontTilemap.as_mut_ptr() as *mut c_void,
                    );
                } else {
                    LZ77UnCompWram(
                        gKantoTrainerCardFront_Tilemap.as_ptr().cast_mut(),
                        (*sData).frontTilemap.as_mut_ptr() as *mut c_void,
                    );
                }
            } else {
                if (*sData).cardType != CARD_TYPE_FRLG {
                    LZ77UnCompWram(
                        gHoennTrainerCardFrontLink_Tilemap.as_ptr().cast_mut(),
                        (*sData).frontTilemap.as_mut_ptr() as *mut c_void,
                    );
                } else {
                    LZ77UnCompWram(
                        gKantoTrainerCardFrontLink_Tilemap.as_ptr().cast_mut(),
                        (*sData).frontTilemap.as_mut_ptr() as *mut c_void,
                    );
                }
            }
        }
        3 => {
            if (*sData).cardType != CARD_TYPE_FRLG {
                LZ77UnCompWram(
                    sHoennTrainerCardBadges_Gfx.as_ptr().cast_mut(),
                    (*sData).badgeTiles.as_mut_ptr() as *mut c_void,
                );
            } else {
                LZ77UnCompWram(
                    sKantoTrainerCardBadges_Gfx.as_ptr().cast_mut(),
                    (*sData).badgeTiles.as_mut_ptr() as *mut c_void,
                );
            }
        }
        4 => {
            if (*sData).cardType != CARD_TYPE_FRLG {
                LZ77UnCompWram(
                    gHoennTrainerCard_Gfx.as_ptr().cast_mut(),
                    (*sData).cardTiles.as_mut_ptr() as *mut c_void,
                );
            } else {
                LZ77UnCompWram(
                    gKantoTrainerCard_Gfx.as_ptr().cast_mut(),
                    (*sData).cardTiles.as_mut_ptr() as *mut c_void,
                );
            }
        }
        5 => {
            if (*sData).cardType == CARD_TYPE_FRLG {
                LZ77UnCompWram(
                    sTrainerCardStickers_Gfx.as_ptr().cast_mut(),
                    (*sData).stickerTiles.as_mut_ptr() as *mut c_void,
                );
            }
        }
        _ => {
            (*sData).gfxLoadState = 0;
            return TRUE;
        }
    }
    (*sData).gfxLoadState += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn CB2_InitTrainerCard() {
    'l1: {
        let sw1: u8 = gMain.state;
        let matched = sw1 == 0
            || sw1 == 1
            || sw1 == 2
            || sw1 == 3
            || sw1 == 4
            || sw1 == 5
            || sw1 == 6
            || sw1 == 7
            || sw1 == 8
            || sw1 == 9
            || sw1 == 10;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            ResetGpuRegs();
            SetUpTrainerCardTask();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
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
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sData).blendColor == 0 {
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
            }
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            ResetSpriteData();
            FreeAllSpritePalettes();
            ResetPaletteFade();
            gMain.state += 1;
        }
        if fall || sw1 == 4 {
            fall = true;
            InitBgsAndWindows();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            LoadMonIconGfx();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if LoadCardGfx() == TRUE {
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            LoadStickerGfx();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 8 {
            fall = true;
            InitGpuRegs();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 9 {
            fall = true;
            BufferTextsVarsForCardPage2();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            if SetCardBgsAndPals() == TRUE {
                gMain.state += 1;
            }
            break 'l1;
        }
        if !matched {
            fall = true;
            SetTrainerCardCb2();
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetCappedGameStat(statId: u8, maxValue: u32) -> u32 {
    let mut statValue: u32 = GetGameStat(statId);
    return if maxValue < statValue {
        maxValue
    } else {
        statValue
    };
}
pub(crate) unsafe extern "C" fn HasAllFrontierSymbols() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_FRONTIER_FACILITIES {
        if FlagGet(FLAG_SYS_TOWER_SILVER + 2 * i as u16) == 0
            || FlagGet(FLAG_SYS_TOWER_GOLD + 2 * i as u16) == 0
        {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPlayerTrainerStars() -> u32 {
    let mut stars: u8 = 0;
    if GetGameStat(GAME_STAT_ENTERED_HOF) != 0 {
        stars += 1;
    }
    if HasAllHoennMons() != 0 {
        stars += 1;
    }
    if CountPlayerMuseumPaintings() >= CONTEST_CATEGORIES_COUNT as u8 {
        stars += 1;
    }
    if HasAllFrontierSymbols() != 0 {
        stars += 1;
    }
    return stars as u32;
}
pub(crate) unsafe extern "C" fn GetRubyTrainerStars(trainerCard: *mut TrainerCard) -> u8 {
    let mut stars: u8 = 0;
    if (*trainerCard).hofDebutHours != 0
        || (*trainerCard).hofDebutMinutes != 0
        || (*trainerCard).hofDebutSeconds != 0
    {
        stars += 1;
    }
    if (*trainerCard).caughtAllHoenn != 0 {
        stars += 1;
    }
    if (*trainerCard).battleTowerStraightWins > 49 {
        stars += 1;
    }
    if (*trainerCard).hasAllPaintings != 0 {
        stars += 1;
    }
    return stars;
}
pub(crate) unsafe extern "C" fn SetPlayerCardData(trainerCard: *mut TrainerCard, cardType: u8) {
    let mut playTime: u32 = 0;
    let mut i: u8 = 0;
    (*trainerCard).gender = (*gSaveBlock2Ptr).playerGender;
    (*trainerCard).playTimeHours = (*gSaveBlock2Ptr).playTimeHours;
    (*trainerCard).playTimeMinutes = (*gSaveBlock2Ptr).playTimeMinutes as u16;
    playTime = GetGameStat(GAME_STAT_FIRST_HOF_PLAY_TIME);
    if GetGameStat(GAME_STAT_ENTERED_HOF) == 0 {
        playTime = 0;
    }
    (*trainerCard).hofDebutHours = (playTime >> 16) as u16;
    (*trainerCard).hofDebutMinutes = (playTime >> 8) as u16 & 0xFF;
    (*trainerCard).hofDebutSeconds = playTime as u16 & 0xFF;
    if playTime >> 16 > 999 {
        (*trainerCard).hofDebutHours = 999;
        (*trainerCard).hofDebutMinutes = 59;
        (*trainerCard).hofDebutSeconds = 59;
    }
    (*trainerCard).hasPokedex = FlagGet(FLAG_SYS_POKEDEX_GET);
    (*trainerCard).caughtAllHoenn = HasAllHoennMons() as u8;
    (*trainerCard).caughtMonsCount = GetCaughtMonsCount();
    (*trainerCard).trainerId = ((*gSaveBlock2Ptr).playerTrainerId[1] as u16) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u16;
    (*trainerCard).linkBattleWins = GetCappedGameStat(GAME_STAT_LINK_BATTLE_WINS, 9999) as u16;
    (*trainerCard).linkBattleLosses = GetCappedGameStat(GAME_STAT_LINK_BATTLE_LOSSES, 9999) as u16;
    (*trainerCard).pokemonTrades = GetCappedGameStat(GAME_STAT_POKEMON_TRADES, 0xFFFF) as u16;
    (*trainerCard).money = GetMoney(&raw mut (*gSaveBlock1Ptr).money);
    i = 0;
    while i < TRAINER_CARD_PROFILE_LENGTH {
        (*trainerCard).easyChatProfile[i] = (*gSaveBlock1Ptr).easyChatProfile[i];
        i += 1;
    }
    StringCopy(
        (*trainerCard).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    'l2: {
        let sw1: u8 = cardType;
        let mut fall = false;
        if sw1 == CARD_TYPE_EMERALD {
            fall = true;
            (*trainerCard).battleTowerWins = 0;
            (*trainerCard).battleTowerStraightWins = 0;
        }
        if fall || sw1 == CARD_TYPE_FRLG {
            fall = true;
            (*trainerCard).contestsWithFriends =
                GetCappedGameStat(GAME_STAT_WON_LINK_CONTEST, 999) as u16;
            (*trainerCard).pokeblocksWithFriends =
                GetCappedGameStat(GAME_STAT_POKEBLOCKS_WITH_FRIENDS, 0xFFFF) as u16;
            if CountPlayerMuseumPaintings() >= CONTEST_CATEGORIES_COUNT as u8 {
                (*trainerCard).hasAllPaintings = TRUE;
            }
            (*trainerCard).stars = GetRubyTrainerStars(trainerCard);
            break 'l2;
        }
        if sw1 == CARD_TYPE_RS {
            fall = true;
            (*trainerCard).battleTowerWins = 0;
            (*trainerCard).battleTowerStraightWins = 0;
            (*trainerCard).contestsWithFriends = 0;
            (*trainerCard).pokeblocksWithFriends = 0;
            (*trainerCard).hasAllPaintings = 0;
            (*trainerCard).stars = 0;
            break 'l2;
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerCard_GenerateCardForPlayer(trainerCard: *mut TrainerCard) {
    memset(trainerCard as *mut u8, 0, 100);
    (*trainerCard).version = GAME_VERSION;
    SetPlayerCardData(trainerCard, CARD_TYPE_EMERALD);
    (*trainerCard).hasAllFrontierSymbols = HasAllFrontierSymbols() as u16;
    (*trainerCard).frontierBP = (*gSaveBlock2Ptr).frontier.cardBattlePoints;
    if (*trainerCard).hasAllFrontierSymbols != 0 {
        (*trainerCard).stars += 1;
    }
    if (*trainerCard).gender == FEMALE {
        (*trainerCard).unionRoomClass = gUnionRoomFacilityClasses
            [(*trainerCard).trainerId as i32 % 8 + NUM_UNION_ROOM_CLASSES as i32]
            as u8;
    } else {
        (*trainerCard).unionRoomClass =
            gUnionRoomFacilityClasses[(*trainerCard).trainerId as i32 % 8] as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrainerCard_GenerateCardForLinkPlayer(trainerCard: *mut TrainerCard) {
    memset(trainerCard as *mut u8, 0, 0x60);
    (*trainerCard).version = GAME_VERSION;
    SetPlayerCardData(trainerCard, CARD_TYPE_EMERALD);
    (*trainerCard).linkHasAllFrontierSymbols = HasAllFrontierSymbols() as u16;
    *(&raw mut (*trainerCard).linkPoints.frontier as *mut u16) =
        (*gSaveBlock2Ptr).frontier.cardBattlePoints;
    if (*trainerCard).linkHasAllFrontierSymbols != 0 {
        (*trainerCard).stars += 1;
    }
    if (*trainerCard).gender == FEMALE {
        (*trainerCard).unionRoomClass = gUnionRoomFacilityClasses
            [(*trainerCard).trainerId as i32 % 8 + NUM_UNION_ROOM_CLASSES as i32]
            as u8;
    } else {
        (*trainerCard).unionRoomClass =
            gUnionRoomFacilityClasses[(*trainerCard).trainerId as i32 % 8] as u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyTrainerCardData(
    dst: *mut TrainerCard,
    src: *mut TrainerCard,
    gameVersion: u8,
) {
    memset(dst as *mut u8, 0, 100);
    (*dst).version = gameVersion;
    match VersionToCardType(gameVersion) {
        CARD_TYPE_FRLG => {
            memcpy(dst as *mut u8, src as *mut u8, 0x60);
        }
        CARD_TYPE_RS => {
            memcpy(dst as *mut u8, src as *mut u8, 0x38);
        }
        CARD_TYPE_EMERALD => {
            memcpy(dst as *mut u8, src as *mut u8, 0x60);
            (*dst).linkPoints.frontier = 0;
            (*dst).hasAllFrontierSymbols = (*src).linkHasAllFrontierSymbols;
            (*dst).frontierBP = *(&raw mut (*src).linkPoints.frontier as *mut u16);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetDataFromTrainerCard() {
    let mut i: u8 = 0;
    let mut badgeFlag: u32 = 0;
    (*sData).hasPokedex = FALSE;
    (*sData).hasHofResult = FALSE;
    (*sData).hasLinkResults = FALSE;
    (*sData).hasBattleTowerWins = FALSE;
    (*sData).unused_E = FALSE;
    (*sData).unused_F = FALSE;
    (*sData).hasTrades = FALSE;
    memset((*sData).badgeCount.as_mut_ptr(), 0, 8);
    if (*sData).trainerCard.hasPokedex != 0 {
        (*sData).hasPokedex += 1;
    }
    if (*sData).trainerCard.hofDebutHours != 0
        || (*sData).trainerCard.hofDebutMinutes != 0
        || (*sData).trainerCard.hofDebutSeconds != 0
    {
        (*sData).hasHofResult += 1;
    }
    if (*sData).trainerCard.linkBattleWins != 0 || (*sData).trainerCard.linkBattleLosses != 0 {
        (*sData).hasLinkResults += 1;
    }
    if (*sData).trainerCard.pokemonTrades != 0 {
        (*sData).hasTrades += 1;
    }
    if (*sData).trainerCard.battleTowerWins != 0
        || (*sData).trainerCard.battleTowerStraightWins != 0
    {
        (*sData).hasBattleTowerWins += 1;
    }
    i = 0;
    badgeFlag = FLAG_BADGE01_GET;
    while badgeFlag < 2159 {
        if FlagGet(badgeFlag as u16) != 0 {
            (*sData).badgeCount[i] += 1;
        }
        badgeFlag += 1;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitGpuRegs() {
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
    SetGpuReg(REG_OFFSET_BLDCNT, 193);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_WININ, 63);
    SetGpuReg(REG_OFFSET_WINOUT, 30);
    SetGpuReg(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
    SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
    if gReceivedRemoteLinkPlayers != 0 {
        EnableInterrupts(199);
    } else {
        EnableInterrupts(3);
    }
}
pub(crate) unsafe extern "C" fn UpdateCardFlipRegs(cardTop: u16) {
    let mut blendY: i8 = ((cardTop as i32 + 40) / 10) as i8;
    if blendY <= 4 {
        blendY = 0;
    }
    (*sData).flipBlendY = blendY;
    SetGpuReg(REG_OFFSET_BLDY, (*sData).flipBlendY as u16);
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (*sData).cardTop << 8 | DISPLAY_HEIGHT - (*sData).cardTop,
    );
}
pub(crate) unsafe extern "C" fn ResetGpuRegs() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
}
pub(crate) unsafe extern "C" fn InitBgsAndWindows() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sTrainerCardBgTemplates.as_ptr().cast_mut(), 4);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    InitWindows(sTrainerCardWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadMessageBoxAndBorderGfx();
}
pub(crate) unsafe extern "C" fn SetTrainerCardCb2() {
    SetMainCallback2(Some(CB2_TrainerCard));
}
pub(crate) unsafe extern "C" fn SetUpTrainerCardTask() {
    ResetTasks();
    ScanlineEffect_Stop();
    CreateTask(Some(Task_TrainerCard), 0);
    InitTrainerCardData();
    SetDataFromTrainerCard();
}
pub(crate) unsafe extern "C" fn PrintAllOnCardFront() -> u8 {
    match (*sData).printState {
        0 => {
            PrintNameOnCardFront();
        }
        1 => {
            PrintIdOnCard();
        }
        2 => {
            PrintMoneyOnCard();
        }
        3 => {
            PrintPokedexOnCard();
        }
        4 => {
            PrintTimeOnCard();
        }
        5 => {
            PrintProfilePhraseOnCard();
        }
        _ => {
            (*sData).printState = 0;
            return TRUE;
        }
    }
    (*sData).printState += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn PrintAllOnCardBack() -> u8 {
    match (*sData).printState {
        0 => {
            PrintNameOnCardBack();
        }
        1 => {
            PrintHofDebutTimeOnCard();
        }
        2 => {
            PrintLinkBattleResultsOnCard();
        }
        3 => {
            PrintTradesStringOnCard();
        }
        4 => {
            PrintBerryCrushStringOnCard();
            PrintPokeblockStringOnCard();
        }
        5 => {
            PrintUnionStringOnCard();
            PrintContestStringOnCard();
        }
        6 => {
            PrintPokemonIconsOnCard();
            PrintBattleFacilityStringOnCard();
        }
        7 => {
            PrintStickersOnCard();
        }
        _ => {
            (*sData).printState = 0;
            return TRUE;
        }
    }
    (*sData).printState += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn BufferTextsVarsForCardPage2() {
    BufferNameForCardBack();
    BufferHofDebutTime();
    BufferLinkBattleResults();
    BufferNumTrades();
    BufferBerryCrushPoints();
    BufferUnionRoomStats();
    BufferLinkPokeblocksNum();
    BufferLinkContestNum();
    BufferBattleFacilityStats();
}
pub(crate) unsafe extern "C" fn PrintNameOnCardFront() {
    let mut buffer: CArray<u8, 32> = zeroed();
    let mut txtPtr: *mut u8 = null_mut();
    txtPtr = StringCopy(
        buffer.as_mut_ptr(),
        gText_TrainerCardName.as_ptr().cast_mut(),
    );
    StringCopy(txtPtr, (*sData).trainerCard.playerName.as_mut_ptr());
    ConvertInternationalString(txtPtr, (*sData).language);
    if (*sData).cardType == CARD_TYPE_FRLG {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            20,
            28,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            buffer.as_mut_ptr(),
        );
    } else {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            16,
            33,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            buffer.as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintIdOnCard() {
    let mut buffer: CArray<u8, 32> = zeroed();
    let mut txtPtr: *mut u8 = null_mut();
    let mut xPos: i32 = 0;
    let mut top: u32 = 0;
    txtPtr = StringCopy(
        buffer.as_mut_ptr(),
        gText_TrainerCardIDNo.as_ptr().cast_mut(),
    );
    ConvertIntToDecimalStringN(
        txtPtr,
        (*sData).trainerCard.trainerId as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        5,
    );
    if (*sData).cardType == CARD_TYPE_FRLG {
        xPos = GetStringCenterAlignXOffset(FONT_NORMAL as i32, buffer.as_mut_ptr(), 80) + 132;
        top = 9;
    } else {
        xPos = GetStringCenterAlignXOffset(FONT_NORMAL as i32, buffer.as_mut_ptr(), 96) + 120;
        top = 9;
    }
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        xPos as u8,
        top as u8,
        sTrainerCardTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        buffer.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn PrintMoneyOnCard() {
    let mut xOffset: i32 = 0;
    let mut top: u8 = 0;
    if (*sData).isHoenn == 0 {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            20,
            56,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gText_TrainerCardMoney.as_ptr().cast_mut(),
        );
    } else {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            16,
            57,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gText_TrainerCardMoney.as_ptr().cast_mut(),
        );
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sData).trainerCard.money as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        6,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PokedollarVar1.as_ptr().cast_mut(),
    );
    if (*sData).isHoenn == 0 {
        xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 144);
        top = 56;
    } else {
        xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 128);
        top = 57;
    }
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        xOffset as u8,
        top,
        sTrainerCardTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn GetCaughtMonsCount() -> u16 {
    if IsNationalPokedexEnabled() != 0 {
        return GetNationalPokedexCount(FLAG_GET_CAUGHT);
    } else {
        return GetHoennPokedexCount(FLAG_GET_CAUGHT);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PrintPokedexOnCard() {
    let mut xOffset: i32 = 0;
    let mut top: u8 = 0;
    if FlagGet(FLAG_SYS_POKEDEX_GET) != 0 {
        if (*sData).isHoenn == 0 {
            AddTextPrinterParameterized3(
                WIN_CARD_TEXT,
                FONT_NORMAL,
                20,
                72,
                sTrainerCardTextColors.as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                gText_TrainerCardPokedex.as_ptr().cast_mut(),
            );
        } else {
            AddTextPrinterParameterized3(
                WIN_CARD_TEXT,
                FONT_NORMAL,
                16,
                73,
                sTrainerCardTextColors.as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                gText_TrainerCardPokedex.as_ptr().cast_mut(),
            );
        }
        StringCopy(
            ConvertIntToDecimalStringN(
                gStringVar4.as_mut_ptr(),
                (*sData).trainerCard.caughtMonsCount as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                3,
            ),
            gText_EmptyString6.as_ptr().cast_mut(),
        );
        if (*sData).isHoenn == 0 {
            xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 144);
            top = 72;
        } else {
            xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 128);
            top = 73;
        }
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            xOffset as u8,
            top,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gStringVar4.as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintTimeOnCard() {
    let mut hours: u16 = 0;
    let mut minutes: u16 = 0;
    let mut width: i32 = 0;
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut totalWidth: u32 = 0;
    if (*sData).isHoenn == 0 {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            20,
            88,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gText_TrainerCardTime.as_ptr().cast_mut(),
        );
    } else {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            16,
            89,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gText_TrainerCardTime.as_ptr().cast_mut(),
        );
    }
    if (*sData).isLink != 0 {
        hours = (*sData).trainerCard.playTimeHours;
        minutes = (*sData).trainerCard.playTimeMinutes;
    } else {
        hours = (*gSaveBlock2Ptr).playTimeHours;
        minutes = (*gSaveBlock2Ptr).playTimeMinutes as u16;
    }
    if hours > 999 {
        hours = 999;
    }
    if minutes > 59 {
        minutes = 59;
    }
    width = GetStringWidth(FONT_NORMAL, gText_Colon2.as_ptr().cast_mut(), 0);
    if (*sData).isHoenn == 0 {
        x = 144;
        y = 88;
    } else {
        x = 128;
        y = 89;
    }
    totalWidth = width as u32 + 30;
    x -= totalWidth;
    FillWindowPixelRect(WIN_CARD_TEXT, 0, x as u16, y as u16, totalWidth as u16, 15);
    ConvertIntToDecimalStringN(
        gStringVar4.as_mut_ptr(),
        hours as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        x as u8,
        y as u8,
        sTrainerCardTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
    x += 18;
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        x as u8,
        y as u8,
        sTimeColonTextColors[(*sData).timeColonInvisible],
        TEXT_SKIP_DRAW as i8,
        gText_Colon2.as_ptr().cast_mut(),
    );
    x += width as u32;
    ConvertIntToDecimalStringN(
        gStringVar4.as_mut_ptr(),
        minutes as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        x as u8,
        y as u8,
        sTrainerCardTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn PrintProfilePhraseOnCard() {
    if (*sData).isLink != 0 {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            8,
            yOffsetsLine1_4[(*sData).isHoenn],
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).easyChatProfile[0].as_mut_ptr(),
        );
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            GetStringWidth(FONT_NORMAL, (*sData).easyChatProfile[0].as_mut_ptr(), 0) as u8 + 14,
            yOffsetsLine1_4[(*sData).isHoenn],
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).easyChatProfile[1].as_mut_ptr(),
        );
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            8,
            yOffsetsLine2_3[(*sData).isHoenn],
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).easyChatProfile[2].as_mut_ptr(),
        );
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            GetStringWidth(FONT_NORMAL, (*sData).easyChatProfile[2].as_mut_ptr(), 0) as u8 + 14,
            yOffsetsLine2_3[(*sData).isHoenn],
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).easyChatProfile[3].as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferNameForCardBack() {
    StringCopy(
        (*sData).textPlayersCard.as_mut_ptr(),
        (*sData).trainerCard.playerName.as_mut_ptr(),
    );
    ConvertInternationalString((*sData).textPlayersCard.as_mut_ptr(), (*sData).language);
    if (*sData).cardType != CARD_TYPE_FRLG {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*sData).textPlayersCard.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            (*sData).textPlayersCard.as_mut_ptr(),
            gText_Var1sTrainerCard.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintNameOnCardBack() {
    if (*sData).isHoenn == 0 {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            136,
            9,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).textPlayersCard.as_mut_ptr(),
        );
    } else {
        AddTextPrinterParameterized3(
            WIN_CARD_TEXT,
            FONT_NORMAL,
            GetStringRightAlignXOffset(
                FONT_NORMAL as i32,
                (*sData).textPlayersCard.as_mut_ptr(),
                216,
            ) as u8,
            9,
            sTrainerCardTextColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            (*sData).textPlayersCard.as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferHofDebutTime() {
    if (*sData).hasHofResult != 0 {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*sData).trainerCard.hofDebutHours as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            3,
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            (*sData).trainerCard.hofDebutMinutes as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        ConvertIntToDecimalStringN(
            gStringVar3.as_mut_ptr(),
            (*sData).trainerCard.hofDebutSeconds as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        StringExpandPlaceholders(
            (*sData).textHofTime.as_mut_ptr(),
            sText_HofTime.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintStatOnBackOfCard(
    top: u8,
    statName: *mut u8,
    stat: *mut u8,
    color: *mut u8,
) {
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        xOffsets_2[(*sData).isHoenn],
        top * 16 + 33,
        sTrainerCardTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        statName,
    );
    AddTextPrinterParameterized3(
        WIN_CARD_TEXT,
        FONT_NORMAL,
        GetStringRightAlignXOffset(FONT_NORMAL as i32, stat, widths_1[(*sData).isHoenn] as i32)
            as u8,
        top * 16 + 33,
        color,
        TEXT_SKIP_DRAW as i8,
        stat,
    );
}
pub(crate) unsafe extern "C" fn PrintHofDebutTimeOnCard() {
    if (*sData).hasHofResult != 0 {
        PrintStatOnBackOfCard(
            0,
            gText_HallOfFameDebut.as_ptr().cast_mut(),
            (*sData).textHofTime.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferLinkBattleResults() {
    if (*sData).hasLinkResults != 0 {
        StringCopy(
            (*sData).textLinkBattleType.as_mut_ptr(),
            sLinkBattleTexts[(*sData).cardType],
        );
        ConvertIntToDecimalStringN(
            (*sData).textLinkBattleWins.as_mut_ptr(),
            (*sData).trainerCard.linkBattleWins as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            4,
        );
        ConvertIntToDecimalStringN(
            (*sData).textLinkBattleLosses.as_mut_ptr(),
            (*sData).trainerCard.linkBattleLosses as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            4,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintLinkBattleResultsOnCard() {
    if (*sData).hasLinkResults != 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*sData).textLinkBattleWins.as_mut_ptr(),
        );
        StringCopy(
            gStringVar2.as_mut_ptr(),
            (*sData).textLinkBattleLosses.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_WinsLosses.as_ptr().cast_mut(),
        );
        PrintStatOnBackOfCard(
            1,
            (*sData).textLinkBattleType.as_mut_ptr(),
            gStringVar4.as_mut_ptr(),
            sTrainerCardTextColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferNumTrades() {
    if (*sData).hasTrades != 0 {
        ConvertIntToDecimalStringN(
            (*sData).textNumTrades.as_mut_ptr(),
            (*sData).trainerCard.pokemonTrades as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            5,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintTradesStringOnCard() {
    if (*sData).hasTrades != 0 {
        PrintStatOnBackOfCard(
            2,
            gText_PokemonTrades.as_ptr().cast_mut(),
            (*sData).textNumTrades.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferBerryCrushPoints() {
    if (*sData).cardType == CARD_TYPE_FRLG && (*sData).trainerCard.linkPoints.berryCrush != 0 {
        ConvertIntToDecimalStringN(
            (*sData).textBerryCrushPts.as_mut_ptr(),
            (*sData).trainerCard.linkPoints.berryCrush as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            5,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBerryCrushStringOnCard() {
    if (*sData).cardType == CARD_TYPE_FRLG && (*sData).trainerCard.linkPoints.berryCrush != 0 {
        PrintStatOnBackOfCard(
            4,
            gText_BerryCrush.as_ptr().cast_mut(),
            (*sData).textBerryCrushPts.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferUnionRoomStats() {
    if (*sData).cardType == CARD_TYPE_FRLG && (*sData).trainerCard.unionRoomNum != 0 {
        ConvertIntToDecimalStringN(
            (*sData).textUnionRoomStats.as_mut_ptr(),
            (*sData).trainerCard.unionRoomNum as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            5,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintUnionStringOnCard() {
    if (*sData).cardType == CARD_TYPE_FRLG && (*sData).trainerCard.unionRoomNum != 0 {
        PrintStatOnBackOfCard(
            3,
            gText_UnionTradesAndBattles.as_ptr().cast_mut(),
            (*sData).textUnionRoomStats.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferLinkPokeblocksNum() {
    if (*sData).cardType != CARD_TYPE_FRLG && (*sData).trainerCard.pokeblocksWithFriends != 0 {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*sData).trainerCard.pokeblocksWithFriends as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            5,
        );
        StringExpandPlaceholders(
            (*sData).textNumLinkPokeblocks.as_mut_ptr(),
            gText_NumPokeblocks.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintPokeblockStringOnCard() {
    if (*sData).cardType != CARD_TYPE_FRLG && (*sData).trainerCard.pokeblocksWithFriends != 0 {
        PrintStatOnBackOfCard(
            3,
            gText_PokeblocksWithFriends.as_ptr().cast_mut(),
            (*sData).textNumLinkPokeblocks.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferLinkContestNum() {
    if (*sData).cardType != CARD_TYPE_FRLG && (*sData).trainerCard.contestsWithFriends != 0 {
        ConvertIntToDecimalStringN(
            (*sData).textNumLinkContests.as_mut_ptr(),
            (*sData).trainerCard.contestsWithFriends as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            5,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintContestStringOnCard() {
    if (*sData).cardType != CARD_TYPE_FRLG && (*sData).trainerCard.contestsWithFriends != 0 {
        PrintStatOnBackOfCard(
            4,
            gText_WonContestsWFriends.as_ptr().cast_mut(),
            (*sData).textNumLinkContests.as_mut_ptr(),
            sTrainerCardStatColors.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferBattleFacilityStats() {
    match (*sData).cardType {
        CARD_TYPE_RS => {
            if (*sData).hasBattleTowerWins != 0 {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    (*sData).trainerCard.battleTowerWins as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    4,
                );
                ConvertIntToDecimalStringN(
                    gStringVar2.as_mut_ptr(),
                    (*sData).trainerCard.battleTowerStraightWins as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    4,
                );
                StringExpandPlaceholders(
                    (*sData).textBattleFacilityStat.as_mut_ptr(),
                    gText_WinsStraight.as_ptr().cast_mut(),
                );
            }
        }
        CARD_TYPE_EMERALD => {
            if (*sData).trainerCard.frontierBP != 0 {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    (*sData).trainerCard.frontierBP as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    5,
                );
                StringExpandPlaceholders(
                    (*sData).textBattleFacilityStat.as_mut_ptr(),
                    gText_NumBP.as_ptr().cast_mut(),
                );
            }
        }
        CARD_TYPE_FRLG => {}
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PrintBattleFacilityStringOnCard() {
    match (*sData).cardType {
        CARD_TYPE_RS => {
            if (*sData).hasBattleTowerWins != 0 {
                PrintStatOnBackOfCard(
                    5,
                    gText_BattleTower.as_ptr().cast_mut(),
                    (*sData).textBattleFacilityStat.as_mut_ptr(),
                    sTrainerCardTextColors.as_ptr().cast_mut(),
                );
            }
        }
        CARD_TYPE_EMERALD => {
            if (*sData).trainerCard.frontierBP != 0 {
                PrintStatOnBackOfCard(
                    5,
                    gText_BattlePtsWon.as_ptr().cast_mut(),
                    (*sData).textBattleFacilityStat.as_mut_ptr(),
                    sTrainerCardStatColors.as_ptr().cast_mut(),
                );
            }
        }
        CARD_TYPE_FRLG => {}
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn PrintPokemonIconsOnCard() {
    let mut i: u8 = 0;
    let mut paletteSlots: CArray<u8, 6> = CArray([5, 6, 7, 8, 9, 10]);
    let mut xOffsets: CArray<u8, 6> = CArray([0, 4, 8, 12, 16, 20]);
    if (*sData).cardType == CARD_TYPE_FRLG {
        i = 0;
        while i < PARTY_SIZE as u8 {
            if (*sData).trainerCard.monSpecies[i] != 0 {
                let mut monSpecies: u8 =
                    GetMonIconPaletteIndexFromSpecies((*sData).trainerCard.monSpecies[i]);
                WriteSequenceToBgTilemapBuffer(
                    3,
                    16 * i as u16 + 224,
                    xOffsets[i] + 3,
                    15,
                    4,
                    4,
                    paletteSlots[monSpecies],
                    1,
                );
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMonIconGfx() {
    let mut i: u8 = 0;
    CpuSet(
        gMonIconPalettes.as_ptr().cast_mut() as *mut c_void,
        (*sData).monIconPal.as_mut_ptr() as *mut c_void,
        0x60,
    );
    match (*sData).trainerCard.monIconTint {
        MON_ICON_TINT_NORMAL => {}
        MON_ICON_TINT_BLACK => {
            TintPalette_CustomTone((*sData).monIconPal.as_mut_ptr(), 96, 0, 0, 0);
        }
        MON_ICON_TINT_PINK => {
            TintPalette_CustomTone((*sData).monIconPal.as_mut_ptr(), 96, 500, 330, 310);
        }
        MON_ICON_TINT_SEPIA => {
            TintPalette_SepiaTone((*sData).monIconPal.as_mut_ptr(), 96);
        }
        _ => {}
    }
    LoadPalette((*sData).monIconPal.as_mut_ptr() as *mut c_void, 80, 192);
    i = 0;
    while i < PARTY_SIZE as u8 {
        if (*sData).trainerCard.monSpecies[i] != 0 {
            LoadBgTiles(
                3,
                GetMonIconTiles((*sData).trainerCard.monSpecies[i], 0) as *mut c_void,
                512,
                16 * i as u16 + 32,
            );
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintStickersOnCard() {
    let mut i: u8 = 0;
    let mut paletteSlots: CArray<u8, 4> = CArray([11, 12, 13, 14]);
    if (*sData).cardType == CARD_TYPE_FRLG && (*sData).trainerCard.shouldDrawStickers == TRUE {
        i = 0;
        while i < TRAINER_CARD_STICKER_TYPES {
            let mut sticker: u8 = (*sData).trainerCard.stickers[i];
            if (*sData).trainerCard.stickers[i] != 0 {
                WriteSequenceToBgTilemapBuffer(
                    3,
                    i as u16 * 4 + 320,
                    i * 3 + 2,
                    2,
                    2,
                    2,
                    paletteSlots[sticker as i32 - 1],
                    1,
                );
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn LoadStickerGfx() {
    LoadPalette(
        sTrainerCardSticker1_Pal.as_ptr().cast_mut() as *mut c_void,
        176,
        32,
    );
    LoadPalette(
        sTrainerCardSticker2_Pal.as_ptr().cast_mut() as *mut c_void,
        192,
        32,
    );
    LoadPalette(
        sTrainerCardSticker3_Pal.as_ptr().cast_mut() as *mut c_void,
        208,
        32,
    );
    LoadPalette(
        sTrainerCardSticker4_Pal.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
    LoadBgTiles(
        3,
        (*sData).stickerTiles.as_mut_ptr() as *mut c_void,
        1024,
        128,
    );
}
pub(crate) unsafe extern "C" fn DrawTrainerCardWindow(windowId: u8) {
    PutWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn SetCardBgsAndPals() -> u8 {
    'l1: {
        let sw1: u8 = (*sData).bgPalLoadState;
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            LoadBgTiles(3, (*sData).badgeTiles.as_mut_ptr() as *mut c_void, 1024, 0);
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            LoadBgTiles(0, (*sData).cardTiles.as_mut_ptr() as *mut c_void, 0x1800, 0);
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sData).cardType != CARD_TYPE_FRLG {
                LoadPalette(
                    sHoennTrainerCardPals[(*sData).trainerCard.stars] as *mut c_void,
                    0,
                    96,
                );
                LoadPalette(
                    sHoennTrainerCardBadges_Pal.as_ptr().cast_mut() as *mut c_void,
                    48,
                    32,
                );
                if (*sData).trainerCard.gender != MALE {
                    LoadPalette(
                        sHoennTrainerCardFemaleBg_Pal.as_ptr().cast_mut() as *mut c_void,
                        16,
                        32,
                    );
                }
            } else {
                LoadPalette(
                    sKantoTrainerCardPals[(*sData).trainerCard.stars] as *mut c_void,
                    0,
                    96,
                );
                LoadPalette(
                    sKantoTrainerCardBadges_Pal.as_ptr().cast_mut() as *mut c_void,
                    48,
                    32,
                );
                if (*sData).trainerCard.gender != MALE {
                    LoadPalette(
                        sKantoTrainerCardFemaleBg_Pal.as_ptr().cast_mut() as *mut c_void,
                        16,
                        32,
                    );
                }
            }
            LoadPalette(
                sTrainerCardStar_Pal.as_ptr().cast_mut() as *mut c_void,
                64,
                32,
            );
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            SetBgTilemapBuffer(0, (*sData).cardTilemapBuffer.as_mut_ptr() as *mut c_void);
            SetBgTilemapBuffer(2, (*sData).bgTilemapBuffer.as_mut_ptr() as *mut c_void);
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 32, 32);
            FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 32, 32);
        }
        if fall || !matched {
            fall = true;
            return 1;
        }
    }
    (*sData).bgPalLoadState += 1;
    return 0;
}
pub(crate) unsafe extern "C" fn DrawCardScreenBackground(ptr: *mut u16) {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut dst: *mut u16 = (*sData).bgTilemapBuffer.as_mut_ptr();
    i = 0;
    while i < 20 {
        j = 0;
        while j < 32 {
            if j < 30 {
                *dst.at(32 * i as i32 + j as i32) = *ptr.at(30 * i as i32 + j as i32);
            } else {
                *dst.at(32 * i as i32 + j as i32) = *ptr;
            }
            j += 1;
        }
        i += 1;
    }
    CopyBgTilemapBufferToVram(2);
}
pub(crate) unsafe extern "C" fn DrawCardFrontOrBack(ptr: *mut u16) {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut dst: *mut u16 = (*sData).cardTilemapBuffer.as_mut_ptr();
    i = 0;
    while i < 20 {
        j = 0;
        while j < 32 {
            if j < 30 {
                *dst.at(32 * i as i32 + j as i32) = *ptr.at(30 * i as i32 + j as i32);
            } else {
                *dst.at(32 * i as i32 + j as i32) = *ptr;
            }
            j += 1;
        }
        i += 1;
    }
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn DrawStarsAndBadgesOnCard() {
    let mut i: i16 = 0;
    let mut x: i16 = 0;
    let mut tileNum: u16 = 192;
    let mut palNum: u8 = 3;
    FillBgTilemapBufferRect(
        3,
        143,
        15,
        yOffsets_0[(*sData).isHoenn],
        (*sData).trainerCard.stars,
        1,
        4,
    );
    if (*sData).isLink == 0 {
        x = 4;
        i = 0;
        while i < NUM_BADGES as i16 {
            if (*sData).badgeCount[i] != 0 {
                FillBgTilemapBufferRect(3, tileNum, x as u8, 15, 1, 1, palNum);
                FillBgTilemapBufferRect(3, tileNum + 1, x as u8 + 1, 15, 1, 1, palNum);
                FillBgTilemapBufferRect(3, tileNum + 16, x as u8, 16, 1, 1, palNum);
                FillBgTilemapBufferRect(3, tileNum + 17, x as u8 + 1, 16, 1, 1, palNum);
            }
            i += 1;
            tileNum += 2;
            x += 3;
        }
    }
    CopyBgTilemapBufferToVram(3);
}
pub(crate) unsafe extern "C" fn DrawCardBackStats() {
    if (*sData).cardType == CARD_TYPE_FRLG {
        if (*sData).hasTrades != 0 {
            FillBgTilemapBufferRect(3, 141, 27, 9, 1, 1, 1);
            FillBgTilemapBufferRect(3, 157, 27, 10, 1, 1, 1);
        }
        if (*sData).trainerCard.linkPoints.berryCrush != 0 {
            FillBgTilemapBufferRect(3, 141, 21, 13, 1, 1, 1);
            FillBgTilemapBufferRect(3, 157, 21, 14, 1, 1, 1);
        }
        if (*sData).trainerCard.unionRoomNum != 0 {
            FillBgTilemapBufferRect(3, 141, 27, 11, 1, 1, 1);
            FillBgTilemapBufferRect(3, 157, 27, 12, 1, 1, 1);
        }
    } else {
        if (*sData).hasTrades != 0 {
            FillBgTilemapBufferRect(3, 141, 27, 9, 1, 1, 0);
            FillBgTilemapBufferRect(3, 157, 27, 10, 1, 1, 0);
        }
        if (*sData).trainerCard.contestsWithFriends != 0 {
            FillBgTilemapBufferRect(3, 141, 27, 13, 1, 1, 0);
            FillBgTilemapBufferRect(3, 157, 27, 14, 1, 1, 0);
        }
        if (*sData).hasBattleTowerWins != 0 {
            FillBgTilemapBufferRect(3, 141, 17, 15, 1, 1, 0);
            FillBgTilemapBufferRect(3, 157, 17, 16, 1, 1, 0);
            FillBgTilemapBufferRect(3, 140, 27, 15, 1, 1, 0);
            FillBgTilemapBufferRect(3, 156, 27, 16, 1, 1, 0);
        }
    }
    CopyBgTilemapBufferToVram(3);
}
pub(crate) unsafe extern "C" fn BlinkTimeColon() {
    if ({
        (*sData).timeColonBlinkTimer += 1;
        (*sData).timeColonBlinkTimer
    }) > 60
    {
        (*sData).timeColonBlinkTimer = 0;
        (*sData).timeColonInvisible ^= 1;
        (*sData).timeColonNeedDraw = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerCardStars(cardId: u8) -> u8 {
    let mut trainerCards: *mut TrainerCard = gTrainerCards.as_mut_ptr();
    return (*trainerCards.at(cardId)).stars;
}
pub(crate) unsafe extern "C" fn FlipTrainerCard() {
    let mut taskId: u8 = CreateTask(Some(Task_DoCardFlipTask), 0);
    Task_DoCardFlipTask(taskId);
    SetHBlankCallback(Some(HblankCb_TrainerCard));
}
pub(crate) unsafe extern "C" fn IsCardFlipTaskActive() -> u8 {
    if FindTaskIdByFunc(Some(Task_DoCardFlipTask)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_DoCardFlipTask(taskId: u8) {
    while sTrainerCardFlipTasks[gTasks[taskId].data[0]].unwrap_unchecked()(&raw mut gTasks[taskId])
        != 0
    {}
}
pub(crate) unsafe extern "C" fn Task_BeginCardFlip(task: *mut Task) -> u8 {
    let mut i: u32 = 0;
    HideBg(1);
    HideBg(3);
    ScanlineEffect_Stop();
    ScanlineEffect_Clear();
    i = 0;
    while i < DISPLAY_HEIGHT as u32 {
        gScanlineEffectRegBuffers[1][i] = 0;
        i += 1;
    }
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_AnimateCardFlipDown(task: *mut Task) -> u8 {
    let mut cardHeight: u32 = 0;
    let mut r5: u32 = 0;
    let mut r10: u32 = 0;
    let mut cardTop: u32 = 0;
    let mut r6: u32 = 0;
    let mut var_24: u32 = 0;
    let mut cardBottom: u32 = 0;
    let mut var: u32 = 0;
    let mut i: i16 = 0;
    (*sData).allowDMACopy = FALSE;
    if (*task).data[1] >= CARD_FLIP_Y {
        (*task).data[1] = CARD_FLIP_Y;
    } else {
        (*task).data[1] += 7;
    }
    (*sData).cardTop = (*task).data[1] as u16;
    UpdateCardFlipRegs((*task).data[1] as u16);
    cardTop = (*task).data[1] as u32;
    cardBottom = DISPLAY_HEIGHT as u32 - cardTop;
    cardHeight = cardBottom - cardTop;
    r6 = cardTop.wrapping_neg() << 16;
    r5 = div_u32(0xa00000, cardHeight);
    r5 -= 0x10000;
    var_24 = r6;
    var_24 += r5 * cardHeight;
    r10 = div_u32(r5, cardHeight);
    r5 *= 2;
    i = 0;
    while (i as u32) < cardTop {
        gScanlineEffectRegBuffers[0][i] = (i as u16).wrapping_neg();
        i += 1;
    }
    while i < cardBottom as i16 {
        var = r6 >> 16;
        r6 += r5;
        r5 -= r10;
        gScanlineEffectRegBuffers[0][i] = var as u16;
        i += 1;
    }
    var = var_24 >> 16;
    while i < DISPLAY_HEIGHT as i16 {
        gScanlineEffectRegBuffers[0][i] = var as u16;
        i += 1;
    }
    (*sData).allowDMACopy = TRUE;
    if (*task).data[1] >= CARD_FLIP_Y {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_DrawFlippedCardSide(task: *mut Task) -> u8 {
    (*sData).allowDMACopy = FALSE;
    if Overworld_IsRecvQueueAtMax() == TRUE as u32 {
        return FALSE;
    }
    loop {
        match (*sData).flipDrawState {
            0 => {
                FillWindowPixelBuffer(WIN_CARD_TEXT, 0);
                FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 0x20, 0x20);
            }
            1 => {
                if (*sData).onBack == 0 {
                    if PrintAllOnCardBack() == 0 {
                        return FALSE;
                    }
                } else {
                    if PrintAllOnCardFront() == 0 {
                        return FALSE;
                    }
                }
            }
            2 => {
                if (*sData).onBack == 0 {
                    DrawCardFrontOrBack((*sData).backTilemap.as_mut_ptr());
                } else {
                    DrawTrainerCardWindow(WIN_CARD_TEXT);
                }
            }
            3 => {
                if (*sData).onBack == 0 {
                    DrawCardBackStats();
                } else {
                    FillWindowPixelBuffer(WIN_TRAINER_PIC, 0);
                }
            }
            4 => {
                if (*sData).onBack != 0 {
                    CreateTrainerCardTrainerPic();
                }
            }
            _ => {
                (*task).data[0] += 1;
                (*sData).allowDMACopy = TRUE;
                (*sData).flipDrawState = 0;
                return FALSE;
            }
        }
        (*sData).flipDrawState += 1;
        if gReceivedRemoteLinkPlayers != 0 {
            break;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_SetCardFlipped(task: *mut Task) -> u8 {
    (*sData).allowDMACopy = FALSE;
    if (*sData).onBack != 0 {
        DrawTrainerCardWindow(WIN_TRAINER_PIC);
        DrawCardScreenBackground((*sData).bgTilemap.as_mut_ptr());
        DrawCardFrontOrBack((*sData).frontTilemap.as_mut_ptr());
        DrawStarsAndBadgesOnCard();
    }
    DrawTrainerCardWindow(WIN_CARD_TEXT);
    (*sData).onBack ^= 1;
    (*task).data[0] += 1;
    (*sData).allowDMACopy = TRUE;
    PlaySE(SE_RG_CARD_FLIPPING);
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_AnimateCardFlipUp(task: *mut Task) -> u8 {
    let mut cardHeight: u32 = 0;
    let mut r5: u32 = 0;
    let mut r10: u32 = 0;
    let mut cardTop: u32 = 0;
    let mut r6: u32 = 0;
    let mut var_24: u32 = 0;
    let mut cardBottom: u32 = 0;
    let mut var: u32 = 0;
    let mut i: i16 = 0;
    (*sData).allowDMACopy = FALSE;
    if (*task).data[1] <= 5 {
        (*task).data[1] = 0;
    } else {
        (*task).data[1] -= 5;
    }
    (*sData).cardTop = (*task).data[1] as u16;
    UpdateCardFlipRegs((*task).data[1] as u16);
    cardTop = (*task).data[1] as u32;
    cardBottom = DISPLAY_HEIGHT as u32 - cardTop;
    cardHeight = cardBottom - cardTop;
    r6 = cardTop.wrapping_neg() << 16;
    r5 = div_u32(0xa00000, cardHeight);
    r5 -= 0x10000;
    var_24 = r6;
    var_24 += r5 * cardHeight;
    r10 = div_u32(r5, cardHeight);
    r5 = r5 / 2;
    i = 0;
    while (i as u32) < cardTop {
        gScanlineEffectRegBuffers[0][i] = (i as u16).wrapping_neg();
        i += 1;
    }
    while i < cardBottom as i16 {
        var = r6 >> 16;
        r6 += r5;
        r5 += r10;
        gScanlineEffectRegBuffers[0][i] = var as u16;
        i += 1;
    }
    var = var_24 >> 16;
    while i < DISPLAY_HEIGHT as i16 {
        gScanlineEffectRegBuffers[0][i] = var as u16;
        i += 1;
    }
    (*sData).allowDMACopy = TRUE;
    if (*task).data[1] <= 0 {
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_EndCardFlip(task: *mut Task) -> u8 {
    ShowBg(1);
    ShowBg(3);
    SetHBlankCallback(None);
    DestroyTask(FindTaskIdByFunc(Some(Task_DoCardFlipTask)));
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPlayerTrainerCard(callback: Option<unsafe extern "C" fn()>) {
    sData = AllocZeroed(31916) as *mut TrainerCardData;
    (*sData).callback2 = callback;
    if callback == Some(CB2_ReshowFrontierPass as unsafe extern "C" fn()) {
        (*sData).blendColor = 32767;
    } else {
        (*sData).blendColor = 0;
    }
    if InUnionRoom() == TRUE as u32 {
        (*sData).isLink = TRUE;
    } else {
        (*sData).isLink = FALSE;
    }
    (*sData).language = GAME_LANGUAGE;
    TrainerCard_GenerateCardForPlayer(&raw mut (*sData).trainerCard);
    SetMainCallback2(Some(CB2_InitTrainerCard));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerCardInLink(
    cardId: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    sData = AllocZeroed(31916) as *mut TrainerCardData;
    (*sData).callback2 = callback;
    (*sData).isLink = TRUE;
    (*sData).trainerCard = gTrainerCards[cardId];
    (*sData).language = gLinkPlayers[cardId].language as u8;
    SetMainCallback2(Some(CB2_InitTrainerCard));
}
pub(crate) unsafe extern "C" fn InitTrainerCardData() {
    let mut i: u8 = 0;
    (*sData).mainState = 0;
    (*sData).timeColonBlinkTimer = (*gSaveBlock2Ptr).playTimeVBlanks;
    (*sData).timeColonInvisible = FALSE;
    (*sData).onBack = FALSE;
    (*sData).flipBlendY = 0;
    (*sData).cardType = GetSetCardType();
    i = 0;
    while i < TRAINER_CARD_PROFILE_LENGTH {
        CopyEasyChatWord(
            (*sData).easyChatProfile[i].as_mut_ptr(),
            (*sData).trainerCard.easyChatProfile[i],
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetSetCardType() -> u8 {
    if sData.is_null() {
        if gGameVersion == VERSION_FIRE_RED as u8 || gGameVersion == VERSION_LEAF_GREEN as u8 {
            return CARD_TYPE_FRLG;
        } else if gGameVersion == VERSION_EMERALD {
            return CARD_TYPE_EMERALD;
        } else {
            return CARD_TYPE_RS;
        }
    } else {
        if (*sData).trainerCard.version == VERSION_FIRE_RED as u8
            || (*sData).trainerCard.version == VERSION_LEAF_GREEN as u8
        {
            (*sData).isHoenn = FALSE;
            return CARD_TYPE_FRLG;
        } else if (*sData).trainerCard.version == VERSION_EMERALD {
            (*sData).isHoenn = TRUE;
            return CARD_TYPE_EMERALD;
        } else {
            (*sData).isHoenn = TRUE;
            return CARD_TYPE_RS;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn VersionToCardType(version: u8) -> u8 {
    if version == VERSION_FIRE_RED as u8 || version == VERSION_LEAF_GREEN as u8 {
        return CARD_TYPE_FRLG;
    } else if version == VERSION_EMERALD {
        return CARD_TYPE_EMERALD;
    } else {
        return CARD_TYPE_RS;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateTrainerCardTrainerPic() {
    if InUnionRoom() == 1 && gReceivedRemoteLinkPlayers == 1 {
        CreateTrainerCardTrainerPicSprite(
            FacilityClassToPicIndex((*sData).trainerCard.unionRoomClass as u16),
            TRUE,
            sTrainerPicOffset[(*sData).isHoenn][(*sData).trainerCard.gender][0] as u16,
            sTrainerPicOffset[(*sData).isHoenn][(*sData).trainerCard.gender][1] as u16,
            8,
            WIN_TRAINER_PIC,
        );
    } else {
        CreateTrainerCardTrainerPicSprite(
            FacilityClassToPicIndex(
                sTrainerPicFacilityClass[(*sData).cardType][(*sData).trainerCard.gender] as u16,
            ),
            TRUE,
            sTrainerPicOffset[(*sData).isHoenn][(*sData).trainerCard.gender][0] as u16,
            sTrainerPicOffset[(*sData).isHoenn][(*sData).trainerCard.gender][1] as u16,
            8,
            WIN_TRAINER_PIC,
        );
    }
}
