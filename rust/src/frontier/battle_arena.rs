//! Translated from `src/battle_arena.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMindRatings sOam_JudgmentIcon sAnim_JudgmentIcon_X sAnim_JudgmentIcon_Triangle sAnim_JudgmentIcon_Circle sAnim_JudgmentIcon_Line sAnims_JudgmentIcon sSpriteTemplate_JudgmentIcon sBattleArenaJudgmentSymbolsSpriteSheet sArenaFunctions sShortStreakPrizeItems sLongStreakPrizeItems
#[allow(unused_imports)]
use crate::data::battle_arena::*;

unsafe extern "C" {
    static mut gBattleArenaJudgmentSymbolsPalette: u8;
    static mut gBattleCommunication: u8;
    static mut gBattleMons: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTextBuff1: u8;
    static mut gBattleTextBuff2: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBitTable: u8;
    static mut gCurrentMove: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gHitMarker: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMoveResultFlags: u8;
    static mut gPaletteFade: u8;
    static mut gProtectStructs: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gText_Body: u8;
    static mut gText_Judgment: u8;
    static mut gText_Mind: u8;
    static mut gText_OpponentMon1Name: u8;
    static mut gText_PlayerMon1Name: u8;
    static mut gText_Skill: u8;
    static mut gText_Vs: u8;
    static mut gTrainerBattleOpponent_A: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn BattlePutTextOnWindow(a0: *mut u8, a1: u8);
    fn BattleStringExpandPlaceholdersToDisplayedString(a0: *mut u8) -> u32;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn HandleBattleWindow(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn SaveGameFrontier();
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattleArenaFunction() {
    unsafe {
        (((((&raw const sArenaFunctions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleArena_ShowJudgmentWindow(state: *mut u8) -> u8 {
    unsafe {
        let mut state = state;
        let mut i: i32 = 0i32;
        let mut result: u8 = 0u8;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(2147483420u32, 4i8, 0u8, 8u8, 0u16);
                SetGpuReg(72u8, 16190u16);
                LoadCompressedSpriteSheet(
                    ((&raw const sBattleArenaJudgmentSymbolsSpriteSheet)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleArenaJudgmentSymbolsPalette).cast::<u32>()).cast::<u32>(),
                    496u16,
                    32u16,
                );
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(255u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(112u16);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    HandleBattleWindow(5u8, 0u8, 24u8, 13u8, 0u8);
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ((&raw mut gBattleTextBuff1).cast::<u8>()).write(161u8);
                    (((&raw mut gBattleTextBuff1).cast::<u8>()).wrapping_offset(1)).write(255u8);
                    ((&raw mut gBattleTextBuff2).cast::<u8>()).write(161u8);
                    (((&raw mut gBattleTextBuff2).cast::<u8>()).wrapping_offset(1)).write(255u8);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (&raw mut gText_PlayerMon1Name).cast::<u8>(),
                    );
                    BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 15u8);
                    BattlePutTextOnWindow((&raw mut gText_Vs).cast::<u8>(), 16u8);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (&raw mut gText_OpponentMon1Name).cast::<u8>(),
                    );
                    BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 17u8);
                    BattlePutTextOnWindow((&raw mut gText_Mind).cast::<u8>(), 18u8);
                    BattlePutTextOnWindow((&raw mut gText_Skill).cast::<u8>(), 19u8);
                    BattlePutTextOnWindow((&raw mut gText_Body).cast::<u8>(), 20u8);
                    BattleStringExpandPlaceholdersToDisplayedString(
                        (&raw mut gText_Judgment).cast::<u8>(),
                    );
                    BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 21u8);
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    SetGpuReg(72u8, 16191u16);
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 8i32) {
                                break 'l2;
                            }
                            'l3: {
                                let mut spriteId: u8 = CreateSprite(
                                    (&raw const sSpriteTemplate_JudgmentIcon)
                                        .cast::<u8>()
                                        .cast_mut(),
                                    (((64i32).wrapping_add((i).wrapping_mul(16i32))) as i16),
                                    84i16,
                                    0u8,
                                );
                                StartSpriteAnim(
                                    ((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                                    3u8,
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    result = 1u8;
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                PlaySE(265u16);
                ShowJudgmentSprite(80u8, 40u8, 0u8, 0u8);
                ShowJudgmentSprite(160u8, 40u8, 0u8, 1u8);
                BattleStringExpandPlaceholdersToDisplayedString(
                    (&raw mut gText_Judgment).cast::<u8>(),
                );
                BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 21u8);
                (state).write(((state).read()).wrapping_add(1));
                result = 1u8;
                break 'l1;
            }
            if __sw1 == 5i32 {
                PlaySE(265u16);
                ShowJudgmentSprite(80u8, 56u8, 1u8, 0u8);
                ShowJudgmentSprite(160u8, 56u8, 1u8, 1u8);
                BattleStringExpandPlaceholdersToDisplayedString(
                    (&raw mut gText_Judgment).cast::<u8>(),
                );
                BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 21u8);
                (state).write(((state).read()).wrapping_add(1));
                result = 1u8;
                break 'l1;
            }
            if __sw1 == 6i32 {
                PlaySE(265u16);
                ShowJudgmentSprite(80u8, 72u8, 2u8, 0u8);
                ShowJudgmentSprite(160u8, 72u8, 2u8, 1u8);
                BattleStringExpandPlaceholdersToDisplayedString(
                    (&raw mut gText_Judgment).cast::<u8>(),
                );
                BattlePutTextOnWindow((&raw mut gDisplayedStringBattle).cast::<u8>(), 21u8);
                (state).write(((state).read()).wrapping_add(1));
                result = 1u8;
                break 'l1;
            }
            if __sw1 == 7i32 {
                PlaySE(266u16);
                if ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32)
                    > ((((&raw mut gBattleTextBuff2).cast::<u8>()).read()) as i32)
                {
                    result = 2u8;
                    (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).write(0u8);
                } else {
                    if ((((&raw mut gBattleTextBuff1).cast::<u8>()).read()) as i32)
                        < ((((&raw mut gBattleTextBuff2).cast::<u8>()).read()) as i32)
                    {
                        result = 3u8;
                        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(23)).write(1u8);
                    } else {
                        result = 4u8;
                    }
                }
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                SetGpuReg(72u8, 16190u16);
                HandleBattleWindow(5u8, 0u8, 24u8, 13u8, 1u8);
                CopyBgTilemapBufferToVram(0u8);
                m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
                BeginNormalPaletteFade(2147483420u32, 4i8, 8u8, 0u8, 0u16);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetGpuReg(72u8, 16191u16);
                    FreeSpriteTilesByTag(1000u16);
                    result = 1u8;
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn ShowJudgmentSprite(x: u8, y: u8, category: u8, battler: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut category = category;
        let mut battler = battler;
        let mut animNum: i32 = 0i32;
        let mut pointsPlayer: i32 = 0i32;
        let mut pointsOpponent: i32 = 0i32;
        let mut mindPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(664)).cast::<i8>();
        let mut skillPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(666)).cast::<i8>();
        let mut hpAtStart: *mut u16 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(668)).cast::<u16>();
        'l1: {
            let __sw1 = ((category) as i32);
            if __sw1 == 0i32 {
                pointsPlayer =
                    ((((mindPoints).wrapping_offset(((battler) as i32) as isize)).read()) as i32);
                pointsOpponent = ((((mindPoints)
                    .wrapping_offset((((battler) as i32) ^ 1i32) as isize))
                .read()) as i32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                pointsPlayer =
                    ((((skillPoints).wrapping_offset(((battler) as i32) as isize)).read()) as i32);
                pointsOpponent = ((((skillPoints)
                    .wrapping_offset((((battler) as i32) ^ 1i32) as isize))
                .read()) as i32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                pointsPlayer = crate::c::div_i32(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize * 88))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(100i32),
                    ((((hpAtStart).wrapping_offset(((battler) as i32) as isize)).read()) as i32),
                );
                pointsOpponent = crate::c::div_i32(
                    ((((((&raw mut gBattleMons).cast::<u8>())
                        .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
                    .wrapping_add(40)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(100i32),
                    ((((hpAtStart).wrapping_offset((((battler) as i32) ^ 1i32) as isize)).read())
                        as i32),
                );
                break 'l1;
            }
        }
        if pointsPlayer > pointsOpponent {
            animNum = 2i32;
            if ((battler) as i32) != 0i32 {
                let __p2 = (&raw mut gBattleTextBuff2).cast::<u8>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as u8));
            } else {
                let __p3 = (&raw mut gBattleTextBuff1).cast::<u8>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as u8));
            }
        } else {
            if pointsPlayer == pointsOpponent {
                animNum = 1i32;
                if ((battler) as i32) != 0i32 {
                    let __p4 = (&raw mut gBattleTextBuff2).cast::<u8>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(1i32)) as u8));
                } else {
                    let __p5 = (&raw mut gBattleTextBuff1).cast::<u8>();
                    (__p5).write((((((__p5).read()) as i32).wrapping_add(1i32)) as u8));
                }
            } else {
                animNum = 0i32;
            }
        }
        pointsPlayer = ((CreateSprite(
            (&raw const sSpriteTemplate_JudgmentIcon)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            0u8,
        )) as i32);
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset((pointsPlayer) as isize * 68),
            ((animNum) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_JudgmentIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((&raw mut gBattleCommunication).cast::<u8>()).read()) as i32) > 8i32 {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleArena_InitPoints() {
    unsafe {
        let mut mindPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(664)).cast::<i8>();
        let mut skillPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(666)).cast::<i8>();
        let mut hpAtStart: *mut u16 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(668)).cast::<u16>();
        (mindPoints).write(0i8);
        ((mindPoints).wrapping_offset(1)).write(0i8);
        (skillPoints).write(0i8);
        ((skillPoints).wrapping_offset(1)).write(0i8);
        (hpAtStart).write(
            (((&raw mut gBattleMons).cast::<u8>())
                .wrapping_add(40)
                .cast::<u16>())
            .read(),
        );
        ((hpAtStart).wrapping_offset(1)).write(
            ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(88))
                .wrapping_add(40)
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleArena_AddMindPoints(battler: u8) {
    unsafe {
        let mut battler = battler;
        let __p1 = (((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(664))
            .cast::<i8>())
        .wrapping_offset(((battler) as i32) as isize);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw const sMindRatings)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i8>())
                .cast::<i8>())
                .wrapping_offset(
                    ((((&raw mut gCurrentMove).cast::<u16>()).read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as i8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleArena_AddSkillPoints(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut skillPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(666)).cast::<i8>();
        if (((&raw mut gHitMarker).cast::<u32>()).read() & 33554432u32) != 0 {
            let mut failedMoveBits: *mut u8 =
                (((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(674);
            if ((((failedMoveBits).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read())
                != 0
            {
                (failedMoveBits).write(
                    (((((failedMoveBits).read()) as u32)
                        & !(((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read())) as u8),
                );
                let __p1 = (skillPoints).wrapping_offset(((battler) as i32) as isize);
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i8));
            } else {
                if (((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 41i32) != 0 {
                    if (!((((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 1i32)
                        != 0))
                        || ((((((&raw mut gBattleCommunication).cast::<u8>()).wrapping_offset(6))
                            .read()) as i32)
                            != 1i32)
                    {
                        let __p2 = (skillPoints).wrapping_offset(((battler) as i32) as isize);
                        (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i8));
                    }
                } else {
                    if ((((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 2i32) != 0)
                        && ((((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 4i32)
                            != 0)
                    {
                        let __p3 = (skillPoints).wrapping_offset(((battler) as i32) as isize);
                        (__p3).write((((((__p3).read()) as i32).wrapping_add(1i32)) as i8));
                    } else {
                        if (((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 2i32)
                            != 0
                        {
                            let __p4 = (skillPoints).wrapping_offset(((battler) as i32) as isize);
                            (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i8));
                        } else {
                            if (((((&raw mut gMoveResultFlags).cast::<u8>()).read()) as i32) & 4i32)
                                != 0
                            {
                                let __p5 =
                                    (skillPoints).wrapping_offset(((battler) as i32) as isize);
                                (__p5).write((((((__p5).read()) as i32).wrapping_sub(1i32)) as i8));
                            } else {
                                if !((crate::c::bf_read(
                                    (((&raw mut gProtectStructs).cast::<u8>())
                                        .wrapping_offset(((battler) as i32) as isize * 16))
                                    .wrapping_add(0),
                                    0,
                                    1,
                                    false,
                                ) as u32)
                                    != 0)
                                {
                                    let __p6 =
                                        (skillPoints).wrapping_offset(((battler) as i32) as isize);
                                    (__p6).write(
                                        (((((__p6).read()) as i32).wrapping_add(1i32)) as i8),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleArena_DeductSkillPoints(battler: u8, stringId: u16) {
    unsafe {
        let mut battler = battler;
        let mut stringId = stringId;
        let mut skillPoints: *mut i8 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(666)).cast::<i8>();
        'l1: {
            let __sw1 = ((stringId) as i32);
            if __sw1 == 327i32
                || __sw1 == 346i32
                || __sw1 == 347i32
                || __sw1 == 350i32
                || __sw1 == 309i32
                || __sw1 == 311i32
                || __sw1 == 305i32
                || __sw1 == 306i32
                || __sw1 == 195i32
                || __sw1 == 196i32
                || __sw1 == 197i32
                || __sw1 == 199i32
                || __sw1 == 200i32
                || __sw1 == 201i32
                || __sw1 == 202i32
                || __sw1 == 203i32
                || __sw1 == 204i32
                || __sw1 == 206i32
                || __sw1 == 119i32
            {
                let __p2 = (skillPoints).wrapping_offset(((battler) as i32) as isize);
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(3i32)) as i8));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHPAtStart(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut hpAtStart: *mut u16 =
            ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(668)).cast::<u16>();
        ((hpAtStart).wrapping_offset(((battler) as i32) as isize)).write(
            ((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
            .read(),
        );
        if ((((hpAtStart).wrapping_offset((((battler) as i32) ^ 1i32) as isize)).read()) as i32)
            > ((((((&raw mut gBattleMons).cast::<u8>())
                .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
            .wrapping_add(40)
            .cast::<u16>())
            .read()) as i32)
        {
            ((hpAtStart).wrapping_offset((((battler) as i32) ^ 1i32) as isize)).write(
                ((((&raw mut gBattleMons).cast::<u8>())
                    .wrapping_offset((((battler) as i32) ^ 1i32) as isize * 88))
                .wrapping_add(40)
                .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InitArenaChallenge() {
    unsafe {
        let mut isCurrent: u32 = 0u32;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(0u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .write(0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (0u8) as i32,
        );
        if lvlMode != 0u32 {
            isCurrent = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
            .read()
                & 128u32);
        } else {
            isCurrent = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
            .read()
                & 64u32);
        }
        if !((isCurrent) != 0) {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1934))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
        }
        SetDynamicWarp(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
        );
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetArenaData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1932)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset(((lvlMode) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if lvlMode != 0u32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 128u32) as u16),
                    );
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>())
                        .read()
                            & 64u32) as u16),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetArenaData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1932)
                    .cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1934))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 2i32 {
                if lvlMode != 0u32 {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p2).write(((__p2).read() | 128u32));
                    } else {
                        let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p3).write(((__p3).read() & 4294967167u32));
                    }
                } else {
                    if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                        let __p4 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p4).write(((__p4).read() | 64u32));
                    } else {
                        let __p5 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1680)
                        .cast::<u32>();
                        (__p5).write(((__p5).read() & 4294967231u32));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveArenaChallenge() {
    unsafe {
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
        VarSet(16384u16, 0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (1u8) as i32,
        );
        SaveGameFrontier();
    }
}
pub(crate) unsafe extern "C" fn SetArenaPrize() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1934))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            > 41i32
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1932)
                .cast::<u16>())
            .write(
                ((((&raw const sLongStreakPrizeItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(18u32, 2u32)))
                        as i32) as isize,
                ))
                .read(),
            );
        } else {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1932)
                .cast::<u16>())
            .write(
                ((((&raw const sShortStreakPrizeItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(12u32, 2u32)))
                        as i32) as isize,
                ))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GiveArenaPrize() {
    unsafe {
        if ((AddBagItem(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1932)
                .cast::<u16>())
            .read(),
            1u16,
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1932)
                    .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1932)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn BufferArenaOpponentName() {
    unsafe {
        GetFrontierTrainerName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawArenaRefereeTextBox() {
    unsafe {
        let mut width: u8 = 27u8;
        let mut palNum: u8 = 7u8;
        FillBgTilemapBufferRect(0u8, 0u16, 254u8, 14u8, 1u8, 6u8, palNum);
        FillBgTilemapBufferRect(0u8, 0u16, 32u8, 14u8, 1u8, 6u8, palNum);
        FillBgTilemapBufferRect(0u8, 49u16, 0u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 51u16, 1u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 52u16, 2u8, 14u8, width, 1u8, palNum);
        width = (width).wrapping_add(1);
        FillBgTilemapBufferRect(0u8, 53u16, 28u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 54u16, 29u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 55u16, 0u8, 15u8, 1u8, 5u8, palNum);
        FillBgTilemapBufferRect(0u8, 57u16, 1u8, 15u8, width, 5u8, palNum);
        FillBgTilemapBufferRect(0u8, 58u16, 29u8, 15u8, 1u8, 5u8, palNum);
        FillBgTilemapBufferRect(0u8, 2097u16, 0u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 2099u16, 1u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(
            0u8,
            2100u16,
            2u8,
            19u8,
            ((((width) as i32).wrapping_sub(2i32)) as u8),
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(0u8, 2101u16, 28u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 2102u16, 29u8, 19u8, 1u8, 1u8, palNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EraseArenaRefereeTextBox() {
    unsafe {
        let mut width: u8 = 0u8;
        let mut height: u8 = 0u8;
        let mut palNum: u8 = 0u8;
        FillBgTilemapBufferRect(0u8, 3u16, 0u8, 14u8, 1u8, 1u8, palNum);
        height = 4u8;
        FillBgTilemapBufferRect(0u8, 4u16, 1u8, 14u8, 1u8, 1u8, palNum);
        width = 27u8;
        FillBgTilemapBufferRect(0u8, 5u16, 2u8, 14u8, width, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 6u16, 28u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 7u16, 29u8, 14u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 8u16, 0u8, 15u8, 1u8, height, palNum);
        FillBgTilemapBufferRect(0u8, 9u16, 1u8, 15u8, 1u8, height, palNum);
        FillBgTilemapBufferRect(0u8, 10u16, 2u8, 15u8, width, height, palNum);
        FillBgTilemapBufferRect(0u8, 11u16, 28u8, 15u8, 1u8, height, palNum);
        FillBgTilemapBufferRect(0u8, 12u16, 29u8, 15u8, 1u8, height, palNum);
        FillBgTilemapBufferRect(0u8, 13u16, 0u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 14u16, 1u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 15u16, 2u8, 19u8, width, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 16u16, 28u8, 19u8, 1u8, 1u8, palNum);
        FillBgTilemapBufferRect(0u8, 17u16, 29u8, 19u8, 1u8, 1u8, palNum);
    }
}
