//! Translated from `src/field_effect_helpers.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::{
    CreateCopySpriteAt, ElevationToPriority, GetFigure8XOffset, GetFigure8YOffset,
    GetObjectEventGraphicsInfo, GetObjectEventIdByLocalIdAndMap, GetObjectPaletteTag,
    LoadPlayerObjectReflectionPalette, LoadSpecialObjectReflectionPalette, MoveCoords,
    PatchObjectPalette, SetObjectSubpriorityByElevation, SetSpritePosToMapCoords,
    SetSpritePosToOffsetMapCoords, TryGetObjectEventIdByLocalIdAndMap,
    UpdateObjectEventSpriteInvisibility,
};
use crate::field_camera::CurrentMapDrawMetatileAt;
use crate::field_effect::{
    FieldEffectActiveListRemove, FieldEffectStart, FieldEffectStop, gFieldEffectArguments,
};
use crate::field_player_avatar::{gObjectEvents, gPlayerAvatar};
use crate::field_weather::UpdateSpritePaletteWithWeather;
use crate::fieldmap::{
    MapGridGetElevationAt, MapGridGetMetatileBehaviorAt, MapGridSetMetatileIdAt, gCamera,
};
use crate::gpu_regs::SetGpuReg;
use crate::load_save::gSaveBlock1Ptr;
use crate::metatile_behavior::{
    MetatileBehavior_GetBridgeType, MetatileBehavior_IsLongGrass, MetatileBehavior_IsPokeGrass,
    MetatileBehavior_IsReflective, MetatileBehavior_IsSurfableWaterOrUnderwater,
    MetatileBehavior_IsTallGrass,
};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSpriteAtEnd` with this module's view of its types.
#[inline]
fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSpriteAtEnd(a0 as _, a1, a2, a3) }
}
/// `SeekSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn SeekSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::SeekSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sBitfield: usize = 0;
const sElevation: usize = 0;
const sFinished: usize = 0;
const sJumpElevation: usize = 0;
const sReflectionObjEventId: usize = 0;
const sSpriteId: usize = 0;
const sWaitFldEff: usize = 0;
const sBobY: usize = 1;
const sEndTimer: usize = 1;
const sJumpFldEff: usize = 1;
const sMoveTimer: usize = 1;
const sPlayerOffset: usize = 1;
const sReflectionObjEventLocalId: usize = 1;
const sX: usize = 1;
const sPlayerObjId: usize = 2;
const sReflectionVerticalOffset: usize = 2;
const sMetatileId: usize = 3;
const sVelocity: usize = 3;
const sYOffset: usize = 3;
const sDelay: usize = 4;
const sStartY: usize = 4;
const sCounter: usize = 5;
const sCurrentMap: usize = 5;
const sIntervalIdx: usize = 5;
const sAnimCounter: usize = 6;
const sAnimState: usize = 7;
const sIsStillReflection: usize = 7;
const sObjectMoved: usize = 7;
const sReadyToEnd: usize = 7;
// Data tables (translate with cdata.py): sShadowEffectTemplateIds gShadowVerticalOffsets gFadeFootprintsTireTracksFuncs gAshFieldEffectFuncs sFigure8XOffsets sFigure8YOffsets

const OBJ_EVENT_PAL_TAG_NONE: u16 = 4607;

static gAshFieldEffectFuncs: Table<CArray<Option<unsafe fn(*mut Sprite)>, 3>> =
    Table((&raw const crate::data::field_effect_helpers::gAshFieldEffectFuncs).cast());
static gFadeFootprintsTireTracksFuncs: Table<CArray<Option<unsafe fn(*mut Sprite)>, 2>> =
    Table((&raw const crate::data::field_effect_helpers::gFadeFootprintsTireTracksFuncs).cast());
static gShadowVerticalOffsets: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::field_effect_helpers::gShadowVerticalOffsets).cast());
static sShadowEffectTemplateIds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_effect_helpers::sShadowEffectTemplateIds).cast());

pub unsafe fn SetUpReflection(
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
    stillReflection: u8,
) {
    let reflectionSprite: *mut Sprite = {
        let gl_i0 = CreateCopySpriteAt(sprite, (*sprite).x, (*sprite).y, 152);
        crate::gl!(&raw gSprites[gl_i0])
    };
    (*reflectionSprite).callback = Some(UpdateObjectReflectionSprite);
    (*reflectionSprite).oam.set_priority(3);
    (*reflectionSprite).oam.set_paletteNum(
        (*(&raw const crate::data::event_object_movement::gReflectionEffectPaletteMap)
            .cast::<CArray<u8, 0>>())[(*reflectionSprite).oam.paletteNum()] as u16,
    );
    (*reflectionSprite).set_usingSheet(TRUE as u16);
    (*reflectionSprite).anims = (*(&raw const crate::sprite::gDummySpriteAnimTable)
        .cast::<CArray<*mut AnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    StartSpriteAnim(reflectionSprite, 0);
    (*reflectionSprite).affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    (*reflectionSprite).set_affineAnimBeginning(TRUE as u16);
    (*reflectionSprite).set_subspriteMode(SUBSPRITES_OFF);
    (*reflectionSprite).data[0] = (*sprite).data[0];
    (*reflectionSprite).data[sReflectionObjEventLocalId] = (*objectEvent).localId as i16;
    (*reflectionSprite).data[sIsStillReflection] = stillReflection as i16;
    LoadObjectReflectionPalette(objectEvent, reflectionSprite);
    if stillReflection == 0 {
        (*reflectionSprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    }
}
unsafe fn GetReflectionVerticalOffset(objectEvent: *mut ObjectEvent) -> i16 {
    (*GetObjectEventGraphicsInfo((*objectEvent).graphicsId)).height - 2
}
unsafe fn LoadObjectReflectionPalette(
    objectEvent: *mut ObjectEvent,
    reflectionSprite: *mut Sprite,
) {
    let mut bridgeType: u8 = 0;
    let mut bridgeReflectionVerticalOffsets: CArray<u16, 3> = zeroed();
    bridgeReflectionVerticalOffsets[0] = 12;
    bridgeReflectionVerticalOffsets[1] = 28;
    bridgeReflectionVerticalOffsets[2] = 44;
    (*reflectionSprite).data[sReflectionVerticalOffset] = 0;
    if (*GetObjectEventGraphicsInfo((*objectEvent).graphicsId)).disableReflectionPaletteLoad() == 0
        && (({
            bridgeType = MetatileBehavior_GetBridgeType((*objectEvent).previousMetatileBehavior);
            bridgeType
        }) != 0
            || ({
                bridgeType = MetatileBehavior_GetBridgeType((*objectEvent).currentMetatileBehavior);
                bridgeType
            }) != 0)
    {
        (*reflectionSprite).data[sReflectionVerticalOffset] =
            bridgeReflectionVerticalOffsets[bridgeType as i32 - 1] as i16;
        LoadObjectHighBridgeReflectionPalette(
            objectEvent,
            (*reflectionSprite).oam.paletteNum() as u8,
        );
    } else {
        LoadObjectRegularReflectionPalette(objectEvent, (*reflectionSprite).oam.paletteNum() as u8);
    }
}
unsafe fn LoadObjectRegularReflectionPalette(objectEvent: *mut ObjectEvent, paletteIndex: u8) {
    let graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
    if (*graphicsInfo).reflectionPaletteTag != OBJ_EVENT_PAL_TAG_NONE {
        if (*graphicsInfo).paletteSlot() == PALSLOT_PLAYER {
            LoadPlayerObjectReflectionPalette((*graphicsInfo).paletteTag, paletteIndex);
        } else if (*graphicsInfo).paletteSlot() == PALSLOT_NPC_SPECIAL {
            LoadSpecialObjectReflectionPalette((*graphicsInfo).paletteTag, paletteIndex);
        } else {
            PatchObjectPalette(GetObjectPaletteTag(paletteIndex), paletteIndex);
        }
        UpdateSpritePaletteWithWeather(paletteIndex);
    }
}
unsafe fn LoadObjectHighBridgeReflectionPalette(objectEvent: *mut ObjectEvent, paletteNum: u8) {
    let graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
    if (*graphicsInfo).reflectionPaletteTag != OBJ_EVENT_PAL_TAG_NONE {
        PatchObjectPalette((*graphicsInfo).reflectionPaletteTag, paletteNum);
        UpdateSpritePaletteWithWeather(paletteNum);
    }
}
pub(crate) unsafe fn UpdateObjectReflectionSprite(reflectionSprite: *mut Sprite) {
    let objectEvent: *mut ObjectEvent = {
        let gl_i0 = (*reflectionSprite).data[sReflectionObjEventId];
        crate::gl!(&raw gObjectEvents[gl_i0])
    };
    let mainSprite: *mut Sprite = {
        let gl_i1 = (*objectEvent).spriteId;
        crate::gl!(&raw gSprites[gl_i1])
    };
    if (*objectEvent).active() == 0
        || (*objectEvent).hasReflection() == 0
        || (*objectEvent).localId as i16 != (*reflectionSprite).data[sReflectionObjEventLocalId]
    {
        (*reflectionSprite).set_inUse(FALSE as u16);
    } else {
        (*reflectionSprite).oam.set_paletteNum(
            (*(&raw const crate::data::event_object_movement::gReflectionEffectPaletteMap)
                .cast::<CArray<u8, 0>>())[(*mainSprite).oam.paletteNum()] as u16,
        );
        (*reflectionSprite).oam.set_shape((*mainSprite).oam.shape());
        (*reflectionSprite).oam.set_size((*mainSprite).oam.size());
        (*reflectionSprite)
            .oam
            .set_matrixNum((*mainSprite).oam.matrixNum() | ST_OAM_VFLIP);
        (*reflectionSprite)
            .oam
            .set_tileNum((*mainSprite).oam.tileNum());
        (*reflectionSprite).subspriteTables = (*mainSprite).subspriteTables;
        (*reflectionSprite).set_subspriteTableNum((*mainSprite).subspriteTableNum());
        (*reflectionSprite).set_invisible((*mainSprite).invisible());
        (*reflectionSprite).x = (*mainSprite).x;
        (*reflectionSprite).y = (*mainSprite).y
            + GetReflectionVerticalOffset(objectEvent)
            + (*reflectionSprite).data[sReflectionVerticalOffset];
        (*reflectionSprite).centerToCornerVecX = (*mainSprite).centerToCornerVecX;
        (*reflectionSprite).centerToCornerVecY = (*mainSprite).centerToCornerVecY;
        (*reflectionSprite).x2 = (*mainSprite).x2;
        (*reflectionSprite).y2 = -(*mainSprite).y2;
        (*reflectionSprite).set_coordOffsetEnabled((*mainSprite).coordOffsetEnabled());
        if (*objectEvent).hideReflection() == TRUE as u32 {
            (*reflectionSprite).set_invisible(TRUE as u16);
        }
        if (*reflectionSprite).data[sIsStillReflection] == FALSE as i16 {
            (*reflectionSprite).oam.set_matrixNum(0);
            if (*mainSprite).oam.matrixNum() & ST_OAM_HFLIP != 0 {
                (*reflectionSprite).oam.set_matrixNum(1);
            }
        }
    }
}
pub unsafe fn CreateWarpArrowSprite() -> u8 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[8],
        0,
        0,
        82,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).set_invisible(TRUE as u16);
    }
    spriteId
}
pub unsafe fn SetSpriteInvisible(spriteId: u8) {
    (*gSprites.as_ptr())[spriteId].set_invisible(TRUE as u16);
}
pub unsafe fn ShowWarpArrowSprite(spriteId: u8, direction: u8, x: i16, y: i16) {
    let mut sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    if (*sprite).invisible() != 0 || (*sprite).data[0] != x || (*sprite).data[1] != y {
        let mut x2: i16 = 0;
        let mut y2: i16 = 0;
        SetSpritePosToMapCoords(x, y, &raw mut x2, &raw mut y2);
        sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).x = x2 + 8;
        (*sprite).y = y2 + 8;
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).data[0] = x;
        (*sprite).data[1] = y;
        StartSpriteAnim(sprite, direction - 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Shadow() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo(crate::gl!(gObjectEvents[objectEventId].graphicsId));
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())
            [sShadowEffectTemplateIds[(*graphicsInfo).shadowSize()]],
        0,
        0,
        148,
    );
    if spriteId != MAX_SPRITES {
        (*gSprites.as_ptr())[spriteId].set_coordOffsetEnabled(TRUE as u16);
        crate::gl!(gSprites[spriteId].data[0] = crate::gl!(gFieldEffectArguments[0]) as i16);
        crate::gl!(gSprites[spriteId].data[1] = crate::gl!(gFieldEffectArguments[1]) as i16);
        crate::gl!(gSprites[spriteId].data[2] = crate::gl!(gFieldEffectArguments[2]) as i16);
        {
            let gl_v = ((*graphicsInfo).height >> 1)
                - gShadowVerticalOffsets[(*graphicsInfo).shadowSize()] as i16;
            let gl_i2 = sYOffset;
            crate::gl!(gSprites[spriteId].data[gl_i2] = gl_v)
        };
    }
    0
}
pub unsafe fn UpdateShadowFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
    {
        FieldEffectStop(sprite, FLDEFF_SHADOW);
    } else {
        let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
        let linkedSprite: *mut Sprite = {
            let gl_i3 = (*objectEvent).spriteId;
            crate::gl!(&raw gSprites[gl_i3])
        };
        (*sprite).oam.set_priority((*linkedSprite).oam.priority());
        (*sprite).x = (*linkedSprite).x;
        (*sprite).y = (*linkedSprite).y + (*sprite).data[sYOffset];
        if (*objectEvent).active() == 0
            || (*objectEvent).hasShadow() == 0
            || MetatileBehavior_IsPokeGrass((*objectEvent).currentMetatileBehavior) != 0
            || MetatileBehavior_IsSurfableWaterOrUnderwater((*objectEvent).currentMetatileBehavior)
                != 0
            || MetatileBehavior_IsSurfableWaterOrUnderwater((*objectEvent).previousMetatileBehavior)
                != 0
            || MetatileBehavior_IsReflective((*objectEvent).currentMetatileBehavior) != 0
            || MetatileBehavior_IsReflective((*objectEvent).previousMetatileBehavior) != 0
        {
            FieldEffectStop(sprite, FLDEFF_SHADOW);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_TallGrass() -> u32 {
    let mut x: i16 = crate::gl!(gFieldEffectArguments[0]) as i16;
    let mut y: i16 = crate::gl!(gFieldEffectArguments[1]) as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[4],
        x,
        y,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sX] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[3] = crate::gl!(gFieldEffectArguments[4]) as i16;
        (*sprite).data[4] = crate::gl!(gFieldEffectArguments[5]) as i16;
        (*sprite).data[sCurrentMap] = crate::gl!(gFieldEffectArguments[6]) as i16;
        if crate::gl!(gFieldEffectArguments[7]) != 0 {
            SeekSpriteAnim(sprite, 4);
        }
    }
    0
}
pub unsafe fn UpdateTallGrassFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    let mut mapNum: u8 = ((*sprite).data[sCurrentMap] >> 8) as u8;
    let mut mapGroup: u8 = (*sprite).data[sCurrentMap] as u8;
    if (*gCamera.as_ptr()).active() != 0
        && ((*crate::gl!(gSaveBlock1Ptr)).location.mapNum as i32 != mapNum as i32
            || (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup as i32 != mapGroup as i32)
    {
        (*sprite).data[sX] -= crate::gl!(gCamera.x) as i16;
        (*sprite).data[2] -= crate::gl!(gCamera.y) as i16;
        (*sprite).data[sCurrentMap] = ((*crate::gl!(gSaveBlock1Ptr)).location.mapNum as u8 as i16)
            << 8
            | (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup as u8 as i16;
    }
    let localId: u8 = ((*sprite).data[3] >> 8) as u8;
    mapNum = (*sprite).data[3] as u8;
    mapGroup = (*sprite).data[4] as u8;
    let mut metatileBehavior: u8 =
        MapGridGetMetatileBehaviorAt((*sprite).data[sX] as i32, (*sprite).data[2] as i32) as u8;
    if TryGetObjectEventIdByLocalIdAndMap(localId, mapNum, mapGroup, &raw mut objectEventId) != 0
        || MetatileBehavior_IsTallGrass(metatileBehavior) == 0
        || (*sprite).data[sObjectMoved] != 0 && (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, FLDEFF_TALL_GRASS);
    } else {
        let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
        if ((*objectEvent).currentCoords.x != (*sprite).data[sX]
            || (*objectEvent).currentCoords.y != (*sprite).data[2])
            && ((*objectEvent).previousCoords.x != (*sprite).data[sX]
                || (*objectEvent).previousCoords.y != (*sprite).data[2])
        {
            (*sprite).data[sObjectMoved] = TRUE as i16;
        }
        metatileBehavior = 0;
        if (*sprite).animCmdIndex == 0 {
            metatileBehavior = 4;
        }
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        UpdateGrassFieldEffectSubpriority(
            sprite,
            (*sprite).data[sElevation] as u8,
            metatileBehavior,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_JumpTallGrass() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        12,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[10],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sJumpElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sJumpFldEff] = FLDEFF_JUMP_TALL_GRASS as i16;
    }
    0
}
pub unsafe fn FindTallGrassFieldEffectSpriteId(
    localId: u8,
    mapNum: u8,
    mapGroup: u8,
    x: i16,
    y: i16,
) -> u8 {
    for i in 0..MAX_SPRITES {
        if (*gSprites.as_ptr())[i].inUse() != 0 {
            let sprite: *mut Sprite = crate::gl!(&raw gSprites[i]);
            if (*sprite).callback == Some(UpdateTallGrassFieldEffect as unsafe fn(*mut Sprite))
                && (x == (*sprite).data[sX] && y == (*sprite).data[2])
                && localId == ((*sprite).data[3] >> 8) as u8
                && mapNum as i32 == (*sprite).data[3] as i32 & 0xFF
                && mapGroup as i16 == (*sprite).data[4]
            {
                return i;
            }
        }
    }
    MAX_SPRITES
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_LongGrass() -> u32 {
    let mut x: i16 = crate::gl!(gFieldEffectArguments[0]) as i16;
    let mut y: i16 = crate::gl!(gFieldEffectArguments[1]) as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[15],
        x,
        y,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(ElevationToPriority(crate::gl!(gFieldEffectArguments[2]) as u8) as u16);
        (*sprite).data[sElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sX] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[3] = crate::gl!(gFieldEffectArguments[4]) as i16;
        (*sprite).data[4] = crate::gl!(gFieldEffectArguments[5]) as i16;
        (*sprite).data[sCurrentMap] = crate::gl!(gFieldEffectArguments[6]) as i16;
        if crate::gl!(gFieldEffectArguments[7]) != 0 {
            SeekSpriteAnim(sprite, 6);
        }
    }
    0
}
pub unsafe fn UpdateLongGrassFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    let mut mapNum: u8 = ((*sprite).data[sCurrentMap] >> 8) as u8;
    let mut mapGroup: u8 = (*sprite).data[sCurrentMap] as u8;
    if (*gCamera.as_ptr()).active() != 0
        && ((*crate::gl!(gSaveBlock1Ptr)).location.mapNum as i32 != mapNum as i32
            || (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup as i32 != mapGroup as i32)
    {
        (*sprite).data[sX] -= crate::gl!(gCamera.x) as i16;
        (*sprite).data[2] -= crate::gl!(gCamera.y) as i16;
        (*sprite).data[sCurrentMap] = ((*crate::gl!(gSaveBlock1Ptr)).location.mapNum as u8 as i16)
            << 8
            | (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup as u8 as i16;
    }
    let localId: u8 = ((*sprite).data[3] >> 8) as u8;
    mapNum = (*sprite).data[3] as u8;
    mapGroup = (*sprite).data[4] as u8;
    let metatileBehavior: u8 =
        MapGridGetMetatileBehaviorAt((*sprite).data[sX] as i32, (*sprite).data[2] as i32) as u8;
    if TryGetObjectEventIdByLocalIdAndMap(localId, mapNum, mapGroup, &raw mut objectEventId) != 0
        || MetatileBehavior_IsLongGrass(metatileBehavior) == 0
        || (*sprite).data[sObjectMoved] != 0 && (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, FLDEFF_LONG_GRASS);
    } else {
        let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
        if ((*objectEvent).currentCoords.x != (*sprite).data[sX]
            || (*objectEvent).currentCoords.y != (*sprite).data[2])
            && ((*objectEvent).previousCoords.x != (*sprite).data[sX]
                || (*objectEvent).previousCoords.y != (*sprite).data[2])
        {
            (*sprite).data[sObjectMoved] = TRUE as i16;
        }
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        UpdateGrassFieldEffectSubpriority(sprite, (*sprite).data[sElevation] as u8, 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_JumpLongGrass() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[16],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sJumpElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sJumpFldEff] = FLDEFF_JUMP_LONG_GRASS as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_ShortGrass() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[30],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority((*gSprites.as_ptr())[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[1] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[3] = {
            let gl_i4 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i4].x)
        };
        (*sprite).data[4] = {
            let gl_i5 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i5].y)
        };
    }
    0
}
pub unsafe fn UpdateShortGrassFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || (*gObjectEvents.as_ptr())[objectEventId].inShortGrass() == 0
    {
        FieldEffectStop(sprite, FLDEFF_SHORT_GRASS);
    } else {
        let graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo(crate::gl!(gObjectEvents[objectEventId].graphicsId));
        let linkedSprite: *mut Sprite = {
            let gl_i6 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(&raw gSprites[gl_i6])
        };
        let parentY: i16 = (*linkedSprite).y;
        let parentX: i16 = (*linkedSprite).x;
        if parentX != (*sprite).data[3] || parentY != (*sprite).data[4] {
            (*sprite).data[3] = parentX;
            (*sprite).data[4] = parentY;
            if (*sprite).animEnded() != 0 {
                StartSpriteAnim(sprite, 0);
            }
        }
        (*sprite).x = parentX;
        (*sprite).y = parentY;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 8;
        (*sprite).subpriority = (*linkedSprite).subpriority - 1;
        (*sprite).oam.set_priority((*linkedSprite).oam.priority());
        UpdateObjectEventSpriteInvisibility(sprite, (*linkedSprite).invisible() as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SandFootprints() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[11],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[7] = FLDEFF_SAND_FOOTPRINTS;
        StartSpriteAnim(sprite, crate::gl!(gFieldEffectArguments[4]) as u8);
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_DeepSandFootprints() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[23],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[7] = FLDEFF_DEEP_SAND_FOOTPRINTS;
        StartSpriteAnim(sprite, crate::gl!(gFieldEffectArguments[4]) as u8);
    }
    spriteId as u32
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_BikeTireTracks() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[27],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[7] = FLDEFF_BIKE_TIRE_TRACKS as i16;
        StartSpriteAnim(sprite, crate::gl!(gFieldEffectArguments[4]) as u8);
    }
    spriteId as u32
}
pub unsafe fn UpdateFootprintsTireTracksFieldEffect(sprite: *mut Sprite) {
    gFadeFootprintsTireTracksFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn FadeFootprintsTireTracks_Step0(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 40
    {
        (*sprite).data[0] = 1;
    }
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
}
pub(crate) unsafe fn FadeFootprintsTireTracks_Step1(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).invisible() ^ 1);
    (*sprite).data[1] += 1;
    UpdateObjectEventSpriteInvisibility(sprite, (*sprite).invisible() as u8);
    if (*sprite).data[1] > 56 {
        FieldEffectStop(sprite, (*sprite).data[7] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Splash() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[13],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        let linkedSprite: *mut Sprite = {
            let gl_i7 = (*objectEvent).spriteId;
            crate::gl!(&raw gSprites[gl_i7])
        };
        (*sprite).oam.set_priority((*linkedSprite).oam.priority());
        (*sprite).data[0] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[1] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 4;
        PlaySE(SE_PUDDLE);
    }
    0
}
pub unsafe fn UpdateSplashFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if (*sprite).animEnded() != 0
        || TryGetObjectEventIdByLocalIdAndMap(
            (*sprite).data[0] as u8,
            (*sprite).data[1] as u8,
            (*sprite).data[2] as u8,
            &raw mut objectEventId,
        ) != 0
    {
        FieldEffectStop(sprite, FLDEFF_SPLASH);
    } else {
        (*sprite).x = {
            let gl_i8 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(gSprites[gl_i8].x)
        };
        (*sprite).y = {
            let gl_i9 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(gSprites[gl_i9].y)
        };
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_JumpSmallSplash() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        12,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[14],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sJumpElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sJumpFldEff] = FLDEFF_JUMP_SMALL_SPLASH as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_JumpBigSplash() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[12],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sJumpElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sJumpFldEff] = FLDEFF_JUMP_BIG_SPLASH as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_FeetInFlowingWater() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[13],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).callback = Some(UpdateFeetInFlowingWaterFieldEffect);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority((*gSprites.as_ptr())[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[1] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[3] = -1;
        (*sprite).data[4] = -1;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 4;
        StartSpriteAnim(sprite, 1);
    }
    0
}
pub(crate) unsafe fn UpdateFeetInFlowingWaterFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || (*gObjectEvents.as_ptr())[objectEventId].inShallowFlowingWater() == 0
    {
        FieldEffectStop(sprite, FLDEFF_FEET_IN_FLOWING_WATER);
    } else {
        let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
        let linkedSprite: *mut Sprite = {
            let gl_i10 = (*objectEvent).spriteId;
            crate::gl!(&raw gSprites[gl_i10])
        };
        (*sprite).x = (*linkedSprite).x;
        (*sprite).y = (*linkedSprite).y;
        (*sprite).subpriority = (*linkedSprite).subpriority;
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        if (*objectEvent).currentCoords.x != (*sprite).data[3]
            || (*objectEvent).currentCoords.y != (*sprite).data[4]
        {
            (*sprite).data[3] = (*objectEvent).currentCoords.x;
            (*sprite).data[4] = (*objectEvent).currentCoords.y;
            if (*sprite).invisible() == 0 {
                PlaySE(SE_PUDDLE);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Ripple() -> u32 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[5],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sWaitFldEff] = FLDEFF_RIPPLE as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_HotSpringsWater() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[31],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority((*gSprites.as_ptr())[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[1] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[3] = {
            let gl_i11 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i11].x)
        };
        (*sprite).data[4] = {
            let gl_i12 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i12].y)
        };
    }
    0
}
pub unsafe fn UpdateHotSpringsWaterFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || (*gObjectEvents.as_ptr())[objectEventId].inHotSprings() == 0
    {
        FieldEffectStop(sprite, FLDEFF_HOT_SPRINGS_WATER);
    } else {
        let graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo(crate::gl!(gObjectEvents[objectEventId].graphicsId));
        let linkedSprite: *mut Sprite = {
            let gl_i13 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(&raw gSprites[gl_i13])
        };
        (*sprite).x = (*linkedSprite).x;
        (*sprite).y = ((*graphicsInfo).height >> 1) + (*linkedSprite).y - 8;
        (*sprite).subpriority = (*linkedSprite).subpriority - 1;
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UnusedGrass() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[17],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sWaitFldEff] = FLDEFF_UNUSED_GRASS;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UnusedGrass2() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[18],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sWaitFldEff] = FLDEFF_UNUSED_GRASS_2;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_UnusedSand() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[19],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sWaitFldEff] = FLDEFF_UNUSED_SAND;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_WaterSurfacing() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[20],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sWaitFldEff] = FLDEFF_WATER_SURFACING;
    }
    0
}
pub unsafe fn StartAshFieldEffect(x: i16, y: i16, metatileId: u16, delay: i16) {
    crate::gl!(gFieldEffectArguments[0] = x as i32);
    crate::gl!(gFieldEffectArguments[1] = y as i32);
    crate::gl!(gFieldEffectArguments[2] = 82);
    crate::gl!(gFieldEffectArguments[3] = 1);
    crate::gl!(gFieldEffectArguments[4] = metatileId as i32);
    crate::gl!(gFieldEffectArguments[5] = delay as i32);
    FieldEffectStart(FLDEFF_ASH);
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Ash() -> u32 {
    let mut x: i16 = crate::gl!(gFieldEffectArguments[0]) as i16;
    let mut y: i16 = crate::gl!(gFieldEffectArguments[1]) as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[6],
        x,
        y,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sX] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[sMetatileId] = crate::gl!(gFieldEffectArguments[4]) as i16;
        (*sprite).data[sDelay] = crate::gl!(gFieldEffectArguments[5]) as i16;
    }
    0
}
pub unsafe fn UpdateAshFieldEffect(sprite: *mut Sprite) {
    gAshFieldEffectFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn UpdateAshFieldEffect_Wait(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).set_animPaused(TRUE);
    if ({
        (*sprite).data[sDelay] -= 1;
        (*sprite).data[sDelay]
    }) == 0
    {
        (*sprite).data[0] = 1;
    }
}
pub(crate) unsafe fn UpdateAshFieldEffect_Show(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    (*sprite).set_animPaused(FALSE);
    MapGridSetMetatileIdAt(
        (*sprite).data[sX] as i32,
        (*sprite).data[2] as i32,
        (*sprite).data[sMetatileId] as u16,
    );
    CurrentMapDrawMetatileAt((*sprite).data[sX] as i32, (*sprite).data[2] as i32);
    (*gObjectEvents.as_ptr())[crate::gl!(gPlayerAvatar.objectEventId)]
        .set_triggerGroundEffectsOnMove(TRUE as u32);
    (*sprite).data[0] = 2;
}
pub(crate) unsafe fn UpdateAshFieldEffect_End(sprite: *mut Sprite) {
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_ASH);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SurfBlob() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[7],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        150,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_paletteNum(0);
        (*sprite).data[sPlayerObjId] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sVelocity] = -1;
        (*sprite).data[6] = -1;
        (*sprite).data[7] = -1;
    }
    FieldEffectActiveListRemove(FLDEFF_SURF_BLOB);
    spriteId as u32
}
pub unsafe fn SetSurfBlob_BobState(spriteId: u8, state: u8) {
    {
        let gl_i14 = sBitfield;
        crate::gl!(
            gSprites[spriteId].data[gl_i14] = ({
                let gl_i15 = sBitfield;
                crate::gl!(gSprites[spriteId].data[gl_i15])
            }) & -16
                | state as i16 & 0xF
        )
    };
}
pub unsafe fn SetSurfBlob_DontSyncAnim(spriteId: u8, dontSync: u8) {
    {
        let gl_i16 = sBitfield;
        crate::gl!(
            gSprites[spriteId].data[gl_i16] = ({
                let gl_i17 = sBitfield;
                crate::gl!(gSprites[spriteId].data[gl_i17])
            }) & -241
                | (dontSync as i16 & 0xF) << 4
        )
    };
}
pub fn SetSurfBlob_PlayerOffset(spriteId: u8, hasOffset: u8, offset: i16) {
    {
        let gl_i18 = sBitfield;
        crate::gl!(
            gSprites[spriteId].data[gl_i18] = ({
                let gl_i19 = sBitfield;
                crate::gl!(gSprites[spriteId].data[gl_i19])
            }) & -3841
                | (hasOffset as i16 & 0xF) << 8
        )
    };
    {
        let gl_i20 = sPlayerOffset;
        crate::gl!(gSprites[spriteId].data[gl_i20] = offset)
    };
}
unsafe fn GetSurfBlob_BobState(sprite: *mut Sprite) -> u8 {
    (*sprite).data[sBitfield] as u8 & 0xF
}
unsafe fn GetSurfBlob_DontSyncAnim(sprite: *mut Sprite) -> u8 {
    (((*sprite).data[sBitfield] as i32 & 0xF0) >> 4) as u8
}
unsafe fn GetSurfBlob_HasPlayerOffset(sprite: *mut Sprite) -> u8 {
    (((*sprite).data[sBitfield] as i32 & 0xF00) >> 8) as u8
}
pub unsafe fn UpdateSurfBlobFieldEffect(sprite: *mut Sprite) {
    let playerObj: *mut ObjectEvent = {
        let gl_i1 = (*sprite).data[sPlayerObjId];
        crate::gl!(&raw gObjectEvents[gl_i1])
    };
    let playerSprite: *mut Sprite = {
        let gl_i21 = (*playerObj).spriteId;
        crate::gl!(&raw gSprites[gl_i21])
    };
    SynchronizeSurfAnim(playerObj, sprite);
    SynchronizeSurfPosition(playerObj, sprite);
    UpdateBobbingEffect(playerObj, playerSprite, sprite);
    (*sprite).oam.set_priority((*playerSprite).oam.priority());
}
unsafe fn SynchronizeSurfAnim(playerObj: *mut ObjectEvent, sprite: *mut Sprite) {
    let mut surfBlobDirectionAnims: CArray<u8, 9> = zeroed();
    surfBlobDirectionAnims[0] = 0;
    surfBlobDirectionAnims[1] = 0;
    surfBlobDirectionAnims[2] = 1;
    surfBlobDirectionAnims[3] = 2;
    surfBlobDirectionAnims[4] = 3;
    surfBlobDirectionAnims[5] = 0;
    surfBlobDirectionAnims[6] = 0;
    surfBlobDirectionAnims[7] = 1;
    surfBlobDirectionAnims[8] = 1;
    if GetSurfBlob_DontSyncAnim(sprite) == 0 {
        StartSpriteAnimIfDifferent(
            sprite,
            surfBlobDirectionAnims[(*playerObj).movementDirection()],
        );
    }
}
pub unsafe fn SynchronizeSurfPosition(playerObj: *mut ObjectEvent, sprite: *mut Sprite) {
    let mut i: u8 = 0;
    let mut x: i16 = (*playerObj).currentCoords.x;
    let mut y: i16 = (*playerObj).currentCoords.y;
    let spriteY: i32 = (*sprite).y2 as i32;
    if spriteY == 0 && (x != (*sprite).data[6] || y != (*sprite).data[7]) {
        (*sprite).data[sIntervalIdx] = 0;
        (*sprite).data[6] = x;
        (*sprite).data[7] = y;
        i = DIR_SOUTH;
        while i <= DIR_EAST {
            MoveCoords(i, &raw mut x, &raw mut y);
            if MapGridGetElevationAt(x as i32, y as i32) == ELEVATION_DEFAULT {
                (*sprite).data[sIntervalIdx] += 1;
                break;
            }
            i += 1;
            x = (*sprite).data[6];
            y = (*sprite).data[7];
        }
    }
}
unsafe fn UpdateBobbingEffect(
    playerObj: *mut ObjectEvent,
    playerSprite: *mut Sprite,
    sprite: *mut Sprite,
) {
    let intervals: CArray<u16, 2> = CArray([3, 7]);
    let bobState: u8 = GetSurfBlob_BobState(sprite);
    if bobState != BOB_NONE {
        if ({
            (*sprite).data[4] += 1;
            (*sprite).data[4]
        }) as u16 as i32
            & intervals[(*sprite).data[sIntervalIdx]] as i32
            == 0
        {
            (*sprite).y2 += (*sprite).data[sVelocity];
        }
        if (*sprite).data[4] as i32 & 15 == 0 {
            (*sprite).data[sVelocity] = -(*sprite).data[sVelocity];
        }
        if bobState != BOB_JUST_MON {
            if GetSurfBlob_HasPlayerOffset(sprite) == 0 {
                (*playerSprite).y2 = (*sprite).y2;
            } else {
                (*playerSprite).y2 = (*sprite).data[sPlayerOffset] + (*sprite).y2;
            }
            (*sprite).x = (*playerSprite).x;
            (*sprite).y = (*playerSprite).y + 8;
        }
    }
}
pub unsafe fn StartUnderwaterSurfBlobBobbing(blobSpriteId: u8) -> u8 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (&raw const (*(&raw const crate::sprite::gDummySpriteTemplate).cast::<SpriteTemplate>()))
            .cast_mut(),
        0,
        0,
        255,
    );
    let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
    (*sprite).callback = Some(SpriteCB_UnderwaterSurfBlob);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[sSpriteId] = blobSpriteId as i16;
    (*sprite).data[sBobY] = 1;
    spriteId
}
pub(crate) unsafe fn SpriteCB_UnderwaterSurfBlob(sprite: *mut Sprite) {
    let blobSprite: *mut Sprite = {
        let gl_i22 = (*sprite).data[sSpriteId];
        crate::gl!(&raw gSprites[gl_i22])
    };
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] += 1;
        t1
    }) as i32
        & 3
        == 0
    {
        (*blobSprite).y2 += (*sprite).data[sBobY];
    }
    if (*sprite).data[2] as i32 & 15 == 0 {
        (*sprite).data[sBobY] = -(*sprite).data[sBobY];
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Dust() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        12,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[9],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).data[sJumpElevation] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[sJumpFldEff] = FLDEFF_DUST as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_SandPile() -> u32 {
    let objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[objectEventId]);
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[29],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority((*gSprites.as_ptr())[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[1] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[2]) as i16;
        (*sprite).data[3] = {
            let gl_i23 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i23].x)
        };
        (*sprite).data[4] = {
            let gl_i24 = (*objectEvent).spriteId;
            crate::gl!(gSprites[gl_i24].y)
        };
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 2;
        SeekSpriteAnim(sprite, 2);
    }
    0
}
pub unsafe fn UpdateSandPileFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || (*gObjectEvents.as_ptr())[objectEventId].inSandPile() == 0
    {
        FieldEffectStop(sprite, FLDEFF_SAND_PILE);
    } else {
        let parentY: i16 = {
            let gl_i25 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(gSprites[gl_i25].y)
        };
        let parentX: i16 = {
            let gl_i26 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(gSprites[gl_i26].x)
        };
        if parentX != (*sprite).data[3] || parentY != (*sprite).data[4] {
            (*sprite).data[3] = parentX;
            (*sprite).data[4] = parentY;
            if (*sprite).animEnded() != 0 {
                StartSpriteAnim(sprite, 0);
            }
        }
        (*sprite).x = parentX;
        (*sprite).y = parentY;
        (*sprite).subpriority = {
            let gl_i27 = crate::gl!(gObjectEvents[objectEventId].spriteId);
            crate::gl!(gSprites[gl_i27].subpriority)
        };
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Bubbles() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        0,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[34],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        82,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(1);
    }
    0
}
pub unsafe fn UpdateBubblesFieldEffect(sprite: *mut Sprite) {
    (*sprite).data[0] += 128;
    (*sprite).data[0] &= 256;
    (*sprite).y -= (*sprite).data[0] >> 8;
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    if (*sprite).invisible() != 0 || (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_BUBBLES);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_BerryTreeGrowthSparkle() -> u32 {
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        4,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[22],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        crate::gl!(gFieldEffectArguments[2]) as u8,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[3]) as u16);
        (*sprite).oam.set_paletteNum(5);
        (*sprite).data[sWaitFldEff] = FLDEFF_BERRY_TREE_GROWTH_SPARKLE as i16;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn ShowTreeDisguiseFieldEffect() -> u32 {
    ShowDisguiseFieldEffect(FLDEFF_TREE_DISGUISE, FLDEFFOBJ_TREE_DISGUISE, 4)
}
#[unsafe(no_mangle)]
pub unsafe fn ShowMountainDisguiseFieldEffect() -> u32 {
    ShowDisguiseFieldEffect(FLDEFF_MOUNTAIN_DISGUISE, FLDEFFOBJ_MOUNTAIN_DISGUISE, 3)
}
#[unsafe(no_mangle)]
pub unsafe fn ShowSandDisguiseFieldEffect() -> u32 {
    ShowDisguiseFieldEffect(FLDEFF_SAND_DISGUISE, FLDEFFOBJ_SAND_DISGUISE, 2)
}
unsafe fn ShowDisguiseFieldEffect(fldEff: u8, fldEffObj: u8, paletteNum: u8) -> u32 {
    let mut spriteId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        crate::gl!(gFieldEffectArguments[0]) as u8,
        crate::gl!(gFieldEffectArguments[1]) as u8,
        crate::gl!(gFieldEffectArguments[2]) as u8,
        &raw mut spriteId,
    ) != 0
    {
        FieldEffectActiveListRemove(fldEff);
        return MAX_SPRITES as u32;
    }
    spriteId = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[fldEffObj],
        0,
        0,
        0,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = crate::gl!(&raw gSprites[spriteId]);
        (*sprite).set_coordOffsetEnabled((*sprite).coordOffsetEnabled() + 1);
        (*sprite).oam.set_paletteNum(paletteNum as u16);
        (*sprite).data[1] = fldEff as i16;
        (*sprite).data[2] = crate::gl!(gFieldEffectArguments[0]) as i16;
        (*sprite).data[3] = crate::gl!(gFieldEffectArguments[1]) as i16;
        (*sprite).data[4] = crate::gl!(gFieldEffectArguments[2]) as i16;
    }
    spriteId as u32
}
pub unsafe fn UpdateDisguiseFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[2] as u8,
        (*sprite).data[3] as u8,
        (*sprite).data[4] as u8,
        &raw mut objectEventId,
    ) != 0
    {
        FieldEffectStop(sprite, (*sprite).data[1] as u8);
    }
    let graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo(crate::gl!(gObjectEvents[objectEventId].graphicsId));
    let linkedSprite: *mut Sprite = {
        let gl_i28 = crate::gl!(gObjectEvents[objectEventId].spriteId);
        crate::gl!(&raw gSprites[gl_i28])
    };
    (*sprite).set_invisible((*linkedSprite).invisible());
    (*sprite).x = (*linkedSprite).x;
    (*sprite).y = ((*graphicsInfo).height >> 1) + (*linkedSprite).y - 16;
    (*sprite).subpriority = (*linkedSprite).subpriority - 1;
    if (*sprite).data[0] == 1 {
        (*sprite).data[0] += 1;
        StartSpriteAnim(sprite, 1);
    }
    if (*sprite).data[0] == 2 && (*sprite).animEnded() != 0 {
        (*sprite).data[sReadyToEnd] = TRUE as i16;
    }
    if (*sprite).data[0] == 3 {
        FieldEffectStop(sprite, (*sprite).data[1] as u8);
    }
}
pub unsafe fn StartRevealDisguise(objectEvent: *mut ObjectEvent) {
    if (*objectEvent).directionSequenceIndex == 1 {
        {
            let gl_i29 = (*objectEvent).fieldEffectSpriteId;
            crate::gl!(gSprites[gl_i29].data[0] += 1)
        };
    }
}
pub unsafe fn UpdateRevealDisguise(objectEvent: *mut ObjectEvent) -> u8 {
    if (*objectEvent).directionSequenceIndex == 2 {
        return TRUE;
    }
    if (*objectEvent).directionSequenceIndex == 0 {
        return TRUE;
    }
    let sprite: *mut Sprite = {
        let gl_i30 = (*objectEvent).fieldEffectSpriteId;
        crate::gl!(&raw gSprites[gl_i30])
    };
    if (*sprite).data[sReadyToEnd] != 0 {
        (*objectEvent).directionSequenceIndex = 2;
        (*sprite).data[0] += 1;
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_Sparkle() -> u32 {
    crate::gl!(gFieldEffectArguments[0] += MAP_OFFSET);
    crate::gl!(gFieldEffectArguments[1] += MAP_OFFSET);
    SetSpritePosToOffsetMapCoords(
        crate::gl!(&raw gFieldEffectArguments[0]) as *mut i16,
        crate::gl!(&raw gFieldEffectArguments[1]) as *mut i16,
        8,
        8,
    );
    let spriteId: u8 = CreateSpriteAtEnd(
        (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
            .cast::<CArray<*mut SpriteTemplate, 0>>())[35],
        crate::gl!(gFieldEffectArguments[0]) as i16,
        crate::gl!(gFieldEffectArguments[1]) as i16,
        82,
    );
    if spriteId != MAX_SPRITES {
        (*gSprites.as_ptr())[spriteId]
            .oam
            .set_priority(crate::gl!(gFieldEffectArguments[2]) as u16);
        (*gSprites.as_ptr())[spriteId].set_coordOffsetEnabled(TRUE as u16);
    }
    0
}
pub unsafe fn UpdateSparkleFieldEffect(sprite: *mut Sprite) {
    if (*sprite).data[sFinished] == 0 && (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).data[sFinished] += 1;
    }
    if (*sprite).data[sFinished] != 0
        && ({
            (*sprite).data[sEndTimer] += 1;
            (*sprite).data[sEndTimer]
        }) > 34
    {
        FieldEffectStop(sprite, FLDEFF_SPARKLE);
    }
}
unsafe fn InitRayquazaForFigure8Anim(sprite: *mut Sprite) {
    (*sprite).data[sAnimCounter] = 0;
    (*sprite).data[sAnimState] = 0;
}
unsafe fn AnimateRayquazaInFigure8(sprite: *mut Sprite) -> u8 {
    let mut finished: u8 = FALSE;
    match (*sprite).data[sAnimState] {
        0 => {
            (*sprite).x2 += GetFigure8XOffset((*sprite).data[sAnimCounter]);
            (*sprite).y2 += GetFigure8YOffset((*sprite).data[sAnimCounter]);
        }
        1 => {
            (*sprite).x2 -= GetFigure8XOffset(71 - (*sprite).data[sAnimCounter]);
            (*sprite).y2 += GetFigure8YOffset(71 - (*sprite).data[sAnimCounter]);
        }
        2 => {
            (*sprite).x2 -= GetFigure8XOffset((*sprite).data[sAnimCounter]);
            (*sprite).y2 += GetFigure8YOffset((*sprite).data[sAnimCounter]);
        }
        3 => {
            (*sprite).x2 += GetFigure8XOffset(71 - (*sprite).data[sAnimCounter]);
            (*sprite).y2 += GetFigure8YOffset(71 - (*sprite).data[sAnimCounter]);
        }
        _ => {}
    }
    SetGpuReg(REG_OFFSET_BG0HOFS, ((*sprite).x2 as u16).wrapping_neg());
    if ({
        (*sprite).data[sAnimCounter] += 1;
        (*sprite).data[sAnimCounter]
    }) == FIGURE_8_LENGTH
    {
        (*sprite).data[sAnimCounter] = 0;
        (*sprite).data[sAnimState] += 1;
    }
    if (*sprite).data[sAnimState] == 4 {
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        finished = TRUE;
    }
    finished
}
pub unsafe fn UpdateRayquazaSpotlightEffect(sprite: *mut Sprite) {
    match (*sprite).data[2] {
        0 => {
            SetGpuReg(REG_OFFSET_BG0VOFS, 120 - ((*sprite).data[0] / 3) as u16);
            if (*sprite).data[0] == 96 {
                for i in 0..3u8 {
                    for j in 12..18u8 {
                        *(0x600f800_usize as *mut u16).at(i as i32 * 32 + j as i32) =
                            0xBFF4 + i as u16 * 6 + j as u16 + 1;
                    }
                }
            }
            if (*sprite).data[0] > 311 {
                (*sprite).data[2] = 1;
                (*sprite).data[0] = 0;
            }
        }
        1 => {
            (*sprite).y = ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sprite).data[0] / 3]
                >> 2)
                + (*sprite).data[sStartY];
            if (*sprite).data[0] == 189 {
                (*sprite).data[2] = 2;
                (*sprite).data[sCounter] = 0;
                (*sprite).data[0] = 0;
            }
        }
        2 => {
            if (*sprite).data[0] == 60 {
                (*sprite).data[sCounter] += 1;
                (*sprite).data[0] = 0;
            }
            if (*sprite).data[sCounter] == 7 {
                (*sprite).data[sCounter] = 0;
                (*sprite).data[2] = 3;
            }
        }
        3 => {
            if (*sprite).y2 == 0 {
                (*sprite).data[0] = 0;
                (*sprite).data[2] += 1;
            }
            if (*sprite).data[0] == 5 {
                (*sprite).data[0] = 0;
                if (*sprite).y2 > 0 {
                    (*sprite).y2 -= 1;
                } else {
                    (*sprite).y2 += 1;
                }
            }
        }
        4 => {
            if (*sprite).data[0] == 60 {
                (*sprite).data[2] = 5;
                (*sprite).data[0] = 0;
                (*sprite).data[sCounter] = 0;
            }
        }
        5 => {
            InitRayquazaForFigure8Anim(sprite);
            (*sprite).data[2] = 6;
            (*sprite).data[0] = 0;
        }
        6 => {
            if AnimateRayquazaInFigure8(sprite) != 0 {
                (*sprite).data[0] = 0;
                if ({
                    (*sprite).data[sCounter] += 1;
                    (*sprite).data[sCounter]
                }) <= 2
                {
                    InitRayquazaForFigure8Anim(sprite);
                } else {
                    (*sprite).data[sCounter] = 0;
                    (*sprite).data[2] = 7;
                }
            }
        }
        7 => {
            if (*sprite).data[0] == 30 {
                (*sprite).data[2] = 8;
                (*sprite).data[0] = 0;
            }
        }
        8 => {
            for i in 0..15u8 {
                for j in 12..18u8 {
                    *(0x600f800_usize as *mut u16).at(i as i32 * 32 + j as i32) = 0;
                }
            }
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            FieldEffectStop(sprite, FLDEFF_RAYQUAZA_SPOTLIGHT);
        }
        _ => {}
    }
    if (*sprite).data[2] == 1 {
        if (*sprite).data[sMoveTimer] as i32 & 7 == 0 {
            (*sprite).y2 += (*sprite).data[sVelocity];
        }
        if (*sprite).data[sMoveTimer] as i32 & 15 == 0 {
            (*sprite).data[sVelocity] = -(*sprite).data[sVelocity];
        }
        (*sprite).data[sMoveTimer] += 1;
    }
    (*sprite).data[0] += 1;
}
pub unsafe fn UpdateJumpImpactEffect(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, (*sprite).data[sJumpFldEff] as u8);
    } else {
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        SetObjectSubpriorityByElevation((*sprite).data[sJumpElevation] as u8, sprite, 0);
    }
}
pub unsafe fn WaitFieldEffectSpriteAnim(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, (*sprite).data[sWaitFldEff] as u8);
    } else {
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
unsafe fn UpdateGrassFieldEffectSubpriority(sprite: *mut Sprite, elevation: u8, subpriority: u8) {
    let mut var: i16 = 0;
    let mut xhi: i16 = 0;
    let mut lyhi: i16 = 0;
    let mut yhi: i16 = 0;
    let mut ylo: i16 = 0;
    SetObjectSubpriorityByElevation(elevation, sprite, subpriority);
    for i in 0..OBJECT_EVENTS_COUNT {
        let objectEvent: *mut ObjectEvent = crate::gl!(&raw gObjectEvents[i]);
        if (*objectEvent).active() != 0 {
            let graphicsInfo: *mut ObjectEventGraphicsInfo =
                GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
            let linkedSprite: *mut Sprite = {
                let gl_i31 = (*objectEvent).spriteId;
                crate::gl!(&raw gSprites[gl_i31])
            };
            xhi = (*sprite).x + (*sprite).centerToCornerVecX as i16;
            var = (*sprite).x - (*sprite).centerToCornerVecX as i16;
            if xhi < (*linkedSprite).x && var > (*linkedSprite).x {
                lyhi = (*linkedSprite).y + (*linkedSprite).centerToCornerVecY as i16;
                var = (*linkedSprite).y;
                ylo = (*sprite).y - (*sprite).centerToCornerVecY as i16;
                yhi = ylo + (*linkedSprite).centerToCornerVecY as i16;
                if (lyhi < yhi || lyhi < ylo)
                    && var > yhi
                    && (*sprite).subpriority <= (*linkedSprite).subpriority
                {
                    (*sprite).subpriority = (*linkedSprite).subpriority + 2;
                    break;
                }
            }
        }
    }
}
