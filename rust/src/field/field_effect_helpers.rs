//! Translated from `src/field_effect_helpers.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sShadowEffectTemplateIds gShadowVerticalOffsets gFadeFootprintsTireTracksFuncs gAshFieldEffectFuncs sFigure8XOffsets sFigure8YOffsets

const OBJ_EVENT_PAL_TAG_NONE: u16 = 4607;

static gAshFieldEffectFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Sprite)>, 3>> =
    Table((&raw const crate::data::field_effect_helpers::gAshFieldEffectFuncs).cast());
static gFadeFootprintsTireTracksFuncs: Table<CArray<Option<unsafe extern "C" fn(*mut Sprite)>, 2>> =
    Table((&raw const crate::data::field_effect_helpers::gFadeFootprintsTireTracksFuncs).cast());
static gShadowVerticalOffsets: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::field_effect_helpers::gShadowVerticalOffsets).cast());
static sShadowEffectTemplateIds: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_effect_helpers::sShadowEffectTemplateIds).cast());

unsafe extern "C" {
    static mut gCamera: Camera;
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static gDummySpriteAnimTable: CArray<*mut AnimCmd, 0>;
    static gDummySpriteTemplate: SpriteTemplate;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static gFieldEffectObjectTemplatePointers: CArray<*mut SpriteTemplate, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static gReflectionEffectPaletteMap: CArray<u8, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    fn CreateCopySpriteAt(a0: *mut Sprite, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn ElevationToPriority(a0: u8) -> u8;
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut Sprite, a1: u8);
    fn GetFigure8XOffset(a0: i16) -> i16;
    fn GetFigure8YOffset(a0: i16) -> i16;
    fn GetObjectEventGraphicsInfo(a0: u8) -> *mut ObjectEventGraphicsInfo;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetObjectPaletteTag(a0: u8) -> u16;
    fn LoadPlayerObjectReflectionPalette(a0: u16, a1: u8);
    fn LoadSpecialObjectReflectionPalette(a0: u16, a1: u8);
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MetatileBehavior_GetBridgeType(a0: u8) -> u8;
    fn MetatileBehavior_IsLongGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsPokeGrass(a0: u8) -> u8;
    fn MetatileBehavior_IsReflective(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableWaterOrUnderwater(a0: u8) -> u8;
    fn MetatileBehavior_IsTallGrass(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn PatchObjectPalette(a0: u16, a1: u8);
    fn PlaySE(a0: u16);
    fn SeekSpriteAnim(a0: *mut Sprite, a1: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetObjectSubpriorityByElevation(a0: u8, a1: *mut Sprite, a2: u8);
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UpdateObjectEventSpriteInvisibility(a0: *mut Sprite, a1: u8);
    fn UpdateSpritePaletteWithWeather(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpReflection(
    objectEvent: *mut ObjectEvent,
    sprite: *mut Sprite,
    stillReflection: u8,
) {
    let mut reflectionSprite: *mut Sprite = null_mut();
    reflectionSprite = &raw mut gSprites[CreateCopySpriteAt(sprite, (*sprite).x, (*sprite).y, 152)];
    (*reflectionSprite).callback = Some(UpdateObjectReflectionSprite);
    (*reflectionSprite).oam.set_priority(3);
    (*reflectionSprite)
        .oam
        .set_paletteNum(gReflectionEffectPaletteMap[(*reflectionSprite).oam.paletteNum()] as u16);
    (*reflectionSprite).set_usingSheet(TRUE as u16);
    (*reflectionSprite).anims = gDummySpriteAnimTable.as_ptr().cast_mut();
    StartSpriteAnim(reflectionSprite, 0);
    (*reflectionSprite).affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    (*reflectionSprite).set_affineAnimBeginning(TRUE as u16);
    (*reflectionSprite).set_subspriteMode(SUBSPRITES_OFF);
    (*reflectionSprite).data[0] = (*sprite).data[0];
    (*reflectionSprite).data[1] = (*objectEvent).localId as i16;
    (*reflectionSprite).data[7] = stillReflection as i16;
    LoadObjectReflectionPalette(objectEvent, reflectionSprite);
    if stillReflection == 0 {
        (*reflectionSprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    }
}
pub(crate) unsafe extern "C" fn GetReflectionVerticalOffset(objectEvent: *mut ObjectEvent) -> i16 {
    return (*GetObjectEventGraphicsInfo((*objectEvent).graphicsId)).height - 2;
}
pub(crate) unsafe extern "C" fn LoadObjectReflectionPalette(
    objectEvent: *mut ObjectEvent,
    reflectionSprite: *mut Sprite,
) {
    let mut bridgeType: u8 = 0;
    let mut bridgeReflectionVerticalOffsets: CArray<u16, 3> = zeroed();
    bridgeReflectionVerticalOffsets[0] = 12;
    bridgeReflectionVerticalOffsets[1] = 28;
    bridgeReflectionVerticalOffsets[2] = 44;
    (*reflectionSprite).data[2] = 0;
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
        (*reflectionSprite).data[2] = bridgeReflectionVerticalOffsets[bridgeType as i32 - 1] as i16;
        LoadObjectHighBridgeReflectionPalette(
            objectEvent,
            (*reflectionSprite).oam.paletteNum() as u8,
        );
    } else {
        LoadObjectRegularReflectionPalette(objectEvent, (*reflectionSprite).oam.paletteNum() as u8);
    }
}
pub(crate) unsafe extern "C" fn LoadObjectRegularReflectionPalette(
    objectEvent: *mut ObjectEvent,
    paletteIndex: u8,
) {
    let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
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
pub(crate) unsafe extern "C" fn LoadObjectHighBridgeReflectionPalette(
    objectEvent: *mut ObjectEvent,
    paletteNum: u8,
) {
    let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
    if (*graphicsInfo).reflectionPaletteTag != OBJ_EVENT_PAL_TAG_NONE {
        PatchObjectPalette((*graphicsInfo).reflectionPaletteTag, paletteNum);
        UpdateSpritePaletteWithWeather(paletteNum);
    }
}
pub(crate) unsafe extern "C" fn UpdateObjectReflectionSprite(reflectionSprite: *mut Sprite) {
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[(*reflectionSprite).data[0]];
    let mut mainSprite: *mut Sprite = &raw mut gSprites[(*objectEvent).spriteId];
    if (*objectEvent).active() == 0
        || (*objectEvent).hasReflection() == 0
        || (*objectEvent).localId as i16 != (*reflectionSprite).data[1]
    {
        (*reflectionSprite).set_inUse(FALSE as u16);
    } else {
        (*reflectionSprite)
            .oam
            .set_paletteNum(gReflectionEffectPaletteMap[(*mainSprite).oam.paletteNum()] as u16);
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
            + (*reflectionSprite).data[2];
        (*reflectionSprite).centerToCornerVecX = (*mainSprite).centerToCornerVecX;
        (*reflectionSprite).centerToCornerVecY = (*mainSprite).centerToCornerVecY;
        (*reflectionSprite).x2 = (*mainSprite).x2;
        (*reflectionSprite).y2 = -(*mainSprite).y2;
        (*reflectionSprite).set_coordOffsetEnabled((*mainSprite).coordOffsetEnabled());
        if (*objectEvent).hideReflection() == TRUE as u32 {
            (*reflectionSprite).set_invisible(TRUE as u16);
        }
        if (*reflectionSprite).data[7] == FALSE as i16 {
            (*reflectionSprite).oam.set_matrixNum(0);
            if (*mainSprite).oam.matrixNum() & ST_OAM_HFLIP != 0 {
                (*reflectionSprite).oam.set_matrixNum(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWarpArrowSprite() -> u8 {
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[8], 0, 0, 82);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).oam.set_priority(1);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).set_invisible(TRUE as u16);
    }
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteInvisible(spriteId: u8) {
    gSprites[spriteId].set_invisible(TRUE as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowWarpArrowSprite(spriteId: u8, direction: u8, x: i16, y: i16) {
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    if (*sprite).invisible() != 0 || (*sprite).data[0] != x || (*sprite).data[1] != y {
        let mut x2: i16 = 0;
        let mut y2: i16 = 0;
        SetSpritePosToMapCoords(x, y, &raw mut x2, &raw mut y2);
        sprite = &raw mut gSprites[spriteId];
        (*sprite).x = x2 + 8;
        (*sprite).y = y2 + 8;
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).data[0] = x;
        (*sprite).data[1] = y;
        StartSpriteAnim(sprite, direction - 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Shadow() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
        GetObjectEventGraphicsInfo(gObjectEvents[objectEventId].graphicsId);
    let mut spriteId: u8 = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[sShadowEffectTemplateIds[(*graphicsInfo).shadowSize()]],
        0,
        0,
        148,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
        gSprites[spriteId].data[0] = gFieldEffectArguments[0] as i16;
        gSprites[spriteId].data[1] = gFieldEffectArguments[1] as i16;
        gSprites[spriteId].data[2] = gFieldEffectArguments[2] as i16;
        gSprites[spriteId].data[3] = ((*graphicsInfo).height >> 1)
            - gShadowVerticalOffsets[(*graphicsInfo).shadowSize()] as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateShadowFieldEffect(sprite: *mut Sprite) {
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
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
        let mut linkedSprite: *mut Sprite = &raw mut gSprites[(*objectEvent).spriteId];
        (*sprite).oam.set_priority((*linkedSprite).oam.priority());
        (*sprite).x = (*linkedSprite).x;
        (*sprite).y = (*linkedSprite).y + (*sprite).data[3];
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
pub unsafe extern "C" fn FldEff_TallGrass() -> u32 {
    let mut spriteId: u8 = 0;
    let mut x: i16 = gFieldEffectArguments[0] as i16;
    let mut y: i16 = gFieldEffectArguments[1] as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    spriteId = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[4], x, y, 0);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = gFieldEffectArguments[0] as i16;
        (*sprite).data[2] = gFieldEffectArguments[1] as i16;
        (*sprite).data[3] = gFieldEffectArguments[4] as i16;
        (*sprite).data[4] = gFieldEffectArguments[5] as i16;
        (*sprite).data[5] = gFieldEffectArguments[6] as i16;
        if gFieldEffectArguments[7] != 0 {
            SeekSpriteAnim(sprite, 4);
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTallGrassFieldEffect(sprite: *mut Sprite) {
    let mut metatileBehavior: u8 = 0;
    let mut localId: u8 = 0;
    let mut objectEventId: u8 = 0;
    let mut mapNum: u8 = ((*sprite).data[5] >> 8) as u8;
    let mut mapGroup: u8 = (*sprite).data[5] as u8;
    if gCamera.active() != 0
        && ((*gSaveBlock1Ptr).location.mapNum as i32 != mapNum as i32
            || (*gSaveBlock1Ptr).location.mapGroup as i32 != mapGroup as i32)
    {
        (*sprite).data[1] -= gCamera.x as i16;
        (*sprite).data[2] -= gCamera.y as i16;
        (*sprite).data[5] = ((*gSaveBlock1Ptr).location.mapNum as u8 as i16) << 8
            | (*gSaveBlock1Ptr).location.mapGroup as u8 as i16;
    }
    localId = ((*sprite).data[3] >> 8) as u8;
    mapNum = (*sprite).data[3] as u8;
    mapGroup = (*sprite).data[4] as u8;
    metatileBehavior =
        MapGridGetMetatileBehaviorAt((*sprite).data[1] as i32, (*sprite).data[2] as i32) as u8;
    if TryGetObjectEventIdByLocalIdAndMap(localId, mapNum, mapGroup, &raw mut objectEventId) != 0
        || MetatileBehavior_IsTallGrass(metatileBehavior) == 0
        || (*sprite).data[7] != 0 && (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, FLDEFF_TALL_GRASS);
    } else {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
        if ((*objectEvent).currentCoords.x != (*sprite).data[1]
            || (*objectEvent).currentCoords.y != (*sprite).data[2])
            && ((*objectEvent).previousCoords.x != (*sprite).data[1]
                || (*objectEvent).previousCoords.y != (*sprite).data[2])
        {
            (*sprite).data[7] = TRUE as i16;
        }
        metatileBehavior = 0;
        if (*sprite).animCmdIndex == 0 {
            metatileBehavior = 4;
        }
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        UpdateGrassFieldEffectSubpriority(sprite, (*sprite).data[0] as u8, metatileBehavior);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpTallGrass() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        12,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[10],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = FLDEFF_JUMP_TALL_GRASS as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FindTallGrassFieldEffectSpriteId(
    localId: u8,
    mapNum: u8,
    mapGroup: u8,
    x: i16,
    y: i16,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_SPRITES {
        if gSprites[i].inUse() != 0 {
            let mut sprite: *mut Sprite = &raw mut gSprites[i];
            if (*sprite).callback
                == Some(UpdateTallGrassFieldEffect as unsafe extern "C" fn(*mut Sprite))
                && (x == (*sprite).data[1] && y == (*sprite).data[2])
                && localId == ((*sprite).data[3] >> 8) as u8
                && mapNum as i32 == (*sprite).data[3] as i32 & 0xFF
                && mapGroup as i16 == (*sprite).data[4]
            {
                return i;
            }
        }
        i += 1;
    }
    return MAX_SPRITES;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_LongGrass() -> u32 {
    let mut spriteId: u8 = 0;
    let mut x: i16 = gFieldEffectArguments[0] as i16;
    let mut y: i16 = gFieldEffectArguments[1] as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    spriteId = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[15], x, y, 0);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(ElevationToPriority(gFieldEffectArguments[2] as u8) as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = gFieldEffectArguments[0] as i16;
        (*sprite).data[2] = gFieldEffectArguments[1] as i16;
        (*sprite).data[3] = gFieldEffectArguments[4] as i16;
        (*sprite).data[4] = gFieldEffectArguments[5] as i16;
        (*sprite).data[5] = gFieldEffectArguments[6] as i16;
        if gFieldEffectArguments[7] != 0 {
            SeekSpriteAnim(sprite, 6);
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateLongGrassFieldEffect(sprite: *mut Sprite) {
    let mut metatileBehavior: u8 = 0;
    let mut localId: u8 = 0;
    let mut objectEventId: u8 = 0;
    let mut mapNum: u8 = ((*sprite).data[5] >> 8) as u8;
    let mut mapGroup: u8 = (*sprite).data[5] as u8;
    if gCamera.active() != 0
        && ((*gSaveBlock1Ptr).location.mapNum as i32 != mapNum as i32
            || (*gSaveBlock1Ptr).location.mapGroup as i32 != mapGroup as i32)
    {
        (*sprite).data[1] -= gCamera.x as i16;
        (*sprite).data[2] -= gCamera.y as i16;
        (*sprite).data[5] = ((*gSaveBlock1Ptr).location.mapNum as u8 as i16) << 8
            | (*gSaveBlock1Ptr).location.mapGroup as u8 as i16;
    }
    localId = ((*sprite).data[3] >> 8) as u8;
    mapNum = (*sprite).data[3] as u8;
    mapGroup = (*sprite).data[4] as u8;
    metatileBehavior =
        MapGridGetMetatileBehaviorAt((*sprite).data[1] as i32, (*sprite).data[2] as i32) as u8;
    if TryGetObjectEventIdByLocalIdAndMap(localId, mapNum, mapGroup, &raw mut objectEventId) != 0
        || MetatileBehavior_IsLongGrass(metatileBehavior) == 0
        || (*sprite).data[7] != 0 && (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, FLDEFF_LONG_GRASS);
    } else {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
        if ((*objectEvent).currentCoords.x != (*sprite).data[1]
            || (*objectEvent).currentCoords.y != (*sprite).data[2])
            && ((*objectEvent).previousCoords.x != (*sprite).data[1]
                || (*objectEvent).previousCoords.y != (*sprite).data[2])
        {
            (*sprite).data[7] = TRUE as i16;
        }
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        UpdateGrassFieldEffectSubpriority(sprite, (*sprite).data[0] as u8, 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpLongGrass() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[16],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = FLDEFF_JUMP_LONG_GRASS as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_ShortGrass() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[30], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(gSprites[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = gFieldEffectArguments[0] as i16;
        (*sprite).data[1] = gFieldEffectArguments[1] as i16;
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).data[3] = gSprites[(*objectEvent).spriteId].x;
        (*sprite).data[4] = gSprites[(*objectEvent).spriteId].y;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateShortGrassFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || gObjectEvents[objectEventId].inShortGrass() == 0
    {
        FieldEffectStop(sprite, FLDEFF_SHORT_GRASS);
    } else {
        let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo(gObjectEvents[objectEventId].graphicsId);
        let mut linkedSprite: *mut Sprite =
            &raw mut gSprites[gObjectEvents[objectEventId].spriteId];
        let mut parentY: i16 = (*linkedSprite).y;
        let mut parentX: i16 = (*linkedSprite).x;
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
pub unsafe extern "C" fn FldEff_SandFootprints() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[11],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[7] = FLDEFF_SAND_FOOTPRINTS;
        StartSpriteAnim(sprite, gFieldEffectArguments[4] as u8);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_DeepSandFootprints() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[23],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[7] = FLDEFF_DEEP_SAND_FOOTPRINTS;
        StartSpriteAnim(sprite, gFieldEffectArguments[4] as u8);
    }
    return spriteId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_BikeTireTracks() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[27],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[7] = FLDEFF_BIKE_TIRE_TRACKS as i16;
        StartSpriteAnim(sprite, gFieldEffectArguments[4] as u8);
    }
    return spriteId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFootprintsTireTracksFieldEffect(sprite: *mut Sprite) {
    gFadeFootprintsTireTracksFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn FadeFootprintsTireTracks_Step0(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 40
    {
        (*sprite).data[0] = 1;
    }
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
}
pub(crate) unsafe extern "C" fn FadeFootprintsTireTracks_Step1(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).invisible() ^ 1);
    (*sprite).data[1] += 1;
    UpdateObjectEventSpriteInvisibility(sprite, (*sprite).invisible() as u8);
    if (*sprite).data[1] > 56 {
        FieldEffectStop(sprite, (*sprite).data[7] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Splash() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[13], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut linkedSprite: *mut Sprite = null_mut();
        let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        linkedSprite = &raw mut gSprites[(*objectEvent).spriteId];
        (*sprite).oam.set_priority((*linkedSprite).oam.priority());
        (*sprite).data[0] = gFieldEffectArguments[0] as i16;
        (*sprite).data[1] = gFieldEffectArguments[1] as i16;
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 4;
        PlaySE(SE_PUDDLE);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSplashFieldEffect(sprite: *mut Sprite) {
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
        (*sprite).x = gSprites[gObjectEvents[objectEventId].spriteId].x;
        (*sprite).y = gSprites[gObjectEvents[objectEventId].spriteId].y;
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpSmallSplash() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        12,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[14],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = FLDEFF_JUMP_SMALL_SPLASH as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_JumpBigSplash() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[12],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = FLDEFF_JUMP_BIG_SPLASH as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_FeetInFlowingWater() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[13], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).callback = Some(UpdateFeetInFlowingWaterFieldEffect);
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(gSprites[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = gFieldEffectArguments[0] as i16;
        (*sprite).data[1] = gFieldEffectArguments[1] as i16;
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).data[3] = -1;
        (*sprite).data[4] = -1;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 4;
        StartSpriteAnim(sprite, 1);
    }
    return 0;
}
pub(crate) unsafe extern "C" fn UpdateFeetInFlowingWaterFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || gObjectEvents[objectEventId].inShallowFlowingWater() == 0
    {
        FieldEffectStop(sprite, FLDEFF_FEET_IN_FLOWING_WATER);
    } else {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
        let mut linkedSprite: *mut Sprite = &raw mut gSprites[(*objectEvent).spriteId];
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
pub unsafe extern "C" fn FldEff_Ripple() -> u32 {
    let mut spriteId: u8 = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[5],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = FLDEFF_RIPPLE as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HotSpringsWater() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[31], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(gSprites[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = gFieldEffectArguments[0] as i16;
        (*sprite).data[1] = gFieldEffectArguments[1] as i16;
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).data[3] = gSprites[(*objectEvent).spriteId].x;
        (*sprite).data[4] = gSprites[(*objectEvent).spriteId].y;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateHotSpringsWaterFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || gObjectEvents[objectEventId].inHotSprings() == 0
    {
        FieldEffectStop(sprite, FLDEFF_HOT_SPRINGS_WATER);
    } else {
        let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo(gObjectEvents[objectEventId].graphicsId);
        let mut linkedSprite: *mut Sprite =
            &raw mut gSprites[gObjectEvents[objectEventId].spriteId];
        (*sprite).x = (*linkedSprite).x;
        (*sprite).y = ((*graphicsInfo).height >> 1) + (*linkedSprite).y - 8;
        (*sprite).subpriority = (*linkedSprite).subpriority - 1;
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedGrass() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[17],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = FLDEFF_UNUSED_GRASS;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedGrass2() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[18],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = FLDEFF_UNUSED_GRASS_2;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UnusedSand() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[19],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = FLDEFF_UNUSED_SAND;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_WaterSurfacing() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[20],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = FLDEFF_WATER_SURFACING;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartAshFieldEffect(x: i16, y: i16, metatileId: u16, delay: i16) {
    gFieldEffectArguments[0] = x as i32;
    gFieldEffectArguments[1] = y as i32;
    gFieldEffectArguments[2] = 82;
    gFieldEffectArguments[3] = 1;
    gFieldEffectArguments[4] = metatileId as i32;
    gFieldEffectArguments[5] = delay as i32;
    FieldEffectStart(FLDEFF_ASH);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Ash() -> u32 {
    let mut spriteId: u8 = 0;
    let mut x: i16 = gFieldEffectArguments[0] as i16;
    let mut y: i16 = gFieldEffectArguments[1] as i16;
    SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[6],
        x,
        y,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[1] = gFieldEffectArguments[0] as i16;
        (*sprite).data[2] = gFieldEffectArguments[1] as i16;
        (*sprite).data[3] = gFieldEffectArguments[4] as i16;
        (*sprite).data[4] = gFieldEffectArguments[5] as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateAshFieldEffect(sprite: *mut Sprite) {
    gAshFieldEffectFuncs[(*sprite).data[0]].unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_Wait(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).set_animPaused(TRUE);
    if ({
        (*sprite).data[4] -= 1;
        (*sprite).data[4]
    }) == 0
    {
        (*sprite).data[0] = 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_Show(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    (*sprite).set_animPaused(FALSE);
    MapGridSetMetatileIdAt(
        (*sprite).data[1] as i32,
        (*sprite).data[2] as i32,
        (*sprite).data[3] as u16,
    );
    CurrentMapDrawMetatileAt((*sprite).data[1] as i32, (*sprite).data[2] as i32);
    gObjectEvents[gPlayerAvatar.objectEventId].set_triggerGroundEffectsOnMove(TRUE as u32);
    (*sprite).data[0] = 2;
}
pub(crate) unsafe extern "C" fn UpdateAshFieldEffect_End(sprite: *mut Sprite) {
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_ASH);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SurfBlob() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[7],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        150,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_paletteNum(0);
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).data[3] = -1;
        (*sprite).data[6] = -1;
        (*sprite).data[7] = -1;
    }
    FieldEffectActiveListRemove(FLDEFF_SURF_BLOB);
    return spriteId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_BobState(spriteId: u8, state: u8) {
    gSprites[spriteId].data[0] = gSprites[spriteId].data[0] & -16 | state as i16 & 0xF;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_DontSyncAnim(spriteId: u8, dontSync: u8) {
    gSprites[spriteId].data[0] = gSprites[spriteId].data[0] & -241 | (dontSync as i16 & 0xF) << 4;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSurfBlob_PlayerOffset(spriteId: u8, hasOffset: u8, offset: i16) {
    gSprites[spriteId].data[0] = gSprites[spriteId].data[0] & -3841 | (hasOffset as i16 & 0xF) << 8;
    gSprites[spriteId].data[1] = offset;
}
pub(crate) unsafe extern "C" fn GetSurfBlob_BobState(sprite: *mut Sprite) -> u8 {
    return (*sprite).data[0] as u8 & 0xF;
}
pub(crate) unsafe extern "C" fn GetSurfBlob_DontSyncAnim(sprite: *mut Sprite) -> u8 {
    return (((*sprite).data[0] as i32 & 0xF0) >> 4) as u8;
}
pub(crate) unsafe extern "C" fn GetSurfBlob_HasPlayerOffset(sprite: *mut Sprite) -> u8 {
    return (((*sprite).data[0] as i32 & 0xF00) >> 8) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSurfBlobFieldEffect(sprite: *mut Sprite) {
    let mut playerObj: *mut ObjectEvent = &raw mut gObjectEvents[(*sprite).data[2]];
    let mut playerSprite: *mut Sprite = &raw mut gSprites[(*playerObj).spriteId];
    SynchronizeSurfAnim(playerObj, sprite);
    SynchronizeSurfPosition(playerObj, sprite);
    UpdateBobbingEffect(playerObj, playerSprite, sprite);
    (*sprite).oam.set_priority((*playerSprite).oam.priority());
}
pub(crate) unsafe extern "C" fn SynchronizeSurfAnim(
    playerObj: *mut ObjectEvent,
    sprite: *mut Sprite,
) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SynchronizeSurfPosition(playerObj: *mut ObjectEvent, sprite: *mut Sprite) {
    let mut i: u8 = 0;
    let mut x: i16 = (*playerObj).currentCoords.x;
    let mut y: i16 = (*playerObj).currentCoords.y;
    let mut spriteY: i32 = (*sprite).y2 as i32;
    if spriteY == 0 && (x != (*sprite).data[6] || y != (*sprite).data[7]) {
        (*sprite).data[5] = 0;
        (*sprite).data[6] = x;
        (*sprite).data[7] = y;
        i = DIR_SOUTH;
        while i <= DIR_EAST {
            MoveCoords(i, &raw mut x, &raw mut y);
            if MapGridGetElevationAt(x as i32, y as i32) == ELEVATION_DEFAULT {
                (*sprite).data[5] += 1;
                break;
            }
            i += 1;
            x = (*sprite).data[6];
            y = (*sprite).data[7];
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBobbingEffect(
    playerObj: *mut ObjectEvent,
    playerSprite: *mut Sprite,
    sprite: *mut Sprite,
) {
    let mut intervals: CArray<u16, 2> = CArray([3, 7]);
    let mut bobState: u8 = GetSurfBlob_BobState(sprite);
    if bobState != BOB_NONE {
        if ({
            (*sprite).data[4] += 1;
            (*sprite).data[4]
        }) as u16 as i32
            & intervals[(*sprite).data[5]] as i32
            == 0
        {
            (*sprite).y2 += (*sprite).data[3];
        }
        if (*sprite).data[4] as i32 & 15 == 0 {
            (*sprite).data[3] = -(*sprite).data[3];
        }
        if bobState != BOB_JUST_MON {
            if GetSurfBlob_HasPlayerOffset(sprite) == 0 {
                (*playerSprite).y2 = (*sprite).y2;
            } else {
                (*playerSprite).y2 = (*sprite).data[1] + (*sprite).y2;
            }
            (*sprite).x = (*playerSprite).x;
            (*sprite).y = (*playerSprite).y + 8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartUnderwaterSurfBlobBobbing(blobSpriteId: u8) -> u8 {
    let mut spriteId: u8 =
        CreateSpriteAtEnd((&raw const gDummySpriteTemplate).cast_mut(), 0, 0, 255);
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).callback = Some(SpriteCB_UnderwaterSurfBlob);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[0] = blobSpriteId as i16;
    (*sprite).data[1] = 1;
    return spriteId;
}
pub(crate) unsafe extern "C" fn SpriteCB_UnderwaterSurfBlob(sprite: *mut Sprite) {
    let mut blobSprite: *mut Sprite = &raw mut gSprites[(*sprite).data[0]];
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] += 1;
        t1
    }) as i32
        & 3
        == 0
    {
        (*blobSprite).y2 += (*sprite).data[1];
    }
    if (*sprite).data[2] as i32 & 15 == 0 {
        (*sprite).data[1] = -(*sprite).data[1];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Dust() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        12,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[9],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        0,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).data[0] = gFieldEffectArguments[2] as i16;
        (*sprite).data[1] = FLDEFF_DUST as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SandPile() -> u32 {
    let mut objectEventId: u8 = GetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
    );
    let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    let mut spriteId: u8 = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[29], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
            GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite)
            .oam
            .set_priority(gSprites[(*objectEvent).spriteId].oam.priority());
        (*sprite).data[0] = gFieldEffectArguments[0] as i16;
        (*sprite).data[1] = gFieldEffectArguments[1] as i16;
        (*sprite).data[2] = gFieldEffectArguments[2] as i16;
        (*sprite).data[3] = gSprites[(*objectEvent).spriteId].x;
        (*sprite).data[4] = gSprites[(*objectEvent).spriteId].y;
        (*sprite).y2 = ((*graphicsInfo).height >> 1) - 2;
        SeekSpriteAnim(sprite, 2);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSandPileFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objectEventId,
    ) != 0
        || gObjectEvents[objectEventId].inSandPile() == 0
    {
        FieldEffectStop(sprite, FLDEFF_SAND_PILE);
    } else {
        let mut parentY: i16 = gSprites[gObjectEvents[objectEventId].spriteId].y;
        let mut parentX: i16 = gSprites[gObjectEvents[objectEventId].spriteId].x;
        if parentX != (*sprite).data[3] || parentY != (*sprite).data[4] {
            (*sprite).data[3] = parentX;
            (*sprite).data[4] = parentY;
            if (*sprite).animEnded() != 0 {
                StartSpriteAnim(sprite, 0);
            }
        }
        (*sprite).x = parentX;
        (*sprite).y = parentY;
        (*sprite).subpriority = gSprites[gObjectEvents[objectEventId].spriteId].subpriority;
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Bubbles() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        0,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[34],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        82,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(1);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateBubblesFieldEffect(sprite: *mut Sprite) {
    (*sprite).data[0] += 128;
    (*sprite).data[0] &= 256;
    (*sprite).y -= (*sprite).data[0] >> 8;
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    if (*sprite).invisible() != 0 || (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, FLDEFF_BUBBLES);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_BerryTreeGrowthSparkle() -> u32 {
    let mut spriteId: u8 = 0;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        4,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[22],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        gFieldEffectArguments[2] as u8,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).oam.set_priority(gFieldEffectArguments[3] as u16);
        (*sprite).oam.set_paletteNum(5);
        (*sprite).data[0] = FLDEFF_BERRY_TREE_GROWTH_SPARKLE as i16;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowTreeDisguiseFieldEffect() -> u32 {
    return ShowDisguiseFieldEffect(FLDEFF_TREE_DISGUISE, FLDEFFOBJ_TREE_DISGUISE, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowMountainDisguiseFieldEffect() -> u32 {
    return ShowDisguiseFieldEffect(FLDEFF_MOUNTAIN_DISGUISE, FLDEFFOBJ_MOUNTAIN_DISGUISE, 3);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSandDisguiseFieldEffect() -> u32 {
    return ShowDisguiseFieldEffect(FLDEFF_SAND_DISGUISE, FLDEFFOBJ_SAND_DISGUISE, 2);
}
pub(crate) unsafe extern "C" fn ShowDisguiseFieldEffect(
    fldEff: u8,
    fldEffObj: u8,
    paletteNum: u8,
) -> u32 {
    let mut spriteId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        gFieldEffectArguments[0] as u8,
        gFieldEffectArguments[1] as u8,
        gFieldEffectArguments[2] as u8,
        &raw mut spriteId,
    ) != 0
    {
        FieldEffectActiveListRemove(fldEff);
        return MAX_SPRITES as u32;
    }
    spriteId = CreateSpriteAtEnd(gFieldEffectObjectTemplatePointers[fldEffObj], 0, 0, 0);
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        (*sprite).set_coordOffsetEnabled((*sprite).coordOffsetEnabled() + 1);
        (*sprite).oam.set_paletteNum(paletteNum as u16);
        (*sprite).data[1] = fldEff as i16;
        (*sprite).data[2] = gFieldEffectArguments[0] as i16;
        (*sprite).data[3] = gFieldEffectArguments[1] as i16;
        (*sprite).data[4] = gFieldEffectArguments[2] as i16;
    }
    return spriteId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateDisguiseFieldEffect(sprite: *mut Sprite) {
    let mut objectEventId: u8 = 0;
    let mut graphicsInfo: *mut ObjectEventGraphicsInfo = null_mut();
    let mut linkedSprite: *mut Sprite = null_mut();
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[2] as u8,
        (*sprite).data[3] as u8,
        (*sprite).data[4] as u8,
        &raw mut objectEventId,
    ) != 0
    {
        FieldEffectStop(sprite, (*sprite).data[1] as u8);
    }
    graphicsInfo = GetObjectEventGraphicsInfo(gObjectEvents[objectEventId].graphicsId);
    linkedSprite = &raw mut gSprites[gObjectEvents[objectEventId].spriteId];
    (*sprite).set_invisible((*linkedSprite).invisible());
    (*sprite).x = (*linkedSprite).x;
    (*sprite).y = ((*graphicsInfo).height >> 1) + (*linkedSprite).y - 16;
    (*sprite).subpriority = (*linkedSprite).subpriority - 1;
    if (*sprite).data[0] == 1 {
        (*sprite).data[0] += 1;
        StartSpriteAnim(sprite, 1);
    }
    if (*sprite).data[0] == 2 && (*sprite).animEnded() != 0 {
        (*sprite).data[7] = TRUE as i16;
    }
    if (*sprite).data[0] == 3 {
        FieldEffectStop(sprite, (*sprite).data[1] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartRevealDisguise(objectEvent: *mut ObjectEvent) {
    if (*objectEvent).directionSequenceIndex == 1 {
        gSprites[(*objectEvent).fieldEffectSpriteId].data[0] += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRevealDisguise(objectEvent: *mut ObjectEvent) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    if (*objectEvent).directionSequenceIndex == 2 {
        return TRUE;
    }
    if (*objectEvent).directionSequenceIndex == 0 {
        return TRUE;
    }
    sprite = &raw mut gSprites[(*objectEvent).fieldEffectSpriteId];
    if (*sprite).data[7] != 0 {
        (*objectEvent).directionSequenceIndex = 2;
        (*sprite).data[0] += 1;
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Sparkle() -> u32 {
    let mut spriteId: u8 = 0;
    gFieldEffectArguments[0] += MAP_OFFSET;
    gFieldEffectArguments[1] += MAP_OFFSET;
    SetSpritePosToOffsetMapCoords(
        &raw mut gFieldEffectArguments[0] as *mut i16,
        &raw mut gFieldEffectArguments[1] as *mut i16,
        8,
        8,
    );
    spriteId = CreateSpriteAtEnd(
        gFieldEffectObjectTemplatePointers[35],
        gFieldEffectArguments[0] as i16,
        gFieldEffectArguments[1] as i16,
        82,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId]
            .oam
            .set_priority(gFieldEffectArguments[2] as u16);
        gSprites[spriteId].set_coordOffsetEnabled(TRUE as u16);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateSparkleFieldEffect(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if (*sprite).animEnded() != 0 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).data[0] += 1;
        }
    }
    if (*sprite).data[0] != 0
        && ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 34
    {
        FieldEffectStop(sprite, FLDEFF_SPARKLE);
    }
}
pub(crate) unsafe extern "C" fn InitRayquazaForFigure8Anim(sprite: *mut Sprite) {
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 0;
}
pub(crate) unsafe extern "C" fn AnimateRayquazaInFigure8(sprite: *mut Sprite) -> u8 {
    let mut finished: u8 = FALSE;
    match (*sprite).data[7] {
        0 => {
            (*sprite).x2 += GetFigure8XOffset((*sprite).data[6]);
            (*sprite).y2 += GetFigure8YOffset((*sprite).data[6]);
        }
        1 => {
            (*sprite).x2 -= GetFigure8XOffset(71 - (*sprite).data[6]);
            (*sprite).y2 += GetFigure8YOffset(71 - (*sprite).data[6]);
        }
        2 => {
            (*sprite).x2 -= GetFigure8XOffset((*sprite).data[6]);
            (*sprite).y2 += GetFigure8YOffset((*sprite).data[6]);
        }
        3 => {
            (*sprite).x2 += GetFigure8XOffset(71 - (*sprite).data[6]);
            (*sprite).y2 += GetFigure8YOffset(71 - (*sprite).data[6]);
        }
        _ => {}
    }
    SetGpuReg(REG_OFFSET_BG0HOFS, ((*sprite).x2 as u16).wrapping_neg());
    if ({
        (*sprite).data[6] += 1;
        (*sprite).data[6]
    }) == FIGURE_8_LENGTH
    {
        (*sprite).data[6] = 0;
        (*sprite).data[7] += 1;
    }
    if (*sprite).data[7] == 4 {
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        finished = TRUE;
    }
    return finished;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRayquazaSpotlightEffect(sprite: *mut Sprite) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    match (*sprite).data[2] {
        0 => {
            SetGpuReg(REG_OFFSET_BG0VOFS, 120 - ((*sprite).data[0] / 3) as u16);
            if (*sprite).data[0] == 96 {
                i = 0;
                while i < 3 {
                    j = 12;
                    while j < 18 {
                        *(0x600f800 as usize as *mut u16).at(i as i32 * 32 + j as i32) =
                            0xBFF4 + i as u16 * 6 + j as u16 + 1;
                        j += 1;
                    }
                    i += 1;
                }
            }
            if (*sprite).data[0] > 311 {
                (*sprite).data[2] = 1;
                (*sprite).data[0] = 0;
            }
        }
        1 => {
            (*sprite).y = (gSineTable[(*sprite).data[0] / 3] >> 2) + (*sprite).data[4];
            if (*sprite).data[0] == 189 {
                (*sprite).data[2] = 2;
                (*sprite).data[5] = 0;
                (*sprite).data[0] = 0;
            }
        }
        2 => {
            if (*sprite).data[0] == 60 {
                (*sprite).data[5] += 1;
                (*sprite).data[0] = 0;
            }
            if (*sprite).data[5] == 7 {
                (*sprite).data[5] = 0;
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
                (*sprite).data[5] = 0;
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
                    (*sprite).data[5] += 1;
                    (*sprite).data[5]
                }) <= 2
                {
                    InitRayquazaForFigure8Anim(sprite);
                } else {
                    (*sprite).data[5] = 0;
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
            i = 0;
            while i < 15 {
                j = 12;
                while j < 18 {
                    *(0x600f800 as usize as *mut u16).at(i as i32 * 32 + j as i32) = 0;
                    j += 1;
                }
                i += 1;
            }
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            FieldEffectStop(sprite, FLDEFF_RAYQUAZA_SPOTLIGHT);
        }
        _ => {}
    }
    if (*sprite).data[2] == 1 {
        if (*sprite).data[1] as i32 & 7 == 0 {
            (*sprite).y2 += (*sprite).data[3];
        }
        if (*sprite).data[1] as i32 & 15 == 0 {
            (*sprite).data[3] = -(*sprite).data[3];
        }
        (*sprite).data[1] += 1;
    }
    (*sprite).data[0] += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateJumpImpactEffect(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, (*sprite).data[1] as u8);
    } else {
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
        SetObjectSubpriorityByElevation((*sprite).data[0] as u8, sprite, 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitFieldEffectSpriteAnim(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        FieldEffectStop(sprite, (*sprite).data[0] as u8);
    } else {
        UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    }
}
pub(crate) unsafe extern "C" fn UpdateGrassFieldEffectSubpriority(
    sprite: *mut Sprite,
    elevation: u8,
    subpriority: u8,
) {
    let mut i: u8 = 0;
    let mut var: i16 = 0;
    let mut xhi: i16 = 0;
    let mut lyhi: i16 = 0;
    let mut yhi: i16 = 0;
    let mut ylo: i16 = 0;
    SetObjectSubpriorityByElevation(elevation, sprite, subpriority);
    i = 0;
    while i < OBJECT_EVENTS_COUNT {
        let mut objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[i];
        if (*objectEvent).active() != 0 {
            let mut graphicsInfo: *mut ObjectEventGraphicsInfo =
                GetObjectEventGraphicsInfo((*objectEvent).graphicsId);
            let mut linkedSprite: *mut Sprite = &raw mut gSprites[(*objectEvent).spriteId];
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
        i += 1;
    }
}
