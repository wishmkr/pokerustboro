//! Translated from `src/trainer_card.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sTrainerCardStickers_Gfx sUnused_Pal sHoennTrainerCardBronze_Pal sKantoTrainerCardGreen_Pal sHoennTrainerCardCopper_Pal sKantoTrainerCardBronze_Pal sHoennTrainerCardSilver_Pal sKantoTrainerCardSilver_Pal sHoennTrainerCardGold_Pal sKantoTrainerCardGold_Pal sHoennTrainerCardFemaleBg_Pal sKantoTrainerCardFemaleBg_Pal sHoennTrainerCardBadges_Pal sKantoTrainerCardBadges_Pal sTrainerCardStar_Pal sTrainerCardSticker1_Pal sTrainerCardSticker2_Pal sTrainerCardSticker3_Pal sTrainerCardSticker4_Pal sHoennTrainerCardBadges_Gfx sKantoTrainerCardBadges_Gfx sTrainerCardBgTemplates sTrainerCardWindowTemplates sHoennTrainerCardPals sKantoTrainerCardPals sTrainerCardTextColors sTrainerCardStatColors sTimeColonInvisibleTextColors sTrainerPicOffset sTrainerPicFacilityClass sTrainerCardFlipTasks sTimeColonTextColors sText_HofTime sLinkBattleTexts widths.1 xOffsets.2 yOffsets.0 yOffsetsLine1.4 yOffsetsLine2.3
#[allow(unused_imports)]
use crate::data::trainer_card::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerCards: crate::ffi::Align4<[u8; 400]> = crate::ffi::Align4([0; 400]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gGameVersion: u8;
    static mut gHoennTrainerCardBack_Tilemap: u8;
    static mut gHoennTrainerCardBg_Tilemap: u8;
    static mut gHoennTrainerCardFrontLink_Tilemap: u8;
    static mut gHoennTrainerCardFront_Tilemap: u8;
    static mut gHoennTrainerCard_Gfx: u8;
    static mut gKantoTrainerCardBack_Tilemap: u8;
    static mut gKantoTrainerCardBg_Tilemap: u8;
    static mut gKantoTrainerCardFrontLink_Tilemap: u8;
    static mut gKantoTrainerCardFront_Tilemap: u8;
    static mut gKantoTrainerCard_Gfx: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gMonIconPalettes: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BattlePtsWon: u8;
    static mut gText_BattleTower: u8;
    static mut gText_BerryCrush: u8;
    static mut gText_Colon2: u8;
    static mut gText_EmptyString6: u8;
    static mut gText_HallOfFameDebut: u8;
    static mut gText_NumBP: u8;
    static mut gText_NumPokeblocks: u8;
    static mut gText_PokeblocksWithFriends: u8;
    static mut gText_PokedollarVar1: u8;
    static mut gText_PokemonTrades: u8;
    static mut gText_TrainerCardIDNo: u8;
    static mut gText_TrainerCardMoney: u8;
    static mut gText_TrainerCardName: u8;
    static mut gText_TrainerCardPokedex: u8;
    static mut gText_TrainerCardTime: u8;
    static mut gText_UnionTradesAndBattles: u8;
    static mut gText_Var1sTrainerCard: u8;
    static mut gText_WaitingTrainerFinishReading: u8;
    static mut gText_WinsLosses: u8;
    static mut gText_WinsStraight: u8;
    static mut gText_WonContestsWFriends: u8;
    static mut gUnionRoomFacilityClasses: u8;
    static mut gWirelessCommType: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
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
    fn Free(a0: *mut u8);
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
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
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
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        BlinkTimeColon();
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).read())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                    .cast::<u8>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    ((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>())
                                    .cast::<u8>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HblankCb_TrainerCard() {
    unsafe {
        let mut backup: u16 = 0u16;
        let mut bgVOffset: u16 = 0u16;
        backup = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        bgVOffset = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
            .cast::<u16>())
        .wrapping_offset(
            (((((67108870i32) as usize as *mut u16).read_volatile()) as i32) & 255i32) as isize,
        ))
        .read();
        crate::c::volatile_write(((67108882i32) as usize as *mut u16), bgVOffset);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), backup);
    }
}
pub(crate) unsafe extern "C" fn CB2_TrainerCard() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CloseTrainerCard(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetMainCallback2(
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1328)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        FreeAllWindowBuffers();
        {
            Free(((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sData).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_TrainerCard(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32);
            if __sw1 == 0i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    FillWindowPixelBuffer(1u8, 0u8);
                    let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PrintAllOnCardFront()) != 0 {
                    let __p3 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawTrainerCardWindow(1u8);
                let __p4 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                FillWindowPixelBuffer(2u8, 0u8);
                CreateTrainerCardTrainerPic();
                DrawTrainerCardWindow(2u8);
                let __p5 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                DrawCardScreenBackground(
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3832))
                        .cast::<u16>(),
                );
                let __p6 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                DrawCardFrontOrBack(
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1432))
                        .cast::<u16>(),
                );
                let __p7 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                DrawStarsAndBadgesOnCard();
                let __p8 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 1i32)
                    && (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        == 1i32)
                {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(230u8, 150u8);
                }
                BlendPalettes(
                    4294967295u32,
                    16u8,
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1324)
                        .cast::<u16>())
                    .read(),
                );
                BeginNormalPaletteFade(
                    4294967295u32,
                    0i8,
                    16u8,
                    0u8,
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1324)
                        .cast::<u16>())
                    .read(),
                );
                SetVBlankCallback(Some(VblankCb_TrainerCard));
                let __p9 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (!((UpdatePaletteFade()) != 0)) && (!((IsDma3ManagerBusyWithBgCopy()) != 0)) {
                    PlaySE(251u16);
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((IsSEPlaying()) != 0) {
                    let __p10 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read());
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (!((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0))
                    && ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1321))
                    .read())
                        != 0)
                {
                    PrintTimeOnCard();
                    DrawTrainerCardWindow(1u8);
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1321))
                        .write(0u8);
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    FlipTrainerCard();
                    PlaySE(249u16);
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(12u8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                            && ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(5))
                            .read())
                                != 0))
                            && (InUnionRoom() == 1u32)
                        {
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(15u8);
                        } else {
                            BeginNormalPaletteFade(
                                4294967295u32,
                                0i8,
                                0u8,
                                16u8,
                                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1324)
                                    .cast::<u16>())
                                .read(),
                            );
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(14u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if ((IsCardFlipTaskActive()) != 0) && (Overworld_IsRecvQueueAtMax() != 1u32) {
                    PlaySE(251u16);
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(11u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                        && ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read())
                            != 0))
                        && (InUnionRoom() == 1u32)
                    {
                        (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(15u8);
                    } else {
                        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                            BeginNormalPaletteFade(
                                4294967295u32,
                                0i8,
                                0u8,
                                16u8,
                                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1324)
                                    .cast::<u16>())
                                .read(),
                            );
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(14u8);
                        } else {
                            FlipTrainerCard();
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(13u8);
                            PlaySE(249u16);
                        }
                    }
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                            && ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(5))
                            .read())
                                != 0))
                            && (InUnionRoom() == 1u32)
                        {
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(15u8);
                        } else {
                            BeginNormalPaletteFade(
                                4294967295u32,
                                0i8,
                                0u8,
                                16u8,
                                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1324)
                                    .cast::<u16>())
                                .read(),
                            );
                            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(14u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                SetCloseLinkCallback();
                DrawDialogueFrame(0u8, 1u8);
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (&raw mut gText_WaitingTrainerFinishReading).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(0u8, 3u8);
                (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(16u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    BeginNormalPaletteFade(
                        4294967295u32,
                        0i8,
                        0u8,
                        16u8,
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1324)
                            .cast::<u16>())
                        .read(),
                    );
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(14u8);
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if !((UpdatePaletteFade()) != 0) {
                    CloseTrainerCard(taskId);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if ((IsCardFlipTaskActive()) != 0) && (Overworld_IsRecvQueueAtMax() != 1u32) {
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(10u8);
                    PlaySE(251u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadCardGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    != 0i32
                {
                    LZ77UnCompWram(
                        ((&raw mut gHoennTrainerCardBg_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3832))
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                } else {
                    LZ77UnCompWram(
                        ((&raw mut gKantoTrainerCardBg_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3832))
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    != 0i32
                {
                    LZ77UnCompWram(
                        ((&raw mut gHoennTrainerCardBack_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2632))
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                } else {
                    LZ77UnCompWram(
                        ((&raw mut gKantoTrainerCardBack_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2632))
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                    .read())
                    != 0)
                {
                    if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1322))
                    .read()) as i32)
                        != 0i32
                    {
                        LZ77UnCompWram(
                            ((&raw mut gHoennTrainerCardFront_Tilemap).cast::<u32>()).cast::<u32>(),
                            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1432))
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    } else {
                        LZ77UnCompWram(
                            ((&raw mut gKantoTrainerCardFront_Tilemap).cast::<u32>()).cast::<u32>(),
                            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1432))
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    }
                } else {
                    if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1322))
                    .read()) as i32)
                        != 0i32
                    {
                        LZ77UnCompWram(
                            ((&raw mut gHoennTrainerCardFrontLink_Tilemap).cast::<u32>())
                                .cast::<u32>(),
                            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1432))
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    } else {
                        LZ77UnCompWram(
                            ((&raw mut gKantoTrainerCardFrontLink_Tilemap).cast::<u32>())
                                .cast::<u32>(),
                            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1432))
                            .cast::<u16>())
                            .cast::<u8>(),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    != 0i32
                {
                    LZ77UnCompWram(
                        ((&raw const sHoennTrainerCardBadges_Gfx)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5032))
                        .cast::<u8>(),
                    );
                } else {
                    LZ77UnCompWram(
                        ((&raw const sKantoTrainerCardBadges_Gfx)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5032))
                        .cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    != 0i32
                {
                    LZ77UnCompWram(
                        ((&raw mut gHoennTrainerCard_Gfx).cast::<u32>()).cast::<u32>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6568))
                        .cast::<u8>(),
                    );
                } else {
                    LZ77UnCompWram(
                        ((&raw mut gKantoTrainerCard_Gfx).cast::<u32>()).cast::<u32>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6568))
                        .cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    == 0i32
                {
                    LZ77UnCompWram(
                        ((&raw const sTrainerCardStickers_Gfx)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6056))
                        .cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(0u8);
                return 1u8;
            }
        }
        let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CB2_InitTrainerCard() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ResetGpuRegs();
                SetUpTrainerCardTask();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                'l2: loop {
                    'l3: {
                        {
                            let mut _dest: *mut u32 =
                                ((117440512i32) as usize as *mut u8).cast::<u32>();
                            let mut _size: u32 = 1024u32;
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut tmp: u32 = 0u32;
                                        (&raw mut tmp).write_volatile(0u32);
                                        'l6: loop {
                                            'l7: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2231369728u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(32i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l6;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1324)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    'l8: loop {
                        'l9: {
                            {
                                let mut _dest: *mut u16 =
                                    ((83886080i32) as usize as *mut u8).cast::<u16>();
                                let mut _size: u32 = 1024u32;
                                'l10: loop {
                                    'l11: {
                                        {
                                            let mut tmp: u16 = 0u16;
                                            (&raw mut tmp).write_volatile(0u16);
                                            'l12: loop {
                                                'l13: {
                                                    {
                                                        let mut dmaRegs: *mut u32 =
                                                            ((67109076i32) as usize as *mut u32);
                                                        crate::c::volatile_write(
                                                            dmaRegs,
                                                            ((&raw mut tmp) as usize as u32),
                                                        );
                                                        crate::c::volatile_write(
                                                            (dmaRegs).wrapping_offset(1),
                                                            ((_dest) as usize as u32),
                                                        );
                                                        crate::c::volatile_write(
                                                            (dmaRegs).wrapping_offset(2),
                                                            (2164260864u32
                                                                | crate::c::div_u32(
                                                                    _size,
                                                                    ((crate::c::div_i32(
                                                                        16i32, 8i32,
                                                                    ))
                                                                        as u32),
                                                                )),
                                                        );
                                                        let _ = ((dmaRegs).wrapping_offset(2))
                                                            .read_volatile();
                                                    }
                                                }
                                                if !((0i32) != 0) {
                                                    break 'l12;
                                                }
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l10;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l8;
                        }
                    }
                }
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                ResetSpriteData();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                InitBgsAndWindows();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                LoadMonIconGfx();
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if ((LoadCardGfx()) as i32) == 1i32 {
                    let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                LoadStickerGfx();
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                InitGpuRegs();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                BufferTextsVarsForCardPage2();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                if ((SetCardBgsAndPals()) as i32) == 1i32 {
                    let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                __fall = true;
                SetTrainerCardCb2();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetCappedGameStat(statId: u8, maxValue: u32) -> u32 {
    unsafe {
        let mut statId = statId;
        let mut maxValue = maxValue;
        let mut statValue: u32 = GetGameStat(statId);
        return (if maxValue < statValue {
            maxValue
        } else {
            statValue
        });
    }
}
pub(crate) unsafe extern "C" fn HasAllFrontierSymbols() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    if (!((FlagGet(
                        (((2244i32).wrapping_add((2i32).wrapping_mul(((i) as i32)))) as u16),
                    )) != 0))
                        || (!((FlagGet(
                            (((2245i32).wrapping_add((2i32).wrapping_mul(((i) as i32)))) as u16),
                        )) != 0))
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPlayerTrainerStars() -> u32 {
    unsafe {
        let mut stars: u8 = 0u8;
        if (GetGameStat(10u8)) != 0 {
            stars = (stars).wrapping_add(1);
        }
        if (HasAllHoennMons()) != 0 {
            stars = (stars).wrapping_add(1);
        }
        if ((CountPlayerMuseumPaintings()) as i32) >= 5i32 {
            stars = (stars).wrapping_add(1);
        }
        if (HasAllFrontierSymbols()) != 0 {
            stars = (stars).wrapping_add(1);
        }
        return ((stars) as u32);
    }
}
pub(crate) unsafe extern "C" fn GetRubyTrainerStars(trainerCard: *mut u8) -> u8 {
    unsafe {
        let mut trainerCard = trainerCard;
        let mut stars: u8 = 0u8;
        if (((((trainerCard).wrapping_add(6).cast::<u16>()).read()) != 0)
            || ((((trainerCard).wrapping_add(8).cast::<u16>()).read()) != 0))
            || ((((trainerCard).wrapping_add(10).cast::<u16>()).read()) != 0)
        {
            stars = (stars).wrapping_add(1);
        }
        if (((trainerCard).wrapping_add(3)).read()) != 0 {
            stars = (stars).wrapping_add(1);
        }
        if ((((trainerCard).wrapping_add(26).cast::<u16>()).read()) as i32) > 49i32 {
            stars = (stars).wrapping_add(1);
        }
        if (((trainerCard).wrapping_add(4)).read()) != 0 {
            stars = (stars).wrapping_add(1);
        }
        return stars;
    }
}
pub(crate) unsafe extern "C" fn SetPlayerCardData(trainerCard: *mut u8, cardType: u8) {
    unsafe {
        let mut trainerCard = trainerCard;
        let mut cardType = cardType;
        let mut playTime: u32 = 0u32;
        let mut i: u8 = 0u8;
        (trainerCard)
            .write(((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read());
        ((trainerCard).wrapping_add(16).cast::<u16>()).write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        ((trainerCard).wrapping_add(18).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16)).read())
                as u16),
        );
        playTime = GetGameStat(1u8);
        if !((GetGameStat(10u8)) != 0) {
            playTime = 0u32;
        }
        ((trainerCard).wrapping_add(6).cast::<u16>()).write(((playTime >> 16) as u16));
        ((trainerCard).wrapping_add(8).cast::<u16>()).write((((playTime >> 8) & 255u32) as u16));
        ((trainerCard).wrapping_add(10).cast::<u16>()).write(((playTime & 255u32) as u16));
        if (playTime >> 16) > 999u32 {
            ((trainerCard).wrapping_add(6).cast::<u16>()).write(999u16);
            ((trainerCard).wrapping_add(8).cast::<u16>()).write(59u16);
            ((trainerCard).wrapping_add(10).cast::<u16>()).write(59u16);
        }
        ((trainerCard).wrapping_add(2)).write(FlagGet(2145u16));
        ((trainerCard).wrapping_add(3)).write(((HasAllHoennMons()) as u8));
        ((trainerCard).wrapping_add(12).cast::<u16>()).write(GetCaughtMonsCount());
        ((trainerCard).wrapping_add(14).cast::<u16>()).write(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8)
                | (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .read()) as i32)) as u16),
        );
        ((trainerCard).wrapping_add(20).cast::<u16>())
            .write(((GetCappedGameStat(23u8, 9999u32)) as u16));
        ((trainerCard).wrapping_add(22).cast::<u16>())
            .write(((GetCappedGameStat(24u8, 9999u32)) as u16));
        ((trainerCard).wrapping_add(32).cast::<u16>())
            .write(((GetCappedGameStat(21u8, 65535u32)) as u16));
        ((trainerCard).wrapping_add(36).cast::<u32>()).write(GetMoney(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1168)
                .cast::<u32>(),
        ));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((trainerCard).wrapping_add(40)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11184))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((trainerCard).wrapping_add(48)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        'l3: {
            let __sw1 = ((cardType) as i32);
            let mut __fall = false;
            if __sw1 == 2i32 {
                __fall = true;
                ((trainerCard).wrapping_add(24).cast::<u16>()).write(0u16);
                ((trainerCard).wrapping_add(26).cast::<u16>()).write(0u16);
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                ((trainerCard).wrapping_add(28).cast::<u16>())
                    .write(((GetCappedGameStat(35u8, 999u32)) as u16));
                ((trainerCard).wrapping_add(30).cast::<u16>())
                    .write(((GetCappedGameStat(34u8, 65535u32)) as u16));
                if ((CountPlayerMuseumPaintings()) as i32) >= 5i32 {
                    ((trainerCard).wrapping_add(4)).write(1u8);
                }
                ((trainerCard).wrapping_add(1)).write(GetRubyTrainerStars(trainerCard));
                break 'l3;
            }
            if __sw1 == 1i32 {
                __fall = true;
                ((trainerCard).wrapping_add(24).cast::<u16>()).write(0u16);
                ((trainerCard).wrapping_add(26).cast::<u16>()).write(0u16);
                ((trainerCard).wrapping_add(28).cast::<u16>()).write(0u16);
                ((trainerCard).wrapping_add(30).cast::<u16>()).write(0u16);
                ((trainerCard).wrapping_add(4)).write(0u8);
                ((trainerCard).wrapping_add(1)).write(0u8);
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerCard_GenerateCardForPlayer(trainerCard: *mut u8) {
    unsafe {
        let mut trainerCard = trainerCard;
        crate::c::memset(trainerCard, 0i32, 100u32);
        ((trainerCard).wrapping_add(56)).write(3u8);
        SetPlayerCardData(trainerCard, 2u8);
        ((trainerCard).wrapping_add(96).cast::<u16>()).write(((HasAllFrontierSymbols()) as u16));
        ((trainerCard).wrapping_add(98).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2158)
                .cast::<u16>())
            .read(),
        );
        if (((trainerCard).wrapping_add(96).cast::<u16>()).read()) != 0 {
            let __p1 = (trainerCard).wrapping_add(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if (((trainerCard).read()) as i32) == 1i32 {
            ((trainerCard).wrapping_add(79)).write(
                ((((((&raw mut gUnionRoomFacilityClasses).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::rem_i32(
                            ((((trainerCard).wrapping_add(14).cast::<u16>()).read()) as i32),
                            8i32,
                        ))
                        .wrapping_add(8i32)) as isize,
                    ))
                .read()) as u8),
            );
        } else {
            ((trainerCard).wrapping_add(79)).write(
                ((((((&raw mut gUnionRoomFacilityClasses).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (crate::c::rem_i32(
                            ((((trainerCard).wrapping_add(14).cast::<u16>()).read()) as i32),
                            8i32,
                        )) as isize,
                    ))
                .read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrainerCard_GenerateCardForLinkPlayer(trainerCard: *mut u8) {
    unsafe {
        let mut trainerCard = trainerCard;
        crate::c::memset(trainerCard, 0i32, 96u32);
        ((trainerCard).wrapping_add(56)).write(3u8);
        SetPlayerCardData(trainerCard, 2u8);
        ((trainerCard).wrapping_add(58).cast::<u16>()).write(((HasAllFrontierSymbols()) as u16));
        ((((trainerCard).wrapping_add(60)).cast::<u32>()).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2158)
                .cast::<u16>())
            .read(),
        );
        if (((trainerCard).wrapping_add(58).cast::<u16>()).read()) != 0 {
            let __p1 = (trainerCard).wrapping_add(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if (((trainerCard).read()) as i32) == 1i32 {
            ((trainerCard).wrapping_add(79)).write(
                ((((((&raw mut gUnionRoomFacilityClasses).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::rem_i32(
                            ((((trainerCard).wrapping_add(14).cast::<u16>()).read()) as i32),
                            8i32,
                        ))
                        .wrapping_add(8i32)) as isize,
                    ))
                .read()) as u8),
            );
        } else {
            ((trainerCard).wrapping_add(79)).write(
                ((((((&raw mut gUnionRoomFacilityClasses).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (crate::c::rem_i32(
                            ((((trainerCard).wrapping_add(14).cast::<u16>()).read()) as i32),
                            8i32,
                        )) as isize,
                    ))
                .read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyTrainerCardData(dst: *mut u8, src: *mut u8, gameVersion: u8) {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        let mut gameVersion = gameVersion;
        crate::c::memset(dst, 0i32, 100u32);
        ((dst).wrapping_add(56)).write(gameVersion);
        'l1: {
            let __sw1 = ((VersionToCardType(gameVersion)) as i32);
            if __sw1 == 0i32 {
                crate::c::memcpy(dst, src, 96u32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::memcpy(dst, src, 56u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::memcpy(dst, src, 96u32);
                (((dst).wrapping_add(60)).cast::<u32>()).write(0u32);
                ((dst).wrapping_add(96).cast::<u16>())
                    .write(((src).wrapping_add(58).cast::<u16>()).read());
                ((dst).wrapping_add(98).cast::<u16>())
                    .write(((((src).wrapping_add(60)).cast::<u32>()).cast::<u16>()).read());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDataFromTrainerCard() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut badgeFlag: u32 = 0u32;
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).write(0u8);
        crate::c::memset(
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(17))
                .cast::<u8>(),
            0i32,
            8u32,
        );
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
            .wrapping_add(2))
        .read())
            != 0
        {
            let __p1 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
            .wrapping_add(6)
            .cast::<u16>())
        .read())
            != 0)
            || (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(8)
            .cast::<u16>())
            .read())
                != 0))
            || (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(10)
                .cast::<u16>())
            .read())
                != 0)
        {
            let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
            .wrapping_add(20)
            .cast::<u16>())
        .read())
            != 0)
            || (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(22)
                .cast::<u16>())
            .read())
                != 0)
        {
            let __p3 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
            .wrapping_add(32)
            .cast::<u16>())
        .read())
            != 0
        {
            let __p4 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16);
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
            .wrapping_add(24)
            .cast::<u16>())
        .read())
            != 0)
            || (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(26)
                .cast::<u16>())
            .read())
                != 0)
        {
            let __p5 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        {
            i = 0u8;
            badgeFlag = 2151u32;
            'l1: loop {
                if !(badgeFlag < 2159u32) {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet(((badgeFlag) as u16))) != 0 {
                        let __p6 = (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                badgeFlag = (badgeFlag).wrapping_add(1);
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitGpuRegs() {
    unsafe {
        SetGpuReg(0u8, 12352u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
        SetGpuReg(80u8, 193u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 30u16);
        SetGpuReg(68u8, 160u16);
        SetGpuReg(64u8, 240u16);
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            EnableInterrupts(199u16);
        } else {
            EnableInterrupts(3u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateCardFlipRegs(cardTop: u16) {
    unsafe {
        let mut cardTop = cardTop;
        let mut blendY: i8 =
            ((crate::c::div_i32(((cardTop) as i32).wrapping_add(40i32), 10i32)) as i8);
        if ((blendY) as i32) <= 4i32 {
            blendY = 0i8;
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1320)
            .cast::<i8>())
        .write(blendY);
        SetGpuReg(
            84u8,
            ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1320)
                .cast::<i8>())
            .read()) as u16),
        );
        SetGpuReg(
            68u8,
            (((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(31912)
                .cast::<u16>())
            .read()) as i32)
                << 8)
                | (160i32).wrapping_sub(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(31912)
                        .cast::<u16>())
                    .read()) as i32),
                )) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ResetGpuRegs() {
    unsafe {
        SetVBlankCallback(None);
        SetHBlankCallback(None);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(8u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(14u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn InitBgsAndWindows() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sTrainerCardBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        InitWindows(
            ((&raw const sTrainerCardWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        LoadMessageBoxAndBorderGfx();
    }
}
pub(crate) unsafe extern "C" fn SetTrainerCardCb2() {
    unsafe {
        SetMainCallback2(Some(CB2_TrainerCard));
    }
}
pub(crate) unsafe extern "C" fn SetUpTrainerCardTask() {
    unsafe {
        ResetTasks();
        ScanlineEffect_Stop();
        CreateTask(Some(Task_TrainerCard), 0u8);
        InitTrainerCardData();
        SetDataFromTrainerCard();
    }
}
pub(crate) unsafe extern "C" fn PrintAllOnCardFront() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                PrintNameOnCardFront();
                break 'l1;
            }
            if __sw1 == 1i32 {
                PrintIdOnCard();
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintMoneyOnCard();
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintPokedexOnCard();
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintTimeOnCard();
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintProfilePhraseOnCard();
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                    .write(0u8);
                return 1u8;
            }
        }
        let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PrintAllOnCardBack() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                PrintNameOnCardBack();
                break 'l1;
            }
            if __sw1 == 1i32 {
                PrintHofDebutTimeOnCard();
                break 'l1;
            }
            if __sw1 == 2i32 {
                PrintLinkBattleResultsOnCard();
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintTradesStringOnCard();
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintBerryCrushStringOnCard();
                PrintPokeblockStringOnCard();
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintUnionStringOnCard();
                PrintContestStringOnCard();
                break 'l1;
            }
            if __sw1 == 6i32 {
                PrintPokemonIconsOnCard();
                PrintBattleFacilityStringOnCard();
                break 'l1;
            }
            if __sw1 == 7i32 {
                PrintStickersOnCard();
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                    .write(0u8);
                return 1u8;
            }
        }
        let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BufferTextsVarsForCardPage2() {
    unsafe {
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
}
pub(crate) unsafe extern "C" fn PrintNameOnCardFront() {
    unsafe {
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        txtPtr = StringCopy(
            (&raw mut buffer).cast::<u8>(),
            (&raw mut gText_TrainerCardName).cast::<u8>(),
        );
        StringCopy(
            txtPtr,
            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(48))
            .cast::<u8>(),
        );
        ConvertInternationalString(
            txtPtr,
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31914)).read(),
        );
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322)).read())
            as i32)
            == 0i32
        {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                20u8,
                28u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut buffer).cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                16u8,
                33u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut buffer).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintIdOnCard() {
    unsafe {
        let mut buffer = crate::ffi::Align4([0u8; 32]);
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut xPos: i32 = 0i32;
        let mut top: u32 = 0u32;
        txtPtr = StringCopy(
            (&raw mut buffer).cast::<u8>(),
            (&raw mut gText_TrainerCardIDNo).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            txtPtr,
            (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(14)
                .cast::<u16>())
            .read()) as i32),
            2i32,
            5u8,
        );
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322)).read())
            as i32)
            == 0i32
        {
            xPos = (GetStringCenterAlignXOffset(1i32, (&raw mut buffer).cast::<u8>(), 80i32))
                .wrapping_add(132i32);
            top = 9u32;
        } else {
            xPos = (GetStringCenterAlignXOffset(1i32, (&raw mut buffer).cast::<u8>(), 96i32))
                .wrapping_add(120i32);
            top = 9u32;
        }
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((xPos) as u8),
            ((top) as u8),
            ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut buffer).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMoneyOnCard() {
    unsafe {
        let mut xOffset: i32 = 0i32;
        let mut top: u8 = 0u8;
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
            .read())
            != 0)
        {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                20u8,
                56u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gText_TrainerCardMoney).cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                16u8,
                57u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gText_TrainerCardMoney).cast::<u8>(),
            );
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(36)
                .cast::<u32>())
            .read()) as i32),
            0i32,
            6u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PokedollarVar1).cast::<u8>(),
        );
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
            .read())
            != 0)
        {
            xOffset = GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 144i32);
            top = 56u8;
        } else {
            xOffset = GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 128i32);
            top = 57u8;
        }
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((xOffset) as u8),
            top,
            ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetCaughtMonsCount() -> u16 {
    unsafe {
        if (IsNationalPokedexEnabled()) != 0 {
            return GetNationalPokedexCount(1u8);
        } else {
            return GetHoennPokedexCount(1u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPokedexOnCard() {
    unsafe {
        let mut xOffset: i32 = 0i32;
        let mut top: u8 = 0u8;
        if (FlagGet(2145u16)) != 0 {
            if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                .read())
                != 0)
            {
                AddTextPrinterParameterized3(
                    1u8,
                    1u8,
                    20u8,
                    72u8,
                    ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    (-1i8),
                    (&raw mut gText_TrainerCardPokedex).cast::<u8>(),
                );
            } else {
                AddTextPrinterParameterized3(
                    1u8,
                    1u8,
                    16u8,
                    73u8,
                    ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    (-1i8),
                    (&raw mut gText_TrainerCardPokedex).cast::<u8>(),
                );
            }
            StringCopy(
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .wrapping_add(12)
                    .cast::<u16>())
                    .read()) as i32),
                    0i32,
                    3u8,
                ),
                (&raw mut gText_EmptyString6).cast::<u8>(),
            );
            if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                .read())
                != 0)
            {
                xOffset =
                    GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 144i32);
                top = 72u8;
            } else {
                xOffset =
                    GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 128i32);
                top = 73u8;
            }
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                ((xOffset) as u8),
                top,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gStringVar4).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTimeOnCard() {
    unsafe {
        let mut hours: u16 = 0u16;
        let mut minutes: u16 = 0u16;
        let mut width: i32 = 0i32;
        let mut x: u32 = 0u32;
        let mut y: u32 = 0u32;
        let mut totalWidth: u32 = 0u32;
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
            .read())
            != 0)
        {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                20u8,
                88u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gText_TrainerCardTime).cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                16u8,
                89u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gText_TrainerCardTime).cast::<u8>(),
            );
        }
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).read())
            != 0
        {
            hours = (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(16)
            .cast::<u16>())
            .read();
            minutes = (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(18)
            .cast::<u16>())
            .read();
        } else {
            hours = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read();
            minutes = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as u16);
        }
        if ((hours) as i32) > 999i32 {
            hours = 999u16;
        }
        if ((minutes) as i32) > 59i32 {
            minutes = 59u16;
        }
        width = GetStringWidth(1u8, (&raw mut gText_Colon2).cast::<u8>(), 0i16);
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
            .read())
            != 0)
        {
            x = 144u32;
            y = 88u32;
        } else {
            x = 128u32;
            y = 89u32;
        }
        totalWidth = (((width).wrapping_add(30i32)) as u32);
        x = (x).wrapping_sub(totalWidth);
        FillWindowPixelRect(
            1u8,
            0u8,
            ((x) as u16),
            ((y) as u16),
            ((totalWidth) as u16),
            15u16,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar4).cast::<u8>(),
            ((hours) as i32),
            1i32,
            3u8,
        );
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((x) as u8),
            ((y) as u8),
            ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
        x = (x).wrapping_add(18u32);
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((x) as u8),
            ((y) as u8),
            ((((&raw const sTimeColonTextColors)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                    .read()) as i32) as isize,
            ))
            .read(),
            (-1i8),
            (&raw mut gText_Colon2).cast::<u8>(),
        );
        x = (x).wrapping_add(((width) as u32));
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar4).cast::<u8>(),
            ((minutes) as i32),
            2i32,
            2u8,
        );
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((x) as u8),
            ((y) as u8),
            ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintProfilePhraseOnCard() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).read())
            != 0
        {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                8u8,
                ((((&raw const yOffsetsLine1_4).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize,
                    ))
                .read(),
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .cast::<u8>())
                .cast::<u8>(),
            );
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                (((GetStringWidth(
                    1u8,
                    (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                        .cast::<u8>())
                    .cast::<u8>(),
                    0i16,
                ))
                .wrapping_add(14i32)) as u8),
                ((((&raw const yOffsetsLine1_4).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize,
                    ))
                .read(),
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .cast::<u8>())
                .wrapping_offset(13))
                .cast::<u8>(),
            );
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                8u8,
                ((((&raw const yOffsetsLine2_3).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize,
                    ))
                .read(),
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .cast::<u8>())
                .wrapping_offset(26))
                .cast::<u8>(),
            );
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                (((GetStringWidth(
                    1u8,
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(25))
                    .cast::<u8>())
                    .wrapping_offset(26))
                    .cast::<u8>(),
                    0i16,
                ))
                .wrapping_add(14i32)) as u8),
                ((((&raw const yOffsetsLine2_3).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize,
                    ))
                .read(),
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(25))
                    .cast::<u8>())
                .wrapping_offset(39))
                .cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferNameForCardBack() {
    unsafe {
        StringCopy(
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                .cast::<u8>(),
            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(48))
            .cast::<u8>(),
        );
        ConvertInternationalString(
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                .cast::<u8>(),
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31914)).read(),
        );
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322)).read())
            as i32)
            != 0i32
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                    .cast::<u8>(),
            );
            StringExpandPlaceholders(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                    .cast::<u8>(),
                (&raw mut gText_Var1sTrainerCard).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintNameOnCardBack() {
    unsafe {
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
            .read())
            != 0)
        {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                136u8,
                9u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                    .cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                1u8,
                1u8,
                ((GetStringRightAlignXOffset(
                    1i32,
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                        .cast::<u8>(),
                    216i32,
                )) as u8),
                9u8,
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                    .cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferHofDebutTime() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11)).read())
            != 0
        {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                1i32,
                3u8,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32),
                2i32,
                2u8,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar3).cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32),
                2i32,
                2u8,
            );
            StringExpandPlaceholders(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(147))
                    .cast::<u8>(),
                ((&raw const sText_HofTime).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintStatOnBackOfCard(
    top: u8,
    statName: *mut u8,
    stat: *mut u8,
    color: *mut u8,
) {
    unsafe {
        let mut top = top;
        let mut statName = statName;
        let mut stat = stat;
        let mut color = color;
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((((&raw const xOffsets_2).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                    .read()) as i32) as isize,
            ))
            .read(),
            (((((top) as i32).wrapping_mul(16i32)).wrapping_add(33i32)) as u8),
            ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            statName,
        );
        AddTextPrinterParameterized3(
            1u8,
            1u8,
            ((GetStringRightAlignXOffset(
                1i32,
                stat,
                ((((((&raw const widths_1).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1323))
                    .read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as u8),
            (((((top) as i32).wrapping_mul(16i32)).wrapping_add(33i32)) as u8),
            color,
            (-1i8),
            stat,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintHofDebutTimeOnCard() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11)).read())
            != 0
        {
            PrintStatOnBackOfCard(
                0u8,
                (&raw mut gText_HallOfFameDebut).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(147))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferLinkBattleResults() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12)).read())
            != 0
        {
            StringCopy(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(217))
                    .cast::<u8>(),
                ((((&raw const sLinkBattleTexts)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1322))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(357))
                    .cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(20)
                    .cast::<u16>())
                .read()) as i32),
                0i32,
                4u8,
            );
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(427))
                    .cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(22)
                    .cast::<u16>())
                .read()) as i32),
                0i32,
                4u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintLinkBattleResultsOnCard() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12)).read())
            != 0
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(357))
                    .cast::<u8>(),
            );
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(427))
                    .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_WinsLosses).cast::<u8>(),
            );
            PrintStatOnBackOfCard(
                1u8,
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(217))
                    .cast::<u8>(),
                (&raw mut gStringVar4).cast::<u8>(),
                ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferNumTrades() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).read())
            != 0
        {
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(567))
                    .cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(32)
                    .cast::<u16>())
                .read()) as i32),
                1i32,
                5u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTradesStringOnCard() {
    unsafe {
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16)).read())
            != 0
        {
            PrintStatOnBackOfCard(
                2u8,
                (&raw mut gText_PokemonTrades).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(567))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferBerryCrushPoints() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            == 0i32)
            && ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(60))
            .cast::<u32>())
            .read())
                != 0)
        {
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(707))
                    .cast::<u8>(),
                ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1332))
                .wrapping_add(60))
                .cast::<u32>())
                .read()) as i32),
                1i32,
                5u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBerryCrushStringOnCard() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            == 0i32)
            && ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(60))
            .cast::<u32>())
            .read())
                != 0)
        {
            PrintStatOnBackOfCard(
                4u8,
                (&raw mut gText_BerryCrush).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(707))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferUnionRoomStats() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            == 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(64)
                .cast::<u32>())
            .read())
                != 0)
        {
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(847))
                    .cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(64)
                    .cast::<u32>())
                .read()) as i32),
                1i32,
                5u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintUnionStringOnCard() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            == 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(64)
                .cast::<u32>())
            .read())
                != 0)
        {
            PrintStatOnBackOfCard(
                3u8,
                (&raw mut gText_UnionTradesAndBattles).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(847))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferLinkPokeblocksNum() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(30)
                .cast::<u16>())
            .read())
                != 0)
        {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(30)
                    .cast::<u16>())
                .read()) as i32),
                1i32,
                5u8,
            );
            StringExpandPlaceholders(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(917))
                    .cast::<u8>(),
                (&raw mut gText_NumPokeblocks).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPokeblockStringOnCard() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(30)
                .cast::<u16>())
            .read())
                != 0)
        {
            PrintStatOnBackOfCard(
                3u8,
                (&raw mut gText_PokeblocksWithFriends).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(917))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferLinkContestNum() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(28)
                .cast::<u16>())
            .read())
                != 0)
        {
            ConvertIntToDecimalStringN(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(987))
                    .cast::<u8>(),
                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                    .wrapping_add(28)
                    .cast::<u16>())
                .read()) as i32),
                1i32,
                5u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintContestStringOnCard() {
    unsafe {
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(28)
                .cast::<u16>())
            .read())
                != 0)
        {
            PrintStatOnBackOfCard(
                4u8,
                (&raw mut gText_WonContestsWFriends).cast::<u8>(),
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(987))
                    .cast::<u8>(),
                ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BufferBattleFacilityStats() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1322))
            .read()) as i32);
            if __sw1 == 1i32 {
                if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                    .read())
                    != 0
                {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(24)
                        .cast::<u16>())
                        .read()) as i32),
                        1i32,
                        4u8,
                    );
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar2).cast::<u8>(),
                        (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(26)
                        .cast::<u16>())
                        .read()) as i32),
                        1i32,
                        4u8,
                    );
                    StringExpandPlaceholders(
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1057))
                        .cast::<u8>(),
                        (&raw mut gText_WinsStraight).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1332))
                .wrapping_add(98)
                .cast::<u16>())
                .read())
                    != 0
                {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(98)
                        .cast::<u16>())
                        .read()) as i32),
                        1i32,
                        5u8,
                    );
                    StringExpandPlaceholders(
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1057))
                        .cast::<u8>(),
                        (&raw mut gText_NumBP).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBattleFacilityStringOnCard() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1322))
            .read()) as i32);
            if __sw1 == 1i32 {
                if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                    .read())
                    != 0
                {
                    PrintStatOnBackOfCard(
                        5u8,
                        (&raw mut gText_BattleTower).cast::<u8>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1057))
                        .cast::<u8>(),
                        ((&raw const sTrainerCardTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1332))
                .wrapping_add(98)
                .cast::<u16>())
                .read())
                    != 0
                {
                    PrintStatOnBackOfCard(
                        5u8,
                        (&raw mut gText_BattlePtsWon).cast::<u8>(),
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1057))
                        .cast::<u8>(),
                        ((&raw const sTrainerCardStatColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPokemonIconsOnCard() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut paletteSlots = crate::ffi::Align4([0u8; 6]);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(0)
            .write(5u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(1)
            .write(6u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(2)
            .write(7u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(3)
            .write(8u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(4)
            .write(9u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(5)
            .write(10u8);
        let mut xOffsets = crate::ffi::Align4([0u8; 6]);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(0).write(0u8);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(1).write(4u8);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(2).write(8u8);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(3).write(12u8);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(4).write(16u8);
        (&raw mut xOffsets).cast::<u8>().wrapping_add(5).write(20u8);
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322)).read())
            as i32)
            == 0i32
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(84))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            let mut monSpecies: u8 = GetMonIconPaletteIndexFromSpecies(
                                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1332))
                                .wrapping_add(84))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                            WriteSequenceToBgTilemapBuffer(
                                3u8,
                                ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(224i32))
                                    as u16),
                                (((((((&raw mut xOffsets).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_add(3i32)) as u8),
                                15u8,
                                4u8,
                                4u8,
                                (((&raw mut paletteSlots).cast::<u8>())
                                    .wrapping_offset(((monSpecies) as i32) as isize))
                                .read(),
                                1i16,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMonIconGfx() {
    unsafe {
        let mut i: u8 = 0u8;
        'l1: loop {
            'l2: {
                CpuSet(
                    (&raw mut gMonIconPalettes).cast::<u8>(),
                    (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1128))
                    .cast::<u16>())
                    .cast::<u8>(),
                    96u32,
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l3: {
            let __sw1 = (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1332))
            .wrapping_add(78))
            .read()) as i32);
            if __sw1 == 0i32 {
                break 'l3;
            }
            if __sw1 == 1i32 {
                TintPalette_CustomTone(
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1128))
                        .cast::<u16>(),
                    96u16,
                    0u16,
                    0u16,
                    0u16,
                );
                break 'l3;
            }
            if __sw1 == 2i32 {
                TintPalette_CustomTone(
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1128))
                        .cast::<u16>(),
                    96u16,
                    500u16,
                    330u16,
                    310u16,
                );
                break 'l3;
            }
            if __sw1 == 3i32 {
                TintPalette_SepiaTone(
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1128))
                        .cast::<u16>(),
                    96u16,
                );
                break 'l3;
            }
        }
        LoadPalette(
            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1128))
                .cast::<u16>())
            .cast::<u8>(),
            80u16,
            192u16,
        );
        {
            i = 0u8;
            'l4: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l4;
                }
                'l5: {
                    if ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .wrapping_add(84))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                        != 0
                    {
                        LoadBgTiles(
                            3u8,
                            GetMonIconTiles(
                                (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1332))
                                .wrapping_add(84))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                0u32,
                            ),
                            512u16,
                            ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(32i32)) as u16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintStickersOnCard() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut paletteSlots = crate::ffi::Align4([0u8; 4]);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(0)
            .write(11u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(1)
            .write(12u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(2)
            .write(13u8);
        (&raw mut paletteSlots)
            .cast::<u8>()
            .wrapping_add(3)
            .write(14u8);
        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .read()) as i32)
            == 0i32)
            && ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(76))
            .read()) as i32)
                == 1i32)
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut sticker: u8 =
                            (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1332))
                            .wrapping_add(80))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                        if ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(80))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            WriteSequenceToBgTilemapBuffer(
                                3u8,
                                (((((i) as i32).wrapping_mul(4i32)).wrapping_add(320i32)) as u16),
                                (((((i) as i32).wrapping_mul(3i32)).wrapping_add(2i32)) as u8),
                                2u8,
                                2u8,
                                2u8,
                                (((&raw mut paletteSlots).cast::<u8>()).wrapping_offset(
                                    (((sticker) as i32).wrapping_sub(1i32)) as isize,
                                ))
                                .read(),
                                1i16,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadStickerGfx() {
    unsafe {
        LoadPalette(
            (((&raw const sTrainerCardSticker1_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            176u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTrainerCardSticker2_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            192u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTrainerCardSticker3_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            208u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTrainerCardSticker4_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            32u16,
        );
        LoadBgTiles(
            3u8,
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6056))
                .cast::<u8>(),
            1024u16,
            128u16,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawTrainerCardWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        PutWindowTilemap(windowId);
        CopyWindowToVram(windowId, 3u8);
    }
}
pub(crate) unsafe extern "C" fn SetCardBgsAndPals() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                LoadBgTiles(
                    3u8,
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5032))
                        .cast::<u8>(),
                    ((crate::c::div_u32(1024u32, 1u32)) as u16),
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                LoadBgTiles(
                    0u8,
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6568))
                        .cast::<u8>(),
                    6144u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1322))
                .read()) as i32)
                    != 0i32
                {
                    LoadPalette(
                        (((((&raw const sHoennTrainerCardPals)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(
                            (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1332))
                            .wrapping_add(1))
                            .read()) as i32) as isize,
                        ))
                        .read())
                        .cast::<u8>(),
                        0u16,
                        96u16,
                    );
                    LoadPalette(
                        (((&raw const sHoennTrainerCardBadges_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        48u16,
                        32u16,
                    );
                    if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32)
                        != 0i32
                    {
                        LoadPalette(
                            (((&raw const sHoennTrainerCardFemaleBg_Pal)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .cast::<u8>(),
                            16u16,
                            32u16,
                        );
                    }
                } else {
                    LoadPalette(
                        (((((&raw const sKantoTrainerCardPals)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(
                            (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(1332))
                            .wrapping_add(1))
                            .read()) as i32) as isize,
                        ))
                        .read())
                        .cast::<u8>(),
                        0u16,
                        96u16,
                    );
                    LoadPalette(
                        (((&raw const sKantoTrainerCardBadges_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        48u16,
                        32u16,
                    );
                    if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32)
                        != 0i32
                    {
                        LoadPalette(
                            (((&raw const sKantoTrainerCardFemaleBg_Pal)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .cast::<u8>(),
                            16u16,
                            32u16,
                        );
                    }
                }
                LoadPalette(
                    (((&raw const sTrainerCardStar_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    64u16,
                    32u16,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                SetBgTilemapBuffer(
                    0u8,
                    (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15528))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23720))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(2u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
            }
            if __fall || !__matched {
                __fall = true;
                return 1u8;
            }
        }
        let __p2 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DrawCardScreenBackground(ptr: *mut u16) {
    unsafe {
        let mut ptr = ptr;
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut dst: *mut u16 = ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(23720))
        .cast::<u16>();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i16;
                        'l3: loop {
                            if !(((j) as i32) < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((j) as i32) < 30i32 {
                                    ((dst).wrapping_offset(
                                        (((32i32).wrapping_mul(((i) as i32)))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write(
                                        ((ptr).wrapping_offset(
                                            (((30i32).wrapping_mul(((i) as i32)))
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                } else {
                                    ((dst).wrapping_offset(
                                        (((32i32).wrapping_mul(((i) as i32)))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write((ptr).read());
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DrawCardFrontOrBack(ptr: *mut u16) {
    unsafe {
        let mut ptr = ptr;
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut dst: *mut u16 = ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(15528))
        .cast::<u16>();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i16;
                        'l3: loop {
                            if !(((j) as i32) < 32i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((j) as i32) < 30i32 {
                                    ((dst).wrapping_offset(
                                        (((32i32).wrapping_mul(((i) as i32)))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write(
                                        ((ptr).wrapping_offset(
                                            (((30i32).wrapping_mul(((i) as i32)))
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                } else {
                                    ((dst).wrapping_offset(
                                        (((32i32).wrapping_mul(((i) as i32)))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .write((ptr).read());
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn DrawStarsAndBadgesOnCard() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut tileNum: u16 = 192u16;
        let mut palNum: u8 = 3u8;
        FillBgTilemapBufferRect(
            3u8,
            143u16,
            15u8,
            ((((&raw const yOffsets_0).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                    .read()) as i32) as isize,
            ))
            .read(),
            (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(1))
            .read(),
            1u8,
            4u8,
        );
        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).read())
            != 0)
        {
            x = 4i16;
            {
                i = 0i16;
                'l1: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(17))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            FillBgTilemapBufferRect(
                                3u8,
                                tileNum,
                                ((x) as u8),
                                15u8,
                                1u8,
                                1u8,
                                palNum,
                            );
                            FillBgTilemapBufferRect(
                                3u8,
                                ((((tileNum) as i32).wrapping_add(1i32)) as u16),
                                ((((x) as i32).wrapping_add(1i32)) as u8),
                                15u8,
                                1u8,
                                1u8,
                                palNum,
                            );
                            FillBgTilemapBufferRect(
                                3u8,
                                ((((tileNum) as i32).wrapping_add(16i32)) as u16),
                                ((x) as u8),
                                16u8,
                                1u8,
                                1u8,
                                palNum,
                            );
                            FillBgTilemapBufferRect(
                                3u8,
                                ((((tileNum) as i32).wrapping_add(17i32)) as u16),
                                ((((x) as i32).wrapping_add(1i32)) as u8),
                                16u8,
                                1u8,
                                1u8,
                                palNum,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                    tileNum = ((((tileNum) as i32).wrapping_add(2i32)) as u16);
                    x = ((((x) as i32).wrapping_add(3i32)) as i16);
                }
            }
        }
        CopyBgTilemapBufferToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn DrawCardBackStats() {
    unsafe {
        if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322)).read())
            as i32)
            == 0i32
        {
            if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 27u8, 9u8, 1u8, 1u8, 1u8);
                FillBgTilemapBufferRect(3u8, 157u16, 27u8, 10u8, 1u8, 1u8, 1u8);
            }
            if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(60))
            .cast::<u32>())
            .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 21u8, 13u8, 1u8, 1u8, 1u8);
                FillBgTilemapBufferRect(3u8, 157u16, 21u8, 14u8, 1u8, 1u8, 1u8);
            }
            if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(64)
                .cast::<u32>())
            .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 27u8, 11u8, 1u8, 1u8, 1u8);
                FillBgTilemapBufferRect(3u8, 157u16, 27u8, 12u8, 1u8, 1u8, 1u8);
            }
        } else {
            if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16))
                .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 27u8, 9u8, 1u8, 1u8, 0u8);
                FillBgTilemapBufferRect(3u8, 157u16, 27u8, 10u8, 1u8, 1u8, 0u8);
            }
            if ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(28)
                .cast::<u16>())
            .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 27u8, 13u8, 1u8, 1u8, 0u8);
                FillBgTilemapBufferRect(3u8, 157u16, 27u8, 14u8, 1u8, 1u8, 0u8);
            }
            if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                .read())
                != 0
            {
                FillBgTilemapBufferRect(3u8, 141u16, 17u8, 15u8, 1u8, 1u8, 0u8);
                FillBgTilemapBufferRect(3u8, 157u16, 17u8, 16u8, 1u8, 1u8, 0u8);
                FillBgTilemapBufferRect(3u8, 140u16, 27u8, 15u8, 1u8, 1u8, 0u8);
                FillBgTilemapBufferRect(3u8, 156u16, 27u8, 16u8, 1u8, 1u8, 0u8);
            }
        }
        CopyBgTilemapBufferToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn BlinkTimeColon() {
    unsafe {
        if (({
            let __p1 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 60i32
        {
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6)).write(0u8);
            let __p3 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7);
            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as u8));
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1321))
                .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerCardStars(cardId: u8) -> u8 {
    unsafe {
        let mut cardId = cardId;
        let mut trainerCards: *mut u8 = ((&raw mut gTrainerCards).cast::<u8>()).cast::<u8>();
        return (((trainerCards).wrapping_offset(((cardId) as i32) as isize * 100))
            .wrapping_add(1))
        .read();
    }
}
pub(crate) unsafe extern "C" fn FlipTrainerCard() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_DoCardFlipTask), 0u8);
        Task_DoCardFlipTask(taskId);
        SetHBlankCallback(Some(HblankCb_TrainerCard));
    }
}
pub(crate) unsafe extern "C" fn IsCardFlipTaskActive() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_DoCardFlipTask))) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoCardFlipTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sTrainerCardFlipTasks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BeginCardFlip(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u32 = 0u32;
        HideBg(1u8);
        HideBg(3u8);
        ScanlineEffect_Stop();
        ScanlineEffect_Clear();
        {
            i = 0u32;
            'l1: loop {
                if !(i < 160u32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateCardFlipDown(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut cardHeight: u32 = 0u32;
        let mut r5: u32 = 0u32;
        let mut r10: u32 = 0u32;
        let mut cardTop: u32 = 0u32;
        let mut r6: u32 = 0u32;
        let mut var_24: u32 = 0u32;
        let mut cardBottom: u32 = 0u32;
        let mut var: u32 = 0u32;
        let mut i: i16 = 0i16;
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(0u8);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= (crate::c::div_i32(160i32, 2i32)).wrapping_sub(3i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                .write((((crate::c::div_i32(160i32, 2i32)).wrapping_sub(3i32)) as i16));
        } else {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(7i32)) as i16));
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(31912)
            .cast::<u16>())
        .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16));
        UpdateCardFlipRegs(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        cardTop = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u32);
        cardBottom = (160u32).wrapping_sub(cardTop);
        cardHeight = (cardBottom).wrapping_sub(cardTop);
        r6 = ((cardTop).wrapping_neg() << 16);
        r5 = crate::c::div_u32(10485760u32, cardHeight);
        r5 = (r5).wrapping_sub(65536u32);
        var_24 = r6;
        var_24 = (var_24).wrapping_add((r5).wrapping_mul(cardHeight));
        r10 = crate::c::div_u32(r5, cardHeight);
        r5 = (r5).wrapping_mul(2u32);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as u32) < cardTop) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((((i) as i32).wrapping_neg()) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(((i) as i32) < (((cardBottom) as i16) as i32)) {
                    break 'l3;
                }
                'l4: {
                    var = (r6 >> 16);
                    r6 = (r6).wrapping_add(r5);
                    r5 = (r5).wrapping_sub(r10);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((var) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        var = (var_24 >> 16);
        {
            'l5: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((var) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(1u8);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= (crate::c::div_i32(160i32, 2i32)).wrapping_sub(3i32)
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_DrawFlippedCardSide(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(0u8);
        if Overworld_IsRecvQueueAtMax() == 1u32 {
            return 0u8;
        }
        'l1: loop {
            'l2: {
                'l3: {
                    let __sw1 = ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32);
                    let __matched = __sw1 == 0i32
                        || __sw1 == 1i32
                        || __sw1 == 2i32
                        || __sw1 == 3i32
                        || __sw1 == 4i32;
                    if __sw1 == 0i32 {
                        FillWindowPixelBuffer(1u8, 0u8);
                        FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                        break 'l3;
                    }
                    if __sw1 == 1i32 {
                        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read())
                            != 0)
                        {
                            if !((PrintAllOnCardBack()) != 0) {
                                return 0u8;
                            }
                        } else {
                            if !((PrintAllOnCardFront()) != 0) {
                                return 0u8;
                            }
                        }
                        break 'l3;
                    }
                    if __sw1 == 2i32 {
                        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read())
                            != 0)
                        {
                            DrawCardFrontOrBack(
                                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2632))
                                .cast::<u16>(),
                            );
                        } else {
                            DrawTrainerCardWindow(1u8);
                        }
                        break 'l3;
                    }
                    if __sw1 == 3i32 {
                        if !((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read())
                            != 0)
                        {
                            DrawCardBackStats();
                        } else {
                            FillWindowPixelBuffer(2u8, 0u8);
                        }
                        break 'l3;
                    }
                    if __sw1 == 4i32 {
                        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read())
                            != 0
                        {
                            CreateTrainerCardTrainerPic();
                        }
                        break 'l3;
                    }
                    if !__matched {
                        let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(9))
                        .write(1u8);
                        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .write(0u8);
                        return 0u8;
                    }
                }
                let __p3 =
                    (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            if !(((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32) {
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_SetCardFlipped(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(0u8);
        if (((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).read())
            != 0
        {
            DrawTrainerCardWindow(2u8);
            DrawCardScreenBackground(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3832))
                    .cast::<u16>(),
            );
            DrawCardFrontOrBack(
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1432))
                    .cast::<u16>(),
            );
            DrawStarsAndBadgesOnCard();
        }
        DrawTrainerCardWindow(1u8);
        let __p1 = (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8);
        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
        let __p2 = ((task).wrapping_add(8)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(1u8);
        PlaySE(250u16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateCardFlipUp(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut cardHeight: u32 = 0u32;
        let mut r5: u32 = 0u32;
        let mut r10: u32 = 0u32;
        let mut cardTop: u32 = 0u32;
        let mut r6: u32 = 0u32;
        let mut var_24: u32 = 0u32;
        let mut cardBottom: u32 = 0u32;
        let mut var: u32 = 0u32;
        let mut i: i16 = 0i16;
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(0u8);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) <= 5i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        } else {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(5i32)) as i16));
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(31912)
            .cast::<u16>())
        .write(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16));
        UpdateCardFlipRegs(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        cardTop = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u32);
        cardBottom = (160u32).wrapping_sub(cardTop);
        cardHeight = (cardBottom).wrapping_sub(cardTop);
        r6 = ((cardTop).wrapping_neg() << 16);
        r5 = crate::c::div_u32(10485760u32, cardHeight);
        r5 = (r5).wrapping_sub(65536u32);
        var_24 = r6;
        var_24 = (var_24).wrapping_add((r5).wrapping_mul(cardHeight));
        r10 = crate::c::div_u32(r5, cardHeight);
        r5 = crate::c::div_u32(r5, 2u32);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as u32) < cardTop) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((((i) as i32).wrapping_neg()) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(((i) as i32) < (((cardBottom) as i16) as i32)) {
                    break 'l3;
                }
                'l4: {
                    var = (r6 >> 16);
                    r6 = (r6).wrapping_add(r5);
                    r5 = (r5).wrapping_add(r10);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((var) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        var = (var_24 >> 16);
        {
            'l5: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l5;
                }
                'l6: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((var) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9)).write(1u8);
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) <= 0i32 {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_EndCardFlip(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ShowBg(1u8);
        ShowBg(3u8);
        SetHBlankCallback(None);
        DestroyTask(FindTaskIdByFunc(Some(Task_DoCardFlipTask)));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPlayerTrainerCard(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        ((&raw mut sData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(31916u32));
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1328)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        if core::mem::transmute::<_, usize>(callback)
            == (CB2_ReshowFrontierPass as *const () as usize)
        {
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1324)
                .cast::<u16>())
            .write(32767u16);
        } else {
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1324)
                .cast::<u16>())
            .write(0u16);
        }
        if InUnionRoom() == 1u32 {
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).write(1u8);
        } else {
            ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).write(0u8);
        }
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31914)).write(2u8);
        TrainerCard_GenerateCardForPlayer(
            (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332),
        );
        SetMainCallback2(Some(CB2_InitTrainerCard));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTrainerCardInLink(
    cardId: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut cardId = cardId;
        let mut callback = callback;
        ((&raw mut sData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(31916u32));
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1328)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5)).write(1u8);
        (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1332)
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(
                (((&raw mut gTrainerCards).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((cardId) as i32) as isize * 100)
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(31914)).write(
            ((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((cardId) as i32) as isize * 28))
            .wrapping_add(26)
            .cast::<u16>())
            .read()) as u8),
        );
        SetMainCallback2(Some(CB2_InitTrainerCard));
    }
}
pub(crate) unsafe extern "C" fn InitTrainerCardData() {
    unsafe {
        let mut i: u8 = 0u8;
        (((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6)).write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(18)).read(),
        );
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).write(0u8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1320)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1322))
            .write(GetSetCardType());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    CopyEasyChatWord(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(25))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 13))
                        .cast::<u8>(),
                        (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .wrapping_add(40))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetSetCardType() -> u8 {
    unsafe {
        if ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            if (((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 4i32)
                || (((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 5i32)
            {
                return 0u8;
            } else {
                if ((((&raw mut gGameVersion).cast::<u8>()).read()) as i32) == 3i32 {
                    return 2u8;
                } else {
                    return 1u8;
                }
            }
        } else {
            if ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1332))
                .wrapping_add(56))
            .read()) as i32)
                == 4i32)
                || ((((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1332))
                .wrapping_add(56))
                .read()) as i32)
                    == 5i32)
            {
                ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                    .write(0u8);
                return 0u8;
            } else {
                if (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1332))
                .wrapping_add(56))
                .read()) as i32)
                    == 3i32
                {
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                        .write(1u8);
                    return 2u8;
                } else {
                    ((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1323))
                        .write(1u8);
                    return 1u8;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn VersionToCardType(version: u8) -> u8 {
    unsafe {
        let mut version = version;
        if (((version) as i32) == 4i32) || (((version) as i32) == 5i32) {
            return 0u8;
        } else {
            if ((version) as i32) == 3i32 {
                return 2u8;
            } else {
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTrainerCardTrainerPic() {
    unsafe {
        if (InUnionRoom() == 1u32)
            && (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
        {
            CreateTrainerCardTrainerPicSprite(
                FacilityClassToPicIndex(
                    (((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .wrapping_add(79))
                    .read()) as u16),
                ),
                1u8,
                (((((((((&raw const sTrainerPicOffset).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .read()) as u16),
                ((((((((((&raw const sTrainerPicOffset).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as u16),
                8u8,
                2u8,
            );
        } else {
            CreateTrainerCardTrainerPicSprite(
                FacilityClassToPicIndex(
                    ((((((((&raw const sTrainerPicFacilityClass)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1322))
                        .read()) as i32) as isize
                            * 2,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1332))
                        .read()) as i32) as isize,
                    ))
                    .read()) as u16),
                ),
                1u8,
                (((((((((&raw const sTrainerPicOffset).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .read()) as u16),
                ((((((((((&raw const sTrainerPicOffset).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1323))
                        .read()) as i32) as isize
                            * 4,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1332))
                    .read()) as i32) as isize
                        * 2,
                ))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as u16),
                8u8,
                2u8,
            );
        }
    }
}
