//! Translated from `src/fldeff_cut.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sHyperCutStruct sOamData_CutGrass sSpriteAnim_CutGrass sSpriteAnimTable_CutGrass sSpriteImageTable_CutGrass gSpritePalette_CutGrass sSpriteTemplate_CutGrass
#[allow(unused_imports)]
use crate::data::fldeff_cut::*;

pub(crate) static mut sCutSquareSide: u8 = 0u8;
pub(crate) static mut sTileCountFromPlayer_X: u8 = 0u8;
pub(crate) static mut sTileCountFromPlayer_Y: u8 = 0u8;
pub(crate) static mut sHyperCutTiles: crate::ffi::Align4<[u8; 25]> = crate::ffi::Align4([0; 25]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCutGrassSpriteArrayPtr: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut EventScript_UseCut: u8;
    static mut FarawayIsland_Interior_EventScript_HideMewWhenGrassCut: u8;
    static mut gFieldCallback2: u8;
    static mut gFieldEffectArguments: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerFacingPosition: u8;
    static mut gPlayerParty: u8;
    static mut gPostMenuFieldCallback: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllowObjectAtPosTriggerGroundEffects(a0: i16, a1: i16);
    fn CheckObjectGraphicsInFrontOfPlayer(a0: u8) -> u8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateFieldMoveTask() -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DrawWholeMapView();
    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut u8, a1: u8);
    fn Free(a0: *mut u8);
    fn GetCursorSelectionMonId() -> u8;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn IncrementGameStat(a0: u8);
    fn IsMewPlayingHideAndSeek() -> u8;
    fn MapGridGetCollisionAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MetatileBehavior_IsAshGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsCuttableGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsLongGrassSouthEdge(a0: u8) -> u8;
    fn MetatileBehavior_IsLongGrass_Duplicate(a0: u8) -> u8;
    fn MetatileBehavior_IsPokeGrass(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn ScriptUnfreezeObjectEvents();
    fn Sin(a0: i16, a1: i16) -> i16;
    fn UnlockPlayerFieldControls();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_Cut() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut tileBehavior: u8 = 0u8;
        let mut userAbility: u8 = 0u8;
        let mut cutTiles = crate::ffi::Align4([0u8; 9]);
        let mut ret: u8 = 0u8;
        if ((CheckObjectGraphicsInFrontOfPlayer(82u8)) as i32) == 1i32 {
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_CutTree));
            return 1u8;
        } else {
            PlayerGetDestCoords(
                ((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>(),
                ((&raw mut gPlayerFacingPosition).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i16>(),
            );
            userAbility = GetMonAbility(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetCursorSelectionMonId()) as i32) as isize * 100),
            );
            if ((userAbility) as i32) == 52i32 {
                ((&raw mut sCutSquareSide).cast::<u8>().cast::<u8>()).write(5u8);
                ((&raw mut sTileCountFromPlayer_X).cast::<u8>().cast::<u8>()).write(2u8);
                ((&raw mut sTileCountFromPlayer_Y).cast::<u8>().cast::<u8>()).write(2u8);
            } else {
                ((&raw mut sCutSquareSide).cast::<u8>().cast::<u8>()).write(3u8);
                ((&raw mut sTileCountFromPlayer_X).cast::<u8>().cast::<u8>()).write(1u8);
                ((&raw mut sTileCountFromPlayer_Y).cast::<u8>().cast::<u8>()).write(1u8);
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 9i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut cutTiles).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                            .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 25i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ret = 0u8;
            {
                i = 0u8;
                'l5: loop {
                    if !(((i) as i32) < 3i32) {
                        break 'l5;
                    }
                    'l6: {
                        y = (((((i) as i32).wrapping_sub(1i32)).wrapping_add(
                            (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<i16>())
                            .read()) as i32),
                        )) as i16);
                        {
                            j = 0u8;
                            'l7: loop {
                                if !(((j) as i32) < 3i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    x = (((((j) as i32).wrapping_sub(1i32)).wrapping_add(
                                        (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                            .cast::<i16>())
                                        .read()) as i32),
                                    )) as i16);
                                    if ((MapGridGetElevationAt(((x) as i32), ((y) as i32))) as i32)
                                        == (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                            .wrapping_add(4)
                                            .cast::<i8>())
                                        .read()) as i32)
                                    {
                                        tileBehavior = ((MapGridGetMetatileBehaviorAt(
                                            ((x) as i32),
                                            ((y) as i32),
                                        ))
                                            as u8);
                                        if (((MetatileBehavior_IsPokeGrass(tileBehavior)) as i32)
                                            == 1i32)
                                            || (((MetatileBehavior_IsAshGrass(tileBehavior))
                                                as i32)
                                                == 1i32)
                                        {
                                            ((((&raw mut sHyperCutTiles).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(
                                                (((6i32)
                                                    .wrapping_add(((i) as i32).wrapping_mul(5i32)))
                                                .wrapping_add(((j) as i32)))
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            ret = 1u8;
                                        }
                                        if ((MapGridGetCollisionAt(((x) as i32), ((y) as i32)))
                                            as i32)
                                            == 1i32
                                        {
                                            (((&raw mut cutTiles).cast::<u8>()).wrapping_offset(
                                                ((((i) as i32).wrapping_mul(3i32))
                                                    .wrapping_add(((j) as i32)))
                                                    as isize,
                                            ))
                                            .write(0u8);
                                        } else {
                                            (((&raw mut cutTiles).cast::<u8>()).wrapping_offset(
                                                ((((i) as i32).wrapping_mul(3i32))
                                                    .wrapping_add(((j) as i32)))
                                                    as isize,
                                            ))
                                            .write(1u8);
                                            if ((MetatileBehavior_IsCuttableGrass(tileBehavior))
                                                as i32)
                                                == 1i32
                                            {
                                                ((((&raw mut sHyperCutTiles).cast::<u8>())
                                                    .cast::<u8>())
                                                .wrapping_offset(
                                                    (((6i32).wrapping_add(
                                                        ((i) as i32).wrapping_mul(5i32),
                                                    ))
                                                    .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .write(1u8);
                                            }
                                        }
                                    } else {
                                        (((&raw mut cutTiles).cast::<u8>()).wrapping_offset(
                                            ((((i) as i32).wrapping_mul(3i32))
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .write(0u8);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((userAbility) as i32) != 52i32 {
                if ((ret) as i32) == 1i32 {
                    ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                        .write(Some(FieldCallback_PrepareFadeInFromMenu));
                    ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(FieldCallback_CutGrass));
                }
            } else {
                let mut tileCuttable: u8 = 0u8;
                {
                    i = 0u8;
                    'l9: loop {
                        if !(((i) as i32) < 16i32) {
                            break 'l9;
                        }
                        'l10: {
                            x = (((((((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>())
                                .read()) as i32)
                                .wrapping_add(
                                    (((((((&raw const sHyperCutStruct).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<i8>())
                                    .read()) as i32),
                                )) as i16);
                            y = (((((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(
                                    (((((((&raw const sHyperCutStruct).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(1)
                                    .cast::<i8>())
                                    .read()) as i32),
                                )) as i16);
                            tileCuttable = 1u8;
                            {
                                j = 0u8;
                                'l11: loop {
                                    if !(((j) as i32) < 2i32) {
                                        break 'l11;
                                    }
                                    'l12: {
                                        if (((((((((&raw const sHyperCutStruct)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                        .wrapping_add(2))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            break 'l11;
                                        }
                                        if (((((&raw mut cutTiles).cast::<u8>()).wrapping_offset(
                                            ((((((((((((&raw const sHyperCutStruct)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2))
                                            .cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as u8)
                                                as i32)
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            tileCuttable = 0u8;
                                            break 'l11;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            if ((tileCuttable) as i32) == 1i32 {
                                if ((MapGridGetElevationAt(((x) as i32), ((y) as i32))) as i32)
                                    == (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<i8>())
                                    .read()) as i32)
                                {
                                    let mut tileArrayId: u8 =
                                        (((((((((((&raw const sHyperCutStruct)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                        .wrapping_add(1)
                                        .cast::<i8>())
                                        .read())
                                            as i32)
                                            .wrapping_mul(5i32))
                                        .wrapping_add(12i32))
                                        .wrapping_add(
                                            (((((((&raw const sHyperCutStruct)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 4))
                                            .cast::<i8>())
                                            .read())
                                                as i32),
                                        )) as u8);
                                    tileBehavior =
                                        ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32)))
                                            as u8);
                                    if (((MetatileBehavior_IsPokeGrass(tileBehavior)) as i32)
                                        == 1i32)
                                        || (((MetatileBehavior_IsAshGrass(tileBehavior)) as i32)
                                            == 1i32)
                                    {
                                        ((&raw mut gFieldCallback2)
                                            .cast::<Option<unsafe extern "C" fn() -> u8>>())
                                        .write(Some(FieldCallback_PrepareFadeInFromMenu));
                                        ((&raw mut gPostMenuFieldCallback)
                                            .cast::<Option<unsafe extern "C" fn()>>())
                                        .write(Some(FieldCallback_CutGrass));
                                        ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>())
                                            .wrapping_offset(((tileArrayId) as i32) as isize))
                                        .write(1u8);
                                        ret = 1u8;
                                    } else {
                                        if ((MetatileBehavior_IsCuttableGrass(tileBehavior)) as i32)
                                            == 1i32
                                        {
                                            ((((&raw mut sHyperCutTiles).cast::<u8>())
                                                .cast::<u8>())
                                            .wrapping_offset(((tileArrayId) as i32) as isize))
                                            .write(1u8);
                                        }
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((ret) as i32) == 1i32 {
                    ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                        .write(Some(FieldCallback_PrepareFadeInFromMenu));
                    ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(FieldCallback_CutGrass));
                }
            }
            return ret;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_CutGrass() {
    unsafe {
        FieldEffectStart(1u8);
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseCutOnGrass() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((StartCutGrassFieldEffect as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((StartCutGrassFieldEffect as *const () as usize as u32) as i16));
        IncrementGameStat(18u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_CutTree() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        ScriptContext_SetupScript((&raw mut EventScript_UseCut).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseCutOnTree() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((StartCutTreeFieldEffect as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((StartCutTreeFieldEffect as *const () as usize as u32) as i16));
        IncrementGameStat(18u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartCutGrassFieldEffect() {
    unsafe {
        FieldEffectActiveListRemove(1u8);
        FieldEffectStart(58u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_CutGrass() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut i: u8 = 0u8;
        PlaySE(128u16);
        PlayerGetDestCoords(
            ((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>(),
            ((&raw mut gPlayerFacingPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 25i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 1i32
                    {
                        let mut xAdd: i8 =
                            (((crate::c::rem_i32(((i) as i32), 5i32)).wrapping_sub(2i32)) as i8);
                        let mut yAdd: i8 =
                            (((crate::c::div_i32(((i) as i32), 5i32)).wrapping_sub(2i32)) as i8);
                        x = ((((xAdd) as i32).wrapping_add(
                            (((((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>())
                                .read()) as i32),
                        )) as i16);
                        y = ((((yAdd) as i32).wrapping_add(
                            (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<i16>())
                            .read()) as i32),
                        )) as i16);
                        SetCutGrassMetatile(x, y);
                        AllowObjectAtPosTriggerGroundEffects(x, y);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetCutGrassMetatiles(
            (((((((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>()).read()) as i32)
                .wrapping_sub(
                    ((((&raw mut sTileCountFromPlayer_X).cast::<u8>().cast::<u8>()).read()) as i32),
                )) as i16),
            (((((((&raw mut gPlayerFacingPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub((1i32).wrapping_add(
                    ((((&raw mut sTileCountFromPlayer_Y).cast::<u8>().cast::<u8>()).read()) as i32),
                ))) as i16),
        );
        DrawWholeMapView();
        ((&raw mut sCutGrassSpriteArrayPtr)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(8u32));
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut sCutGrassSpriteArrayPtr)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_CutGrass)
                            .cast::<u8>()
                            .cast_mut(),
                        (((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                    as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(2),
                            0,
                            9,
                            false,
                        ) as u32)
                            .wrapping_add(8u32)) as i16),
                        (((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                    as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(0),
                            0,
                            8,
                            false,
                        ) as u32)
                            .wrapping_add(20u32)) as i16),
                        0u8,
                    ));
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sCutGrassSpriteArrayPtr)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write((((32i32).wrapping_mul(((i) as i32))) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetCutGrassMetatile(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 520i32 || __sw1 == 21i32 || __sw1 == 13i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 1u16);
                break 'l1;
            }
            if __sw1 == 454i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 462u16);
                break 'l1;
            }
            if __sw1 == 455i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 463u16);
                break 'l1;
            }
            if __sw1 == 641i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 633u16);
                break 'l1;
            }
            if __sw1 == 642i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 634u16);
                break 'l1;
            }
            if __sw1 == 643i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 635u16);
                break 'l1;
            }
            if __sw1 == 518i32 || __sw1 == 519i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 625u16);
                break 'l1;
            }
            if __sw1 == 530i32 || __sw1 == 522i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 536u16);
                break 'l1;
            }
            if __sw1 == 37i32 {
                MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 14u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetLongGrassCaseAt(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileId: u16 = ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16);
        if ((metatileId) as i32) == 1i32 {
            return 1u8;
        } else {
            if ((metatileId) as i32) == 633i32 {
                return 2u8;
            } else {
                if ((metatileId) as i32) == 634i32 {
                    return 3u8;
                } else {
                    if ((metatileId) as i32) == 635i32 {
                        return 4u8;
                    } else {
                        return 0u8;
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
pub(crate) unsafe extern "C" fn SetCutGrassMetatiles(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut i: i16 = 0i16;
        let mut lowerY: i16 = ((((y) as i32)
            .wrapping_add(((((&raw mut sCutSquareSide).cast::<u8>().cast::<u8>()).read()) as i32)))
            as i16);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut sCutSquareSide).cast::<u8>().cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut currentX: i16 = ((((x) as i32).wrapping_add(((i) as i32))) as i16);
                    if MapGridGetMetatileIdAt(((currentX) as i32), ((y) as i32)) == 21i32 {
                        'l3: {
                            let __sw1 = ((GetLongGrassCaseAt(
                                currentX,
                                ((((y) as i32).wrapping_add(1i32)) as i16),
                            )) as i32);
                            if __sw1 == 1i32 {
                                MapGridSetMetatileIdAt(
                                    ((currentX) as i32),
                                    ((y) as i32).wrapping_add(1i32),
                                    520u16,
                                );
                                break 'l3;
                            }
                            if __sw1 == 2i32 {
                                MapGridSetMetatileIdAt(
                                    ((currentX) as i32),
                                    ((y) as i32).wrapping_add(1i32),
                                    641u16,
                                );
                                break 'l3;
                            }
                            if __sw1 == 3i32 {
                                MapGridSetMetatileIdAt(
                                    ((currentX) as i32),
                                    ((y) as i32).wrapping_add(1i32),
                                    642u16,
                                );
                                break 'l3;
                            }
                            if __sw1 == 4i32 {
                                MapGridSetMetatileIdAt(
                                    ((currentX) as i32),
                                    ((y) as i32).wrapping_add(1i32),
                                    643u16,
                                );
                                break 'l3;
                            }
                        }
                    }
                    if MapGridGetMetatileIdAt(((currentX) as i32), ((lowerY) as i32)) == 1i32 {
                        if MapGridGetMetatileIdAt(
                            ((currentX) as i32),
                            ((lowerY) as i32).wrapping_add(1i32),
                        ) == 520i32
                        {
                            MapGridSetMetatileIdAt(
                                ((currentX) as i32),
                                ((lowerY) as i32).wrapping_add(1i32),
                                1u16,
                            );
                        }
                        if MapGridGetMetatileIdAt(
                            ((currentX) as i32),
                            ((lowerY) as i32).wrapping_add(1i32),
                        ) == 641i32
                        {
                            MapGridSetMetatileIdAt(
                                ((currentX) as i32),
                                ((lowerY) as i32).wrapping_add(1i32),
                                633u16,
                            );
                        }
                        if MapGridGetMetatileIdAt(
                            ((currentX) as i32),
                            ((lowerY) as i32).wrapping_add(1i32),
                        ) == 642i32
                        {
                            MapGridSetMetatileIdAt(
                                ((currentX) as i32),
                                ((lowerY) as i32).wrapping_add(1i32),
                                634u16,
                            );
                        }
                        if MapGridGetMetatileIdAt(
                            ((currentX) as i32),
                            ((lowerY) as i32).wrapping_add(1i32),
                        ) == 643i32
                        {
                            MapGridSetMetatileIdAt(
                                ((currentX) as i32),
                                ((lowerY) as i32).wrapping_add(1i32),
                                635u16,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut sCutSquareSide).cast::<u8>().cast::<u8>()).read()) as i32) == 5i32 {
            HandleLongGrassOnHyper(0u8, x, y);
            HandleLongGrassOnHyper(1u8, x, y);
        }
    }
}
pub(crate) unsafe extern "C" fn HandleLongGrassOnHyper(caseId: u8, x: i16, y: i16) {
    unsafe {
        let mut caseId = caseId;
        let mut x = x;
        let mut y = y;
        let mut newX: i16 = 0i16;
        let mut arr = crate::ffi::Align4([0u8; 3]);
        if ((caseId) as i32) == 0i32 {
            ((&raw mut arr).cast::<u8>()).write(
                ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(5)).read(),
            );
            (((&raw mut arr).cast::<u8>()).wrapping_offset(1)).write(
                ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(10))
                    .read(),
            );
            (((&raw mut arr).cast::<u8>()).wrapping_offset(2)).write(
                ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(15))
                    .read(),
            );
            newX = x;
        } else {
            if ((caseId) as i32) == 1i32 {
                ((&raw mut arr).cast::<u8>()).write(
                    ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(9))
                        .read(),
                );
                (((&raw mut arr).cast::<u8>()).wrapping_offset(1)).write(
                    ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(14))
                        .read(),
                );
                (((&raw mut arr).cast::<u8>()).wrapping_offset(2)).write(
                    ((((&raw mut sHyperCutTiles).cast::<u8>()).cast::<u8>()).wrapping_offset(19))
                        .read(),
                );
                newX = ((((x) as i32).wrapping_add(4i32)) as i16);
            } else {
                return;
            }
        }
        if ((((&raw mut arr).cast::<u8>()).read()) as i32) == 1i32 {
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32)) == 520i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32), 1u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32)) == 641i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32), 633u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32)) == 642i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32), 634u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32)) == 643i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32), 635u16);
            }
        }
        if (((((&raw mut arr).cast::<u8>()).wrapping_offset(1)).read()) as i32) == 1i32 {
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(2i32)) == 21i32 {
                'l1: {
                    let __sw1 =
                        ((GetLongGrassCaseAt(newX, ((((y) as i32).wrapping_add(3i32)) as i16)))
                            as i32);
                    if __sw1 == 1i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(3i32),
                            520u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 2i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(3i32),
                            641u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 3i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(3i32),
                            642u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 4i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(3i32),
                            643u16,
                        );
                        break 'l1;
                    }
                }
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32)) == 520i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32), 1u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32)) == 641i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32), 633u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32)) == 642i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32), 634u16);
            }
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32)) == 643i32 {
                MapGridSetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(4i32), 635u16);
            }
        }
        if (((((&raw mut arr).cast::<u8>()).wrapping_offset(2)).read()) as i32) == 1i32 {
            if MapGridGetMetatileIdAt(((newX) as i32), ((y) as i32).wrapping_add(3i32)) == 21i32 {
                'l2: {
                    let __sw2 =
                        ((GetLongGrassCaseAt(newX, ((((y) as i32).wrapping_add(4i32)) as i16)))
                            as i32);
                    if __sw2 == 1i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(4i32),
                            520u16,
                        );
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(4i32),
                            641u16,
                        );
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(4i32),
                            642u16,
                        );
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        MapGridSetMetatileIdAt(
                            ((newX) as i32),
                            ((y) as i32).wrapping_add(4i32),
                            643u16,
                        );
                        break 'l2;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallback1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(CutGrassSpriteCallback2));
    }
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallback2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(8i32)
                & 255i32) as i16),
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add((1i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 2),
            ))) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 28i32
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(CutGrassSpriteCallbackEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallbackEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sCutGrassSpriteArrayPtr)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FieldEffectStop(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut sCutGrassSpriteArrayPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read()) as i32) as isize
                    * 68,
            ),
            58u8,
        );
        {
            Free(
                ((&raw mut sCutGrassSpriteArrayPtr)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sCutGrassSpriteArrayPtr)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        ScriptUnfreezeObjectEvents();
        UnlockPlayerFieldControls();
        if ((IsMewPlayingHideAndSeek()) as i32) == 1i32 {
            ScriptContext_SetupScript(
                (&raw mut FarawayIsland_Interior_EventScript_HideMewWhenGrassCut).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixLongGrassMetatilesWindowTop(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileBehavior: u8 =
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
        if (MetatileBehavior_IsLongGrass_Duplicate(metatileBehavior)) != 0 {
            'l1: {
                let __sw1 =
                    ((GetLongGrassCaseAt(x, ((((y) as i32).wrapping_add(1i32)) as i16))) as i32);
                if __sw1 == 1i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32), 520u16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32), 641u16);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32), 642u16);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32), 643u16);
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixLongGrassMetatilesWindowBottom(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if MapGridGetMetatileIdAt(((x) as i32), ((y) as i32)) == 1i32 {
            let mut metatileBehavior: u8 =
                ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32).wrapping_add(1i32)))
                    as u8);
            if (MetatileBehavior_IsLongGrassSouthEdge(metatileBehavior)) != 0 {
                let mut metatileId: i32 =
                    MapGridGetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32));
                'l1: {
                    let __sw1 = metatileId;
                    if __sw1 == 520i32 {
                        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_add(1i32), 1u16);
                        break 'l1;
                    }
                    if __sw1 == 641i32 {
                        MapGridSetMetatileIdAt(
                            ((x) as i32),
                            ((y) as i32).wrapping_add(1i32),
                            633u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 642i32 {
                        MapGridSetMetatileIdAt(
                            ((x) as i32),
                            ((y) as i32).wrapping_add(1i32),
                            634u16,
                        );
                        break 'l1;
                    }
                    if __sw1 == 643i32 {
                        MapGridSetMetatileIdAt(
                            ((x) as i32),
                            ((y) as i32).wrapping_add(1i32),
                            635u16,
                        );
                        break 'l1;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartCutTreeFieldEffect() {
    unsafe {
        PlaySE(128u16);
        FieldEffectActiveListRemove(2u8);
        ScriptContext_Enable();
    }
}
