//! Translated from `src/pokenav_match_call_list.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::fieldmap::gMapHeader;
use crate::international_string_util::GetStringClearToWidth;
use crate::load_save::gSaveBlock1Ptr;
use crate::match_call::SelectMatchCallMessage;
use crate::overworld::{Overworld_GetMapHeaderByGroupAndId, Overworld_MapTypeAllowsTeleportAndFly};
use crate::pokenav::CreateLoopedTask;
use crate::pokenav::{
    AllocSubstruct, FreePokenavSubstruct, GetPokenavMode, GetSubstructPtr, SetPokenavMode,
};
use crate::pokenav_list::PokenavList_GetSelectedIndex;
use crate::pokenav_match_call_data::{
    GetTrainerIdxByRematchIdx, MatchCall_GetEnabled, MatchCall_GetMapSec, MatchCall_GetMessage,
    MatchCall_GetNameAndDesc, MatchCall_GetOverrideFacilityClass, MatchCall_GetOverrideFlavorText,
    MatchCall_GetRematchTableIdx, MatchCall_HasCheckPage, MatchCall_HasRematchId,
};
use crate::sound::PlaySE;
use crate::string_util::gStringVar4;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): gText_MatchCallAromaLady_Rose_Strategy gText_MatchCallAromaLady_Rose_Pokemon gText_MatchCallAromaLady_Rose_Intro1 gText_MatchCallAromaLady_Rose_Intro2 gText_MatchCallRuinManiac_Andres_Strategy gText_MatchCallRuinManiac_Andres_Pokemon gText_MatchCallRuinManiac_Andres_Intro1 gText_MatchCallRuinManiac_Andres_Intro2 gText_MatchCallRuinManiac_Dusty_Strategy gText_MatchCallRuinManiac_Dusty_Pokemon gText_MatchCallRuinManiac_Dusty_Intro1 gText_MatchCallRuinManiac_Dusty_Intro2 gText_MatchCallTuber_Lola_Strategy gText_MatchCallTuber_Lola_Pokemon gText_MatchCallTuber_Lola_Intro1 gText_MatchCallTuber_Lola_Intro2 gText_MatchCallTuber_Ricky_Strategy gText_MatchCallTuber_Ricky_Pokemon gText_MatchCallTuber_Ricky_Intro1 gText_MatchCallTuber_Ricky_Intro2 gText_MatchCallSisAndBro_LilaAndRoy_Strategy gText_MatchCallSisAndBro_LilaAndRoy_Pokemon gText_MatchCallSisAndBro_LilaAndRoy_Intro1 gText_MatchCallSisAndBro_LilaAndRoy_Intro2 gText_MatchCallCooltrainer_Cristin_Strategy gText_MatchCallCooltrainer_Cristin_Pokemon gText_MatchCallCooltrainer_Cristin_Intro1 gText_MatchCallCooltrainer_Cristin_Intro2 gText_MatchCallCooltrainer_Brooke_Strategy gText_MatchCallCooltrainer_Brooke_Pokemon gText_MatchCallCooltrainer_Brooke_Intro1 gText_MatchCallCooltrainer_Brooke_Intro2 gText_MatchCallCooltrainer_Wilton_Strategy gText_MatchCallCooltrainer_Wilton_Pokemon gText_MatchCallCooltrainer_Wilton_Intro1 gText_MatchCallCooltrainer_Wilton_Intro2 gText_MatchCallHexManiac_Valerie_Strategy gText_MatchCallHexManiac_Valerie_Pokemon gText_MatchCallHexManiac_Valerie_Intro1 gText_MatchCallHexManiac_Valerie_Intro2 gText_MatchCallLady_Cindy_Strategy gText_MatchCallLady_Cindy_Pokemon gText_MatchCallLady_Cindy_Intro1 gText_MatchCallLady_Cindy_Intro2 gText_MatchCallBeauty_Thalia_Strategy gText_MatchCallBeauty_Thalia_Pokemon gText_MatchCallBeauty_Thalia_Intro1 gText_MatchCallBeauty_Thalia_Intro2 gText_MatchCallBeauty_Jessica_Strategy gText_MatchCallBeauty_Jessica_Pokemon gText_MatchCallBeauty_Jessica_Intro1 gText_MatchCallBeauty_Jessica_Intro2 gText_MatchCallRichBoy_Winston_Strategy gText_MatchCallRichBoy_Winston_Pokemon gText_MatchCallRichBoy_Winston_Intro1 gText_MatchCallRichBoy_Winston_Intro2 gText_MatchCallPokeManiac_Steve_Strategy gText_MatchCallPokeManiac_Steve_Pokemon gText_MatchCallPokeManiac_Steve_Intro1 gText_MatchCallPokeManiac_Steve_Intro2 gText_MatchCallSwimmer_Tony_Strategy gText_MatchCallSwimmer_Tony_Pokemon gText_MatchCallSwimmer_Tony_Intro1 gText_MatchCallSwimmer_Tony_Intro2 gText_MatchCallBlackBelt_Nob_Strategy gText_MatchCallBlackBelt_Nob_Pokemon gText_MatchCallBlackBelt_Nob_Intro1 gText_MatchCallBlackBelt_Nob_Intro2 gText_MatchCallBlackBelt_Koji_Strategy gText_MatchCallBlackBelt_Koji_Pokemon gText_MatchCallBlackBelt_Koji_Intro1 gText_MatchCallBlackBelt_Koji_Intro2 gText_MatchCallGuitarist_Fernando_Strategy gText_MatchCallGuitarist_Fernando_Pokemon gText_MatchCallGuitarist_Fernando_Intro1 gText_MatchCallGuitarist_Fernando_Intro2 gText_MatchCallGuitarist_Dalton_Strategy gText_MatchCallGuitarist_Dalton_Pokemon gText_MatchCallGuitarist_Dalton_Intro1 gText_MatchCallGuitarist_Dalton_Intro2 gText_MatchCallKindler_Bernie_Strategy gText_MatchCallKindler_Bernie_Pokemon gText_MatchCallKindler_Bernie_Intro1 gText_MatchCallKindler_Bernie_Intro2 gText_MatchCallCamper_Ethan_Strategy gText_MatchCallCamper_Ethan_Pokemon gText_MatchCallCamper_Ethan_Intro1 gText_MatchCallCamper_Ethan_Intro2 gText_MatchCallOldCouple_JohnAndJay_Strategy gText_MatchCallOldCouple_JohnAndJay_Pokemon gText_MatchCallOldCouple_JohnAndJay_Intro1 gText_MatchCallOldCouple_JohnAndJay_Intro2 gText_MatchCallBugManiac_Jeffrey_Strategy gText_MatchCallBugManiac_Jeffrey_Pokemon gText_MatchCallBugManiac_Jeffrey_Intro1 gText_MatchCallBugManiac_Jeffrey_Intro2 gText_MatchCallPsychic_Cameron_Strategy gText_MatchCallPsychic_Cameron_Pokemon gText_MatchCallPsychic_Cameron_Intro1 gText_MatchCallPsychic_Cameron_Intro2 gText_MatchCallPsychic_Jacki_Strategy gText_MatchCallPsychic_Jacki_Pokemon gText_MatchCallPsychic_Jacki_Intro1 gText_MatchCallPsychic_Jacki_Intro2 gText_MatchCallGentleman_Walter_Strategy gText_MatchCallGentleman_Walter_Pokemon gText_MatchCallGentleman_Walter_Intro1 gText_MatchCallGentleman_Walter_Intro2 gText_MatchCallSchoolKid_Karen_Strategy gText_MatchCallSchoolKid_Karen_Pokemon gText_MatchCallSchoolKid_Karen_Intro1 gText_MatchCallSchoolKid_Karen_Intro2 gText_MatchCallSchoolKid_Jerry_Strategy gText_MatchCallSchoolKid_Jerry_Pokemon gText_MatchCallSchoolKid_Jerry_Intro1 gText_MatchCallSchoolKid_Jerry_Intro2 gText_MatchCallSrAndJr_AnnaAndMeg_Strategy gText_MatchCallSrAndJr_AnnaAndMeg_Pokemon gText_MatchCallSrAndJr_AnnaAndMeg_Intro1 gText_MatchCallSrAndJr_AnnaAndMeg_Intro2 gText_MatchCallPokefan_Isabel_Strategy gText_MatchCallPokefan_Isabel_Pokemon gText_MatchCallPokefan_Isabel_Intro1 gText_MatchCallPokefan_Isabel_Intro2 gText_MatchCallPokefan_Miguel_Strategy gText_MatchCallPokefan_Miguel_Pokemon gText_MatchCallPokefan_Miguel_Intro1 gText_MatchCallPokefan_Miguel_Intro2 gText_MatchCallExpert_Timothy_Strategy gText_MatchCallExpert_Timothy_Pokemon gText_MatchCallExpert_Timothy_Intro1 gText_MatchCallExpert_Timothy_Intro2 gText_MatchCallExpert_Shelby_Strategy gText_MatchCallExpert_Shelby_Pokemon gText_MatchCallExpert_Shelby_Intro1 gText_MatchCallExpert_Shelby_Intro2 gText_MatchCallYoungster_Calvin_Strategy gText_MatchCallYoungster_Calvin_Pokemon gText_MatchCallYoungster_Calvin_Intro1 gText_MatchCallYoungster_Calvin_Intro2 gText_MatchCallFisherman_Elliot_Strategy gText_MatchCallFisherman_Elliot_Pokemon gText_MatchCallFisherman_Elliot_Intro1 gText_MatchCallFisherman_Elliot_Intro2 gText_MatchCallTriathlete_Isaiah_Strategy gText_MatchCallTriathlete_Isaiah_Pokemon gText_MatchCallTriathlete_Isaiah_Intro1 gText_MatchCallTriathlete_Isaiah_Intro2 gText_MatchCallTriathlete_Maria_Strategy gText_MatchCallTriathlete_Maria_Pokemon gText_MatchCallTriathlete_Maria_Intro1 gText_MatchCallTriathlete_Maria_Intro2 gText_MatchCallTriathlete_Abigail_Strategy gText_MatchCallTriathlete_Abigail_Pokemon gText_MatchCallTriathlete_Abigail_Intro1 gText_MatchCallTriathlete_Abigail_Intro2 gText_MatchCallTriathlete_Dylan_Strategy gText_MatchCallTriathlete_Dylan_Pokemon gText_MatchCallTriathlete_Dylan_Intro1 gText_MatchCallTriathlete_Dylan_Intro2 gText_MatchCallTriathlete_Katelyn_Strategy gText_MatchCallTriathlete_Katelyn_Pokemon gText_MatchCallTriathlete_Katelyn_Intro1 gText_MatchCallTriathlete_Katelyn_Intro2 gText_MatchCallTriathlete_Benjamin_Strategy gText_MatchCallTriathlete_Benjamin_Pokemon gText_MatchCallTriathlete_Benjamin_Intro1 gText_MatchCallTriathlete_Benjamin_Intro2 gText_MatchCallTriathlete_Pablo_Strategy gText_MatchCallTriathlete_Pablo_Pokemon gText_MatchCallTriathlete_Pablo_Intro1 gText_MatchCallTriathlete_Pablo_Intro2 gText_MatchCallDragonTamer_Nicolas_Strategy gText_MatchCallDragonTamer_Nicolas_Pokemon gText_MatchCallDragonTamer_Nicolas_Intro1 gText_MatchCallDragonTamer_Nicolas_Intro2 gText_MatchCallBirdKeeper_Robert_Strategy gText_MatchCallBirdKeeper_Robert_Pokemon gText_MatchCallBirdKeeper_Robert_Intro1 gText_MatchCallBirdKeeper_Robert_Intro2 gText_MatchCallNinjaBoy_Lao_Strategy gText_MatchCallNinjaBoy_Lao_Pokemon gText_MatchCallNinjaBoy_Lao_Intro1 gText_MatchCallNinjaBoy_Lao_Intro2 gText_MatchCallBattleGirl_Cyndy_Strategy gText_MatchCallBattleGirl_Cyndy_Pokemon gText_MatchCallBattleGirl_Cyndy_Intro1 gText_MatchCallBattleGirl_Cyndy_Intro2 gText_MatchCallParasolLady_Madeline_Strategy gText_MatchCallParasolLady_Madeline_Pokemon gText_MatchCallParasolLady_Madeline_Intro1 gText_MatchCallParasolLady_Madeline_Intro2 gText_MatchCallSwimmer_Jenny_Strategy gText_MatchCallSwimmer_Jenny_Pokemon gText_MatchCallSwimmer_Jenny_Intro1 gText_MatchCallSwimmer_Jenny_Intro2 gText_MatchCallPicnicker_Diana_Strategy gText_MatchCallPicnicker_Diana_Pokemon gText_MatchCallPicnicker_Diana_Intro1 gText_MatchCallPicnicker_Diana_Intro2 gText_MatchCallTwins_AmyAndLiv_Strategy gText_MatchCallTwins_AmyAndLiv_Pokemon gText_MatchCallTwins_AmyAndLiv_Intro1 gText_MatchCallTwins_AmyAndLiv_Intro2 gText_MatchCallSailor_Ernest_Strategy gText_MatchCallSailor_Ernest_Pokemon gText_MatchCallSailor_Ernest_Intro1 gText_MatchCallSailor_Ernest_Intro2 gText_MatchCallSailor_Cory_Strategy gText_MatchCallSailor_Cory_Pokemon gText_MatchCallSailor_Cory_Intro1 gText_MatchCallSailor_Cory_Intro2 gText_MatchCallCollector_Edwin_Strategy gText_MatchCallCollector_Edwin_Pokemon gText_MatchCallCollector_Edwin_Intro1 gText_MatchCallCollector_Edwin_Intro2 gText_MatchCallPkmnBreeder_Lydia_Strategy gText_MatchCallPkmnBreeder_Lydia_Pokemon gText_MatchCallPkmnBreeder_Lydia_Intro1 gText_MatchCallPkmnBreeder_Lydia_Intro2 gText_MatchCallPkmnBreeder_Isaac_Strategy gText_MatchCallPkmnBreeder_Isaac_Pokemon gText_MatchCallPkmnBreeder_Isaac_Intro1 gText_MatchCallPkmnBreeder_Isaac_Intro2 gText_MatchCallPkmnBreeder_Gabrielle_Strategy gText_MatchCallPkmnBreeder_Gabrielle_Pokemon gText_MatchCallPkmnBreeder_Gabrielle_Intro1 gText_MatchCallPkmnBreeder_Gabrielle_Intro2 gText_MatchCallPkmnRanger_Catherine_Strategy gText_MatchCallPkmnRanger_Catherine_Pokemon gText_MatchCallPkmnRanger_Catherine_Intro1 gText_MatchCallPkmnRanger_Catherine_Intro2 gText_MatchCallPkmnRanger_Jackson_Strategy gText_MatchCallPkmnRanger_Jackson_Pokemon gText_MatchCallPkmnRanger_Jackson_Intro1 gText_MatchCallPkmnRanger_Jackson_Intro2 gText_MatchCallLass_Haley_Strategy gText_MatchCallLass_Haley_Pokemon gText_MatchCallLass_Haley_Intro1 gText_MatchCallLass_Haley_Intro2 gText_MatchCallBugCatcher_James_Strategy gText_MatchCallBugCatcher_James_Pokemon gText_MatchCallBugCatcher_James_Intro1 gText_MatchCallBugCatcher_James_Intro2 gText_MatchCallHiker_Trent_Strategy gText_MatchCallHiker_Trent_Pokemon gText_MatchCallHiker_Trent_Intro1 gText_MatchCallHiker_Trent_Intro2 gText_MatchCallHiker_Sawyer_Strategy gText_MatchCallHiker_Sawyer_Pokemon gText_MatchCallHiker_Sawyer_Intro1 gText_MatchCallHiker_Sawyer_Intro2 gText_MatchCallYoungCouple_LoisAndHal_Strategy gText_MatchCallYoungCouple_LoisAndHal_Pokemon gText_MatchCallYoungCouple_LoisAndHal_Intro1 gText_MatchCallYoungCouple_LoisAndHal_Intro2 gText_MatchCallPkmnTrainer_Wally_Strategy gText_MatchCallPkmnTrainer_Wally_Pokemon gText_MatchCallPkmnTrainer_Wally_Intro1 gText_MatchCallPkmnTrainer_Wally_Intro2 gText_MatchCallRockinWhiz_Roxanne_Strategy gText_MatchCallRockinWhiz_Roxanne_Pokemon gText_MatchCallRockinWhiz_Roxanne_Intro1 gText_MatchCallRockinWhiz_Roxanne_Intro2 gText_MatchCallTheBigHit_Brawly_Strategy gText_MatchCallTheBigHit_Brawly_Pokemon gText_MatchCallTheBigHit_Brawly_Intro1 gText_MatchCallTheBigHit_Brawly_Intro2 gText_MatchCallSwellShock_Wattson_Strategy gText_MatchCallSwellShock_Wattson_Pokemon gText_MatchCallSwellShock_Wattson_Intro1 gText_MatchCallSwellShock_Wattson_Intro2 gText_MatchCallPassionBurn_Flannery_Strategy gText_MatchCallPassionBurn_Flannery_Pokemon gText_MatchCallPassionBurn_Flannery_Intro1 gText_MatchCallPassionBurn_Flannery_Intro2 gText_MatchCallReliableOne_Dad_Strategy gText_MatchCallReliableOne_Dad_Pokemon gText_MatchCallReliableOne_Dad_Intro1 gText_MatchCallReliableOne_Dad_Intro2 gText_MatchCallSkyTamer_Winona_Strategy gText_MatchCallSkyTamer_Winona_Pokemon gText_MatchCallSkyTamer_Winona_Intro1 gText_MatchCallSkyTamer_Winona_Intro2 gText_MatchCallMysticDuo_TateAndLiza_Strategy gText_MatchCallMysticDuo_TateAndLiza_Pokemon gText_MatchCallMysticDuo_TateAndLiza_Intro1 gText_MatchCallMysticDuo_TateAndLiza_Intro2 gText_MatchCallDandyCharm_Juan_Strategy gText_MatchCallDandyCharm_Juan_Pokemon gText_MatchCallDandyCharm_Juan_Intro1 gText_MatchCallDandyCharm_Juan_Intro2 gText_MatchCallEliteFour_Sidney_Strategy gText_MatchCallEliteFour_Sidney_Pokemon gText_MatchCallEliteFour_Sidney_Intro1 gText_MatchCallEliteFour_Sidney_Intro2 gText_MatchCallEliteFour_Phoebe_Strategy gText_MatchCallEliteFour_Phoebe_Pokemon gText_MatchCallEliteFour_Phoebe_Intro1 gText_MatchCallEliteFour_Phoebe_Intro2 gText_MatchCallEliteFour_Glacia_Strategy gText_MatchCallEliteFour_Glacia_Pokemon gText_MatchCallEliteFour_Glacia_Intro1 gText_MatchCallEliteFour_Glacia_Intro2 gText_MatchCallEliteFour_Drake_Strategy gText_MatchCallEliteFour_Drake_Pokemon gText_MatchCallEliteFour_Drake_Intro1 gText_MatchCallEliteFour_Drake_Intro2 gText_MatchCallChampion_Wallace_Strategy gText_MatchCallChampion_Wallace_Pokemon gText_MatchCallChampion_Wallace_Intro1 gText_MatchCallChampion_Wallace_Intro2 gMatchCallFlavorTexts sMatchCallOptionsNoCheckPage sMatchCallOptionsHasCheckPage

/// `struct Pokenav_MatchCallMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_MatchCallMenu {
    pub optionCursorPos: u16,
    pub maxOptionId: u16,
    pub matchCallOptions: *mut u8,
    pub headerId: u16,
    pub numRegistered: u16,
    pub numSpecialTrainers: u16,
    pub initFinished: u32,
    pub loopedTaskId: u32,
    pub callback: Option<unsafe fn(*mut Pokenav_MatchCallMenu) -> u32>,
    pub matchCallEntries: CArray<PokenavMatchCallEntry, 99>,
}

unsafe impl Sync for Pokenav_MatchCallMenu {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_MatchCallMenu>() == 424);
    assert!(offset_of!(Pokenav_MatchCallMenu, optionCursorPos) == 0);
    assert!(offset_of!(Pokenav_MatchCallMenu, maxOptionId) == 2);
    assert!(offset_of!(Pokenav_MatchCallMenu, matchCallOptions) == 4);
    assert!(offset_of!(Pokenav_MatchCallMenu, headerId) == 8);
    assert!(offset_of!(Pokenav_MatchCallMenu, numRegistered) == 10);
    assert!(offset_of!(Pokenav_MatchCallMenu, numSpecialTrainers) == 12);
    assert!(offset_of!(Pokenav_MatchCallMenu, initFinished) == 16);
    assert!(offset_of!(Pokenav_MatchCallMenu, loopedTaskId) == 20);
    assert!(offset_of!(Pokenav_MatchCallMenu, callback) == 24);
    assert!(offset_of!(Pokenav_MatchCallMenu, matchCallEntries) == 28);
};

static gMatchCallFlavorTexts: Table<CArray<CArray<*mut u8, 4>, 78>> =
    Table((&raw const crate::data::pokenav_match_call_list::gMatchCallFlavorTexts).cast());
static sMatchCallOptionsHasCheckPage: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokenav_match_call_list::sMatchCallOptionsHasCheckPage).cast());
static sMatchCallOptionsNoCheckPage: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokenav_match_call_list::sMatchCallOptionsNoCheckPage).cast());

pub unsafe fn PokenavCallback_Init_MatchCall() -> u32 {
    let state: *mut Pokenav_MatchCallMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN, 424) as *mut Pokenav_MatchCallMenu;
    if state.is_null() {
        return FALSE as u32;
    }
    (*state).callback = Some(CB2_HandleMatchCallInput);
    (*state).headerId = 0;
    (*state).initFinished = FALSE as u32;
    (*state).loopedTaskId = CreateLoopedTask(Some(LoopedTask_BuildMatchCallList), 1);
    TRUE as u32
}
pub unsafe fn GetMatchCallCallback() -> u32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).callback.unwrap_unchecked()(state)
}
pub unsafe fn FreeMatchCallSubstruct1() {
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN);
}
pub(crate) unsafe fn CB2_HandleMatchCallInput(state: *mut Pokenav_MatchCallMenu) -> u32 {
    let mut selection: i32 = 0;
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return POKENAV_MC_FUNC_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return POKENAV_MC_FUNC_DOWN;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        return POKENAV_MC_FUNC_PG_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        return POKENAV_MC_FUNC_PG_DOWN;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*state).callback = Some(CB2_HandleMatchCallOptionsInput);
        (*state).optionCursorPos = 0;
        selection = PokenavList_GetSelectedIndex() as i32;
        if (*state).matchCallEntries[selection].isSpecialTrainer == 0
            || MatchCall_HasCheckPage((*state).matchCallEntries[selection].headerId as u32) != 0
        {
            (*state).matchCallOptions = sMatchCallOptionsHasCheckPage.as_ptr().cast_mut();
            (*state).maxOptionId = 2;
        } else {
            (*state).matchCallOptions = sMatchCallOptionsNoCheckPage.as_ptr().cast_mut();
            (*state).maxOptionId = 1;
        }
        return POKENAV_MC_FUNC_SELECT;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        if GetPokenavMode() != POKENAV_MODE_FORCE_CALL_READY {
            (*state).callback = Some(GetExitMatchCallMenuId);
            return POKENAV_MC_FUNC_EXIT;
        } else {
            PlaySE(SE_FAILURE);
        }
    }
    POKENAV_MC_FUNC_NONE
}
pub(crate) unsafe fn GetExitMatchCallMenuId(state: *mut Pokenav_MatchCallMenu) -> u32 {
    POKENAV_MAIN_MENU_CURSOR_ON_MATCH_CALL
}
pub(crate) unsafe fn CB2_HandleMatchCallOptionsInput(state: *mut Pokenav_MatchCallMenu) -> u32 {
    if gMain.newKeys as i32 & DPAD_UP != 0 && (*state).optionCursorPos != 0 {
        (*state).optionCursorPos -= 1;
        return POKENAV_MC_FUNC_MOVE_OPTIONS_CURSOR;
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0 && (*state).optionCursorPos < (*state).maxOptionId {
        (*state).optionCursorPos += 1;
        return POKENAV_MC_FUNC_MOVE_OPTIONS_CURSOR;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        match *(*state).matchCallOptions.at((*state).optionCursorPos) {
            MATCH_CALL_OPTION_CANCEL => {
                (*state).callback = Some(CB2_HandleMatchCallInput);
                return POKENAV_MC_FUNC_CANCEL;
            }
            MATCH_CALL_OPTION_CALL => {
                if GetPokenavMode() == POKENAV_MODE_FORCE_CALL_READY {
                    SetPokenavMode(POKENAV_MODE_FORCE_CALL_EXIT);
                }
                (*state).callback = Some(CB2_HandleCallExitInput);
                if ShouldDoNearbyMessage() != 0 {
                    return POKENAV_MC_FUNC_NEARBY_MSG;
                }
                return POKENAV_MC_FUNC_CALL_MSG;
            }
            MATCH_CALL_OPTION_CHECK => {
                (*state).callback = Some(CB2_HandleCheckPageInput);
                return POKENAV_MC_FUNC_SHOW_CHECK_PAGE;
            }
            _ => {}
        }
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*state).callback = Some(CB2_HandleMatchCallInput);
        return POKENAV_MC_FUNC_CANCEL;
    }
    POKENAV_MC_FUNC_NONE
}
pub(crate) unsafe fn CB2_HandleCheckPageInput(state: *mut Pokenav_MatchCallMenu) -> u32 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return POKENAV_MC_FUNC_CHECK_PAGE_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return POKENAV_MC_FUNC_CHECK_PAGE_DOWN;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*state).callback = Some(CB2_HandleMatchCallInput);
        return POKENAV_MC_FUNC_EXIT_CHECK_PAGE;
    }
    POKENAV_MC_FUNC_NONE
}
pub(crate) unsafe fn CB2_HandleCallExitInput(state: *mut Pokenav_MatchCallMenu) -> u32 {
    if gMain.newKeys as i32 & 3 != 0 {
        (*state).callback = Some(CB2_HandleMatchCallInput);
        return POKENAV_MC_FUNC_EXIT_CALL;
    }
    POKENAV_MC_FUNC_NONE
}
pub(crate) unsafe fn LoopedTask_BuildMatchCallList(taskState: i32) -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    match taskState {
        0 => {
            (*state).headerId = 0;
            (*state).numRegistered = 0;
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            i = 0;
            j = (*state).headerId as i32;
            while i < 30 {
                if MatchCall_GetEnabled(j as u32) != 0 {
                    (*state).matchCallEntries[(*state).numRegistered].headerId = j as u16;
                    (*state).matchCallEntries[(*state).numRegistered].isSpecialTrainer = TRUE;
                    (*state).matchCallEntries[(*state).numRegistered].mapSec =
                        MatchCall_GetMapSec(j as u32);
                    (*state).numRegistered += 1;
                }
                if ({
                    (*state).headerId += 1;
                    (*state).headerId
                }) >= MC_HEADER_COUNT
                {
                    (*state).numSpecialTrainers = (*state).headerId;
                    (*state).headerId = 0;
                    return LT_INC_AND_CONTINUE;
                }
                i += 1;
                j += 1;
            }
            return LT_CONTINUE;
        }
        2 => {
            i = 0;
            j = (*state).headerId as i32;
            while i < 30 {
                if MatchCall_HasRematchId((*state).headerId as u32) == 0
                    && IsRematchEntryRegistered((*state).headerId as i32) != 0
                {
                    (*state).matchCallEntries[(*state).numRegistered].headerId = (*state).headerId;
                    (*state).matchCallEntries[(*state).numRegistered].isSpecialTrainer = FALSE;
                    (*state).matchCallEntries[(*state).numRegistered].mapSec =
                        GetMatchTableMapSectionId(j);
                    (*state).numRegistered += 1;
                }
                if ({
                    (*state).headerId += 1;
                    (*state).headerId
                }) > 77
                {
                    return LT_INC_AND_CONTINUE;
                }
                i += 1;
                j += 1;
            }
            return LT_CONTINUE;
        }
        3 => {
            (*state).initFinished = TRUE as u32;
        }
        _ => {}
    }
    LT_FINISH
}
pub unsafe fn IsRematchEntryRegistered(rematchIndex: i32) -> u32 {
    if rematchIndex < REMATCH_TABLE_ENTRIES {
        return FlagGet(TRAINER_REGISTERED_FLAGS_START + rematchIndex as u16) as u32;
    }
    FALSE as u32
}
pub unsafe fn IsMatchCallListInitFinished() -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).initFinished as i32
}
pub unsafe fn GetNumberRegistered() -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).numRegistered as i32
}
unsafe fn GetNumSpecialTrainers() -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).numSpecialTrainers as i32
}
unsafe fn GetNumNormalTrainers() -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).numRegistered as i32 - (*state).numSpecialTrainers as i32
}
unsafe fn GetNormalTrainerHeaderId(mut index: i32) -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    index += (*state).numSpecialTrainers as i32;
    if index >= (*state).numRegistered as i32 {
        return REMATCH_TABLE_ENTRIES;
    }
    (*state).matchCallEntries[index].headerId as i32
}
pub unsafe fn GetMatchCallList() -> *mut PokenavMatchCallEntry {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).matchCallEntries.as_mut_ptr()
}
pub unsafe fn GetMatchCallMapSec(index: i32) -> u16 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).matchCallEntries[index].mapSec as u16
}
pub unsafe fn ShouldDrawRematchPokeballIcon(mut index: i32) -> u32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    if (*state).matchCallEntries[index].isSpecialTrainer == 0 {
        index = (*state).matchCallEntries[index].headerId as i32;
    } else {
        index =
            MatchCall_GetRematchTableIdx((*state).matchCallEntries[index].headerId as u32) as i32;
    }
    if index == REMATCH_TABLE_ENTRIES {
        return FALSE as u32;
    }
    ((*gSaveBlock1Ptr).trainerRematches[index] != 0) as u32
}
pub unsafe fn GetMatchCallTrainerPic(mut index: i32) -> i32 {
    let mut headerId: i32 = 0;
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    if (*state).matchCallEntries[index].isSpecialTrainer == 0 {
        index = GetTrainerIdxByRematchIdx((*state).matchCallEntries[index].headerId as u32) as i32;
        return (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [index]
            .trainerPic as i32;
    }
    headerId = (*state).matchCallEntries[index].headerId as i32;
    index = MatchCall_GetRematchTableIdx(headerId as u32) as i32;
    if index != REMATCH_TABLE_ENTRIES {
        index = GetTrainerIdxByRematchIdx(index as u32) as i32;
        return (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
            [index]
            .trainerPic as i32;
    }
    index = MatchCall_GetOverrideFacilityClass(headerId as u32);
    (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex).cast::<CArray<u8, 0>>())[index]
        as i32
}
pub unsafe fn GetMatchCallMessageText(index: i32, newRematchRequest: *mut u8) -> *mut u8 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    *newRematchRequest = FALSE;
    if Overworld_MapTypeAllowsTeleportAndFly(gMapHeader.mapType) == 0 {
        return (*(&raw const crate::data::strings::gText_CallCantBeMadeHere)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    if (*state).matchCallEntries[index].isSpecialTrainer == 0 {
        *newRematchRequest = SelectMatchCallMessage(
            GetTrainerIdxByRematchIdx((*state).matchCallEntries[index].headerId as u32) as i32,
            gStringVar4.as_mut_ptr(),
        ) as u8;
    } else {
        MatchCall_GetMessage(
            (*state).matchCallEntries[index].headerId as u32,
            gStringVar4.as_mut_ptr(),
        );
    }
    gStringVar4.as_mut_ptr()
}
pub unsafe fn GetMatchCallFlavorText(index: i32, checkPageEntry: i32) -> *mut u8 {
    let mut rematchId: i32 = 0;
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    if (*state).matchCallEntries[index].isSpecialTrainer != 0 {
        rematchId =
            MatchCall_GetRematchTableIdx((*state).matchCallEntries[index].headerId as u32) as i32;
        if rematchId == REMATCH_TABLE_ENTRIES {
            return MatchCall_GetOverrideFlavorText(
                (*state).matchCallEntries[index].headerId as u32,
                checkPageEntry as u32,
            );
        }
    } else {
        rematchId = (*state).matchCallEntries[index].headerId as i32;
    }
    gMatchCallFlavorTexts[rematchId][checkPageEntry]
}
pub unsafe fn GetMatchCallOptionCursorPos() -> u16 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    (*state).optionCursorPos
}
pub unsafe fn GetMatchCallOptionId(optionId: i32) -> u16 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    if ((*state).maxOptionId as i32) < optionId {
        return MATCH_CALL_OPTION_COUNT as u16;
    }
    *(*state).matchCallOptions.at(optionId) as u16
}
pub unsafe fn BufferMatchCallNameAndDesc(matchCallEntry: *mut PokenavMatchCallEntry, str: *mut u8) {
    let mut trainerName: *mut u8 = null_mut();
    let mut className: *mut u8 = null_mut();
    if (*matchCallEntry).isSpecialTrainer == 0 {
        let index: i32 = GetTrainerIdxByRematchIdx((*matchCallEntry).headerId as u32) as i32;
        let trainer: *mut Trainer =
            (&raw const (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[index])
                .cast_mut();
        let class: i32 = (*trainer).trainerClass as i32;
        className = (*(&raw const crate::data::data_tables::gTrainerClassNames)
            .cast::<CArray<CArray<u8, 13>, 0>>())[class]
            .as_ptr()
            .cast_mut();
        trainerName = (*trainer).trainerName.as_mut_ptr();
    } else {
        MatchCall_GetNameAndDesc(
            (*matchCallEntry).headerId as u32,
            &raw mut className,
            &raw mut trainerName,
        );
    }
    if !className.is_null() && !trainerName.is_null() {
        let str2: *mut u8 = GetStringClearToWidth(str, FONT_NARROW as i32, className, 69);
        GetStringClearToWidth(str2, FONT_NARROW as i32, trainerName, 51);
    } else {
        GetStringClearToWidth(str, FONT_NARROW as i32, null_mut(), 120);
    }
}
pub unsafe fn GetMatchTableMapSectionId(rematchIndex: i32) -> u8 {
    let mapGroup: i32 = (*(&raw const crate::data::battle_setup::gRematchTable)
        .cast::<CArray<RematchTrainer, 78>>())[rematchIndex]
        .mapGroup as i32;
    let mapNum: i32 = (*(&raw const crate::data::battle_setup::gRematchTable)
        .cast::<CArray<RematchTrainer, 78>>())[rematchIndex]
        .mapNum as i32;
    (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).regionMapSectionId
}
pub unsafe fn GetIndexDeltaOfNextCheckPageDown(mut index: i32) -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    let mut count: i32 = 1;
    while ({
        index += 1;
        index
    }) < (*state).numRegistered as i32
    {
        if (*state).matchCallEntries[index].isSpecialTrainer == 0 {
            return count;
        }
        if MatchCall_HasCheckPage((*state).matchCallEntries[index].headerId as u32) != 0 {
            return count;
        }
        count += 1;
    }
    0
}
pub unsafe fn GetIndexDeltaOfNextCheckPageUp(mut index: i32) -> i32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    let mut count: i32 = -1;
    while ({
        index -= 1;
        index
    }) >= 0
    {
        if (*state).matchCallEntries[index].isSpecialTrainer == 0 {
            return count;
        }
        if MatchCall_HasCheckPage((*state).matchCallEntries[index].headerId as u32) != 0 {
            return count;
        }
        count -= 1;
    }
    0
}
unsafe fn HasRematchEntry() -> u32 {
    let mut i: i32 = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if IsRematchEntryRegistered(i) != 0 && (*gSaveBlock1Ptr).trainerRematches[i] != 0 {
            return TRUE as u32;
        }
        i += 1;
    }
    for i in 0..(MC_HEADER_COUNT as i32) {
        if MatchCall_GetEnabled(i as u32) != 0 {
            let index: i32 = MatchCall_GetRematchTableIdx(i as u32) as i32;
            if (*gSaveBlock1Ptr).trainerRematches[index] != 0 {
                return TRUE as u32;
            }
        }
    }
    FALSE as u32
}
unsafe fn ShouldDoNearbyMessage() -> u32 {
    let state: *mut Pokenav_MatchCallMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MATCH_CALL_MAIN) as *mut Pokenav_MatchCallMenu;
    let selection: i32 = PokenavList_GetSelectedIndex() as i32;
    if (*state).matchCallEntries[selection].isSpecialTrainer == 0 {
        if GetMatchCallMapSec(selection) == gMapHeader.regionMapSectionId as u16
            && (*gSaveBlock1Ptr).trainerRematches[(*state).matchCallEntries[selection].headerId]
                == 0
        {
            return TRUE as u32;
        }
    } else {
        if (*state).matchCallEntries[selection].headerId == MC_HEADER_WATTSON
            && GetMatchCallMapSec(selection) == gMapHeader.regionMapSectionId as u16
            && FlagGet(FLAG_BADGE05_GET) == TRUE
            && FlagGet(FLAG_WATTSON_REMATCH_AVAILABLE) == 0
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
