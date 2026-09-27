//! Translated from `src/battle_ai_switch_items.c` by tools/rustport/c2rs.py, then reviewed.
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

unsafe extern "C" {
    static mut gAbsentBattlerFlags: u8;
    static mut gActiveBattler: u8;
    static mut gBattleMons: u8;
    static mut gBattleMoveDamage: u8;
    static mut gBattleMoves: u8;
    static mut gBattleResources: u8;
    static mut gBattleScripting: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBitTable: u8;
    static mut gCritMultiplier: u8;
    static mut gDisableStructs: u8;
    static mut gDynamicBasePower: u8;
    static mut gEnemyParty: u8;
    static mut gItemEffectTable: u8;
    static mut gLastHitBy: u8;
    static mut gLastLandedMoves: u8;
    static mut gMoveResultFlags: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSideTimers: u8;
    static mut gSpeciesInfo: u8;
    static mut gStatuses3: u8;
    static mut gTypeEffectiveness: u8;
    fn AI_CalcDmg(a0: u8, a1: u8);
    fn AI_TypeCalc(a0: u16, a1: u16, a2: u8) -> u8;
    fn AbilityBattleEffects(a0: u8, a1: u8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BtlController_EmitTwoReturnValues(a0: u8, a1: u8, a2: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetItemEffectParamOffset(a0: u16, a1: u8, a2: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn Random() -> u16;
    fn TypeCalc(a0: u16, a1: u8, a2: u8) -> u8;
}

pub(crate) unsafe extern "C" fn ShouldSwitchIfPerishSong() -> u8 {
    unsafe {
        if ((((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()
            & 32u32)
            != 0)
            && (((crate::c::bf_read(
                (((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 28,
                ))
                .wrapping_add(15),
                0,
                4,
                false,
            ) as u8) as i32)
                == 0i32)
        {
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                .cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(6u8);
            BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
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
pub(crate) unsafe extern "C" fn ShouldSwitchIfWonderGuard() -> u8 {
    unsafe {
        let mut opposingPosition: u8 = 0u8;
        let mut opposingBattler: u8 = 0u8;
        let mut moveFlags: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut r#move: u16 = 0u16;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            return 0u8;
        }
        opposingPosition = ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
            as i32)
            ^ 1i32) as u8);
        if ((((((&raw mut gBattleMons).cast::<u8>())
            .wrapping_offset(((GetBattlerAtPosition(opposingPosition)) as i32) as isize * 88))
        .wrapping_add(32))
        .read()) as i32)
            != 25i32
        {
            return 0u8;
        }
        {
            opposingBattler = GetBattlerAtPosition(opposingPosition);
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    r#move = ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(12))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if ((r#move) as i32) == 0i32 {
                        break 'l2;
                    }
                    moveFlags = AI_TypeCalc(
                        r#move,
                        ((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                        .cast::<u16>())
                        .read(),
                        ((((&raw mut gBattleMons).cast::<u8>())
                            .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                        .wrapping_add(32))
                        .read(),
                    );
                    if (((moveFlags) as i32) & 2i32) != 0 {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32) == 0i32 {
                firstId = 0i32;
                lastId = crate::c::div_i32(6i32, 2i32);
            } else {
                firstId = crate::c::div_i32(6i32, 2i32);
                lastId = 6i32;
            }
        } else {
            firstId = 0i32;
            lastId = 6i32;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        {
            i = firstId;
            'l3: loop {
                if !(i < lastId) {
                    break 'l3;
                }
                'l4: {
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) == 0u32 {
                        break 'l4;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 0u32 {
                        break 'l4;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 412u32 {
                        break 'l4;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32)
                    {
                        break 'l4;
                    }
                    GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32);
                    GetMonData2((party).wrapping_offset((i) as isize * 100), 46i32);
                    {
                        opposingBattler = GetBattlerAtPosition(opposingPosition);
                        j = 0i32;
                        'l5: loop {
                            if !(j < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                r#move = ((GetMonData2(
                                    (party).wrapping_offset((i) as isize * 100),
                                    (13i32).wrapping_add(j),
                                )) as u16);
                                if ((r#move) as i32) == 0i32 {
                                    break 'l6;
                                }
                                moveFlags = AI_TypeCalc(
                                    r#move,
                                    ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((opposingBattler) as i32) as isize * 88,
                                    ))
                                    .cast::<u16>())
                                    .read(),
                                    ((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((opposingBattler) as i32) as isize * 88,
                                    ))
                                    .wrapping_add(32))
                                    .read(),
                                );
                                if ((((moveFlags) as i32) & 2i32) != 0)
                                    && (crate::c::rem_i32(((Random()) as i32), 3i32) < 2i32)
                                {
                                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                        .wrapping_add(660))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize,
                                    ))
                                    .write(((i) as u8));
                                    BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
                                    return 1u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FindMonThatAbsorbsOpponentsMove() -> u8 {
    unsafe {
        let mut battlerIn1: u8 = 0u8;
        let mut battlerIn2: u8 = 0u8;
        let mut absorbingTypeAbility: u8 = 0u8;
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        if ((HasSuperEffectiveMoveAgainstOpponents(1u8)) != 0)
            && (crate::c::rem_i32(((Random()) as i32), 3i32) != 0i32)
        {
            return 0u8;
        }
        if ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        if ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 65535i32
        {
            return 0u8;
        }
        if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((GetBattlerAtPosition(
                        ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            ^ 2i32) as u8),
                    )) as i32) as isize,
                ))
                .read())
                != 0
            {
                battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            } else {
                battlerIn2 = GetBattlerAtPosition(
                    ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                        as i32)
                        ^ 2i32) as u8),
                );
            }
        } else {
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
        }
        if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(2))
        .read()) as i32)
            == 10i32
        {
            absorbingTypeAbility = 18u8;
        } else {
            if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(2))
            .read()) as i32)
                == 11i32
            {
                absorbingTypeAbility = 11u8;
            } else {
                if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(2))
                .read()) as i32)
                    == 13i32
                {
                    absorbingTypeAbility = 10u8;
                } else {
                    return 0u8;
                }
            }
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(32))
        .read()) as i32)
            == ((absorbingTypeAbility) as i32)
        {
            return 0u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32) == 0i32 {
                firstId = 0i32;
                lastId = crate::c::div_i32(6i32, 2i32);
            } else {
                firstId = crate::c::div_i32(6i32, 2i32);
                lastId = 6i32;
            }
        } else {
            firstId = 0i32;
            lastId = 6i32;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        {
            i = firstId;
            'l1: loop {
                if !(i < lastId) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = 0u16;
                    let mut monAbility: u8 = 0u8;
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 412u32 {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    species =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32)) as u16);
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 46i32) != 0u32 {
                        monAbility = ((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read();
                    } else {
                        monAbility = (((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .read();
                    }
                    if (((absorbingTypeAbility) as i32) == ((monAbility) as i32))
                        && ((((Random()) as i32) & 1i32) != 0)
                    {
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(660))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .write(((i) as u8));
                        BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldSwitchIfNaturalCure() -> u8 {
    unsafe {
        if !((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(76)
        .cast::<u32>())
        .read()
            & 7u32)
            != 0)
        {
            return 0u8;
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(32))
        .read()) as i32)
            != 30i32
        {
            return 0u8;
        }
        if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
        ))
        .wrapping_add(40)
        .cast::<u16>())
        .read()) as i32)
            < crate::c::div_i32(
                ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(44)
                .cast::<u16>())
                .read()) as i32),
                2i32,
            )
        {
            return 0u8;
        }
        if ((((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 0i32)
            || (((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                == 65535i32))
            && ((((Random()) as i32) & 1i32) != 0)
        {
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                .cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(6u8);
            BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
            return 1u8;
        } else {
            if (((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == 0i32)
                && ((((Random()) as i32) & 1i32) != 0)
            {
                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(6u8);
                BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
                return 1u8;
            }
        }
        if (FindMonWithFlagsAndSuperEffective(8u8, 1u8)) != 0 {
            return 1u8;
        }
        if (FindMonWithFlagsAndSuperEffective(4u8, 1u8)) != 0 {
            return 1u8;
        }
        if (((Random()) as i32) & 1i32) != 0 {
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                .cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .write(6u8);
            BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HasSuperEffectiveMoveAgainstOpponents(noRng: u8) -> u8 {
    unsafe {
        let mut noRng = noRng;
        let mut opposingPosition: u8 = 0u8;
        let mut opposingBattler: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut moveFlags: u8 = 0u8;
        let mut r#move: u16 = 0u16;
        opposingPosition = ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
            as i32)
            ^ 1i32) as u8);
        opposingBattler = GetBattlerAtPosition(opposingPosition);
        if !((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                .wrapping_offset(((opposingBattler) as i32) as isize))
            .read())
            != 0)
        {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        r#move = ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read();
                        if ((r#move) as i32) == 0i32 {
                            break 'l2;
                        }
                        moveFlags = AI_TypeCalc(
                            r#move,
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                            .cast::<u16>())
                            .read(),
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                        if (((moveFlags) as i32) & 2i32) != 0 {
                            if (noRng) != 0 {
                                return 1u8;
                            }
                            if crate::c::rem_i32(((Random()) as i32), 10i32) != 0i32 {
                                return 1u8;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
            return 0u8;
        }
        opposingBattler = GetBattlerAtPosition(((((opposingPosition) as i32) ^ 2i32) as u8));
        if !((((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
            & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                .wrapping_offset(((opposingBattler) as i32) as isize))
            .read())
            != 0)
        {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        r#move = ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(12))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read();
                        if ((r#move) as i32) == 0i32 {
                            break 'l4;
                        }
                        moveFlags = AI_TypeCalc(
                            r#move,
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                            .cast::<u16>())
                            .read(),
                            ((((&raw mut gBattleMons).cast::<u8>())
                                .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                            .wrapping_add(32))
                            .read(),
                        );
                        if (((moveFlags) as i32) & 2i32) != 0 {
                            if (noRng) != 0 {
                                return 1u8;
                            }
                            if crate::c::rem_i32(((Random()) as i32), 10i32) != 0i32 {
                                return 1u8;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AreStatsRaised() -> u8 {
    unsafe {
        let mut buffedStatsValue: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                    ))
                    .wrapping_add(24))
                    .cast::<i8>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        > 6i32
                    {
                        buffedStatsValue = ((((buffedStatsValue) as i32).wrapping_add(
                            ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(24))
                            .cast::<i8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                .wrapping_sub(6i32),
                        )) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((((buffedStatsValue) as i32) > 3i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn FindMonWithFlagsAndSuperEffective(
    flags: u8,
    moduloPercent: u8,
) -> u8 {
    unsafe {
        let mut flags = flags;
        let mut moduloPercent = moduloPercent;
        let mut battlerIn1: u8 = 0u8;
        let mut battlerIn2: u8 = 0u8;
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut r#move: u16 = 0u16;
        let mut moveFlags: u8 = 0u8;
        if ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        if ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 65535i32
        {
            return 0u8;
        }
        if (((((&raw mut gLastHitBy).cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            == 255i32
        {
            return 0u8;
        }
        if ((((((&raw mut gBattleMoves).cast::<u8>()).wrapping_offset(
            ((((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 0i32
        {
            return 0u8;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((GetBattlerAtPosition(
                        ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            ^ 2i32) as u8),
                    )) as i32) as isize,
                ))
                .read())
                != 0
            {
                battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            } else {
                battlerIn2 = GetBattlerAtPosition(
                    ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                        as i32)
                        ^ 2i32) as u8),
                );
            }
        } else {
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32) == 0i32 {
                firstId = 0i32;
                lastId = crate::c::div_i32(6i32, 2i32);
            } else {
                firstId = crate::c::div_i32(6i32, 2i32);
                lastId = 6i32;
            }
        } else {
            firstId = 0i32;
            lastId = 6i32;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        {
            i = firstId;
            'l1: loop {
                if !(i < lastId) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = 0u16;
                    let mut monAbility: u8 = 0u8;
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 412u32 {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    species =
                        ((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32)) as u16);
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 46i32) != 0u32 {
                        monAbility = ((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read();
                    } else {
                        monAbility = (((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(22))
                        .cast::<u8>())
                        .read();
                    }
                    moveFlags = AI_TypeCalc(
                        ((((&raw mut gLastLandedMoves).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                            ))
                        .read(),
                        species,
                        monAbility,
                    );
                    if (((moveFlags) as i32) & ((flags) as i32)) != 0 {
                        battlerIn1 = (((&raw mut gLastHitBy).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read();
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    r#move = ((GetMonData2(
                                        (party).wrapping_offset((i) as isize * 100),
                                        (13i32).wrapping_add(j),
                                    )) as u16);
                                    if ((r#move) as i32) == 0i32 {
                                        break 'l4;
                                    }
                                    moveFlags = AI_TypeCalc(
                                        r#move,
                                        ((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battlerIn1) as i32) as isize * 88))
                                        .cast::<u16>())
                                        .read(),
                                        ((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(((battlerIn1) as i32) as isize * 88))
                                        .wrapping_add(32))
                                        .read(),
                                    );
                                    if ((((moveFlags) as i32) & 2i32) != 0)
                                        && (crate::c::rem_i32(
                                            ((Random()) as i32),
                                            ((moduloPercent) as i32),
                                        ) == 0i32)
                                    {
                                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                            .wrapping_add(660))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize,
                                        ))
                                        .write(((i) as u8));
                                        BtlController_EmitTwoReturnValues(1u8, 2u8, 0u16);
                                        return 1u8;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldSwitch() -> u8 {
    unsafe {
        let mut battlerIn1: u8 = 0u8;
        let mut battlerIn2: u8 = 0u8;
        let mut activeBattlerPtr: *mut u8 = core::ptr::null_mut();
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut availableToSwitch: i32 = 0i32;
        if (((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
            ((({
                let __v2 = (&raw mut gActiveBattler).cast::<u8>();
                activeBattlerPtr = __v2;
                __v2
            })
            .read()) as i32) as isize
                * 88,
        ))
        .wrapping_add(80)
        .cast::<u32>())
        .read()
            & 67166208u32)
            != 0
        {
            return 0u8;
        }
        if (((((&raw mut gStatuses3).cast::<u32>()).cast::<u32>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()
            & 1024u32)
            != 0
        {
            return 0u8;
        }
        if (AbilityBattleEffects(
            12u8,
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            23u8,
            0u8,
            0u16,
        )) != 0
        {
            return 0u8;
        }
        if (AbilityBattleEffects(
            12u8,
            ((&raw mut gActiveBattler).cast::<u8>()).read(),
            71u8,
            0u8,
            0u16,
        )) != 0
        {
            return 0u8;
        }
        if (AbilityBattleEffects(14u8, 0u8, 42u8, 0u8, 0u16)) != 0 {
            if ((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
            ))
            .wrapping_add(33))
            .cast::<u8>())
            .read()) as i32)
                == 8i32)
                || (((((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 88,
                ))
                .wrapping_add(33))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 8i32)
            {
                return 0u8;
            }
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0 {
            return 0u8;
        }
        availableToSwitch = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            battlerIn1 = (activeBattlerPtr).read();
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((GetBattlerAtPosition(
                        ((((GetBattlerPosition((activeBattlerPtr).read())) as i32) ^ 2i32) as u8),
                    )) as i32) as isize,
                ))
                .read())
                != 0
            {
                battlerIn2 = (activeBattlerPtr).read();
            } else {
                battlerIn2 = GetBattlerAtPosition(
                    ((((GetBattlerPosition((activeBattlerPtr).read())) as i32) ^ 2i32) as u8),
                );
            }
        } else {
            battlerIn1 = (activeBattlerPtr).read();
            battlerIn2 = (activeBattlerPtr).read();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32) == 0i32 {
                firstId = 0i32;
                lastId = crate::c::div_i32(6i32, 2i32);
            } else {
                firstId = crate::c::div_i32(6i32, 2i32);
                lastId = 6i32;
            }
        } else {
            firstId = 0i32;
            lastId = 6i32;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        {
            i = firstId;
            'l1: loop {
                if !(i < lastId) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 0u32 {
                        break 'l2;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32) == 412u32 {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l2;
                    }
                    availableToSwitch = (availableToSwitch).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        if availableToSwitch == 0i32 {
            return 0u8;
        }
        if (ShouldSwitchIfPerishSong()) != 0 {
            return 1u8;
        }
        if (ShouldSwitchIfWonderGuard()) != 0 {
            return 1u8;
        }
        if (FindMonThatAbsorbsOpponentsMove()) != 0 {
            return 1u8;
        }
        if (ShouldSwitchIfNaturalCure()) != 0 {
            return 1u8;
        }
        if (HasSuperEffectiveMoveAgainstOpponents(0u8)) != 0 {
            return 0u8;
        }
        if (AreStatsRaised()) != 0 {
            return 0u8;
        }
        if ((FindMonWithFlagsAndSuperEffective(8u8, 2u8)) != 0)
            || ((FindMonWithFlagsAndSuperEffective(4u8, 3u8)) != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AI_TrySwitchOrUseItem() {
    unsafe {
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut battlerIn1: u8 = 0u8;
        let mut battlerIn2: u8 = 0u8;
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut battlerIdentity: u8 =
            GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read());
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
            if (ShouldSwitch()) != 0 {
                if ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32)
                    == 6i32
                {
                    let mut monToSwitchId: i32 = ((GetMostSuitableMonToSwitchInto()) as i32);
                    if monToSwitchId == 6i32 {
                        if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0) {
                            battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                            battlerIn2 = battlerIn1;
                        } else {
                            battlerIn1 = GetBattlerAtPosition(battlerIdentity);
                            battlerIn2 =
                                GetBattlerAtPosition(((((battlerIdentity) as i32) ^ 2i32) as u8));
                        }
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
                            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32)
                                == 0i32
                            {
                                firstId = 0i32;
                                lastId = crate::c::div_i32(6i32, 2i32);
                            } else {
                                firstId = crate::c::div_i32(6i32, 2i32);
                                lastId = 6i32;
                            }
                        } else {
                            firstId = 0i32;
                            lastId = 6i32;
                        }
                        {
                            monToSwitchId = firstId;
                            'l1: loop {
                                if !(monToSwitchId < lastId) {
                                    break 'l1;
                                }
                                'l2: {
                                    if GetMonData2(
                                        (party).wrapping_offset((monToSwitchId) as isize * 100),
                                        57i32,
                                    ) == 0u32
                                    {
                                        break 'l2;
                                    }
                                    if monToSwitchId
                                        == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((battlerIn1) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l2;
                                    }
                                    if monToSwitchId
                                        == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(((battlerIn2) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l2;
                                    }
                                    if monToSwitchId
                                        == ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(92))
                                        .cast::<u8>())
                                        .wrapping_offset(((battlerIn1) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l2;
                                    }
                                    if monToSwitchId
                                        == ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(92))
                                        .cast::<u8>())
                                        .wrapping_offset(((battlerIn2) as i32) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l2;
                                    }
                                    break 'l1;
                                }
                                monToSwitchId = (monToSwitchId).wrapping_add(1);
                            }
                        }
                    }
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .write(((monToSwitchId) as u8));
                }
                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(92))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
                .write(
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(660))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                return;
            } else {
                if (ShouldUseItem()) != 0 {
                    return;
                }
            }
        }
        BtlController_EmitTwoReturnValues(
            1u8,
            0u8,
            (((((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) ^ 1i32) << 8) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ModulateByTypeEffectiveness(
    atkType: u8,
    defType1: u8,
    defType2: u8,
    var: *mut u8,
) {
    unsafe {
        let mut atkType = atkType;
        let mut defType1 = defType1;
        let mut defType2 = defType2;
        let mut var = var;
        let mut i: i32 = 0i32;
        'l1: loop {
            if !((((((&raw mut gTypeEffectiveness).cast::<u8>())
                .wrapping_offset(((i).wrapping_add(0i32)) as isize))
            .read()) as i32)
                != 255i32)
            {
                break 'l1;
            }
            if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                .wrapping_offset(((i).wrapping_add(0i32)) as isize))
            .read()) as i32)
                == 254i32
            {
                i = (i).wrapping_add(3i32);
                continue 'l1;
            } else {
                if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(0i32)) as isize))
                .read()) as i32)
                    == ((atkType) as i32)
                {
                    if (((((&raw mut gTypeEffectiveness).cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .read()) as i32)
                        == ((defType1) as i32)
                    {
                        (var).write(
                            ((crate::c::div_i32(
                                (((var).read()) as i32).wrapping_mul(
                                    (((((&raw mut gTypeEffectiveness).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                    .read()) as i32),
                                ),
                                10i32,
                            )) as u8),
                        );
                    }
                    if ((((((&raw mut gTypeEffectiveness).cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .read()) as i32)
                        == ((defType2) as i32))
                        && (((defType1) as i32) != ((defType2) as i32))
                    {
                        (var).write(
                            ((crate::c::div_i32(
                                (((var).read()) as i32).wrapping_mul(
                                    (((((&raw mut gTypeEffectiveness).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_add(2i32)) as isize))
                                    .read()) as i32),
                                ),
                                10i32,
                            )) as u8),
                        );
                    }
                }
            }
            i = (i).wrapping_add(3i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMostSuitableMonToSwitchInto() -> u8 {
    unsafe {
        let mut opposingBattler: u8 = 0u8;
        let mut bestDmg: u8 = 0u8;
        let mut bestMonId: u8 = 0u8;
        let mut battlerIn1: u8 = 0u8;
        let mut battlerIn2: u8 = 0u8;
        let mut firstId: i32 = 0i32;
        let mut lastId: i32 = 0i32;
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut invalidMons: u8 = 0u8;
        let mut r#move: u16 = 0u16;
        if ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(92))
            .cast::<u8>())
        .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
        .read()) as i32)
            != 6i32
        {
            return ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(92))
                .cast::<u8>())
            .wrapping_offset(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize))
            .read();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0 {
            return ((((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize,
                ))
            .read()) as i32)
                .wrapping_add(1i32)) as u8);
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>()).wrapping_offset(
                    ((GetBattlerAtPosition(
                        ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                            as i32)
                            ^ 2i32) as u8),
                    )) as i32) as isize,
                ))
                .read())
                != 0
            {
                battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            } else {
                battlerIn2 = GetBattlerAtPosition(
                    ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read()))
                        as i32)
                        ^ 2i32) as u8),
                );
            }
            opposingBattler = ((((Random()) as i32) & 2i32) as u8);
            if (((((&raw mut gAbsentBattlerFlags).cast::<u8>()).read()) as u32)
                & ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                    .wrapping_offset(((opposingBattler) as i32) as isize))
                .read())
                != 0
            {
                opposingBattler = ((((opposingBattler) as i32) ^ 2i32) as u8);
            }
        } else {
            opposingBattler = GetBattlerAtPosition(
                ((((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                    ^ 1i32) as u8),
            );
            battlerIn1 = ((&raw mut gActiveBattler).cast::<u8>()).read();
            battlerIn2 = ((&raw mut gActiveBattler).cast::<u8>()).read();
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8421376u32) != 0 {
            if (((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) & 2i32) == 0i32 {
                firstId = 0i32;
                lastId = crate::c::div_i32(6i32, 2i32);
            } else {
                firstId = crate::c::div_i32(6i32, 2i32);
                lastId = 6i32;
            }
        } else {
            firstId = 0i32;
            lastId = 6i32;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        invalidMons = 0u8;
        'l1: loop {
            if !(((invalidMons) as i32) != 63i32) {
                break 'l1;
            }
            bestDmg = 0u8;
            bestMonId = 6u8;
            {
                i = firstId;
                'l2: loop {
                    if !(i < lastId) {
                        break 'l2;
                    }
                    'l3: {
                        let mut species: u16 =
                            ((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32))
                                as u16);
                        if ((((((((species) as i32) != 0i32)
                            && (GetMonData2(
                                (party).wrapping_offset((i) as isize * 100),
                                57i32,
                            ) != 0u32))
                            && (!((((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                .wrapping_offset((i) as isize))
                            .read()
                                & ((invalidMons) as u32))
                                != 0)))
                            && (((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerIn1) as i32) as isize))
                            .read()) as i32)
                                != i))
                            && (((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battlerIn2) as i32) as isize))
                            .read()) as i32)
                                != i))
                            && (i
                                != ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(92))
                                .cast::<u8>())
                                .wrapping_offset(((battlerIn1) as i32) as isize))
                                .read()) as i32)))
                            && (i
                                != ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                    .wrapping_add(92))
                                .cast::<u8>())
                                .wrapping_offset(((battlerIn2) as i32) as isize))
                                .read()) as i32))
                        {
                            let mut type1: u8 = (((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .read();
                            let mut type2: u8 = ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read();
                            let mut typeDmg: u8 = 10u8;
                            ModulateByTypeEffectiveness(
                                (((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                                .wrapping_add(33))
                                .cast::<u8>())
                                .read(),
                                type1,
                                type2,
                                &raw mut typeDmg,
                            );
                            ModulateByTypeEffectiveness(
                                ((((((&raw mut gBattleMons).cast::<u8>())
                                    .wrapping_offset(((opposingBattler) as i32) as isize * 88))
                                .wrapping_add(33))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                                type1,
                                type2,
                                &raw mut typeDmg,
                            );
                            if ((bestDmg) as i32) < ((typeDmg) as i32) {
                                bestDmg = typeDmg;
                                bestMonId = ((i) as u8);
                            }
                        } else {
                            invalidMons = ((((invalidMons) as u32)
                                | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                                    .wrapping_offset((i) as isize))
                                .read()) as u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((bestMonId) as i32) != 6i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            r#move = ((GetMonData2(
                                (party).wrapping_offset(((bestMonId) as i32) as isize * 100),
                                (13i32).wrapping_add(i),
                            )) as u16);
                            if (((r#move) as i32) != 0i32)
                                && ((((TypeCalc(
                                    r#move,
                                    ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                    opposingBattler,
                                )) as i32)
                                    & 2i32)
                                    != 0)
                            {
                                break 'l4;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i != 4i32 {
                    return bestMonId;
                }
                invalidMons = ((((invalidMons) as u32)
                    | ((((&raw mut gBitTable).cast::<u32>()).cast::<u32>())
                        .wrapping_offset(((bestMonId) as i32) as isize))
                    .read()) as u8);
            } else {
                invalidMons = 63u8;
            }
        }
        ((&raw mut gDynamicBasePower).cast::<u16>()).write(0u16);
        ((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(19)).write(0u8);
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(14)).write(1u8);
        ((&raw mut gMoveResultFlags).cast::<u8>()).write(0u8);
        ((&raw mut gCritMultiplier).cast::<u8>()).write(1u8);
        bestDmg = 0u8;
        bestMonId = 6u8;
        {
            i = firstId;
            'l6: loop {
                if !(i < lastId) {
                    break 'l6;
                }
                'l7: {
                    if (((GetMonData2((party).wrapping_offset((i) as isize * 100), 11i32)) as u16)
                        as i32)
                        == 0i32
                    {
                        break 'l7;
                    }
                    if GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) == 0u32 {
                        break 'l7;
                    }
                    if ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                        == i
                    {
                        break 'l7;
                    }
                    if ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                        == i
                    {
                        break 'l7;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn1) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l7;
                    }
                    if i == ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                        .wrapping_add(92))
                    .cast::<u8>())
                    .wrapping_offset(((battlerIn2) as i32) as isize))
                    .read()) as i32)
                    {
                        break 'l7;
                    }
                    {
                        j = 0i32;
                        'l8: loop {
                            if !(j < 4i32) {
                                break 'l8;
                            }
                            'l9: {
                                r#move = ((GetMonData2(
                                    (party).wrapping_offset((i) as isize * 100),
                                    (13i32).wrapping_add(j),
                                )) as u16);
                                ((&raw mut gBattleMoveDamage).cast::<i32>()).write(0i32);
                                if (((r#move) as i32) != 0i32)
                                    && (((((((&raw mut gBattleMoves).cast::<u8>())
                                        .wrapping_offset(((r#move) as i32) as isize * 12))
                                    .wrapping_add(1))
                                    .read()) as i32)
                                        != 1i32)
                                {
                                    AI_CalcDmg(
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                        opposingBattler,
                                    );
                                    TypeCalc(
                                        r#move,
                                        ((&raw mut gActiveBattler).cast::<u8>()).read(),
                                        opposingBattler,
                                    );
                                }
                                if ((bestDmg) as i32)
                                    < ((&raw mut gBattleMoveDamage).cast::<i32>()).read()
                                {
                                    bestDmg = ((((&raw mut gBattleMoveDamage).cast::<i32>()).read())
                                        as u8);
                                    bestMonId = ((i) as u8);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return bestMonId;
    }
}
pub(crate) unsafe extern "C" fn GetAI_ItemType(itemId: u8, itemEffect: *mut u8) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut itemEffect = itemEffect;
        if ((itemId) as i32) == 19i32 {
            return 1u8;
        } else {
            if (((((itemEffect).wrapping_offset(4)).read()) as i32) & 4i32) != 0 {
                return 2u8;
            } else {
                if (((((itemEffect).wrapping_offset(3)).read()) as i32) & 63i32) != 0 {
                    return 3u8;
                } else {
                    if ((((((itemEffect).read()) as i32) & 63i32) != 0)
                        || (((((itemEffect).wrapping_offset(1)).read()) as i32) != 0i32))
                        || (((((itemEffect).wrapping_offset(2)).read()) as i32) != 0i32)
                    {
                        return 4u8;
                    } else {
                        if (((((itemEffect).wrapping_offset(3)).read()) as i32) & 128i32) != 0 {
                            return 5u8;
                        } else {
                            return 6u8;
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ShouldUseItem() -> u8 {
    unsafe {
        let mut party: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut validMons: u8 = 0u8;
        let mut shouldUse: u8 = 0u8;
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 4194304u32) != 0)
            && (((GetBattlerPosition(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32)
                == 2i32)
        {
            return 0u8;
        }
        if ((GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read())) as i32) == 0i32 {
            party = (&raw mut gPlayerParty).cast::<u8>();
        } else {
            party = (&raw mut gEnemyParty).cast::<u8>();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetMonData2((party).wrapping_offset((i) as isize * 100), 57i32) != 0u32)
                        && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                            != 0u32))
                        && (GetMonData2((party).wrapping_offset((i) as isize * 100), 65i32)
                            != 412u32)
                    {
                        validMons = (validMons).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    let mut item: u16 = 0u16;
                    let mut itemEffects: *mut u8 = core::ptr::null_mut();
                    let mut paramOffset: u8 = 0u8;
                    let mut battlerSide: u8 = 0u8;
                    if (i != 0i32)
                        && (((validMons) as i32)
                            > (((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(80))
                            .read()) as i32)
                                .wrapping_sub(i))
                            .wrapping_add(1i32))
                    {
                        break 'l4;
                    }
                    item = ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(72))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    if ((item) as i32) == 0i32 {
                        break 'l4;
                    }
                    if ((((((&raw mut gItemEffectTable).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
                    .read()) as usize)
                        == 0usize
                    {
                        break 'l4;
                    }
                    if ((item) as i32) == 175i32 {
                        itemEffects = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(12792))
                        .wrapping_add(28))
                        .cast::<u8>();
                    } else {
                        itemEffects = ((((&raw mut gItemEffectTable).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
                        .read();
                    }
                    ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(196))
                        .cast::<u8>())
                    .wrapping_offset(
                        (crate::c::div_i32(
                            ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                            2i32,
                        )) as isize,
                    ))
                    .write(GetAI_ItemType(((item) as u8), itemEffects));
                    'l5: {
                        let __sw1 = ((((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(196))
                        .cast::<u8>())
                        .wrapping_offset(
                            (crate::c::div_i32(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                2i32,
                            )) as isize,
                        ))
                        .read()) as i32);
                        if __sw1 == 1i32 {
                            if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                >= crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    4i32,
                                )
                            {
                                break 'l5;
                            }
                            if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                == 0i32
                            {
                                break 'l5;
                            }
                            shouldUse = 1u8;
                            break 'l5;
                        }
                        if __sw1 == 2i32 {
                            paramOffset = GetItemEffectParamOffset(item, 4u8, 4u8);
                            if ((paramOffset) as i32) == 0i32 {
                                break 'l5;
                            }
                            if ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                == 0i32
                            {
                                break 'l5;
                            }
                            if (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 88,
                            ))
                            .wrapping_add(40)
                            .cast::<u16>())
                            .read()) as i32)
                                < crate::c::div_i32(
                                    ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(44)
                                    .cast::<u16>())
                                    .read()) as i32),
                                    4i32,
                                ))
                                || (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(44)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_sub(
                                        ((((((&raw mut gBattleMons).cast::<u8>())
                                            .wrapping_offset(
                                                ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                    as i32)
                                                    as isize
                                                    * 88,
                                            ))
                                        .wrapping_add(40)
                                        .cast::<u16>())
                                        .read()) as i32),
                                    )
                                    > ((((itemEffects)
                                        .wrapping_offset(((paramOffset) as i32) as isize))
                                    .read()) as i32))
                            {
                                shouldUse = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw1 == 3i32 {
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(198))
                            .cast::<u8>())
                            .wrapping_offset(
                                (crate::c::div_i32(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                    2i32,
                                )) as isize,
                            ))
                            .write(0u8);
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 32i32) != 0)
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0)
                            {
                                let __p2 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p2).write((((((__p2).read()) as i32) | 32i32) as u8));
                                shouldUse = 1u8;
                            }
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 16i32) != 0)
                                && (((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 8u32)
                                    != 0)
                                    || ((((((&raw mut gBattleMons).cast::<u8>())
                                        .wrapping_offset(
                                            ((((&raw mut gActiveBattler).cast::<u8>()).read())
                                                as i32)
                                                as isize
                                                * 88,
                                        ))
                                    .wrapping_add(76)
                                    .cast::<u32>())
                                    .read()
                                        & 128u32)
                                        != 0))
                            {
                                let __p3 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p3).write((((((__p3).read()) as i32) | 16i32) as u8));
                                shouldUse = 1u8;
                            }
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 8i32) != 0)
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 16u32)
                                    != 0)
                            {
                                let __p4 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p4).write((((((__p4).read()) as i32) | 8i32) as u8));
                                shouldUse = 1u8;
                            }
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 4i32) != 0)
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 32u32)
                                    != 0)
                            {
                                let __p5 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p5).write((((((__p5).read()) as i32) | 4i32) as u8));
                                shouldUse = 1u8;
                            }
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 2i32) != 0)
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(76)
                                .cast::<u32>())
                                .read()
                                    & 64u32)
                                    != 0)
                            {
                                let __p6 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p6).write((((((__p6).read()) as i32) | 2i32) as u8));
                                shouldUse = 1u8;
                            }
                            if ((((((itemEffects).wrapping_offset(3)).read()) as i32) & 1i32) != 0)
                                && ((((((&raw mut gBattleMons).cast::<u8>()).wrapping_offset(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 88,
                                ))
                                .wrapping_add(80)
                                .cast::<u32>())
                                .read()
                                    & 7u32)
                                    != 0)
                            {
                                let __p7 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p7).write((((((__p7).read()) as i32) | 1i32) as u8));
                                shouldUse = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw1 == 4i32 {
                            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                                .wrapping_add(198))
                            .cast::<u8>())
                            .wrapping_offset(
                                (crate::c::div_i32(
                                    ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                    2i32,
                                )) as isize,
                            ))
                            .write(0u8);
                            if ((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(22))
                            .read()) as i32)
                                == 0i32
                            {
                                break 'l5;
                            }
                            if ((((itemEffects).read()) as i32) & 15i32) != 0 {
                                let __p8 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p8).write((((((__p8).read()) as i32) | 1i32) as u8));
                            }
                            if (((((itemEffects).wrapping_offset(1)).read()) as i32) & 240i32) != 0
                            {
                                let __p9 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p9).write((((((__p9).read()) as i32) | 2i32) as u8));
                            }
                            if (((((itemEffects).wrapping_offset(1)).read()) as i32) & 15i32) != 0 {
                                let __p10 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p10).write((((((__p10).read()) as i32) | 4i32) as u8));
                            }
                            if (((((itemEffects).wrapping_offset(2)).read()) as i32) & 15i32) != 0 {
                                let __p11 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p11).write((((((__p11).read()) as i32) | 8i32) as u8));
                            }
                            if (((((itemEffects).wrapping_offset(2)).read()) as i32) & 240i32) != 0
                            {
                                let __p12 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p12).write((((((__p12).read()) as i32) | 32i32) as u8));
                            }
                            if ((((itemEffects).read()) as i32) & 48i32) != 0 {
                                let __p13 = (((((&raw mut gBattleStruct).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(198))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (crate::c::div_i32(
                                        ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                        2i32,
                                    )) as isize,
                                );
                                (__p13).write((((((__p13).read()) as i32) | 128i32) as u8));
                            }
                            shouldUse = 1u8;
                            break 'l5;
                        }
                        if __sw1 == 5i32 {
                            battlerSide =
                                GetBattlerSide(((&raw mut gActiveBattler).cast::<u8>()).read());
                            if (((((((&raw mut gDisableStructs).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(22))
                            .read()) as i32)
                                != 0i32)
                                && (((((((&raw mut gSideTimers).cast::<u8>())
                                    .wrapping_offset(((battlerSide) as i32) as isize * 12))
                                .wrapping_add(4))
                                .read()) as i32)
                                    == 0i32)
                            {
                                shouldUse = 1u8;
                            }
                            break 'l5;
                        }
                        if __sw1 == 6i32 {
                            return 0u8;
                        }
                    }
                    if (shouldUse) != 0 {
                        BtlController_EmitTwoReturnValues(1u8, 1u8, 0u16);
                        ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                            .wrapping_add(192))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::div_i32(
                                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32),
                                2i32,
                            ))
                            .wrapping_mul(2i32)) as isize,
                        ))
                        .write(((item) as u8));
                        ((((((((&raw mut gBattleResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(72))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(0u16);
                        return shouldUse;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
