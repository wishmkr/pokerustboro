//! Translated from `src/pokenav_match_call_list.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gText_MatchCallAromaLady_Rose_Strategy gText_MatchCallAromaLady_Rose_Pokemon gText_MatchCallAromaLady_Rose_Intro1 gText_MatchCallAromaLady_Rose_Intro2 gText_MatchCallRuinManiac_Andres_Strategy gText_MatchCallRuinManiac_Andres_Pokemon gText_MatchCallRuinManiac_Andres_Intro1 gText_MatchCallRuinManiac_Andres_Intro2 gText_MatchCallRuinManiac_Dusty_Strategy gText_MatchCallRuinManiac_Dusty_Pokemon gText_MatchCallRuinManiac_Dusty_Intro1 gText_MatchCallRuinManiac_Dusty_Intro2 gText_MatchCallTuber_Lola_Strategy gText_MatchCallTuber_Lola_Pokemon gText_MatchCallTuber_Lola_Intro1 gText_MatchCallTuber_Lola_Intro2 gText_MatchCallTuber_Ricky_Strategy gText_MatchCallTuber_Ricky_Pokemon gText_MatchCallTuber_Ricky_Intro1 gText_MatchCallTuber_Ricky_Intro2 gText_MatchCallSisAndBro_LilaAndRoy_Strategy gText_MatchCallSisAndBro_LilaAndRoy_Pokemon gText_MatchCallSisAndBro_LilaAndRoy_Intro1 gText_MatchCallSisAndBro_LilaAndRoy_Intro2 gText_MatchCallCooltrainer_Cristin_Strategy gText_MatchCallCooltrainer_Cristin_Pokemon gText_MatchCallCooltrainer_Cristin_Intro1 gText_MatchCallCooltrainer_Cristin_Intro2 gText_MatchCallCooltrainer_Brooke_Strategy gText_MatchCallCooltrainer_Brooke_Pokemon gText_MatchCallCooltrainer_Brooke_Intro1 gText_MatchCallCooltrainer_Brooke_Intro2 gText_MatchCallCooltrainer_Wilton_Strategy gText_MatchCallCooltrainer_Wilton_Pokemon gText_MatchCallCooltrainer_Wilton_Intro1 gText_MatchCallCooltrainer_Wilton_Intro2 gText_MatchCallHexManiac_Valerie_Strategy gText_MatchCallHexManiac_Valerie_Pokemon gText_MatchCallHexManiac_Valerie_Intro1 gText_MatchCallHexManiac_Valerie_Intro2 gText_MatchCallLady_Cindy_Strategy gText_MatchCallLady_Cindy_Pokemon gText_MatchCallLady_Cindy_Intro1 gText_MatchCallLady_Cindy_Intro2 gText_MatchCallBeauty_Thalia_Strategy gText_MatchCallBeauty_Thalia_Pokemon gText_MatchCallBeauty_Thalia_Intro1 gText_MatchCallBeauty_Thalia_Intro2 gText_MatchCallBeauty_Jessica_Strategy gText_MatchCallBeauty_Jessica_Pokemon gText_MatchCallBeauty_Jessica_Intro1 gText_MatchCallBeauty_Jessica_Intro2 gText_MatchCallRichBoy_Winston_Strategy gText_MatchCallRichBoy_Winston_Pokemon gText_MatchCallRichBoy_Winston_Intro1 gText_MatchCallRichBoy_Winston_Intro2 gText_MatchCallPokeManiac_Steve_Strategy gText_MatchCallPokeManiac_Steve_Pokemon gText_MatchCallPokeManiac_Steve_Intro1 gText_MatchCallPokeManiac_Steve_Intro2 gText_MatchCallSwimmer_Tony_Strategy gText_MatchCallSwimmer_Tony_Pokemon gText_MatchCallSwimmer_Tony_Intro1 gText_MatchCallSwimmer_Tony_Intro2 gText_MatchCallBlackBelt_Nob_Strategy gText_MatchCallBlackBelt_Nob_Pokemon gText_MatchCallBlackBelt_Nob_Intro1 gText_MatchCallBlackBelt_Nob_Intro2 gText_MatchCallBlackBelt_Koji_Strategy gText_MatchCallBlackBelt_Koji_Pokemon gText_MatchCallBlackBelt_Koji_Intro1 gText_MatchCallBlackBelt_Koji_Intro2 gText_MatchCallGuitarist_Fernando_Strategy gText_MatchCallGuitarist_Fernando_Pokemon gText_MatchCallGuitarist_Fernando_Intro1 gText_MatchCallGuitarist_Fernando_Intro2 gText_MatchCallGuitarist_Dalton_Strategy gText_MatchCallGuitarist_Dalton_Pokemon gText_MatchCallGuitarist_Dalton_Intro1 gText_MatchCallGuitarist_Dalton_Intro2 gText_MatchCallKindler_Bernie_Strategy gText_MatchCallKindler_Bernie_Pokemon gText_MatchCallKindler_Bernie_Intro1 gText_MatchCallKindler_Bernie_Intro2 gText_MatchCallCamper_Ethan_Strategy gText_MatchCallCamper_Ethan_Pokemon gText_MatchCallCamper_Ethan_Intro1 gText_MatchCallCamper_Ethan_Intro2 gText_MatchCallOldCouple_JohnAndJay_Strategy gText_MatchCallOldCouple_JohnAndJay_Pokemon gText_MatchCallOldCouple_JohnAndJay_Intro1 gText_MatchCallOldCouple_JohnAndJay_Intro2 gText_MatchCallBugManiac_Jeffrey_Strategy gText_MatchCallBugManiac_Jeffrey_Pokemon gText_MatchCallBugManiac_Jeffrey_Intro1 gText_MatchCallBugManiac_Jeffrey_Intro2 gText_MatchCallPsychic_Cameron_Strategy gText_MatchCallPsychic_Cameron_Pokemon gText_MatchCallPsychic_Cameron_Intro1 gText_MatchCallPsychic_Cameron_Intro2 gText_MatchCallPsychic_Jacki_Strategy gText_MatchCallPsychic_Jacki_Pokemon gText_MatchCallPsychic_Jacki_Intro1 gText_MatchCallPsychic_Jacki_Intro2 gText_MatchCallGentleman_Walter_Strategy gText_MatchCallGentleman_Walter_Pokemon gText_MatchCallGentleman_Walter_Intro1 gText_MatchCallGentleman_Walter_Intro2 gText_MatchCallSchoolKid_Karen_Strategy gText_MatchCallSchoolKid_Karen_Pokemon gText_MatchCallSchoolKid_Karen_Intro1 gText_MatchCallSchoolKid_Karen_Intro2 gText_MatchCallSchoolKid_Jerry_Strategy gText_MatchCallSchoolKid_Jerry_Pokemon gText_MatchCallSchoolKid_Jerry_Intro1 gText_MatchCallSchoolKid_Jerry_Intro2 gText_MatchCallSrAndJr_AnnaAndMeg_Strategy gText_MatchCallSrAndJr_AnnaAndMeg_Pokemon gText_MatchCallSrAndJr_AnnaAndMeg_Intro1 gText_MatchCallSrAndJr_AnnaAndMeg_Intro2 gText_MatchCallPokefan_Isabel_Strategy gText_MatchCallPokefan_Isabel_Pokemon gText_MatchCallPokefan_Isabel_Intro1 gText_MatchCallPokefan_Isabel_Intro2 gText_MatchCallPokefan_Miguel_Strategy gText_MatchCallPokefan_Miguel_Pokemon gText_MatchCallPokefan_Miguel_Intro1 gText_MatchCallPokefan_Miguel_Intro2 gText_MatchCallExpert_Timothy_Strategy gText_MatchCallExpert_Timothy_Pokemon gText_MatchCallExpert_Timothy_Intro1 gText_MatchCallExpert_Timothy_Intro2 gText_MatchCallExpert_Shelby_Strategy gText_MatchCallExpert_Shelby_Pokemon gText_MatchCallExpert_Shelby_Intro1 gText_MatchCallExpert_Shelby_Intro2 gText_MatchCallYoungster_Calvin_Strategy gText_MatchCallYoungster_Calvin_Pokemon gText_MatchCallYoungster_Calvin_Intro1 gText_MatchCallYoungster_Calvin_Intro2 gText_MatchCallFisherman_Elliot_Strategy gText_MatchCallFisherman_Elliot_Pokemon gText_MatchCallFisherman_Elliot_Intro1 gText_MatchCallFisherman_Elliot_Intro2 gText_MatchCallTriathlete_Isaiah_Strategy gText_MatchCallTriathlete_Isaiah_Pokemon gText_MatchCallTriathlete_Isaiah_Intro1 gText_MatchCallTriathlete_Isaiah_Intro2 gText_MatchCallTriathlete_Maria_Strategy gText_MatchCallTriathlete_Maria_Pokemon gText_MatchCallTriathlete_Maria_Intro1 gText_MatchCallTriathlete_Maria_Intro2 gText_MatchCallTriathlete_Abigail_Strategy gText_MatchCallTriathlete_Abigail_Pokemon gText_MatchCallTriathlete_Abigail_Intro1 gText_MatchCallTriathlete_Abigail_Intro2 gText_MatchCallTriathlete_Dylan_Strategy gText_MatchCallTriathlete_Dylan_Pokemon gText_MatchCallTriathlete_Dylan_Intro1 gText_MatchCallTriathlete_Dylan_Intro2 gText_MatchCallTriathlete_Katelyn_Strategy gText_MatchCallTriathlete_Katelyn_Pokemon gText_MatchCallTriathlete_Katelyn_Intro1 gText_MatchCallTriathlete_Katelyn_Intro2 gText_MatchCallTriathlete_Benjamin_Strategy gText_MatchCallTriathlete_Benjamin_Pokemon gText_MatchCallTriathlete_Benjamin_Intro1 gText_MatchCallTriathlete_Benjamin_Intro2 gText_MatchCallTriathlete_Pablo_Strategy gText_MatchCallTriathlete_Pablo_Pokemon gText_MatchCallTriathlete_Pablo_Intro1 gText_MatchCallTriathlete_Pablo_Intro2 gText_MatchCallDragonTamer_Nicolas_Strategy gText_MatchCallDragonTamer_Nicolas_Pokemon gText_MatchCallDragonTamer_Nicolas_Intro1 gText_MatchCallDragonTamer_Nicolas_Intro2 gText_MatchCallBirdKeeper_Robert_Strategy gText_MatchCallBirdKeeper_Robert_Pokemon gText_MatchCallBirdKeeper_Robert_Intro1 gText_MatchCallBirdKeeper_Robert_Intro2 gText_MatchCallNinjaBoy_Lao_Strategy gText_MatchCallNinjaBoy_Lao_Pokemon gText_MatchCallNinjaBoy_Lao_Intro1 gText_MatchCallNinjaBoy_Lao_Intro2 gText_MatchCallBattleGirl_Cyndy_Strategy gText_MatchCallBattleGirl_Cyndy_Pokemon gText_MatchCallBattleGirl_Cyndy_Intro1 gText_MatchCallBattleGirl_Cyndy_Intro2 gText_MatchCallParasolLady_Madeline_Strategy gText_MatchCallParasolLady_Madeline_Pokemon gText_MatchCallParasolLady_Madeline_Intro1 gText_MatchCallParasolLady_Madeline_Intro2 gText_MatchCallSwimmer_Jenny_Strategy gText_MatchCallSwimmer_Jenny_Pokemon gText_MatchCallSwimmer_Jenny_Intro1 gText_MatchCallSwimmer_Jenny_Intro2 gText_MatchCallPicnicker_Diana_Strategy gText_MatchCallPicnicker_Diana_Pokemon gText_MatchCallPicnicker_Diana_Intro1 gText_MatchCallPicnicker_Diana_Intro2 gText_MatchCallTwins_AmyAndLiv_Strategy gText_MatchCallTwins_AmyAndLiv_Pokemon gText_MatchCallTwins_AmyAndLiv_Intro1 gText_MatchCallTwins_AmyAndLiv_Intro2 gText_MatchCallSailor_Ernest_Strategy gText_MatchCallSailor_Ernest_Pokemon gText_MatchCallSailor_Ernest_Intro1 gText_MatchCallSailor_Ernest_Intro2 gText_MatchCallSailor_Cory_Strategy gText_MatchCallSailor_Cory_Pokemon gText_MatchCallSailor_Cory_Intro1 gText_MatchCallSailor_Cory_Intro2 gText_MatchCallCollector_Edwin_Strategy gText_MatchCallCollector_Edwin_Pokemon gText_MatchCallCollector_Edwin_Intro1 gText_MatchCallCollector_Edwin_Intro2 gText_MatchCallPkmnBreeder_Lydia_Strategy gText_MatchCallPkmnBreeder_Lydia_Pokemon gText_MatchCallPkmnBreeder_Lydia_Intro1 gText_MatchCallPkmnBreeder_Lydia_Intro2 gText_MatchCallPkmnBreeder_Isaac_Strategy gText_MatchCallPkmnBreeder_Isaac_Pokemon gText_MatchCallPkmnBreeder_Isaac_Intro1 gText_MatchCallPkmnBreeder_Isaac_Intro2 gText_MatchCallPkmnBreeder_Gabrielle_Strategy gText_MatchCallPkmnBreeder_Gabrielle_Pokemon gText_MatchCallPkmnBreeder_Gabrielle_Intro1 gText_MatchCallPkmnBreeder_Gabrielle_Intro2 gText_MatchCallPkmnRanger_Catherine_Strategy gText_MatchCallPkmnRanger_Catherine_Pokemon gText_MatchCallPkmnRanger_Catherine_Intro1 gText_MatchCallPkmnRanger_Catherine_Intro2 gText_MatchCallPkmnRanger_Jackson_Strategy gText_MatchCallPkmnRanger_Jackson_Pokemon gText_MatchCallPkmnRanger_Jackson_Intro1 gText_MatchCallPkmnRanger_Jackson_Intro2 gText_MatchCallLass_Haley_Strategy gText_MatchCallLass_Haley_Pokemon gText_MatchCallLass_Haley_Intro1 gText_MatchCallLass_Haley_Intro2 gText_MatchCallBugCatcher_James_Strategy gText_MatchCallBugCatcher_James_Pokemon gText_MatchCallBugCatcher_James_Intro1 gText_MatchCallBugCatcher_James_Intro2 gText_MatchCallHiker_Trent_Strategy gText_MatchCallHiker_Trent_Pokemon gText_MatchCallHiker_Trent_Intro1 gText_MatchCallHiker_Trent_Intro2 gText_MatchCallHiker_Sawyer_Strategy gText_MatchCallHiker_Sawyer_Pokemon gText_MatchCallHiker_Sawyer_Intro1 gText_MatchCallHiker_Sawyer_Intro2 gText_MatchCallYoungCouple_LoisAndHal_Strategy gText_MatchCallYoungCouple_LoisAndHal_Pokemon gText_MatchCallYoungCouple_LoisAndHal_Intro1 gText_MatchCallYoungCouple_LoisAndHal_Intro2 gText_MatchCallPkmnTrainer_Wally_Strategy gText_MatchCallPkmnTrainer_Wally_Pokemon gText_MatchCallPkmnTrainer_Wally_Intro1 gText_MatchCallPkmnTrainer_Wally_Intro2 gText_MatchCallRockinWhiz_Roxanne_Strategy gText_MatchCallRockinWhiz_Roxanne_Pokemon gText_MatchCallRockinWhiz_Roxanne_Intro1 gText_MatchCallRockinWhiz_Roxanne_Intro2 gText_MatchCallTheBigHit_Brawly_Strategy gText_MatchCallTheBigHit_Brawly_Pokemon gText_MatchCallTheBigHit_Brawly_Intro1 gText_MatchCallTheBigHit_Brawly_Intro2 gText_MatchCallSwellShock_Wattson_Strategy gText_MatchCallSwellShock_Wattson_Pokemon gText_MatchCallSwellShock_Wattson_Intro1 gText_MatchCallSwellShock_Wattson_Intro2 gText_MatchCallPassionBurn_Flannery_Strategy gText_MatchCallPassionBurn_Flannery_Pokemon gText_MatchCallPassionBurn_Flannery_Intro1 gText_MatchCallPassionBurn_Flannery_Intro2 gText_MatchCallReliableOne_Dad_Strategy gText_MatchCallReliableOne_Dad_Pokemon gText_MatchCallReliableOne_Dad_Intro1 gText_MatchCallReliableOne_Dad_Intro2 gText_MatchCallSkyTamer_Winona_Strategy gText_MatchCallSkyTamer_Winona_Pokemon gText_MatchCallSkyTamer_Winona_Intro1 gText_MatchCallSkyTamer_Winona_Intro2 gText_MatchCallMysticDuo_TateAndLiza_Strategy gText_MatchCallMysticDuo_TateAndLiza_Pokemon gText_MatchCallMysticDuo_TateAndLiza_Intro1 gText_MatchCallMysticDuo_TateAndLiza_Intro2 gText_MatchCallDandyCharm_Juan_Strategy gText_MatchCallDandyCharm_Juan_Pokemon gText_MatchCallDandyCharm_Juan_Intro1 gText_MatchCallDandyCharm_Juan_Intro2 gText_MatchCallEliteFour_Sidney_Strategy gText_MatchCallEliteFour_Sidney_Pokemon gText_MatchCallEliteFour_Sidney_Intro1 gText_MatchCallEliteFour_Sidney_Intro2 gText_MatchCallEliteFour_Phoebe_Strategy gText_MatchCallEliteFour_Phoebe_Pokemon gText_MatchCallEliteFour_Phoebe_Intro1 gText_MatchCallEliteFour_Phoebe_Intro2 gText_MatchCallEliteFour_Glacia_Strategy gText_MatchCallEliteFour_Glacia_Pokemon gText_MatchCallEliteFour_Glacia_Intro1 gText_MatchCallEliteFour_Glacia_Intro2 gText_MatchCallEliteFour_Drake_Strategy gText_MatchCallEliteFour_Drake_Pokemon gText_MatchCallEliteFour_Drake_Intro1 gText_MatchCallEliteFour_Drake_Intro2 gText_MatchCallChampion_Wallace_Strategy gText_MatchCallChampion_Wallace_Pokemon gText_MatchCallChampion_Wallace_Intro1 gText_MatchCallChampion_Wallace_Intro2 gMatchCallFlavorTexts sMatchCallOptionsNoCheckPage sMatchCallOptionsHasCheckPage
#[allow(unused_imports)]
use crate::data::pokenav_match_call_list::*;

unsafe extern "C" {
    static mut gFacilityClassToPicIndex: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gRematchTable: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gStringVar4: u8;
    static mut gText_CallCantBeMadeHere: u8;
    static mut gTrainerClassNames: u8;
    static mut gTrainers: u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn FlagGet(a0: u16) -> u8;
    fn FreePokenavSubstruct(a0: u32);
    fn GetPokenavMode() -> u32;
    fn GetStringClearToWidth(a0: *mut u8, a1: i32, a2: *mut u8, a3: i32) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn GetTrainerIdxByRematchIdx(a0: u32) -> u32;
    fn MatchCall_GetEnabled(a0: u32) -> u32;
    fn MatchCall_GetMapSec(a0: u32) -> u8;
    fn MatchCall_GetMessage(a0: u32, a1: *mut u8);
    fn MatchCall_GetNameAndDesc(a0: u32, a1: *mut *mut u8, a2: *mut *mut u8);
    fn MatchCall_GetOverrideFacilityClass(a0: u32) -> i32;
    fn MatchCall_GetOverrideFlavorText(a0: u32, a1: u32) -> *mut u8;
    fn MatchCall_GetRematchTableIdx(a0: u32) -> u32;
    fn MatchCall_HasCheckPage(a0: u32) -> u32;
    fn MatchCall_HasRematchId(a0: u32) -> u32;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PokenavList_GetSelectedIndex() -> u32;
    fn SelectMatchCallMessage(a0: i32, a1: *mut u8) -> u32;
    fn SetPokenavMode(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_MatchCall() -> u32 {
    unsafe {
        let mut state: *mut u8 = AllocSubstruct(5u32, 424u32);
        if !(!(state).is_null()) {
            return 0u32;
        }
        ((state)
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(CB2_HandleMatchCallInput));
        ((state).wrapping_add(8).cast::<u16>()).write(0u16);
        ((state).wrapping_add(16).cast::<u32>()).write(0u32);
        ((state).wrapping_add(20).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_BuildMatchCallList), 1u32));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallCallback() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return (((state)
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .read())
        .unwrap_unchecked()(state);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMatchCallSubstruct1() {
    unsafe {
        FreePokenavSubstruct(5u32);
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleMatchCallInput(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        let mut selection: i32 = 0i32;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return 2u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return 1u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            return 4u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            return 3u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((state)
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(CB2_HandleMatchCallOptionsInput));
            ((state).cast::<u16>()).write(0u16);
            selection = ((PokenavList_GetSelectedIndex()) as i32);
            if (!((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((selection) as isize * 4))
            .read())
                != 0))
                || ((MatchCall_HasCheckPage(
                    (((((((state).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset((selection) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as u32),
                )) != 0)
            {
                ((state).wrapping_add(4).cast::<*mut u8>()).write(
                    ((&raw const sMatchCallOptionsHasCheckPage)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((state).wrapping_add(2).cast::<u16>())
                    .write((((crate::c::div_u32(3u32, 1u32)).wrapping_sub(1u32)) as u16));
            } else {
                ((state).wrapping_add(4).cast::<*mut u8>()).write(
                    ((&raw const sMatchCallOptionsNoCheckPage)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((state).wrapping_add(2).cast::<u16>())
                    .write((((crate::c::div_u32(2u32, 1u32)).wrapping_sub(1u32)) as u16));
            }
            return 5u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            if GetPokenavMode() != 1u32 {
                ((state)
                    .wrapping_add(24)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(GetExitMatchCallMenuId));
                return 15u32;
            } else {
                PlaySE(32u16);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetExitMatchCallMenuId(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        return 100004u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleMatchCallOptionsInput(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && ((((state).cast::<u16>()).read()) != 0)
        {
            let __p1 = (state).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return 6u32;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((state).cast::<u16>()).read()) as i32)
                < ((((state).wrapping_add(2).cast::<u16>()).read()) as i32))
        {
            let __p2 = (state).cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            return 6u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            'l1: {
                let __sw3 = ((((((state).wrapping_add(4).cast::<*mut u8>()).read())
                    .wrapping_offset(((((state).cast::<u16>()).read()) as i32) as isize))
                .read()) as i32);
                if __sw3 == 2i32 {
                    ((state)
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(CB2_HandleMatchCallInput));
                    return 7u32;
                }
                if __sw3 == 0i32 {
                    if GetPokenavMode() == 1u32 {
                        SetPokenavMode(2u16);
                    }
                    ((state)
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(CB2_HandleCallExitInput));
                    if (ShouldDoNearbyMessage()) != 0 {
                        return 9u32;
                    }
                    return 8u32;
                }
                if __sw3 == 1i32 {
                    ((state)
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                    .write(Some(CB2_HandleCheckPageInput));
                    return 11u32;
                }
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((state)
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(CB2_HandleMatchCallInput));
            return 7u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleCheckPageInput(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return 12u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return 13u32;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((state)
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(CB2_HandleMatchCallInput));
            return 14u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CB2_HandleCallExitInput(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ((state)
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(CB2_HandleMatchCallInput));
            return 10u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_BuildMatchCallList(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        'l1: {
            let __sw1 = taskState;
            if __sw1 == 0i32 {
                ((state).wrapping_add(8).cast::<u16>()).write(0u16);
                ((state).wrapping_add(10).cast::<u16>()).write(0u16);
                return 1u32;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    j = ((((state).wrapping_add(8).cast::<u16>()).read()) as i32);
                    'l2: loop {
                        if !(i < 30i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (MatchCall_GetEnabled(((j) as u32))) != 0 {
                                (((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write(((j) as u16));
                                ((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .write(1u8);
                                (((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(1))
                                .write(MatchCall_GetMapSec(((j) as u32)));
                                let __p2 = (state).wrapping_add(10).cast::<u16>();
                                (__p2).write(((__p2).read()).wrapping_add(1));
                            }
                            if (({
                                let __p3 = (state).wrapping_add(8).cast::<u16>();
                                let __t4 = ((__p3).read()).wrapping_add(1);
                                (__p3).write(__t4);
                                __t4
                            }) as i32)
                                >= 21i32
                            {
                                ((state).wrapping_add(12).cast::<u16>())
                                    .write(((state).wrapping_add(8).cast::<u16>()).read());
                                ((state).wrapping_add(8).cast::<u16>()).write(0u16);
                                return 1u32;
                            }
                        }
                        i = (i).wrapping_add(1);
                        j = (j).wrapping_add(1);
                    }
                }
                return 3u32;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    j = ((((state).wrapping_add(8).cast::<u16>()).read()) as i32);
                    'l4: loop {
                        if !(i < 30i32) {
                            break 'l4;
                        }
                        'l5: {
                            if (!((MatchCall_HasRematchId(
                                ((((state).wrapping_add(8).cast::<u16>()).read()) as u32),
                            )) != 0))
                                && ((IsRematchEntryRegistered(
                                    ((((state).wrapping_add(8).cast::<u16>()).read()) as i32),
                                )) != 0)
                            {
                                (((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write(((state).wrapping_add(8).cast::<u16>()).read());
                                ((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .write(0u8);
                                (((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset(
                                    ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(1))
                                .write(GetMatchTableMapSectionId(j));
                                let __p5 = (state).wrapping_add(10).cast::<u16>();
                                (__p5).write(((__p5).read()).wrapping_add(1));
                            }
                            if (({
                                let __p6 = (state).wrapping_add(8).cast::<u16>();
                                let __t7 = ((__p6).read()).wrapping_add(1);
                                (__p6).write(__t7);
                                __t7
                            }) as i32)
                                > 77i32
                            {
                                return 1u32;
                            }
                        }
                        i = (i).wrapping_add(1);
                        j = (j).wrapping_add(1);
                    }
                }
                return 3u32;
            }
            if __sw1 == 3i32 {
                ((state).wrapping_add(16).cast::<u32>()).write(1u32);
                break 'l1;
            }
        }
        return 4u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRematchEntryRegistered(rematchIndex: i32) -> u32 {
    unsafe {
        let mut rematchIndex = rematchIndex;
        if rematchIndex < 78i32 {
            return ((FlagGet((((348i32).wrapping_add(rematchIndex)) as u16))) as u32);
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMatchCallListInitFinished() -> i32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((((state).wrapping_add(16).cast::<u32>()).read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumberRegistered() -> i32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((((state).wrapping_add(10).cast::<u16>()).read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetNumSpecialTrainers() -> i32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((((state).wrapping_add(12).cast::<u16>()).read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetNumNormalTrainers() -> i32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((((state).wrapping_add(10).cast::<u16>()).read()) as i32)
            .wrapping_sub(((((state).wrapping_add(12).cast::<u16>()).read()) as i32));
    }
}
pub(crate) unsafe extern "C" fn GetNormalTrainerHeaderId(index: i32) -> i32 {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        index = (index).wrapping_add(((((state).wrapping_add(12).cast::<u16>()).read()) as i32));
        if index >= ((((state).wrapping_add(10).cast::<u16>()).read()) as i32) {
            return 78i32;
        }
        return (((((((state).wrapping_add(28)).cast::<u8>())
            .wrapping_offset((index) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallList() -> *mut u8 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((state).wrapping_add(28)).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallMapSec(index: i32) -> u16 {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return (((((((state).wrapping_add(28)).cast::<u8>())
            .wrapping_offset((index) as isize * 4))
        .wrapping_add(1))
        .read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDrawRematchPokeballIcon(index: i32) -> u32 {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        if !((((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset((index) as isize * 4))
            .read())
            != 0)
        {
            index = (((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((index) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32);
        } else {
            index = ((MatchCall_GetRematchTableIdx(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
            )) as i32);
        }
        if index == 78i32 {
            return 0u32;
        }
        return ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2506))
            .cast::<u8>())
        .wrapping_offset((index) as isize))
        .read()) as i32)
            != 0i32) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallTrainerPic(index: i32) -> i32 {
    unsafe {
        let mut index = index;
        let mut headerId: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        if !((((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset((index) as isize * 4))
            .read())
            != 0)
        {
            index = ((GetTrainerIdxByRematchIdx(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
            )) as i32);
            return ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset((index) as isize * 40))
                .wrapping_add(3))
            .read()) as i32);
        }
        headerId = (((((((state).wrapping_add(28)).cast::<u8>())
            .wrapping_offset((index) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32);
        index = ((MatchCall_GetRematchTableIdx(((headerId) as u32))) as i32);
        if index != 78i32 {
            index = ((GetTrainerIdxByRematchIdx(((index) as u32))) as i32);
            return ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset((index) as isize * 40))
                .wrapping_add(3))
            .read()) as i32);
        }
        index = MatchCall_GetOverrideFacilityClass(((headerId) as u32));
        return (((((&raw mut gFacilityClassToPicIndex).cast::<u8>())
            .wrapping_offset((index) as isize))
        .read()) as i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallMessageText(
    index: i32,
    newRematchRequest: *mut u8,
) -> *mut u8 {
    unsafe {
        let mut index = index;
        let mut newRematchRequest = newRematchRequest;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        (newRematchRequest).write(0u8);
        if !((Overworld_MapTypeAllowsTeleportAndFly(
            (((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read(),
        )) != 0)
        {
            return (&raw mut gText_CallCantBeMadeHere).cast::<u8>();
        }
        if !((((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset((index) as isize * 4))
            .read())
            != 0)
        {
            (newRematchRequest).write(
                ((SelectMatchCallMessage(
                    ((GetTrainerIdxByRematchIdx(
                        (((((((state).wrapping_add(28)).cast::<u8>())
                            .wrapping_offset((index) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as u32),
                    )) as i32),
                    (&raw mut gStringVar4).cast::<u8>(),
                )) as u8),
            );
        } else {
            MatchCall_GetMessage(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
                (&raw mut gStringVar4).cast::<u8>(),
            );
        }
        return (&raw mut gStringVar4).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallFlavorText(index: i32, checkPageEntry: i32) -> *mut u8 {
    unsafe {
        let mut index = index;
        let mut checkPageEntry = checkPageEntry;
        let mut rematchId: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        if (((((state).wrapping_add(28)).cast::<u8>()).wrapping_offset((index) as isize * 4))
            .read())
            != 0
        {
            rematchId = ((MatchCall_GetRematchTableIdx(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
            )) as i32);
            if rematchId == 78i32 {
                return MatchCall_GetOverrideFlavorText(
                    (((((((state).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset((index) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as u32),
                    ((checkPageEntry) as u32),
                );
            }
        } else {
            rematchId = (((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((index) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32);
        }
        return ((((((&raw const gMatchCallFlavorTexts).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((rematchId) as isize * 16))
        .cast::<*mut u8>())
        .wrapping_offset((checkPageEntry) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallOptionCursorPos() -> u16 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        return ((state).cast::<u16>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchCallOptionId(optionId: i32) -> u16 {
    unsafe {
        let mut optionId = optionId;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        if ((((state).wrapping_add(2).cast::<u16>()).read()) as i32) < optionId {
            return 3u16;
        }
        return ((((((state).wrapping_add(4).cast::<*mut u8>()).read())
            .wrapping_offset((optionId) as isize))
        .read()) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMatchCallNameAndDesc(matchCallEntry: *mut u8, str: *mut u8) {
    unsafe {
        let mut matchCallEntry = matchCallEntry;
        let mut str = str;
        let mut trainerName: *mut u8 = core::ptr::null_mut();
        let mut className: *mut u8 = core::ptr::null_mut();
        if !(((matchCallEntry).read()) != 0) {
            let mut index: i32 = ((GetTrainerIdxByRematchIdx(
                ((((matchCallEntry).wrapping_add(2).cast::<u16>()).read()) as u32),
            )) as i32);
            let mut trainer: *mut u8 =
                ((&raw mut gTrainers).cast::<u8>()).wrapping_offset((index) as isize * 40);
            let mut class: i32 = ((((trainer).wrapping_add(1)).read()) as i32);
            className = (((&raw mut gTrainerClassNames).cast::<u8>())
                .wrapping_offset((class) as isize * 13))
            .cast::<u8>();
            trainerName = ((trainer).wrapping_add(4)).cast::<u8>();
        } else {
            MatchCall_GetNameAndDesc(
                ((((matchCallEntry).wrapping_add(2).cast::<u16>()).read()) as u32),
                &raw mut className,
                &raw mut trainerName,
            );
        }
        if (!(className).is_null()) && (!(trainerName).is_null()) {
            let mut str2: *mut u8 = GetStringClearToWidth(str, 7i32, className, 69i32);
            GetStringClearToWidth(str2, 7i32, trainerName, 51i32);
        } else {
            GetStringClearToWidth(str, 7i32, core::ptr::null_mut(), 120i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMatchTableMapSectionId(rematchIndex: i32) -> u8 {
    unsafe {
        let mut rematchIndex = rematchIndex;
        let mut mapGroup: i32 = ((((((&raw mut gRematchTable).cast::<u8>())
            .wrapping_offset((rematchIndex) as isize * 16))
        .wrapping_add(10)
        .cast::<u16>())
        .read()) as i32);
        let mut mapNum: i32 = ((((((&raw mut gRematchTable).cast::<u8>())
            .wrapping_offset((rematchIndex) as isize * 16))
        .wrapping_add(12)
        .cast::<u16>())
        .read()) as i32);
        return ((Overworld_GetMapHeaderByGroupAndId(((mapGroup) as u16), ((mapNum) as u16)))
            .wrapping_add(20))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIndexDeltaOfNextCheckPageDown(index: i32) -> i32 {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        let mut count: i32 = 1i32;
        'l1: loop {
            if !({
                let __t1 = (index).wrapping_add(1);
                index = __t1;
                __t1
            } < ((((state).wrapping_add(10).cast::<u16>()).read()) as i32))
            {
                break 'l1;
            }
            if !((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((index) as isize * 4))
            .read())
                != 0)
            {
                return count;
            }
            if (MatchCall_HasCheckPage(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
            )) != 0
            {
                return count;
            }
            count = (count).wrapping_add(1);
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIndexDeltaOfNextCheckPageUp(index: i32) -> i32 {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        let mut count: i32 = (-1i32);
        'l1: loop {
            if !({
                let __t1 = (index).wrapping_sub(1);
                index = __t1;
                __t1
            } >= 0i32)
            {
                break 'l1;
            }
            if !((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((index) as isize * 4))
            .read())
                != 0)
            {
                return count;
            }
            if (MatchCall_HasCheckPage(
                (((((((state).wrapping_add(28)).cast::<u8>())
                    .wrapping_offset((index) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as u32),
            )) != 0
            {
                return count;
            }
            count = (count).wrapping_sub(1);
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn HasRematchEntry() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if ((IsRematchEntryRegistered(i)) != 0)
                        && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read())
                            != 0)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 21i32) {
                    break 'l3;
                }
                'l4: {
                    if (MatchCall_GetEnabled(((i) as u32))) != 0 {
                        let mut index: i32 = ((MatchCall_GetRematchTableIdx(((i) as u32))) as i32);
                        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset((index) as isize))
                        .read())
                            != 0
                        {
                            return 1u32;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ShouldDoNearbyMessage() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(5u32);
        let mut selection: i32 = ((PokenavList_GetSelectedIndex()) as i32);
        if !((((((state).wrapping_add(28)).cast::<u8>())
            .wrapping_offset((selection) as isize * 4))
        .read())
            != 0)
        {
            if ((GetMatchCallMapSec(selection)) as i32)
                == (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32)
            {
                if !((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2506))
                .cast::<u8>())
                .wrapping_offset(
                    (((((((state).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset((selection) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read())
                    != 0)
                {
                    return 1u32;
                }
            }
        } else {
            if (((((((state).wrapping_add(28)).cast::<u8>())
                .wrapping_offset((selection) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                == 11i32
            {
                if (((GetMatchCallMapSec(selection)) as i32)
                    == (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32))
                    && (((FlagGet(2155u16)) as i32) == 1i32)
                {
                    if !((FlagGet(91u16)) != 0) {
                        return 1u32;
                    }
                }
            }
        }
        return 0u32;
    }
}
