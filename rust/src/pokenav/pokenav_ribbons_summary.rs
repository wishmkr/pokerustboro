//! Translated from `src/pokenav_ribbons_summary.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sRibbonData gRibbonDescriptionPart1_Champion gRibbonDescriptionPart2_Champion gRibbonDescriptionPart1_CoolContest gRibbonDescriptionPart1_BeautyContest gRibbonDescriptionPart1_CuteContest gRibbonDescriptionPart1_SmartContest gRibbonDescriptionPart1_ToughContest gRibbonDescriptionPart2_NormalRank gRibbonDescriptionPart2_SuperRank gRibbonDescriptionPart2_HyperRank gRibbonDescriptionPart2_MasterRank gRibbonDescriptionPart1_Winning gRibbonDescriptionPart2_Winning gRibbonDescriptionPart1_Victory gRibbonDescriptionPart2_Victory gRibbonDescriptionPart1_Artist gRibbonDescriptionPart2_Artist gRibbonDescriptionPart1_Effort gRibbonDescriptionPart2_Effort gRibbonDescriptionPointers gGiftRibbonDescriptionPart1_2003RegionalTourney gGiftRibbonDescriptionPart2_Champion gGiftRibbonDescriptionPart1_2003NationalTourney gGiftRibbonDescriptionPart1_2003GlobalCup gGiftRibbonDescriptionPart2_RunnerUp gGiftRibbonDescriptionPart2_Semifinalist gGiftRibbonDescriptionPart1_2004RegionalTourney gGiftRibbonDescriptionPart1_2004NationalTourney gGiftRibbonDescriptionPart1_2004GlobalCup gGiftRibbonDescriptionPart1_2005RegionalTourney gGiftRibbonDescriptionPart1_2005NationalTourney gGiftRibbonDescriptionPart1_2005GlobalCup gGiftRibbonDescriptionPart1_PokemonBattleCup gGiftRibbonDescriptionPart2_Participation gGiftRibbonDescriptionPart1_PokemonLeague gGiftRibbonDescriptionPart1_AdvanceCup gGiftRibbonDescriptionPart1_PokemonTournament gGiftRibbonDescriptionPart2_Participation2 gGiftRibbonDescriptionPart1_PokemonEvent gGiftRibbonDescriptionPart1_PokemonFestival gGiftRibbonDescriptionPart1_DifficultyClearing gGiftRibbonDescriptionPart2_Commemorative gGiftRibbonDescriptionPart1_ClearingAllChallenges gGiftRibbonDescriptionPart2_ClearingAllChallenges gGiftRibbonDescriptionPart1_100StraightWin gGiftRibbonDescriptionPart1_DarknessTower gGiftRibbonDescriptionPart1_RedTower gGiftRibbonDescriptionPart1_BlackironTower gGiftRibbonDescriptionPart1_FinalTower gGiftRibbonDescriptionPart1_LegendMaking gGiftRibbonDescriptionPart1_PokemonCenterTokyo gGiftRibbonDescriptionPart1_PokemonCenterOsaka gGiftRibbonDescriptionPart1_PokemonCenterNagoya gGiftRibbonDescriptionPart1_PokemonCenterNY gGiftRibbonDescriptionPart1_SummerHolidays gGiftRibbonDescriptionPart2_EmptyString gGiftRibbonDescriptionPart1_WinterHolidays gGiftRibbonDescriptionPart1_SpringHolidays gGiftRibbonDescriptionPart1_Evergreen gGiftRibbonDescriptionPart1_SpecialHoliday gGiftRibbonDescriptionPart1_HardWorker gGiftRibbonDescriptionPart1_LotsOfFriends gGiftRibbonDescriptionPart1_FullOfEnergy gGiftRibbonDescriptionPart1_LovedPokemon gGiftRibbonDescriptionPart2_LovedPokemon gGiftRibbonDescriptionPart1_LoveForPokemon gGiftRibbonDescriptionPart2_LoveForPokemon gGiftRibbonDescriptionPointers sRibbonIcons1_Pal sRibbonIcons2_Pal sRibbonIcons3_Pal sRibbonIcons4_Pal sRibbonIcons5_Pal sMonInfo_Pal sRibbonIconsSmall_Gfx sRibbonIconsBig_Gfx sBgTemplates sRibbonsSummaryMenuLoopTaskFuncs sRibbonCountWindowTemplate sRibbonSummaryMonNameWindowTemplate sText_MaleSymbol sText_FemaleSymbol sGenderlessIconString sRibbonMonListIndexWindowTemplate sRibbonGfxData sSpriteSheet_RibbonIconsBig sSpritePalettes_RibbonIcons sOamData_RibbonIconBig sAffineAnim_RibbonIconBig_Normal sAffineAnim_RibbonIconBig_ZoomIn sAffineAnim_RibbonIconBig_ZoomOut sAffineAnims_RibbonIconBig sSpriteTemplate_RibbonIconBig
#[allow(unused_imports)]
use crate::data::pokenav_ribbons_summary::*;

pub(crate) static mut sRibbonDraw_Total: u32 = 0u32;
pub(crate) static mut sRibbonDraw_Current: u32 = 0u32;

unsafe extern "C" {
    static mut gKeyRepeatContinueDelay: u8;
    static mut gKeyRepeatStartDelay: u8;
    static mut gMain: u8;
    static mut gPlayerParty: u8;
    static mut gPokenavRibbonsSummaryBg_Gfx: u8;
    static mut gPokenavRibbonsSummaryBg_Pal: u8;
    static mut gPokenavRibbonsSummaryBg_Tilemap: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gText_RibbonsF700: u8;
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
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
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
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBoxMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonDataAt(a0: u8, a1: u8, a2: i32) -> u32;
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetLevelFromMonExp(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut u8, a1: i32);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn PlaySE(a0: u16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavFillPalette(a0: u32, a1: u16);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PrintHelpBarText(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetAllPicSprites() -> u16;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_RibbonsSummaryMenu() -> u32 {
    unsafe {
        let mut list: *mut u8 = AllocSubstruct(13u32, 156u32);
        if ((list) as usize) == 0usize {
            return 0u32;
        }
        ((list).wrapping_add(8).cast::<*mut u8>()).write(GetSubstructPtr(18u32));
        if ((((list).wrapping_add(8).cast::<*mut u8>()).read()) as usize) == 0usize {
            return 0u32;
        }
        GetMonRibbons(list);
        ((list)
            .wrapping_add(152)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(RibbonsSummaryHandleInput));
        ((&raw mut gKeyRepeatContinueDelay).cast::<u16>()).write(3u16);
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(10u16);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonsSummaryMenuCallback() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        return (((list)
            .wrapping_add(152)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .read())
        .unwrap_unchecked()(list);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsSummaryScreen1() {
    unsafe {
        FreePokenavSubstruct(13u32);
    }
}
pub(crate) unsafe extern "C" fn RibbonsSummaryHandleInput(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((list).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            let __p1 = (((list).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((list).wrapping_add(12).cast::<u16>()).write(0u16);
            GetMonRibbons(list);
            return 1u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((((list).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                < ((((((list).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u16>()).read())
                    as i32)
                    .wrapping_sub(1i32))
        {
            let __p2 = (((list).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((list).wrapping_add(12).cast::<u16>()).write(0u16);
            GetMonRibbons(list);
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((list)
                .wrapping_add(152)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleExpandedRibbonInput));
            return 2u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((list)
                .wrapping_add(152)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(ReturnToRibbonsListFromSummary));
            return 5u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleExpandedRibbonInput(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && ((TrySelectRibbonUp(list)) != 0)
        {
            return 3u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && ((TrySelectRibbonDown(list)) != 0)
        {
            return 3u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && ((TrySelectRibbonLeft(list)) != 0)
        {
            return 3u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && ((TrySelectRibbonRight(list)) != 0)
        {
            return 3u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((list)
                .wrapping_add(152)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(RibbonsSummaryHandleInput));
            return 4u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ReturnToRibbonsListFromSummary(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        return 100014u32;
    }
}
pub(crate) unsafe extern "C" fn TrySelectRibbonUp(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32) < 25i32 {
            if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32) < 9i32 {
                return 0u32;
            }
            let __p1 = (list).wrapping_add(12).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(9i32)) as u16));
            return 1u32;
        }
        if ((((list).wrapping_add(16).cast::<u16>()).read()) as i32) != 0i32 {
            let mut ribbonPos: u32 = ((((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                .wrapping_sub(
                    (9i32).wrapping_mul((1i32).wrapping_add(crate::c::div_i32(25i32, 9i32))),
                )) as u32);
            ((list).wrapping_add(12).cast::<u16>()).write(
                (((ribbonPos)
                    .wrapping_add(((((list).wrapping_add(14).cast::<u16>()).read()) as u32)))
                    as u16),
            );
            if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                >= ((((list).wrapping_add(16).cast::<u16>()).read()) as i32)
            {
                ((list).wrapping_add(12).cast::<u16>()).write(
                    ((((((list).wrapping_add(16).cast::<u16>()).read()) as i32).wrapping_sub(1i32))
                        as u16),
                );
            }
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TrySelectRibbonDown(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32) >= 25i32 {
            return 0u32;
        }
        if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
            < ((((list).wrapping_add(14).cast::<u16>()).read()) as i32)
        {
            let __p1 = (list).wrapping_add(12).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(9i32)) as u16));
            if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                >= ((((list).wrapping_add(16).cast::<u16>()).read()) as i32)
            {
                ((list).wrapping_add(12).cast::<u16>()).write(
                    ((((((list).wrapping_add(16).cast::<u16>()).read()) as i32).wrapping_sub(1i32))
                        as u16),
                );
            }
            return 1u32;
        }
        if ((((list).wrapping_add(18).cast::<u16>()).read()) as i32) != 0i32 {
            let mut ribbonPos: i32 = ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                .wrapping_sub(((((list).wrapping_add(14).cast::<u16>()).read()) as i32));
            if ribbonPos >= ((((list).wrapping_add(18).cast::<u16>()).read()) as i32) {
                ribbonPos =
                    ((((list).wrapping_add(18).cast::<u16>()).read()) as i32).wrapping_sub(1i32);
            }
            ((list).wrapping_add(12).cast::<u16>()).write(
                (((ribbonPos).wrapping_add(
                    (9i32).wrapping_mul((1i32).wrapping_add(crate::c::div_i32(25i32, 9i32))),
                )) as u16),
            );
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TrySelectRibbonLeft(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        let mut column: u16 = ((crate::c::rem_i32(
            ((((list).wrapping_add(12).cast::<u16>()).read()) as i32),
            9i32,
        )) as u16);
        if ((column) as i32) != 0i32 {
            let __p1 = (list).wrapping_add(12).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TrySelectRibbonRight(list: *mut u8) -> u32 {
    unsafe {
        let mut list = list;
        let mut column: i32 = crate::c::rem_i32(
            ((((list).wrapping_add(12).cast::<u16>()).read()) as i32),
            9i32,
        );
        if column >= 8i32 {
            return 0u32;
        }
        if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
            < (9i32).wrapping_mul((1i32).wrapping_add(crate::c::div_i32(25i32, 9i32)))
        {
            if ((((list).wrapping_add(12).cast::<u16>()).read()) as i32)
                < ((((list).wrapping_add(16).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            {
                let __p1 = (list).wrapping_add(12).cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                return 1u32;
            }
        } else {
            if column < ((((list).wrapping_add(18).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            {
                let __p2 = (list).wrapping_add(12).cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetRibbonsSummaryCurrentIndex() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        return ((((((list).wrapping_add(8).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as u32);
    }
}
pub(crate) unsafe extern "C" fn GetRibbonsSummaryMonListCount() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        return ((((((list).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u16>()).read())
            as u32);
    }
}
pub(crate) unsafe extern "C" fn GetMonNicknameLevelGender(
    nick: *mut u8,
    level: *mut u8,
    gender: *mut u8,
) {
    unsafe {
        let mut nick = nick;
        let mut level = level;
        let mut gender = gender;
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        let mut mons: *mut u8 = ((list).wrapping_add(8).cast::<*mut u8>()).read();
        let mut monInfo: *mut u8 = (((mons).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((((mons).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4);
        if (((monInfo).read()) as i32) == 14i32 {
            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((monInfo).wrapping_add(1)).read()) as i32) as isize * 100);
            GetMonData3(mon, 2i32, nick);
            (level).write(GetLevelFromMonExp(mon));
            (gender).write(GetMonGender(mon));
        } else {
            let mut boxMon: *mut u8 =
                GetBoxedMonPtr((monInfo).read(), ((monInfo).wrapping_add(1)).read());
            (gender).write(GetBoxMonGender(boxMon));
            (level).write(GetLevelFromBoxMonExp(boxMon));
            GetBoxMonData3(boxMon, 2i32, nick);
        }
        StringGet_Nickname(nick);
    }
}
pub(crate) unsafe extern "C" fn GetMonSpeciesPersonalityOtId(
    species: *mut u16,
    personality: *mut u32,
    otId: *mut u32,
) {
    unsafe {
        let mut species = species;
        let mut personality = personality;
        let mut otId = otId;
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        let mut mons: *mut u8 = ((list).wrapping_add(8).cast::<*mut u8>()).read();
        let mut monInfo: *mut u8 = (((mons).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((((mons).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4);
        if (((monInfo).read()) as i32) == 14i32 {
            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((monInfo).wrapping_add(1)).read()) as i32) as isize * 100);
            (species).write(((GetMonData2(mon, 11i32)) as u16));
            (personality).write(GetMonData2(mon, 0i32));
            (otId).write(GetMonData2(mon, 1i32));
        } else {
            let mut boxMon: *mut u8 =
                GetBoxedMonPtr((monInfo).read(), ((monInfo).wrapping_add(1)).read());
            (species).write(((GetBoxMonData2(boxMon, 11i32)) as u16));
            (personality).write(GetBoxMonData2(boxMon, 0i32));
            (otId).write(GetBoxMonData2(boxMon, 1i32));
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrMonRibbonCount() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        let mut mons: *mut u8 = ((list).wrapping_add(8).cast::<*mut u8>()).read();
        let mut monInfo: *mut u8 = (((mons).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((((mons).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4);
        if (((monInfo).read()) as i32) == 14i32 {
            return GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((((monInfo).wrapping_add(1)).read()) as i32) as isize * 100),
                82i32,
            );
        } else {
            return GetBoxMonDataAt((monInfo).read(), ((monInfo).wrapping_add(1)).read(), 82i32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonRibbons(list: *mut u8) {
    unsafe {
        let mut list = list;
        let mut ribbonFlags: u32 = 0u32;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut mons: *mut u8 = ((list).wrapping_add(8).cast::<*mut u8>()).read();
        let mut monInfo: *mut u8 = (((mons).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((((mons).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4);
        if (((monInfo).read()) as i32) == 14i32 {
            ribbonFlags = GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((((monInfo).wrapping_add(1)).read()) as i32) as isize * 100),
                83i32,
            );
        } else {
            ribbonFlags =
                GetBoxMonDataAt((monInfo).read(), ((monInfo).wrapping_add(1)).read(), 83i32);
        }
        ((list).wrapping_add(16).cast::<u16>()).write(0u16);
        ((list).wrapping_add(18).cast::<u16>()).write(0u16);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(68u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut numRibbons: i32 = (((((crate::c::shl_i32(
                        1i32,
                        ((((((&raw const sRibbonData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .read()) as u32),
                    ))
                    .wrapping_sub(1i32)) as u32)
                        & ribbonFlags) as i32);
                    if !(((((((&raw const sRibbonData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                    .wrapping_add(3))
                    .read())
                        != 0)
                    {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < numRibbons) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((list).wrapping_add(20)).cast::<u32>()).wrapping_offset(
                                        (({
                                            let __p1 = (list).wrapping_add(16).cast::<u16>();
                                            let __t2 = (__p1).read();
                                            (__p1).write(((__p1).read()).wrapping_add(1));
                                            __t2
                                        }) as i32) as isize,
                                    ))
                                    .write(
                                        (((((((((&raw const sRibbonData)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(2))
                                        .read()) as i32)
                                            .wrapping_add(j))
                                            as u32),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    } else {
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < numRibbons) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((((list).wrapping_add(120)).cast::<u32>()).wrapping_offset(
                                        (({
                                            let __p3 = (list).wrapping_add(18).cast::<u16>();
                                            let __t4 = (__p3).read();
                                            (__p3).write(((__p3).read()).wrapping_add(1));
                                            __t4
                                        }) as i32) as isize,
                                    ))
                                    .write(
                                        (((((((((&raw const sRibbonData)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 4))
                                        .wrapping_add(2))
                                        .read()) as i32)
                                            .wrapping_add(j))
                                            as u32),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    ribbonFlags = crate::c::shr_u32(
                        ribbonFlags,
                        ((((((&raw const sRibbonData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .read()) as u32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((list).wrapping_add(16).cast::<u16>()).read()) as i32) != 0i32 {
            ((list).wrapping_add(14).cast::<u16>()).write(
                (((crate::c::div_i32(
                    ((((list).wrapping_add(16).cast::<u16>()).read()) as i32).wrapping_sub(1i32),
                    9i32,
                ))
                .wrapping_mul(9i32)) as u16),
            );
            ((list).wrapping_add(12).cast::<u16>()).write(0u16);
        } else {
            ((list).wrapping_add(14).cast::<u16>()).write(0u16);
            ((list).wrapping_add(12).cast::<u16>()).write(
                (((9i32).wrapping_mul((1i32).wrapping_add(crate::c::div_i32(25i32, 9i32)))) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetNormalRibbonIds(size: *mut u32) -> *mut u32 {
    unsafe {
        let mut size = size;
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        (size).write(((((list).wrapping_add(16).cast::<u16>()).read()) as u32));
        return ((list).wrapping_add(20)).cast::<u32>();
    }
}
pub(crate) unsafe extern "C" fn GetGiftRibbonIds(size: *mut u32) -> *mut u32 {
    unsafe {
        let mut size = size;
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        (size).write(((((list).wrapping_add(18).cast::<u16>()).read()) as u32));
        return ((list).wrapping_add(120)).cast::<u32>();
    }
}
pub(crate) unsafe extern "C" fn GetSelectedPosition() -> u16 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        return ((list).wrapping_add(12).cast::<u16>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetRibbonId() -> u32 {
    unsafe {
        let mut list: *mut u8 = GetSubstructPtr(13u32);
        let mut ribbonPos: i32 = ((((list).wrapping_add(12).cast::<u16>()).read()) as i32);
        if ribbonPos < 25i32 {
            return ((((list).wrapping_add(20)).cast::<u32>())
                .wrapping_offset((ribbonPos) as isize))
            .read();
        } else {
            return ((((list).wrapping_add(120)).cast::<u32>()).wrapping_offset(
                ((ribbonPos).wrapping_sub(
                    (9i32).wrapping_mul((1i32).wrapping_add(crate::c::div_i32(25i32, 9i32))),
                )) as isize,
            ))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenRibbonsSummaryMenu() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(14u32, 4124u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ((menu).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            Some(LoopedTask_OpenRibbonsSummaryMenu),
            1u32,
        ));
        ((menu).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRibbonsSummaryLoopedTask(id: i32) {
    unsafe {
        let mut id = id;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        ((menu).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sRibbonsSummaryMenuLoopTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((id) as isize))
            .read(),
            1u32,
        ));
        ((menu).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRibbonsSummaryLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        return (((menu).cast::<Option<unsafe extern "C" fn() -> u32>>()).read())
            .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRibbonsSummaryScreen2() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        RemoveWindow(((((menu).wrapping_add(10).cast::<u16>()).read()) as u8));
        RemoveWindow(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8));
        RemoveWindow(((((menu).wrapping_add(12).cast::<u16>()).read()) as u8));
        RemoveWindow(((((menu).wrapping_add(14).cast::<u16>()).read()) as u8));
        DestroyRibbonsMonFrontPic(menu);
        FreeSpriteTilesByTag(9u16);
        FreeSpritePaletteByTag(15u16);
        FreeSpritePaletteByTag(16u16);
        FreeSpritePaletteByTag(17u16);
        FreeSpritePaletteByTag(18u16);
        FreeSpritePaletteByTag(19u16);
        FreeSpriteOamMatrix(((menu).wrapping_add(20).cast::<*mut u8>()).read());
        DestroySprite(((menu).wrapping_add(20).cast::<*mut u8>()).read());
        FreePokenavSubstruct(14u32);
    }
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        return IsLoopedTaskActive(((menu).wrapping_add(4).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRibbonsSummaryMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                InitBgTemplates(
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(8u32, 4u32)) as i32),
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gPokenavRibbonsSummaryBg_Gfx).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                SetBgTilemapBuffer(2u8, (((menu).wrapping_add(28)).cast::<u8>()).cast::<u8>());
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw mut gPokenavRibbonsSummaryBg_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw mut gPokenavRibbonsSummaryBg_Pal).cast::<u16>()).cast::<u16>(),
                    16u32,
                    32u32,
                );
                CopyBgTilemapBufferToVram(2u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    BgDmaFill(1u32, 0u8, 0i32, 1i32);
                    DecompressAndCopyTileDataToVram(
                        1u8,
                        (((&raw const sRibbonIconsSmall_Gfx)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>())
                        .cast::<u8>(),
                        0u32,
                        1u16,
                        0u8,
                    );
                    SetBgTilemapBuffer(
                        1u8,
                        ((((menu).wrapping_add(28)).cast::<u8>()).wrapping_offset(2048))
                            .cast::<u8>(),
                    );
                    FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 20u8);
                    CopyPaletteIntoBufferUnfaded(
                        ((&raw const sRibbonIcons1_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>(),
                        32u32,
                        160u32,
                    );
                    CopyPaletteIntoBufferUnfaded(
                        ((&raw const sMonInfo_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>(),
                        160u32,
                        32u32,
                    );
                    CopyBgTilemapBufferToVram(1u8);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 2i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    AddRibbonCountWindow(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 3i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    AddRibbonSummaryMonNameWindow(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 4i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    AddRibbonListIndexWindow(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 5i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CopyBgTilemapBufferToVram(2u8);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 6i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ResetSpritesAndDrawMonFrontPic(menu);
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 7i32 {
                DrawAllRibbonsSmall(menu);
                PrintHelpBarText(10u32);
                return 0u32;
            }
            if __sw1 == 8i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateBigRibbonSprite(menu);
                    ChangeBgX(1u8, 0i32, 0u8);
                    ChangeBgY(1u8, 0i32, 0u8);
                    ChangeBgX(2u8, 0i32, 0u8);
                    ChangeBgY(2u8, 0i32, 0u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    HideBg(3u8);
                    PokenavFadeScreen(1i32);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 9i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitRibbonsSummaryMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PokenavFadeScreen(0i32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                return 4u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_SwitchRibbonsSummaryMon(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                SlideMonSpriteOff(menu);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if !((IsMonSpriteAnimating(menu)) != 0) {
                    PrintRibbbonsSummaryMonInfo(menu);
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 2i32 {
                DrawAllRibbonsSmall(menu);
                return 1u32;
            }
            if __sw1 == 3i32 {
                PrintRibbonsMonListIndex(menu);
                return 1u32;
            }
            if __sw1 == 4i32 {
                PrintCurrentMonRibbonCount(menu);
                return 1u32;
            }
            if __sw1 == 5i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SlideMonSpriteOn(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 6i32 {
                if (IsMonSpriteAnimating(menu)) != 0 {
                    return 2u32;
                }
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ExpandSelectedRibbon(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                UpdateAndZoomInSelectedRibbon(menu);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if !((IsRibbonAnimating(menu)) != 0) {
                    PrintRibbonNameAndDescription(menu);
                    PrintHelpBarText(11u32);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 2i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveRibbonsCursorExpanded(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ZoomOutSelectedRibbon(menu);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if !((IsRibbonAnimating(menu)) != 0) {
                    UpdateAndZoomInSelectedRibbon(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 2i32 {
                if !((IsRibbonAnimating(menu)) != 0) {
                    PrintRibbonNameAndDescription(menu);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ShrinkExpandedRibbon(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(14u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ZoomOutSelectedRibbon(menu);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if !((IsRibbonAnimating(menu)) != 0) {
                    PrintCurrentMonRibbonCount(menu);
                    PrintHelpBarText(10u32);
                    return 0u32;
                }
                return 2u32;
            }
            if __sw1 == 2i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn AddRibbonCountWindow(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        ((menu).wrapping_add(10).cast::<u16>()).write(AddWindow(
            (&raw const sRibbonCountWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        PutWindowTilemap(((((menu).wrapping_add(10).cast::<u16>()).read()) as u8));
        PrintCurrentMonRibbonCount(menu);
    }
}
pub(crate) unsafe extern "C" fn PrintCurrentMonRibbonCount(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut color = crate::ffi::Align4([0u8; 3]);
        (&raw mut color).cast::<u8>().wrapping_add(0).write(4u8);
        (&raw mut color).cast::<u8>().wrapping_add(1).write(2u8);
        (&raw mut color).cast::<u8>().wrapping_add(2).write(3u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((GetCurrMonRibbonCount()) as i32),
            0i32,
            2u8,
        );
        DynamicPlaceholderTextUtil_Reset();
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut gStringVar1).cast::<u8>());
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_RibbonsF700).cast::<u8>(),
        );
        FillWindowPixelBuffer(
            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
            68u8,
        );
        AddTextPrinterParameterized3(
            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
            1u8,
            0u8,
            1u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
        CopyWindowToVram(
            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintRibbonNameAndDescription(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut i: i32 = 0i32;
        let mut ribbonId: u32 = GetRibbonId();
        let mut color = crate::ffi::Align4([0u8; 3]);
        (&raw mut color).cast::<u8>().wrapping_add(0).write(4u8);
        (&raw mut color).cast::<u8>().wrapping_add(1).write(2u8);
        (&raw mut color).cast::<u8>().wrapping_add(2).write(3u8);
        FillWindowPixelBuffer(
            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
            68u8,
        );
        if ribbonId < 25u32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        AddTextPrinterParameterized3(
                            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
                            1u8,
                            0u8,
                            ((((i).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                            (&raw mut color).cast::<u8>(),
                            (-1i8),
                            ((((((&raw const gRibbonDescriptionPointers)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((ribbonId) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            ribbonId = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(12712))
            .cast::<u8>())
            .wrapping_offset((((ribbonId).wrapping_sub(25u32)) as i32) as isize))
            .read()) as u32);
            if ribbonId == 0u32 {
                return;
            }
            ribbonId = (ribbonId).wrapping_sub(1);
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 2i32) {
                        break 'l3;
                    }
                    'l4: {
                        AddTextPrinterParameterized3(
                            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
                            1u8,
                            0u8,
                            ((((i).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                            (&raw mut color).cast::<u8>(),
                            (-1i8),
                            ((((((&raw const gGiftRibbonDescriptionPointers)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((ribbonId) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        CopyWindowToVram(
            ((((menu).wrapping_add(10).cast::<u16>()).read()) as u8),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn AddRibbonSummaryMonNameWindow(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        ((menu).wrapping_add(8).cast::<u16>()).write(AddWindow(
            (&raw const sRibbonSummaryMonNameWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        PutWindowTilemap(((((menu).wrapping_add(8).cast::<u16>()).read()) as u8));
        PrintRibbbonsSummaryMonInfo(menu);
    }
}
pub(crate) unsafe extern "C" fn PrintRibbbonsSummaryMonInfo(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut genderTxt: *mut u8 = core::ptr::null_mut();
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut level: u8 = 0u8;
        let mut gender: u8 = 0u8;
        let mut windowId: u16 = ((menu).wrapping_add(8).cast::<u16>()).read();
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        GetMonNicknameLevelGender(
            (&raw mut gStringVar3).cast::<u8>(),
            &raw mut level,
            &raw mut gender,
        );
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut gStringVar3).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        'l1: {
            let __sw1 = ((gender) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 254i32;
            if __sw1 == 0i32 {
                genderTxt = ((&raw const sText_MaleSymbol).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 254i32 {
                genderTxt = ((&raw const sText_FemaleSymbol).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
            if !__matched {
                genderTxt =
                    ((&raw const sGenderlessIconString).cast::<u8>().cast_mut()).cast::<u8>();
                break 'l1;
            }
        }
        txtPtr = StringCopy((&raw mut gStringVar1).cast::<u8>(), genderTxt);
        ({
            let __t2 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t2
        })
        .write(186u8);
        ({
            let __t3 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t3
        })
        .write(249u8);
        ({
            let __t4 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t4
        })
        .write(5u8);
        ConvertIntToDecimalStringN(txtPtr, ((level) as i32), 0i32, 3u8);
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut gStringVar1).cast::<u8>(),
            60u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(((windowId) as u8), 2u8);
    }
}
pub(crate) unsafe extern "C" fn AddRibbonListIndexWindow(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        ((menu).wrapping_add(12).cast::<u16>()).write(AddWindow(
            ((&raw const sRibbonMonListIndexWindowTemplate)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        ));
        FillWindowPixelBuffer(
            ((((menu).wrapping_add(12).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((menu).wrapping_add(12).cast::<u16>()).read()) as u8));
        PrintRibbonsMonListIndex(menu);
    }
}
pub(crate) unsafe extern "C" fn PrintRibbonsMonListIndex(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut x: i32 = 0i32;
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut id: u32 = (GetRibbonsSummaryCurrentIndex()).wrapping_add(1u32);
        let mut count: u32 = GetRibbonsSummaryMonListCount();
        txtPtr = ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((id) as i32),
            1i32,
            3u8,
        );
        ({
            let __t1 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t1
        })
        .write(186u8);
        ConvertIntToDecimalStringN(txtPtr, ((count) as i32), 1i32, 3u8);
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar1).cast::<u8>(), 56i32);
        AddTextPrinterParameterized(
            ((((menu).wrapping_add(12).cast::<u16>()).read()) as u8),
            1u8,
            (&raw mut gStringVar1).cast::<u8>(),
            ((x) as u8),
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((((menu).wrapping_add(12).cast::<u16>()).read()) as u8),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ResetSpritesAndDrawMonFrontPic(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        GetMonSpeciesPersonalityOtId(&raw mut species, &raw mut personality, &raw mut otId);
        ResetAllPicSprites();
        ((menu).wrapping_add(16).cast::<u16>()).write(DrawRibbonsMonFrontPic(40i32, 104i32));
        PokenavFillPalette(15u32, 0u16);
    }
}
pub(crate) unsafe extern "C" fn DestroyRibbonsMonFrontPic(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        FreeAndDestroyMonPicSprite(((menu).wrapping_add(16).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn DrawRibbonsMonFrontPic(x: i32, y: i32) -> u16 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut species: u16 = 0u16;
        let mut spriteId: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        GetMonSpeciesPersonalityOtId(&raw mut species, &raw mut personality, &raw mut otId);
        spriteId = CreateMonPicSprite_HandleDeoxys(
            species,
            otId,
            personality,
            1u8,
            40i16,
            104i16,
            15u8,
            65535u16,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SlideMonSpriteOff(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        StartMonSpriteSlide(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 68,
            ),
            40i32,
            (-32i32),
            6i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SlideMonSpriteOn(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        FreeAndDestroyMonPicSprite(((menu).wrapping_add(16).cast::<u16>()).read());
        ((menu).wrapping_add(16).cast::<u16>()).write(DrawRibbonsMonFrontPic((-32i32), 104i32));
        StartMonSpriteSlide(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 68,
            ),
            (-32i32),
            40i32,
            6i32,
        );
    }
}
pub(crate) unsafe extern "C" fn IsMonSpriteAnimating(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        return ((core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) != (SpriteCallbackDummy as *const () as usize)) as u32);
    }
}
pub(crate) unsafe extern "C" fn StartMonSpriteSlide(
    sprite: *mut u8,
    startX: i32,
    destX: i32,
    time: i32,
) {
    unsafe {
        let mut sprite = sprite;
        let mut startX = startX;
        let mut destX = destX;
        let mut time = time;
        let mut delta: u32 = (((destX).wrapping_sub(startX)) as u32);
        ((sprite).wrapping_add(32).cast::<i16>()).write(((startX) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((startX << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((crate::c::div_u32((delta << 4), ((time) as u32))) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((time) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(((destX) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MonSpriteSlide));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MonSpriteSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 4) as i16),
            );
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= (-32i32) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        } else {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn DrawAllRibbonsSmall(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut ribbonIds: *mut u32 = core::ptr::null_mut();
        ClearRibbonsSummaryBg();
        ribbonIds = GetNormalRibbonIds((&raw mut sRibbonDraw_Total).cast::<u8>().cast::<u32>());
        {
            ((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).write(0u32);
            'l1: loop {
                if !(((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).read()
                    < ((&raw mut sRibbonDraw_Total).cast::<u8>().cast::<u32>()).read())
                {
                    break 'l1;
                }
                'l2: {
                    DrawRibbonSmall(
                        ((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).read(),
                        ({
                            let __t2 = ribbonIds;
                            ribbonIds = (ribbonIds).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                let __p3 = (&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
        ribbonIds = GetGiftRibbonIds((&raw mut sRibbonDraw_Total).cast::<u8>().cast::<u32>());
        {
            ((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).write(0u32);
            'l3: loop {
                if !(((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).read()
                    < ((&raw mut sRibbonDraw_Total).cast::<u8>().cast::<u32>()).read())
                {
                    break 'l3;
                }
                'l4: {
                    DrawRibbonSmall(
                        (((&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>()).read())
                            .wrapping_add(
                                (((9i32).wrapping_mul(
                                    (1i32).wrapping_add(crate::c::div_i32(25i32, 9i32)),
                                )) as u32),
                            ),
                        ({
                            let __t5 = ribbonIds;
                            ribbonIds = (ribbonIds).wrapping_offset(1);
                            __t5
                        })
                        .read(),
                    );
                }
                let __p6 = (&raw mut sRibbonDraw_Current).cast::<u8>().cast::<u32>();
                (__p6).write(((__p6).read()).wrapping_add(1));
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn ClearRibbonsSummaryBg() {
    unsafe {
        FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 20u8);
    }
}
pub(crate) unsafe extern "C" fn DrawRibbonSmall(i: u32, ribbonId: u32) {
    unsafe {
        let mut i = i;
        let mut ribbonId = ribbonId;
        let mut bgData = crate::ffi::Align4([0u8; 8]);
        let mut destX: u32 = ((crate::c::rem_u32(i, 9u32)).wrapping_mul(2u32)).wrapping_add(11u32);
        let mut destY: u32 = ((crate::c::div_u32(i, 9u32)).wrapping_mul(2u32)).wrapping_add(4u32);
        BufferSmallRibbonGfxData((&raw mut bgData).cast::<u16>(), ribbonId);
        CopyToBgTilemapBufferRect(
            1u8,
            ((&raw mut bgData).cast::<u16>()).cast::<u8>(),
            ((destX) as u8),
            ((destY) as u8),
            2u8,
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferSmallRibbonGfxData(dst: *mut u16, ribbonId: u32) {
    unsafe {
        let mut dst = dst;
        let mut ribbonId = ribbonId;
        let mut palNum: u16 = (((((((((&raw const sRibbonGfxData).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((ribbonId) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_add(2i32)) as u16);
        let mut tileNum: u16 = ((((((((((&raw const sRibbonGfxData).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((ribbonId) as i32) as isize * 4))
        .cast::<u16>())
        .read()) as i32)
            .wrapping_mul(2i32))
        .wrapping_add(1i32)) as u16);
        (dst).write(((((tileNum) as i32) | (((palNum) as i32) << 12)) as u16));
        ((dst).wrapping_offset(1))
            .write((((((tileNum) as i32) | (((palNum) as i32) << 12)) | 1024i32) as u16));
        ((dst).wrapping_offset(2))
            .write(((((tileNum) as i32).wrapping_add(1i32) | (((palNum) as i32) << 12)) as u16));
        ((dst).wrapping_offset(3)).write(
            (((((tileNum) as i32).wrapping_add(1i32) | (((palNum) as i32) << 12)) | 1024i32)
                as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateBigRibbonSprite(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_RibbonIconsBig)
                .cast::<u8>()
                .cast_mut(),
        );
        Pokenav_AllocAndLoadPalettes(
            ((&raw const sSpritePalettes_RibbonIcons)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_RibbonIconBig)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            0u8,
        );
        ((menu).wrapping_add(20).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            (((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateAndZoomInSelectedRibbon(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut ribbonId: u32 = 0u32;
        let mut position: i32 = ((GetSelectedPosition()) as i32);
        let mut x: i32 =
            ((crate::c::rem_i32(position, 9i32)).wrapping_mul(16i32)).wrapping_add(96i32);
        let mut y: i32 =
            ((crate::c::div_i32(position, 9i32)).wrapping_mul(16i32)).wrapping_add(40i32);
        ((((menu).wrapping_add(20).cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .write(((x) as i16));
        ((((menu).wrapping_add(20).cast::<*mut u8>()).read())
            .wrapping_add(34)
            .cast::<i16>())
        .write(((y) as i16));
        ribbonId = GetRibbonId();
        crate::c::bf_write(
            (((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(4),
            0,
            10,
            ((((((((((&raw const sRibbonGfxData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((ribbonId) as i32) as isize * 4))
            .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(16i32))
            .wrapping_add(((GetSpriteTileStartByTag(9u16)) as i32))) as u16) as i32,
        );
        crate::c::bf_write(
            (((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(5),
            4,
            4,
            ((IndexOfSpritePaletteTag(
                (((((((((&raw const sRibbonGfxData).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((ribbonId) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(15i32)) as u16),
            )) as u16) as i32,
        );
        StartSpriteAffineAnim(((menu).wrapping_add(20).cast::<*mut u8>()).read(), 1u8);
        crate::c::bf_write(
            (((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        (((((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(46)).cast::<i16>())
            .write(0i16);
        ((((menu).wrapping_add(20).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_WaitForRibbonAnimation));
    }
}
pub(crate) unsafe extern "C" fn ZoomOutSelectedRibbon(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        (((((menu).wrapping_add(20).cast::<*mut u8>()).read()).wrapping_add(46)).cast::<i16>())
            .write(1i16);
        StartSpriteAffineAnim(((menu).wrapping_add(20).cast::<*mut u8>()).read(), 2u8);
        ((((menu).wrapping_add(20).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_WaitForRibbonAnimation));
    }
}
pub(crate) unsafe extern "C" fn IsRibbonAnimating(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        return ((core::mem::transmute::<_, usize>(
            ((((menu).wrapping_add(20).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) != (SpriteCallbackDummy as *const () as usize)) as u32);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WaitForRibbonAnimation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16) as i32,
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
