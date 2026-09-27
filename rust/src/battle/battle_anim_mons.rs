//! Translated from `src/battle_anim_mons.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBattlerCoords gCastformFrontSpriteCoords sCastformElevations sCastformBackSpriteYCoords sSpriteTemplates_MoveEffectMons sSpriteSheets_MoveEffectMons
#[allow(unused_imports)]
use crate::data::battle_anim_mons::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAnimTaskAffineAnim: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gAffineAnims_BattleSpriteContest: u8;
    static mut gAnimBattlerSpecies: u8;
    static mut gAnimFriendship: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimBgTileBuffer: u8;
    static mut gBattleAnimBgTilemapBuffer: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlersCount: u8;
    static mut gContestResources: u8;
    static mut gEnemyMonElevation: u8;
    static mut gEnemyParty: u8;
    static mut gMonBackPicCoords: u8;
    static mut gMonBackPicTable: u8;
    static mut gMonFrontPicCoords: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gOamMatrices: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gTransformedPersonalities: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimSetCenterToCornerVecX(a0: *mut u8);
    fn ArcTan2(a0: i16, a1: i16) -> u16;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn Free(a0: *mut u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetAnimBgAttribute(a0: u8, a1: u8) -> i32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32, a4: u8);
    fn LoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32, a4: u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn ObjAffineSet(a0: *mut u8, a1: *mut u8, a2: i32, a3: i32);
    fn PaletteStruct_ResetById(a0: u16);
    fn RelocateBattleBgPal(a0: u16, a1: *mut u16, a2: u32, a3: u8);
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn ShouldIgnoreDeoxysForm(a0: u8, a1: u8) -> u8;
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn UpdateMonIconFrame(a0: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteCoord(battler: u8, coordType: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut coordType = coordType;
        let mut retVal: u8 = 0u8;
        let mut species: u16 = 0u16;
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        if (IsContest()) != 0 {
            if (((coordType) as i32) == 3i32) && (((battler) as i32) == 3i32) {
                coordType = 1u8;
            }
        }
        'l1: {
            let __sw1 = ((coordType) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 1i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 || __sw1 == 2i32 {
                retVal = ((((((&raw const sBattlerCoords).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) as i32)
                            as isize
                            * 16,
                    ))
                .cast::<u8>())
                .wrapping_offset(((GetBattlerPosition(battler)) as i32) as isize * 4))
                .read();
                break 'l1;
            }
            if __sw1 == 1i32 {
                retVal = (((((((&raw const sBattlerCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) as i32) as isize
                        * 16,
                ))
                .cast::<u8>())
                .wrapping_offset(((GetBattlerPosition(battler)) as i32) as isize * 4))
                .wrapping_add(1))
                .read();
                break 'l1;
            }
            if __sw1 == 3i32 || __sw1 == 4i32 || !__matched {
                if (IsContest()) != 0 {
                    if (crate::c::bf_read(
                        (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read();
                    } else {
                        species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .read();
                    }
                } else {
                    if ((GetBattlerSide(battler)) as i32) != 0i32 {
                        spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read();
                        if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read())
                            != 0)
                        {
                            species = ((GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                11i32,
                            )) as u16);
                        } else {
                            species = (((spriteInfo)
                                .wrapping_offset(((battler) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                        }
                    } else {
                        spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                            .read())
                        .cast::<*mut u8>())
                        .read();
                        if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read())
                            != 0)
                        {
                            species = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                11i32,
                            )) as u16);
                        } else {
                            species = (((spriteInfo)
                                .wrapping_offset(((battler) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                        }
                    }
                }
                if ((coordType) as i32) == 3i32 {
                    retVal = GetBattlerSpriteFinal_Y(battler, species, 1u8);
                } else {
                    retVal = GetBattlerSpriteFinal_Y(battler, species, 0u8);
                }
                break 'l1;
            }
        }
        return retVal;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerYDelta(battler: u8, species: u16) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut species = species;
        let mut letter: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        let mut ret: u8 = 0u8;
        let mut coordSpecies: u16 = 0u16;
        if (((GetBattlerSide(battler)) as i32) == 0i32) || ((IsContest()) != 0) {
            if ((species) as i32) == 201i32 {
                if (IsContest()) != 0 {
                    if (crate::c::bf_read(
                        (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        personality = ((((((&raw mut gContestResources).cast::<*mut u8>())
                            .read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(16)
                        .cast::<u32>())
                        .read();
                    } else {
                        personality = ((((((&raw mut gContestResources).cast::<*mut u8>())
                            .read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(8)
                        .cast::<u32>())
                        .read();
                    }
                } else {
                    spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read();
                    if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read())
                        != 0)
                    {
                        personality = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((battler) as i32) as isize))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            0i32,
                        );
                    } else {
                        personality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                            .cast::<u32>())
                        .wrapping_offset(((battler) as i32) as isize))
                        .read();
                    }
                }
                letter = ((crate::c::rem_u32(
                    (((((personality & 50331648u32) >> 18) | ((personality & 196608u32) >> 12))
                        | ((personality & 768u32) >> 6))
                        | ((personality & 3u32) >> 0)),
                    28u32,
                )) as u16);
                if !((letter) != 0) {
                    coordSpecies = species;
                } else {
                    coordSpecies =
                        (((((letter) as i32).wrapping_add(413i32)).wrapping_sub(1i32)) as u16);
                }
                ret = ((((&raw mut gMonBackPicCoords).cast::<u8>())
                    .wrapping_offset(((coordSpecies) as i32) as isize * 4))
                .wrapping_add(1))
                .read();
            } else {
                if ((species) as i32) == 385i32 {
                    ret = ((((&raw const sCastformBackSpriteYCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMonForms).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .read();
                } else {
                    if ((species) as i32) > 412i32 {
                        ret = (((&raw mut gMonBackPicCoords).cast::<u8>()).wrapping_add(1)).read();
                    } else {
                        ret = ((((&raw mut gMonBackPicCoords).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read();
                    }
                }
            }
        } else {
            if ((species) as i32) == 201i32 {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    personality = GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0i32,
                    );
                } else {
                    personality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                        .cast::<u32>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read();
                }
                letter = ((crate::c::rem_u32(
                    (((((personality & 50331648u32) >> 18) | ((personality & 196608u32) >> 12))
                        | ((personality & 768u32) >> 6))
                        | ((personality & 3u32) >> 0)),
                    28u32,
                )) as u16);
                if !((letter) != 0) {
                    coordSpecies = species;
                } else {
                    coordSpecies =
                        (((((letter) as i32).wrapping_add(413i32)).wrapping_sub(1i32)) as u16);
                }
                ret = ((((&raw mut gMonFrontPicCoords).cast::<u8>())
                    .wrapping_offset(((coordSpecies) as i32) as isize * 4))
                .wrapping_add(1))
                .read();
            } else {
                if ((species) as i32) == 385i32 {
                    ret = (((((&raw const gCastformFrontSpriteCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMonForms).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .read();
                } else {
                    if ((species) as i32) > 412i32 {
                        ret = (((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_add(1)).read();
                    } else {
                        ret = ((((&raw mut gMonFrontPicCoords).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read();
                    }
                }
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerElevation(battler: u8, species: u16) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut species = species;
        let mut ret: u8 = 0u8;
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            if !((IsContest()) != 0) {
                if ((species) as i32) == 385i32 {
                    ret = ((((&raw const sCastformElevations).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMonForms).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize,
                    ))
                    .read();
                } else {
                    if ((species) as i32) > 412i32 {
                        ret = ((&raw mut gEnemyMonElevation).cast::<u8>()).read();
                    } else {
                        ret = (((&raw mut gEnemyMonElevation).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize))
                        .read();
                    }
                }
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteFinal_Y(battler: u8, species: u16, a3: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut species = species;
        let mut a3 = a3;
        let mut offset: u16 = 0u16;
        let mut y: u8 = 0u8;
        if (((GetBattlerSide(battler)) as i32) == 0i32) || ((IsContest()) != 0) {
            offset = ((GetBattlerYDelta(battler, species)) as u16);
        } else {
            offset = ((GetBattlerYDelta(battler, species)) as u16);
            offset = ((((offset) as i32)
                .wrapping_sub(((GetBattlerElevation(battler, species)) as i32)))
                as u16);
        }
        y = ((((offset) as i32).wrapping_add(
            (((((((((&raw const sBattlerCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) as i32) as isize
                        * 16,
                ))
            .cast::<u8>())
            .wrapping_offset(((GetBattlerPosition(battler)) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as i32),
        )) as u8);
        if (a3) != 0 {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                y = ((((y) as i32).wrapping_add(8i32)) as u8);
            }
            if ((y) as i32) > 104i32 {
                y = 104u8;
            }
        }
        return y;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteCoord2(battler: u8, coordType: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut coordType = coordType;
        let mut species: u16 = 0u16;
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        if (((coordType) as i32) == 3i32) || (((coordType) as i32) == 4i32) {
            if (IsContest()) != 0 {
                if (crate::c::bf_read(
                    (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read();
                } else {
                    species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read();
                }
            } else {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    species = ((((&raw mut gAnimBattlerSpecies).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read();
                } else {
                    species = (((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read();
                }
            }
            if ((coordType) as i32) == 3i32 {
                return GetBattlerSpriteFinal_Y(battler, species, 1u8);
            } else {
                return GetBattlerSpriteFinal_Y(battler, species, 0u8);
            }
        } else {
            return GetBattlerSpriteCoord(battler, coordType);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteDefault_Y(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        return GetBattlerSpriteCoord(battler, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSubstituteSpriteDefault_Y(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut y: u16 = 0u16;
        if ((GetBattlerSide(battler)) as i32) != 0i32 {
            y = ((((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(16i32)) as u16);
        } else {
            y = ((((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(17i32)) as u16);
        }
        return ((y) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerYCoordWithElevation(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut species: u16 = 0u16;
        let mut y: u8 = 0u8;
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        y = GetBattlerSpriteCoord(battler, 1u8);
        if !((IsContest()) != 0) {
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    species = ((GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                } else {
                    species = (((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read();
                }
            } else {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    species = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                } else {
                    species = (((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read();
                }
            }
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                y = ((((y) as i32).wrapping_sub(((GetBattlerElevation(battler, species)) as i32)))
                    as u8);
            }
        }
        return y;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAnimBattlerSpriteId(animBattler: u8) -> u8 {
    unsafe {
        let mut animBattler = animBattler;
        let mut sprites: *mut u8 = core::ptr::null_mut();
        if ((animBattler) as i32) == 0i32 {
            if (IsBattlerSpritePresent(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
                sprites = (&raw mut gBattlerSpriteIds).cast::<u8>();
                return ((sprites).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read();
            } else {
                return 255u8;
            }
        } else {
            if ((animBattler) as i32) == 1i32 {
                if (IsBattlerSpritePresent(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) != 0
                {
                    sprites = (&raw mut gBattlerSpriteIds).cast::<u8>();
                    return ((sprites).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read();
                } else {
                    return 255u8;
                }
            } else {
                if ((animBattler) as i32) == 2i32 {
                    if !((IsBattlerSpriteVisible(
                        ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                            as u8),
                    )) != 0)
                    {
                        return 255u8;
                    } else {
                        return (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read();
                    }
                } else {
                    if (IsBattlerSpriteVisible(
                        ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                            as u8),
                    )) != 0
                    {
                        return (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                                as isize,
                        ))
                        .read();
                    } else {
                        return 255u8;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StoreSpriteCallbackInData6(
    sprite: *mut u8,
    callback: Option<unsafe extern "C" fn(*mut u8)>,
) {
    unsafe {
        let mut sprite = sprite;
        let mut callback = callback;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write((((core::mem::transmute::<_, usize>(callback) as u32) & 65535u32) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write((((core::mem::transmute::<_, usize>(callback) as u32) >> 16) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCallbackToStoredInData6(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut callback: u32 = (((((((((sprite).wrapping_add(46)).cast::<i16>())
            .wrapping_offset(6))
        .read()) as u16) as i32)
            | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 16)) as u32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(
            (core::mem::transmute::<usize, Option<unsafe extern "C" fn(*mut u8)>>(
                (callback) as usize,
            )),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteInCircle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 256i32 {
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p4).write(((__p4).read()).wrapping_sub(1));
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteInGrowingCircle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >> 8)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >> 8)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
            ));
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 256i32 {
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p5).write(((__p5).read()).wrapping_sub(1));
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn TranslateSpriteInLissajousCurve(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 256i32 {
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >= 256i32
            {
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    < 0i32
                {
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p6).write((((((__p6).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p7).write(((__p7).read()).wrapping_sub(1));
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteInEllipse(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ));
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 256i32 {
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p4).write(((__p4).read()).wrapping_sub(1));
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitAnimForDuration(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPosToTranslateLinear(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ConvertPosDataToTranslateLinearData(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinear));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertPosDataToTranslateLinearData(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut old: i16 = 0i16;
        let mut xDiff: i32 = 0i32;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        xDiff = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
        old = (((sprite).wrapping_add(46)).cast::<i16>()).read();
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((if crate::c::div_i32(
                xDiff,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            ) < 0i32
            {
                (crate::c::div_i32(
                    xDiff,
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                ))
                .wrapping_neg()
            } else {
                crate::c::div_i32(
                    xDiff,
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                )
            }) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    .wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    ),
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(old);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteLinear(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteLinearFixedPoint(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn TranslateSpriteLinearFixedPointIconFrame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
        } else {
            SetCallbackToStoredInData6(sprite);
        }
        UpdateMonIconFrame(sprite);
    }
}
pub(crate) unsafe extern "C" fn TranslateSpriteToBattleTargetPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimPosToTranslateLinear));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteLinearById(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteLinearByIdFixedPoint(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateSpriteLinearAndFlicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            if crate::c::rem_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            ) == 0i32
            {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) != 0 {
                    crate::c::bf_write(
                        (sprite).wrapping_add(62),
                        2,
                        1,
                        ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16)
                            as i32)
                            ^ 1i32) as u16) as i32,
                    );
                }
            }
        } else {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroySpriteAndMatrix(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FreeSpriteOamMatrix(sprite);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn TranslateSpriteToBattleAttackerPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimPosToTranslateLinear));
    }
}
pub(crate) unsafe extern "C" fn EndUnkPaletteAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        PaletteStruct_ResetById(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u16),
        );
        DestroySpriteAndMatrix(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunStoredCallbackWhenAffineAnimEnds(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunStoredCallbackWhenAnimEnds(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSpriteAndDisableBlend(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimVisualTaskAndDisableBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteCoordsToAnimAttackerCoords(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetAnimSpriteInitialXOffset(sprite: *mut u8, xOffset: i16) {
    unsafe {
        let mut sprite = sprite;
        let mut xOffset = xOffset;
        let mut attackerX: u16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as u16);
        let mut targetX: u16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as u16);
        if ((attackerX) as i32) > ((targetX) as i32) {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(((xOffset) as i32))) as i16));
        } else {
            if ((attackerX) as i32) < ((targetX) as i32) {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(((xOffset) as i32))) as i16));
            } else {
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_sub(((xOffset) as i32))) as i16),
                    );
                } else {
                    let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(((xOffset) as i32))) as i16),
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimArcTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                32768i32,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateAnimHorizontalArc(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            return 1u8;
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((Sin(
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        >> 8) as u8) as i16),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32),
            )) as i16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateAnimVerticalArc(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            return 1u8;
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(36).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((Sin(
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        >> 8) as u8) as i16),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32),
            )) as i16),
        );
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpritePrimaryCoordsFromSecondaryCoords(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSpritePosToAnimTarget(sprite: *mut u8, respectMonPicOffsets: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets = respectMonPicOffsets;
        if !((respectMonPicOffsets) != 0) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        }
        SetAnimSpriteInitialXOffset(
            sprite,
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSpritePosToAnimAttacker(sprite: *mut u8, respectMonPicOffsets: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets = respectMonPicOffsets;
        if !((respectMonPicOffsets) != 0) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        }
        SetAnimSpriteInitialXOffset(
            sprite,
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSide(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        return (((((((&raw mut gBattlerPositions).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()) as i32)
            & 1i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerPosition(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        return (((&raw mut gBattlerPositions).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerAtPosition(position: u8) -> u8 {
    unsafe {
        let mut position = position;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gBattlerPositions).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((position) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return i;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattlerSpritePresent(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        if (IsContest()) != 0 {
            if ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) == ((battler) as i32)
            {
                return 1u8;
            } else {
                if ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                    == ((battler) as i32)
                {
                    return 1u8;
                } else {
                    return 0u8;
                }
            }
        } else {
            if (((((&raw mut gBattlerPositions).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32)
                == 255i32
            {
                return 0u8;
            } else {
                if ((GetBattlerSide(battler)) as i32) != 0i32 {
                    if GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        57i32,
                    ) != 0u32
                    {
                        return 1u8;
                    }
                } else {
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        57i32,
                    ) != 0u32
                    {
                        return 1u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsDoubleBattle() -> u8 {
    unsafe {
        return ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleAnimBg1Data(out: *mut u8) {
    unsafe {
        let mut out = out;
        if (IsContest()) != 0 {
            ((out).cast::<*mut u8>())
                .write(((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read());
            ((out).wrapping_add(4).cast::<*mut u16>()).write(
                (((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read()).cast::<u16>(),
            );
            ((out).wrapping_add(8)).write(14u8);
            ((out).wrapping_add(9)).write(1u8);
            ((out).wrapping_add(10).cast::<u16>()).write(0u16);
            ((out).wrapping_add(12).cast::<u16>()).write(0u16);
        } else {
            ((out).cast::<*mut u8>())
                .write(((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read());
            ((out).wrapping_add(4).cast::<*mut u16>()).write(
                (((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read()).cast::<u16>(),
            );
            ((out).wrapping_add(8)).write(8u8);
            ((out).wrapping_add(9)).write(1u8);
            ((out).wrapping_add(10).cast::<u16>()).write(512u16);
            ((out).wrapping_add(12).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleAnimBgData(out: *mut u8, bgId: u32) {
    unsafe {
        let mut out = out;
        let mut bgId = bgId;
        if (IsContest()) != 0 {
            ((out).cast::<*mut u8>())
                .write(((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read());
            ((out).wrapping_add(4).cast::<*mut u16>()).write(
                (((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read()).cast::<u16>(),
            );
            ((out).wrapping_add(8)).write(14u8);
            ((out).wrapping_add(9)).write(1u8);
            ((out).wrapping_add(10).cast::<u16>()).write(0u16);
            ((out).wrapping_add(12).cast::<u16>()).write(0u16);
        } else {
            if bgId == 1u32 {
                GetBattleAnimBg1Data(out);
            } else {
                ((out).cast::<*mut u8>())
                    .write(((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read());
                ((out).wrapping_add(4).cast::<*mut u16>()).write(
                    (((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read())
                        .cast::<u16>(),
                );
                ((out).wrapping_add(8)).write(9u8);
                ((out).wrapping_add(9)).write(2u8);
                ((out).wrapping_add(10).cast::<u16>()).write(768u16);
                ((out).wrapping_add(12).cast::<u16>()).write(0u16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBgDataForTransform(out: *mut u8, battler: u8) {
    unsafe {
        let mut out = out;
        let mut battler = battler;
        ((out).cast::<*mut u8>())
            .write(((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read());
        ((out).wrapping_add(4).cast::<*mut u16>()).write(
            (((&raw mut gBattleAnimBgTilemapBuffer).cast::<*mut u8>()).read()).cast::<u16>(),
        );
        if (IsContest()) != 0 {
            ((out).wrapping_add(8)).write(14u8);
            ((out).wrapping_add(9)).write(1u8);
            ((out).wrapping_add(10).cast::<u16>()).write(0u16);
            ((out).wrapping_add(12).cast::<u16>()).write(0u16);
        } else {
            if ((GetBattlerSpriteBGPriorityRank(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            )) as i32)
                == 1i32
            {
                ((out).wrapping_add(8)).write(8u8);
                ((out).wrapping_add(9)).write(1u8);
                ((out).wrapping_add(10).cast::<u16>()).write(512u16);
                ((out).wrapping_add(12).cast::<u16>()).write(0u16);
            } else {
                ((out).wrapping_add(8)).write(9u8);
                ((out).wrapping_add(9)).write(2u8);
                ((out).wrapping_add(10).cast::<u16>()).write(768u16);
                ((out).wrapping_add(12).cast::<u16>()).write(0u16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBattleAnimBg(bgId: u32) {
    unsafe {
        let mut bgId = bgId;
        let mut bgAnimData = crate::ffi::Align4([0u8; 16]);
        GetBattleAnimBgData((&raw mut bgAnimData).cast::<u8>(), bgId);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((&raw mut bgAnimData).cast::<u8>()).cast::<*mut u8>()).read(),
                                ((83886080i32
                                    | (crate::c::div_i32(8192i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        LoadBgTiles(
            (((&raw mut bgAnimData).cast::<u8>()).wrapping_add(9)).read(),
            (((&raw mut bgAnimData).cast::<u8>()).cast::<*mut u8>()).read(),
            8192u16,
            (((&raw mut bgAnimData).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read(),
        );
        FillBgTilemapBufferRect(
            (((&raw mut bgAnimData).cast::<u8>()).wrapping_add(9)).read(),
            0u16,
            0u8,
            0u8,
            32u8,
            64u8,
            17u8,
        );
        CopyBgTilemapBufferToVram((((&raw mut bgAnimData).cast::<u8>()).wrapping_add(9)).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimLoadCompressedBgGfx(bgId: u32, src: *mut u32, tilesOffset: u32) {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        let mut tilesOffset = tilesOffset;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read(),
                                ((83886080i32
                                    | (crate::c::div_i32(8192i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            src,
            ((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read(),
        );
        LoadBgTiles(
            ((bgId) as u8),
            ((&raw mut gBattleAnimBgTileBuffer).cast::<*mut u8>()).read(),
            8192u16,
            ((tilesOffset) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn InitAnimBgTilemapBuffer(bgId: u32, src: *mut u8) {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        FillBgTilemapBufferRect(((bgId) as u8), 0u16, 0u8, 0u8, 32u8, 64u8, 17u8);
        CopyToBgTilemapBuffer(((bgId) as u8), src, 0u16, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimLoadCompressedBgTilemap(bgId: u32, src: *mut u8) {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        InitAnimBgTilemapBuffer(bgId, src);
        CopyBgTilemapBufferToVram(((bgId) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimLoadCompressedBgTilemapHandleContest(
    data: *mut u8,
    src: *mut u8,
    largeScreen: u32,
) {
    unsafe {
        let mut data = data;
        let mut src = src;
        let mut largeScreen = largeScreen;
        InitAnimBgTilemapBuffer(((((data).wrapping_add(9)).read()) as u32), src);
        if ((IsContest()) as i32) == 1i32 {
            RelocateBattleBgPal(
                ((((data).wrapping_add(8)).read()) as u16),
                ((data).wrapping_add(4).cast::<*mut u16>()).read(),
                0u32,
                ((largeScreen) as u8),
            );
        }
        CopyBgTilemapBufferToVram(((data).wrapping_add(9)).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleBgPaletteNum() -> u8 {
    unsafe {
        if (IsContest()) != 0 {
            return 1u8;
        } else {
            return 2u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateAnimBg3ScreenSize(largeScreenSize: u8) {
    unsafe {
        let mut largeScreenSize = largeScreenSize;
        if (!((largeScreenSize) != 0)) || ((IsContest()) != 0) {
            SetAnimBgAttribute(3u8, 0u8, 0u8);
            SetAnimBgAttribute(3u8, 1u8, 1u8);
        } else {
            SetAnimBgAttribute(3u8, 0u8, 1u8);
            SetAnimBgAttribute(3u8, 1u8, 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Trade_MoveSelectedMonToTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitSpriteDataForLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPointIconFrame));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSpriteDataForLinearTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )
            << 8) as i16);
        let mut y: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )
            << 8) as i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((crate::c::div_i32(
                ((x) as i32),
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((crate::c::div_i32(
                ((y) as i32),
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimLinearTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
        let mut y: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
            as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            );
        let mut movingLeft: u8 = ((x < 0i32) as u8);
        let mut movingUp: u8 = ((y < 0i32) as u8);
        let mut xDelta: u16 = (((if x < 0i32 { (x).wrapping_neg() } else { x }) << 8) as u16);
        let mut yDelta: u16 = (((if y < 0i32 { (y).wrapping_neg() } else { y }) << 8) as u16);
        xDelta = ((crate::c::div_i32(
            ((xDelta) as i32),
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
        )) as u16);
        yDelta = ((crate::c::div_i32(
            ((yDelta) as i32),
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
        )) as u16);
        if (movingLeft) != 0 {
            xDelta = ((((xDelta) as i32) | 1i32) as u16);
        } else {
            xDelta = ((((xDelta) as i32) & (-2i32)) as u16);
        }
        if (movingUp) != 0 {
            yDelta = ((((yDelta) as i32) | 1i32) as u16);
        } else {
            yDelta = ((((yDelta) as i32) & (-2i32)) as u16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(((xDelta) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((yDelta) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartAnimLinearTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateLinear_WithFollowup));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn StartAnimLinearTranslation_SetCornerVecX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateLinear_WithFollowup_SetCornerVecX));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTranslateLinear(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut v1: u16 = 0u16;
        let mut v2: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            return 1u8;
        }
        v1 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        v2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
        x = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
        y = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16);
        x = ((((x) as i32).wrapping_add(((v1) as i32))) as u16);
        y = ((((y) as i32).wrapping_add(((v2) as i32))) as u16);
        if (((v1) as i32) & 1i32) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>())
                .write((((((x) as i32) >> 8).wrapping_neg()) as i16));
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(((((x) as i32) >> 8) as i16));
        }
        if (((v2) as i32) & 1i32) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>())
                .write((((((y) as i32) >> 8).wrapping_neg()) as i16));
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(((((y) as i32) >> 8) as i16));
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(((x) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(((y) as i16));
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTranslateLinear_WithFollowup(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTranslateLinear_WithFollowup_SetCornerVecX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        AnimSetCenterToCornerVecX(sprite);
        if (AnimTranslateLinear(sprite)) != 0 {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimLinearTranslationWithSpeed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut v1: i32 = ((if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )
            < 0i32
        {
            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ))
            .wrapping_neg()
        } else {
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )
        }) << 8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((crate::c::div_i32(
                v1,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        InitAnimLinearTranslation(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimLinearTranslationWithSpeedAndPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimLinearTranslationWithSpeed(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateLinear_WithFollowup));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn InitAnimFastLinearTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xDiff: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            );
        let mut yDiff: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            );
        let mut x_sign: u8 = ((xDiff < 0i32) as u8);
        let mut y_sign: u8 = ((yDiff < 0i32) as u8);
        let mut x2: u16 = (((if xDiff < 0i32 {
            (xDiff).wrapping_neg()
        } else {
            xDiff
        }) << 4) as u16);
        let mut y2: u16 = (((if yDiff < 0i32 {
            (yDiff).wrapping_neg()
        } else {
            yDiff
        }) << 4) as u16);
        x2 = ((crate::c::div_i32(
            ((x2) as i32),
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
        )) as u16);
        y2 = ((crate::c::div_i32(
            ((y2) as i32),
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
        )) as u16);
        if (x_sign) != 0 {
            x2 = ((((x2) as i32) | 1i32) as u16);
        } else {
            x2 = ((((x2) as i32) & (-2i32)) as u16);
        }
        if (y_sign) != 0 {
            y2 = ((((y2) as i32) | 1i32) as u16);
        } else {
            y2 = ((((y2) as i32) & (-2i32)) as u16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(((x2) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((y2) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAndRunAnimFastLinearTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimFastLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFastTranslateLinearWaitEnd));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimFastTranslateLinear(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut v1: u16 = 0u16;
        let mut v2: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            return 1u8;
        }
        v1 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        v2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16);
        x = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
        y = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16);
        x = ((((x) as i32).wrapping_add(((v1) as i32))) as u16);
        y = ((((y) as i32).wrapping_add(((v2) as i32))) as u16);
        if (((v1) as i32) & 1i32) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>())
                .write((((((x) as i32) >> 4).wrapping_neg()) as i16));
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(((((x) as i32) >> 4) as i16));
        }
        if (((v2) as i32) & 1i32) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>())
                .write((((((y) as i32) >> 4).wrapping_neg()) as i16));
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(((((y) as i32) >> 4) as i16));
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(((x) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(((y) as i16));
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AnimFastTranslateLinearWaitEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimFastTranslateLinear(sprite)) != 0 {
            SetCallbackToStoredInData6(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimFastLinearTranslationWithSpeed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xDiff: i32 = ((if ((((((sprite).wrapping_add(46)).cast::<i16>())
            .wrapping_offset(2))
        .read()) as i32)
            .wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )
            < 0i32
        {
            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ))
            .wrapping_neg()
        } else {
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )
        }) << 4);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((crate::c::div_i32(
                xDiff,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        InitAnimFastLinearTranslation(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitAnimFastLinearTranslationWithSpeedAndPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimFastLinearTranslationWithSpeed(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFastTranslateLinearWaitEnd));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteRotScale(spriteId: u8, xScale: i16, yScale: i16, rotation: u16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut xScale = xScale;
        let mut yScale = yScale;
        let mut rotation = rotation;
        let mut i: i32 = 0i32;
        let mut src = crate::ffi::Align4([0u8; 8]);
        let mut matrix = crate::ffi::Align4([0u8; 8]);
        (((&raw mut src).cast::<u8>()).cast::<i16>()).write(xScale);
        (((&raw mut src).cast::<u8>()).wrapping_add(2).cast::<i16>()).write(yScale);
        (((&raw mut src).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(rotation);
        if (ShouldRotScaleSpeciesBeFlipped()) != 0 {
            (((&raw mut src).cast::<u8>()).cast::<i16>()).write(
                (((((((&raw mut src).cast::<u8>()).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        i = ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            false,
        ) as u32) as i32);
        ObjAffineSet(
            (&raw mut src).cast::<u8>(),
            (&raw mut matrix).cast::<u8>(),
            1i32,
            2i32,
        );
        ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8)).cast::<i16>())
            .write((((&raw mut matrix).cast::<u8>()).cast::<i16>()).read());
        ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
            .wrapping_add(2)
            .cast::<i16>())
        .write(
            (((&raw mut matrix).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read(),
        );
        ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
            .wrapping_add(4)
            .cast::<i16>())
        .write(
            (((&raw mut matrix).cast::<u8>())
                .wrapping_add(4)
                .cast::<i16>())
            .read(),
        );
        ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
            .wrapping_add(6)
            .cast::<i16>())
        .write(
            (((&raw mut matrix).cast::<u8>())
                .wrapping_add(6)
                .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ShouldRotScaleSpeciesBeFlipped() -> u8 {
    unsafe {
        if (IsContest()) != 0 {
            if ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                == 201i32
            {
                return 0u8;
            } else {
                return 1u8;
            }
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepareBattlerSpriteForRotScale(spriteId: u8, objMode: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut objMode = objMode;
        let mut battler: u8 = (((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as u8);
        if ((IsContest()) != 0) || ((IsBattlerSpriteVisible(battler)) != 0) {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            2,
            2,
            ((objMode) as u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            7,
            1,
            (1u8) as i32,
        );
        if (!((IsContest()) != 0))
            && (!((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32)
                != 0))
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(6))
                .read()) as u32) as i32,
            );
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSpriteRotScale(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        SetSpriteRotScale(spriteId, 256i16, 256i16, 0u16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            2,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            7,
            1,
            (0u8) as i32,
        );
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerSpriteYOffsetFromRotation(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut matrixNum: u16 = ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            false,
        ) as u32) as u16);
        let mut c: i16 = ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i16>())
        .read();
        if ((c) as i32) < 0i32 {
            c = ((((c) as i32).wrapping_neg()) as i16);
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write(((((c) as i32) >> 3) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetSpriteRotScale(
    sprite: *mut u8,
    recalcCenterVector: u8,
    xScale: i16,
    yScale: i16,
    rotation: u16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut recalcCenterVector = recalcCenterVector;
        let mut xScale = xScale;
        let mut yScale = yScale;
        let mut rotation = rotation;
        let mut i: i32 = 0i32;
        let mut src = crate::ffi::Align4([0u8; 8]);
        let mut matrix = crate::ffi::Align4([0u8; 8]);
        if ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) & 1u32) != 0 {
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
            if (recalcCenterVector) != 0 {
                CalcCenterToCornerVec(
                    sprite,
                    ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
                    ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
                    ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) as u8),
                );
            }
            (((&raw mut src).cast::<u8>()).cast::<i16>()).write(xScale);
            (((&raw mut src).cast::<u8>()).wrapping_add(2).cast::<i16>()).write(yScale);
            (((&raw mut src).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(rotation);
            if (ShouldRotScaleSpeciesBeFlipped()) != 0 {
                (((&raw mut src).cast::<u8>()).cast::<i16>()).write(
                    (((((((&raw mut src).cast::<u8>()).cast::<i16>()).read()) as i32)
                        .wrapping_neg()) as i16),
                );
            }
            i = ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as i32);
            ObjAffineSet(
                (&raw mut src).cast::<u8>(),
                (&raw mut matrix).cast::<u8>(),
                1i32,
                2i32,
            );
            ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
                .cast::<i16>())
            .write((((&raw mut matrix).cast::<u8>()).cast::<i16>()).read());
            ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
                .wrapping_add(2)
                .cast::<i16>())
            .write(
                (((&raw mut matrix).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read(),
            );
            ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
                .wrapping_add(4)
                .cast::<i16>())
            .write(
                (((&raw mut matrix).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i16>())
                .read(),
            );
            ((((&raw mut gOamMatrices).cast::<u8>()).wrapping_offset((i) as isize * 8))
                .wrapping_add(6)
                .cast::<i16>())
            .write(
                (((&raw mut matrix).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<i16>())
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSpriteRotScale_PreserveAffine(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TrySetSpriteRotScale(sprite, 1u8, 256i16, 256i16, 0u16);
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
        CalcCenterToCornerVec(
            sprite,
            ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn ArcTan2_(x: i16, y: i16) -> u16 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return ArcTan2(x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ArcTan2Neg(x: i16, y: i16) -> u16 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut var: u16 = ArcTan2_(x, y);
        return ((((var) as i32).wrapping_neg()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGrayscaleOrOriginalPalette(paletteNum: u16, restoreOriginalColor: u8) {
    unsafe {
        let mut paletteNum = paletteNum;
        let mut restoreOriginalColor = restoreOriginalColor;
        let mut i: i32 = 0i32;
        let mut originalColor: *mut u8 = core::ptr::null_mut();
        let mut destColor: *mut u8 = core::ptr::null_mut();
        let mut average: u16 = 0u16;
        let mut paletteOffset: u16 = ((((paletteNum) as i32).wrapping_mul(16i32)) as u16);
        if !((restoreOriginalColor) != 0) {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        originalColor = ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset((((paletteOffset) as i32).wrapping_add(i)) as isize))
                        .cast::<u8>();
                        average =
                            (((((crate::c::bf_read((originalColor).wrapping_add(0), 0, 5, false)
                                as u16) as i32)
                                .wrapping_add(
                                    ((crate::c::bf_read(
                                        (originalColor).wrapping_add(0),
                                        5,
                                        5,
                                        false,
                                    ) as u16) as i32),
                                ))
                            .wrapping_add(
                                ((crate::c::bf_read((originalColor).wrapping_add(1), 2, 5, false)
                                    as u16) as i32),
                            )) as u16);
                        average = ((crate::c::div_i32(((average) as i32), 3i32)) as u16);
                        destColor = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset((((paletteOffset) as i32).wrapping_add(i)) as isize))
                        .cast::<u8>();
                        crate::c::bf_write((destColor).wrapping_add(0), 0, 5, (average) as i32);
                        crate::c::bf_write((destColor).wrapping_add(0), 5, 5, (average) as i32);
                        crate::c::bf_write((destColor).wrapping_add(1), 2, 5, (average) as i32);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            'l3: loop {
                'l4: {
                    'l5: loop {
                        'l6: {
                            CpuSet(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((paletteOffset) as i32) as isize))
                                .cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
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
                            break 'l5;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l3;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePalettesMask(
    battleBackground: u8,
    attacker: u8,
    target: u8,
    attackerPartner: u8,
    targetPartner: u8,
    anim1: u8,
    anim2: u8,
) -> u32 {
    unsafe {
        let mut battleBackground = battleBackground;
        let mut attacker = attacker;
        let mut target = target;
        let mut attackerPartner = attackerPartner;
        let mut targetPartner = targetPartner;
        let mut anim1 = anim1;
        let mut anim2 = anim2;
        let mut selectedPalettes: u32 = 0u32;
        let mut shift: u32 = 0u32;
        if (battleBackground) != 0 {
            if !((IsContest()) != 0) {
                selectedPalettes = 14u32;
            } else {
                selectedPalettes =
                    ((crate::c::shl_i32(1i32, ((GetBattleBgPaletteNum()) as u32))) as u32);
            }
        }
        if (attacker) != 0 {
            shift = ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                .wrapping_add(16i32)) as u32);
            selectedPalettes = (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
        }
        if (target) != 0 {
            shift = ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                .wrapping_add(16i32)) as u32);
            selectedPalettes = (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
        }
        if (attackerPartner) != 0 {
            if (IsBattlerSpriteVisible(
                ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
            )) != 0
            {
                shift = (((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                    .wrapping_add(16i32)) as u32);
                selectedPalettes = (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
            }
        }
        if (targetPartner) != 0 {
            if (IsBattlerSpriteVisible(
                ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
            )) != 0
            {
                shift = (((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                    .wrapping_add(16i32)) as u32);
                selectedPalettes = (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
            }
        }
        if (anim1) != 0 {
            if !((IsContest()) != 0) {
                selectedPalettes = (selectedPalettes | 256u32);
            } else {
                selectedPalettes = (selectedPalettes | 16384u32);
            }
        }
        if (anim2) != 0 {
            if !((IsContest()) != 0) {
                selectedPalettes = (selectedPalettes | 512u32);
            }
        }
        return selectedPalettes;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleMonSpritePalettesMask(
    playerLeft: u8,
    playerRight: u8,
    opponentLeft: u8,
    opponentRight: u8,
) -> u32 {
    unsafe {
        let mut playerLeft = playerLeft;
        let mut playerRight = playerRight;
        let mut opponentLeft = opponentLeft;
        let mut opponentRight = opponentRight;
        let mut selectedPalettes: u32 = 0u32;
        let mut shift: u32 = 0u32;
        if (IsContest()) != 0 {
            if (playerLeft) != 0 {
                selectedPalettes = (selectedPalettes | 262144u32);
                return selectedPalettes;
            }
        } else {
            if (playerLeft) != 0 {
                if (IsBattlerSpriteVisible(GetBattlerAtPosition(0u8))) != 0 {
                    selectedPalettes = (selectedPalettes
                        | ((crate::c::shl_i32(
                            1i32,
                            ((((GetBattlerAtPosition(0u8)) as i32).wrapping_add(16i32)) as u32),
                        )) as u32));
                }
            }
            if (playerRight) != 0 {
                if (IsBattlerSpriteVisible(GetBattlerAtPosition(2u8))) != 0 {
                    shift = ((((GetBattlerAtPosition(2u8)) as i32).wrapping_add(16i32)) as u32);
                    selectedPalettes =
                        (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
                }
            }
            if (opponentLeft) != 0 {
                if (IsBattlerSpriteVisible(GetBattlerAtPosition(1u8))) != 0 {
                    shift = ((((GetBattlerAtPosition(1u8)) as i32).wrapping_add(16i32)) as u32);
                    selectedPalettes =
                        (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
                }
            }
            if (opponentRight) != 0 {
                if (IsBattlerSpriteVisible(GetBattlerAtPosition(3u8))) != 0 {
                    shift = ((((GetBattlerAtPosition(3u8)) as i32).wrapping_add(16i32)) as u32);
                    selectedPalettes =
                        (selectedPalettes | ((crate::c::shl_i32(1i32, shift)) as u32));
                }
            }
        }
        return selectedPalettes;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpritePalIdxByBattler(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        return battler;
    }
}
pub(crate) unsafe extern "C" fn GetSpritePalIdxByPosition(position: u8) -> u8 {
    unsafe {
        let mut position = position;
        return GetBattlerAtPosition(position);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSpriteOnMonPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets: u8 = 0u8;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read())
                != 0)
            {
                respectMonPicOffsets = 1u8;
            } else {
                respectMonPicOffsets = 0u8;
            }
            if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read())
                != 0)
            {
                InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
            } else {
                InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
            }
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
                || ((crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0)
            {
                DestroySpriteAndMatrix(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TranslateAnimSpriteToTargetMonLocation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets: u8 = 0u8;
        let mut coordType: u8 = 0u8;
        if !((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            & 65280i32)
            != 0)
        {
            respectMonPicOffsets = 1u8;
        } else {
            respectMonPicOffsets = 0u8;
        }
        if !((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            & 255i32)
            != 0)
        {
            coordType = 3u8;
        } else {
            coordType = 1u8;
        }
        InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                coordType,
            )) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimThrowProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimThrowProjectile_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimThrowProjectile_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTravelDiagonally(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut coordType: u8 = 0u8;
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
            .read())
            != 0)
        {
            respectMonPicOffsets = 1u8;
            coordType = 3u8;
        } else {
            respectMonPicOffsets = 0u8;
            coordType = 1u8;
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(battler, coordType)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloneBattlerSpriteWithBlend(animBattler: u8) -> i16 {
    unsafe {
        let mut animBattler = animBattler;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(animBattler);
        if ((spriteId) as i32) != 255i32 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 64i32) {
                        break 'l1;
                    }
                    'l2: {
                        if !((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                            .wrapping_add(62),
                            0,
                            1,
                            false,
                        ) as u16)
                            != 0)
                        {
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68)
                                .cast::<crate::c::Rec4<68>>()
                                .write_unaligned(
                                    ((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68)
                                        .cast::<crate::c::Rec4<68>>()
                                        .read_unaligned(),
                                );
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(1),
                                2,
                                2,
                                (1u32) as i32,
                            );
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(62),
                                2,
                                1,
                                (0u16) as i32,
                            );
                            return ((i) as i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return (-1i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroySpriteWithActiveSheet(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(63), 6, 1, (1u16) as i32);
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AlphaFadeIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut v1: i16 = 0i16;
        let mut v2: i16 = 0i16;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            > (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
        {
            v2 = 1i16;
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            < (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
        {
            v2 = (-1i16);
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            > ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
        {
            v1 = 1i16;
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            < ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
        {
            v1 = (-1i16);
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(v2);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(v1);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        SetGpuReg(
            82u8,
            (((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                << 8)
                | (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32))
                as u16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_AlphaFadeIn_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AlphaFadeIn_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (({
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            if ((({
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                & 1i32)
                != 0
            {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                {
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p5).write(
                        (((((__p5).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32),
                        )) as i16),
                    );
                }
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    != ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                {
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p6).write(
                        (((((__p6).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                                as i32),
                        )) as i16),
                    );
                }
            }
            SetGpuReg(
                82u8,
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    << 8)
                    | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)) as u16),
            );
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32))
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32))
            {
                DestroyAnimVisualTask(taskId);
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendMonInAndOut(task: u8) {
    unsafe {
        let mut task = task;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        if ((spriteId) as i32) == 255i32 {
            DestroyAnimVisualTask(task);
            return;
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((256i32).wrapping_add(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    4,
                    4,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(16i32),
            ))
            .wrapping_add(1i32)) as i16),
        );
        AnimTask_BlendPalInAndOutSetup(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendPalInAndOutSetup(task: *mut u8) {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_BlendMonInAndOut_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendMonInAndOut_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0) {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                BlendPalette(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u16),
                    15u16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(1i16);
                }
            } else {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_sub(1));
                BlendPalette(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u16),
                    15u16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
                );
                if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) != 0) {
                    if ({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                        let __t6 = ((__p5).read()).wrapping_sub(1);
                        (__p5).write(__t6);
                        __t6
                    }) != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                    } else {
                        DestroyAnimVisualTask(taskId);
                        return;
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendPalInAndOutByTag(task: u8) {
    unsafe {
        let mut task = task;
        let mut palette: u8 = IndexOfSpritePaletteTag(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16),
        );
        if ((palette) as i32) == 255i32 {
            DestroyAnimVisualTask(task);
            return;
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((((((palette) as i32).wrapping_mul(16i32)).wrapping_add(257i32)) as i16));
        AnimTask_BlendPalInAndOutSetup(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepareAffineAnimInTaskData(
    task: *mut u8,
    spriteId: u8,
    affineAnimCmds: *mut u8,
) {
    unsafe {
        let mut task = task;
        let mut spriteId = spriteId;
        let mut affineAnimCmds = affineAnimCmds;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(((spriteId) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(256i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(256i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
        StorePointerInVars(
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13),
            (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14),
            affineAnimCmds,
        );
        PrepareBattlerSpriteForRotScale(spriteId, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunAffineAnimFromTaskData(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((&raw mut sAnimTaskAffineAnim)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(
            (LoadPointerFromVars(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
            ))
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 8,
            ),
        );
        'l1: {
            let __sw1 = ((((((&raw mut sAnimTaskAffineAnim)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 32766i32 || __sw1 == 32765i32 || __sw1 == 32767i32;
            if !__matched {
                if !((((((&raw mut sAnimTaskAffineAnim)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(5))
                .read())
                    != 0)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
                        ((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<i16>())
                        .read(),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                        ((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                        ((((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .read()) as i16),
                    );
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    let __p3 = (&raw mut sAnimTaskAffineAnim)
                        .cast::<u8>()
                        .cast::<*mut u8>();
                    (__p3).write(((__p3).read()).wrapping_offset(8));
                }
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((&raw mut sAnimTaskAffineAnim)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .read()) as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read(),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                        as u16),
                );
                SetBattlerSpriteYOffsetFromYScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    >= ((((((&raw mut sAnimTaskAffineAnim)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(5))
                    .read()) as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 32766i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
                    ((((((&raw mut sAnimTaskAffineAnim)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i16),
                );
                break 'l1;
            }
            if __sw1 == 32765i32 {
                if (((((&raw mut sAnimTaskAffineAnim)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<i16>())
                .read())
                    != 0
                {
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) != 0 {
                        if !(({
                            let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                            let __t11 = ((__p10).read()).wrapping_sub(1);
                            (__p10).write(__t11);
                            __t11
                        }) != 0)
                        {
                            let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                            (__p12).write(((__p12).read()).wrapping_add(1));
                            break 'l1;
                        }
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
                            ((((&raw mut sAnimTaskAffineAnim)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read(),
                        );
                    }
                    if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        != 0)
                    {
                        break 'l1;
                    }
                    {
                        'l2: loop {
                            'l3: {
                                let __p13 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                                (__p13).write(((__p13).read()).wrapping_sub(1));
                                let __p14 = (&raw mut sAnimTaskAffineAnim)
                                    .cast::<u8>()
                                    .cast::<*mut u8>();
                                (__p14).write(((__p14).read()).wrapping_offset(-8));
                                if ((((((&raw mut sAnimTaskAffineAnim)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<i16>())
                                .read()) as i32)
                                    == 32765i32
                                {
                                    let __p15 =
                                        (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                                    (__p15).write(((__p15).read()).wrapping_add(1));
                                    return 1u8;
                                }
                                if !((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(7))
                                .read())
                                    != 0)
                                {
                                    return 1u8;
                                }
                            }
                        }
                    }
                }
                let __p16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 32767i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ResetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerSpriteYOffsetFromYScale(spriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut var: i32 = (64i32)
            .wrapping_sub(((GetBattlerYDeltaFromSpriteId(spriteId)) as i32).wrapping_mul(2i32));
        let mut matrix: u16 = ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            false,
        ) as u32) as u16);
        let mut var2: i32 = (if ((((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrix) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            crate::c::div_i32(
                (var << 8),
                ((((((&raw mut gOamMatrices).cast::<u8>())
                    .wrapping_offset(((matrix) as i32) as isize * 8))
                .wrapping_add(6)
                .cast::<i16>())
                .read()) as i32),
            )
        } else {
            0i32
        });
        if var2 > 128i32 {
            var2 = 128i32;
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write(((crate::c::div_i32((var).wrapping_sub(var2), 2i32)) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattlerSpriteYOffsetFromOtherYScale(spriteId: u8, otherSpriteId: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut otherSpriteId = otherSpriteId;
        let mut var: i32 = (64i32).wrapping_sub(
            ((GetBattlerYDeltaFromSpriteId(otherSpriteId)) as i32).wrapping_mul(2i32),
        );
        let mut matrix: u16 = ((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            false,
        ) as u32) as u16);
        let mut var2: i32 = (if ((((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrix) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<i16>())
        .read()) as i32)
            != 0i32
        {
            crate::c::div_i32(
                (var << 8),
                ((((((&raw mut gOamMatrices).cast::<u8>())
                    .wrapping_offset(((matrix) as i32) as isize * 8))
                .wrapping_add(6)
                .cast::<i16>())
                .read()) as i32),
            )
        } else {
            0i32
        });
        if var2 > 128i32 {
            var2 = 128i32;
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write(((crate::c::div_i32((var).wrapping_sub(var2), 2i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn GetBattlerYDeltaFromSpriteId(spriteId: u8) -> u16 {
    unsafe {
        let mut spriteId = spriteId;
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        let mut battler: u8 = (((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .read()) as u8);
        let mut species: u16 = 0u16;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((spriteId) as i32)
                    {
                        if (IsContest()) != 0 {
                            species = ((((((&raw mut gContestResources).cast::<*mut u8>())
                                .read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                            .read())
                            .cast::<u16>())
                            .read();
                            return ((((((&raw mut gMonBackPicCoords).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 4))
                            .wrapping_add(1))
                            .read()) as u16);
                        } else {
                            if ((GetBattlerSide(((i) as u8))) as i32) == 0i32 {
                                spriteInfo =
                                    ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                        .cast::<*mut u8>())
                                    .read();
                                if !(((((spriteInfo)
                                    .wrapping_offset(((battler) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read())
                                    != 0)
                                {
                                    species = ((GetMonData2(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        11i32,
                                    )) as u16);
                                } else {
                                    species = (((spriteInfo)
                                        .wrapping_offset(((battler) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read();
                                }
                                if ((species) as i32) == 385i32 {
                                    return ((((((&raw const sCastformBackSpriteYCoords)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattleMonForms).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as u16);
                                } else {
                                    return ((((((&raw mut gMonBackPicCoords).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 4))
                                    .wrapping_add(1))
                                    .read()) as u16);
                                }
                            } else {
                                spriteInfo =
                                    ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                        .cast::<*mut u8>())
                                    .read();
                                if !(((((spriteInfo)
                                    .wrapping_offset(((battler) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read())
                                    != 0)
                                {
                                    species = ((GetMonData2(
                                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        11i32,
                                    )) as u16);
                                } else {
                                    species = (((spriteInfo)
                                        .wrapping_offset(((battler) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read();
                                }
                                if ((species) as i32) == 385i32 {
                                    return ((((((&raw const sCastformElevations)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gBattleMonForms).cast::<u8>())
                                            .wrapping_offset(((battler) as i32) as isize))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as u16);
                                } else {
                                    return ((((((&raw mut gMonFrontPicCoords).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 4))
                                    .wrapping_add(1))
                                    .read()) as u16);
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 64u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorePointerInVars(lo: *mut i16, hi: *mut i16, ptr: *mut u8) {
    unsafe {
        let mut lo = lo;
        let mut hi = hi;
        let mut ptr = ptr;
        (lo).write(((((ptr) as usize as i32) & 65535i32) as i16));
        (hi).write((((((ptr) as usize as i32) >> 16) & 65535i32) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadPointerFromVars(lo: i16, hi: i16) -> *mut u8 {
    unsafe {
        let mut lo = lo;
        let mut hi = hi;
        return (((((lo) as u16) as i32) | ((((hi) as u16) as i32) << 16)) as usize as *mut u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepareEruptAnimTaskData(
    task: *mut u8,
    spriteId: u8,
    xScaleStart: i16,
    yScaleStart: i16,
    xScaleEnd: i16,
    yScaleEnd: i16,
    duration: u16,
) {
    unsafe {
        let mut task = task;
        let mut spriteId = spriteId;
        let mut xScaleStart = xScaleStart;
        let mut yScaleStart = yScaleStart;
        let mut xScaleEnd = xScaleEnd;
        let mut yScaleEnd = yScaleEnd;
        let mut duration = duration;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(((duration) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(((spriteId) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(xScaleStart);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(yScaleStart);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(xScaleEnd);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(yScaleEnd);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
            ((crate::c::div_i32(
                ((xScaleEnd) as i32).wrapping_sub(((xScaleStart) as i32)),
                ((duration) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            ((crate::c::div_i32(
                ((yScaleEnd) as i32).wrapping_sub(((yScaleStart) as i32)),
                ((duration) as i32),
            )) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateEruptAnimTask(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) != 0) {
            return 0u8;
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            != 0i32
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                        as i32),
                )) as i16),
            );
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                        as i32),
                )) as i16),
            );
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read());
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read());
        }
        SetSpriteRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read(),
            0u16,
        );
        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) != 0 {
            SetBattlerSpriteYOffsetFromYScale(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
        }
        return ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetFrustrationPowerLevel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut powerLevel: u16 = 0u16;
        if ((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) <= 30i32 {
            powerLevel = 0u16;
        } else {
            if ((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) <= 100i32 {
                powerLevel = 1u16;
            } else {
                if ((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) <= 200i32 {
                    powerLevel = 2u16;
                } else {
                    powerLevel = 3u16;
                }
            }
        }
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .write(((powerLevel) as i16));
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetPriorityForVisibleBattlers(priority: u8) {
    unsafe {
        let mut priority = priority;
        if (IsBattlerSpriteVisible(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
        }
        if (IsBattlerSpriteVisible(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
        }
        if (IsBattlerSpriteVisible(
            ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
        )) != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                            as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
        }
        if (IsBattlerSpriteVisible(
            ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
        )) != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                            as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPrioritiesForVisibleBattlers() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (IsBattlerSpriteVisible(((i) as u8))) != 0 {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(67))
                        .write(GetBattlerSpriteSubpriority(((i) as u8)));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (2u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteSubpriority(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut position: u8 = 0u8;
        let mut subpriority: u8 = 0u8;
        if (IsContest()) != 0 {
            if ((battler) as i32) == 2i32 {
                return 30u8;
            } else {
                return 40u8;
            }
        } else {
            position = GetBattlerPosition(battler);
            if ((position) as i32) == 0i32 {
                subpriority = 30u8;
            } else {
                if ((position) as i32) == 2i32 {
                    subpriority = 20u8;
                } else {
                    if ((position) as i32) == 1i32 {
                        subpriority = 40u8;
                    } else {
                        subpriority = 50u8;
                    }
                }
            }
        }
        return subpriority;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteBGPriority(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut position: u8 = GetBattlerPosition(battler);
        if (IsContest()) != 0 {
            return 2u8;
        } else {
            if (((position) as i32) == 0i32) || (((position) as i32) == 3i32) {
                return ((GetAnimBgAttribute(2u8, 4u8)) as u8);
            } else {
                return ((GetAnimBgAttribute(1u8, 4u8)) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteBGPriorityRank(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        if !((IsContest()) != 0) {
            let mut position: u8 = GetBattlerPosition(battler);
            if (((position) as i32) == 0i32) || (((position) as i32) == 3i32) {
                return 2u8;
            } else {
                return 1u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateAdditionalMonSpriteForMoveAnim(
    species: u16,
    isBackpic: u8,
    id: u8,
    x: i16,
    y: i16,
    subpriority: u8,
    personality: u32,
    trainerId: u32,
    battler: u32,
    ignoreDeoxysForm: u32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut isBackpic = isBackpic;
        let mut id = id;
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        let mut personality = personality;
        let mut trainerId = trainerId;
        let mut battler = battler;
        let mut ignoreDeoxysForm = ignoreDeoxysForm;
        let mut spriteId: u8 = 0u8;
        let mut sheet: u16 = LoadSpriteSheet(
            (((&raw const sSpriteSheets_MoveEffectMons)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 8),
        );
        let mut palette: u16 = ((AllocSpritePalette(
            (((((&raw const sSpriteTemplates_MoveEffectMons)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 24))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        )) as u16);
        if (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read()) as usize) != 0usize)
            && (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .read()) as usize)
                == 0usize)
        {
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .write(
                (AllocZeroed((((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(4i32)) as u32)))
                    .cast::<u16>(),
            );
        }
        if !((isBackpic) != 0) {
            LoadCompressedPalette(
                GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
                (((256i32).wrapping_add(((palette) as i32).wrapping_mul(16i32))) as u16),
                32u16,
            );
            if ((ignoreDeoxysForm == 1u32)
                || (((ShouldIgnoreDeoxysForm(5u8, ((battler) as u8))) as i32) == 1i32))
                || ((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    != 0i32)
            {
                LoadSpecialPokePic_DontHandleDeoxys(
                    ((&raw mut gMonFrontPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(380)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    ((species) as i32),
                    personality,
                    1u8,
                );
            } else {
                LoadSpecialPokePic_2(
                    ((&raw mut gMonFrontPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(380)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    ((species) as i32),
                    personality,
                    1u8,
                );
            }
        } else {
            LoadCompressedPalette(
                GetMonSpritePalFromSpeciesAndPersonality(species, trainerId, personality),
                (((256i32).wrapping_add(((palette) as i32).wrapping_mul(16i32))) as u16),
                32u16,
            );
            if ((ignoreDeoxysForm == 1u32)
                || (((ShouldIgnoreDeoxysForm(5u8, ((battler) as u8))) as i32) == 1i32))
                || ((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    != 0i32)
            {
                LoadSpecialPokePic_DontHandleDeoxys(
                    ((&raw mut gMonBackPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(380)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    ((species) as i32),
                    personality,
                    0u8,
                );
            } else {
                LoadSpecialPokePic_2(
                    ((&raw mut gMonBackPicTable).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 8),
                    (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(380)
                        .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    ((species) as i32),
                    personality,
                    0u8,
                );
            }
        }
        RequestDma3Copy(
            (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            (((100728832i32).wrapping_add(((sheet) as i32).wrapping_mul(32i32))) as usize
                as *mut u8),
            ((crate::c::div_i32(4096i32, 2i32)) as u16),
            1u8,
        );
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
        if !((isBackpic) != 0) {
            spriteId = CreateSprite(
                (((&raw const sSpriteTemplates_MoveEffectMons)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 24),
                x,
                ((((y) as i32).wrapping_add(
                    ((((((&raw mut gMonFrontPicCoords).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 4))
                    .wrapping_add(1))
                    .read()) as i32),
                )) as i16),
                subpriority,
            );
        } else {
            spriteId = CreateSprite(
                (((&raw const sSpriteTemplates_MoveEffectMons)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 24),
                x,
                ((((y) as i32).wrapping_add(
                    ((((((&raw mut gMonBackPicCoords).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 4))
                    .wrapping_add(1))
                    .read()) as i32),
                )) as i16),
                subpriority,
            );
        }
        if (IsContest()) != 0 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
            .write(
                ((&raw mut gAffineAnims_BattleSpriteContest).cast::<*mut u8>()).cast::<*mut u8>(),
            );
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                0u8,
            );
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroySpriteAndFreeResources_(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        DestroySpriteAndFreeResources(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlerSpriteCoordAttr(battler: u8, attr: u8) -> i16 {
    unsafe {
        let mut battler = battler;
        let mut attr = attr;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut letter: u16 = 0u16;
        let mut unownSpecies: u16 = 0u16;
        let mut ret: i32 = 0i32;
        let mut coords: *mut u8 = core::ptr::null_mut();
        let mut spriteInfo: *mut u8 = core::ptr::null_mut();
        if (IsContest()) != 0 {
            if (crate::c::bf_read(
                (((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4),
                0,
                1,
                false,
            ) as u8)
                != 0
            {
                species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .read();
                personality = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(16)
                .cast::<u32>())
                .read();
            } else {
                species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read();
                personality = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<u32>())
                .read();
            }
            if ((species) as i32) == 201i32 {
                letter = ((crate::c::rem_u32(
                    (((((personality & 50331648u32) >> 18) | ((personality & 196608u32) >> 12))
                        | ((personality & 768u32) >> 6))
                        | ((personality & 3u32) >> 0)),
                    28u32,
                )) as u16);
                if !((letter) != 0) {
                    unownSpecies = 201u16;
                } else {
                    unownSpecies =
                        (((((letter) as i32).wrapping_add(413i32)).wrapping_sub(1i32)) as u16);
                }
                coords = ((&raw mut gMonBackPicCoords).cast::<u8>())
                    .wrapping_offset(((unownSpecies) as i32) as isize * 4);
            } else {
                if ((species) as i32) == 385i32 {
                    coords = (((&raw const gCastformFrontSpriteCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBattleMonForms).cast::<u8>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 4,
                    );
                } else {
                    if ((species) as i32) <= 412i32 {
                        coords = ((&raw mut gMonBackPicCoords).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 4);
                    } else {
                        coords = (&raw mut gMonBackPicCoords).cast::<u8>();
                    }
                }
            }
        } else {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    species = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                    personality = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0i32,
                    );
                } else {
                    species = (((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read();
                    personality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                        .cast::<u32>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read();
                }
                if ((species) as i32) == 201i32 {
                    letter = ((crate::c::rem_u32(
                        (((((personality & 50331648u32) >> 18)
                            | ((personality & 196608u32) >> 12))
                            | ((personality & 768u32) >> 6))
                            | ((personality & 3u32) >> 0)),
                        28u32,
                    )) as u16);
                    if !((letter) != 0) {
                        unownSpecies = 201u16;
                    } else {
                        unownSpecies =
                            (((((letter) as i32).wrapping_add(413i32)).wrapping_sub(1i32)) as u16);
                    }
                    coords = ((&raw mut gMonBackPicCoords).cast::<u8>())
                        .wrapping_offset(((unownSpecies) as i32) as isize * 4);
                } else {
                    if ((species) as i32) > 412i32 {
                        coords = (&raw mut gMonBackPicCoords).cast::<u8>();
                    } else {
                        coords = ((&raw mut gMonBackPicCoords).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 4);
                    }
                }
            } else {
                spriteInfo = ((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                if !(((((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read())
                    != 0)
                {
                    species = ((GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                    personality = GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 100,
                        ),
                        0i32,
                    );
                } else {
                    species = (((spriteInfo).wrapping_offset(((battler) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read();
                    personality = ((((&raw mut gTransformedPersonalities).cast::<u32>())
                        .cast::<u32>())
                    .wrapping_offset(((battler) as i32) as isize))
                    .read();
                }
                if ((species) as i32) == 201i32 {
                    letter = ((crate::c::rem_u32(
                        (((((personality & 50331648u32) >> 18)
                            | ((personality & 196608u32) >> 12))
                            | ((personality & 768u32) >> 6))
                            | ((personality & 3u32) >> 0)),
                        28u32,
                    )) as u16);
                    if !((letter) != 0) {
                        unownSpecies = 201u16;
                    } else {
                        unownSpecies =
                            (((((letter) as i32).wrapping_add(413i32)).wrapping_sub(1i32)) as u16);
                    }
                    coords = ((&raw mut gMonFrontPicCoords).cast::<u8>())
                        .wrapping_offset(((unownSpecies) as i32) as isize * 4);
                } else {
                    if ((species) as i32) == 385i32 {
                        coords = (((&raw const gCastformFrontSpriteCoords)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gBattleMonForms).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 4,
                        );
                    } else {
                        if ((species) as i32) > 412i32 {
                            coords = (&raw mut gMonFrontPicCoords).cast::<u8>();
                        } else {
                            coords = ((&raw mut gMonFrontPicCoords).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 4);
                        }
                    }
                }
            }
        }
        'l1: {
            let __sw1 = ((attr) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                return ((((((coords).read()) as i32) & 15i32).wrapping_mul(8i32)) as i16);
            }
            if __sw1 == 1i32 {
                return ((((((coords).read()) as i32) >> 4).wrapping_mul(8i32)) as i16);
            }
            if __sw1 == 4i32 {
                return ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_sub(
                    crate::c::div_i32(((((coords).read()) as i32) >> 4).wrapping_mul(8i32), 2i32),
                )) as i16);
            }
            if __sw1 == 5i32 {
                return ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_add(
                    crate::c::div_i32(((((coords).read()) as i32) >> 4).wrapping_mul(8i32), 2i32),
                )) as i16);
            }
            if __sw1 == 2i32 {
                return ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_sub(
                    crate::c::div_i32(
                        ((((coords).read()) as i32) & 15i32).wrapping_mul(8i32),
                        2i32,
                    ),
                )) as i16);
            }
            if __sw1 == 3i32 {
                return ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_add(
                    crate::c::div_i32(
                        ((((coords).read()) as i32) & 15i32).wrapping_mul(8i32),
                        2i32,
                    ),
                )) as i16);
            }
            if __sw1 == 6i32 {
                ret = ((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(31i32);
                return (((ret).wrapping_sub(((((coords).wrapping_add(1)).read()) as i32))) as i16);
            }
            if !__matched {
                return 0i16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetAverageBattlerPositions(
    battler: u8,
    respectMonPicOffsets: u8,
    x: *mut i16,
    y: *mut i16,
) {
    unsafe {
        let mut battler = battler;
        let mut respectMonPicOffsets = respectMonPicOffsets;
        let mut x = x;
        let mut y = y;
        let mut xCoordType: u8 = 0u8;
        let mut yCoordType: u8 = 0u8;
        let mut battlerX: i16 = 0i16;
        let mut battlerY: i16 = 0i16;
        let mut partnerX: i16 = 0i16;
        let mut partnerY: i16 = 0i16;
        if !((respectMonPicOffsets) != 0) {
            xCoordType = 0u8;
            yCoordType = 1u8;
        } else {
            xCoordType = 2u8;
            yCoordType = 3u8;
        }
        battlerX = ((GetBattlerSpriteCoord(battler, xCoordType)) as i16);
        battlerY = ((GetBattlerSpriteCoord(battler, yCoordType)) as i16);
        if ((IsDoubleBattle()) != 0) && (!((IsContest()) != 0)) {
            partnerX =
                ((GetBattlerSpriteCoord(((((battler) as i32) ^ 2i32) as u8), xCoordType)) as i16);
            partnerY =
                ((GetBattlerSpriteCoord(((((battler) as i32) ^ 2i32) as u8), yCoordType)) as i16);
        } else {
            partnerX = battlerX;
            partnerY = battlerY;
        }
        (x).write(
            ((crate::c::div_i32(((battlerX) as i32).wrapping_add(((partnerX) as i32)), 2i32))
                as i16),
        );
        (y).write(
            ((crate::c::div_i32(((battlerY) as i32).wrapping_add(((partnerY) as i32)), 2i32))
                as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateInvisibleSpriteCopy(battler: i32, spriteId: u8, species: i32) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut spriteId = spriteId;
        let mut species = species;
        let mut newSpriteId: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
        ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68)
            .cast::<crate::c::Rec4<68>>()
            .write_unaligned(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68)
                    .cast::<crate::c::Rec4<68>>()
                    .read_unaligned(),
            );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68))
            .wrapping_add(63),
            6,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (2u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        return newSpriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTranslateLinearAndFlicker_Flipped(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearAndFlicker));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTranslateLinearAndFlicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            let __p2 =
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3);
            (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        }
        let __p4 = (sprite).wrapping_add(34).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
                .read()) as u8),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearAndFlicker));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSpinningSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AttackerPunchWithTrace(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut src: u16 = 0u16;
        let mut dest: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                (-8i32)
            } else {
                8i32
            }) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_sub((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((AllocSpritePalette(10097u16)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        dest = (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            .wrapping_add(16i32))
        .wrapping_mul(16i32)) as u16);
        src = (((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            false,
        ) as u16) as i32)
            .wrapping_add(16i32))
        .wrapping_mul(16i32)) as u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 20i32)
            || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                == 40i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(2i16);
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(3i16);
        }
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((src) as i32) as isize))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((dest) as i32) as isize))
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
        BlendPalette(
            dest,
            16u16,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as u8),
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_AttackerPunchWithTrace_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AttackerPunchWithTrace_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                CreateBattlerTrace(task, taskId);
                let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 5i32
                {
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                CreateBattlerTrace(task, taskId);
                let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 0i32
                {
                    FreeSpritePaletteByTag(10097u16);
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBattlerTrace(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut spriteId: i16 = CloneBattlerSpriteWithBlend(0u8);
        if ((spriteId) as i32) >= 0i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as u16)
                    as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16)
                    as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(8i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((taskId) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(spriteId);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .read(),
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBattlerTrace));
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBattlerTrace(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_sub(1));
            DestroySpriteWithActiveSheet(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimWeatherBallUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(5i16);
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write((-10i16));
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-40i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWeatherBallUp_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWeatherBallUp_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
                10i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                10i32,
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            < (-20i32)
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            < (-32i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimWeatherBallDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i32 = 0i32;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                    .read()) as i32),
            )) as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            x = (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u16) as i32)
                .wrapping_add(30i32);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(x)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                    .read()) as i32)
                    .wrapping_sub(20i32)) as i16),
            );
        } else {
            x = (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u16) as i32)
                .wrapping_sub(30i32);
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(x)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                    .read()) as i32)
                    .wrapping_sub(80i32)) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
