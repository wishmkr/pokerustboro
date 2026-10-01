//! Translated from `src/fldeff_cut.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sHyperCutStruct sOamData_CutGrass sSpriteAnim_CutGrass sSpriteAnimTable_CutGrass sSpriteImageTable_CutGrass gSpritePalette_CutGrass sSpriteTemplate_CutGrass

/// `struct HyperCutterUnk`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct HyperCutterUnk {
    pub x: i8,
    pub y: i8,
    pub unk2: CArray<u8, 2>,
}

unsafe impl Sync for HyperCutterUnk {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<HyperCutterUnk>() == 4);
    assert!(offset_of!(HyperCutterUnk, x) == 0);
    assert!(offset_of!(HyperCutterUnk, y) == 1);
    assert!(offset_of!(HyperCutterUnk, unk2) == 2);
};

const CUT_HYPER_AREA: u8 = 25;
const CUT_HYPER_SIDE: u8 = 5;
const CUT_NORMAL_AREA: u8 = 9;
const CUT_NORMAL_SIDE: u8 = 3;
const CUT_SPRITE_ARRAY_COUNT: u8 = 8;
const LONG_GRASS_BASE_CENTER: u8 = 3;
const LONG_GRASS_BASE_LEFT: u8 = 2;
const LONG_GRASS_BASE_RIGHT: u8 = 4;
const LONG_GRASS_FIELD: u8 = 1;
const LONG_GRASS_NONE: u8 = 0;

static sHyperCutStruct: Table<CArray<HyperCutterUnk, 16>> =
    Table((&raw const crate::data::fldeff_cut::sHyperCutStruct).cast());
static sSpriteTemplate_CutGrass: Table<SpriteTemplate> =
    Table((&raw const crate::data::fldeff_cut::sSpriteTemplate_CutGrass).cast());

pub(crate) static mut sCutSquareSide: u8 = 0;
pub(crate) static mut sTileCountFromPlayer_X: u8 = 0;
pub(crate) static mut sTileCountFromPlayer_Y: u8 = 0;
pub(crate) static mut sHyperCutTiles: Aligned<CArray<u8, 25>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCutGrassSpriteArrayPtr: *mut u8 = null_mut();

unsafe extern "C" {
    static EventScript_UseCut: CArray<u8, 0>;
    static FarawayIsland_Interior_EventScript_HideMewWhenGrassCut: CArray<u8, 0>;
    static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerFacingPosition: MapPosition;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPostMenuFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AllowObjectAtPosTriggerGroundEffects(a0: i16, a1: i16);
    fn CheckObjectGraphicsInFrontOfPlayer(a0: u8) -> u8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateFieldMoveTask() -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DrawWholeMapView();
    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut Sprite, a1: u8);
    fn Free(a0: *mut c_void);
    fn GetCursorSelectionMonId() -> u8;
    fn GetMonAbility(a0: *mut Pokemon) -> u8;
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
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut tileBehavior: u8 = 0;
    let mut userAbility: u8 = 0;
    let mut cutTiles: CArray<u8, 9> = zeroed();
    let mut ret: u8 = 0;
    if CheckObjectGraphicsInFrontOfPlayer(OBJ_EVENT_GFX_CUTTABLE_TREE) == TRUE {
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_CutTree);
        return TRUE;
    } else {
        PlayerGetDestCoords(
            &raw mut gPlayerFacingPosition.x,
            &raw mut gPlayerFacingPosition.y,
        );
        userAbility = GetMonAbility(&raw mut gPlayerParty[GetCursorSelectionMonId()]);
        if userAbility == ABILITY_HYPER_CUTTER {
            sCutSquareSide = CUT_HYPER_SIDE;
            sTileCountFromPlayer_X = 2;
            sTileCountFromPlayer_Y = 2;
        } else {
            sCutSquareSide = CUT_NORMAL_SIDE;
            sTileCountFromPlayer_X = 1;
            sTileCountFromPlayer_Y = 1;
        }
        i = 0;
        while i < CUT_NORMAL_AREA {
            cutTiles[i] = FALSE;
            i += 1;
        }
        i = 0;
        while i < CUT_HYPER_AREA {
            sHyperCutTiles[i] = FALSE;
            i += 1;
        }
        ret = FALSE;
        i = 0;
        while i < CUT_NORMAL_SIDE {
            y = i as i16 - 1 + gPlayerFacingPosition.y;
            j = 0;
            while j < CUT_NORMAL_SIDE {
                x = j as i16 - 1 + gPlayerFacingPosition.x;
                if MapGridGetElevationAt(x as i32, y as i32) as i32
                    == gPlayerFacingPosition.elevation as i32
                {
                    tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
                    if MetatileBehavior_IsPokeGrass(tileBehavior) == TRUE
                        || MetatileBehavior_IsAshGrass(tileBehavior) == TRUE
                    {
                        sHyperCutTiles[6 + i as i32 * 5 + j as i32] = TRUE;
                        ret = TRUE;
                    }
                    if MapGridGetCollisionAt(x as i32, y as i32) == 1 {
                        cutTiles[i as i32 * 3 + j as i32] = FALSE;
                    } else {
                        cutTiles[i as i32 * 3 + j as i32] = TRUE;
                        if MetatileBehavior_IsCuttableGrass(tileBehavior) == TRUE {
                            sHyperCutTiles[6 + i as i32 * 5 + j as i32] = TRUE;
                        }
                    }
                } else {
                    cutTiles[i as i32 * 3 + j as i32] = FALSE;
                }
                j += 1;
            }
            i += 1;
        }
        if userAbility != ABILITY_HYPER_CUTTER {
            if ret == TRUE {
                gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
                gPostMenuFieldCallback = Some(FieldCallback_CutGrass);
            }
        } else {
            let mut tileCuttable: u8 = 0;
            i = 0;
            while i < 16 {
                x = gPlayerFacingPosition.x + sHyperCutStruct[i].x as i16;
                y = gPlayerFacingPosition.y + sHyperCutStruct[i].y as i16;
                tileCuttable = TRUE;
                j = 0;
                while j < 2 {
                    if sHyperCutStruct[i].unk2[j] == 0 {
                        break;
                    }
                    if cutTiles[sHyperCutStruct[i].unk2[j] as i32 - 1] == FALSE {
                        tileCuttable = FALSE;
                        break;
                    }
                    j += 1;
                }
                if tileCuttable == TRUE {
                    if MapGridGetElevationAt(x as i32, y as i32) as i32
                        == gPlayerFacingPosition.elevation as i32
                    {
                        let mut tileArrayId: u8 =
                            sHyperCutStruct[i].y as u8 * 5 + 12 + sHyperCutStruct[i].x as u8;
                        tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
                        if MetatileBehavior_IsPokeGrass(tileBehavior) == TRUE
                            || MetatileBehavior_IsAshGrass(tileBehavior) == TRUE
                        {
                            gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
                            gPostMenuFieldCallback = Some(FieldCallback_CutGrass);
                            sHyperCutTiles[tileArrayId] = TRUE;
                            ret = TRUE;
                        } else {
                            if MetatileBehavior_IsCuttableGrass(tileBehavior) == TRUE {
                                sHyperCutTiles[tileArrayId] = TRUE;
                            }
                        }
                    }
                }
                i += 1;
            }
            if ret == TRUE {
                gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
                gPostMenuFieldCallback = Some(FieldCallback_CutGrass);
            }
        }
        return ret;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_CutGrass() {
    FieldEffectStart(FLDEFF_USE_CUT_ON_GRASS);
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseCutOnGrass() -> u8 {
    let mut taskId: u8 = CreateFieldMoveTask();
    gTasks[taskId].data[8] = (StartCutGrassFieldEffect as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[9] = StartCutGrassFieldEffect as *const () as usize as u32 as i16;
    IncrementGameStat(GAME_STAT_USED_CUT);
    return FALSE;
}
pub(crate) unsafe extern "C" fn FieldCallback_CutTree() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    ScriptContext_SetupScript(EventScript_UseCut.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseCutOnTree() -> u8 {
    let mut taskId: u8 = CreateFieldMoveTask();
    gTasks[taskId].data[8] = (StartCutTreeFieldEffect as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[9] = StartCutTreeFieldEffect as *const () as usize as u32 as i16;
    IncrementGameStat(GAME_STAT_USED_CUT);
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartCutGrassFieldEffect() {
    FieldEffectActiveListRemove(FLDEFF_USE_CUT_ON_GRASS);
    FieldEffectStart(FLDEFF_CUT_GRASS);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_CutGrass() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut i: u8 = 0;
    PlaySE(SE_M_CUT);
    PlayerGetDestCoords(
        &raw mut gPlayerFacingPosition.x,
        &raw mut gPlayerFacingPosition.y,
    );
    i = 0;
    while i < CUT_HYPER_AREA {
        if sHyperCutTiles[i] == TRUE {
            let mut xAdd: i8 = (i as i32 % 5) as i8 - 2;
            let mut yAdd: i8 = (i as i32 / 5) as i8 - 2;
            x = xAdd as i16 + gPlayerFacingPosition.x;
            y = yAdd as i16 + gPlayerFacingPosition.y;
            SetCutGrassMetatile(x, y);
            AllowObjectAtPosTriggerGroundEffects(x, y);
        }
        i += 1;
    }
    SetCutGrassMetatiles(
        gPlayerFacingPosition.x - sTileCountFromPlayer_X as i16,
        gPlayerFacingPosition.y - (1 + sTileCountFromPlayer_Y as i16),
    );
    DrawWholeMapView();
    sCutGrassSpriteArrayPtr = AllocZeroed(CUT_SPRITE_ARRAY_COUNT as u32) as *mut u8;
    i = 0;
    while i < CUT_SPRITE_ARRAY_COUNT {
        *sCutGrassSpriteArrayPtr.at(i) = CreateSprite(
            (&raw const *sSpriteTemplate_CutGrass).cast_mut(),
            gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + 8,
            gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + 20,
            0,
        );
        gSprites[*sCutGrassSpriteArrayPtr.at(i)].data[2] = 32 * i as i16;
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetCutGrassMetatile(x: i16, y: i16) {
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        METATILE_Fortree_LongGrass_Root
        | METATILE_General_LongGrass
        | METATILE_General_TallGrass => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_General_Grass);
        }
        METATILE_General_TallGrass_TreeLeft => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_General_Grass_TreeLeft);
        }
        METATILE_General_TallGrass_TreeRight => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_General_Grass_TreeRight);
        }
        METATILE_Fortree_SecretBase_LongGrass_BottomLeft => {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                METATILE_Fortree_SecretBase_LongGrass_TopLeft,
            );
        }
        METATILE_Fortree_SecretBase_LongGrass_BottomMid => {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                METATILE_Fortree_SecretBase_LongGrass_TopMid,
            );
        }
        METATILE_Fortree_SecretBase_LongGrass_BottomRight => {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                METATILE_Fortree_SecretBase_LongGrass_TopRight,
            );
        }
        518 | METATILE_Lavaridge_AshGrass => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_Lavaridge_LavaField);
        }
        530 | METATILE_Fallarbor_AshGrass => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_Fallarbor_AshField);
        }
        METATILE_General_TallGrass_TreeUp => {
            MapGridSetMetatileIdAt(x as i32, y as i32, METATILE_General_Grass_TreeUp);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetLongGrassCaseAt(x: i16, y: i16) -> u8 {
    let mut metatileId: u16 = MapGridGetMetatileIdAt(x as i32, y as i32) as u16;
    if metatileId == METATILE_General_Grass {
        return LONG_GRASS_FIELD;
    } else if metatileId == METATILE_Fortree_SecretBase_LongGrass_TopLeft {
        return LONG_GRASS_BASE_LEFT;
    } else if metatileId == METATILE_Fortree_SecretBase_LongGrass_TopMid {
        return LONG_GRASS_BASE_CENTER;
    } else if metatileId == METATILE_Fortree_SecretBase_LongGrass_TopRight {
        return LONG_GRASS_BASE_RIGHT;
    } else {
        return LONG_GRASS_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetCutGrassMetatiles(x: i16, y: i16) {
    let mut i: i16 = 0;
    let mut lowerY: i16 = y + sCutSquareSide as i16;
    i = 0;
    while i < sCutSquareSide as i16 {
        let mut currentX: i16 = x + i;
        if MapGridGetMetatileIdAt(currentX as i32, y as i32) == METATILE_General_LongGrass {
            match GetLongGrassCaseAt(currentX, y + 1) {
                LONG_GRASS_FIELD => {
                    MapGridSetMetatileIdAt(
                        currentX as i32,
                        y as i32 + 1,
                        METATILE_Fortree_LongGrass_Root as u16,
                    );
                }
                LONG_GRASS_BASE_LEFT => {
                    MapGridSetMetatileIdAt(
                        currentX as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_BottomLeft as u16,
                    );
                }
                LONG_GRASS_BASE_CENTER => {
                    MapGridSetMetatileIdAt(
                        currentX as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_BottomMid as u16,
                    );
                }
                LONG_GRASS_BASE_RIGHT => {
                    MapGridSetMetatileIdAt(
                        currentX as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_BottomRight as u16,
                    );
                }
                _ => {}
            }
        }
        if MapGridGetMetatileIdAt(currentX as i32, lowerY as i32) == METATILE_General_Grass as i32 {
            if MapGridGetMetatileIdAt(currentX as i32, lowerY as i32 + 1)
                == METATILE_Fortree_LongGrass_Root
            {
                MapGridSetMetatileIdAt(currentX as i32, lowerY as i32 + 1, 0x001);
            }
            if MapGridGetMetatileIdAt(currentX as i32, lowerY as i32 + 1)
                == METATILE_Fortree_SecretBase_LongGrass_BottomLeft
            {
                MapGridSetMetatileIdAt(
                    currentX as i32,
                    lowerY as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_TopLeft,
                );
            }
            if MapGridGetMetatileIdAt(currentX as i32, lowerY as i32 + 1)
                == METATILE_Fortree_SecretBase_LongGrass_BottomMid
            {
                MapGridSetMetatileIdAt(
                    currentX as i32,
                    lowerY as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_TopMid,
                );
            }
            if MapGridGetMetatileIdAt(currentX as i32, lowerY as i32 + 1)
                == METATILE_Fortree_SecretBase_LongGrass_BottomRight
            {
                MapGridSetMetatileIdAt(
                    currentX as i32,
                    lowerY as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_TopRight,
                );
            }
        }
        i += 1;
    }
    if sCutSquareSide == CUT_HYPER_SIDE {
        HandleLongGrassOnHyper(0, x, y);
        HandleLongGrassOnHyper(1, x, y);
    }
}
pub(crate) unsafe extern "C" fn HandleLongGrassOnHyper(caseId: u8, x: i16, y: i16) {
    let mut newX: i16 = 0;
    let mut arr: CArray<u8, 3> = zeroed();
    if caseId == 0 {
        arr[0] = sHyperCutTiles[5];
        arr[1] = sHyperCutTiles[10];
        arr[2] = sHyperCutTiles[15];
        newX = x;
    } else if caseId == 1 {
        arr[0] = sHyperCutTiles[9];
        arr[1] = sHyperCutTiles[14];
        arr[2] = sHyperCutTiles[19];
        newX = x + 4;
    } else {
        return;
    }
    if arr[0] == TRUE {
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 3) == METATILE_Fortree_LongGrass_Root {
            MapGridSetMetatileIdAt(newX as i32, y as i32 + 3, METATILE_General_Grass);
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 3)
            == METATILE_Fortree_SecretBase_LongGrass_BottomLeft
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 3,
                METATILE_Fortree_SecretBase_LongGrass_TopLeft,
            );
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 3)
            == METATILE_Fortree_SecretBase_LongGrass_BottomMid
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 3,
                METATILE_Fortree_SecretBase_LongGrass_TopMid,
            );
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 3)
            == METATILE_Fortree_SecretBase_LongGrass_BottomRight
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 3,
                METATILE_Fortree_SecretBase_LongGrass_TopRight,
            );
        }
    }
    if arr[1] == 1 {
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 2) == METATILE_General_LongGrass {
            match GetLongGrassCaseAt(newX, y + 3) {
                LONG_GRASS_FIELD => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 3,
                        METATILE_Fortree_LongGrass_Root as u16,
                    );
                }
                LONG_GRASS_BASE_LEFT => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 3,
                        METATILE_Fortree_SecretBase_LongGrass_BottomLeft as u16,
                    );
                }
                LONG_GRASS_BASE_CENTER => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 3,
                        METATILE_Fortree_SecretBase_LongGrass_BottomMid as u16,
                    );
                }
                LONG_GRASS_BASE_RIGHT => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 3,
                        METATILE_Fortree_SecretBase_LongGrass_BottomRight as u16,
                    );
                }
                _ => {}
            }
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 4) == METATILE_Fortree_LongGrass_Root {
            MapGridSetMetatileIdAt(newX as i32, y as i32 + 4, METATILE_General_Grass);
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 4)
            == METATILE_Fortree_SecretBase_LongGrass_BottomLeft
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 4,
                METATILE_Fortree_SecretBase_LongGrass_TopLeft,
            );
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 4)
            == METATILE_Fortree_SecretBase_LongGrass_BottomMid
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 4,
                METATILE_Fortree_SecretBase_LongGrass_TopMid,
            );
        }
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 4)
            == METATILE_Fortree_SecretBase_LongGrass_BottomRight
        {
            MapGridSetMetatileIdAt(
                newX as i32,
                y as i32 + 4,
                METATILE_Fortree_SecretBase_LongGrass_TopRight,
            );
        }
    }
    if arr[2] == TRUE {
        if MapGridGetMetatileIdAt(newX as i32, y as i32 + 3) == METATILE_General_LongGrass {
            match GetLongGrassCaseAt(newX, y + 4) {
                LONG_GRASS_FIELD => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 4,
                        METATILE_Fortree_LongGrass_Root as u16,
                    );
                }
                LONG_GRASS_BASE_LEFT => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 4,
                        METATILE_Fortree_SecretBase_LongGrass_BottomLeft as u16,
                    );
                }
                LONG_GRASS_BASE_CENTER => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 4,
                        METATILE_Fortree_SecretBase_LongGrass_BottomMid as u16,
                    );
                }
                LONG_GRASS_BASE_RIGHT => {
                    MapGridSetMetatileIdAt(
                        newX as i32,
                        y as i32 + 4,
                        METATILE_Fortree_SecretBase_LongGrass_BottomRight as u16,
                    );
                }
                _ => {}
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallback1(sprite: *mut Sprite) {
    (*sprite).data[0] = 8;
    (*sprite).data[1] = 0;
    (*sprite).data[3] = 0;
    (*sprite).callback = Some(CutGrassSpriteCallback2);
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallback2(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[2], (*sprite).data[0]);
    (*sprite).y2 = Cos((*sprite).data[2], (*sprite).data[0]);
    (*sprite).data[2] = (*sprite).data[2] + 8 & 0xFF;
    (*sprite).data[0] += 1 + ((*sprite).data[3] >> 2);
    (*sprite).data[3] += 1;
    if (*sprite).data[1] != 28 {
        (*sprite).data[1] += 1;
    } else {
        (*sprite).callback = Some(CutGrassSpriteCallbackEnd);
    }
}
pub(crate) unsafe extern "C" fn CutGrassSpriteCallbackEnd(sprite: *mut Sprite) {
    let mut i: u8 = 0;
    i = 1;
    while i < CUT_SPRITE_ARRAY_COUNT {
        DestroySprite(&raw mut gSprites[*sCutGrassSpriteArrayPtr.at(i)]);
        i += 1;
    }
    FieldEffectStop(
        &raw mut gSprites[*sCutGrassSpriteArrayPtr],
        FLDEFF_CUT_GRASS,
    );
    Free(sCutGrassSpriteArrayPtr as *mut c_void);
    sCutGrassSpriteArrayPtr = null_mut();
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
    if IsMewPlayingHideAndSeek() == TRUE {
        ScriptContext_SetupScript(
            FarawayIsland_Interior_EventScript_HideMewWhenGrassCut
                .as_ptr()
                .cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixLongGrassMetatilesWindowTop(x: i16, y: i16) {
    let mut metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    if MetatileBehavior_IsLongGrass_Duplicate(metatileBehavior) != 0 {
        match GetLongGrassCaseAt(x, y + 1) {
            LONG_GRASS_FIELD => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32 + 1,
                    METATILE_Fortree_LongGrass_Root as u16,
                );
            }
            LONG_GRASS_BASE_LEFT => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_BottomLeft as u16,
                );
            }
            LONG_GRASS_BASE_CENTER => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_BottomMid as u16,
                );
            }
            LONG_GRASS_BASE_RIGHT => {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32 + 1,
                    METATILE_Fortree_SecretBase_LongGrass_BottomRight as u16,
                );
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FixLongGrassMetatilesWindowBottom(x: i16, y: i16) {
    if MapGridGetMetatileIdAt(x as i32, y as i32) == METATILE_General_Grass as i32 {
        let mut metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32 + 1) as u8;
        if MetatileBehavior_IsLongGrassSouthEdge(metatileBehavior) != 0 {
            let mut metatileId: i32 = MapGridGetMetatileIdAt(x as i32, y as i32 + 1);
            match metatileId {
                METATILE_Fortree_LongGrass_Root => {
                    MapGridSetMetatileIdAt(x as i32, y as i32 + 1, 0x001);
                }
                METATILE_Fortree_SecretBase_LongGrass_BottomLeft => {
                    MapGridSetMetatileIdAt(
                        x as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_TopLeft,
                    );
                }
                METATILE_Fortree_SecretBase_LongGrass_BottomMid => {
                    MapGridSetMetatileIdAt(
                        x as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_TopMid,
                    );
                }
                METATILE_Fortree_SecretBase_LongGrass_BottomRight => {
                    MapGridSetMetatileIdAt(
                        x as i32,
                        y as i32 + 1,
                        METATILE_Fortree_SecretBase_LongGrass_TopRight,
                    );
                }
                _ => {}
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartCutTreeFieldEffect() {
    PlaySE(SE_M_CUT);
    FieldEffectActiveListRemove(FLDEFF_USE_CUT_ON_TREE);
    ScriptContext_Enable();
}
