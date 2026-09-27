//! Translated from `src/item.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDummyDesc sMasterBallDesc sUltraBallDesc sGreatBallDesc sPokeBallDesc sSafariBallDesc sNetBallDesc sDiveBallDesc sNestBallDesc sRepeatBallDesc sTimerBallDesc sLuxuryBallDesc sPremierBallDesc sPotionDesc sAntidoteDesc sBurnHealDesc sIceHealDesc sAwakeningDesc sParalyzeHealDesc sFullRestoreDesc sMaxPotionDesc sHyperPotionDesc sSuperPotionDesc sFullHealDesc sReviveDesc sMaxReviveDesc sFreshWaterDesc sSodaPopDesc sLemonadeDesc sMoomooMilkDesc sEnergyPowderDesc sEnergyRootDesc sHealPowderDesc sRevivalHerbDesc sEtherDesc sMaxEtherDesc sElixirDesc sMaxElixirDesc sLavaCookieDesc sBlueFluteDesc sYellowFluteDesc sRedFluteDesc sBlackFluteDesc sWhiteFluteDesc sBerryJuiceDesc sSacredAshDesc sShoalSaltDesc sShoalShellDesc sRedShardDesc sBlueShardDesc sYellowShardDesc sGreenShardDesc sHPUpDesc sProteinDesc sIronDesc sCarbosDesc sCalciumDesc sRareCandyDesc sPPUpDesc sZincDesc sPPMaxDesc sGuardSpecDesc sDireHitDesc sXAttackDesc sXDefendDesc sXSpeedDesc sXAccuracyDesc sXSpecialDesc sPokeDollDesc sFluffyTailDesc sSuperRepelDesc sMaxRepelDesc sEscapeRopeDesc sRepelDesc sSunStoneDesc sMoonStoneDesc sFireStoneDesc sThunderStoneDesc sWaterStoneDesc sLeafStoneDesc sTinyMushroomDesc sBigMushroomDesc sPearlDesc sBigPearlDesc sStardustDesc sStarPieceDesc sNuggetDesc sHeartScaleDesc sOrangeMailDesc sHarborMailDesc sGlitterMailDesc sMechMailDesc sWoodMailDesc sWaveMailDesc sBeadMailDesc sShadowMailDesc sTropicMailDesc sDreamMailDesc sFabMailDesc sRetroMailDesc sCheriBerryDesc sChestoBerryDesc sPechaBerryDesc sRawstBerryDesc sAspearBerryDesc sLeppaBerryDesc sOranBerryDesc sPersimBerryDesc sLumBerryDesc sSitrusBerryDesc sFigyBerryDesc sWikiBerryDesc sMagoBerryDesc sAguavBerryDesc sIapapaBerryDesc sRazzBerryDesc sBlukBerryDesc sNanabBerryDesc sWepearBerryDesc sPinapBerryDesc sPomegBerryDesc sKelpsyBerryDesc sQualotBerryDesc sHondewBerryDesc sGrepaBerryDesc sTamatoBerryDesc sCornnBerryDesc sMagostBerryDesc sRabutaBerryDesc sNomelBerryDesc sSpelonBerryDesc sPamtreBerryDesc sWatmelBerryDesc sDurinBerryDesc sBelueBerryDesc sLiechiBerryDesc sGanlonBerryDesc sSalacBerryDesc sPetayaBerryDesc sApicotBerryDesc sLansatBerryDesc sStarfBerryDesc sEnigmaBerryDesc sBrightPowderDesc sWhiteHerbDesc sMachoBraceDesc sExpShareDesc sQuickClawDesc sSootheBellDesc sMentalHerbDesc sChoiceBandDesc sKingsRockDesc sSilverPowderDesc sAmuletCoinDesc sCleanseTagDesc sSoulDewDesc sDeepSeaToothDesc sDeepSeaScaleDesc sSmokeBallDesc sEverstoneDesc sFocusBandDesc sLuckyEggDesc sScopeLensDesc sMetalCoatDesc sLeftoversDesc sDragonScaleDesc sLightBallDesc sSoftSandDesc sHardStoneDesc sMiracleSeedDesc sBlackGlassesDesc sBlackBeltDesc sMagnetDesc sMysticWaterDesc sSharpBeakDesc sPoisonBarbDesc sNeverMeltIceDesc sSpellTagDesc sTwistedSpoonDesc sCharcoalDesc sDragonFangDesc sSilkScarfDesc sUpGradeDesc sShellBellDesc sSeaIncenseDesc sLaxIncenseDesc sLuckyPunchDesc sMetalPowderDesc sThickClubDesc sStickDesc sRedScarfDesc sBlueScarfDesc sPinkScarfDesc sGreenScarfDesc sYellowScarfDesc sMachBikeDesc sCoinCaseDesc sItemfinderDesc sOldRodDesc sGoodRodDesc sSuperRodDesc sSSTicketDesc sContestPassDesc sWailmerPailDesc sDevonGoodsDesc sSootSackDesc sBasementKeyDesc sAcroBikeDesc sPokeblockCaseDesc sLetterDesc sEonTicketDesc sRedOrbDesc sBlueOrbDesc sScannerDesc sGoGogglesDesc sMeteoriteDesc sRoom1KeyDesc sRoom2KeyDesc sRoom4KeyDesc sRoom6KeyDesc sStorageKeyDesc sRootFossilDesc sClawFossilDesc sDevonScopeDesc sTM01Desc sTM02Desc sTM03Desc sTM04Desc sTM05Desc sTM06Desc sTM07Desc sTM08Desc sTM09Desc sTM10Desc sTM11Desc sTM12Desc sTM13Desc sTM14Desc sTM15Desc sTM16Desc sTM17Desc sTM18Desc sTM19Desc sTM20Desc sTM21Desc sTM22Desc sTM23Desc sTM24Desc sTM25Desc sTM26Desc sTM27Desc sTM28Desc sTM29Desc sTM30Desc sTM31Desc sTM32Desc sTM33Desc sTM34Desc sTM35Desc sTM36Desc sTM37Desc sTM38Desc sTM39Desc sTM40Desc sTM41Desc sTM42Desc sTM43Desc sTM44Desc sTM45Desc sTM46Desc sTM47Desc sTM48Desc sTM49Desc sTM50Desc sHM01Desc sHM02Desc sHM03Desc sHM04Desc sHM05Desc sHM06Desc sHM07Desc sHM08Desc sOaksParcelDesc sPokeFluteDesc sSecretKeyDesc sBikeVoucherDesc sGoldTeethDesc sOldAmberDesc sCardKeyDesc sLiftKeyDesc sHelixFossilDesc sDomeFossilDesc sSilphScopeDesc sBicycleDesc sTownMapDesc sVSSeekerDesc sFameCheckerDesc sTMCaseDesc sBerryPouchDesc sTeachyTVDesc sTriPassDesc sRainbowPassDesc sTeaDesc sMysticTicketDesc sAuroraTicketDesc sPowderJarDesc sRubyDesc sSapphireDesc sMagmaEmblemDesc sOldSeaMapDesc gItems
#[allow(unused_imports)]
use crate::data::item::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBagPockets: crate::ffi::Align4<[u8; 40]> = crate::ffi::Align4([0; 40]);

unsafe extern "C" {
    static mut gBerries: u8;
    static mut gPyramidBagMenuState: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_Result: u8;
    static mut gText_Berries: u8;
    static mut gText_Berry: u8;
    static mut gText_PokeBalls: u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn ApplyNewEncryptionKeyToHword(a0: *mut u16, a1: u32);
    fn CurMapIsSecretBase() -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn GetItemListPosition(a0: u8) -> u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn GetBagItemQuantity(quantity: *mut u16) -> u16 {
    unsafe {
        let mut quantity = quantity;
        return ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(172)
            .cast::<u32>())
        .read()
            ^ (((quantity).read()) as u32)) as u16);
    }
}
pub(crate) unsafe extern "C" fn SetBagItemQuantity(quantity: *mut u16, newValue: u16) {
    unsafe {
        let mut quantity = quantity;
        let mut newValue = newValue;
        (quantity).write(
            ((((newValue) as u32)
                ^ ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(172)
                    .cast::<u32>())
                .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn GetPCItemQuantity(quantity: *mut u16) -> u16 {
    unsafe {
        let mut quantity = quantity;
        return (quantity).read();
    }
}
pub(crate) unsafe extern "C" fn SetPCItemQuantity(quantity: *mut u16, newValue: u16) {
    unsafe {
        let mut quantity = quantity;
        let mut newValue = newValue;
        (quantity).write(newValue);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToBagItems(newKey: u32) {
    unsafe {
        let mut newKey = newKey;
        let mut pocket: u32 = 0u32;
        let mut item: u32 = 0u32;
        {
            pocket = 0u32;
            'l1: loop {
                if !(pocket < 5u32) {
                    break 'l1;
                }
                'l2: {
                    {
                        item = 0u32;
                        'l3: loop {
                            if !(item
                                < (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((pocket) as i32) as isize * 8))
                                .wrapping_add(4))
                                .read()) as u32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                ApplyNewEncryptionKeyToHword(
                                    (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((pocket) as i32) as isize * 8))
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((item) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                    newKey,
                                );
                            }
                            item = (item).wrapping_add(1);
                        }
                    }
                }
                pocket = (pocket).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToBagItems_(newKey: u32) {
    unsafe {
        let mut newKey = newKey;
        ApplyNewEncryptionKeyToBagItems(newKey);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBagItemsPointers() {
    unsafe {
        ((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1376))
                .cast::<u8>(),
        );
        ((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_add(4)).write(30u8);
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(32))
            .cast::<*mut u8>())
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1496))
                .cast::<u8>(),
        );
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(32))
            .wrapping_add(4))
        .write(30u8);
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(8))
            .cast::<*mut u8>())
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1616))
                .cast::<u8>(),
        );
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(8)).wrapping_add(4))
            .write(16u8);
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(16))
            .cast::<*mut u8>())
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1680))
                .cast::<u8>(),
        );
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(16))
            .wrapping_add(4))
        .write(64u8);
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .cast::<*mut u8>())
        .write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1936))
                .cast::<u8>(),
        );
        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .wrapping_add(4))
        .write(46u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemName(itemId: u16, dst: *mut u8) {
    unsafe {
        let mut itemId = itemId;
        let mut dst = dst;
        StringCopy(dst, GetItemName(itemId));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemNameHandlePlural(itemId: u16, dst: *mut u8, quantity: u32) {
    unsafe {
        let mut itemId = itemId;
        let mut dst = dst;
        let mut quantity = quantity;
        if ((itemId) as i32) == 4i32 {
            if quantity < 2u32 {
                StringCopy(dst, GetItemName(4u16));
            } else {
                StringCopy(dst, (&raw mut gText_PokeBalls).cast::<u8>());
            }
        } else {
            if (((itemId) as i32) >= 133i32) && (((itemId) as i32) <= 175i32) {
                GetBerryCountString(
                    dst,
                    (((&raw mut gBerries).cast::<u8>())
                        .wrapping_offset((((itemId) as i32).wrapping_sub(133i32)) as isize * 28))
                    .cast::<u8>(),
                    quantity,
                );
            } else {
                StringCopy(dst, GetItemName(itemId));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBerryCountString(dst: *mut u8, berryName: *mut u8, quantity: u32) {
    unsafe {
        let mut dst = dst;
        let mut berryName = berryName;
        let mut quantity = quantity;
        let mut berryString: *mut u8 = core::ptr::null_mut();
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        if quantity < 2u32 {
            berryString = (&raw mut gText_Berry).cast::<u8>();
        } else {
            berryString = (&raw mut gText_Berries).cast::<u8>();
        }
        txtPtr = StringCopy(dst, berryName);
        (txtPtr).write(0u8);
        StringCopy((txtPtr).wrapping_offset(1), berryString);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBagPocketNonEmpty(pocket: u8) -> u8 {
    unsafe {
        let mut pocket = pocket;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((((pocket) as i32).wrapping_sub(1i32)) as isize * 8))
                    .wrapping_add(4))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((((pocket) as i32).wrapping_sub(1i32)) as isize * 8))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBagHasItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut pocket: u8 = 0u8;
        if ((GetItemPocket(itemId)) as i32) == 0i32 {
            return 0u8;
        }
        if (((CurrentBattlePyramidLocation()) as i32) != 0i32)
            || (((FlagGet(16388u16)) as i32) == 1i32)
        {
            return CheckPyramidBagHasItem(itemId, count);
        }
        pocket = ((((GetItemPocket(itemId)) as i32).wrapping_sub(1i32)) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((pocket) as i32) as isize * 8))
                    .wrapping_add(4))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((pocket) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((itemId) as i32)
                    {
                        let mut quantity: u16 = 0u16;
                        quantity = GetBagItemQuantity(
                            (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((pocket) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>(),
                        );
                        if ((quantity) as i32) >= ((count) as i32) {
                            return 1u8;
                        }
                        count = ((((count) as i32).wrapping_sub(((quantity) as i32))) as u16);
                        if ((count) as i32) == 0i32 {
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAtLeastOneBerry() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 133u16;
            'l1: loop {
                if !(((i) as i32) < 179i32) {
                    break 'l1;
                }
                'l2: {
                    if ((CheckBagHasItem(i, 1u16)) as i32) == 1i32 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBagHasSpace(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut pocket: u8 = 0u8;
        let mut slotCapacity: u16 = 0u16;
        let mut ownedCount: u16 = 0u16;
        if ((GetItemPocket(itemId)) as i32) == 0i32 {
            return 0u8;
        }
        if (((CurrentBattlePyramidLocation()) as i32) != 0i32)
            || (((FlagGet(16388u16)) as i32) == 1i32)
        {
            return CheckPyramidBagHasSpace(itemId, count);
        }
        pocket = ((((GetItemPocket(itemId)) as i32).wrapping_sub(1i32)) as u8);
        if ((pocket) as i32) != 3i32 {
            slotCapacity = 99u16;
        } else {
            slotCapacity = 999u16;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((pocket) as i32) as isize * 8))
                    .wrapping_add(4))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((pocket) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((itemId) as i32)
                    {
                        ownedCount = GetBagItemQuantity(
                            (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((pocket) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>(),
                        );
                        if ((ownedCount) as i32).wrapping_add(((count) as i32))
                            <= ((slotCapacity) as i32)
                        {
                            return 1u8;
                        }
                        if (((pocket) as i32) == 2i32) || (((pocket) as i32) == 3i32) {
                            return 0u8;
                        }
                        count = ((((count) as i32).wrapping_sub(
                            ((slotCapacity) as i32).wrapping_sub(((ownedCount) as i32)),
                        )) as u16);
                        if ((count) as i32) == 0i32 {
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((count) as i32) > 0i32 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32)
                        < (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((pocket) as i32) as isize * 8))
                        .wrapping_add(4))
                        .read()) as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((pocket) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            == 0i32
                        {
                            if ((count) as i32) > ((slotCapacity) as i32) {
                                if (((pocket) as i32) == 2i32) || (((pocket) as i32) == 3i32) {
                                    return 0u8;
                                }
                                count = ((((count) as i32).wrapping_sub(((slotCapacity) as i32)))
                                    as u16);
                            } else {
                                count = 0u16;
                                break 'l3;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((count) as i32) > 0i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        if ((GetItemPocket(itemId)) as i32) == 0i32 {
            return 0u8;
        }
        if (((CurrentBattlePyramidLocation()) as i32) != 0i32)
            || (((FlagGet(16388u16)) as i32) == 1i32)
        {
            return AddPyramidBagItem(itemId, count);
        } else {
            let mut itemPocket: *mut u8 = core::ptr::null_mut();
            let mut newItems: *mut u8 = core::ptr::null_mut();
            let mut slotCapacity: u16 = 0u16;
            let mut ownedCount: u16 = 0u16;
            let mut pocket: u8 = ((((GetItemPocket(itemId)) as i32).wrapping_sub(1i32)) as u8);
            itemPocket = (((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((pocket) as i32) as isize * 8);
            newItems =
                AllocZeroed(((((itemPocket).wrapping_add(4)).read()) as u32).wrapping_mul(4u32));
            crate::c::memcpy(
                newItems,
                ((itemPocket).cast::<*mut u8>()).read(),
                ((((itemPocket).wrapping_add(4)).read()) as u32).wrapping_mul(4u32),
            );
            if ((pocket) as i32) != 3i32 {
                slotCapacity = 99u16;
            } else {
                slotCapacity = 999u16;
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((((itemPocket).wrapping_add(4)).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((newItems).wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                        .read()) as i32)
                            == ((itemId) as i32)
                        {
                            ownedCount = GetBagItemQuantity(
                                ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                            );
                            if ((ownedCount) as i32).wrapping_add(((count) as i32))
                                <= ((slotCapacity) as i32)
                            {
                                SetBagItemQuantity(
                                    ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                        .wrapping_add(2)
                                        .cast::<u16>(),
                                    ((((ownedCount) as i32).wrapping_add(((count) as i32))) as u16),
                                );
                                crate::c::memcpy(
                                    ((itemPocket).cast::<*mut u8>()).read(),
                                    newItems,
                                    ((((itemPocket).wrapping_add(4)).read()) as u32)
                                        .wrapping_mul(4u32),
                                );
                                Free(newItems);
                                return 1u8;
                            } else {
                                if (((pocket) as i32) == 2i32) || (((pocket) as i32) == 3i32) {
                                    Free(newItems);
                                    return 0u8;
                                } else {
                                    count = ((((count) as i32).wrapping_sub(
                                        ((slotCapacity) as i32).wrapping_sub(((ownedCount) as i32)),
                                    )) as u16);
                                    SetBagItemQuantity(
                                        ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>(),
                                        slotCapacity,
                                    );
                                    if ((count) as i32) == 0i32 {
                                        break 'l1;
                                    }
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((count) as i32) > 0i32 {
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32) < ((((itemPocket).wrapping_add(4)).read()) as i32)) {
                            break 'l3;
                        }
                        'l4: {
                            if (((((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<u16>())
                            .read()) as i32)
                                == 0i32
                            {
                                (((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                .write(itemId);
                                if ((count) as i32) > ((slotCapacity) as i32) {
                                    if (((pocket) as i32) == 2i32) || (((pocket) as i32) == 3i32) {
                                        Free(newItems);
                                        return 0u8;
                                    }
                                    count = ((((count) as i32)
                                        .wrapping_sub(((slotCapacity) as i32)))
                                        as u16);
                                    SetBagItemQuantity(
                                        ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>(),
                                        slotCapacity,
                                    );
                                } else {
                                    SetBagItemQuantity(
                                        ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                            .wrapping_add(2)
                                            .cast::<u16>(),
                                        count,
                                    );
                                    count = 0u16;
                                    break 'l3;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((count) as i32) > 0i32 {
                    Free(newItems);
                    return 0u8;
                }
            }
            crate::c::memcpy(
                ((itemPocket).cast::<*mut u8>()).read(),
                newItems,
                ((((itemPocket).wrapping_add(4)).read()) as u32).wrapping_mul(4u32),
            );
            Free(newItems);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut totalQuantity: u16 = 0u16;
        if (((GetItemPocket(itemId)) as i32) == 0i32) || (((itemId) as i32) == 0i32) {
            return 0u8;
        }
        if (((CurrentBattlePyramidLocation()) as i32) != 0i32)
            || (((FlagGet(16388u16)) as i32) == 1i32)
        {
            return RemovePyramidBagItem(itemId, count);
        } else {
            let mut pocket: u8 = 0u8;
            let mut var: u8 = 0u8;
            let mut ownedCount: u16 = 0u16;
            let mut itemPocket: *mut u8 = core::ptr::null_mut();
            pocket = ((((GetItemPocket(itemId)) as i32).wrapping_sub(1i32)) as u8);
            itemPocket = (((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((pocket) as i32) as isize * 8);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((((itemPocket).wrapping_add(4)).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((itemPocket).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            == ((itemId) as i32)
                        {
                            totalQuantity = ((((totalQuantity) as i32).wrapping_add(
                                ((GetBagItemQuantity(
                                    ((((itemPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                )) as i32),
                            )) as u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((totalQuantity) as i32) < ((count) as i32) {
                return 0u8;
            }
            if ((CurMapIsSecretBase()) as i32) == 1i32 {
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 512i32) as u16));
                VarSet(16621u16, itemId);
            }
            var = GetItemListPosition(pocket);
            if (((((itemPocket).wrapping_add(4)).read()) as i32) > ((var) as i32))
                && ((((((((itemPocket).cast::<*mut u8>()).read())
                    .wrapping_offset(((var) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    == ((itemId) as i32))
            {
                ownedCount = GetBagItemQuantity(
                    ((((itemPocket).cast::<*mut u8>()).read())
                        .wrapping_offset(((var) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>(),
                );
                if ((ownedCount) as i32) >= ((count) as i32) {
                    SetBagItemQuantity(
                        ((((itemPocket).cast::<*mut u8>()).read())
                            .wrapping_offset(((var) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>(),
                        ((((ownedCount) as i32).wrapping_sub(((count) as i32))) as u16),
                    );
                    count = 0u16;
                } else {
                    count = ((((count) as i32).wrapping_sub(((ownedCount) as i32))) as u16);
                    SetBagItemQuantity(
                        ((((itemPocket).cast::<*mut u8>()).read())
                            .wrapping_offset(((var) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>(),
                        0u16,
                    );
                }
                if ((GetBagItemQuantity(
                    ((((itemPocket).cast::<*mut u8>()).read())
                        .wrapping_offset(((var) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>(),
                )) as i32)
                    == 0i32
                {
                    (((((itemPocket).cast::<*mut u8>()).read())
                        .wrapping_offset(((var) as i32) as isize * 4))
                    .cast::<u16>())
                    .write(0u16);
                }
                if ((count) as i32) == 0i32 {
                    return 1u8;
                }
            }
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < ((((itemPocket).wrapping_add(4)).read()) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((itemPocket).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            == ((itemId) as i32)
                        {
                            ownedCount = GetBagItemQuantity(
                                ((((itemPocket).cast::<*mut u8>()).read())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>(),
                            );
                            if ((ownedCount) as i32) >= ((count) as i32) {
                                SetBagItemQuantity(
                                    ((((itemPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                    ((((ownedCount) as i32).wrapping_sub(((count) as i32))) as u16),
                                );
                                count = 0u16;
                            } else {
                                count =
                                    ((((count) as i32).wrapping_sub(((ownedCount) as i32))) as u16);
                                SetBagItemQuantity(
                                    ((((itemPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                    0u16,
                                );
                            }
                            if ((GetBagItemQuantity(
                                ((((itemPocket).cast::<*mut u8>()).read())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>(),
                            )) as i32)
                                == 0i32
                            {
                                (((((itemPocket).cast::<*mut u8>()).read())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<u16>())
                                .write(0u16);
                            }
                            if ((count) as i32) == 0i32 {
                                return 1u8;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPocketByItemId(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return GetItemPocket(itemId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearItemSlots(itemSlots: *mut u8, itemCount: u8) {
    unsafe {
        let mut itemSlots = itemSlots;
        let mut itemCount = itemCount;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((itemCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((itemSlots).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>())
                        .write(0u16);
                    SetBagItemQuantity(
                        ((itemSlots).wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>(),
                        0u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FindFreePCItemSlot() -> i32 {
    unsafe {
        let mut i: i8 = 0i8;
        {
            i = 0i8;
            'l1: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1176))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        return ((i) as i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountUsedPCItemSlots() -> u8 {
    unsafe {
        let mut usedSlots: u8 = 0u8;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1176))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        usedSlots = (usedSlots).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return usedSlots;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPCHasItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1176))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((itemId) as i32))
                        && (((GetPCItemQuantity(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1176))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>(),
                        )) as i32)
                            >= ((count) as i32))
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPCItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut freeSlot: i8 = 0i8;
        let mut ownedCount: u16 = 0u16;
        let mut newItems: *mut u8 = core::ptr::null_mut();
        newItems = AllocZeroed(200u32);
        crate::c::memcpy(
            newItems,
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>(),
            200u32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 50i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((newItems).wrapping_offset(((i) as i32) as isize * 4)).cast::<u16>())
                        .read()) as i32)
                        == ((itemId) as i32)
                    {
                        ownedCount = GetPCItemQuantity(
                            ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>(),
                        );
                        if ((ownedCount) as i32).wrapping_add(((count) as i32)) <= 999i32 {
                            SetPCItemQuantity(
                                ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                ((((ownedCount) as i32).wrapping_add(((count) as i32))) as u16),
                            );
                            crate::c::memcpy(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1176))
                                .cast::<u8>(),
                                newItems,
                                200u32,
                            );
                            Free(newItems);
                            return 1u8;
                        }
                        count = ((((count) as i32)
                            .wrapping_add(((ownedCount) as i32).wrapping_sub(999i32)))
                            as u16);
                        SetPCItemQuantity(
                            ((newItems).wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>(),
                            999u16,
                        );
                        if ((count) as i32) == 0i32 {
                            crate::c::memcpy(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1176))
                                .cast::<u8>(),
                                newItems,
                                200u32,
                            );
                            Free(newItems);
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((count) as i32) > 0i32 {
            freeSlot = ((FindFreePCItemSlot()) as i8);
            if ((freeSlot) as i32) == (-1i32) {
                Free(newItems);
                return 0u8;
            } else {
                (((newItems).wrapping_offset(((freeSlot) as i32) as isize * 4)).cast::<u16>())
                    .write(itemId);
                SetPCItemQuantity(
                    ((newItems).wrapping_offset(((freeSlot) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>(),
                    count,
                );
            }
        }
        crate::c::memcpy(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>(),
            newItems,
            200u32,
        );
        Free(newItems);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemovePCItem(index: u8, count: u16) {
    unsafe {
        let mut index = index;
        let mut count = count;
        let __p1 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
            .cast::<u8>())
        .wrapping_offset(((index) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(((count) as i32))) as u16));
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
            .cast::<u8>())
        .wrapping_offset(((index) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((index) as i32) as isize * 4))
            .cast::<u16>())
            .write(0u16);
            CompactPCItems();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactPCItems() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 49i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u16);
                        'l3: loop {
                            if !(((j) as i32) < 50i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1176))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    let mut temp = crate::ffi::Align4([0u8; 4]);
                                    (&raw mut temp)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .write_unaligned(
                                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(1176))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 4)
                                            .cast::<crate::c::Rec4<4>>()
                                            .read_unaligned(),
                                        );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1176))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .write_unaligned(
                                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(1176))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 4)
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                    );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1176))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .write_unaligned(
                                        (&raw mut temp)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<4>>()
                                            .read_unaligned(),
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwapRegisteredBike() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1174)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 259i32 {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1174)
                    .cast::<u16>())
                .write(272u16);
                break 'l1;
            }
            if __sw1 == 272i32 {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1174)
                    .cast::<u16>())
                .write(259u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagGetItemIdByPocketPosition(pocketId: u8, pocketPos: u16) -> u16 {
    unsafe {
        let mut pocketId = pocketId;
        let mut pocketPos = pocketPos;
        return ((((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
            .wrapping_offset((((pocketId) as i32).wrapping_sub(1i32)) as isize * 8))
        .cast::<*mut u8>())
        .read())
        .wrapping_offset(((pocketPos) as i32) as isize * 4))
        .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagGetQuantityByPocketPosition(pocketId: u8, pocketPos: u16) -> u16 {
    unsafe {
        let mut pocketId = pocketId;
        let mut pocketPos = pocketPos;
        return GetBagItemQuantity(
            (((((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                .wrapping_offset((((pocketId) as i32).wrapping_sub(1i32)) as isize * 8))
            .cast::<*mut u8>())
            .read())
            .wrapping_offset(((pocketPos) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn SwapItemSlots(a: *mut u8, b: *mut u8) {
    unsafe {
        let mut a = a;
        let mut b = b;
        let mut temp = crate::ffi::Align4([0u8; 4]);
        {
            (&raw mut temp)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .write_unaligned(a.cast::<crate::c::Rec4<4>>().read_unaligned());
            a.cast::<crate::c::Rec4<4>>()
                .write_unaligned(b.cast::<crate::c::Rec4<4>>().read_unaligned());
            b.cast::<crate::c::Rec4<4>>().write_unaligned(
                (&raw mut temp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactItemsInBagPocket(bagPocket: *mut u8) {
    unsafe {
        let mut bagPocket = bagPocket;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((bagPocket).wrapping_add(4)).read()) as i32).wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u16);
                        'l3: loop {
                            if !(((j) as i32) < ((((bagPocket).wrapping_add(4)).read()) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((GetBagItemQuantity(
                                    ((((bagPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                )) as i32)
                                    == 0i32
                                {
                                    SwapItemSlots(
                                        (((bagPocket).cast::<*mut u8>()).read())
                                            .wrapping_offset(((i) as i32) as isize * 4),
                                        (((bagPocket).cast::<*mut u8>()).read())
                                            .wrapping_offset(((j) as i32) as isize * 4),
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SortBerriesOrTMHMs(bagPocket: *mut u8) {
    unsafe {
        let mut bagPocket = bagPocket;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((bagPocket).wrapping_add(4)).read()) as i32).wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u16);
                        'l3: loop {
                            if !(((j) as i32) < ((((bagPocket).wrapping_add(4)).read()) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((GetBagItemQuantity(
                                    ((((bagPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>(),
                                )) as i32)
                                    != 0i32
                                {
                                    if ((GetBagItemQuantity(
                                        ((((bagPocket).cast::<*mut u8>()).read())
                                            .wrapping_offset(((j) as i32) as isize * 4))
                                        .wrapping_add(2)
                                        .cast::<u16>(),
                                    )) as i32)
                                        == 0i32
                                    {
                                        break 'l4;
                                    }
                                    if (((((((bagPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .read()) as i32)
                                        <= (((((((bagPocket).cast::<*mut u8>()).read())
                                            .wrapping_offset(((j) as i32) as isize * 4))
                                        .cast::<u16>())
                                        .read()) as i32)
                                    {
                                        break 'l4;
                                    }
                                }
                                SwapItemSlots(
                                    (((bagPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((i) as i32) as isize * 4),
                                    (((bagPocket).cast::<*mut u8>()).read())
                                        .wrapping_offset(((j) as i32) as isize * 4),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveItemSlotInList(itemSlots: *mut u8, from: u32, to: u32) {
    unsafe {
        let mut itemSlots = itemSlots;
        let mut from = from;
        let mut to = to;
        let mut firstSlot = crate::ffi::Align4([0u8; 4]);
        let mut i: i16 = 0i16;
        if from == to {
            return;
        }
        (&raw mut firstSlot)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (itemSlots)
                    .wrapping_offset(((from) as i32) as isize * 4)
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        if to > from {
            to = (to).wrapping_sub(1);
            {
                i = ((from) as i16);
                'l1: loop {
                    if !(((i) as i32) < (((to) as i16) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (itemSlots)
                            .wrapping_offset(((i) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (itemSlots)
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = ((from) as i16);
                'l3: loop {
                    if !(((i) as i32) > (((to) as i16) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        (itemSlots)
                            .wrapping_offset(((i) as i32) as isize * 4)
                            .cast::<crate::c::Rec4<4>>()
                            .write_unaligned(
                                (itemSlots)
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize * 4)
                                    .cast::<crate::c::Rec4<4>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
        }
        (itemSlots)
            .wrapping_offset(((to) as i32) as isize * 4)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut firstSlot)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearBag() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ClearItemSlots(
                        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .read(),
                        (((((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .wrapping_add(4))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountTotalItemQuantityInBag(itemId: u16) -> u16 {
    unsafe {
        let mut itemId = itemId;
        let mut i: u16 = 0u16;
        let mut ownedCount: u16 = 0u16;
        let mut bagPocket: *mut u8 = (((&raw mut gBagPockets).cast::<u8>()).cast::<u8>())
            .wrapping_offset((((GetItemPocket(itemId)) as i32).wrapping_sub(1i32)) as isize * 8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((((bagPocket).wrapping_add(4)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((bagPocket).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((itemId) as i32)
                    {
                        ownedCount = ((((ownedCount) as i32).wrapping_add(
                            ((GetBagItemQuantity(
                                ((((bagPocket).cast::<*mut u8>()).read())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>(),
                            )) as i32),
                        )) as u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ownedCount;
    }
}
pub(crate) unsafe extern "C" fn CheckPyramidBagHasItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut items: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((items).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        == ((itemId) as i32)
                    {
                        if ((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            >= ((count) as i32)
                        {
                            return 1u8;
                        }
                        count = ((((count) as i32).wrapping_sub(
                            ((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32),
                        )) as u16);
                        if ((count) as i32) == 0i32 {
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CheckPyramidBagHasSpace(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u8 = 0u8;
        let mut items: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((items).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        == ((itemId) as i32))
                        || (((((items).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            == 0i32)
                    {
                        if ((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            .wrapping_add(((count) as i32))
                            <= 99i32
                        {
                            return 1u8;
                        }
                        count = (((((((quantities).wrapping_offset(((i) as i32) as isize)).read())
                            as i32)
                            .wrapping_add(((count) as i32)))
                        .wrapping_sub(99i32)) as u16);
                        if ((count) as i32) == 0i32 {
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPyramidBagItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u16 = 0u16;
        let mut items: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        let mut newItems: *mut u16 = (Alloc(20u32)).cast::<u16>();
        let mut newQuantities: *mut u8 = Alloc(10u32);
        crate::c::memcpy((newItems).cast::<u8>(), (items).cast::<u8>(), 20u32);
        crate::c::memcpy(newQuantities, quantities, 10u32);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((newItems).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        == ((itemId) as i32))
                        && (((((newQuantities).wrapping_offset(((i) as i32) as isize)).read())
                            as i32)
                            < 99i32)
                    {
                        let __p1 = (newQuantities).wrapping_offset(((i) as i32) as isize);
                        (__p1).write(
                            (((((__p1).read()) as i32).wrapping_add(((count) as i32))) as u8),
                        );
                        if ((((newQuantities).wrapping_offset(((i) as i32) as isize)).read())
                            as i32)
                            > 99i32
                        {
                            count = ((((((newQuantities).wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                .wrapping_sub(99i32)) as u16);
                            ((newQuantities).wrapping_offset(((i) as i32) as isize)).write(99u8);
                        } else {
                            count = 0u16;
                        }
                        if ((count) as i32) == 0i32 {
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((count) as i32) > 0i32 {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((newItems).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            == 0i32
                        {
                            ((newItems).wrapping_offset(((i) as i32) as isize)).write(itemId);
                            ((newQuantities).wrapping_offset(((i) as i32) as isize))
                                .write(((count) as u8));
                            if ((((newQuantities).wrapping_offset(((i) as i32) as isize)).read())
                                as i32)
                                > 99i32
                            {
                                count = ((((((newQuantities)
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_sub(99i32))
                                    as u16);
                                ((newQuantities).wrapping_offset(((i) as i32) as isize))
                                    .write(99u8);
                            } else {
                                count = 0u16;
                            }
                            if ((count) as i32) == 0i32 {
                                break 'l3;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if ((count) as i32) == 0i32 {
            crate::c::memcpy((items).cast::<u8>(), (newItems).cast::<u8>(), 20u32);
            crate::c::memcpy(quantities, newQuantities, 10u32);
            Free((newItems).cast::<u8>());
            Free(newQuantities);
            return 1u8;
        } else {
            Free((newItems).cast::<u8>());
            Free(newQuantities);
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemovePyramidBagItem(itemId: u16, count: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut count = count;
        let mut i: u16 = 0u16;
        let mut items: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        i = (((((((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        if (((((items).wrapping_offset(((i) as i32) as isize)).read()) as i32) == ((itemId) as i32))
            && (((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                >= ((count) as i32))
        {
            let __p1 = (quantities).wrapping_offset(((i) as i32) as isize);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(((count) as i32))) as u8));
            if ((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32) == 0i32 {
                ((items).wrapping_offset(((i) as i32) as isize)).write(0u16);
            }
            return 1u8;
        } else {
            let mut newItems: *mut u16 = (Alloc(20u32)).cast::<u16>();
            let mut newQuantities: *mut u8 = Alloc(10u32);
            crate::c::memcpy((newItems).cast::<u8>(), (items).cast::<u8>(), 20u32);
            crate::c::memcpy(newQuantities, quantities, 10u32);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((newItems).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            == ((itemId) as i32)
                        {
                            if ((((newQuantities).wrapping_offset(((i) as i32) as isize)).read())
                                as i32)
                                >= ((count) as i32)
                            {
                                let __p2 = (newQuantities).wrapping_offset(((i) as i32) as isize);
                                (__p2).write(
                                    (((((__p2).read()) as i32).wrapping_sub(((count) as i32)))
                                        as u8),
                                );
                                count = 0u16;
                                if ((((newQuantities).wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                    == 0i32
                                {
                                    ((newItems).wrapping_offset(((i) as i32) as isize)).write(0u16);
                                }
                            } else {
                                count = ((((count) as i32).wrapping_sub(
                                    ((((newQuantities).wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                )) as u16);
                                ((newQuantities).wrapping_offset(((i) as i32) as isize)).write(0u8);
                                ((newItems).wrapping_offset(((i) as i32) as isize)).write(0u16);
                            }
                            if ((count) as i32) == 0i32 {
                                break 'l1;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((count) as i32) == 0i32 {
                crate::c::memcpy((items).cast::<u8>(), (newItems).cast::<u8>(), 20u32);
                crate::c::memcpy(quantities, newQuantities, 10u32);
                Free((newItems).cast::<u8>());
                Free(newQuantities);
                return 1u8;
            } else {
                Free((newItems).cast::<u8>());
                Free(newQuantities);
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SanitizeItemId(itemId: u16) -> u16 {
    unsafe {
        let mut itemId = itemId;
        if ((itemId) as i32) >= 377i32 {
            return 0u16;
        } else {
            return itemId;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemName(itemId: u16) -> *mut u8 {
    unsafe {
        let mut itemId = itemId;
        return ((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemId(itemId: u16) -> u16 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(14)
        .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemPrice(itemId: u16) -> u16 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(16)
        .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemHoldEffect(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(18))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemHoldEffectParam(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(19))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemDescription(itemId: u16) -> *mut u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(20)
        .cast::<*mut u8>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemImportance(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(24))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemRegistrability(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(25))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemPocket(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(26))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemType(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(27))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemFieldFunc(itemId: u16) -> Option<unsafe extern "C" fn(u8)> {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemBattleUsage(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(32))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemBattleFunc(itemId: u16) -> Option<unsafe extern "C" fn(u8)> {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(36)
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemSecondaryId(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        return (((((&raw const gItems).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((SanitizeItemId(itemId)) as i32) as isize * 44))
        .wrapping_add(40))
        .read();
    }
}
