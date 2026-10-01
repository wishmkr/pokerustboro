//! Translated from `src/item.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sDummyDesc sMasterBallDesc sUltraBallDesc sGreatBallDesc sPokeBallDesc sSafariBallDesc sNetBallDesc sDiveBallDesc sNestBallDesc sRepeatBallDesc sTimerBallDesc sLuxuryBallDesc sPremierBallDesc sPotionDesc sAntidoteDesc sBurnHealDesc sIceHealDesc sAwakeningDesc sParalyzeHealDesc sFullRestoreDesc sMaxPotionDesc sHyperPotionDesc sSuperPotionDesc sFullHealDesc sReviveDesc sMaxReviveDesc sFreshWaterDesc sSodaPopDesc sLemonadeDesc sMoomooMilkDesc sEnergyPowderDesc sEnergyRootDesc sHealPowderDesc sRevivalHerbDesc sEtherDesc sMaxEtherDesc sElixirDesc sMaxElixirDesc sLavaCookieDesc sBlueFluteDesc sYellowFluteDesc sRedFluteDesc sBlackFluteDesc sWhiteFluteDesc sBerryJuiceDesc sSacredAshDesc sShoalSaltDesc sShoalShellDesc sRedShardDesc sBlueShardDesc sYellowShardDesc sGreenShardDesc sHPUpDesc sProteinDesc sIronDesc sCarbosDesc sCalciumDesc sRareCandyDesc sPPUpDesc sZincDesc sPPMaxDesc sGuardSpecDesc sDireHitDesc sXAttackDesc sXDefendDesc sXSpeedDesc sXAccuracyDesc sXSpecialDesc sPokeDollDesc sFluffyTailDesc sSuperRepelDesc sMaxRepelDesc sEscapeRopeDesc sRepelDesc sSunStoneDesc sMoonStoneDesc sFireStoneDesc sThunderStoneDesc sWaterStoneDesc sLeafStoneDesc sTinyMushroomDesc sBigMushroomDesc sPearlDesc sBigPearlDesc sStardustDesc sStarPieceDesc sNuggetDesc sHeartScaleDesc sOrangeMailDesc sHarborMailDesc sGlitterMailDesc sMechMailDesc sWoodMailDesc sWaveMailDesc sBeadMailDesc sShadowMailDesc sTropicMailDesc sDreamMailDesc sFabMailDesc sRetroMailDesc sCheriBerryDesc sChestoBerryDesc sPechaBerryDesc sRawstBerryDesc sAspearBerryDesc sLeppaBerryDesc sOranBerryDesc sPersimBerryDesc sLumBerryDesc sSitrusBerryDesc sFigyBerryDesc sWikiBerryDesc sMagoBerryDesc sAguavBerryDesc sIapapaBerryDesc sRazzBerryDesc sBlukBerryDesc sNanabBerryDesc sWepearBerryDesc sPinapBerryDesc sPomegBerryDesc sKelpsyBerryDesc sQualotBerryDesc sHondewBerryDesc sGrepaBerryDesc sTamatoBerryDesc sCornnBerryDesc sMagostBerryDesc sRabutaBerryDesc sNomelBerryDesc sSpelonBerryDesc sPamtreBerryDesc sWatmelBerryDesc sDurinBerryDesc sBelueBerryDesc sLiechiBerryDesc sGanlonBerryDesc sSalacBerryDesc sPetayaBerryDesc sApicotBerryDesc sLansatBerryDesc sStarfBerryDesc sEnigmaBerryDesc sBrightPowderDesc sWhiteHerbDesc sMachoBraceDesc sExpShareDesc sQuickClawDesc sSootheBellDesc sMentalHerbDesc sChoiceBandDesc sKingsRockDesc sSilverPowderDesc sAmuletCoinDesc sCleanseTagDesc sSoulDewDesc sDeepSeaToothDesc sDeepSeaScaleDesc sSmokeBallDesc sEverstoneDesc sFocusBandDesc sLuckyEggDesc sScopeLensDesc sMetalCoatDesc sLeftoversDesc sDragonScaleDesc sLightBallDesc sSoftSandDesc sHardStoneDesc sMiracleSeedDesc sBlackGlassesDesc sBlackBeltDesc sMagnetDesc sMysticWaterDesc sSharpBeakDesc sPoisonBarbDesc sNeverMeltIceDesc sSpellTagDesc sTwistedSpoonDesc sCharcoalDesc sDragonFangDesc sSilkScarfDesc sUpGradeDesc sShellBellDesc sSeaIncenseDesc sLaxIncenseDesc sLuckyPunchDesc sMetalPowderDesc sThickClubDesc sStickDesc sRedScarfDesc sBlueScarfDesc sPinkScarfDesc sGreenScarfDesc sYellowScarfDesc sMachBikeDesc sCoinCaseDesc sItemfinderDesc sOldRodDesc sGoodRodDesc sSuperRodDesc sSSTicketDesc sContestPassDesc sWailmerPailDesc sDevonGoodsDesc sSootSackDesc sBasementKeyDesc sAcroBikeDesc sPokeblockCaseDesc sLetterDesc sEonTicketDesc sRedOrbDesc sBlueOrbDesc sScannerDesc sGoGogglesDesc sMeteoriteDesc sRoom1KeyDesc sRoom2KeyDesc sRoom4KeyDesc sRoom6KeyDesc sStorageKeyDesc sRootFossilDesc sClawFossilDesc sDevonScopeDesc sTM01Desc sTM02Desc sTM03Desc sTM04Desc sTM05Desc sTM06Desc sTM07Desc sTM08Desc sTM09Desc sTM10Desc sTM11Desc sTM12Desc sTM13Desc sTM14Desc sTM15Desc sTM16Desc sTM17Desc sTM18Desc sTM19Desc sTM20Desc sTM21Desc sTM22Desc sTM23Desc sTM24Desc sTM25Desc sTM26Desc sTM27Desc sTM28Desc sTM29Desc sTM30Desc sTM31Desc sTM32Desc sTM33Desc sTM34Desc sTM35Desc sTM36Desc sTM37Desc sTM38Desc sTM39Desc sTM40Desc sTM41Desc sTM42Desc sTM43Desc sTM44Desc sTM45Desc sTM46Desc sTM47Desc sTM48Desc sTM49Desc sTM50Desc sHM01Desc sHM02Desc sHM03Desc sHM04Desc sHM05Desc sHM06Desc sHM07Desc sHM08Desc sOaksParcelDesc sPokeFluteDesc sSecretKeyDesc sBikeVoucherDesc sGoldTeethDesc sOldAmberDesc sCardKeyDesc sLiftKeyDesc sHelixFossilDesc sDomeFossilDesc sSilphScopeDesc sBicycleDesc sTownMapDesc sVSSeekerDesc sFameCheckerDesc sTMCaseDesc sBerryPouchDesc sTeachyTVDesc sTriPassDesc sRainbowPassDesc sTeaDesc sMysticTicketDesc sAuroraTicketDesc sPowderJarDesc sRubyDesc sSapphireDesc sMagmaEmblemDesc sOldSeaMapDesc gItems

static gItems: Table<CArray<Item, 377>> = Table((&raw const crate::data::item::gItems).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBagPockets: CArray<BagPocket, 5> = unsafe { zeroed() };

unsafe extern "C" {
    static gBerries: CArray<Berry, 0>;
    static mut gPyramidBagMenuState: PyramidBagMenuState;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_Result: u16;
    static gText_Berries: CArray<u8, 0>;
    static gText_Berry: CArray<u8, 0>;
    static gText_PokeBalls: CArray<u8, 0>;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn ApplyNewEncryptionKeyToHword(a0: *mut u16, a1: u32);
    fn CurMapIsSecretBase() -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn GetItemListPosition(a0: u8) -> u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn GetBagItemQuantity(quantity: *mut u16) -> u16 {
    return (*gSaveBlock2Ptr).encryptionKey as u16 ^ *quantity;
}
pub(crate) unsafe extern "C" fn SetBagItemQuantity(quantity: *mut u16, newValue: u16) {
    *quantity = newValue ^ (*gSaveBlock2Ptr).encryptionKey as u16;
}
pub(crate) unsafe extern "C" fn GetPCItemQuantity(quantity: *mut u16) -> u16 {
    return *quantity;
}
pub(crate) unsafe extern "C" fn SetPCItemQuantity(quantity: *mut u16, newValue: u16) {
    *quantity = newValue;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToBagItems(newKey: u32) {
    let mut pocket: u32 = 0;
    let mut item: u32 = 0;
    pocket = 0;
    while pocket < POCKETS_COUNT as u32 {
        item = 0;
        while item < gBagPockets[pocket].capacity as u32 {
            ApplyNewEncryptionKeyToHword(
                &raw mut (*gBagPockets[pocket].itemSlots.at(item)).quantity,
                newKey,
            );
            item += 1;
        }
        pocket += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToBagItems_(newKey: u32) {
    ApplyNewEncryptionKeyToBagItems(newKey);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBagItemsPointers() {
    gBagPockets[0].itemSlots = (*gSaveBlock1Ptr).bagPocket_Items.as_mut_ptr();
    gBagPockets[0].capacity = BAG_ITEMS_COUNT;
    gBagPockets[4].itemSlots = (*gSaveBlock1Ptr).bagPocket_KeyItems.as_mut_ptr();
    gBagPockets[4].capacity = BAG_KEYITEMS_COUNT;
    gBagPockets[1].itemSlots = (*gSaveBlock1Ptr).bagPocket_PokeBalls.as_mut_ptr();
    gBagPockets[1].capacity = BAG_POKEBALLS_COUNT;
    gBagPockets[2].itemSlots = (*gSaveBlock1Ptr).bagPocket_TMHM.as_mut_ptr();
    gBagPockets[2].capacity = BAG_TMHM_COUNT;
    gBagPockets[3].itemSlots = (*gSaveBlock1Ptr).bagPocket_Berries.as_mut_ptr();
    gBagPockets[3].capacity = BAG_BERRIES_COUNT;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemName(itemId: u16, dst: *mut u8) {
    StringCopy(dst, GetItemName(itemId));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemNameHandlePlural(itemId: u16, dst: *mut u8, quantity: u32) {
    if itemId == ITEM_POKE_BALL {
        if quantity < 2 {
            StringCopy(dst, GetItemName(ITEM_POKE_BALL));
        } else {
            StringCopy(dst, gText_PokeBalls.as_ptr().cast_mut());
        }
    } else {
        if itemId >= ITEM_CHERI_BERRY && itemId <= ITEM_ENIGMA_BERRY {
            GetBerryCountString(
                dst,
                gBerries[itemId as i32 - ITEM_CHERI_BERRY as i32]
                    .name
                    .as_ptr()
                    .cast_mut(),
                quantity,
            );
        } else {
            StringCopy(dst, GetItemName(itemId));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryCountString(dst: *mut u8, berryName: *mut u8, quantity: u32) {
    let mut berryString: *mut u8 = null_mut();
    let mut txtPtr: *mut u8 = null_mut();
    if quantity < 2 {
        berryString = gText_Berry.as_ptr().cast_mut();
    } else {
        berryString = gText_Berries.as_ptr().cast_mut();
    }
    txtPtr = StringCopy(dst, berryName);
    *txtPtr = CHAR_SPACE;
    StringCopy(txtPtr.at(1), berryString);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBagPocketNonEmpty(pocket: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < gBagPockets[pocket as i32 - 1].capacity {
        if (*gBagPockets[pocket as i32 - 1].itemSlots.at(i)).itemId != 0 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBagHasItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut pocket: u8 = 0;
    if GetItemPocket(itemId) == 0 {
        return FALSE;
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE
        || FlagGet(FLAG_STORING_ITEMS_IN_PYRAMID_BAG) == TRUE
    {
        return CheckPyramidBagHasItem(itemId, count);
    }
    pocket = GetItemPocket(itemId) - 1;
    i = 0;
    while i < gBagPockets[pocket].capacity {
        if (*gBagPockets[pocket].itemSlots.at(i)).itemId == itemId {
            let mut quantity: u16 = 0;
            quantity = GetBagItemQuantity(&raw mut (*gBagPockets[pocket].itemSlots.at(i)).quantity);
            if quantity >= count {
                return TRUE;
            }
            count -= quantity;
            if count == 0 {
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAtLeastOneBerry() -> u8 {
    let mut i: u16 = 0;
    i = ITEM_CHERI_BERRY;
    while i < ITEM_BRIGHT_POWDER {
        if CheckBagHasItem(i, 1) == 1 {
            gSpecialVar_Result = TRUE as u16;
            return TRUE;
        }
        i += 1;
    }
    gSpecialVar_Result = FALSE as u16;
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBagHasSpace(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut pocket: u8 = 0;
    let mut slotCapacity: u16 = 0;
    let mut ownedCount: u16 = 0;
    if GetItemPocket(itemId) == POCKET_NONE {
        return FALSE;
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE
        || FlagGet(FLAG_STORING_ITEMS_IN_PYRAMID_BAG) == TRUE
    {
        return CheckPyramidBagHasSpace(itemId, count);
    }
    pocket = GetItemPocket(itemId) - 1;
    if pocket != BERRIES_POCKET {
        slotCapacity = MAX_BAG_ITEM_CAPACITY;
    } else {
        slotCapacity = MAX_BERRY_CAPACITY;
    }
    i = 0;
    while i < gBagPockets[pocket].capacity {
        if (*gBagPockets[pocket].itemSlots.at(i)).itemId == itemId {
            ownedCount =
                GetBagItemQuantity(&raw mut (*gBagPockets[pocket].itemSlots.at(i)).quantity);
            if ownedCount as i32 + count as i32 <= slotCapacity as i32 {
                return TRUE;
            }
            if pocket == TMHM_POCKET || pocket == BERRIES_POCKET {
                return FALSE;
            }
            count -= slotCapacity - ownedCount;
            if count == 0 {
                break;
            }
        }
        i += 1;
    }
    if count > 0 {
        i = 0;
        while i < gBagPockets[pocket].capacity {
            if (*gBagPockets[pocket].itemSlots.at(i)).itemId == 0 {
                if count > slotCapacity {
                    if pocket == TMHM_POCKET || pocket == BERRIES_POCKET {
                        return FALSE;
                    }
                    count -= slotCapacity;
                } else {
                    count = 0;
                    break;
                }
            }
            i += 1;
        }
        if count > 0 {
            return FALSE;
        }
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    if GetItemPocket(itemId) == POCKET_NONE {
        return FALSE;
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE
        || FlagGet(FLAG_STORING_ITEMS_IN_PYRAMID_BAG) == TRUE
    {
        return AddPyramidBagItem(itemId, count);
    } else {
        let mut itemPocket: *mut BagPocket = null_mut();
        let mut newItems: *mut ItemSlot = null_mut();
        let mut slotCapacity: u16 = 0;
        let mut ownedCount: u16 = 0;
        let mut pocket: u8 = GetItemPocket(itemId) - 1;
        itemPocket = &raw mut gBagPockets[pocket];
        newItems = AllocZeroed((*itemPocket).capacity as u32 * 4) as *mut ItemSlot;
        memcpy(
            newItems as *mut u8,
            (*itemPocket).itemSlots as *mut u8,
            (*itemPocket).capacity as u32 * 4,
        );
        if pocket != BERRIES_POCKET {
            slotCapacity = MAX_BAG_ITEM_CAPACITY;
        } else {
            slotCapacity = MAX_BERRY_CAPACITY;
        }
        i = 0;
        while i < (*itemPocket).capacity {
            if (*newItems.at(i)).itemId == itemId {
                ownedCount = GetBagItemQuantity(&raw mut (*newItems.at(i)).quantity);
                if ownedCount as i32 + count as i32 <= slotCapacity as i32 {
                    SetBagItemQuantity(&raw mut (*newItems.at(i)).quantity, ownedCount + count);
                    memcpy(
                        (*itemPocket).itemSlots as *mut u8,
                        newItems as *mut u8,
                        (*itemPocket).capacity as u32 * 4,
                    );
                    Free(newItems as *mut c_void);
                    return TRUE;
                } else {
                    if pocket == TMHM_POCKET || pocket == BERRIES_POCKET {
                        Free(newItems as *mut c_void);
                        return FALSE;
                    } else {
                        count -= slotCapacity - ownedCount;
                        SetBagItemQuantity(&raw mut (*newItems.at(i)).quantity, slotCapacity);
                        if count == 0 {
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        if count > 0 {
            i = 0;
            while i < (*itemPocket).capacity {
                if (*newItems.at(i)).itemId == ITEM_NONE {
                    (*newItems.at(i)).itemId = itemId;
                    if count > slotCapacity {
                        if pocket == TMHM_POCKET || pocket == BERRIES_POCKET {
                            Free(newItems as *mut c_void);
                            return FALSE;
                        }
                        count -= slotCapacity;
                        SetBagItemQuantity(&raw mut (*newItems.at(i)).quantity, slotCapacity);
                    } else {
                        SetBagItemQuantity(&raw mut (*newItems.at(i)).quantity, count);
                        count = 0;
                        break;
                    }
                }
                i += 1;
            }
            if count > 0 {
                Free(newItems as *mut c_void);
                return FALSE;
            }
        }
        memcpy(
            (*itemPocket).itemSlots as *mut u8,
            newItems as *mut u8,
            (*itemPocket).capacity as u32 * 4,
        );
        Free(newItems as *mut c_void);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut totalQuantity: u16 = 0;
    if GetItemPocket(itemId) == POCKET_NONE || itemId == ITEM_NONE {
        return FALSE;
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE
        || FlagGet(FLAG_STORING_ITEMS_IN_PYRAMID_BAG) == TRUE
    {
        return RemovePyramidBagItem(itemId, count);
    } else {
        let mut pocket: u8 = 0;
        let mut var: u8 = 0;
        let mut ownedCount: u16 = 0;
        let mut itemPocket: *mut BagPocket = null_mut();
        pocket = GetItemPocket(itemId) - 1;
        itemPocket = &raw mut gBagPockets[pocket];
        i = 0;
        while i < (*itemPocket).capacity {
            if (*(*itemPocket).itemSlots.at(i)).itemId == itemId {
                totalQuantity +=
                    GetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(i)).quantity);
            }
            i += 1;
        }
        if totalQuantity < count {
            return FALSE;
        }
        if CurMapIsSecretBase() == TRUE {
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_BAG,
            );
            VarSet(VAR_SECRET_BASE_LAST_ITEM_USED, itemId);
        }
        var = GetItemListPosition(pocket);
        if (*itemPocket).capacity > var && (*(*itemPocket).itemSlots.at(var)).itemId == itemId {
            ownedCount = GetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(var)).quantity);
            if ownedCount >= count {
                SetBagItemQuantity(
                    &raw mut (*(*itemPocket).itemSlots.at(var)).quantity,
                    ownedCount - count,
                );
                count = 0;
            } else {
                count -= ownedCount;
                SetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(var)).quantity, 0);
            }
            if GetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(var)).quantity) == 0 {
                (*(*itemPocket).itemSlots.at(var)).itemId = ITEM_NONE;
            }
            if count == 0 {
                return TRUE;
            }
        }
        i = 0;
        while i < (*itemPocket).capacity {
            if (*(*itemPocket).itemSlots.at(i)).itemId == itemId {
                ownedCount = GetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(i)).quantity);
                if ownedCount >= count {
                    SetBagItemQuantity(
                        &raw mut (*(*itemPocket).itemSlots.at(i)).quantity,
                        ownedCount - count,
                    );
                    count = 0;
                } else {
                    count -= ownedCount;
                    SetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(i)).quantity, 0);
                }
                if GetBagItemQuantity(&raw mut (*(*itemPocket).itemSlots.at(i)).quantity) == 0 {
                    (*(*itemPocket).itemSlots.at(i)).itemId = ITEM_NONE;
                }
                if count == 0 {
                    return TRUE;
                }
            }
            i += 1;
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPocketByItemId(itemId: u16) -> u8 {
    return GetItemPocket(itemId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearItemSlots(mut itemSlots: *mut ItemSlot, itemCount: u8) {
    let mut i: u16 = 0;
    i = 0;
    while i < itemCount as u16 {
        (*itemSlots.at(i)).itemId = ITEM_NONE;
        SetBagItemQuantity(&raw mut (*itemSlots.at(i)).quantity, 0);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FindFreePCItemSlot() -> i32 {
    let mut i: i8 = 0;
    i = 0;
    while i < PC_ITEMS_COUNT as i8 {
        if (*gSaveBlock1Ptr).pcItems[i].itemId == ITEM_NONE {
            return i as i32;
        }
        i += 1;
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountUsedPCItemSlots() -> u8 {
    let mut usedSlots: u8 = 0;
    let mut i: u8 = 0;
    i = 0;
    while i < PC_ITEMS_COUNT {
        if (*gSaveBlock1Ptr).pcItems[i].itemId != ITEM_NONE {
            usedSlots += 1;
        }
        i += 1;
    }
    return usedSlots;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPCHasItem(itemId: u16, count: u16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < PC_ITEMS_COUNT {
        if (*gSaveBlock1Ptr).pcItems[i].itemId == itemId
            && GetPCItemQuantity(&raw mut (*gSaveBlock1Ptr).pcItems[i].quantity) >= count
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPCItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut freeSlot: i8 = 0;
    let mut ownedCount: u16 = 0;
    let mut newItems: *mut ItemSlot = null_mut();
    newItems = AllocZeroed(200) as *mut ItemSlot;
    memcpy(
        newItems as *mut u8,
        (*gSaveBlock1Ptr).pcItems.as_mut_ptr() as *mut u8,
        200,
    );
    i = 0;
    while i < PC_ITEMS_COUNT {
        if (*newItems.at(i)).itemId == itemId {
            ownedCount = GetPCItemQuantity(&raw mut (*newItems.at(i)).quantity);
            if ownedCount as i32 + count as i32 <= MAX_PC_ITEM_CAPACITY as i32 {
                SetPCItemQuantity(&raw mut (*newItems.at(i)).quantity, ownedCount + count);
                memcpy(
                    (*gSaveBlock1Ptr).pcItems.as_mut_ptr() as *mut u8,
                    newItems as *mut u8,
                    200,
                );
                Free(newItems as *mut c_void);
                return TRUE;
            }
            count += ownedCount - MAX_PC_ITEM_CAPACITY;
            SetPCItemQuantity(&raw mut (*newItems.at(i)).quantity, MAX_PC_ITEM_CAPACITY);
            if count == 0 {
                memcpy(
                    (*gSaveBlock1Ptr).pcItems.as_mut_ptr() as *mut u8,
                    newItems as *mut u8,
                    200,
                );
                Free(newItems as *mut c_void);
                return TRUE;
            }
        }
        i += 1;
    }
    if count > 0 {
        freeSlot = FindFreePCItemSlot() as i8;
        if freeSlot == -1 {
            Free(newItems as *mut c_void);
            return FALSE;
        } else {
            (*newItems.at(freeSlot)).itemId = itemId;
            SetPCItemQuantity(&raw mut (*newItems.at(freeSlot)).quantity, count);
        }
    }
    memcpy(
        (*gSaveBlock1Ptr).pcItems.as_mut_ptr() as *mut u8,
        newItems as *mut u8,
        200,
    );
    Free(newItems as *mut c_void);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemovePCItem(index: u8, count: u16) {
    (*gSaveBlock1Ptr).pcItems[index].quantity -= count;
    if (*gSaveBlock1Ptr).pcItems[index].quantity == 0 {
        (*gSaveBlock1Ptr).pcItems[index].itemId = ITEM_NONE;
        CompactPCItems();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactPCItems() {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while i < 49 {
        j = i + 1;
        while j < PC_ITEMS_COUNT as u16 {
            if (*gSaveBlock1Ptr).pcItems[i].itemId == 0 {
                let mut temp: ItemSlot = zeroed();
                temp = (*gSaveBlock1Ptr).pcItems[i];
                (*gSaveBlock1Ptr).pcItems[i] = (*gSaveBlock1Ptr).pcItems[j];
                (*gSaveBlock1Ptr).pcItems[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwapRegisteredBike() {
    match (*gSaveBlock1Ptr).registeredItem {
        ITEM_MACH_BIKE => {
            (*gSaveBlock1Ptr).registeredItem = ITEM_ACRO_BIKE;
        }
        ITEM_ACRO_BIKE => {
            (*gSaveBlock1Ptr).registeredItem = ITEM_MACH_BIKE;
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagGetItemIdByPocketPosition(pocketId: u8, pocketPos: u16) -> u16 {
    return (*gBagPockets[pocketId as i32 - 1].itemSlots.at(pocketPos)).itemId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagGetQuantityByPocketPosition(pocketId: u8, pocketPos: u16) -> u16 {
    return GetBagItemQuantity(
        &raw mut (*gBagPockets[pocketId as i32 - 1].itemSlots.at(pocketPos)).quantity,
    );
}
pub(crate) unsafe extern "C" fn SwapItemSlots(a: *mut ItemSlot, b: *mut ItemSlot) {
    let mut temp: ItemSlot = zeroed();
    temp = *a;
    *a = *b;
    *b = temp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactItemsInBagPocket(bagPocket: *mut BagPocket) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while (i as i32) < (*bagPocket).capacity as i32 - 1 {
        j = i + 1;
        while j < (*bagPocket).capacity as u16 {
            if GetBagItemQuantity(&raw mut (*(*bagPocket).itemSlots.at(i)).quantity) == 0 {
                SwapItemSlots((*bagPocket).itemSlots.at(i), (*bagPocket).itemSlots.at(j));
            }
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SortBerriesOrTMHMs(bagPocket: *mut BagPocket) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while (i as i32) < (*bagPocket).capacity as i32 - 1 {
        j = i + 1;
        while j < (*bagPocket).capacity as u16 {
            'l2: {
                if GetBagItemQuantity(&raw mut (*(*bagPocket).itemSlots.at(i)).quantity) != 0 {
                    if GetBagItemQuantity(&raw mut (*(*bagPocket).itemSlots.at(j)).quantity) == 0 {
                        break 'l2;
                    }
                    if (*(*bagPocket).itemSlots.at(i)).itemId
                        <= (*(*bagPocket).itemSlots.at(j)).itemId
                    {
                        break 'l2;
                    }
                }
                SwapItemSlots((*bagPocket).itemSlots.at(i), (*bagPocket).itemSlots.at(j));
            }
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveItemSlotInList(mut itemSlots: *mut ItemSlot, from: u32, mut to: u32) {
    let mut firstSlot: ItemSlot = zeroed();
    let mut i: i16 = 0;
    if from == to {
        return;
    }
    firstSlot = *itemSlots.at(from);
    if to > from {
        to -= 1;
        i = from as i16;
        while i < to as i16 {
            *itemSlots.at(i) = *itemSlots.at(i as i32 + 1);
            i += 1;
        }
    } else {
        i = from as i16;
        while i > to as i16 {
            *itemSlots.at(i) = *itemSlots.at(i as i32 - 1);
            i -= 1;
        }
    }
    *itemSlots.at(to) = firstSlot;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBag() {
    let mut i: u16 = 0;
    i = 0;
    while i < POCKETS_COUNT as u16 {
        ClearItemSlots(gBagPockets[i].itemSlots, gBagPockets[i].capacity);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountTotalItemQuantityInBag(itemId: u16) -> u16 {
    let mut i: u16 = 0;
    let mut ownedCount: u16 = 0;
    let mut bagPocket: *mut BagPocket = &raw mut gBagPockets[GetItemPocket(itemId) as i32 - 1];
    i = 0;
    while i < (*bagPocket).capacity as u16 {
        if (*(*bagPocket).itemSlots.at(i)).itemId == itemId {
            ownedCount += GetBagItemQuantity(&raw mut (*(*bagPocket).itemSlots.at(i)).quantity);
        }
        i += 1;
    }
    return ownedCount;
}
pub(crate) unsafe extern "C" fn CheckPyramidBagHasItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut items: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    i = 0;
    while i < PYRAMID_BAG_ITEMS_COUNT as u8 {
        if *items.at(i) == itemId {
            if *quantities.at(i) as u16 >= count {
                return TRUE;
            }
            count -= *quantities.at(i) as u16;
            if count == 0 {
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CheckPyramidBagHasSpace(itemId: u16, mut count: u16) -> u8 {
    let mut i: u8 = 0;
    let mut items: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    i = 0;
    while i < PYRAMID_BAG_ITEMS_COUNT as u8 {
        if *items.at(i) == itemId || *items.at(i) == ITEM_NONE {
            if *quantities.at(i) as i32 + count as i32 <= MAX_BAG_ITEM_CAPACITY as i32 {
                return TRUE;
            }
            count = *quantities.at(i) as u16 + count - MAX_BAG_ITEM_CAPACITY;
            if count == 0 {
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPyramidBagItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u16 = 0;
    let mut items: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut newItems: *mut u16 = Alloc(20) as *mut u16;
    let mut newQuantities: *mut u8 = Alloc(PYRAMID_BAG_ITEMS_COUNT) as *mut u8;
    memcpy(newItems as *mut u8, items as *mut u8, 20);
    memcpy(newQuantities, quantities, PYRAMID_BAG_ITEMS_COUNT);
    i = 0;
    while i < PYRAMID_BAG_ITEMS_COUNT as u16 {
        if *newItems.at(i) == itemId && *newQuantities.at(i) < MAX_BAG_ITEM_CAPACITY as u8 {
            *newQuantities.at(i) += count as u8;
            if *newQuantities.at(i) > MAX_BAG_ITEM_CAPACITY as u8 {
                count = *newQuantities.at(i) as u16 - MAX_BAG_ITEM_CAPACITY;
                *newQuantities.at(i) = MAX_BAG_ITEM_CAPACITY as u8;
            } else {
                count = 0;
            }
            if count == 0 {
                break;
            }
        }
        i += 1;
    }
    if count > 0 {
        i = 0;
        while i < PYRAMID_BAG_ITEMS_COUNT as u16 {
            if *newItems.at(i) == ITEM_NONE {
                *newItems.at(i) = itemId;
                *newQuantities.at(i) = count as u8;
                if *newQuantities.at(i) > MAX_BAG_ITEM_CAPACITY as u8 {
                    count = *newQuantities.at(i) as u16 - MAX_BAG_ITEM_CAPACITY;
                    *newQuantities.at(i) = MAX_BAG_ITEM_CAPACITY as u8;
                } else {
                    count = 0;
                }
                if count == 0 {
                    break;
                }
            }
            i += 1;
        }
    }
    if count == 0 {
        memcpy(items as *mut u8, newItems as *mut u8, 20);
        memcpy(quantities, newQuantities, PYRAMID_BAG_ITEMS_COUNT);
        Free(newItems as *mut c_void);
        Free(newQuantities as *mut c_void);
        return TRUE;
    } else {
        Free(newItems as *mut c_void);
        Free(newQuantities as *mut c_void);
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemovePyramidBagItem(itemId: u16, mut count: u16) -> u8 {
    let mut i: u16 = 0;
    let mut items: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    i = gPyramidBagMenuState.cursorPosition + gPyramidBagMenuState.scrollPosition;
    if *items.at(i) == itemId && *quantities.at(i) as u16 >= count {
        *quantities.at(i) -= count as u8;
        if *quantities.at(i) == 0 {
            *items.at(i) = ITEM_NONE;
        }
        return TRUE;
    } else {
        let mut newItems: *mut u16 = Alloc(20) as *mut u16;
        let mut newQuantities: *mut u8 = Alloc(PYRAMID_BAG_ITEMS_COUNT) as *mut u8;
        memcpy(newItems as *mut u8, items as *mut u8, 20);
        memcpy(newQuantities, quantities, PYRAMID_BAG_ITEMS_COUNT);
        i = 0;
        while i < PYRAMID_BAG_ITEMS_COUNT as u16 {
            if *newItems.at(i) == itemId {
                if *newQuantities.at(i) as u16 >= count {
                    *newQuantities.at(i) -= count as u8;
                    count = 0;
                    if *newQuantities.at(i) == 0 {
                        *newItems.at(i) = ITEM_NONE;
                    }
                } else {
                    count -= *newQuantities.at(i) as u16;
                    *newQuantities.at(i) = 0;
                    *newItems.at(i) = ITEM_NONE;
                }
                if count == 0 {
                    break;
                }
            }
            i += 1;
        }
        if count == 0 {
            memcpy(items as *mut u8, newItems as *mut u8, 20);
            memcpy(quantities, newQuantities, PYRAMID_BAG_ITEMS_COUNT);
            Free(newItems as *mut c_void);
            Free(newQuantities as *mut c_void);
            return TRUE;
        } else {
            Free(newItems as *mut c_void);
            Free(newQuantities as *mut c_void);
            return FALSE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SanitizeItemId(itemId: u16) -> u16 {
    if itemId >= ITEMS_COUNT {
        return ITEM_NONE;
    } else {
        return itemId;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemName(itemId: u16) -> *mut u8 {
    return gItems[SanitizeItemId(itemId)].name.as_ptr().cast_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemId(itemId: u16) -> u16 {
    return gItems[SanitizeItemId(itemId)].itemId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemPrice(itemId: u16) -> u16 {
    return gItems[SanitizeItemId(itemId)].price;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemHoldEffect(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].holdEffect;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemHoldEffectParam(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].holdEffectParam;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemDescription(itemId: u16) -> *mut u8 {
    return gItems[SanitizeItemId(itemId)].description;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemImportance(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].importance;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemRegistrability(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].registrability;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemPocket(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].pocket;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemType(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].r#type;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemFieldFunc(itemId: u16) -> Option<unsafe extern "C" fn(u8)> {
    return gItems[SanitizeItemId(itemId)].fieldUseFunc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemBattleUsage(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].battleUsage;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemBattleFunc(itemId: u16) -> Option<unsafe extern "C" fn(u8)> {
    return gItems[SanitizeItemId(itemId)].battleUseFunc;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemSecondaryId(itemId: u16) -> u8 {
    return gItems[SanitizeItemId(itemId)].secondaryId;
}
