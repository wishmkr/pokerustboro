//! Translated from `src/pokemon_jump.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPokeJumpMons sPokeJumpLeaderFuncs sPokeJumpMemberFuncs sVineBaseSpeeds sVineSpeedDelays sSoundEffects sJumpOffsets sScoreBonuses sPrizeItems sPrizeQuantityData sPokeJumpPal1 sPokeJumpPal2 sVine1_Gfx sVine2_Gfx sVine3_Gfx sVine4_Gfx sStar_Gfx sCompressedSpriteSheets sSpritePalettes sOamData_JumpMon sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sSpriteTemplate_JumpMon sVineYCoords sVineXCoords sSpriteTemplates_Vine sOamData_JumpMon sOamData_Vine16x32 sOamData_Vine32x32 sOamData_Vine32x16 sAnims_Vine_Highest sAnims_Vine_Higher sAnims_Vine_High sAnims_Vine_Low sAnims_Vine_Lower sAnims_Vine_Lowest sAnims_VineTall_Highest sAnims_VineTall_Higher sAnims_VineTall_High sAnims_VineTall_Low sAnims_VineTall_Lower sAnims_VineTall_Lowest sAnims_Vine sAnims_VineTall sSpriteTemplate_Vine1 sSpriteTemplate_Vine2 sSpriteTemplate_Vine3 sSpriteTemplate_Vine4 sOamData_Star sAnim_Star_Still sAnim_Star_Spinning sAnims_Star sSpriteTemplate_Star sInterface_Pal sBg_Pal sBg_Gfx sBg_Tilemap sVenusaur_Pal sVenusaur_Gfx sVenusaur_Tilemap sBonuses_Pal sBonuses_Gfx sBonuses_Tilemap sBgTemplates sWindowTemplates sPokeJumpGfxFuncs sVenusaurStates sSpriteSheet_Digits sSpritePalette_Digits sPlayerNameWindowCoords_2Players sPlayerNameWindowCoords_3Players sPlayerNameWindowCoords_4Players sPlayerNameWindowCoords_5Players sPlayerNameWindowCoords sMonXCoords_2Players sMonXCoords_3Players sMonXCoords_4Players sMonXCoords_5Players sMonXCoords sWindowTemplate_Records sRecordsTexts
#[allow(unused_imports)]
use crate::data::pokemon_jump::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonJump: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonJumpGfx: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gMonFrontPicCoords: u8;
    static mut gMonStillFrontPicTable: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gRfu: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gTasks: u8;
    static mut gText_AwesomeWonF701F700: u8;
    static mut gText_CantHoldMore: u8;
    static mut gText_CommunicationStandby4: u8;
    static mut gText_FilledStorageSpace2: u8;
    static mut gText_PkmnJumpRecords: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_SomeoneDroppedOut2: u8;
    static mut gText_SpacePoints2: u8;
    static mut gText_SpaceTimes3: u8;
    static mut gText_WantToPlayAgain2: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckBagHasSpace(a0: u16, a1: u16) -> u8;
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyItemNameHandlePlural(a0: u16, a1: *mut u8, a2: u32);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyTask(a0: u8);
    fn DigitObjUtil_CreatePrinter(a0: u32, a1: i32, a2: *mut u8) -> u32;
    fn DigitObjUtil_Free();
    fn DigitObjUtil_Init(a0: u32) -> u32;
    fn DigitObjUtil_PrintNumOn(a0: u32, a1: i32);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn EraseYesNoWindow();
    fn FadeOutAndPlayNewMapMusic(a0: u16, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetMultiplayerId() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HandleLoadSpecialPokePic(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMinigameCountdownRunning() -> u32;
    fn IsNotWaitingForBGMStop() -> u8;
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfxOnBg(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn ResetBgPositions();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn Rfu_SendPacket(a0: *mut u8);
    fn RunTasks();
    fn ScriptContext_Enable();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartMinigameCountdown(a0: u16, a1: u16, a2: i16, a3: i16, a4: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartPokemonJump(
    partyId: u16,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut partyId = partyId;
        let mut exitCallback = exitCallback;
        let mut taskId: u8 = 0u8;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            ((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).write(Alloc(33712u32));
            if !(((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
                ResetTasks();
                taskId = CreateTask(Some(Task_StartPokemonJump), 1u8);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(exitCallback);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .write(taskId);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                    .write(GetMultiplayerId());
                InitJumpMonInfo(
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33448))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read()) as i32) as isize
                            * 12,
                    ),
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((partyId) as i32) as isize * 100),
                );
                InitGame(((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read());
                SetWordTaskArg(
                    taskId,
                    2u8,
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()) as usize
                        as u32),
                );
                SetMainCallback2(Some(CB2_PokemonJump));
                return;
            }
        }
        SetMainCallback2(exitCallback);
    }
}
pub(crate) unsafe extern "C" fn FreePokemonJump() {
    unsafe {
        FreeWindowsAndDigitObj();
        Free(((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn InitGame(jump: *mut u8) {
    unsafe {
        let mut jump = jump;
        ((jump).wrapping_add(5)).write(GetLinkPlayerCount());
        ((jump).wrapping_add(112)).write(5u8);
        (((jump).wrapping_add(112)).wrapping_add(2).cast::<u16>()).write(0u16);
        InitPlayerAndJumpTypes();
        ResetForNewGame(jump);
        if ((((jump).wrapping_add(5)).read()) as i32) == 5i32 {
            IncrementGamesWithMaxPlayers();
        }
    }
}
pub(crate) unsafe extern "C" fn ResetForNewGame(jump: *mut u8) {
    unsafe {
        let mut jump = jump;
        let mut i: i32 = 0i32;
        ((jump).wrapping_add(20).cast::<u32>()).write(6u32);
        ((jump).wrapping_add(24).cast::<u32>()).write(6u32);
        ((jump).wrapping_add(74).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(28).cast::<i32>()).write(0i32);
        ((jump).wrapping_add(92).cast::<u32>()).write(0u32);
        ((jump).wrapping_add(71)).write(((((GetMultiplayerId()) as i32) == 0i32) as u8));
        ((jump).wrapping_add(8).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(10).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(12).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(14).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(88).cast::<u32>()).write(0u32);
        ((jump).wrapping_add(58).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(68)).write(0u8);
        ((jump).wrapping_add(84).cast::<i32>()).write(0i32);
        ((jump).wrapping_add(70)).write(0u8);
        ((jump).wrapping_add(73)).write(0u8);
        ((jump).wrapping_add(72)).write(1u8);
        (((jump).wrapping_add(112)).wrapping_add(8).cast::<u32>()).write(0u32);
        (((jump).wrapping_add(112)).wrapping_add(1)).write(0u8);
        (((jump).wrapping_add(112)).wrapping_add(4).cast::<u16>()).write(0u16);
        ((jump).wrapping_add(96).cast::<u32>()).write(1u32);
        ((jump).wrapping_add(77)).write(0u8);
        ((jump).wrapping_add(104).cast::<u32>()).write(0u32);
        ((jump).wrapping_add(100).cast::<u32>()).write(0u32);
        ((jump).wrapping_add(44).cast::<i32>()).write(0i32);
        ((jump).wrapping_add(48).cast::<u32>()).write(0u32);
        ResetPlayersForNewGame();
        ResetPlayersJumpStates();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((((jump).wrapping_add(124)).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(0u8);
                    ((((jump).wrapping_add(154)).cast::<u16>()).wrapping_offset((i) as isize))
                        .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPlayerAndJumpTypes() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut index: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    index = ((GetPokemonJumpSpeciesIdx(
                        (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33448))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .cast::<u16>())
                        .read(),
                    )) as i32);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(12)
                    .cast::<u16>())
                    .write(
                        (((((&raw const sPokeJumpMons).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((index) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .write(
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33508))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32) as isize
                    * 40,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn ResetPlayersForNewGame() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(14)
                    .cast::<u16>())
                    .write(0u16);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .write(0u16);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(18)
                    .cast::<u16>())
                    .write(0u16);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .cast::<i32>())
                    .write(0i32);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(2147483647i32);
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(20)
                    .cast::<i32>())
                    .write(0i32);
                    ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(139))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(9u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetPokemonJumpSpeciesIdx(species: u16) -> i16 {
    unsafe {
        let mut species = species;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(400u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sPokeJumpMons).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((species) as i32)
                    {
                        return ((i) as i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i16);
    }
}
pub(crate) unsafe extern "C" fn InitJumpMonInfo(monInfo: *mut u8, mon: *mut u8) {
    unsafe {
        let mut monInfo = monInfo;
        let mut mon = mon;
        ((monInfo).cast::<u16>()).write(((GetMonData2(mon, 11i32)) as u16));
        ((monInfo).wrapping_add(4).cast::<u32>()).write(GetMonData2(mon, 1i32));
        ((monInfo).wrapping_add(8).cast::<u32>()).write(GetMonData2(mon, 0i32));
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokemonJump() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
    }
}
pub(crate) unsafe extern "C" fn CB2_PokemonJump() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn SetPokeJumpTask(func: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut func = func;
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(CreateTask(func, 1u8));
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_StartPokemonJump(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ResetSpriteData();
                FreeAllSpritePalettes();
                SetTaskWithPokeJumpStruct(Some(Task_CommunicateMonInfo), 5u8);
                FadeOutMapMusic(4u8);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FuncIsActiveTask(Some(Task_CommunicateMonInfo))) != 0) {
                    StartPokeJumpGfx(
                        (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(164),
                    );
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (!((IsPokeJumpGfxFuncFinished()) != 0))
                    && (((IsNotWaitingForBGMStop()) as i32) == 1i32)
                {
                    FadeOutAndPlayNewMapMusic(538u16, 8u8);
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    BlendPalettes(4294967295u32, 16u8, 0u16);
                    BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                    SetVBlankCallback(Some(VBlankCB_PokemonJump));
                    let __p5 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                UpdatePaletteFade();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .write(0u8);
                    let __p6 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                let __p7 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7);
                (__p7).write(((__p7).read()).wrapping_add(1));
                if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .read()) as i32)
                    >= 20i32
                {
                    if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(71))
                    .read())
                        != 0
                    {
                        SetPokeJumpTask(Some(Task_PokemonJump_Leader));
                    } else {
                        SetPokeJumpTask(Some(Task_PokemonJump_Member));
                    }
                    InitVineState();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetLinkTimeInterval(intervalId: i32) {
    unsafe {
        let mut intervalId = intervalId;
        if intervalId == 0i32 {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48)
                .cast::<u32>())
            .write(4369u32);
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>())
            .write(1i32);
        } else {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48)
                .cast::<u32>())
            .write(
                (((crate::c::shl_i32(1i32, (((intervalId).wrapping_sub(1i32)) as u32)))
                    .wrapping_sub(1i32)) as u32),
            );
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>())
            .write(0i32);
        }
    }
}
pub(crate) unsafe extern "C" fn SetFunc_Leader(funcId: u8) {
    unsafe {
        let mut funcId = funcId;
        let mut i: i32 = 0i32;
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
            .write(funcId);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
            .write(1u8);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(73))
            .write(0u8);
        {
            i = 1i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(24)
                    .cast::<u32>())
                    .write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Leader() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numReady: i32 = 0i32;
        let mut monState: u16 = 0u16;
        let mut funcId: u8 = 0u8;
        let mut playAgainState: u16 = 0u16;
        {
            i = 1i32;
            numReady = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    monState = (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .read();
                    if (RecvPacket_MemberStateToLeader(
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33508))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 40),
                        i,
                        &raw mut funcId,
                        &raw mut playAgainState,
                    )) != 0
                    {
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(playAgainState);
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(139))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(funcId);
                        (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(33508))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 40))
                        .wrapping_add(18)
                        .cast::<u16>())
                        .write(monState);
                    }
                    if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(24)
                    .cast::<u32>())
                    .read())
                        != 0)
                        && (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(139))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(112))
                            .read()) as i32))
                    {
                        numReady = (numReady).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if numReady
            == ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5))
            .read()) as i32)
                .wrapping_sub(1i32)
        {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(73))
                .write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonJump_Leader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RecvLinkData_Leader();
        TryUpdateScore();
        if (!((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(72))
        .read())
            != 0))
            && ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(73))
            .read())
                != 0)
        {
            SetFunc_Leader(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76))
                .read(),
            );
            SetLinkTimeInterval(3i32);
        }
        if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
            .read()) as i32)
            == 1i32
        {
            if !(((((((&raw const sPokeJumpLeaderFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn() -> u32>>())
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
            .wrapping_offset(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()())
                != 0)
            {
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72))
                .write(0u8);
                (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33508))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(24)
                .cast::<u32>())
                .write(1u32);
            }
        }
        UpdateGame();
        SendLinkData_Leader();
    }
}
pub(crate) unsafe extern "C" fn SendLinkData_Leader() {
    unsafe {
        if !((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<i32>())
        .read())
            != 0)
        {
            SendPacket_LeaderState(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33508))
                .cast::<u8>(),
                (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112),
            );
        }
        if ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48)
            .cast::<u32>())
        .read()
            != 4369u32
        {
            let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>();
            (__p2).write(
                (((((__p2).read()) as u32)
                    & ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(48)
                        .cast::<u32>())
                    .read()) as i32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetFunc_Member(funcId: u8) {
    unsafe {
        let mut funcId = funcId;
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
            .write(funcId);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
            .write(1u8);
        (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33508))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(24)
        .cast::<u32>())
        .write(0u32);
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData_Member() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut monState: u16 = 0u16;
        let mut leaderData = crate::ffi::Align4([0u8; 12]);
        monState = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33508))
        .cast::<u8>())
        .wrapping_add(16)
        .cast::<u16>())
        .read();
        if (RecvPacket_LeaderState(
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(33508))
                .cast::<u8>(),
            (&raw mut leaderData).cast::<u8>(),
        )) != 0
        {
            if ((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33508))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(24)
            .cast::<u32>())
            .read()
                == 1u32)
                && (((((&raw mut leaderData).cast::<u8>()).read()) as i32)
                    != ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .read()) as i32))
            {
                SetFunc_Member(((&raw mut leaderData).cast::<u8>()).read());
            }
            if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(112))
            .wrapping_add(8)
            .cast::<u32>())
            .read()
                != (((&raw mut leaderData).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read()
            {
                (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(8)
                .cast::<u32>())
                .write(
                    (((&raw mut leaderData).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .read(),
                );
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<u32>())
                .write(1u32);
                (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(1))
                .write((((&raw mut leaderData).cast::<u8>()).wrapping_add(1)).read());
                if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(1))
                .read())
                    != 0
                {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(77))
                    .write(1u8);
                } else {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(77))
                    .write(0u8);
                }
            }
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .wrapping_add(2)
                .cast::<u16>())
            .write(
                (((&raw mut leaderData).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read(),
            );
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .wrapping_add(4)
                .cast::<u16>())
            .write(
                (((&raw mut leaderData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read(),
            );
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33508))
            .cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
            .write(monState);
        }
        {
            i = 1i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if i != ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32)
                    {
                        monState =
                            (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read();
                        if (RecvPacket_MemberStateToMember(
                            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40),
                            i,
                        )) != 0
                        {
                            (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .write(monState);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonJump_Member(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RecvLinkData_Member();
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(72))
            .read())
            != 0
        {
            if !(((((((&raw const sPokeJumpMemberFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn() -> u32>>())
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
            .wrapping_offset(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()())
                != 0)
            {
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72))
                .write(0u8);
                (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33508))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(24)
                .cast::<u32>())
                .write(1u32);
                SetLinkTimeInterval(3i32);
            }
        }
        UpdateGame();
        SendLinkData_Member();
    }
}
pub(crate) unsafe extern "C" fn SendLinkData_Member() {
    unsafe {
        if !((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<i32>())
        .read())
            != 0)
        {
            SendPacket_MemberState(
                (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33508))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32) as isize
                        * 40,
                ),
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .read(),
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(66)
                    .cast::<u16>())
                .read(),
            );
        }
        if ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48)
            .cast::<u32>())
        .read()
            != 4369u32
        {
            let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<i32>();
            (__p2).write(
                (((((__p2).read()) as u32)
                    & ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(48)
                        .cast::<u32>())
                    .read()) as i32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GameIntro_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetLinkTimeInterval(3i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((DoGameIntro()) != 0) {
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74)
                            .cast::<u16>())
                        .read(),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .write(1u8);
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GameIntro_Member() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetLinkTimeInterval(0i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<u32>())
                .write(
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as u32),
                );
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                return DoGameIntro();
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn WaitRound_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ResetPlayersJumpStates();
                SetLinkTimeInterval(5i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(73))
                .read())
                    != 0
                {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .write(2u8);
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn WaitRound_Member() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ResetPlayersJumpStates();
                SetLinkTimeInterval(0i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74)
                    .cast::<u16>())
                .write(
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read(),
                );
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (AreLinkQueuesEmpty()) != 0 {
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GameRound_Leader() -> u32 {
    unsafe {
        if !((HandleSwingRound()) != 0) {
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .wrapping_add(2)
                .cast::<u16>())
            .write(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(76))
                .write(1u8);
        } else {
            if (UpdateVineHitStates()) != 0 {
                return 1u32;
            } else {
                ResetVineAfterHit();
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76))
                .write(3u8);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GameRound_Member() -> u32 {
    unsafe {
        if !((HandleSwingRound()) != 0) {
        } else {
            if (UpdateVineHitStates()) != 0 {
                return 1u32;
            } else {
                ResetVineAfterHit();
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GameOver_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                UpdateVineHitStates();
                if (AllPlayersJumpedOrHit()) != 0 {
                    let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoVineHitEffect()) != 0) {
                    if (HasEnoughScoreForPrize()) != 0 {
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(GetPrizeData());
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(76))
                        .write(7u8);
                    } else {
                        if (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32)
                            >= 200i32
                        {
                            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(112))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(14)
                                    .cast::<u16>())
                                .read(),
                            );
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(76))
                            .write(8u8);
                        } else {
                            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(112))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(14)
                                    .cast::<u16>())
                                .read(),
                            );
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(76))
                            .write(4u8);
                        }
                    }
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GameOver_Member() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((UpdateVineHitStates()) != 0) {
                    ResetVineAfterHit();
                }
                if (AllPlayersJumpedOrHit()) != 0 {
                    let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DoVineHitEffect()) != 0) {
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn AskPlayAgain_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetLinkTimeInterval(4i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((DoPlayAgainPrompt()) != 0) {
                    TryUpdateRecords(
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(8)
                        .cast::<u32>())
                        .read(),
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read(),
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(73))
                .read())
                    != 0
                {
                    if (ShouldPlayAgain()) != 0 {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(76))
                        .write(5u8);
                    } else {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(76))
                        .write(6u8);
                    }
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn AskPlayAgain_Member() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                SetLinkTimeInterval(0i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((DoPlayAgainPrompt()) != 0) {
                    TryUpdateRecords(
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(8)
                        .cast::<u32>())
                        .read(),
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read(),
                        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(66)
                        .cast::<u16>())
                    .write(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(69))
                        .read()) as u16),
                    );
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ResetGame_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((CloseMessageAndResetScore()) != 0) {
                    let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(73))
                .read())
                    != 0
                {
                    ResetForNewGame(
                        ((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36)
                        .cast::<u32>())
                    .write(((Random()) as u32));
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<u32>())
                        .read()) as u16),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .write(0u8);
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ResetGame_Member() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((CloseMessageAndResetScore()) != 0) {
                    ResetForNewGame(
                        ((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ExitGame() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>())
                .write(1u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetLinkTimeInterval(0i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((ClosePokeJumpLink()) != 0) {
                    SetMainCallback2(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    FreePokemonJump();
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GivePrize_Leader() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetLinkTimeInterval(4i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((TryGivePrize()) != 0) {
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14)
                            .cast::<u16>())
                        .read(),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .write(8u8);
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GivePrize_Member() -> u32 {
    unsafe {
        SetLinkTimeInterval(0i32);
        if !((TryGivePrize()) != 0) {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn SavePokeJump() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                TryUpdateRecords(
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read(),
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read(),
                );
                SetUpPokeJumpGfxFuncById(5i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    SetLinkTimeInterval(0i32);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (AreLinkQueuesEmpty()) != 0 {
                    CreateTask(Some(Task_LinkFullSave), 6u8);
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((FuncIsActiveTask(Some(Task_LinkFullSave))) != 0) {
                    ClearMessageWindow();
                    let __p5 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((RemoveMessageWindow()) != 0) {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76))
                    .write(4u8);
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn DoGameIntro() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetUpPokeJumpGfxFuncById(2i32);
                ResetMonSpriteSubpriorities();
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    StartMonIntroBounce(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read()) as i32),
                    );
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>())
                    .write(0u16);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 120i32
                {
                    SetUpPokeJumpGfxFuncById(3i32);
                    let __p6 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsPokeJumpGfxFuncFinished() != 1u32) && (IsMonIntroBounceActive() != 1i32) {
                    let __p7 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetUpPokeJumpGfxFuncById(9i32);
                let __p8 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    DisallowVineUpdates();
                    SetUpResetVineGfx();
                    let __p9 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((ResetVineGfx()) != 0) {
                    AllowVineUpdates();
                    ResetVineState();
                    let __p10 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn HandleSwingRound() -> u32 {
    unsafe {
        UpdateVineState();
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(54)
            .cast::<u16>())
        .read())
            != 0
        {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(54)
                .cast::<u16>())
            .write(0u16);
            return 0u32;
        }
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if (IsPlayersMonState(0u16)) != 0 {
                    let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    SetMonStateJump();
                    SetLinkTimeInterval(3i32);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if IsPlayersMonState(1u16) == 1u32 {
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if IsPlayersMonState(0u16) == 1u32 {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .write(0u16);
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn DoVineHitEffect() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i
                            < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(5))
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            if IsMonHitShakeActive(i) == 1i32 {
                                return 1u32;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i
                            < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(5))
                            .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read()) as i32)
                                == 2i32
                            {
                                StartMonHitFlash(((i) as u8));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SetUpPokeJumpGfxFuncById(1i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60)
                    .cast::<u16>())
                .write(0u16);
                let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 100i32
                {
                    SetUpPokeJumpGfxFuncById(3i32);
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>())
                    .write(0u16);
                    let __p6 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    StopMonHitFlash();
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(1))
                    .write(0u8);
                    ResetPlayersMonState();
                    let __p7 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn TryGivePrize() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                UnpackPrizeData(
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read(),
                    (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(62)
                        .cast::<u16>(),
                    (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(64)
                        .cast::<u16>(),
                );
                PrintPrizeMessage(
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(62)
                        .cast::<u16>())
                    .read(),
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(64)
                        .cast::<u16>())
                    .read(),
                );
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 4i32 {
                if !((DoPrizeMessageAndFanfare()) != 0) {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>())
                    .write(0u16);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 5i32 {
                let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0)
                    || (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>())
                    .read()) as i32)
                        > 180i32)
                {
                    ClearMessageWindow();
                    let __p5 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((RemoveMessageWindow()) != 0) {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(64)
                        .cast::<u16>())
                    .write(GetQuantityLimitedByBag(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(62)
                            .cast::<u16>())
                        .read(),
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(64)
                            .cast::<u16>())
                        .read(),
                    ));
                    if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(64)
                        .cast::<u16>())
                    .read())
                        != 0)
                        && ((AddBagItem(
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(62)
                                .cast::<u16>())
                            .read(),
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(64)
                                .cast::<u16>())
                            .read(),
                        )) != 0)
                    {
                        if !((CheckBagHasSpace(
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(62)
                                .cast::<u16>())
                            .read(),
                            1u16,
                        )) != 0)
                        {
                            PrintPrizeFilledBagMessage(
                                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(62)
                                    .cast::<u16>())
                                .read(),
                            );
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(10)
                                .cast::<u16>())
                            .write(4u16);
                        } else {
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(10)
                                .cast::<u16>())
                            .write(6u16);
                            break 'l1;
                        }
                    } else {
                        PrintNoRoomForPrizeMessage(
                            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(62)
                                .cast::<u16>())
                            .read(),
                        );
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10)
                            .cast::<u16>())
                        .write(4u16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((RemoveMessageWindow()) != 0) {
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn DoPlayAgainPrompt() -> u32 {
    unsafe {
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetUpPokeJumpGfxFuncById(4i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = HandlePlayAgainInput();
                'l2: {
                    let __sw4 = ((input) as i32);
                    if __sw4 == (-1i32) || __sw4 == 1i32 {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(69))
                        .write(1u8);
                        SetUpPokeJumpGfxFuncById(6i32);
                        let __p5 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(10)
                        .cast::<u16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        break 'l2;
                    }
                    if __sw4 == 0i32 {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(69))
                        .write(2u8);
                        SetUpPokeJumpGfxFuncById(6i32);
                        let __p6 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(10)
                        .cast::<u16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    let __p7 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetUpPokeJumpGfxFuncById(8i32);
                let __p8 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    let __p9 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ClosePokeJumpLink() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ClearMessageWindow();
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RemoveMessageWindow()) != 0) {
                    SetUpPokeJumpGfxFuncById(7i32);
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsPokeJumpGfxFuncFinished()) != 0) {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>())
                    .write(0u16);
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p5 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<u16>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 120i32
                {
                    BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                    let __p7 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetCloseLinkCallback();
                    let __p8 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    return 0u32;
                }
                break 'l1;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn CloseMessageAndResetScore() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ClearMessageWindow();
                PrintScore(0i32);
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RemoveMessageWindow()) != 0) {
                    let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Task_CommunicateMonInfo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut jump: *mut u8 = ((GetWordTaskArg(taskId, 14u8)) as usize as *mut u8);
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 5i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((data).wrapping_offset(((i).wrapping_add(2i32)) as isize)).write(0i16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                SendPacket_MonInfo(
                    (((jump).wrapping_add(33448)).cast::<u8>())
                        .wrapping_offset(((((jump).wrapping_add(6)).read()) as i32) as isize * 12),
                );
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 5i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (!((((data).wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                .read())
                                != 0))
                                && ((RecvPacket_MonInfo(
                                    i,
                                    (((jump).wrapping_add(33448)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 12),
                                )) != 0)
                            {
                                StringCopy(
                                    (((((jump).wrapping_add(33508)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 40))
                                    .wrapping_add(28))
                                    .cast::<u8>(),
                                    ((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(8))
                                    .cast::<u8>(),
                                );
                                ((data).wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                    .write(1i16);
                                let __p2 = (data).wrapping_offset(1);
                                (__p2).write(((__p2).read()).wrapping_add(1));
                                if ((((data).wrapping_offset(1)).read()) as i32)
                                    == ((((jump).wrapping_add(5)).read()) as i32)
                                {
                                    InitPlayerAndJumpTypes();
                                    DestroyTask(taskId);
                                    break 'l4;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTaskWithPokeJumpStruct(
    func: Option<unsafe extern "C" fn(u8)>,
    taskPriority: u8,
) {
    unsafe {
        let mut func = func;
        let mut taskPriority = taskPriority;
        let mut taskId: u8 = CreateTask(func, taskPriority);
        SetWordTaskArg(
            taskId,
            14u8,
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()) as usize as u32),
        );
    }
}
pub(crate) unsafe extern "C" fn InitVineState() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .write(6u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<i32>())
        .write(0i32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(54)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u32>())
        .write(0u32);
    }
}
pub(crate) unsafe extern "C" fn ResetVineState() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u16>())
        .write(1791u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .write(7u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(54)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u32>())
        .write(0u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(81))
            .write(0u8);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(80))
            .write(0u8);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<u32>())
        .write(0u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(78)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(108)
            .cast::<u32>())
        .write(0u32);
        UpdateVineSpeed();
    }
}
pub(crate) unsafe extern "C" fn UpdateVineState() {
    unsafe {
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(70))
            .read())
            != 0
        {
            let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<u16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(GetVineSpeed())) as u16));
            if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<u16>())
            .read()) as i32)
                >= 2559i32
            {
                let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<u16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(2559i32)) as u16));
            }
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<u32>())
            .write(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<u32>())
                .read(),
            );
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<u32>())
            .write(
                ((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<u16>())
                .read()) as i32)
                    >> 8) as u32),
            );
            if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<u32>())
            .read()
                > 6u32)
                && (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u32>())
                .read()
                    < 7u32)
            {
                let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                UpdateVineSpeed();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetVineSpeed() -> i32 {
    unsafe {
        let mut speed: i32 = 0i32;
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u32>())
        .read())
            != 0
        {
            return 0i32;
        }
        speed = ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<i32>())
        .read();
        if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u16>())
        .read()) as i32)
            <= 1535i32
        {
            let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(80u32));
            speed = ((((speed) as u32).wrapping_add(crate::c::div_u32(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<u32>())
                .read(),
                256u32,
            ))) as i32);
        }
        return speed;
    }
}
pub(crate) unsafe extern "C" fn UpdateVineSpeed() {
    unsafe {
        let mut baseSpeed: i32 = 0i32;
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<u32>())
        .write(0u32);
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(78)
            .cast::<u16>())
        .read())
            != 0
        {
            let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(78)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(108)
                .cast::<u32>())
            .read())
                != 0
            {
                if (crate::c::rem_i32(PokeJumpRandom(), 4i32)) != 0 {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<i32>())
                    .write(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40)
                            .cast::<u32>())
                        .read()) as i32),
                    );
                } else {
                    if ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40)
                        .cast::<u32>())
                    .read()
                        > 54u32
                    {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<i32>())
                        .write(30i32);
                    } else {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<i32>())
                        .write(82i32);
                    }
                }
            }
        } else {
            if !((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80))
            .read()) as u32)
                & crate::c::div_u32(16u32, 2u32))
                != 0)
            {
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<u32>())
                .write(
                    ((((((((&raw const sVineBaseSpeeds)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(81))
                            .read()) as i32)
                                .wrapping_mul(7i32),
                        )) as u32),
                );
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(78)
                    .cast::<u16>())
                .write(
                    ((((((((&raw const sVineSpeedDelays)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(
                        ((crate::c::rem_u32(
                            ((PokeJumpRandom()) as u32),
                            crate::c::div_u32(8u32, 2u32),
                        )) as i32) as isize,
                    ))
                    .read()) as i32)
                        .wrapping_add(2i32)) as u16),
                );
                let __p2 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(80);
                (__p2).write(((__p2).read()).wrapping_add(1));
            } else {
                if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(80))
                .read()) as u32)
                    == crate::c::div_u32(16u32, 2u32)
                {
                    if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(81))
                    .read()) as i32)
                        < 3i32
                    {
                        let __p3 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(81);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(108)
                            .cast::<u32>())
                        .write(1u32);
                    }
                }
                baseSpeed = ((((((&raw const sVineBaseSpeeds)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((15i32).wrapping_sub(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .read()) as i32),
                    )) as isize,
                ))
                .read()) as i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<u32>())
                .write(
                    (((baseSpeed).wrapping_add(
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(81))
                        .read()) as i32)
                            .wrapping_mul(7i32),
                    )) as u32),
                );
                if (({
                    let __p4 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(80);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 15i32
                {
                    if crate::c::rem_i32(PokeJumpRandom(), 4i32) == 0i32 {
                        let __p6 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(40)
                        .cast::<u32>();
                        (__p6).write(((__p6).read()).wrapping_sub(5u32));
                    }
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(80))
                    .write(0u8);
                }
            }
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<i32>())
            .write(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<u32>())
                .read()) as i32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PokeJumpRandom() -> i32 {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<u32>())
        .write(
            ((1103515245u32).wrapping_mul(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<u32>())
                .read(),
            ))
            .wrapping_add(24691u32),
        );
        return ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<u32>())
        .read()
            >> 16) as i32);
    }
}
pub(crate) unsafe extern "C" fn ResetVineAfterHit() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u32>())
        .write(1u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .write(6u32);
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u16>())
        .write(1535u16);
        AllowVineUpdates();
    }
}
pub(crate) unsafe extern "C" fn IsGameOver() -> i32 {
    unsafe {
        return ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u32>())
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn ResetPlayersJumpStates() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(20)
                    .cast::<i32>())
                    .write(0i32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetPlayersMonState() {
    unsafe {
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16)
        .cast::<u16>())
        .write(0u16);
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(18)
        .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn IsPlayersMonState(monState: u16) -> u32 {
    unsafe {
        let mut monState = monState;
        if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33508))
        .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(16)
        .cast::<u16>())
        .read()) as i32)
            == ((monState) as i32)
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn SetMonStateJump() {
    unsafe {
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(14)
        .cast::<u16>())
        .write(
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(18)
        .cast::<u16>())
        .write(
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16)
        .cast::<u16>())
        .write(1u16);
    }
}
pub(crate) unsafe extern "C" fn SetMonStateHit() {
    unsafe {
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(18)
        .cast::<u16>())
        .write(
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16)
        .cast::<u16>())
        .write(2u16);
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(14)
        .cast::<u16>())
        .write(
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(20)
        .cast::<i32>())
        .write(2i32);
    }
}
pub(crate) unsafe extern "C" fn SetMonStateNormal() {
    unsafe {
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(18)
        .cast::<u16>())
        .write(
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33708)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(16)
        .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateGame() {
    unsafe {
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(92)
            .cast::<u32>())
        .read())
            != 0
        {
            PrintScore(
                (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(8)
                .cast::<u32>())
                .read()) as i32),
            );
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<u32>())
            .write(0u32);
            if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(77))
            .read())
                != 0
            {
                let mut numPlayers: i32 = DoSameJumpTimeBonus(
                    (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(1))
                    .read(),
                );
                PlaySE(
                    ((((&raw const sSoundEffects)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((numPlayers).wrapping_sub(2i32)) as isize))
                    .read(),
                );
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(77))
                .write(0u8);
            }
        }
        PrintJumpsInRow(
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        HandleMonState();
        TryUpdateVineSwing();
    }
}
pub(crate) unsafe extern "C" fn TryUpdateVineSwing() {
    unsafe {
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(70))
            .read())
            != 0
        {
            UpdateVineSwing(
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<u32>())
                .read()) as i32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DisallowVineUpdates() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(70))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn AllowVineUpdates() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(70))
            .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn HandleMonState() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut soundFlags: i32 = 0i32;
        let mut numPlayers: i32 =
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < numPlayers) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 =
                            (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read()) as i32);
                        if __sw1 == 0i32 {
                            SetMonSpriteY(((i) as u32), 0i16);
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            if ((((((((((&raw mut sPokemonJump)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .read()) as i32)
                                != 1i32)
                                || ((((((((((&raw mut sPokemonJump)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(33508))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 40))
                                .wrapping_add(14)
                                .cast::<u16>())
                                .read()) as i32)
                                    != ((((((((&raw mut sPokemonJump)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(154))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32))
                            {
                                if i == ((((((&raw mut sPokemonJump)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(6))
                                .read()) as i32)
                                {
                                    (((((((&raw mut sPokemonJump)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(33508))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 40))
                                    .wrapping_add(18)
                                    .cast::<u16>())
                                    .write(1u16);
                                }
                                soundFlags = (soundFlags | 1i32);
                                (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(33508))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 40))
                                .wrapping_add(4)
                                .cast::<i32>())
                                .write(2147483647i32);
                                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(154))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .write(
                                    (((((((&raw mut sPokemonJump)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(33508))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 40))
                                    .wrapping_add(14)
                                    .cast::<u16>())
                                    .read(),
                                );
                            }
                            UpdateJump(i);
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(33508))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 40))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .read()) as i32)
                                != 2i32
                            {
                                if i == ((((((&raw mut sPokemonJump)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(6))
                                .read()) as i32)
                                {
                                    (((((((&raw mut sPokemonJump)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(33508))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 40))
                                    .wrapping_add(18)
                                    .cast::<u16>())
                                    .write(2u16);
                                }
                                soundFlags = (soundFlags | 2i32);
                                StartMonHitShake(((i) as u8));
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (soundFlags & 2i32) != 0 {
            PlaySE(262u16);
        } else {
            if (soundFlags & 1i32) != 0 {
                PlaySE(10u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateJump(multiplayerId: i32) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        let mut jumpOffsetIdx: i32 = 0i32;
        let mut jumpOffset: i32 = 0i32;
        let mut player: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(104)
            .cast::<u32>())
        .read())
            != 0
        {
            return;
        }
        player = (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33508))
        .cast::<u8>())
        .wrapping_offset((multiplayerId) as isize * 40);
        if ((player).wrapping_add(4).cast::<i32>()).read() != 2147483647i32 {
            let __p1 = (player).wrapping_add(4).cast::<i32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            jumpOffsetIdx = ((player).wrapping_add(4).cast::<i32>()).read();
        } else {
            jumpOffsetIdx = ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_sub(((((player).wrapping_add(14).cast::<u16>()).read()) as i32));
            if jumpOffsetIdx >= 65000i32 {
                jumpOffsetIdx = (jumpOffsetIdx).wrapping_sub(65000i32);
                jumpOffsetIdx = (jumpOffsetIdx).wrapping_add(
                    ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .read()) as i32),
                );
            }
            ((player).wrapping_add(4).cast::<i32>()).write(jumpOffsetIdx);
        }
        if jumpOffsetIdx < 4i32 {
            return;
        }
        jumpOffsetIdx = (jumpOffsetIdx).wrapping_sub(4i32);
        if jumpOffsetIdx < ((crate::c::div_u32(48u32, 1u32)) as i32) {
            jumpOffset = ((((((((&raw const sJumpOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((player).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 48,
                ))
            .cast::<i8>())
            .wrapping_offset((jumpOffsetIdx) as isize))
            .read()) as i32);
        } else {
            jumpOffset = 0i32;
        }
        SetMonSpriteY(((multiplayerId) as u32), ((jumpOffset) as i16));
        if (jumpOffset == 0i32)
            && (multiplayerId
                == ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32))
        {
            SetMonStateNormal();
        }
        ((player).cast::<i32>()).write(jumpOffset);
    }
}
pub(crate) unsafe extern "C" fn TryUpdateScore() {
    unsafe {
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .read()
            == 8u32)
            && (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<u32>())
            .read()
                == 7u32)
        {
            if !((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(88)
                .cast::<u32>())
            .read())
                != 0)
            {
                ClearUnreadField();
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .write(0i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u32>())
                .write(1u32);
                (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(1))
                .write(0u8);
            } else {
                if ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .read()
                    == 5i32
                {
                    let __p1 = (((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<u16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                    TryUpdateExcellentsRecord(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<u16>())
                        .read(),
                    );
                } else {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .write(0u16);
                }
                if ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .read()
                    > 1i32
                {
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(1u32);
                    crate::c::memcpy(
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(134))
                        .cast::<u8>(),
                        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(129))
                        .cast::<u8>(),
                        5u32,
                    );
                }
                ClearUnreadField();
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .write(0i32);
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u32>())
                .write(1u32);
                (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(1))
                .write(0u8);
                if (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as i32)
                    < 9999i32
                {
                    let __p2 = ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(4)
                    .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                AddJumpScore(10i32);
                SetLinkTimeInterval(3i32);
            }
        }
        if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(100)
            .cast::<u32>())
        .read())
            != 0)
            && ((DidAllPlayersClearVine() == 1u32)
                || (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<u32>())
                .read()
                    == 0u32))
        {
            let mut numPlayers: i32 = GetNumPlayersForBonus(
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(134))
                .cast::<u8>(),
            );
            AddJumpScore(GetScoreBonus(numPlayers));
            SetLinkTimeInterval(3i32);
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(100)
                .cast::<u32>())
            .write(0u32);
        }
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(88)
            .cast::<u32>())
        .read())
            != 0
        {
            let mut numAtPeak: i32 = GetPlayersAtJumpPeak();
            if numAtPeak
                > ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .read()
            {
                ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<i32>())
                .write(numAtPeak);
                crate::c::memcpy(
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(129))
                    .cast::<u8>(),
                    ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .cast::<u8>(),
                    5u32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateVineHitStates() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .read()
            == 6u32)
            && (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .cast::<i32>())
            .read()
                == 0i32)
        {
            if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(18)
            .cast::<u16>())
            .read()) as i32)
                == 1i32)
                && (IsGameOver() == 1i32)
            {
                ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(33708)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(20)
                .cast::<i32>())
                .write(1i32);
            } else {
                SetMonStateHit();
                SetLinkTimeInterval(3i32);
            }
        }
        if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u32>())
        .read()
            == 7u32)
            && (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<u32>())
            .read()
                == 6u32))
            && (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<u16>())
            .read()) as i32)
                != 2i32)
        {
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(33708)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(20)
            .cast::<i32>())
            .write(1i32);
            SetLinkTimeInterval(3i32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .read()) as i32)
                        == 2i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn AllPlayersJumpedOrHit() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numPlayers: i32 =
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read()) as i32);
        let mut numJumpedOrHit: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < numPlayers) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(20)
                    .cast::<i32>())
                    .read()
                        != 0i32
                    {
                        numJumpedOrHit = (numJumpedOrHit).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((numJumpedOrHit == numPlayers) as u32);
    }
}
pub(crate) unsafe extern "C" fn DidAllPlayersClearVine() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .wrapping_add(20)
                    .cast::<i32>())
                    .read()
                        != 1i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ShouldPlayAgain() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(69))
            .read()) as i32)
            == 1i32
        {
            return 0u32;
        }
        {
            i = 1i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 1i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn AddJumpScore(score: i32) {
    unsafe {
        let mut score = score;
        let __p1 = ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(112))
        .wrapping_add(8)
        .cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(((score) as u32)));
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(92)
            .cast::<u32>())
        .write(1u32);
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
            .wrapping_add(8)
            .cast::<u32>())
        .read()
            >= 99990u32
        {
            (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .wrapping_add(8)
                .cast::<u32>())
            .write(99990u32);
        }
    }
}
pub(crate) unsafe extern "C" fn GetPlayersAtJumpPeak() -> i32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numAtPeak: i32 = 0i32;
        let mut numPlayers: i32 =
            ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < numPlayers) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(33508))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 40))
                    .cast::<i32>())
                    .read()
                        == (-30i32)
                    {
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(1u8);
                        numAtPeak = (numAtPeak).wrapping_add(1);
                    } else {
                        ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return numAtPeak;
    }
}
pub(crate) unsafe extern "C" fn AreLinkQueuesEmpty() -> u32 {
    unsafe {
        return (((!((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
            .read_volatile())
            != 0))
            && (!((((((&raw mut gRfu).cast::<u8>()).wrapping_add(2536)).wrapping_add(562))
                .read_volatile())
                != 0))) as u32);
    }
}
pub(crate) unsafe extern "C" fn GetNumPlayersForBonus(atJumpPeak: *mut u8) -> i32 {
    unsafe {
        let mut atJumpPeak = atJumpPeak;
        let mut i: i32 = 0i32;
        let mut flags: i32 = 0i32;
        let mut count: i32 = 0i32;
        {
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (((atJumpPeak).wrapping_offset((i) as isize)).read()) != 0 {
                        flags = (flags | crate::c::shl_i32(1i32, ((i) as u32)));
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
            .wrapping_add(1))
        .write(((flags) as u8));
        if (flags) != 0 {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(77))
                .write(1u8);
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn ClearUnreadField() {
    unsafe {
        ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(68))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetScoreBonus(numPlayers: i32) -> i32 {
    unsafe {
        let mut numPlayers = numPlayers;
        return ((((&raw const sScoreBonuses)
            .cast::<u8>()
            .cast_mut()
            .cast::<i32>())
        .cast::<i32>())
        .wrapping_offset((numPlayers) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn TryUpdateExcellentsRecord(excellentsInRow: u16) {
    unsafe {
        let mut excellentsInRow = excellentsInRow;
        if ((excellentsInRow) as i32)
            > ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read()) as i32)
        {
            ((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .write(excellentsInRow);
        }
    }
}
pub(crate) unsafe extern "C" fn HasEnoughScoreForPrize() -> u32 {
    unsafe {
        if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
            .wrapping_add(8)
            .cast::<u32>())
        .read()
            >= ((((&raw const sPrizeQuantityData).cast::<u8>().cast_mut()).cast::<u8>())
                .cast::<u32>())
            .read()
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetPrizeData() -> u16 {
    unsafe {
        let mut itemId: u16 = GetPrizeItemId();
        let mut quantity: u16 = GetPrizeQuantity();
        return (((((quantity) as i32) << 12) | (((itemId) as i32) & 4095i32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn UnpackPrizeData(data: u16, itemId: *mut u16, quantity: *mut u16) {
    unsafe {
        let mut data = data;
        let mut itemId = itemId;
        let mut quantity = quantity;
        (quantity).write(((((data) as i32) >> 12) as u16));
        (itemId).write(((((data) as i32) & 4095i32) as u16));
    }
}
pub(crate) unsafe extern "C" fn GetPrizeItemId() -> u16 {
    unsafe {
        let mut index: u16 =
            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(16u32, 2u32))) as u16);
        return ((((&raw const sPrizeItems)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((index) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetPrizeQuantity() -> u16 {
    unsafe {
        let mut quantity: u32 = 0u32;
        let mut i: u32 = 0u32;
        quantity = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(40u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112))
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read()
                        >= (((((&raw const sPrizeQuantityData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<u32>())
                        .read()
                    {
                        quantity = (((((&raw const sPrizeQuantityData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .read();
                    } else {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((quantity) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetQuantityLimitedByBag(item: u16, quantity: u16) -> u16 {
    unsafe {
        let mut item = item;
        let mut quantity = quantity;
        'l1: loop {
            if !(((quantity) != 0) && (!((CheckBagHasSpace(item, quantity)) != 0))) {
                break 'l1;
            }
            quantity = (quantity).wrapping_sub(1);
        }
        return quantity;
    }
}
pub(crate) unsafe extern "C" fn GetNumPokeJumpPlayers() -> u16 {
    unsafe {
        return ((GetLinkPlayerCount()) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetPokeJumpMultiplayerId() -> u16 {
    unsafe {
        return ((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6))
        .read()) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetMonInfoByMultiplayerId(multiplayerId: u8) -> *mut u8 {
    unsafe {
        let mut multiplayerId = multiplayerId;
        return (((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33448))
        .cast::<u8>())
        .wrapping_offset(((multiplayerId) as i32) as isize * 12);
    }
}
pub(crate) unsafe extern "C" fn GetPokeJumpPlayerName(multiplayerId: u8) -> *mut u8 {
    unsafe {
        let mut multiplayerId = multiplayerId;
        return (((((((&raw mut sPokemonJump).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33508))
        .cast::<u8>())
        .wrapping_offset(((multiplayerId) as i32) as isize * 40))
        .wrapping_add(28))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpeciesAllowedInPokemonJump(species: u16) -> u32 {
    unsafe {
        let mut species = species;
        return ((((GetPokemonJumpSpeciesIdx(species)) as i32) > (-1i32)) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonJumpSpeciesInParty() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        5i32,
                    )) != 0
                    {
                        let mut species: u16 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        )) as u16);
                        if (IsSpeciesAllowedInPokemonJump(species)) != 0 {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                            return;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadSpriteSheetsAndPalettes(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(40u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sCompressedSpriteSheets).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 8u32)) {
                    break 'l3;
                }
                'l4: {
                    LoadSpritePalette(
                        (((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((jumpGfx).wrapping_add(14)).write(IndexOfSpritePaletteTag(5u16));
        ((jumpGfx).wrapping_add(15)).write(IndexOfSpritePaletteTag(6u16));
    }
}
pub(crate) unsafe extern "C" fn ResetPokeJumpSpriteData(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(16u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateJumpMonSprite(
    jumpGfx: *mut u8,
    monInfo: *mut u8,
    x: i16,
    y: i16,
    multiplayerId: u8,
) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut monInfo = monInfo;
        let mut x = x;
        let mut y = y;
        let mut multiplayerId = multiplayerId;
        let mut spriteTemplate = crate::ffi::Align4([0u8; 24]);
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut spritePalette = crate::ffi::Align4([0u8; 8]);
        let mut buffer: *mut u8 = core::ptr::null_mut();
        let mut unusedBuffer: *mut u8 = core::ptr::null_mut();
        let mut subpriority: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        (&raw mut spriteTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sSpriteTemplate_JumpMon)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        buffer = Alloc((((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(4i32)) as u32));
        unusedBuffer = Alloc(((crate::c::div_i32(4096i32, 2i32)) as u32));
        if ((multiplayerId) as i32) == ((GetPokeJumpMultiplayerId()) as i32) {
            subpriority = 3u8;
        } else {
            subpriority = ((((multiplayerId) as i32).wrapping_add(4i32)) as u8);
        }
        if (!(buffer).is_null()) && (!(unusedBuffer).is_null()) {
            HandleLoadSpecialPokePic(
                ((&raw mut gMonStillFrontPicTable).cast::<u8>())
                    .wrapping_offset(((((monInfo).cast::<u16>()).read()) as i32) as isize * 8),
                buffer,
                ((((monInfo).cast::<u16>()).read()) as i32),
                ((monInfo).wrapping_add(8).cast::<u32>()).read(),
            );
            (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u8>()).write(buffer);
            (((&raw mut spriteSheet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(((multiplayerId) as u16));
            (((&raw mut spriteSheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(((crate::c::div_i32(4096i32, 2i32)) as u16));
            LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>());
            (((&raw mut spritePalette).cast::<u8>()).cast::<*mut u32>()).write(
                GetMonSpritePalFromSpeciesAndPersonality(
                    ((monInfo).cast::<u16>()).read(),
                    ((monInfo).wrapping_add(4).cast::<u32>()).read(),
                    ((monInfo).wrapping_add(8).cast::<u32>()).read(),
                ),
            );
            (((&raw mut spritePalette).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(((multiplayerId) as u16));
            LoadCompressedSpritePalette((&raw mut spritePalette).cast::<u8>());
            Free(buffer);
            Free(unusedBuffer);
            let __p1 = ((&raw mut spriteTemplate).cast::<u8>()).cast::<u16>();
            (__p1)
                .write((((((__p1).read()) as i32).wrapping_add(((multiplayerId) as i32))) as u16));
            let __p2 = ((&raw mut spriteTemplate).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>();
            (__p2)
                .write((((((__p2).read()) as i32).wrapping_add(((multiplayerId) as i32))) as u16));
            spriteId = CreateSprite((&raw mut spriteTemplate).cast::<u8>(), x, y, subpriority);
            if ((spriteId) as i32) != 64i32 {
                ((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                    .wrapping_offset(((multiplayerId) as i32) as isize))
                .write(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
                ((((jumpGfx).wrapping_add(33276)).cast::<u8>())
                    .wrapping_offset(((multiplayerId) as i32) as isize))
                .write(subpriority);
                return;
            }
        }
        ((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
            .wrapping_offset(((multiplayerId) as i32) as isize))
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn DoStarAnim(jumpGfx: *mut u8, multiplayerId: i32) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut multiplayerId = multiplayerId;
        ResetPokeJumpSpriteData(
            ((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read(),
        );
        ((((((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(
            ((((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read()) as usize)
                .wrapping_sub(((&raw mut gSprites).cast::<u8>()) as usize) as i32
                / 68) as i16),
        );
        crate::c::bf_write(
            (((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read())
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(96i16);
        ((((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Star));
        StartSpriteAnim(
            ((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read(),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Star(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 72i32 {
                    ((sprite).wrapping_add(34).cast::<i16>()).write(72i16);
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    >= 48i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonHitShake(jumpGfx: *mut u8, multiplayerId: i32) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut multiplayerId = multiplayerId;
        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MonHitShake));
        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(38)
        .cast::<i16>())
        .write(0i16);
        ResetPokeJumpSpriteData(
            ((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Gfx_IsMonHitShakeActive(
    jumpGfx: *mut u8,
    multiplayerId: i32,
) -> u32 {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut multiplayerId = multiplayerId;
        return ((core::mem::transmute::<_, usize>(
            ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCB_MonHitShake as *const () as usize)) as u32);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonHitShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            if ((({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                & 1i32)
                != 0
            {
                ((sprite).wrapping_add(38).cast::<i16>()).write(2i16);
            } else {
                ((sprite).wrapping_add(38).cast::<i16>()).write((-2i16));
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 12i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonHitFlash(jumpGfx: *mut u8, multiplayerId: i32) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut multiplayerId = multiplayerId;
        ResetPokeJumpSpriteData(
            ((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read(),
        );
        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MonHitFlash));
    }
}
pub(crate) unsafe extern "C" fn Gfx_StopMonHitFlash(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut i: i32 = 0i32;
        let mut numPlayers: u16 = GetNumPokeJumpPlayers();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if core::mem::transmute::<_, usize>(
                        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .read(),
                    ) == (SpriteCB_MonHitFlash as *const () as usize)
                    {
                        crate::c::bf_write(
                            (((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                                .wrapping_offset((i) as isize))
                            .read())
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCallbackDummy));
                        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(67))
                        .write(10u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonHitFlash(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Gfx_ResetMonSpriteSubpriorities(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut i: i32 = 0i32;
        let mut numPlayers: u16 = GetNumPokeJumpPlayers();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(67))
                    .write(
                        ((((jumpGfx).wrapping_add(33276)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Gfx_StartMonIntroBounce(jumpGfx: *mut u8, multiplayerId: i32) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut multiplayerId = multiplayerId;
        ResetPokeJumpSpriteData(
            ((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                .wrapping_offset((multiplayerId) as isize))
            .read(),
        );
        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
            .wrapping_offset((multiplayerId) as isize))
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MonIntroBounce));
    }
}
pub(crate) unsafe extern "C" fn Gfx_IsMonIntroBounceActive(jumpGfx: *mut u8) -> u32 {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut i: i32 = 0i32;
        let mut numPlayers: u16 = GetNumPokeJumpPlayers();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if core::mem::transmute::<_, usize>(
                        ((((((jumpGfx).wrapping_add(33192)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .read(),
                    ) == (SpriteCB_MonIntroBounce as *const () as usize)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonIntroBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PlaySE(34u16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 127i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 3)
                        .wrapping_neg()) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 0i32
                {
                    if (({
                        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        < 2i32
                    {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    } else {
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCallbackDummy));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStarSprite(
    jumpGfx: *mut u8,
    x: i16,
    y: i16,
    multiplayerId: u8,
) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut x = x;
        let mut y = y;
        let mut multiplayerId = multiplayerId;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_Star).cast::<u8>().cast_mut(),
            x,
            y,
            1u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            ((((jumpGfx).wrapping_add(33212)).cast::<*mut u8>())
                .wrapping_offset(((multiplayerId) as i32) as isize))
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreateVineSprites(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        count = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        ((((&raw const sSpriteTemplates_Vine)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((((&raw const sVineXCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<i16>())
                        .cast::<i16>())
                        .wrapping_offset((count) as isize))
                        .read(),
                        (((((&raw const sVineYCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<i16>())
                        .read(),
                        2u8,
                    );
                    ((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                        .wrapping_offset((count) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    count = (count).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 3i32;
            'l3: loop {
                if !(i >= 0i32) {
                    break 'l3;
                }
                'l4: {
                    spriteId = CreateSprite(
                        ((((&raw const sSpriteTemplates_Vine)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((((&raw const sVineXCoords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<i16>())
                        .cast::<i16>())
                        .wrapping_offset((count) as isize))
                        .read(),
                        (((((&raw const sVineYCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<i16>())
                        .read(),
                        2u8,
                    );
                    ((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                        .wrapping_offset((count) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    crate::c::bf_write(
                        (((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read())
                        .wrapping_add(63),
                        0,
                        1,
                        (1u16) as i32,
                    );
                    count = (count).wrapping_add(1);
                }
                i = (i).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateVineAnim(jumpGfx: *mut u8, vineState: i32) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut vineState = vineState;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut palNum: i32 = 0i32;
        let mut priority: i32 = 0i32;
        if vineState > 5i32 {
            vineState = (10i32).wrapping_sub(vineState);
            priority = 3i32;
            palNum = ((((jumpGfx).wrapping_add(15)).read()) as i32);
        } else {
            priority = 2i32;
            palNum = ((((jumpGfx).wrapping_add(14)).read()) as i32);
        }
        count = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                        .wrapping_offset((count) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(
                        ((((((&raw const sVineYCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<i16>())
                        .wrapping_offset((vineState) as isize))
                        .read(),
                    );
                    crate::c::bf_write(
                        (((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        ((priority) as u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read())
                        .wrapping_add(5),
                        4,
                        4,
                        ((palNum) as u16) as i32,
                    );
                    StartSpriteAnim(
                        ((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read(),
                        ((vineState) as u8),
                    );
                    count = (count).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 3i32;
            'l3: loop {
                if !(i >= 0i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                        .wrapping_offset((count) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(
                        ((((((&raw const sVineYCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 20))
                        .cast::<i16>())
                        .wrapping_offset((vineState) as isize))
                        .read(),
                    );
                    crate::c::bf_write(
                        (((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        ((priority) as u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read())
                        .wrapping_add(5),
                        4,
                        4,
                        ((palNum) as u16) as i32,
                    );
                    StartSpriteAnim(
                        ((((jumpGfx).wrapping_add(33232)).cast::<*mut u8>())
                            .wrapping_offset((count) as isize))
                        .read(),
                        ((vineState) as u8),
                    );
                    count = (count).wrapping_add(1);
                }
                i = (i).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartPokeJumpCountdown(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        StartMinigameCountdown(9u16, 7u16, 120i16, 80i16, 0u8);
        Gfx_ResetMonSpriteSubpriorities(jumpGfx);
    }
}
pub(crate) unsafe extern "C" fn IsPokeJumpCountdownRunning() -> u32 {
    unsafe {
        return IsMinigameCountdownRunning();
    }
}
pub(crate) unsafe extern "C" fn StartPokeJumpGfx(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        let mut taskId: u8 = 0u8;
        ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).write(jumpGfx);
        InitPokeJumpGfx(((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read());
        taskId = CreateTask(Some(Task_RunPokeJumpGfxFunc), 3u8);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .write(taskId);
        SetWordTaskArg(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .read(),
            2u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()) as usize as u32),
        );
        SetUpPokeJumpGfxFunc(Some(LoadPokeJumpGfx));
    }
}
pub(crate) unsafe extern "C" fn FreeWindowsAndDigitObj() {
    unsafe {
        FreeAllWindowBuffers();
        DigitObjUtil_Free();
    }
}
pub(crate) unsafe extern "C" fn InitPokeJumpGfx(jumpGfx: *mut u8) {
    unsafe {
        let mut jumpGfx = jumpGfx;
        ((jumpGfx).wrapping_add(4).cast::<u16>()).write(0u16);
        ((jumpGfx).cast::<u32>()).write(0u32);
        ((jumpGfx).wrapping_add(18).cast::<u16>()).write(255u16);
    }
}
pub(crate) unsafe extern "C" fn SetUpPokeJumpGfxFuncById(id: i32) {
    unsafe {
        let mut id = id;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(80u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw const sPokeJumpGfxFuncs).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                    .cast::<i32>())
                    .read()
                        == id
                    {
                        SetUpPokeJumpGfxFunc(
                            (((((&raw const sPokeJumpGfxFuncs).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 8))
                            .wrapping_add(4)
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsPokeJumpGfxFuncFinished() -> u32 {
    unsafe {
        return ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<u32>())
        .read()
            != 1u32) as u32);
    }
}
pub(crate) unsafe extern "C" fn SetUpPokeJumpGfxFunc(func: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut func = func;
        SetWordTaskArg(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .read(),
            0u8,
            (core::mem::transmute::<_, usize>(func) as u32),
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>())
            .write(0u32);
    }
}
pub(crate) unsafe extern "C" fn Task_RunPokeJumpGfxFunc(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>())
            .read())
            != 0)
        {
            let mut func: Option<unsafe extern "C" fn()> =
                core::mem::transmute::<_, Option<unsafe extern "C" fn()>>(
                    ((GetWordTaskArg(taskId, 0u8)) as usize as *mut u8),
                );
            (func).unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPokeJumpGfx() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
                ResetTempTileDataBuffers();
                LoadSpriteSheetsAndPalettes(
                    ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                );
                InitDigitPrinters();
                LoadPalette(
                    (((&raw const sBg_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                        .cast::<u8>(),
                    0u16,
                    32u16,
                );
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sBg_Gfx).cast::<u8>().cast_mut().cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sBg_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    1u8,
                );
                LoadPalette(
                    (((&raw const sVenusaur_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sVenusaur_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sVenusaur_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    1u8,
                );
                LoadPalette(
                    (((&raw const sBonuses_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sBonuses_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw const sBonuses_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    1u8,
                );
                LoadPalette(
                    (((&raw const sInterface_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    32u16,
                    32u16,
                );
                SetBgTilemapBuffer(
                    0u8,
                    (((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(422))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                PrintScoreSuffixes();
                PrintScore(0i32);
                LoadUserWindowBorderGfxOnBg(0u8, 1u16, 224u8);
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(1u8);
                ResetBgPositions();
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    CreateJumpMonSprites();
                    CreateVineSprites(
                        ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                    );
                    UpdateVineAnim(
                        ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                        6i32,
                    );
                    ShowBg(3u8);
                    ShowBg(0u8);
                    ShowBg(2u8);
                    HideBg(1u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u32>())
                .write(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNamesNoHighlight() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                AddPlayerNameWindows();
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PrintPokeJumpPlayerNames(0u32);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    DrawPlayerNameWindows();
                    let __p4 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNamesWithHighlight() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                AddPlayerNameWindows();
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PrintPokeJumpPlayerNames(1u32);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    DrawPlayerNameWindows();
                    let __p4 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ErasePlayerNames() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut numPlayers: i32 = 0i32;
        numPlayers = ((GetNumPokeJumpPlayers()) as i32);
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < numPlayers) {
                            break 'l2;
                        }
                        'l3: {
                            ClearWindowTilemap(
                                ((((((((&raw mut sPokemonJumpGfx)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(28))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                CopyBgTilemapBufferToVram(0u8);
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < numPlayers) {
                                break 'l4;
                            }
                            'l5: {
                                RemoveWindow(
                                    ((((((((&raw mut sPokemonJumpGfx)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(28))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_WantToPlayAgain() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(((AddMessageWindow(1u32, 8u32, 20u32, 2u32)) as u16));
                AddTextPrinterParameterized(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    1u8,
                    (&raw mut gText_WantToPlayAgain2).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    2u8,
                );
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    DrawTextBorderOuter(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                        1u16,
                        14u8,
                    );
                    CreatePokeJumpYesNoMenu(23u16, 7u16, 0u8);
                    CopyBgTilemapBufferToVram(0u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_SavingDontTurnOff() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(((AddMessageWindow(2u32, 7u32, 26u32, 4u32)) as u16));
                AddTextPrinterParameterized(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    1u8,
                    (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    2u8,
                );
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    DrawTextBorderOuter(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                        1u16,
                        14u8,
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EraseMessage() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ClearMessageWindow();
                EraseYesNoWindow();
                CopyBgTilemapBufferToVram(0u8);
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((RemoveMessageWindow()) != 0)) && (!((IsDma3ManagerBusyWithBgCopy()) != 0)) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_SomeoneDroppedOut() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(((AddMessageWindow(2u32, 8u32, 22u32, 4u32)) as u16));
                AddTextPrinterParameterized(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    1u8,
                    (&raw mut gText_SomeoneDroppedOut2).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    2u8,
                );
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    DrawTextBorderOuter(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                        1u16,
                        14u8,
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Msg_CommunicationStandby() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(((AddMessageWindow(7u32, 10u32, 16u32, 2u32)) as u16));
                AddTextPrinterParameterized(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    1u8,
                    (&raw mut gText_CommunicationStandby4).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                CopyWindowToVram(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as u8),
                    2u8,
                );
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    DrawTextBorderOuter(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                        1u16,
                        14u8,
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoPokeJumpCountdown() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StartPokeJumpCountdown(
                    ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                );
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsPokeJumpCountdownRunning()) != 0) {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u32>())
                    .write(1u32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetUpResetVineGfx() {
    unsafe {
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(0u8);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(11))
            .write(0u8);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .write(6u8);
        UpdateVineSwing(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .read()) as i32),
        );
    }
}
pub(crate) unsafe extern "C" fn ResetVineGfx() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11))
                .read()) as i32)
                    > 10i32
                {
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11))
                    .write(0u8);
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32)
                        >= 10i32
                    {
                        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .write(0u8);
                        let __p4 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(10);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                UpdateVineSwing(
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32),
                );
                if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read()) as i32)
                    != 7i32
                {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn PrintPrizeMessage(itemId: u16, quantity: u16) {
    unsafe {
        let mut itemId = itemId;
        let mut quantity = quantity;
        CopyItemNameHandlePlural(
            itemId,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
            ((quantity) as u32),
        );
        ConvertIntToDecimalStringN(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(102))
            .cast::<u8>(),
            ((quantity) as i32),
            0i32,
            1u8,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            1u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(102))
            .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            (&raw mut gText_AwesomeWonF701F700).cast::<u8>(),
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(((AddMessageWindow(4u32, 8u32, 22u32, 4u32)) as u16));
        AddTextPrinterParameterized(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            1u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            2u8,
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(367u16);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintPrizeFilledBagMessage(itemId: u16) {
    unsafe {
        let mut itemId = itemId;
        CopyItemName(
            itemId,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            (&raw mut gText_FilledStorageSpace2).cast::<u8>(),
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(((AddMessageWindow(4u32, 8u32, 22u32, 4u32)) as u16));
        AddTextPrinterParameterized(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            1u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            2u8,
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintNoRoomForPrizeMessage(itemId: u16) {
    unsafe {
        let mut itemId = itemId;
        CopyItemName(
            itemId,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(38))
                .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            (&raw mut gText_CantHoldMore).cast::<u8>(),
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(((AddMessageWindow(4u32, 9u32, 22u32, 2u32)) as u16));
        AddTextPrinterParameterized(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            1u8,
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(166))
            .cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as u8),
            2u8,
        );
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DoPrizeMessageAndFanfare() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(13))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    PutWindowTilemap(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    DrawTextBorderOuter(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                        1u16,
                        14u8,
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    break 'l1;
                }
                if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    let __p3 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13);
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as u8));
                    return 0u32;
                }
                PlayFanfare(
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20)
                        .cast::<u16>())
                    .read(),
                );
                let __p4 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if !((IsFanfareTaskInactive()) != 0) {
                    break 'l1;
                }
                let __p5 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13);
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn ClearMessageWindow() {
    unsafe {
        if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            != 255i32
        {
            rbox_fill_rectangle(
                ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as u8),
            );
            CopyWindowToVram(
                ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read()) as u8),
                1u8,
            );
            ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveMessageWindow() -> u32 {
    unsafe {
        if ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 255i32
        {
            return 0u32;
        }
        'l1: {
            let __sw1 = ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(13))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    RemoveWindow(
                        ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read()) as u8),
                    );
                    ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .write(255u16);
                    let __p2 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                return 0u32;
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn HandlePlayAgainInput() -> i8 {
    unsafe {
        return Menu_ProcessInputNoWrapClearOnChoose();
    }
}
pub(crate) unsafe extern "C" fn AddMessageWindow(
    left: u32,
    top: u32,
    width: u32,
    height: u32,
) -> u32 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut windowId: u32 = 0u32;
        let mut window = crate::ffi::Align4([0u8; 8]);
        ((&raw mut window).cast::<u8>()).write(0u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(1)).write(((left) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(2)).write(((top) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(((width) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(((height) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(5)).write(15u8);
        (((&raw mut window).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(67u16);
        windowId = ((AddWindow((&raw mut window).cast::<u8>())) as u32);
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        return windowId;
    }
}
pub(crate) unsafe extern "C" fn CreatePokeJumpYesNoMenu(left: u16, top: u16, cursorPos: u8) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut cursorPos = cursorPos;
        let mut window = crate::ffi::Align4([0u8; 8]);
        ((&raw mut window).cast::<u8>()).write(0u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(1)).write(((left) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(2)).write(((top) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(6u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(4u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(5)).write(2u8);
        (((&raw mut window).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(43u16);
        CreateYesNoMenu((&raw mut window).cast::<u8>(), 1u16, 13u8, cursorPos);
    }
}
pub(crate) unsafe extern "C" fn PrintScoreSuffixes() {
    unsafe {
        let mut color = crate::ffi::Align4([0u8; 3]);
        (&raw mut color).cast::<u8>().wrapping_add(0).write(0u8);
        (&raw mut color).cast::<u8>().wrapping_add(1).write(2u8);
        (&raw mut color).cast::<u8>().wrapping_add(2).write(3u8);
        PutWindowTilemap(0u8);
        PutWindowTilemap(1u8);
        FillWindowPixelBuffer(0u8, 0u8);
        FillWindowPixelBuffer(1u8, 0u8);
        AddTextPrinterParameterized3(
            0u8,
            0u8,
            0u8,
            1u8,
            (&raw mut color).cast::<u8>(),
            0i8,
            (&raw mut gText_SpacePoints2).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            1u8,
            0u8,
            0u8,
            1u8,
            (&raw mut color).cast::<u8>(),
            0i8,
            (&raw mut gText_SpaceTimes3).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateJumpMonSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut playersCount: i32 = ((GetNumPokeJumpPlayers()) as i32);
        let mut xCoords: *mut i16 = ((((&raw const sMonXCoords)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut i16>())
        .cast::<*mut i16>())
        .wrapping_offset(((playersCount).wrapping_sub(2i32)) as isize))
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < playersCount) {
                    break 'l1;
                }
                'l2: {
                    let mut monInfo: *mut u8 = GetMonInfoByMultiplayerId(((i) as u8));
                    y = ((((((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_offset(
                        ((((monInfo).cast::<u16>()).read()) as i32) as isize * 4,
                    ))
                    .wrapping_add(1))
                    .read()) as i32);
                    CreateJumpMonSprite(
                        ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                        monInfo,
                        (xCoords).read(),
                        (((y).wrapping_add(112i32)) as i16),
                        ((i) as u8),
                    );
                    CreateStarSprite(
                        ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                        (xCoords).read(),
                        112i16,
                        ((i) as u8),
                    );
                    xCoords = (xCoords).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMonSpriteY(id: u32, y: i16) {
    unsafe {
        let mut id = id;
        let mut y = y;
        ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(33192))
        .cast::<*mut u8>())
        .wrapping_offset(((id) as i32) as isize))
        .read())
        .wrapping_add(38)
        .cast::<i16>())
        .write(y);
    }
}
pub(crate) unsafe extern "C" fn UpdateVineSwing(vineState: i32) {
    unsafe {
        let mut vineState = vineState;
        UpdateVineAnim(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
            vineState,
        );
        ChangeBgY(
            2u8,
            (((((((&raw const sVenusaurStates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((vineState) as isize))
            .read()) as i32)
                .wrapping_mul(5i32)
                << 13),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DoSameJumpTimeBonus(flags: u8) -> i32 {
    unsafe {
        let mut flags = flags;
        let mut i: i32 = 0i32;
        let mut numPlayers: i32 = 0i32;
        {
            i = 0i32;
            numPlayers = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (((flags) as i32) & 1i32) != 0 {
                        DoStarAnim(
                            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
                            i,
                        );
                        numPlayers = (numPlayers).wrapping_add(1);
                    }
                    flags = ((((flags) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ShowBonus((((numPlayers).wrapping_sub(2i32)) as u8));
        return numPlayers;
    }
}
pub(crate) unsafe extern "C" fn InitDigitPrinters() {
    unsafe {
        let mut template = crate::ffi::Align4([0u8; 16]);
        crate::c::bf_write(
            (&raw mut template).cast::<u8>().wrapping_add(0),
            2,
            2,
            (0i32) as i32,
        );
        crate::c::bf_write(
            (&raw mut template).cast::<u8>().wrapping_add(0),
            4,
            2,
            (0i32) as i32,
        );
        crate::c::bf_write(
            (&raw mut template).cast::<u8>().wrapping_add(0),
            0,
            2,
            (0i32) as i32,
        );
        crate::c::bf_write(
            (&raw mut template).cast::<u8>().wrapping_add(0),
            6,
            2,
            (1i32) as i32,
        );
        (&raw mut template).cast::<u8>().wrapping_add(1).write(5u8);
        (&raw mut template).cast::<u8>().wrapping_add(2).write(8u8);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(108i16);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<i16>()
            .write(6i16);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut u8>()
            .write((&raw const sSpriteSheet_Digits).cast::<u8>().cast_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write((&raw const sSpritePalette_Digits).cast::<u8>().cast_mut());
        DigitObjUtil_Init(2u32);
        DigitObjUtil_CreatePrinter(0u32, 0i32, (&raw mut template).cast::<u8>());
        (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(4u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(4)
            .cast::<i16>())
        .write(30i16);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<i16>())
        .write(6i16);
        DigitObjUtil_CreatePrinter(1u32, 0i32, (&raw mut template).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn PrintScore(num: i32) {
    unsafe {
        let mut num = num;
        DigitObjUtil_PrintNumOn(0u32, num);
    }
}
pub(crate) unsafe extern "C" fn PrintJumpsInRow(num: u16) {
    unsafe {
        let mut num = num;
        DigitObjUtil_PrintNumOn(1u32, ((num) as i32));
    }
}
pub(crate) unsafe extern "C" fn StartMonHitShake(multiplayerId: u8) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        Gfx_StartMonHitShake(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
            ((multiplayerId) as i32),
        );
    }
}
pub(crate) unsafe extern "C" fn StartMonHitFlash(multiplayerId: u8) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        Gfx_StartMonHitFlash(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
            ((multiplayerId) as i32),
        );
    }
}
pub(crate) unsafe extern "C" fn IsMonHitShakeActive(multiplayerId: i32) -> i32 {
    unsafe {
        let mut multiplayerId = multiplayerId;
        return ((Gfx_IsMonHitShakeActive(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
            multiplayerId,
        )) as i32);
    }
}
pub(crate) unsafe extern "C" fn StopMonHitFlash() {
    unsafe {
        Gfx_StopMonHitFlash(((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn ResetMonSpriteSubpriorities() {
    unsafe {
        Gfx_ResetMonSpriteSubpriorities(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn StartMonIntroBounce(multiplayerId: i32) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        Gfx_StartMonIntroBounce(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
            multiplayerId,
        );
    }
}
pub(crate) unsafe extern "C" fn IsMonIntroBounceActive() -> i32 {
    unsafe {
        return ((Gfx_IsMonIntroBounceActive(
            ((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read(),
        )) as i32);
    }
}
pub(crate) unsafe extern "C" fn AddPlayerNameWindows() {
    unsafe {
        let mut window = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut playersCount: i32 = ((GetNumPokeJumpPlayers()) as i32);
        let mut winCoords: *mut u16 = ((((&raw const sPlayerNameWindowCoords)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u16>())
        .cast::<*mut u16>())
        .wrapping_offset(((playersCount).wrapping_sub(2i32)) as isize))
        .read();
        ((&raw mut window).cast::<u8>()).write(0u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(8u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(2u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(5)).write(2u8);
        (((&raw mut window).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(43u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < playersCount) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut window).cast::<u8>()).wrapping_add(1))
                        .write((((winCoords).read()) as u8));
                    (((&raw mut window).cast::<u8>()).wrapping_add(2))
                        .write(((((winCoords).wrapping_offset(1)).read()) as u8));
                    ((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(AddWindow((&raw mut window).cast::<u8>()));
                    ClearWindowTilemap(
                        ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(28))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as u8),
                    );
                    let __p1 = ((&raw mut window).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as u16));
                    winCoords = (winCoords).wrapping_offset(2);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintPokeJumpPlayerName(
    multiplayerId: i32,
    bgColor: u8,
    fgColor: u8,
    shadow: u8,
) {
    unsafe {
        let mut multiplayerId = multiplayerId;
        let mut bgColor = bgColor;
        let mut fgColor = fgColor;
        let mut shadow = shadow;
        let mut x: u32 = 0u32;
        let mut colors = crate::ffi::Align4([0u8; 3]);
        (&raw mut colors)
            .cast::<u8>()
            .wrapping_add(0)
            .write(bgColor);
        (&raw mut colors)
            .cast::<u8>()
            .wrapping_add(1)
            .write(fgColor);
        (&raw mut colors).cast::<u8>().wrapping_add(2).write(shadow);
        FillWindowPixelBuffer(
            ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<u16>())
            .wrapping_offset((multiplayerId) as isize))
            .read()) as u8),
            0u8,
        );
        x = (((64i32).wrapping_sub(GetStringWidth(
            1u8,
            GetPokeJumpPlayerName(((multiplayerId) as u8)),
            (-1i16),
        ))) as u32);
        x = crate::c::div_u32(x, 2u32);
        AddTextPrinterParameterized3(
            ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<u16>())
            .wrapping_offset((multiplayerId) as isize))
            .read()) as u8),
            1u8,
            ((x) as u8),
            1u8,
            (&raw mut colors).cast::<u8>(),
            (-1i8),
            GetPokeJumpPlayerName(((multiplayerId) as u8)),
        );
        CopyWindowToVram(
            ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28))
            .cast::<u16>())
            .wrapping_offset((multiplayerId) as isize))
            .read()) as u8),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintPokeJumpPlayerNames(highlightSelf: u32) {
    unsafe {
        let mut highlightSelf = highlightSelf;
        let mut i: i32 = 0i32;
        let mut multiplayerId: i32 = 0i32;
        let mut playersCount: i32 = ((GetNumPokeJumpPlayers()) as i32);
        if !((highlightSelf) != 0) {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < playersCount) {
                        break 'l1;
                    }
                    'l2: {
                        PrintPokeJumpPlayerName(i, 0u8, 2u8, 3u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            multiplayerId = ((GetPokeJumpMultiplayerId()) as i32);
            {
                i = 0i32;
                'l3: loop {
                    if !(i < playersCount) {
                        break 'l3;
                    }
                    'l4: {
                        if multiplayerId != i {
                            PrintPokeJumpPlayerName(i, 0u8, 2u8, 3u8);
                        } else {
                            PrintPokeJumpPlayerName(i, 0u8, 4u8, 5u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPlayerNameWindows() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut playersCount: i32 = ((GetNumPokeJumpPlayers()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < playersCount) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(
                        ((((((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(28))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn ShowBonus(bonusId: u8) {
    unsafe {
        let mut bonusId = bonusId;
        ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u32>())
        .write(0u32);
        ChangeBgX(
            1u8,
            ((crate::c::div_i32(((bonusId) as i32), 2i32)).wrapping_mul(256i32))
                .wrapping_mul(256i32),
            0u8,
        );
        ChangeBgY(
            1u8,
            (((crate::c::rem_i32(((bonusId) as i32), 2i32)).wrapping_mul(256i32))
                .wrapping_sub(40i32))
            .wrapping_mul(256i32),
            0u8,
        );
        ShowBg(1u8);
        CreateTask(Some(Task_UpdateBonus), 4u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateBonus() -> u32 {
    unsafe {
        if ((((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<u32>())
        .read()
            >= 32u32
        {
            return 0u32;
        } else {
            ChangeBgY(1u8, 128i32, 1u8);
            if {
                let __p1 = (((&raw mut sPokemonJumpGfx).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<u32>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            } >= 32u32
            {
                HideBg(1u8);
            }
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateBonus(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((UpdateBonus()) != 0) {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SendPacket_MonInfo(monInfo: *mut u8) {
    unsafe {
        let mut monInfo = monInfo;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        ((&raw mut packet).cast::<u8>()).write(1u8);
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(((monInfo).cast::<u16>()).read());
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(8)
            .cast::<u32>())
        .write(((monInfo).wrapping_add(4).cast::<u32>()).read());
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(((monInfo).wrapping_add(8).cast::<u32>()).read());
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_MonInfo(multiplayerId: i32, monInfo: *mut u8) -> u32 {
    unsafe {
        let mut multiplayerId = multiplayerId;
        let mut monInfo = monInfo;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        if (((((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset((multiplayerId) as isize * 16))
        .cast::<u16>())
        .read()) as i32)
            & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        crate::c::memcpy(
            (&raw mut packet).cast::<u8>(),
            (((((&raw mut gRecvCmds).cast::<u8>())
                .wrapping_offset((multiplayerId) as isize * 16))
            .cast::<u16>())
            .wrapping_offset(1))
            .cast::<u8>(),
            12u32,
        );
        if ((((&raw mut packet).cast::<u8>()).read()) as i32) == 1i32 {
            ((monInfo).cast::<u16>()).write(
                (((&raw mut packet).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read(),
            );
            ((monInfo).wrapping_add(4).cast::<u32>()).write(
                (((&raw mut packet).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u32>())
                .read(),
            );
            ((monInfo).wrapping_add(8).cast::<u32>()).write(
                (((&raw mut packet).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read(),
            );
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SendPacket_Unused(data: u32) {
    unsafe {
        let mut data = data;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        ((&raw mut packet).cast::<u8>()).write(2u8);
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(data);
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn SendPacket_LeaderState(player: *mut u8, comm: *mut u8) {
    unsafe {
        let mut player = player;
        let mut comm = comm;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        ((&raw mut packet).cast::<u8>()).write(3u8);
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(9),
            7,
            17,
            (((comm).wrapping_add(8).cast::<u32>()).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(3),
            0,
            5,
            (((comm).wrapping_add(1)).read()) as i32,
        );
        (((&raw mut packet).cast::<u8>()).wrapping_add(1)).write((comm).read());
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((comm).wrapping_add(2).cast::<u16>()).read());
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(8),
            0,
            15,
            ((((comm).wrapping_add(4).cast::<u16>()).read()) as u32) as i32,
        );
        (((&raw mut packet).cast::<u8>()).wrapping_add(2))
            .write(((((player).wrapping_add(16).cast::<u16>()).read()) as u8));
        crate::c::bf_write(
            ((&raw mut packet).cast::<u8>()).wrapping_add(3),
            5,
            3,
            ((((player).wrapping_add(20).cast::<i32>()).read()) as u8) as i32,
        );
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(((player).wrapping_add(14).cast::<u16>()).read());
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_LeaderState(player: *mut u8, comm: *mut u8) -> u32 {
    unsafe {
        let mut player = player;
        let mut comm = comm;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        crate::c::memcpy(
            (&raw mut packet).cast::<u8>(),
            ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).cast::<u8>(),
            12u32,
        );
        if ((((&raw mut packet).cast::<u8>()).read()) as i32) != 3i32 {
            return 0u32;
        }
        ((comm).wrapping_add(8).cast::<u32>()).write(
            (crate::c::bf_read(
                ((&raw mut packet).cast::<u8>()).wrapping_add(9),
                7,
                17,
                false,
            ) as u32),
        );
        ((comm).wrapping_add(1)).write(
            (crate::c::bf_read(
                ((&raw mut packet).cast::<u8>()).wrapping_add(3),
                0,
                5,
                false,
            ) as u8),
        );
        (comm).write((((&raw mut packet).cast::<u8>()).wrapping_add(1)).read());
        ((comm).wrapping_add(2).cast::<u16>()).write(
            (((&raw mut packet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        ((comm).wrapping_add(4).cast::<u16>()).write(
            ((crate::c::bf_read(
                ((&raw mut packet).cast::<u8>()).wrapping_add(8),
                0,
                15,
                false,
            ) as u32) as u16),
        );
        ((player).wrapping_add(16).cast::<u16>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(2)).read()) as u16));
        ((player).wrapping_add(20).cast::<i32>()).write(
            ((crate::c::bf_read(
                ((&raw mut packet).cast::<u8>()).wrapping_add(3),
                5,
                3,
                false,
            ) as u8) as i32),
        );
        ((player).wrapping_add(14).cast::<u16>()).write(
            (((&raw mut packet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn SendPacket_MemberState(
    player: *mut u8,
    funcId: u8,
    playAgainState: u16,
) {
    unsafe {
        let mut player = player;
        let mut funcId = funcId;
        let mut playAgainState = playAgainState;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        ((&raw mut packet).cast::<u8>()).write(4u8);
        (((&raw mut packet).cast::<u8>()).wrapping_add(1))
            .write(((((player).wrapping_add(16).cast::<u16>()).read()) as u8));
        (((&raw mut packet).cast::<u8>()).wrapping_add(2))
            .write(((((player).wrapping_add(20).cast::<i32>()).read()) as u8));
        (((&raw mut packet).cast::<u8>()).wrapping_add(3))
            .write(((((player).wrapping_add(24).cast::<u32>()).read()) as u8));
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(((player).wrapping_add(14).cast::<u16>()).read());
        (((&raw mut packet).cast::<u8>()).wrapping_add(6)).write(funcId);
        (((&raw mut packet).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .write(playAgainState);
        Rfu_SendPacket((&raw mut packet).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_MemberStateToLeader(
    player: *mut u8,
    multiplayerId: i32,
    funcId: *mut u8,
    playAgainState: *mut u16,
) -> u32 {
    unsafe {
        let mut player = player;
        let mut multiplayerId = multiplayerId;
        let mut funcId = funcId;
        let mut playAgainState = playAgainState;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        if (((((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset((multiplayerId) as isize * 16))
        .cast::<u16>())
        .read()) as i32)
            & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        crate::c::memcpy(
            (&raw mut packet).cast::<u8>(),
            (((((&raw mut gRecvCmds).cast::<u8>())
                .wrapping_offset((multiplayerId) as isize * 16))
            .cast::<u16>())
            .wrapping_offset(1))
            .cast::<u8>(),
            12u32,
        );
        if ((((&raw mut packet).cast::<u8>()).read()) as i32) != 4i32 {
            return 0u32;
        }
        ((player).wrapping_add(16).cast::<u16>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(1)).read()) as u16));
        ((player).wrapping_add(20).cast::<i32>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(2)).read()) as i32));
        ((player).wrapping_add(24).cast::<u32>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(3)).read()) as u32));
        ((player).wrapping_add(14).cast::<u16>()).write(
            (((&raw mut packet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        (funcId).write((((&raw mut packet).cast::<u8>()).wrapping_add(6)).read());
        (playAgainState).write(
            (((&raw mut packet).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn RecvPacket_MemberStateToMember(
    player: *mut u8,
    multiplayerId: i32,
) -> u32 {
    unsafe {
        let mut player = player;
        let mut multiplayerId = multiplayerId;
        let mut packet = crate::ffi::Align4([0u8; 12]);
        if (((((((&raw mut gRecvCmds).cast::<u8>())
            .wrapping_offset((multiplayerId) as isize * 16))
        .cast::<u16>())
        .read()) as i32)
            & 65280i32)
            != 12032i32
        {
            return 0u32;
        }
        crate::c::memcpy(
            (&raw mut packet).cast::<u8>(),
            (((((&raw mut gRecvCmds).cast::<u8>())
                .wrapping_offset((multiplayerId) as isize * 16))
            .cast::<u16>())
            .wrapping_offset(1))
            .cast::<u8>(),
            12u32,
        );
        if ((((&raw mut packet).cast::<u8>()).read()) as i32) != 4i32 {
            return 0u32;
        }
        ((player).wrapping_add(16).cast::<u16>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(1)).read()) as u16));
        ((player).wrapping_add(20).cast::<i32>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(2)).read()) as i32));
        ((player).wrapping_add(24).cast::<u32>())
            .write((((((&raw mut packet).cast::<u8>()).wrapping_add(3)).read()) as u32));
        ((player).wrapping_add(14).cast::<u16>()).write(
            (((&raw mut packet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn GetPokeJumpRecords() -> *mut u8 {
    unsafe {
        return (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(508);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokemonJumpRecords() {
    unsafe {
        let mut records: *mut u8 = GetPokeJumpRecords();
        ((records).cast::<u16>()).write(0u16);
        ((records).wrapping_add(12).cast::<u32>()).write(0u32);
        ((records).wrapping_add(4).cast::<u16>()).write(0u16);
        ((records).wrapping_add(6).cast::<u16>()).write(0u16);
        ((records).wrapping_add(8).cast::<u32>()).write(0u32);
        ((records).wrapping_add(2).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn TryUpdateRecords(
    jumpScore: u32,
    jumpsInRow: u16,
    excellentsInRow: u16,
) -> u32 {
    unsafe {
        let mut jumpScore = jumpScore;
        let mut jumpsInRow = jumpsInRow;
        let mut excellentsInRow = excellentsInRow;
        let mut records: *mut u8 = GetPokeJumpRecords();
        let mut newRecord: u32 = 0u32;
        if (((records).wrapping_add(12).cast::<u32>()).read() < jumpScore)
            && (jumpScore <= 99990u32)
        {
            ((records).wrapping_add(12).cast::<u32>()).write(jumpScore);
            newRecord = 1u32;
        }
        if (((((records).cast::<u16>()).read()) as i32) < ((jumpsInRow) as i32))
            && (((jumpsInRow) as i32) <= 9999i32)
        {
            ((records).cast::<u16>()).write(jumpsInRow);
            newRecord = 1u32;
        }
        if (((((records).wrapping_add(4).cast::<u16>()).read()) as i32)
            < ((excellentsInRow) as i32))
            && (((excellentsInRow) as i32) <= 9999i32)
        {
            ((records).wrapping_add(4).cast::<u16>()).write(excellentsInRow);
            newRecord = 1u32;
        }
        return newRecord;
    }
}
pub(crate) unsafe extern "C" fn IncrementGamesWithMaxPlayers() {
    unsafe {
        let mut records: *mut u8 = GetPokeJumpRecords();
        if ((((records).wrapping_add(6).cast::<u16>()).read()) as i32) < 9999i32 {
            let __p1 = (records).wrapping_add(6).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonJumpRecords() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_ShowPokemonJumpRecords), 0u8);
        Task_ShowPokemonJumpRecords(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPokemonJumpRecords(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut window = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        let mut width: i32 = 0i32;
        let mut widthCurr: i32 = 0i32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                (&raw mut window)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_Records)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                width = GetStringWidth(1u8, (&raw mut gText_PkmnJumpRecords).cast::<u8>(), 0i16);
                {
                    i = 0i32;
                    'l2: loop {
                        if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                            break 'l2;
                        }
                        'l3: {
                            widthCurr = (GetStringWidth(
                                1u8,
                                ((((&raw const sRecordsTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                                0i16,
                            ))
                            .wrapping_add(38i32);
                            if widthCurr > width {
                                width = widthCurr;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                width = crate::c::div_i32((width).wrapping_add(7i32), 8i32);
                if (width & 1i32) != 0 {
                    width = (width).wrapping_add(1);
                }
                (((&raw mut window).cast::<u8>()).wrapping_add(1))
                    .write(((crate::c::div_i32((30i32).wrapping_sub(width), 2i32)) as u8));
                (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(((width) as u8));
                ((data).wrapping_offset(1))
                    .write(((AddWindow((&raw mut window).cast::<u8>())) as i16));
                PrintRecordsText(((((data).wrapping_offset(1)).read()) as u16), width);
                CopyWindowToVram(((((data).wrapping_offset(1)).read()) as u8), 3u8);
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    rbox_fill_rectangle(((((data).wrapping_offset(1)).read()) as u8));
                    CopyWindowToVram(((((data).wrapping_offset(1)).read()) as u8), 1u8);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    RemoveWindow(((((data).wrapping_offset(1)).read()) as u8));
                    DestroyTask(taskId);
                    ScriptContext_Enable();
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintRecordsText(windowId: u16, width: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut width = width;
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut recordNums = crate::ffi::Align4([0u8; 12]);
        let mut records: *mut u8 = GetPokeJumpRecords();
        ((&raw mut recordNums).cast::<i32>()).write(((((records).cast::<u16>()).read()) as i32));
        (((&raw mut recordNums).cast::<i32>()).wrapping_offset(1))
            .write(((((records).wrapping_add(12).cast::<u32>()).read()) as i32));
        (((&raw mut recordNums).cast::<i32>()).wrapping_offset(2))
            .write(((((records).wrapping_add(4).cast::<u16>()).read()) as i32));
        LoadUserWindowBorderGfx_(((windowId) as u8), 541u16, 208u8);
        DrawTextBorderOuter(((windowId) as u8), 541u16, 13u8);
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut gText_PkmnJumpRecords).cast::<u8>(),
            ((GetStringCenterAlignXOffset(
                1i32,
                (&raw mut gText_PkmnJumpRecords).cast::<u8>(),
                (width).wrapping_mul(8i32),
            )) as u8),
            1u8,
            255u8,
            None,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized(
                        ((windowId) as u8),
                        1u8,
                        ((((&raw const sRecordsTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        0u8,
                        (((25i32).wrapping_add((i).wrapping_mul(16i32))) as u8),
                        255u8,
                        None,
                    );
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((&raw mut recordNums).cast::<i32>()).wrapping_offset((i) as isize))
                            .read(),
                        0i32,
                        5u8,
                    );
                    TruncateToFirstWordOnly((&raw mut gStringVar1).cast::<u8>());
                    x = ((width).wrapping_mul(8i32)).wrapping_sub(GetStringWidth(
                        1u8,
                        (&raw mut gStringVar1).cast::<u8>(),
                        0i16,
                    ));
                    AddTextPrinterParameterized(
                        ((windowId) as u8),
                        1u8,
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((x) as u8),
                        (((25i32).wrapping_add((i).wrapping_mul(16i32))) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(((windowId) as u8));
    }
}
pub(crate) unsafe extern "C" fn TruncateToFirstWordOnly(str: *mut u8) {
    unsafe {
        let mut str = str;
        {
            'l1: loop {
                if !((((str).read()) as i32) != 255i32) {
                    break 'l1;
                }
                'l2: {
                    if (((str).read()) as i32) == 0i32 {
                        (str).write(255u8);
                        break 'l1;
                    }
                }
                str = (str).wrapping_offset(1);
            }
        }
    }
}
