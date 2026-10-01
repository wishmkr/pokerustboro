//! Translated from `src/fldeff_cut.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_lock::ScriptUnfreezeObjectEvents;
use crate::event_object_movement::AllowObjectAtPosTriggerGroundEffects;
use crate::faraway_island::IsMewPlayingHideAndSeek;
use crate::field_camera::DrawWholeMapView;
use crate::field_effect::{
    FieldEffectActiveListRemove, FieldEffectStart, FieldEffectStop, gFieldEffectArguments,
};
use crate::field_player_avatar::{PlayerGetDestCoords, gPlayerAvatar};
use crate::fieldmap::{
    MapGridGetCollisionAt, MapGridGetElevationAt, MapGridGetMetatileBehaviorAt,
    MapGridGetMetatileIdAt, MapGridSetMetatileIdAt,
};
use crate::fldeff_misc::gPlayerFacingPosition;
use crate::fldeff_rocksmash::{CheckObjectGraphicsInFrontOfPlayer, CreateFieldMoveTask};
use crate::metatile_behavior::{
    MetatileBehavior_IsAshGrass, MetatileBehavior_IsCuttableGrass,
    MetatileBehavior_IsLongGrass_Duplicate, MetatileBehavior_IsLongGrassSouthEdge,
    MetatileBehavior_IsPokeGrass,
};
use crate::overworld::{IncrementGameStat, gFieldCallback2};
use crate::party_menu::{
    FieldCallback_PrepareFadeInFromMenu, GetCursorSelectionMonId, gPostMenuFieldCallback,
};
use crate::pokemon::{GetMonAbility, gPlayerParty};
use crate::script::{ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::task::task_set;
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *mut u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
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

pub(crate) static sCutSquareSide: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sTileCountFromPlayer_X: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sTileCountFromPlayer_Y: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static mut sHyperCutTiles: Aligned<CArray<u8, 25>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCutGrassSpriteArrayPtr: *mut u8 = null_mut();

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}

pub unsafe fn SetUpFieldMove_Cut() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut i: u8 = 0;
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
            sCutSquareSide.set(CUT_HYPER_SIDE);
            sTileCountFromPlayer_X.set(2);
            sTileCountFromPlayer_Y.set(2);
        } else {
            sCutSquareSide.set(CUT_NORMAL_SIDE);
            sTileCountFromPlayer_X.set(1);
            sTileCountFromPlayer_Y.set(1);
        }
        for i in 0..CUT_NORMAL_AREA {
            cutTiles[i] = FALSE;
        }
        for i in 0..CUT_HYPER_AREA {
            sHyperCutTiles[i] = FALSE;
        }
        ret = FALSE;
        i = 0;
        while i < CUT_NORMAL_SIDE {
            y = i as i16 - 1 + gPlayerFacingPosition.y;
            for j in 0..CUT_NORMAL_SIDE {
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
            for i in 0..16u8 {
                x = gPlayerFacingPosition.x + sHyperCutStruct[i].x as i16;
                y = gPlayerFacingPosition.y + sHyperCutStruct[i].y as i16;
                tileCuttable = TRUE;
                for j in 0..2u8 {
                    if sHyperCutStruct[i].unk2[j] == 0 {
                        break;
                    }
                    if cutTiles[sHyperCutStruct[i].unk2[j] as i32 - 1] == FALSE {
                        tileCuttable = FALSE;
                        break;
                    }
                }
                if tileCuttable == TRUE
                    && MapGridGetElevationAt(x as i32, y as i32) as i32
                        == gPlayerFacingPosition.elevation as i32
                {
                    let tileArrayId: u8 =
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
            if ret == TRUE {
                gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
                gPostMenuFieldCallback = Some(FieldCallback_CutGrass);
            }
        }
        return ret;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn FieldCallback_CutGrass() {
    FieldEffectStart(FLDEFF_USE_CUT_ON_GRASS);
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseCutOnGrass() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    task_set(
        taskId,
        8,
        (StartCutGrassFieldEffect as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        StartCutGrassFieldEffect as *const () as usize as u32 as i16,
    );
    IncrementGameStat(GAME_STAT_USED_CUT);
    FALSE
}
pub(crate) unsafe fn FieldCallback_CutTree() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_UseCut.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseCutOnTree() -> u8 {
    let taskId: u8 = CreateFieldMoveTask();
    task_set(
        taskId,
        8,
        (StartCutTreeFieldEffect as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        9,
        StartCutTreeFieldEffect as *const () as usize as u32 as i16,
    );
    IncrementGameStat(GAME_STAT_USED_CUT);
    FALSE
}
pub(crate) unsafe fn StartCutGrassFieldEffect() {
    FieldEffectActiveListRemove(FLDEFF_USE_CUT_ON_GRASS);
    FieldEffectStart(FLDEFF_CUT_GRASS);
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_CutGrass() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlaySE(SE_M_CUT);
    PlayerGetDestCoords(
        &raw mut gPlayerFacingPosition.x,
        &raw mut gPlayerFacingPosition.y,
    );
    let mut i: u8 = 0;
    while i < CUT_HYPER_AREA {
        if sHyperCutTiles[i] == TRUE {
            let xAdd: i8 = (i as i32 % 5) as i8 - 2;
            let yAdd: i8 = (i as i32 / 5) as i8 - 2;
            x = xAdd as i16 + gPlayerFacingPosition.x;
            y = yAdd as i16 + gPlayerFacingPosition.y;
            SetCutGrassMetatile(x, y);
            AllowObjectAtPosTriggerGroundEffects(x, y);
        }
        i += 1;
    }
    SetCutGrassMetatiles(
        gPlayerFacingPosition.x - sTileCountFromPlayer_X.get() as i16,
        gPlayerFacingPosition.y - (1 + sTileCountFromPlayer_Y.get() as i16),
    );
    DrawWholeMapView();
    sCutGrassSpriteArrayPtr = AllocZeroed(CUT_SPRITE_ARRAY_COUNT as u32) as *mut u8;
    for i in 0..CUT_SPRITE_ARRAY_COUNT {
        *sCutGrassSpriteArrayPtr.at(i) = CreateSprite(
            (&raw const *sSpriteTemplate_CutGrass).cast_mut(),
            gSprites[gPlayerAvatar.spriteId].oam.x() as i16 + 8,
            gSprites[gPlayerAvatar.spriteId].oam.y() as i16 + 20,
            0,
        );
        gSprites[*sCutGrassSpriteArrayPtr.at(i)].data[2] = 32 * i as i16;
    }
    FALSE
}
unsafe fn SetCutGrassMetatile(x: i16, y: i16) {
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
unsafe fn GetLongGrassCaseAt(x: i16, y: i16) -> u8 {
    let metatileId: u16 = MapGridGetMetatileIdAt(x as i32, y as i32) as u16;
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
        0
    }
}
unsafe fn SetCutGrassMetatiles(x: i16, y: i16) {
    let lowerY: i16 = y + sCutSquareSide.get() as i16;
    let mut i: i16 = 0;
    while i < sCutSquareSide.get() as i16 {
        let currentX: i16 = x + i;
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
    if sCutSquareSide.get() == CUT_HYPER_SIDE {
        HandleLongGrassOnHyper(0, x, y);
        HandleLongGrassOnHyper(1, x, y);
    }
}
unsafe fn HandleLongGrassOnHyper(caseId: u8, x: i16, y: i16) {
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
    if arr[2] == TRUE
        && MapGridGetMetatileIdAt(newX as i32, y as i32 + 3) == METATILE_General_LongGrass
    {
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
pub(crate) unsafe fn CutGrassSpriteCallback1(sprite: *mut Sprite) {
    (*sprite).data[0] = 8;
    (*sprite).data[1] = 0;
    (*sprite).data[3] = 0;
    (*sprite).callback = Some(CutGrassSpriteCallback2);
}
pub(crate) unsafe fn CutGrassSpriteCallback2(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[2], (*sprite).data[0]);
    (*sprite).y2 = Cos((*sprite).data[2], (*sprite).data[0]);
    (*sprite).data[2] = ((*sprite).data[2] + 8) & 0xFF;
    (*sprite).data[0] += 1 + ((*sprite).data[3] >> 2);
    (*sprite).data[3] += 1;
    if (*sprite).data[1] != 28 {
        (*sprite).data[1] += 1;
    } else {
        (*sprite).callback = Some(CutGrassSpriteCallbackEnd);
    }
}
pub(crate) unsafe fn CutGrassSpriteCallbackEnd(sprite: *mut Sprite) {
    for i in 1..CUT_SPRITE_ARRAY_COUNT {
        DestroySprite(&raw mut gSprites[*sCutGrassSpriteArrayPtr.at(i)]);
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
            (*crate::asmdata::FarawayIsland_Interior_EventScript_HideMewWhenGrassCut
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
}
pub unsafe fn FixLongGrassMetatilesWindowTop(x: i16, y: i16) {
    let metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
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
pub unsafe fn FixLongGrassMetatilesWindowBottom(x: i16, y: i16) {
    if MapGridGetMetatileIdAt(x as i32, y as i32) == METATILE_General_Grass as i32 {
        let metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32 + 1) as u8;
        if MetatileBehavior_IsLongGrassSouthEdge(metatileBehavior) != 0 {
            let metatileId: i32 = MapGridGetMetatileIdAt(x as i32, y as i32 + 1);
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
pub(crate) unsafe fn StartCutTreeFieldEffect() {
    PlaySE(SE_M_CUT);
    FieldEffectActiveListRemove(FLDEFF_USE_CUT_ON_TREE);
    ScriptContext_Enable();
}
