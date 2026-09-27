//! Translated from `src/battle_gfx_sfx_util.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sSpriteSheet_SinglesPlayerHealthbox sSpriteSheet_SinglesOpponentHealthbox sSpriteSheets_DoublesPlayerHealthbox sSpriteSheets_DoublesOpponentHealthbox sSpriteSheet_SafariHealthbox sSpriteSheets_HealthBar sSpritePalettes_HealthBoxHealthBar
#[allow(unused_imports)]
use crate::data::battle_gfx_sfx_util::*;

unsafe extern "C" {
    static mut gActiveBattler: u8;
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleAnims_General: u8;
    static mut gBattleAnims_Special: u8;
    static mut gBattleBufferA: u8;
    static mut gBattleInterfaceGfx_BattleBar: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoves: u8;
    static mut gBattlePalaceNatureToMoveGroupLikelihood: u8;
    static mut gBattlePalaceNatureToMoveTarget: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerSpriteTemplates: u8;
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gContestResources: u8;
    static mut gDecompressionBuffer: u8;
    static mut gEnemyMonElevation: u8;
    static mut gEnemyParty: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gIntroSlideFlags: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMain: u8;
    static mut gMonBackPicTable: u8;
    static mut gMonFrontAnimsPtrTable: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gProtectStructs: u8;
    static mut gSpriteSheet_EnemyShadow: u8;
    static mut gSpriteTemplate_EnemyShadow: u8;
    static mut gSprites: u8;
    static mut gSubstituteDollBackGfx: u8;
    static mut gSubstituteDollFrontGfx: u8;
    static mut gSubstituteDollPal: u8;
    static mut gTasks: u8;
    static mut gTrainerBackPicPaletteTable: u8;
    static mut gTrainerBackPicTable: u8;
    static mut gTrainerFrontPicPaletteTable: u8;
    static mut gTrainerFrontPicTable: u8;
    static mut gTransformedPersonalities: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprite(a0: *mut u8);
    fn BattleAI_ChooseMoveOrAction() -> u8;
    fn BattleAI_SetupAIData(a0: u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BufferBattlePartyCurrentOrder();
    fn CheckMoveLimitations(a0: u8, a1: u8, a2: u8) -> u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateBattlerHealthboxSprites(a0: u8) -> u8;
    fn CreateSafariPlayerHealthboxSprites() -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressPicFromTable_2(a0: *mut u8, a1: *mut u8, a2: i32);
    fn DestroyTask(a0: u8);
    fn DummyBattleInterfaceFunc(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(a0: u8) -> u8;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonFrontSpritePal(a0: *mut u8) -> *mut u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetPartyIdFromBattlePartyId(a0: u8) -> u8;
    fn GetSubstituteSpriteDefault_Y(a0: u8) -> u8;
    fn HandleLoadSpecialPokePic(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn InitBattlerHealthboxCoords(a0: u8);
    fn IsBattlerSpritePresent(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsSEPlaying() -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LaunchBattleAnimation(a0: *mut *mut u8, a1: u16, a2: u8);
    fn LaunchStatusAnimation(a0: u8, a1: u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn ShouldIgnoreDeoxysForm(a0: u8, a1: u8) -> u8;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn UpdateHealthboxAttribute(a0: u8, a1: *mut u8, a2: u8);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocateBattleSpritesData() {
    unsafe {
        ((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).write(AllocZeroed(16u32));
        ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(AllocZeroed(16u32));
        ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(AllocZeroed(48u32));
        ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .write(AllocZeroed(16u32));
        ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(AllocZeroed(80u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeBattleSpritesData() {
    unsafe {
        if ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()) as usize) == 0usize {
            return;
        }
        {
            Free(
                ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
            );
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        {
            Free(((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read());
            ((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMoveAndTargetInBattlePalace() -> u16 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut var1: i32 = 0i32;
        let mut var2: i32 = 0i32;
        let mut chosenMoveId: i32 = (-1i32);
        let mut moveInfo: *mut u8 = ((((&raw mut gBattleBufferA).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 512,
        ))
        .cast::<u8>())
        .wrapping_offset(4);
        let mut unusableMovesBits: u8 =
            CheckMoveLimitations(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8, 255u8);
        let mut percent: i32 = crate::c::rem_i32(((Random()) as i32), 100i32);
        i = (if (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(146)).read())
            as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read())
            != 0
        {
            2i32
        } else {
            0i32
        });
        var2 = i;
        var1 = (i).wrapping_add(2i32);
        {
            'l1: loop {
                if !(i < var1) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gBattlePalaceNatureToMoveGroupLikelihood).cast::<u8>())
                        .wrapping_offset(
                            ((GetNatureFromPersonality(
                                ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(72)
                                .cast::<u32>())
                                .read(),
                            )) as i32) as isize
                                * 4,
                        ))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        > percent
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        percent = (i).wrapping_sub(var2);
        if i == var1 {
            percent = 2i32;
        }
        {
            var2 = 0i32;
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if (((((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)
                        == 0i32
                    {
                        break 'l3;
                    }
                    if (percent
                        == ((GetBattlePalaceMoveGroup(
                            (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                        )) as i32))
                        && (((((((moveInfo).wrapping_add(8)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32)
                    {
                        var2 = ((((var2) as u32)
                            | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if var2 != 0i32 {
            let __p1 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(146);
            (__p1).write((((((__p1).read()) as i32) & 15i32) as u8));
            let __p2 = (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(146);
            (__p2).write((((((__p2).read()) as i32) | (var2 << 4)) as u8));
            BattleAI_SetupAIData(((var2) as u8));
            chosenMoveId = ((BattleAI_ChooseMoveOrAction()) as i32);
        }
        if chosenMoveId == (-1i32) {
            if ((unusableMovesBits) as i32) != 15i32 {
                var1 = 0i32;
                var2 = 0i32;
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < 4i32) {
                            break 'l5;
                        }
                        'l6: {
                            if (((GetBattlePalaceMoveGroup(
                                (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                            )) as i32)
                                == 0i32)
                                && (!((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()
                                    & ((unusableMovesBits) as u32))
                                    != 0))
                            {
                                var1 = (var1).wrapping_add(1i32);
                            }
                            if (((GetBattlePalaceMoveGroup(
                                (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                            )) as i32)
                                == 1i32)
                                && (!((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()
                                    & ((unusableMovesBits) as u32))
                                    != 0))
                            {
                                var1 = (var1).wrapping_add(16i32);
                            }
                            if (((GetBattlePalaceMoveGroup(
                                (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize)).read(),
                            )) as i32)
                                == 2i32)
                                && (!((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()
                                    & ((unusableMovesBits) as u32))
                                    != 0))
                            {
                                var1 = (var1).wrapping_add(256i32);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (var1 & 15i32) >= 2i32 {
                    var2 = (var2).wrapping_add(1);
                }
                if (var1 & 240i32) >= 32i32 {
                    var2 = (var2).wrapping_add(1);
                }
                if (var1 & 240i32) >= 512i32 {
                    var2 = (var2).wrapping_add(1);
                }
                if (var2 > 1i32) || (var2 == 0i32) {
                    'l7: loop {
                        'l8: {
                            i = crate::c::rem_i32(((Random()) as i32), 4i32);
                            if !((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()
                                & ((unusableMovesBits) as u32))
                                != 0)
                            {
                                chosenMoveId = i;
                            }
                        }
                        if !(chosenMoveId == (-1i32)) {
                            break 'l7;
                        }
                    }
                } else {
                    if (var1 & 15i32) >= 2i32 {
                        var2 = 0i32;
                    }
                    if (var1 & 240i32) >= 32i32 {
                        var2 = 1i32;
                    }
                    if (var1 & 240i32) >= 512i32 {
                        var2 = 2i32;
                    }
                    'l9: loop {
                        'l10: {
                            i = crate::c::rem_i32(((Random()) as i32), 4i32);
                            if (!((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()
                                & ((unusableMovesBits) as u32))
                                != 0))
                                && (var2
                                    == ((GetBattlePalaceMoveGroup(
                                        (((moveInfo).cast::<u16>()).wrapping_offset((i) as isize))
                                            .read(),
                                    )) as i32))
                            {
                                chosenMoveId = i;
                            }
                        }
                        if !(chosenMoveId == (-1i32)) {
                            break 'l9;
                        }
                    }
                }
                if crate::c::rem_i32(((Random()) as i32), 100i32) >= 50i32 {
                    crate::c::bf_write(
                        (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 16,
                        ))
                        .wrapping_add(2),
                        4,
                        1,
                        (1u32) as i32,
                    );
                    return 0u16;
                }
            } else {
                crate::c::bf_write(
                    (((&raw mut gProtectStructs).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 16,
                    ))
                    .wrapping_add(2),
                    4,
                    1,
                    (1u32) as i32,
                );
                return 0u16;
            }
        }
        if (((((moveInfo).cast::<u16>()).wrapping_offset((chosenMoveId) as isize)).read()) as i32)
            == 174i32
        {
            if ((((((moveInfo).wrapping_add(18)).cast::<u8>()).read()) as i32) != 7i32)
                && (((((((moveInfo).wrapping_add(18)).cast::<u8>()).wrapping_offset(1)).read())
                    as i32)
                    != 7i32)
            {
                var1 = 16i32;
            } else {
                var1 = 0i32;
            }
        } else {
            var1 = ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                (((((moveInfo).cast::<u16>()).wrapping_offset((chosenMoveId) as isize)).read())
                    as i32) as isize
                    * 12,
            ))
            .wrapping_add(6))
            .read()) as i32);
        }
        if (var1 & 16i32) != 0 {
            chosenMoveId =
                (chosenMoveId | (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) << 8));
        } else {
            if var1 == 0i32 {
                chosenMoveId = (chosenMoveId | ((GetBattlePalaceTarget()) as i32));
            } else {
                chosenMoveId = (chosenMoveId
                    | (((GetBattlerAtPosition(
                        (((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            & 1i32)
                            ^ 1i32) as u8),
                    )) as i32)
                        << 8));
            }
        }
        return ((chosenMoveId) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetBattlePalaceMoveGroup(r#move: u16) -> u8 {
    unsafe {
        let mut r#move = r#move;
        'l1: {
            let __sw1 = ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(6))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 2i32
                || __sw1 == 4i32
                || __sw1 == 8i32
                || __sw1 == 32i32
                || __sw1 == 1i32
                || __sw1 == 64i32
                || __sw1 == 16i32;
            if __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 4i32 || __sw1 == 8i32 || __sw1 == 32i32 {
                if ((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(1))
                .read()) as i32)
                    == 0i32
                {
                    return 2u8;
                } else {
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 64i32 {
                return 2u8;
            }
            if __sw1 == 16i32 {
                return 1u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetBattlePalaceTarget() -> u16 {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            let mut opposing1: u8 = 0u8;
            let mut opposing2: u8 = 0u8;
            if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
                opposing1 = GetBattlerAtPosition(1u8);
                opposing2 = GetBattlerAtPosition(3u8);
            } else {
                opposing1 = GetBattlerAtPosition(0u8);
                opposing2 = GetBattlerAtPosition(2u8);
            }
            if ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((opposing1) as i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32)
                == ((((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset(((opposing2) as i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read()) as i32)
            {
                return ((((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 1i32)
                    ^ 1i32)
                    .wrapping_add((((Random()) as i32) & 2i32))
                    << 8) as u16);
            }
            'l1: {
                let __sw1 = (((((&raw mut gBattlePalaceNatureToMoveTarget).cast::<u8>())
                    .wrapping_offset(
                        ((GetNatureFromPersonality(
                            ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(72)
                            .cast::<u32>())
                            .read(),
                        )) as i32) as isize,
                    ))
                .read()) as i32);
                let mut __fall = false;
                if __sw1 == 0i32 {
                    __fall = true;
                    if ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((opposing1) as i32) as isize * 88))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read()) as i32)
                        > ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((opposing2) as i32) as isize * 88))
                        .wrapping_add(40)
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        return ((((opposing1) as i32) << 8) as u16);
                    } else {
                        return ((((opposing2) as i32) << 8) as u16);
                    }
                }
                if __fall || __sw1 == 1i32 {
                    __fall = true;
                    if ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((opposing1) as i32) as isize * 88))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read()) as i32)
                        < ((((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((opposing2) as i32) as isize * 88))
                        .wrapping_add(40)
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        return ((((opposing1) as i32) << 8) as u16);
                    } else {
                        return ((((opposing2) as i32) << 8) as u16);
                    }
                }
                if __fall || __sw1 == 2i32 {
                    __fall = true;
                    return ((((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 1i32)
                        ^ 1i32)
                        .wrapping_add((((Random()) as i32) & 2i32))
                        << 8) as u16);
                }
            }
        }
        return (((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 1i32) << 8) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_WaitForBattlerBallReleaseAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8);
        if !((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0)
        {
            return;
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            6,
            1,
            false,
        ) as u8)
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
                6,
                1,
                (0u8) as i32,
            );
        } else {
            if (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
                4,
                1,
                false,
            ) as u16)
                != 0
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UnusedDoBattleSpriteAffineAnim(sprite: *mut u8, pointless: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut pointless = pointless;
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        if !((pointless) != 0) {
            StartSpriteAffineAnim(sprite, 1u8);
        } else {
            StartSpriteAffineAnim(sprite, 1u8);
        }
        AnimateSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_TrainerSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((&raw mut gIntroSlideFlags).cast::<u16>()).read()) as i32) & 1i32) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) != 0i32 {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_TrainerSlideVertical));
                } else {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerSlideVertical(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndLaunchChosenStatusAnimation(isStatus2: u8, status: u32) {
    unsafe {
        let mut isStatus2 = isStatus2;
        let mut status = status;
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            4,
            1,
            (1u8) as i32,
        );
        if !((isStatus2) != 0) {
            if status == 32u32 {
                LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 6u8);
            } else {
                if (status == 8u32) || ((status & 128u32) != 0) {
                    LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 0u8);
                } else {
                    if status == 16u32 {
                        LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 2u8);
                    } else {
                        if (status & 7u32) != 0 {
                            LaunchStatusAnimation(
                                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                4u8,
                            );
                        } else {
                            if status == 64u32 {
                                LaunchStatusAnimation(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    5u8,
                                );
                            } else {
                                crate::c::bf_write(
                                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 12,
                                    ))
                                    .wrapping_add(0),
                                    4,
                                    1,
                                    (0u8) as i32,
                                );
                            }
                        }
                    }
                }
            }
        } else {
            if (status & 983040u32) != 0 {
                LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 3u8);
            } else {
                if (status & 7u32) != 0 {
                    LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 1u8);
                } else {
                    if (status & 268435456u32) != 0 {
                        LaunchStatusAnimation(((&raw mut gActiveBattler).cast::<u8>()).read(), 7u8);
                    } else {
                        if (status & 134217728u32) != 0 {
                            LaunchStatusAnimation(
                                ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                8u8,
                            );
                        } else {
                            if (status & 57344u32) != 0 {
                                LaunchStatusAnimation(
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    9u8,
                                );
                            } else {
                                crate::c::bf_write(
                                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 12,
                                    ))
                                    .wrapping_add(0),
                                    4,
                                    1,
                                    (0u8) as i32,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryHandleLaunchBattleTableAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
    argument: u16,
) -> u8 {
    unsafe {
        let mut activeBattler = activeBattler;
        let mut atkBattler = atkBattler;
        let mut defBattler = defBattler;
        let mut tableId = tableId;
        let mut argument = argument;
        let mut taskId: u8 = 0u8;
        if (((tableId) as i32) == 0i32) && ((((argument) as i32) & 128i32) != 0) {
            (((&raw mut gBattleMonForms).cast::<u8>())
                .wrapping_offset(((activeBattler) as i32) as isize))
            .write(((((argument) as i32) & (-129i32)) as u8));
            return 1u8;
        }
        if ((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((activeBattler) as i32) as isize * 4))
            .wrapping_add(0),
            2,
            1,
            false,
        ) as u16)
            != 0)
            && (!((ShouldAnimBeDoneRegardlessOfSubstitute(tableId)) != 0))
        {
            return 1u8;
        }
        if (((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((activeBattler) as i32) as isize * 4))
            .wrapping_add(0),
            2,
            1,
            false,
        ) as u16)
            != 0)
            && (((tableId) as i32) == 2i32))
            && ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((activeBattler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16)
                != 0)
        {
            LoadBattleMonGfxAndAnimate(
                activeBattler,
                1u8,
                (((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((activeBattler) as i32) as isize))
                .read(),
            );
            ClearBehindSubstituteBit(activeBattler);
            return 1u8;
        }
        ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(atkBattler);
        ((&raw mut gBattleAnimTarget).cast::<u8>()).write(defBattler);
        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<*mut u8>())
        .read())
        .cast::<u16>())
        .write(argument);
        LaunchBattleAnimation(
            ((&raw mut gBattleAnims_General).cast::<*mut u8>()).cast::<*mut u8>(),
            ((tableId) as u16),
            0u8,
        );
        taskId = CreateTask(Some(Task_ClearBitWhenBattleTableAnimDone), 10u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((activeBattler) as i16));
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(0),
            5,
            1,
            (1u8) as i32,
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ClearBitWhenBattleTableAnimDone(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((&raw mut gAnimScriptCallback).cast::<Option<unsafe extern "C" fn()>>()).read())
            .unwrap_unchecked()();
        if !((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0) {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                5,
                1,
                (0u8) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ShouldAnimBeDoneRegardlessOfSubstitute(animId: u8) -> u8 {
    unsafe {
        let mut animId = animId;
        'l1: {
            let __sw1 = ((animId) as i32);
            let __matched = __sw1 == 2i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 17i32;
            if __sw1 == 2i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 17i32
            {
                return 1u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndLaunchSpecialAnimation(
    activeBattler: u8,
    atkBattler: u8,
    defBattler: u8,
    tableId: u8,
) {
    unsafe {
        let mut activeBattler = activeBattler;
        let mut atkBattler = atkBattler;
        let mut defBattler = defBattler;
        let mut tableId = tableId;
        let mut taskId: u8 = 0u8;
        ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(atkBattler);
        ((&raw mut gBattleAnimTarget).cast::<u8>()).write(defBattler);
        LaunchBattleAnimation(
            ((&raw mut gBattleAnims_Special).cast::<*mut u8>()).cast::<*mut u8>(),
            ((tableId) as u16),
            0u8,
        );
        taskId = CreateTask(Some(Task_ClearBitWhenSpecialAnimDone), 10u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((activeBattler) as i16));
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(0),
            6,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ClearBitWhenSpecialAnimDone(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((&raw mut gAnimScriptCallback).cast::<Option<unsafe extern "C" fn()>>()).read())
            .unwrap_unchecked()();
        if !((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0) {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                6,
                1,
                (0u8) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMoveWithoutAnimation(r#move: u16, animationTurn: u8) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut animationTurn = animationTurn;
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattleSEPlaying(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut zero: u8 = 0u8;
        if (IsSEPlaying()) != 0 {
            let __p1 = ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(8);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(8))
            .read()) as i32)
                < 30i32
            {
                return 1u8;
            }
            m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
            m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
        }
        if ((zero) as i32) == 0i32 {
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(8))
            .write(0u8);
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadOpponentMonSpriteGfx(mon: *mut u8, battler: u8) {
    unsafe {
        let mut mon = mon;
        let mut battler = battler;
        let mut monsPersonality: u32 = 0u32;
        let mut currentPersonality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        let mut species: u16 = 0u16;
        let mut position: u8 = 0u8;
        let mut paletteOffset: u16 = 0u16;
        let mut lzPaletteData: *mut u8 = core::ptr::null_mut();
        monsPersonality = GetMonData2(mon, 0i32);
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            species = ((GetMonData2(mon, 11i32)) as u16);
            currentPersonality = monsPersonality;
        } else {
            species = (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
            currentPersonality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                .cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize))
            .read();
        }
        otId = GetMonData2(mon, 1i32);
        position = GetBattlerPosition(battler);
        HandleLoadSpecialPokePic_DontHandleDeoxys(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .read(),
            ((species) as i32),
            currentPersonality,
        );
        paletteOffset = (((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as u16);
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            lzPaletteData = (GetMonFrontSpritePal(mon)).cast::<u8>();
        } else {
            lzPaletteData =
                (GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality))
                    .cast::<u8>();
        }
        LZDecompressWram(
            (lzPaletteData).cast::<u32>(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        LoadPalette(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            paletteOffset,
            32u16,
        );
        LoadPalette(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            (((128i32).wrapping_add((0i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))))
                as u16),
            32u16,
        );
        if ((species) as i32) == 385i32 {
            paletteOffset =
                (((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as u16);
            LZDecompressWram(
                (lzPaletteData).cast::<u32>(),
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                    .cast::<u8>(),
            );
            LoadPalette(
                (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                    .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gBattleMonForms).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 32,
                ))
                .cast::<u16>())
                .cast::<u8>(),
                paletteOffset,
                32u16,
            );
        }
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            BlendPalette(paletteOffset, 16u16, 6u8, 32767u16);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadPlayerMonSpriteGfx(mon: *mut u8, battler: u8) {
    unsafe {
        let mut mon = mon;
        let mut battler = battler;
        let mut monsPersonality: u32 = 0u32;
        let mut currentPersonality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        let mut species: u16 = 0u16;
        let mut position: u8 = 0u8;
        let mut paletteOffset: u16 = 0u16;
        let mut lzPaletteData: *mut u8 = core::ptr::null_mut();
        monsPersonality = GetMonData2(mon, 0i32);
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            species = ((GetMonData2(mon, 11i32)) as u16);
            currentPersonality = monsPersonality;
        } else {
            species = (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
            currentPersonality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                .cast::<u32>())
            .wrapping_offset(((battler) as i32) as isize))
            .read();
        }
        otId = GetMonData2(mon, 1i32);
        position = GetBattlerPosition(battler);
        if (((ShouldIgnoreDeoxysForm(1u8, battler)) as i32) == 1i32)
            || ((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            HandleLoadSpecialPokePic_DontHandleDeoxys(
                ((&raw mut gMonBackPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize))
                .read(),
                ((species) as i32),
                currentPersonality,
            );
        } else {
            HandleLoadSpecialPokePic(
                ((&raw mut gMonBackPicTable).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 8),
                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize))
                .read(),
                ((species) as i32),
                currentPersonality,
            );
        }
        paletteOffset = (((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as u16);
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            lzPaletteData = (GetMonFrontSpritePal(mon)).cast::<u8>();
        } else {
            lzPaletteData =
                (GetMonSpritePalFromSpeciesAndPersonality(species, otId, monsPersonality))
                    .cast::<u8>();
        }
        LZDecompressWram(
            (lzPaletteData).cast::<u32>(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        LoadPalette(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            paletteOffset,
            32u16,
        );
        LoadPalette(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            (((128i32).wrapping_add((0i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))))
                as u16),
            32u16,
        );
        if ((species) as i32) == 385i32 {
            paletteOffset =
                (((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as u16);
            LZDecompressWram(
                (lzPaletteData).cast::<u32>(),
                ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                    .cast::<u8>(),
            );
            LoadPalette(
                (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                    .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gBattleMonForms).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 32,
                ))
                .cast::<u16>())
                .cast::<u8>(),
                paletteOffset,
                32u16,
            );
        }
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            BlendPalette(paletteOffset, 16u16, 6u8, 32767u16);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
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
pub(crate) unsafe extern "C" fn BattleGfxSfxDummy1() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleGfxSfxDummy2(species: u16) {
    unsafe {
        let mut species = species;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressTrainerFrontPic(frontPicId: u16, battler: u8) {
    unsafe {
        let mut frontPicId = frontPicId;
        let mut battler = battler;
        let mut position: u8 = GetBattlerPosition(battler);
        DecompressPicFromTable_2(
            ((&raw mut gTrainerFrontPicTable).cast::<u8>())
                .wrapping_offset(((frontPicId) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .read(),
            0i32,
        );
        LoadCompressedSpritePalette(
            ((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                .wrapping_offset(((frontPicId) as i32) as isize * 8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressTrainerBackPic(backPicId: u16, battler: u8) {
    unsafe {
        let mut backPicId = backPicId;
        let mut battler = battler;
        let mut position: u8 = GetBattlerPosition(battler);
        DecompressPicFromTable_2(
            ((&raw mut gTrainerBackPicTable).cast::<u8>())
                .wrapping_offset(((backPicId) as i32) as isize * 8),
            ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<*mut u8>())
            .wrapping_offset(((position) as i32) as isize))
            .read(),
            0i32,
        );
        LoadCompressedPalette(
            ((((&raw mut gTrainerBackPicPaletteTable).cast::<u8>())
                .wrapping_offset(((backPicId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            (((256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleGfxSfxDummy3(gender: u8) {
    unsafe {
        let mut gender = gender;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeTrainerFrontPicPalette(frontPicId: u16) {
    unsafe {
        let mut frontPicId = frontPicId;
        FreeSpritePaletteByTag(
            ((((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                .wrapping_offset(((frontPicId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadAllHealthBoxesGfxAtOnce() {
    unsafe {
        let mut numberOfBattlers: u8 = 0u8;
        let mut i: u8 = 0u8;
        LoadSpritePalette(
            ((&raw const sSpritePalettes_HealthBoxHealthBar)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadSpritePalette(
            (((&raw const sSpritePalettes_HealthBoxHealthBar)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(8),
        );
        if !((IsDoubleBattle()) != 0) {
            LoadCompressedSpriteSheet(
                (&raw const sSpriteSheet_SinglesPlayerHealthbox)
                    .cast::<u8>()
                    .cast_mut(),
            );
            LoadCompressedSpriteSheet(
                (&raw const sSpriteSheet_SinglesOpponentHealthbox)
                    .cast::<u8>()
                    .cast_mut(),
            );
            numberOfBattlers = 2u8;
        } else {
            LoadCompressedSpriteSheet(
                ((&raw const sSpriteSheets_DoublesPlayerHealthbox)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            LoadCompressedSpriteSheet(
                (((&raw const sSpriteSheets_DoublesPlayerHealthbox)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(8),
            );
            LoadCompressedSpriteSheet(
                ((&raw const sSpriteSheets_DoublesOpponentHealthbox)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            LoadCompressedSpriteSheet(
                (((&raw const sSpriteSheets_DoublesOpponentHealthbox)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(8),
            );
            numberOfBattlers = 4u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numberOfBattlers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpriteSheets_HealthBar).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gBattlerPositions).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 8,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadAllHealthBoxesGfx(state: u8) -> u8 {
    unsafe {
        let mut state = state;
        let mut retVal: u8 = 0u8;
        if ((state) as i32) != 0i32 {
            if ((state) as i32) == 1i32 {
                LoadSpritePalette(
                    ((&raw const sSpritePalettes_HealthBoxHealthBar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadSpritePalette(
                    (((&raw const sSpritePalettes_HealthBoxHealthBar)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(8),
                );
            } else {
                if !((IsDoubleBattle()) != 0) {
                    if ((state) as i32) == 2i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0 {
                            LoadCompressedSpriteSheet(
                                (&raw const sSpriteSheet_SafariHealthbox)
                                    .cast::<u8>()
                                    .cast_mut(),
                            );
                        } else {
                            LoadCompressedSpriteSheet(
                                (&raw const sSpriteSheet_SinglesPlayerHealthbox)
                                    .cast::<u8>()
                                    .cast_mut(),
                            );
                        }
                    } else {
                        if ((state) as i32) == 3i32 {
                            LoadCompressedSpriteSheet(
                                (&raw const sSpriteSheet_SinglesOpponentHealthbox)
                                    .cast::<u8>()
                                    .cast_mut(),
                            );
                        } else {
                            if ((state) as i32) == 4i32 {
                                LoadCompressedSpriteSheet(
                                    (((&raw const sSpriteSheets_HealthBar)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattlerPositions).cast::<u8>()).read())
                                            as i32)
                                            as isize
                                            * 8,
                                    ),
                                );
                            } else {
                                if ((state) as i32) == 5i32 {
                                    LoadCompressedSpriteSheet(
                                        (((&raw const sSpriteSheets_HealthBar)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((&raw mut gBattlerPositions).cast::<u8>())
                                                .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                as isize
                                                * 8,
                                        ),
                                    );
                                } else {
                                    retVal = 1u8;
                                }
                            }
                        }
                    }
                } else {
                    if ((state) as i32) == 2i32 {
                        LoadCompressedSpriteSheet(
                            ((&raw const sSpriteSheets_DoublesPlayerHealthbox)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    } else {
                        if ((state) as i32) == 3i32 {
                            LoadCompressedSpriteSheet(
                                (((&raw const sSpriteSheets_DoublesPlayerHealthbox)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(8),
                            );
                        } else {
                            if ((state) as i32) == 4i32 {
                                LoadCompressedSpriteSheet(
                                    ((&raw const sSpriteSheets_DoublesOpponentHealthbox)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                            } else {
                                if ((state) as i32) == 5i32 {
                                    LoadCompressedSpriteSheet(
                                        (((&raw const sSpriteSheets_DoublesOpponentHealthbox)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(8),
                                    );
                                } else {
                                    if ((state) as i32) == 6i32 {
                                        LoadCompressedSpriteSheet(
                                            (((&raw const sSpriteSheets_HealthBar)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gBattlerPositions).cast::<u8>())
                                                    .read())
                                                    as i32)
                                                    as isize
                                                    * 8,
                                            ),
                                        );
                                    } else {
                                        if ((state) as i32) == 7i32 {
                                            LoadCompressedSpriteSheet(
                                                (((&raw const sSpriteSheets_HealthBar)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((((&raw mut gBattlerPositions).cast::<u8>())
                                                        .wrapping_offset(1))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 8,
                                                ),
                                            );
                                        } else {
                                            if ((state) as i32) == 8i32 {
                                                LoadCompressedSpriteSheet(
                                                    (((&raw const sSpriteSheets_HealthBar)
                                                        .cast::<u8>()
                                                        .cast_mut())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut gBattlerPositions)
                                                            .cast::<u8>())
                                                        .wrapping_offset(2))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 8,
                                                    ),
                                                );
                                            } else {
                                                if ((state) as i32) == 9i32 {
                                                    LoadCompressedSpriteSheet(
                                                        (((&raw const sSpriteSheets_HealthBar)
                                                            .cast::<u8>()
                                                            .cast_mut())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((&raw mut gBattlerPositions)
                                                                .cast::<u8>())
                                                            .wrapping_offset(3))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 8,
                                                        ),
                                                    );
                                                } else {
                                                    retVal = 1u8;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleBarGfx(unused: u8) {
    unsafe {
        let mut unused = unused;
        LZDecompressWram(
            ((&raw mut gBattleInterfaceGfx_BattleBar).cast::<u32>()).cast::<u32>(),
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(372)
                .cast::<*mut u8>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleInitAllSprites(state1: *mut u8, battler: *mut u8) -> u8 {
    unsafe {
        let mut state1 = state1;
        let mut battler = battler;
        let mut retVal: u8 = 0u8;
        'l1: {
            let __sw1 = (((state1).read()) as i32);
            if __sw1 == 0i32 {
                ClearSpritesBattlerHealthboxAnimData();
                (state1).write(((state1).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((BattleLoadAllHealthBoxesGfx((battler).read())) != 0) {
                    (battler).write(((battler).read()).wrapping_add(1));
                } else {
                    (battler).write(0u8);
                    (state1).write(((state1).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (state1).write(((state1).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0)
                    && ((((battler).read()) as i32) == 0i32)
                {
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler).read()) as i32) as isize))
                    .write(CreateSafariPlayerHealthboxSprites());
                } else {
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler).read()) as i32) as isize))
                    .write(CreateBattlerHealthboxSprites((battler).read()));
                }
                (battler).write(((battler).read()).wrapping_add(1));
                if (((battler).read()) as i32)
                    == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                {
                    (battler).write(0u8);
                    (state1).write(((state1).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitBattlerHealthboxCoords((battler).read());
                if (((((&raw mut gBattlerPositions).cast::<u8>())
                    .wrapping_offset((((battler).read()) as i32) as isize))
                .read()) as i32)
                    <= 1i32
                {
                    DummyBattleInterfaceFunc(
                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                            .wrapping_offset((((battler).read()) as i32) as isize))
                        .read(),
                        0u8,
                    );
                } else {
                    DummyBattleInterfaceFunc(
                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                            .wrapping_offset((((battler).read()) as i32) as isize))
                        .read(),
                        1u8,
                    );
                }
                (battler).write(((battler).read()).wrapping_add(1));
                if (((battler).read()) as i32)
                    == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                {
                    (battler).write(0u8);
                    (state1).write(((state1).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((GetBattlerSide((battler).read())) as i32) == 0i32 {
                    if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0) {
                        UpdateHealthboxAttribute(
                            (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset((((battler).read()) as i32) as isize))
                            .read(),
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset((((battler).read()) as i32) as isize))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            0u8,
                        );
                    }
                } else {
                    UpdateHealthboxAttribute(
                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                            .wrapping_offset((((battler).read()) as i32) as isize))
                        .read(),
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset((((battler).read()) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0u8,
                    );
                }
                SetHealthboxSpriteInvisible(
                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset((((battler).read()) as i32) as isize))
                    .read(),
                );
                (battler).write(((battler).read()).wrapping_add(1));
                if (((battler).read()) as i32)
                    == ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)
                {
                    (battler).write(0u8);
                    (state1).write(((state1).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadAndCreateEnemyShadowSprites();
                BufferBattlePartyCurrentOrder();
                retVal = 1u8;
                break 'l1;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSpritesHealthboxAnimData() {
    unsafe {
        crate::c::memset(
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
            0i32,
            48u32,
        );
        crate::c::memset(
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read(),
            0i32,
            16u32,
        );
    }
}
pub(crate) unsafe extern "C" fn ClearSpritesBattlerHealthboxAnimData() {
    unsafe {
        ClearSpritesHealthboxAnimData();
        crate::c::memset(
            ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read(),
            0i32,
            16u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyAllBattleSpritesInvisibilities() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 4))
                        .wrapping_add(0),
                        0,
                        1,
                        (crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            false,
                        ) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBattleSpriteInvisibility(battler: u8) {
    unsafe {
        let mut battler = battler;
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(0),
            0,
            1,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleSpeciesGfxDataChange(battlerAtk: u8, battlerDef: u8, castform: u8) {
    unsafe {
        let mut battlerAtk = battlerAtk;
        let mut battlerDef = battlerDef;
        let mut castform = castform;
        let mut paletteOffset: u16 = 0u16;
        let mut personalityValue: u32 = 0u32;
        let mut otId: u32 = 0u32;
        let mut position: u8 = 0u8;
        let mut lzPaletteData: *mut u32 = core::ptr::null_mut();
        if (castform) != 0 {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battlerAtk) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ),
                ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read()) as u8),
            );
            paletteOffset =
                (((256i32).wrapping_add(((battlerAtk) as i32).wrapping_mul(16i32))) as u16);
            LoadPalette(
                (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read()) as i32) as isize
                        * 32,
                ))
                .cast::<u16>())
                .cast::<u8>(),
                paletteOffset,
                32u16,
            );
            (((&raw mut gBattleMonForms).cast::<u8>())
                .wrapping_offset(((battlerAtk) as i32) as isize))
            .write(
                ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read()) as u8),
            );
            if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battlerAtk) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                BlendPalette(paletteOffset, 16u16, 6u8, 32767u16);
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((paletteOffset) as i32) as isize))
                                    .cast::<u8>(),
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((paletteOffset) as i32) as isize))
                                    .cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
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
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battlerAtk) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(((GetBattlerSpriteDefault_Y(battlerAtk)) as i16));
        } else {
            let mut targetSpecies: u16 = 0u16;
            if (IsContest()) != 0 {
                position = 0u8;
                targetSpecies = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .read();
                personalityValue = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .read();
                otId = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(12)
                .cast::<u32>())
                .read();
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    ((&raw mut gMonBackPicTable).cast::<u8>())
                        .wrapping_offset(((targetSpecies) as i32) as isize * 8),
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(((position) as i32) as isize))
                    .read(),
                    ((targetSpecies) as i32),
                    ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(16)
                    .cast::<u32>())
                    .read(),
                );
            } else {
                position = GetBattlerPosition(battlerAtk);
                if ((GetBattlerSide(battlerDef)) as i32) == 1i32 {
                    targetSpecies = ((GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerDef) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                } else {
                    targetSpecies = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerDef) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                }
                if ((GetBattlerSide(battlerAtk)) as i32) == 0i32 {
                    personalityValue = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerAtk) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0i32,
                    );
                    otId = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerAtk) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        1i32,
                    );
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        ((&raw mut gMonBackPicTable).cast::<u8>())
                            .wrapping_offset(((targetSpecies) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((position) as i32) as isize))
                        .read(),
                        ((targetSpecies) as i32),
                        ((((&raw mut gTransformedPersonalities).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(((battlerAtk) as i32) as isize))
                        .read(),
                    );
                } else {
                    personalityValue = GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerAtk) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0i32,
                    );
                    otId = GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerAtk) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        1i32,
                    );
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((targetSpecies) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(((position) as i32) as isize))
                        .read(),
                        ((targetSpecies) as i32),
                        ((((&raw mut gTransformedPersonalities).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(((battlerAtk) as i32) as isize))
                        .read(),
                    );
                }
            }
            {
                let mut _src: *mut u8 =
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(((position) as i32) as isize))
                    .read();
                let mut _dest: *mut u8 = (((100728832i32).wrapping_add(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset(((battlerAtk) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_mul(32i32),
                )) as usize as *mut u8);
                let mut _size: u32 = ((crate::c::div_i32(4096i32, 2i32)) as u32);
                'l5: loop {
                    'l6: {
                        'l7: loop {
                            'l8: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((_dest) as usize as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (2214592512u32
                                            | crate::c::div_u32(
                                                _size,
                                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                                            )),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l5;
                    }
                }
            }
            paletteOffset =
                (((256i32).wrapping_add(((battlerAtk) as i32).wrapping_mul(16i32))) as u16);
            lzPaletteData =
                GetMonSpritePalFromSpeciesAndPersonality(targetSpecies, otId, personalityValue);
            LZDecompressWram(lzPaletteData, (&raw mut gDecompressionBuffer).cast::<u8>());
            LoadPalette(
                (&raw mut gDecompressionBuffer).cast::<u8>(),
                paletteOffset,
                32u16,
            );
            if ((targetSpecies) as i32) == 385i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battlerAtk) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(8)
                .cast::<*mut *mut u8>())
                .write(
                    ((((&raw mut gMonFrontAnimsPtrTable).cast::<*mut *mut u8>())
                        .cast::<*mut *mut u8>())
                    .wrapping_offset(((targetSpecies) as i32) as isize))
                    .read(),
                );
                LZDecompressWram(
                    lzPaletteData,
                    ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                        .cast::<u8>(),
                );
                LoadPalette(
                    (((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(256))
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMonForms).cast::<u8>())
                            .wrapping_offset(((battlerDef) as i32) as isize))
                        .read()) as i32) as isize
                            * 32,
                    ))
                    .cast::<u16>())
                    .cast::<u8>(),
                    paletteOffset,
                    32u16,
                );
            }
            BlendPalette(paletteOffset, 16u16, 6u8, 32767u16);
            'l9: loop {
                'l10: {
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l9;
                }
            }
            if !((IsContest()) != 0) {
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battlerAtk) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .write(targetSpecies);
                (((&raw mut gBattleMonForms).cast::<u8>())
                    .wrapping_offset(((battlerAtk) as i32) as isize))
                .write(
                    (((&raw mut gBattleMonForms).cast::<u8>())
                        .wrapping_offset(((battlerDef) as i32) as isize))
                    .read(),
                );
            }
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battlerAtk) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(((GetBattlerSpriteDefault_Y(battlerAtk)) as i16));
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battlerAtk) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ),
                (((&raw mut gBattleMonForms).cast::<u8>())
                    .wrapping_offset(((battlerAtk) as i32) as isize))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleLoadSubstituteOrMonSpriteGfx(battler: u8, loadMonSprite: u8) {
    unsafe {
        let mut battler = battler;
        let mut loadMonSprite = loadMonSprite;
        let mut i: i32 = 0i32;
        let mut position: i32 = 0i32;
        let mut palOffset: i32 = 0i32;
        if !((loadMonSprite) != 0) {
            if (IsContest()) != 0 {
                position = 0i32;
            } else {
                position = ((GetBattlerPosition(battler)) as i32);
            }
            if (IsContest()) != 0 {
                LZDecompressVram(
                    ((&raw mut gSubstituteDollBackGfx).cast::<u32>()).cast::<u32>(),
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset((position) as isize))
                    .read(),
                );
            } else {
                if ((GetBattlerSide(battler)) as i32) != 0i32 {
                    LZDecompressVram(
                        ((&raw mut gSubstituteDollFrontGfx).cast::<u32>()).cast::<u32>(),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset((position) as isize))
                        .read(),
                    );
                } else {
                    LZDecompressVram(
                        ((&raw mut gSubstituteDollBackGfx).cast::<u32>()).cast::<u32>(),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset((position) as isize))
                        .read(),
                    );
                }
            }
            {
                i = 1i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            let mut _src: *mut u8 =
                                ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<*mut u8>())
                                .wrapping_offset((position) as isize))
                                .read();
                            let mut _dest: *mut u8 =
                                (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<*mut u8>())
                                .wrapping_offset((position) as isize))
                                .read())
                                .wrapping_offset(
                                    ((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(i)) as isize,
                                );
                            let mut _size: u32 = ((crate::c::div_i32(4096i32, 2i32)) as u32);
                            'l3: loop {
                                if !((1i32) != 0) {
                                    break 'l3;
                                }
                                if _size <= 4096u32 {
                                    'l4: loop {
                                        'l5: {
                                            'l6: loop {
                                                'l7: {
                                                    {
                                                        let mut dmaRegs: *mut u32 =
                                                            ((67109076i32) as usize as *mut u32);
                                                        crate::c::volatile_write(
                                                            dmaRegs,
                                                            ((_src) as usize as u32),
                                                        );
                                                        crate::c::volatile_write(
                                                            (dmaRegs).wrapping_offset(1),
                                                            ((_dest) as usize as u32),
                                                        );
                                                        crate::c::volatile_write(
                                                            (dmaRegs).wrapping_offset(2),
                                                            (2214592512u32
                                                                | crate::c::div_u32(
                                                                    _size,
                                                                    ((crate::c::div_i32(
                                                                        32i32, 8i32,
                                                                    ))
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
                                        if !((0i32) != 0) {
                                            break 'l4;
                                        }
                                    }
                                    break 'l3;
                                }
                                'l8: loop {
                                    'l9: {
                                        'l10: loop {
                                            'l11: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((_src) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (((-2080374784i32)
                                                            | crate::c::div_i32(
                                                                4096i32,
                                                                crate::c::div_i32(32i32, 8i32),
                                                            ))
                                                            as u32),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l10;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                                _src = (_src).wrapping_offset(4096);
                                _dest = (_dest).wrapping_offset(4096);
                                _size = (_size).wrapping_sub(4096u32);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            palOffset = (256i32).wrapping_add(((battler) as i32).wrapping_mul(16i32));
            LoadCompressedPalette(
                ((&raw mut gSubstituteDollPal).cast::<u32>()).cast::<u32>(),
                ((palOffset) as u16),
                32u16,
            );
        } else {
            if !((IsContest()) != 0) {
                if ((GetBattlerSide(battler)) as i32) != 0i32 {
                    BattleLoadOpponentMonSpriteGfx(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        battler,
                    );
                } else {
                    BattleLoadPlayerMonSpriteGfx(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        battler,
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattleMonGfxAndAnimate(battler: u8, loadMonSprite: u8, spriteId: u8) {
    unsafe {
        let mut battler = battler;
        let mut loadMonSprite = loadMonSprite;
        let mut spriteId = spriteId;
        BattleLoadSubstituteOrMonSpriteGfx(battler, loadMonSprite);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            (((&raw mut gBattleMonForms).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read(),
        );
        if !((loadMonSprite) != 0) {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(((GetSubstituteSpriteDefault_Y(battler)) as i16));
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(((GetBattlerSpriteDefault_Y(battler)) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetBehindSubstituteSpriteBit(battler: u8, r#move: u16) {
    unsafe {
        let mut battler = battler;
        let mut r#move = r#move;
        if ((r#move) as i32) == 164i32 {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(0),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBehindSubstituteBit(battler: u8) {
    unsafe {
        let mut battler = battler;
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(0),
            2,
            1,
            (0u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleLowHpMusicChange(mon: *mut u8, battler: u8) {
    unsafe {
        let mut mon = mon;
        let mut battler = battler;
        let mut hp: u16 = ((GetMonData2(mon, 57i32)) as u16);
        let mut maxHP: u16 = ((GetMonData2(mon, 58i32)) as u16);
        if ((GetHPBarLevel(((hp) as i16), ((maxHP) as i16))) as i32) == 1i32 {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(0),
                1,
                1,
                false,
            ) as u16)
                != 0)
            {
                if !((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((((battler) as i32) ^ 2i32) as isize * 4))
                    .wrapping_add(0),
                    1,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlaySE(90u16);
                }
                crate::c::bf_write(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(0),
                    1,
                    1,
                    (1u16) as i32,
                );
            }
        } else {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(0),
                1,
                1,
                (0u16) as i32,
            );
            if !((IsDoubleBattle()) != 0) {
                m4aSongNumStop(90u16);
                return;
            }
            if ((IsDoubleBattle()) != 0)
                && (!((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((((battler) as i32) ^ 2i32) as isize * 4))
                    .wrapping_add(0),
                    1,
                    1,
                    false,
                ) as u16)
                    != 0))
            {
                m4aSongNumStop(90u16);
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleStopLowHpSound() {
    unsafe {
        let mut playerBattler: u8 = GetBattlerAtPosition(0u8);
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((playerBattler) as i32) as isize * 4))
            .wrapping_add(0),
            1,
            1,
            (0u16) as i32,
        );
        if (IsDoubleBattle()) != 0 {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset((((playerBattler) as i32) ^ 2i32) as isize * 4))
                .wrapping_add(0),
                1,
                1,
                (0u16) as i32,
            );
        }
        m4aSongNumStop(90u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonHPBarLevel(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut hp: u16 = ((GetMonData2(mon, 57i32)) as u16);
        let mut maxHP: u16 = ((GetMonData2(mon, 58i32)) as u16);
        return GetHPBarLevel(((hp) as i16), ((maxHP) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleBattleLowHpMusicChange() {
    unsafe {
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            let mut playerBattler1: u8 = GetBattlerAtPosition(0u8);
            let mut playerBattler2: u8 = GetBattlerAtPosition(2u8);
            let mut battler1PartyId: u8 = GetPartyIdFromBattlePartyId(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((playerBattler1) as i32) as isize))
                .read()) as u8),
            );
            let mut battler2PartyId: u8 = GetPartyIdFromBattlePartyId(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((playerBattler2) as i32) as isize))
                .read()) as u8),
            );
            if GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((battler1PartyId) as i32) as isize * 100),
                57i32,
            ) != 0u32
            {
                HandleLowHpMusicChange(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((battler1PartyId) as i32) as isize * 100),
                    playerBattler1,
                );
            }
            if ((IsDoubleBattle()) != 0)
                && (GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((battler2PartyId) as i32) as isize * 100),
                    57i32,
                ) != 0u32)
            {
                HandleLowHpMusicChange(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((battler2PartyId) as i32) as isize * 100),
                    playerBattler2,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerSpriteAffineMode(affineMode: u8) {
    unsafe {
        let mut affineMode = affineMode;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (IsBattlerSpritePresent(((i) as u8))) != 0 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(1),
                            0,
                            2,
                            ((affineMode) as u32) as i32,
                        );
                        if ((affineMode) as i32) == 0i32 {
                            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 12))
                            .wrapping_add(6))
                            .write(
                                ((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(3),
                                    1,
                                    5,
                                    false,
                                ) as u32) as u8),
                            );
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(3),
                                1,
                                5,
                                (0u32) as i32,
                            );
                        } else {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(3),
                                1,
                                5,
                                (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 12))
                                .wrapping_add(6))
                                .read()) as u32) as i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadAndCreateEnemyShadowSprites() {
    unsafe {
        let mut battler: u8 = 0u8;
        LoadCompressedSpriteSheet((&raw mut gSpriteSheet_EnemyShadow).cast::<u8>());
        battler = GetBattlerAtPosition(1u8);
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 12))
        .wrapping_add(7))
        .write(CreateSprite(
            (&raw mut gSpriteTemplate_EnemyShadow).cast::<u8>(),
            ((GetBattlerSpriteCoord(battler, 0u8)) as i16),
            ((((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(29i32)) as i16),
            200u8,
        ));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(7))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(((battler) as i16));
        if (IsDoubleBattle()) != 0 {
            battler = GetBattlerAtPosition(3u8);
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(7))
            .write(CreateSprite(
                (&raw mut gSpriteTemplate_EnemyShadow).cast::<u8>(),
                ((GetBattlerSpriteCoord(battler, 0u8)) as i16),
                ((((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(29i32)) as i16),
                200u8,
            ));
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(7))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(((battler) as i16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_EnemyShadow(shadowSprite: *mut u8) {
    unsafe {
        let mut shadowSprite = shadowSprite;
        let mut invisible: u8 = 0u8;
        let mut battler: u8 = (((((shadowSprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        let mut battlerSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        );
        if (!((crate::c::bf_read((battlerSprite).wrapping_add(62), 0, 1, false) as u16) != 0))
            || (!((IsBattlerSpritePresent(battler)) != 0))
        {
            ((shadowSprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_SetInvisible));
            return;
        }
        if ((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0)
            || ((crate::c::bf_read((battlerSprite).wrapping_add(62), 2, 1, false) as u16) != 0)
        {
            invisible = 1u8;
        } else {
            if ((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                != 0i32)
                && ((((((&raw mut gEnemyMonElevation).cast::<u8>()).wrapping_offset(
                    (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    == 0i32)
            {
                invisible = 1u8;
            }
        }
        if (crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(0),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            invisible = 1u8;
        }
        ((shadowSprite).wrapping_add(32).cast::<i16>())
            .write(((battlerSprite).wrapping_add(32).cast::<i16>()).read());
        ((shadowSprite).wrapping_add(36).cast::<i16>())
            .write(((battlerSprite).wrapping_add(36).cast::<i16>()).read());
        crate::c::bf_write(
            (shadowSprite).wrapping_add(62),
            2,
            1,
            ((invisible) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpriteCB_SetInvisible(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerShadowSpriteCallback(battler: u8, species: u16) {
    unsafe {
        let mut battler = battler;
        let mut species = species;
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            return;
        }
        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            species = (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read();
        }
        if (((((&raw mut gEnemyMonElevation).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read()) as i32)
            != 0i32
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(7))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_EnemyShadow));
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(7))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_SetInvisible));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideBattlerShadowSprite(battler: u8) {
    unsafe {
        let mut battler = battler;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 12))
            .wrapping_add(7))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SetInvisible));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillAroundBattleWindows() {
    unsafe {
        let mut vramPtr: *mut u16 = ((100663872i32) as usize as *mut u16);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 16i32) {
                                break 'l3;
                            }
                            'l4: {
                                if !(((((vramPtr).read()) as i32) & 61440i32) != 0) {
                                    (vramPtr)
                                        .write((((((vramPtr).read()) as i32) | 61440i32) as u16));
                                }
                                if !(((((vramPtr).read()) as i32) & 3840i32) != 0) {
                                    (vramPtr)
                                        .write((((((vramPtr).read()) as i32) | 3840i32) as u16));
                                }
                                if !(((((vramPtr).read()) as i32) & 240i32) != 0) {
                                    (vramPtr)
                                        .write((((((vramPtr).read()) as i32) | 240i32) as u16));
                                }
                                if !(((((vramPtr).read()) as i32) & 15i32) != 0) {
                                    (vramPtr).write((((((vramPtr).read()) as i32) | 15i32) as u16));
                                }
                                vramPtr = (vramPtr).wrapping_offset(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTemporarySpeciesSpriteData(battler: u8, dontClearSubstitute: u8) {
    unsafe {
        let mut battler = battler;
        let mut dontClearSubstitute = dontClearSubstitute;
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
            .read())
        .wrapping_offset(((battler) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .write(0u16);
        (((&raw mut gBattleMonForms).cast::<u8>()).wrapping_offset(((battler) as i32) as isize))
            .write(0u8);
        if !((dontClearSubstitute) != 0) {
            ClearBehindSubstituteBit(battler);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocateMonSpritesGfx() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).write(core::ptr::null_mut());
        ((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).write(AllocZeroed(384u32));
        ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).cast::<*mut u8>()).write(
            AllocZeroed(
                ((((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(4i32)).wrapping_mul(4i32))
                    as u32),
            ),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((i) as i32).wrapping_mul(crate::c::div_i32(4096i32, 2i32)))
                                .wrapping_mul(4i32)) as isize
                                * 1,
                        ),
                    );
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24)
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        ((&raw mut gBattlerSpriteTemplates).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24)
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(116))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 8))
                                .cast::<*mut u8>())
                                .write(
                                    (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4))
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_offset(
                                        (((j) as i32)
                                            .wrapping_mul(crate::c::div_i32(4096i32, 2i32)))
                                            as isize
                                            * 1,
                                    ),
                                );
                                (((((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(116))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 8))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .write(((crate::c::div_i32(4096i32, 2i32)) as u16));
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(116))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(372)
            .cast::<*mut u8>())
        .write(AllocZeroed(4096u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonSpritesGfx() {
    unsafe {
        if ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) == 0usize {
            return;
        }
        if ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(380)
            .cast::<*mut u16>())
        .read()) as usize)
            != 0usize
        {
            Free(
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(380)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        if ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(376)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            Free(
                ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(376)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(376)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(372)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(372)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        {
            Free(
                ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                    .read(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
        (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(core::ptr::null_mut());
        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<*mut u8>())
        .wrapping_offset(2))
        .write(core::ptr::null_mut());
        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<*mut u8>())
        .wrapping_offset(3))
        .write(core::ptr::null_mut());
        {
            Free(((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read());
            ((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldPlayNormalMonCry(mon: *mut u8) -> u32 {
    unsafe {
        let mut mon = mon;
        let mut hp: i16 = 0i16;
        let mut maxHP: i16 = 0i16;
        let mut barLevel: i32 = 0i32;
        if (GetMonData2(mon, 55i32) & 4095u32) != 0 {
            return 0u32;
        }
        hp = ((GetMonData2(mon, 57i32)) as i16);
        maxHP = ((GetMonData2(mon, 58i32)) as i16);
        barLevel = ((GetHPBarLevel(hp, maxHP)) as i32);
        if barLevel <= 2i32 {
            return 0u32;
        }
        return 1u32;
    }
}
