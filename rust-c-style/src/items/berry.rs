//! Translated from `src/berry.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBerryDescriptionPart1_Cheri sBerryDescriptionPart2_Cheri sBerryDescriptionPart1_Chesto sBerryDescriptionPart2_Chesto sBerryDescriptionPart1_Pecha sBerryDescriptionPart2_Pecha sBerryDescriptionPart1_Rawst sBerryDescriptionPart2_Rawst sBerryDescriptionPart1_Aspear sBerryDescriptionPart2_Aspear sBerryDescriptionPart1_Leppa sBerryDescriptionPart2_Leppa sBerryDescriptionPart1_Oran sBerryDescriptionPart2_Oran sBerryDescriptionPart1_Persim sBerryDescriptionPart2_Persim sBerryDescriptionPart1_Lum sBerryDescriptionPart2_Lum sBerryDescriptionPart1_Sitrus sBerryDescriptionPart2_Sitrus sBerryDescriptionPart1_Figy sBerryDescriptionPart2_Figy sBerryDescriptionPart1_Wiki sBerryDescriptionPart2_Wiki sBerryDescriptionPart1_Mago sBerryDescriptionPart2_Mago sBerryDescriptionPart1_Aguav sBerryDescriptionPart2_Aguav sBerryDescriptionPart1_Iapapa sBerryDescriptionPart2_Iapapa sBerryDescriptionPart1_Razz sBerryDescriptionPart2_Razz sBerryDescriptionPart1_Bluk sBerryDescriptionPart2_Bluk sBerryDescriptionPart1_Nanab sBerryDescriptionPart2_Nanab sBerryDescriptionPart1_Wepear sBerryDescriptionPart2_Wepear sBerryDescriptionPart1_Pinap sBerryDescriptionPart2_Pinap sBerryDescriptionPart1_Pomeg sBerryDescriptionPart2_Pomeg sBerryDescriptionPart1_Kelpsy sBerryDescriptionPart2_Kelpsy sBerryDescriptionPart1_Qualot sBerryDescriptionPart2_Qualot sBerryDescriptionPart1_Hondew sBerryDescriptionPart2_Hondew sBerryDescriptionPart1_Grepa sBerryDescriptionPart2_Grepa sBerryDescriptionPart1_Tamato sBerryDescriptionPart2_Tamato sBerryDescriptionPart1_Cornn sBerryDescriptionPart2_Cornn sBerryDescriptionPart1_Magost sBerryDescriptionPart2_Magost sBerryDescriptionPart1_Rabuta sBerryDescriptionPart2_Rabuta sBerryDescriptionPart1_Nomel sBerryDescriptionPart2_Nomel sBerryDescriptionPart1_Spelon sBerryDescriptionPart2_Spelon sBerryDescriptionPart1_Pamtre sBerryDescriptionPart2_Pamtre sBerryDescriptionPart1_Watmel sBerryDescriptionPart2_Watmel sBerryDescriptionPart1_Durin sBerryDescriptionPart2_Durin sBerryDescriptionPart1_Belue sBerryDescriptionPart2_Belue sBerryDescriptionPart1_Liechi sBerryDescriptionPart2_Liechi sBerryDescriptionPart1_Ganlon sBerryDescriptionPart2_Ganlon sBerryDescriptionPart1_Salac sBerryDescriptionPart2_Salac sBerryDescriptionPart1_Petaya sBerryDescriptionPart2_Petaya sBerryDescriptionPart1_Apicot sBerryDescriptionPart2_Apicot sBerryDescriptionPart1_Lansat sBerryDescriptionPart2_Lansat sBerryDescriptionPart1_Starf sBerryDescriptionPart2_Starf sBerryDescriptionPart1_Enigma sBerryDescriptionPart2_Enigma gBerries gBerryCrush_BerryData gBlankBerryTree

static gBerries: Table<CArray<Berry, 43>> = Table((&raw const crate::data::berry::gBerries).cast());
static gBlankBerryTree: Table<BerryTree> =
    Table((&raw const crate::data::berry::gBlankBerryTree).cast());

unsafe extern "C" {
    static BerryTreeScript: CArray<u8, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSelectedObjectEvent: u8;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_ItemId: u16;
    static mut gSpecialVar_LastTalked: u16;
    static mut gStringVar1: CArray<u8, 256>;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn CB2_ChooseBerry();
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn GetBerryCountString(a0: *mut u8, a1: *mut u8, a2: u32);
    fn GetCameraCoords(a0: *mut u16, a1: *mut u16);
    fn GetObjectEventBerryTreeId(a0: u8) -> u8;
    fn GetObjectEventScriptPointerPlayerFacing() -> *mut u8;
    fn IsBagPocketNonEmpty(a0: u8) -> u8;
    fn IsBerryTreeSparkling(a0: u8, a1: u8, a2: u8) -> u8;
    fn Random() -> u16;
    fn SetBerryTreeJustPicked(a0: u8, a1: u8, a2: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearEnigmaBerries() {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*gSaveBlock1Ptr).enigmaBerry as *mut c_void,
                0x100001a,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetEnigmaBerry(src: *mut u8) {
    let mut i: u32 = 0;
    let mut dest: *mut u8 = &raw mut (*gSaveBlock1Ptr).enigmaBerry as *mut u8;
    i = 0;
    while i < 52 {
        *dest.at(i) = *src.at(i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetEnigmaBerryChecksum(enigmaBerry: *mut EnigmaBerry) -> u32 {
    let mut i: u32 = 0;
    let mut checksum: u32 = 0;
    let mut dest: *mut u8 = null_mut();
    dest = enigmaBerry as *mut u8;
    checksum = 0;
    i = 0;
    while i < 48 {
        checksum += *dest.at(i) as u32;
        i += 1;
    }
    return checksum;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEnigmaBerryValid() -> u32 {
    if (*gSaveBlock1Ptr).enigmaBerry.berry.stageDuration == 0 {
        return FALSE as u32;
    }
    if (*gSaveBlock1Ptr).enigmaBerry.berry.maxYield == 0 {
        return FALSE as u32;
    }
    if GetEnigmaBerryChecksum(&raw mut (*gSaveBlock1Ptr).enigmaBerry)
        != (*gSaveBlock1Ptr).enigmaBerry.checksum
    {
        return FALSE as u32;
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryInfo(mut berry: u8) -> *mut Berry {
    if berry == 43 && IsEnigmaBerryValid() != 0 {
        return &raw mut (*gSaveBlock1Ptr).enigmaBerry.berry as *mut Berry;
    } else {
        if berry == BERRY_NONE || berry > 43 {
            berry = 1;
        }
        return (&raw const gBerries[berry as i32 - 1]).cast_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryTreeInfo(id: u8) -> *mut BerryTree {
    return &raw mut (*gSaveBlock1Ptr).berryTrees[id];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionWaterBerryTree() -> u32 {
    let mut tree: *mut BerryTree =
        GetBerryTreeInfo(GetObjectEventBerryTreeId(gSelectedObjectEvent));
    match (*tree).stage() {
        BERRY_STAGE_PLANTED => {
            (*tree).set_watered1(TRUE);
        }
        BERRY_STAGE_SPROUTED => {
            (*tree).set_watered2(TRUE);
        }
        BERRY_STAGE_TALLER => {
            (*tree).set_watered3(TRUE);
        }
        BERRY_STAGE_FLOWERING => {
            (*tree).set_watered4(TRUE);
        }
        _ => {
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerFacingEmptyBerryTreePatch() -> u8 {
    if GetObjectEventScriptPointerPlayerFacing() == BerryTreeScript.as_ptr().cast_mut()
        && GetStageByBerryTreeId(GetObjectEventBerryTreeId(gSelectedObjectEvent))
            == BERRY_STAGE_NO_BERRY
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryToWaterBerryTree() -> u8 {
    if GetObjectEventScriptPointerPlayerFacing() != BerryTreeScript.as_ptr().cast_mut() {
        return FALSE;
    } else {
        return ObjectEventInteractionWaterBerryTree() as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBerryTrees() {
    let mut i: i32 = 0;
    i = 0;
    while i < BERRY_TREES_COUNT {
        (*gSaveBlock1Ptr).berryTrees[i] = *gBlankBerryTree;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn BerryTreeGrow(tree: *mut BerryTree) -> u32 {
    if (*tree).stopGrowth() != 0 {
        return FALSE as u32;
    }
    'l1: {
        let sw1: u8 = (*tree).stage();
        let mut fall = false;
        if sw1 == BERRY_STAGE_NO_BERRY {
            fall = true;
            return FALSE as u32;
        }
        if sw1 == BERRY_STAGE_FLOWERING {
            fall = true;
            (*tree).berryYield = CalcBerryYield(tree);
        }
        if fall
            || sw1 == BERRY_STAGE_PLANTED
            || sw1 == BERRY_STAGE_SPROUTED
            || sw1 == BERRY_STAGE_TALLER
        {
            fall = true;
            (*tree).set_stage((*tree).stage() + 1);
            break 'l1;
        }
        if sw1 == BERRY_STAGE_BERRIES {
            fall = true;
            (*tree).set_watered1(0);
            (*tree).set_watered2(0);
            (*tree).set_watered3(0);
            (*tree).set_watered4(0);
            (*tree).berryYield = 0;
            (*tree).set_stage(BERRY_STAGE_SPROUTED);
            if ({
                (*tree).set_regrowthCount((*tree).regrowthCount() + 1);
                (*tree).regrowthCount()
            }) == 10
            {
                *tree = *gBlankBerryTree;
            }
            break 'l1;
        }
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BerryTreeTimeUpdate(minutes: i32) {
    let mut i: i32 = 0;
    let mut tree: *mut BerryTree = null_mut();
    i = 0;
    while i < BERRY_TREES_COUNT {
        tree = &raw mut (*gSaveBlock1Ptr).berryTrees[i];
        if (*tree).berry != 0 && (*tree).stage() != 0 && (*tree).stopGrowth() == 0 {
            if minutes >= GetStageDurationByBerryType((*tree).berry) as i32 * 71 {
                *tree = *gBlankBerryTree;
            } else {
                let mut time: i32 = minutes;
                while time != 0 {
                    if (*tree).minutesUntilNextStage as i32 > time {
                        (*tree).minutesUntilNextStage -= time as u16;
                        break;
                    }
                    time -= (*tree).minutesUntilNextStage as i32;
                    (*tree).minutesUntilNextStage = GetStageDurationByBerryType((*tree).berry);
                    if BerryTreeGrow(tree) == 0 {
                        break;
                    }
                    if (*tree).stage() == BERRY_STAGE_BERRIES {
                        (*tree).minutesUntilNextStage *= 4;
                    }
                }
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlantBerryTree(id: u8, berry: u8, stage: u8, allowGrowth: u8) {
    let mut tree: *mut BerryTree = GetBerryTreeInfo(id);
    *tree = *gBlankBerryTree;
    (*tree).berry = berry;
    (*tree).minutesUntilNextStage = GetStageDurationByBerryType(berry);
    (*tree).set_stage(stage);
    if stage == BERRY_STAGE_BERRIES {
        (*tree).berryYield = CalcBerryYield(tree);
        (*tree).minutesUntilNextStage *= 4;
    }
    if allowGrowth == 0 {
        (*tree).set_stopGrowth(TRUE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBerryTree(id: u8) {
    (*gSaveBlock1Ptr).berryTrees[id] = *gBlankBerryTree;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryTypeByBerryTreeId(id: u8) -> u8 {
    return (*gSaveBlock1Ptr).berryTrees[id].berry;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStageByBerryTreeId(id: u8) -> u8 {
    return (*gSaveBlock1Ptr).berryTrees[id].stage();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemIdToBerryType(item: u16) -> u8 {
    let mut berry: u16 = item - ITEM_CHERI_BERRY;
    if berry > 42 {
        return 1;
    } else {
        return item as u8 - ITEM_CHERI_BERRY as u8 + 1;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn BerryTypeToItemId(berry: u16) -> u16 {
    let mut item: u16 = berry - 1;
    if item > 42 {
        return ITEM_CHERI_BERRY;
    } else {
        return berry + ITEM_CHERI_BERRY - 1;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryNameByBerryType(berry: u8, mut string: *mut u8) {
    memcpy(
        string,
        (*GetBerryInfo(berry)).name.as_mut_ptr(),
        BERRY_NAME_LENGTH as u32,
    );
    *string.at(6) = EOS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryCountStringByBerryType(berry: u8, dest: *mut u8, berryCount: u32) {
    GetBerryCountString(dest, (*GetBerryInfo(berry)).name.as_mut_ptr(), berryCount);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllowBerryTreeGrowth(id: u8) {
    (*GetBerryTreeInfo(id)).set_stopGrowth(FALSE);
}
pub(crate) unsafe extern "C" fn BerryTreeGetNumStagesWatered(tree: *mut BerryTree) -> u8 {
    let mut count: u8 = 0;
    if (*tree).watered1() != 0 {
        count += 1;
    }
    if (*tree).watered2() != 0 {
        count += 1;
    }
    if (*tree).watered3() != 0 {
        count += 1;
    }
    if (*tree).watered4() != 0 {
        count += 1;
    }
    return count;
}
pub(crate) unsafe extern "C" fn GetNumStagesWateredByBerryTreeId(id: u8) -> u8 {
    return BerryTreeGetNumStagesWatered(GetBerryTreeInfo(id));
}
pub(crate) unsafe extern "C" fn CalcBerryYieldInternal(max: u16, min: u16, water: u8) -> u8 {
    let mut randMin: u32 = 0;
    let mut randMax: u32 = 0;
    let mut rand: u32 = 0;
    let mut extraYield: u32 = 0;
    if water == 0 {
        return min as u8;
    } else {
        randMin = (max as u32 - min as u32) * (water as u32 - 1);
        randMax = (max as u32 - min as u32) * water as u32;
        rand = randMin + rem_u32(Random() as u32, randMax - randMin + 1);
        if rand % 4 >= 2 {
            extraYield = rand / 4 + 1;
        } else {
            extraYield = rand / 4;
        }
        return extraYield as u8 + min as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CalcBerryYield(tree: *mut BerryTree) -> u8 {
    let mut berry: *mut Berry = GetBerryInfo((*tree).berry);
    let mut min: u8 = (*berry).minYield;
    let mut max: u8 = (*berry).maxYield;
    return CalcBerryYieldInternal(max as u16, min as u16, BerryTreeGetNumStagesWatered(tree));
}
pub(crate) unsafe extern "C" fn GetBerryCountByBerryTreeId(id: u8) -> u8 {
    return (*gSaveBlock1Ptr).berryTrees[id].berryYield;
}
pub(crate) unsafe extern "C" fn GetStageDurationByBerryType(berry: u8) -> u16 {
    return (*GetBerryInfo(berry)).stageDuration as u16 * 60;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryTreeData() {
    let mut id: u8 = 0;
    let mut berry: u8 = 0;
    let mut localId: u8 = 0;
    let mut group: u8 = 0;
    let mut num: u8 = 0;
    id = GetObjectEventBerryTreeId(gSelectedObjectEvent);
    berry = GetBerryTypeByBerryTreeId(id);
    AllowBerryTreeGrowth(id);
    localId = gSpecialVar_LastTalked as u8;
    num = (*gSaveBlock1Ptr).location.mapNum as u8;
    group = (*gSaveBlock1Ptr).location.mapGroup as u8;
    if IsBerryTreeSparkling(localId, num, group) != 0 {
        gSpecialVar_0x8004 = BERRY_STAGE_SPARKLING;
    } else {
        gSpecialVar_0x8004 = GetStageByBerryTreeId(id) as u16;
    }
    gSpecialVar_0x8005 = GetNumStagesWateredByBerryTreeId(id) as u16;
    gSpecialVar_0x8006 = GetBerryCountByBerryTreeId(id) as u16;
    GetBerryCountStringByBerryType(berry, gStringVar1.as_mut_ptr(), gSpecialVar_0x8006 as u32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryName() {
    let mut berryType: u8 =
        GetBerryTypeByBerryTreeId(GetObjectEventBerryTreeId(gSelectedObjectEvent));
    GetBerryNameByBerryType(berryType, gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionGetBerryCountString() {
    let mut treeId: u8 = GetObjectEventBerryTreeId(gSelectedObjectEvent);
    let mut berry: u8 = GetBerryTypeByBerryTreeId(treeId);
    let mut count: u8 = GetBerryCountByBerryTreeId(treeId);
    GetBerryCountStringByBerryType(berry, gStringVar1.as_mut_ptr(), count as u32);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bag_ChooseBerry() {
    SetMainCallback2(Some(CB2_ChooseBerry));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionPlantBerryTree() {
    let mut berry: u8 = ItemIdToBerryType(gSpecialVar_ItemId);
    PlantBerryTree(GetObjectEventBerryTreeId(gSelectedObjectEvent), berry, 1, 1);
    ObjectEventInteractionGetBerryTreeData();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionPickBerryTree() {
    let mut id: u8 = GetObjectEventBerryTreeId(gSelectedObjectEvent);
    let mut berry: u8 = GetBerryTypeByBerryTreeId(id);
    gSpecialVar_0x8004 = AddBagItem(
        BerryTypeToItemId(berry as u16),
        GetBerryCountByBerryTreeId(id) as u16,
    ) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventInteractionRemoveBerryTree() {
    RemoveBerryTree(GetObjectEventBerryTreeId(gSelectedObjectEvent));
    SetBerryTreeJustPicked(
        gSpecialVar_LastTalked as u8,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerHasBerries() -> u8 {
    return IsBagPocketNonEmpty(POCKET_BERRIES);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBerryTreesSeen() {
    let mut cam_left: i16 = 0;
    let mut cam_top: i16 = 0;
    let mut left: i16 = 0;
    let mut top: i16 = 0;
    let mut right: i16 = 0;
    let mut bottom: i16 = 0;
    let mut i: i32 = 0;
    GetCameraCoords(&raw mut cam_left as *mut u16, &raw mut cam_top as *mut u16);
    left = cam_left;
    top = cam_top + 3;
    right = cam_left + 14;
    bottom = top + 8;
    i = 0;
    while i < OBJECT_EVENTS_COUNT as i32 {
        if gObjectEvents[i].active() != 0
            && gObjectEvents[i].movementType == MOVEMENT_TYPE_BERRY_TREE_GROWTH
        {
            cam_left = gObjectEvents[i].currentCoords.x;
            cam_top = gObjectEvents[i].currentCoords.y;
            if left <= cam_left && cam_left <= right && top <= cam_top && cam_top <= bottom {
                AllowBerryTreeGrowth(gObjectEvents[i].trainerRange_berryTreeId);
            }
        }
        i += 1;
    }
}
