//! Translated from `src/battle_tower.c` by tools/rustport/c2rs.py.
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
    clippy::explicit_counter_loop,
    clippy::if_same_then_else,
    clippy::manual_is_multiple_of,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gGameLanguage;
use crate::agb_main::gMain;
use crate::apprentice::{
    BufferApprenticeChallengeText, GetApprenticeNameInLanguage, ResetApprenticeStruct,
};
use crate::battle_factory::{FillFactoryBrainParty, GetFactoryMonFixedIV, SetMonMoveAvoidReturn};
use crate::battle_main::gBattlerPartyIndexes;
use crate::battle_main::{
    CB2_InitBattle, gBattleMons, gBattleOutcome, gBattleScripting, gBattleTypeFlags,
};
use crate::battle_setup::{
    BattleSetup_ConfigureTrainerBattle, GetSpecialBattleTransition, gPartnerTrainerId,
    gTrainerBattleOpponent_A, gTrainerBattleOpponent_B,
};
use crate::battle_transition::{BattleTransition_StartOnField, IsBattleTransitionDone};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::easy_chat::ConvertEasyChatWordsToString;
use crate::event_data::{FlagClear, FlagGet, VarGet, VarSet};
use crate::ffi::{
    gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_LastTalked,
    gSpecialVar_Result,
};
use crate::field_message_box::ShowFieldMessage;
use crate::frontier_util::{
    CopyFrontierBrainTrainerName, CreateFrontierBrainPokemon, GetCurrentFacilityWinStreak,
    GetFrontierBrainTrainerClass, GetFrontierBrainTrainerPicIndex, IsFrontierBrainFemale,
    ResetFrontierTrainerIds, SaveGameFrontier, SetFrontierBrainObjEventGfx_2,
};
use crate::gym_leader_rematch::UpdateGymLeaderRematch;
use crate::international_string_util::TVShowConvertInternationalString;
use crate::link::gBlockRecvBuffer;
use crate::link::{
    BitmaskAllOtherLinkPlayers, GetBlockReceivedStatus, GetMultiplayerId, IsLinkTaskFinished,
    ResetBlockReceivedFlags, SendBlock, SetCloseLinkCallback, gReceivedRemoteLinkPlayers,
    gWirelessCommType,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::new_game::CopyTrainerId;
use crate::overworld::{
    CB2_ReturnToFieldContinueScriptPlayMapMusic, IncrementGameStat, SetDynamicWarp,
};
use crate::pokemon::{
    CalculateMonStats, ConvertPokemonToBattleTowerPokemon, CreateApprenticeMon,
    CreateBattleTowerMon, CreateBattleTowerMon_HandleLevel, CreateMon, CreateMonWithEVSpread,
    CreateMonWithEVSpreadNatureOTID, CreateTask_PlayMapChosenOrBattleBGM, GetMonData2, GetMonData3,
    GetNatureFromPersonality, IsShinyOtIdPersonality, PlayMapChosenOrBattleBGM, SetMonData,
    SetMonMoveSlot, ZeroEnemyPartyMons, gEnemyParty, gPlayerParty,
};
use crate::random::Random;
use crate::recorded_battle::{
    GetRecordedBattleApprenticeId, GetRecordedBattleApprenticeLanguage,
    GetRecordedBattleRecordMixFriendClass, GetRecordedBattleRecordMixFriendLanguage,
    GetRecordedBattleRecordMixFriendName, RecordedBattle_SaveBattleOutcome,
};
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringCopy_PlayerName};
use crate::string_util::{ConvertInternationalString, StripExtCtrlCodes};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::DestroyTask;
use crate::text::GetStringWidth;
use crate::trainer_see::gApproachingTrainerId;
use crate::tv::{GetRibbonCount, HideBattleTowerReporter, TryPutSpotTheCutiesOnAir};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// Data tables (translate with cdata.py): gBattleFrontierHeldItems gBattleFrontierTrainerMons_Brady gBattleFrontierTrainerMons_Conner gBattleFrontierTrainerMons_Bradley gBattleFrontierTrainerMons_Cybil gBattleFrontierTrainerMons_Rodette gBattleFrontierTrainerMons_Peggy gBattleFrontierTrainerMons_Keith gBattleFrontierTrainerMons_Grayson gBattleFrontierTrainerMons_Glenn gBattleFrontierTrainerMons_Liliana gBattleFrontierTrainerMons_Elise gBattleFrontierTrainerMons_Zoey gBattleFrontierTrainerMons_Manuel gBattleFrontierTrainerMons_Russ gBattleFrontierTrainerMons_Dustin gBattleFrontierTrainerMons_Tina gBattleFrontierTrainerMons_Gillian gBattleFrontierTrainerMons_Zoe gBattleFrontierTrainerMons_Chen gBattleFrontierTrainerMons_Al gBattleFrontierTrainerMons_Mitch gBattleFrontierTrainerMons_Anne gBattleFrontierTrainerMons_Alize gBattleFrontierTrainerMons_Lauren gBattleFrontierTrainerMons_Kipp gBattleFrontierTrainerMons_Jason gBattleFrontierTrainerMons_John gBattleFrontierTrainerMons_Ann gBattleFrontierTrainerMons_Eileen gBattleFrontierTrainerMons_Carlie gBattleFrontierTrainerMons_Gordon gBattleFrontierTrainerMons_Ayden gBattleFrontierTrainerMons_Marco gBattleFrontierTrainerMons_Cierra gBattleFrontierTrainerMons_Marcy gBattleFrontierTrainerMons_Kathy gBattleFrontierTrainerMons_Peyton gBattleFrontierTrainerMons_Julian gBattleFrontierTrainerMons_Quinn gBattleFrontierTrainerMons_Haylee gBattleFrontierTrainerMons_Amanda gBattleFrontierTrainerMons_Stacy gBattleFrontierTrainerMons_Rafael gBattleFrontierTrainerMons_Oliver gBattleFrontierTrainerMons_Payton gBattleFrontierTrainerMons_Pamela gBattleFrontierTrainerMons_Eliza gBattleFrontierTrainerMons_Marisa gBattleFrontierTrainerMons_Lewis gBattleFrontierTrainerMons_Yoshi gBattleFrontierTrainerMons_Destin gBattleFrontierTrainerMons_Keon gBattleFrontierTrainerMons_Stuart gBattleFrontierTrainerMons_Nestor gBattleFrontierTrainerMons_Derrick gBattleFrontierTrainerMons_Bryson gBattleFrontierTrainerMons_Clayton gBattleFrontierTrainerMons_Trenton gBattleFrontierTrainerMons_Jenson gBattleFrontierTrainerMons_Wesley gBattleFrontierTrainerMons_Anton gBattleFrontierTrainerMons_Lawson gBattleFrontierTrainerMons_Sammy gBattleFrontierTrainerMons_Arnie gBattleFrontierTrainerMons_Adrian gBattleFrontierTrainerMons_Tristan gBattleFrontierTrainerMons_Juliana gBattleFrontierTrainerMons_Rylee gBattleFrontierTrainerMons_Chelsea gBattleFrontierTrainerMons_Danela gBattleFrontierTrainerMons_Lizbeth gBattleFrontierTrainerMons_Amelia gBattleFrontierTrainerMons_Jillian gBattleFrontierTrainerMons_Abbie gBattleFrontierTrainerMons_Briana gBattleFrontierTrainerMons_Antonio gBattleFrontierTrainerMons_Jaden gBattleFrontierTrainerMons_Dakota gBattleFrontierTrainerMons_Brayden gBattleFrontierTrainerMons_Corson gBattleFrontierTrainerMons_Trevin gBattleFrontierTrainerMons_Patrick gBattleFrontierTrainerMons_Kaden gBattleFrontierTrainerMons_Maxwell gBattleFrontierTrainerMons_Daryl gBattleFrontierTrainerMons_Kenneth gBattleFrontierTrainerMons_Rich gBattleFrontierTrainerMons_Caden gBattleFrontierTrainerMons_Marlon gBattleFrontierTrainerMons_Nash gBattleFrontierTrainerMons_Robby gBattleFrontierTrainerMons_Reece gBattleFrontierTrainerMons_Kathryn gBattleFrontierTrainerMons_Ellen gBattleFrontierTrainerMons_Ramon gBattleFrontierTrainerMons_Arthur gBattleFrontierTrainerMons_Alondra gBattleFrontierTrainerMons_Adriana gBattleFrontierTrainerMons_Malik gBattleFrontierTrainerMons_Jill gBattleFrontierTrainerMons_Erik gBattleFrontierTrainerMons_Yazmin gBattleFrontierTrainerMons_Jamal gBattleFrontierTrainerMons_Leslie gBattleFrontierTrainerMons_Dave gBattleFrontierTrainerMons_Carlo gBattleFrontierTrainerMons_Emilia gBattleFrontierTrainerMons_Dalia gBattleFrontierTrainerMons_Hitomi gBattleFrontierTrainerMons_Ricardo gBattleFrontierTrainerMons_Shizuka gBattleFrontierTrainerMons_Joana gBattleFrontierTrainerMons_Kelly gBattleFrontierTrainerMons_Rayna gBattleFrontierTrainerMons_Evan gBattleFrontierTrainerMons_Jordan gBattleFrontierTrainerMons_Joel gBattleFrontierTrainerMons_Kristen gBattleFrontierTrainerMons_Selphy gBattleFrontierTrainerMons_Chloe gBattleFrontierTrainerMons_Norton gBattleFrontierTrainerMons_Lukas gBattleFrontierTrainerMons_Zach gBattleFrontierTrainerMons_Kaitlyn gBattleFrontierTrainerMons_Breanna gBattleFrontierTrainerMons_Kendra gBattleFrontierTrainerMons_Molly gBattleFrontierTrainerMons_Jazmin gBattleFrontierTrainerMons_Kelsey gBattleFrontierTrainerMons_Jalen gBattleFrontierTrainerMons_Griffen gBattleFrontierTrainerMons_Xander gBattleFrontierTrainerMons_Marvin gBattleFrontierTrainerMons_Brennan gBattleFrontierTrainerMons_Baley gBattleFrontierTrainerMons_Zackary gBattleFrontierTrainerMons_Gabriel gBattleFrontierTrainerMons_Emily gBattleFrontierTrainerMons_Jordyn gBattleFrontierTrainerMons_Sofia gBattleFrontierTrainerMons_Braden gBattleFrontierTrainerMons_Kayden gBattleFrontierTrainerMons_Cooper gBattleFrontierTrainerMons_Julia gBattleFrontierTrainerMons_Amara gBattleFrontierTrainerMons_Lynn gBattleFrontierTrainerMons_Jovan gBattleFrontierTrainerMons_Dominic gBattleFrontierTrainerMons_Nikolas gBattleFrontierTrainerMons_Valeria gBattleFrontierTrainerMons_Delaney gBattleFrontierTrainerMons_Meghan gBattleFrontierTrainerMons_Roberto gBattleFrontierTrainerMons_Damian gBattleFrontierTrainerMons_Brody gBattleFrontierTrainerMons_Graham gBattleFrontierTrainerMons_Tylor gBattleFrontierTrainerMons_Jaren gBattleFrontierTrainerMons_Cordell gBattleFrontierTrainerMons_Jazlyn gBattleFrontierTrainerMons_Zachery gBattleFrontierTrainerMons_Johan gBattleFrontierTrainerMons_Shea gBattleFrontierTrainerMons_Kaila gBattleFrontierTrainerMons_Isiah gBattleFrontierTrainerMons_Garrett gBattleFrontierTrainerMons_Haylie gBattleFrontierTrainerMons_Megan gBattleFrontierTrainerMons_Issac gBattleFrontierTrainerMons_Quinton gBattleFrontierTrainerMons_Salma gBattleFrontierTrainerMons_Ansley gBattleFrontierTrainerMons_Holden gBattleFrontierTrainerMons_Luca gBattleFrontierTrainerMons_Jamison gBattleFrontierTrainerMons_Gunnar gBattleFrontierTrainerMons_Craig gBattleFrontierTrainerMons_Pierce gBattleFrontierTrainerMons_Regina gBattleFrontierTrainerMons_Alison gBattleFrontierTrainerMons_Hank gBattleFrontierTrainerMons_Earl gBattleFrontierTrainerMons_Ramiro gBattleFrontierTrainerMons_Hunter gBattleFrontierTrainerMons_Aiden gBattleFrontierTrainerMons_Xavier gBattleFrontierTrainerMons_Clinton gBattleFrontierTrainerMons_Jesse gBattleFrontierTrainerMons_Eduardo gBattleFrontierTrainerMons_Hal gBattleFrontierTrainerMons_Gage gBattleFrontierTrainerMons_Arnold gBattleFrontierTrainerMons_Jarrett gBattleFrontierTrainerMons_Garett gBattleFrontierTrainerMons_Emanuel gBattleFrontierTrainerMons_Gustavo gBattleFrontierTrainerMons_Kameron gBattleFrontierTrainerMons_Alfredo gBattleFrontierTrainerMons_Ruben gBattleFrontierTrainerMons_Lamar gBattleFrontierTrainerMons_Jaxon gBattleFrontierTrainerMons_Logan gBattleFrontierTrainerMons_Emilee gBattleFrontierTrainerMons_Josie gBattleFrontierTrainerMons_Armando gBattleFrontierTrainerMons_Skyler gBattleFrontierTrainerMons_Ruth gBattleFrontierTrainerMons_Melody gBattleFrontierTrainerMons_Pedro gBattleFrontierTrainerMons_Erick gBattleFrontierTrainerMons_Elaine gBattleFrontierTrainerMons_Joyce gBattleFrontierTrainerMons_Todd gBattleFrontierTrainerMons_Gavin gBattleFrontierTrainerMons_Malory gBattleFrontierTrainerMons_Esther gBattleFrontierTrainerMons_Oscar gBattleFrontierTrainerMons_Wilson gBattleFrontierTrainerMons_Clare gBattleFrontierTrainerMons_Tess gBattleFrontierTrainerMons_Leon gBattleFrontierTrainerMons_Alonzo gBattleFrontierTrainerMons_Vince gBattleFrontierTrainerMons_Bryon gBattleFrontierTrainerMons_Ava gBattleFrontierTrainerMons_Miriam gBattleFrontierTrainerMons_Carrie gBattleFrontierTrainerMons_Gillian2 gBattleFrontierTrainerMons_Tyler gBattleFrontierTrainerMons_Chaz gBattleFrontierTrainerMons_Nelson gBattleFrontierTrainerMons_Shania gBattleFrontierTrainerMons_Stella gBattleFrontierTrainerMons_Dorine gBattleFrontierTrainerMons_Maddox gBattleFrontierTrainerMons_Davin gBattleFrontierTrainerMons_Trevon gBattleFrontierTrainerMons_Mateo gBattleFrontierTrainerMons_Bret gBattleFrontierTrainerMons_Raul gBattleFrontierTrainerMons_Kay gBattleFrontierTrainerMons_Elena gBattleFrontierTrainerMons_Alana gBattleFrontierTrainerMons_Alexas gBattleFrontierTrainerMons_Weston gBattleFrontierTrainerMons_Jasper gBattleFrontierTrainerMons_Nadia gBattleFrontierTrainerMons_Miranda gBattleFrontierTrainerMons_Emma gBattleFrontierTrainerMons_Rolando gBattleFrontierTrainerMons_Stanly gBattleFrontierTrainerMons_Dario gBattleFrontierTrainerMons_Karlee gBattleFrontierTrainerMons_Jaylin gBattleFrontierTrainerMons_Ingrid gBattleFrontierTrainerMons_Delilah gBattleFrontierTrainerMons_Carly gBattleFrontierTrainerMons_Lexie gBattleFrontierTrainerMons_Miller gBattleFrontierTrainerMons_Marv gBattleFrontierTrainerMons_Layton gBattleFrontierTrainerMons_Brooks gBattleFrontierTrainerMons_Gregory gBattleFrontierTrainerMons_Reese gBattleFrontierTrainerMons_Mason gBattleFrontierTrainerMons_Toby gBattleFrontierTrainerMons_Dorothy gBattleFrontierTrainerMons_Piper gBattleFrontierTrainerMons_Finn gBattleFrontierTrainerMons_Samir gBattleFrontierTrainerMons_Fiona gBattleFrontierTrainerMons_Gloria gBattleFrontierTrainerMons_Nico gBattleFrontierTrainerMons_Jeremy gBattleFrontierTrainerMons_Caitlin gBattleFrontierTrainerMons_Reena gBattleFrontierTrainerMons_Avery gBattleFrontierTrainerMons_Liam gBattleFrontierTrainerMons_Theo gBattleFrontierTrainerMons_Bailey gBattleFrontierTrainerMons_Hugo gBattleFrontierTrainerMons_Bryce gBattleFrontierTrainerMons_Gideon gBattleFrontierTrainerMons_Triston gBattleFrontierTrainerMons_Charles gBattleFrontierTrainerMons_Raymond gBattleFrontierTrainerMons_Dirk gBattleFrontierTrainerMons_Harold gBattleFrontierTrainerMons_Omar gBattleFrontierTrainerMons_Peter gBattleFrontierTrainerMons_Dev gBattleFrontierTrainerMons_Corey gBattleFrontierTrainerMons_Andre gBattleFrontierTrainerMons_Ferris gBattleFrontierTrainerMons_Alivia gBattleFrontierTrainerMons_Paige gBattleFrontierTrainerMons_Anya gBattleFrontierTrainerMons_Dawn gBattleFrontierTrainerMons_Abby gBattleFrontierTrainerMons_Gretel gBattleFrontierTrainers gBattleFrontierMons gTowerMaleFacilityClasses gTowerFemaleFacilityClasses gTowerMaleTrainerGfxIds gTowerFemaleTrainerGfxIds sRubyFacilityClassToEmerald sPartnerApprenticeTexts1 sPartnerApprenticeTexts2 sPartnerApprenticeTexts3 sPartnerApprenticeTexts4 sPartnerApprenticeTexts5 sPartnerApprenticeTexts6 sPartnerApprenticeTexts7 sPartnerApprenticeTexts8 sPartnerApprenticeTexts9 sPartnerApprenticeTexts10 sPartnerApprenticeTexts11 sPartnerApprenticeTexts12 sPartnerApprenticeTexts13 sPartnerApprenticeTexts14 sPartnerApprenticeTexts15 sPartnerApprenticeTexts16 sPartnerTextsLass sPartnerTextsYoungster sPartnerTextsHiker sPartnerTextsBeauty sPartnerTextsFisherman sPartnerTextsLady sPartnerTextsCyclingTriathleteF sPartnerTextsBugCatcher sPartnerTextsSchoolKidM sPartnerTextsRichBoy sPartnerTextsBlackBelt sPartnerTextsTuberF sPartnerTextsHexManiac sPartnerTextsPkmnBreederM sPartnerTextsRunningTriathleteF sPartnerTextsRunningTriathleteM sPartnerTextsBattleGirl sPartnerTextsCyclingTriathleteM sPartnerTextsTuberM sPartnerTextsGuitarist sPartnerTextsGentleman sPartnerTextsPokefanM sPartnerTextsExpertM sPartnerTextsExpertF sPartnerTextsDragonTamer sPartnerTextsBirdKeeper sPartnerTextsNinjaBoy sPartnerTextsParasolLady sPartnerTextsBugManiac sPartnerTextsSailor sPartnerTextsCollector sPartnerTextsPkmnRangerM sPartnerTextsPkmnRangerF sPartnerTextsAromaLady sPartnerTextsRuinManiac sPartnerTextsCoolTrainerM sPartnerTextsCoolTrainerF sPartnerTextsPokemaniac sPartnerTextsKindler sPartnerTextsCamper sPartnerTextsPicnicker sPartnerTextsPsychicM sPartnerTextsPsychicF sPartnerTextsSchoolKidF sPartnerTextsPkmnBreederF sPartnerTextsPokefanF sPartnerTextsSwimmerF sPartnerTextsSwimmingTriathleteM sPartnerTextsSwimmingTriathleteF sPartnerTextsSwimmerM sPartnerTrainerTextTables sPartnerApprenticeTextTables sStevenMons gSlateportBattleTentTrainerMons_Jolie gSlateportBattleTentTrainerMons_Malachi gSlateportBattleTentTrainerMons_Kelsie gSlateportBattleTentTrainerMons_Davon gSlateportBattleTentTrainerMons_Glenda gSlateportBattleTentTrainerMons_Helena gSlateportBattleTentTrainerMons_Rodolfo gSlateportBattleTentTrainerMons_Davion gSlateportBattleTentTrainerMons_Kendall gSlateportBattleTentTrainerMons_Colten gSlateportBattleTentTrainerMons_Irvin gSlateportBattleTentTrainerMons_Shaun gSlateportBattleTentTrainerMons_Kyler gSlateportBattleTentTrainerMons_Maggie gSlateportBattleTentTrainerMons_Stephon gSlateportBattleTentTrainerMons_Rebecca gSlateportBattleTentTrainerMons_Reggie gSlateportBattleTentTrainerMons_Janae gSlateportBattleTentTrainerMons_Caiden gSlateportBattleTentTrainerMons_Kirsten gSlateportBattleTentTrainerMons_Kurtis gSlateportBattleTentTrainerMons_Stefan gSlateportBattleTentTrainerMons_Avery gSlateportBattleTentTrainerMons_Dwane gSlateportBattleTentTrainerMons_Mckenna gSlateportBattleTentTrainerMons_Camryn gSlateportBattleTentTrainerMons_Natasha gSlateportBattleTentTrainerMons_Austyn gSlateportBattleTentTrainerMons_Donovan gSlateportBattleTentTrainerMons_Tamia gSlateportBattleTentTrainers gSlateportBattleTentMons gVerdanturfBattleTentTrainerMons_Brenna gVerdanturfBattleTentTrainerMons_Dilan gVerdanturfBattleTentTrainerMons_Eliana gVerdanturfBattleTentTrainerMons_Markus gVerdanturfBattleTentTrainerMons_Caitlyn gVerdanturfBattleTentTrainerMons_Desiree gVerdanturfBattleTentTrainerMons_Ronald gVerdanturfBattleTentTrainerMons_Ashten gVerdanturfBattleTentTrainerMons_Gerard gVerdanturfBattleTentTrainerMons_Bradly gVerdanturfBattleTentTrainerMons_Dennis gVerdanturfBattleTentTrainerMons_Prestin gVerdanturfBattleTentTrainerMons_Ernesto gVerdanturfBattleTentTrainerMons_Nala gVerdanturfBattleTentTrainerMons_Darnell gVerdanturfBattleTentTrainerMons_Ashlyn gVerdanturfBattleTentTrainerMons_Addison gVerdanturfBattleTentTrainerMons_Justine gVerdanturfBattleTentTrainerMons_Tyson gVerdanturfBattleTentTrainerMons_Laila gVerdanturfBattleTentTrainerMons_Waren gVerdanturfBattleTentTrainerMons_Tobias gVerdanturfBattleTentTrainerMons_Josiah gVerdanturfBattleTentTrainerMons_Dion gVerdanturfBattleTentTrainerMons_Kenzie gVerdanturfBattleTentTrainerMons_Lillian gVerdanturfBattleTentTrainerMons_Lesley gVerdanturfBattleTentTrainerMons_Marquis gVerdanturfBattleTentTrainerMons_Freddy gVerdanturfBattleTentTrainerMons_Cecilia gVerdanturfBattleTentTrainers gVerdanturfBattleTentMons gFallarborBattleTentTrainerMons_Amber gFallarborBattleTentTrainerMons_Javier gFallarborBattleTentTrainerMons_Natalie gFallarborBattleTentTrainerMons_Treve gFallarborBattleTentTrainerMons_Arianna gFallarborBattleTentTrainerMons_Jadyn gFallarborBattleTentTrainerMons_Gerardo gFallarborBattleTentTrainerMons_Jonn gFallarborBattleTentTrainerMons_Esteban gFallarborBattleTentTrainerMons_Jameson gFallarborBattleTentTrainerMons_Alanzo gFallarborBattleTentTrainerMons_Howard gFallarborBattleTentTrainerMons_Conrad gFallarborBattleTentTrainerMons_Makenna gFallarborBattleTentTrainerMons_Brayan gFallarborBattleTentTrainerMons_Mariana gFallarborBattleTentTrainerMons_Sheldon gFallarborBattleTentTrainerMons_Gianna gFallarborBattleTentTrainerMons_Yahir gFallarborBattleTentTrainerMons_Britney gFallarborBattleTentTrainerMons_Hecter gFallarborBattleTentTrainerMons_Tannor gFallarborBattleTentTrainerMons_Benji gFallarborBattleTentTrainerMons_Rory gFallarborBattleTentTrainerMons_Eleanor gFallarborBattleTentTrainerMons_Evelyn gFallarborBattleTentTrainerMons_Arielle gFallarborBattleTentTrainerMons_Connar gFallarborBattleTentTrainerMons_Maurice gFallarborBattleTentTrainerMons_Kianna gFallarborBattleTentTrainers gFallarborBattleTentMons sBattleTowerFuncs sWinStreakFlags sWinStreakMasks sApprenticeChallengeThreshold sBattleTowerPartySizes2 sFrontierTrainerIdRanges sFrontierTrainerIdRangesHard sUnused sBattleTowerPartySizes sRecordTrainerSpeechWon sRecordTrainerSpeechLost

/// `struct RibbonCounter`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RibbonCounter {
    pub partyIndex: u8,
    pub count: u8,
}

unsafe impl Sync for RibbonCounter {}

/// `__typeof__(sPartnerTrainerTextTables[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPartnerTrainerTextTables_0_t {
    pub facilityClass: u32,
    pub strings: *mut *mut u8,
}

unsafe impl Sync for sPartnerTrainerTextTables_0_t {}

/// `__typeof__(sStevenMons[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sStevenMons_0_t {
    pub species: u16,
    pub fixedIV: u8,
    pub level: u8,
    pub nature: u8,
    pub evs: CArray<u8, 6>,
    pub moves: CArray<u16, 4>,
}

unsafe impl Sync for sStevenMons_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RibbonCounter>() == 4);
    assert!(offset_of!(RibbonCounter, partyIndex) == 0);
    assert!(offset_of!(RibbonCounter, count) == 1);
    assert!(size_of::<sPartnerTrainerTextTables_0_t>() == 8);
    assert!(offset_of!(sPartnerTrainerTextTables_0_t, facilityClass) == 0);
    assert!(offset_of!(sPartnerTrainerTextTables_0_t, strings) == 4);
    assert!(size_of::<sStevenMons_0_t>() == 20);
    assert!(offset_of!(sStevenMons_0_t, species) == 0);
    assert!(offset_of!(sStevenMons_0_t, fixedIV) == 2);
    assert!(offset_of!(sStevenMons_0_t, level) == 3);
    assert!(offset_of!(sStevenMons_0_t, nature) == 4);
    assert!(offset_of!(sStevenMons_0_t, evs) == 5);
    assert!(offset_of!(sStevenMons_0_t, moves) == 12);
};

const STEVEN_OTID: u32 = 61226;

static gBattleFrontierHeldItems: Table<CArray<u16, 63>> =
    Table((&raw const crate::data::battle_tower::gBattleFrontierHeldItems).cast());
static gBattleFrontierMons: Table<CArray<FacilityMon, 882>> =
    Table((&raw const crate::data::battle_tower::gBattleFrontierMons).cast());
static gBattleFrontierTrainers: Table<CArray<BattleFrontierTrainer, 300>> =
    Table((&raw const crate::data::battle_tower::gBattleFrontierTrainers).cast());
static gFallarborBattleTentMons: Table<CArray<FacilityMon, 45>> =
    Table((&raw const crate::data::battle_tower::gFallarborBattleTentMons).cast());
static gFallarborBattleTentTrainers: Table<CArray<BattleFrontierTrainer, 30>> =
    Table((&raw const crate::data::battle_tower::gFallarborBattleTentTrainers).cast());
static gSlateportBattleTentMons: Table<CArray<FacilityMon, 70>> =
    Table((&raw const crate::data::battle_tower::gSlateportBattleTentMons).cast());
static gSlateportBattleTentTrainers: Table<CArray<BattleFrontierTrainer, 30>> =
    Table((&raw const crate::data::battle_tower::gSlateportBattleTentTrainers).cast());
static gTowerFemaleFacilityClasses: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_tower::gTowerFemaleFacilityClasses).cast());
static gTowerFemaleTrainerGfxIds: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::battle_tower::gTowerFemaleTrainerGfxIds).cast());
static gTowerMaleFacilityClasses: Table<CArray<u8, 30>> =
    Table((&raw const crate::data::battle_tower::gTowerMaleFacilityClasses).cast());
static gTowerMaleTrainerGfxIds: Table<CArray<u8, 30>> =
    Table((&raw const crate::data::battle_tower::gTowerMaleTrainerGfxIds).cast());
static gVerdanturfBattleTentMons: Table<CArray<FacilityMon, 45>> =
    Table((&raw const crate::data::battle_tower::gVerdanturfBattleTentMons).cast());
static gVerdanturfBattleTentTrainers: Table<CArray<BattleFrontierTrainer, 30>> =
    Table((&raw const crate::data::battle_tower::gVerdanturfBattleTentTrainers).cast());
static sApprenticeChallengeThreshold: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::battle_tower::sApprenticeChallengeThreshold).cast());
static sBattleTowerFuncs: Table<CArray<Option<unsafe fn()>, 16>> =
    Table((&raw const crate::data::battle_tower::sBattleTowerFuncs).cast());
static sBattleTowerPartySizes: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_tower::sBattleTowerPartySizes).cast());
static sBattleTowerPartySizes2: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_tower::sBattleTowerPartySizes2).cast());
static sFrontierTrainerIdRanges: Table<CArray<CArray<u16, 2>, 8>> =
    Table((&raw const crate::data::battle_tower::sFrontierTrainerIdRanges).cast());
static sFrontierTrainerIdRangesHard: Table<CArray<CArray<u16, 2>, 8>> =
    Table((&raw const crate::data::battle_tower::sFrontierTrainerIdRangesHard).cast());
static sPartnerApprenticeTextTables: Table<CArray<*mut *mut u8, 16>> =
    Table((&raw const crate::data::battle_tower::sPartnerApprenticeTextTables).cast());
static sPartnerTrainerTextTables: Table<CArray<sPartnerTrainerTextTables_0_t, 50>> =
    Table((&raw const crate::data::battle_tower::sPartnerTrainerTextTables).cast());
static sRecordTrainerSpeechLost: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::battle_tower::sRecordTrainerSpeechLost).cast());
static sRecordTrainerSpeechWon: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::battle_tower::sRecordTrainerSpeechWon).cast());
static sRubyFacilityClassToEmerald: Table<CArray<CArray<u8, 2>, 75>> =
    Table((&raw const crate::data::battle_tower::sRubyFacilityClassToEmerald).cast());
static sStevenMons: Table<CArray<sStevenMons_0_t, 3>> =
    Table((&raw const crate::data::battle_tower::sStevenMons).cast());
static sWinStreakFlags: Table<CArray<CArray<u32, 2>, 4>> =
    Table((&raw const crate::data::battle_tower::sWinStreakFlags).cast());
static sWinStreakMasks: Table<CArray<CArray<u32, 2>, 4>> =
    Table((&raw const crate::data::battle_tower::sWinStreakMasks).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gFacilityTrainers: *mut BattleFrontierTrainer = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gFacilityTrainerMons: *mut FacilityMon = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFrontierTempParty: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CallBattleTowerFunc() {
    sBattleTowerFuncs[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitTowerChallenge() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    (*gSaveBlock2Ptr).frontier.challengeStatus = CHALLENGE_STATUS_SAVING;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(FALSE);
    ResetFrontierTrainerIds();
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & sWinStreakFlags[battleMode][lvlMode] == 0 {
        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] = 0;
    }
    ValidateBattleTowerRecordChecksums();
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
    gTrainerBattleOpponent_A = 0;
}
pub(crate) unsafe fn GetTowerData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {}
        TOWER_DATA_WIN_STREAK => {
            gSpecialVar_Result = GetCurrentBattleTowerWinStreak(lvlMode as u8, battleMode as u8);
        }
        TOWER_DATA_WIN_STREAK_ACTIVE => {
            gSpecialVar_Result = ((*gSaveBlock2Ptr).frontier.winStreakActiveFlags
                & sWinStreakFlags[battleMode][lvlMode]
                != 0) as u16;
        }
        TOWER_DATA_LVL_MODE => {
            (*gSaveBlock2Ptr).frontier.towerLvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetTowerData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {}
        TOWER_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
        }
        TOWER_DATA_WIN_STREAK_ACTIVE => {
            if gSpecialVar_0x8006 != 0 {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |=
                    sWinStreakFlags[battleMode][lvlMode];
            } else {
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &=
                    sWinStreakMasks[battleMode][lvlMode];
            }
        }
        TOWER_DATA_LVL_MODE => {
            (*gSaveBlock2Ptr).frontier.towerLvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetTowerBattleWon() {
    if gTrainerBattleOpponent_A == TRAINER_EREADER {
        ClearEReaderTrainer(&raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer);
    }
    if (*gSaveBlock2Ptr).frontier.towerNumWins < MAX_STREAK {
        (*gSaveBlock2Ptr).frontier.towerNumWins += 1;
    }
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum += 1;
    SaveCurrentWinStreak();
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
}
unsafe fn ChooseSpecialBattleTowerTrainer() -> u8 {
    let mut j: i32 = 0;
    let mut validMons: i32 = 0;
    let mut trainerIds: CArray<i32, 9> = zeroed();
    let mut idsCount: i32 = 0;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    if VarGet(VAR_FRONTIER_FACILITY) != FRONTIER_FACILITY_TOWER {
        return FALSE;
    }
    let winStreak: i32 = GetCurrentBattleTowerWinStreak(lvlMode, battleMode) as i32;
    let mut i: i32 = 0;
    while i < BATTLE_TOWER_RECORD_COUNT {
        let record: *mut u32 = &raw mut (*gSaveBlock2Ptr).frontier.towerRecords[i] as *mut u32;
        let mut recordHasData: u32 = 0;
        let mut checksum: u32 = 0;
        for j in 0..58i32 {
            recordHasData |= *record.at(j);
            checksum += *record.at(j);
        }
        validMons = 0;
        j = 0;
        while j
            < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
                3
            } else {
                if 4 >= 2 { 4 } else { 2 }
            })
        {
            if (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].species != SPECIES_NONE
                && (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].level
                    <= GetFrontierEnemyMonLevel(lvlMode)
            {
                validMons += 1;
            }
            j += 1;
        }
        if validMons >= sBattleTowerPartySizes2[battleMode] as i32
            && (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak as i32 == winStreak
            && (*gSaveBlock2Ptr).frontier.towerRecords[i].lvlMode == lvlMode
            && recordHasData != 0
            && (*gSaveBlock2Ptr).frontier.towerRecords[i].checksum == checksum
        {
            trainerIds[idsCount] = i + TRAINER_RECORD_MIXING_FRIEND;
            idsCount += 1;
        }
        i += 1;
    }
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        ValidateApprenticesChecksums();
        for i in 0..APPRENTICE_COUNT {
            if (*gSaveBlock2Ptr).apprentices[i].lvlMode() != 0
                && sApprenticeChallengeThreshold[(*gSaveBlock2Ptr).apprentices[i].numQuestions]
                    as i32
                    == winStreak
                && (*gSaveBlock2Ptr).apprentices[i].lvlMode() as i32 - 1 == lvlMode as i32
            {
                trainerIds[idsCount] = i + TRAINER_RECORD_MIXING_APPRENTICE;
                idsCount += 1;
            }
        }
    }
    if idsCount != 0 {
        gTrainerBattleOpponent_A = trainerIds[rem_i32(Random() as i32, idsCount)] as u16;
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn SetNextFacilityOpponent() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    if lvlMode == FRONTIER_LVL_TENT as u32 {
        SetNextBattleTentOpponent();
    } else {
        let mut id: u16 = 0;
        let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
        let winStreak: u16 = GetCurrentFacilityWinStreak() as u16;
        let challengeNum: u32 = (winStreak as i32 / 7) as u32;
        SetFacilityPtrsGetLevel();
        if battleMode == FRONTIER_MODE_MULTIS as u32
            || battleMode == FRONTIER_MODE_LINK_MULTIS as u32
        {
            id = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
            gTrainerBattleOpponent_A = (*gSaveBlock2Ptr).frontier.trainerIds[id as i32 * 2];
            gTrainerBattleOpponent_B = (*gSaveBlock2Ptr).frontier.trainerIds[id as i32 * 2 + 1];
            SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
            SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_B, 1);
        } else if ChooseSpecialBattleTowerTrainer() != 0 {
            SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
            (*gSaveBlock2Ptr).frontier.trainerIds
                [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum] = gTrainerBattleOpponent_A;
        } else {
            let mut i: i32 = 0;
            loop {
                id = GetRandomScaledFrontierTrainerId(
                    challengeNum as u8,
                    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u8,
                );
                i = 0;
                while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
                    if (*gSaveBlock2Ptr).frontier.trainerIds[i] == id {
                        break;
                    }
                    i += 1;
                }
                if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
                    break;
                }
            }
            gTrainerBattleOpponent_A = id;
            SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
            if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 + 1
                < FRONTIER_STAGES_PER_CHALLENGE as i32
            {
                (*gSaveBlock2Ptr).frontier.trainerIds
                    [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum] = gTrainerBattleOpponent_A;
            }
        }
    }
}
pub fn GetRandomScaledFrontierTrainerId(challengeNum: u8, battleNum: u8) -> u16 {
    let mut trainerId: u16 = 0;
    if challengeNum <= 7 {
        if battleNum == 6 {
            trainerId = sFrontierTrainerIdRangesHard[challengeNum][1]
                - sFrontierTrainerIdRangesHard[challengeNum][0]
                + 1;
            trainerId = sFrontierTrainerIdRangesHard[challengeNum][0]
                + rem_i32(Random() as i32, trainerId as i32) as u16;
        } else {
            trainerId = sFrontierTrainerIdRanges[challengeNum][1]
                - sFrontierTrainerIdRanges[challengeNum][0]
                + 1;
            trainerId = sFrontierTrainerIdRanges[challengeNum][0]
                + rem_i32(Random() as i32, trainerId as i32) as u16;
        }
    } else {
        trainerId = sFrontierTrainerIdRanges[7][1] - sFrontierTrainerIdRanges[7][0] + 1;
        trainerId =
            sFrontierTrainerIdRanges[7][0] + rem_i32(Random() as i32, trainerId as i32) as u16;
    }
    trainerId
}
unsafe fn GetRandomScaledFrontierTrainerIdRange(
    challengeNum: u8,
    battleNum: u8,
    trainerIdPtr: *mut u16,
    rangePtr: *mut u8,
) {
    let mut trainerId: u16 = 0;
    let mut range: u16 = 0;
    if challengeNum <= 7 {
        if battleNum == 6 {
            range = sFrontierTrainerIdRangesHard[challengeNum][1]
                - sFrontierTrainerIdRangesHard[challengeNum][0]
                + 1;
            trainerId = sFrontierTrainerIdRangesHard[challengeNum][0];
        } else {
            range = sFrontierTrainerIdRanges[challengeNum][1]
                - sFrontierTrainerIdRanges[challengeNum][0]
                + 1;
            trainerId = sFrontierTrainerIdRanges[challengeNum][0];
        }
    } else {
        range = sFrontierTrainerIdRanges[7][1] - sFrontierTrainerIdRanges[7][0] + 1;
        trainerId = sFrontierTrainerIdRanges[7][0];
    }
    *trainerIdPtr = trainerId;
    *rangePtr = range as u8;
}
#[unsafe(no_mangle)]
pub unsafe fn SetBattleFacilityTrainerGfxId(trainerId: u16, tempVarId: u8) {
    let mut facilityClass: u8 = 0;
    let mut trainerObjectGfxId: u8 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        facilityClass = (*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass;
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        SetFrontierBrainObjEventGfx_2();
        return;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        facilityClass = (*gFacilityTrainers.at(trainerId)).facilityClass;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        facilityClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
    } else {
        facilityClass = (*(&raw const crate::data::apprentice::gApprentices)
            .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
            [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id()]
        .facilityClass;
    }
    let mut i: u32 = 0;
    while i < 30 {
        if gTowerMaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 30 {
        trainerObjectGfxId = gTowerMaleTrainerGfxIds[i];
        match tempVarId {
            1 => {
                VarSet(VAR_OBJ_GFX_ID_1, trainerObjectGfxId as u16);
                return;
            }
            15 => {
                VarSet(VAR_OBJ_GFX_ID_E, trainerObjectGfxId as u16);
                return;
            }
            _ => {
                VarSet(VAR_OBJ_GFX_ID_0, trainerObjectGfxId as u16);
                return;
            }
        }
    }
    i = 0;
    while i < 20 {
        if gTowerFemaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 20 {
        trainerObjectGfxId = gTowerFemaleTrainerGfxIds[i];
        match tempVarId {
            1 => {
                VarSet(VAR_OBJ_GFX_ID_1, trainerObjectGfxId as u16);
                return;
            }
            15 => {
                VarSet(VAR_OBJ_GFX_ID_E, trainerObjectGfxId as u16);
                return;
            }
            _ => {
                VarSet(VAR_OBJ_GFX_ID_0, trainerObjectGfxId as u16);
                return;
            }
        }
    }
    match tempVarId {
        1 => {
            VarSet(VAR_OBJ_GFX_ID_1, OBJ_EVENT_GFX_BOY_1);
        }
        15 => {
            VarSet(VAR_OBJ_GFX_ID_E, OBJ_EVENT_GFX_BOY_1);
        }
        _ => {
            VarSet(VAR_OBJ_GFX_ID_0, OBJ_EVENT_GFX_BOY_1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetEReaderTrainerGfxId() {
    SetBattleFacilityTrainerGfxId(TRAINER_EREADER, 0);
}
pub unsafe fn GetBattleFacilityTrainerGfxId(trainerId: u16) -> u8 {
    let mut facilityClass: u8 = 0;
    let mut trainerObjectGfxId: u8 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        facilityClass = (*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        facilityClass = (*gFacilityTrainers.at(trainerId)).facilityClass;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        facilityClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
    } else {
        facilityClass = (*(&raw const crate::data::apprentice::gApprentices)
            .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
            [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id()]
        .facilityClass;
    }
    let mut i: u32 = 0;
    while i < 30 {
        if gTowerMaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 30 {
        trainerObjectGfxId = gTowerMaleTrainerGfxIds[i];
        return trainerObjectGfxId;
    }
    i = 0;
    while i < 20 {
        if gTowerFemaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 20 {
        trainerObjectGfxId = gTowerFemaleTrainerGfxIds[i];
        return trainerObjectGfxId;
    } else {
        return OBJ_EVENT_GFX_BOY_1 as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PutNewBattleTowerRecord(newRecordEm: *mut EmeraldBattleTowerRecord) {
    let mut slotValues: CArray<u16, 6> = zeroed();
    let mut slotIds: CArray<u16, 6> = zeroed();
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut slotsCount: i32 = 0;
    let newRecord: *mut EmeraldBattleTowerRecord = newRecordEm;
    let mut i: i32 = 0;
    while i < BATTLE_TOWER_RECORD_COUNT {
        k = 0;
        j = 0;
        while j < TRAINER_ID_LENGTH as i32 {
            if (*gSaveBlock2Ptr).frontier.towerRecords[i].trainerId[j] != (*newRecord).trainerId[j]
            {
                break;
            }
            j += 1;
        }
        if j == TRAINER_ID_LENGTH as i32 {
            k = 0;
            while k < PLAYER_NAME_LENGTH {
                if (*gSaveBlock2Ptr).frontier.towerRecords[i].name[j] != (*newRecord).name[j] {
                    break;
                }
                if (*newRecord).name[j] == EOS {
                    k = PLAYER_NAME_LENGTH;
                    break;
                }
                k += 1;
            }
        }
        if k == PLAYER_NAME_LENGTH {
            break;
        }
        i += 1;
    }
    if i < BATTLE_TOWER_RECORD_COUNT {
        (*gSaveBlock2Ptr).frontier.towerRecords[i] = *newRecord;
        return;
    }
    i = 0;
    while i < BATTLE_TOWER_RECORD_COUNT {
        if (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak == 0 {
            break;
        }
        i += 1;
    }
    if i < BATTLE_TOWER_RECORD_COUNT {
        (*gSaveBlock2Ptr).frontier.towerRecords[i] = *newRecord;
        return;
    }
    slotValues[0] = (*gSaveBlock2Ptr).frontier.towerRecords[0].winStreak;
    slotIds[0] = 0;
    slotsCount += 1;
    for i in 1..BATTLE_TOWER_RECORD_COUNT {
        j = 0;
        while j < slotsCount {
            if (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak < slotValues[j] {
                j = 0;
                slotsCount = 1;
                slotValues[0] = (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak;
                slotIds[0] = i as u16;
                break;
            } else if (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak > slotValues[j] {
                break;
            }
            j += 1;
        }
        if j == slotsCount {
            slotValues[slotsCount] = (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak;
            slotIds[slotsCount] = i as u16;
            slotsCount += 1;
        }
    }
    i = rem_i32(Random() as i32, slotsCount);
    (*gSaveBlock2Ptr).frontier.towerRecords[slotIds[i]] = *newRecord;
}
pub unsafe fn GetFrontierTrainerFrontSpriteId(trainerId: u16) -> u8 {
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
            .cast::<CArray<u8, 0>>())[(*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass];
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        return GetFrontierBrainTrainerPicIndex();
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
            .cast::<CArray<u8, 0>>())[(*gFacilityTrainers.at(trainerId)).facilityClass];
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
                .cast::<CArray<u8, 0>>())[GetRecordedBattleRecordMixFriendClass()];
        } else {
            return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
                .cast::<CArray<u8, 0>>())[(*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .facilityClass];
        }
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
                .cast::<CArray<u8, 0>>())[(*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[GetRecordedBattleApprenticeId()]
            .facilityClass];
        } else {
            return (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex)
                .cast::<CArray<u8, 0>>())[(*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .id()]
            .facilityClass];
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetFrontierOpponentClass(trainerId: u16) -> u8 {
    let mut trainerClass: u8 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
            .cast::<CArray<u8, 0>>())[(*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass];
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        return GetFrontierBrainTrainerClass();
    } else if trainerId == TRAINER_STEVEN_PARTNER {
        trainerClass = (*(&raw const crate::data::data_tables::gTrainers)
            .cast::<CArray<Trainer, 0>>())[804]
            .trainerClass;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
            .cast::<CArray<u8, 0>>())[(*gFacilityTrainers.at(trainerId)).facilityClass];
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[GetRecordedBattleRecordMixFriendClass()];
        } else {
            trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[(*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .facilityClass];
        }
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[(*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[GetRecordedBattleApprenticeId()]
            .facilityClass];
        } else {
            trainerClass = (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[(*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .id()]
            .facilityClass];
        }
    }
    trainerClass
}
unsafe fn GetFrontierTrainerFacilityClass(trainerId: u16) -> u8 {
    let mut facilityClass: u8 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        facilityClass = (*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        facilityClass = (*gFacilityTrainers.at(trainerId)).facilityClass;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            facilityClass = GetRecordedBattleRecordMixFriendClass();
        } else {
            facilityClass = (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .facilityClass;
        }
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            facilityClass = (*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[GetRecordedBattleApprenticeId()]
            .facilityClass;
        } else {
            facilityClass = (*(&raw const crate::data::apprentice::gApprentices)
                .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .id()]
            .facilityClass;
        }
    }
    facilityClass
}
pub unsafe fn GetFrontierTrainerName(dst: *mut u8, trainerId: u16) {
    let i: i32 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        for i in 0..PLAYER_NAME_LENGTH {
            *dst.at(i) = (*gSaveBlock2Ptr).frontier.ereaderTrainer.name[i];
        }
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        CopyFrontierBrainTrainerName(dst);
        return;
    } else if trainerId == TRAINER_STEVEN_PARTNER {
        for i in 0..PLAYER_NAME_LENGTH {
            *dst.at(i) = (*(&raw const crate::data::data_tables::gTrainers)
                .cast::<CArray<Trainer, 0>>())[804]
                .trainerName[i];
        }
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        for i in 0..PLAYER_NAME_LENGTH {
            *dst.at(i) = (*gFacilityTrainers.at(trainerId)).trainerName[i];
        }
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            GetRecordedBattleRecordMixFriendName(dst);
            return;
        } else {
            let record: *mut EmeraldBattleTowerRecord = &raw mut (*gSaveBlock2Ptr)
                .frontier
                .towerRecords[trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND];
            TVShowConvertInternationalString(
                dst,
                (*record).name.as_mut_ptr(),
                (*record).language as i32,
            );
            return;
        }
    } else {
        let mut id: u8 = 0;
        let mut language: u8 = 0;
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            id = GetRecordedBattleApprenticeId();
            language = GetRecordedBattleApprenticeLanguage();
        } else {
            let apprentice: *mut Apprentice = &raw mut (*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE];
            id = (*apprentice).id();
            language = (*apprentice).language;
        }
        TVShowConvertInternationalString(
            dst,
            GetApprenticeNameInLanguage(id as u32, language as i32),
            language as i32,
        );
        return;
    }
    *dst.at(i) = EOS;
}
unsafe fn IsFrontierTrainerFemale(trainerId: u16) -> u8 {
    let mut facilityClass: u8 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_EREADER {
        facilityClass = (*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass;
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        return IsFrontierBrainFemale();
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        facilityClass = (*gFacilityTrainers.at(trainerId)).facilityClass;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        facilityClass = (*gSaveBlock2Ptr).frontier.towerRecords
            [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .facilityClass;
    } else {
        facilityClass = (*(&raw const crate::data::apprentice::gApprentices)
            .cast::<CArray<ApprenticeTrainer, 0>>())[(*gSaveBlock2Ptr).apprentices
            [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .id()]
        .facilityClass;
    }
    let mut i: u32 = 0;
    while i < 20 {
        if gTowerFemaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 20 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn FillFrontierTrainerParty(monsCount: u8) {
    ZeroEnemyPartyMons();
    FillTrainerParty(gTrainerBattleOpponent_A, 0, monsCount);
}
pub unsafe fn FillFrontierTrainersParties(monsCount: u8) {
    ZeroEnemyPartyMons();
    FillTrainerParty(gTrainerBattleOpponent_A, 0, monsCount);
    FillTrainerParty(gTrainerBattleOpponent_B, 3, monsCount);
}
unsafe fn FillTentTrainerParty(monsCount: u8) {
    ZeroEnemyPartyMons();
    FillTentTrainerParty_(gTrainerBattleOpponent_A, 0, monsCount);
}
unsafe fn FillTrainerParty(trainerId: u16, firstMonId: u8, monCount: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut chosenMonIndices: CArray<u16, 4> = zeroed();
    let mut friendship: u8 = MAX_FRIENDSHIP;
    let level: u8 = SetFacilityPtrsGetLevel();
    let mut fixedIV: u8 = 0;
    let mut monSet: *mut u16 = null_mut();
    if trainerId < FRONTIER_TRAINERS_COUNT {
        fixedIV = GetFrontierTrainerFixedIvs(trainerId);
        monSet = (*gFacilityTrainers.at(gTrainerBattleOpponent_A)).monSet;
    } else if trainerId == TRAINER_EREADER {
        for i in (firstMonId as i32)..(firstMonId as i32 + FRONTIER_PARTY_SIZE) {
            CreateBattleTowerMon(
                &raw mut gEnemyParty[i],
                &raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer.party[i - firstMonId as i32],
            );
        }
        return;
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        CreateFrontierBrainPokemon();
        return;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        j = 0;
        for i in (firstMonId as i32)..(firstMonId as i32 + monCount as i32) {
            if (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[j]
                .species
                != SPECIES_NONE
                && (*gSaveBlock2Ptr).frontier.towerRecords
                    [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                    .party[j]
                    .level
                    <= level
            {
                CreateBattleTowerMon_HandleLevel(
                    &raw mut gEnemyParty[i],
                    &raw mut (*gSaveBlock2Ptr).frontier.towerRecords
                        [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .party[j],
                    FALSE,
                );
            }
            j += 1;
        }
        return;
    } else {
        for i in (firstMonId as i32)..(firstMonId as i32 + FRONTIER_PARTY_SIZE) {
            CreateApprenticeMon(
                &raw mut gEnemyParty[i],
                &raw mut (*gSaveBlock2Ptr).apprentices
                    [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE],
                i as u8 - firstMonId,
            );
        }
        return;
    }
    let mut bfMonCount: u8 = 0;
    while *monSet.at(bfMonCount) != 0xFFFF {
        bfMonCount += 1;
    }
    i = 0;
    let otID: u32 = Random() as u32 | (Random() as u32) << 16;
    while i != monCount as i32 {
        let monId: u16 = *monSet.at(rem_i32(Random() as i32, bfMonCount as i32));
        if (level == FRONTIER_MAX_LEVEL_50 || level == 20) && monId > FRONTIER_MONS_HIGH_TIER {
            continue;
        }
        j = 0;
        while j < i + firstMonId as i32 {
            if GetMonData3(&raw mut gEnemyParty[j], MON_DATA_SPECIES, null_mut())
                == (*gFacilityTrainerMons.at(monId)).species as u32
            {
                break;
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        j = 0;
        while j < i + firstMonId as i32 {
            if GetMonData3(&raw mut gEnemyParty[j], MON_DATA_HELD_ITEM, null_mut())
                != ITEM_NONE as u32
                && GetMonData3(&raw mut gEnemyParty[j], MON_DATA_HELD_ITEM, null_mut())
                    == gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId]
                        as u32
            {
                break;
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        j = 0;
        while j < i {
            if chosenMonIndices[j] == monId {
                break;
            }
            j += 1;
        }
        if j != i {
            continue;
        }
        chosenMonIndices[i] = monId;
        CreateMonWithEVSpreadNatureOTID(
            &raw mut gEnemyParty[i + firstMonId as i32],
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            fixedIV,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otID,
        );
        friendship = MAX_FRIENDSHIP;
        for j in 0..MAX_MON_MOVES {
            SetMonMoveSlot(
                &raw mut gEnemyParty[i + firstMonId as i32],
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j as u8,
            );
            if (*gFacilityTrainerMons.at(monId)).moves[j] == MOVE_FRUSTRATION {
                friendship = 0;
            }
        }
        SetMonData(
            &raw mut gEnemyParty[i + firstMonId as i32],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[i + firstMonId as i32],
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
        i += 1;
    }
}
unsafe fn Unused_CreateApprenticeMons(trainerId: u16, firstMonId: u8) {
    let mut friendship: u8 = MAX_FRIENDSHIP;
    let mut level: u8 = 0;
    let mut fixedIV: u8 = 0;
    let apprentice: *mut Apprentice = &raw mut (*gSaveBlock2Ptr).apprentices[0];
    if (*apprentice).numQuestions < 5 {
        fixedIV = 6;
    } else {
        fixedIV = 9;
    }
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        level = FRONTIER_MAX_LEVEL_OPEN;
    } else {
        level = FRONTIER_MAX_LEVEL_50;
    }
    let mut i: i32 = 0;
    while i != FRONTIER_PARTY_SIZE {
        CreateMonWithEVSpread(
            &raw mut gEnemyParty[firstMonId as i32 + i],
            (*apprentice).party[i].species,
            level,
            fixedIV,
            8,
        );
        friendship = MAX_FRIENDSHIP;
        for j in 0..MAX_MON_MOVES {
            if (*apprentice).party[i].moves[j] == MOVE_FRUSTRATION {
                friendship = 0;
            }
        }
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i],
            MON_DATA_HELD_ITEM,
            &raw mut (*apprentice).party[i].item as *mut c_void,
        );
        i += 1;
    }
}
pub unsafe fn GetRandomFrontierMonFromSet(trainerId: u16) -> u16 {
    let level: u8 = SetFacilityPtrsGetLevel();
    let monSet: *mut u16 = (*gFacilityTrainers.at(trainerId)).monSet;
    let mut numMons: u8 = 0;
    let mut monId: u32 = *monSet.at(numMons) as u32;
    while monId != 0xFFFF {
        numMons += 1;
        monId = *monSet.at(numMons) as u32;
        if monId == 0xFFFF {
            break;
        }
    }
    loop {
        monId = *monSet.at(rem_i32(Random() as i32, numMons as i32)) as u32;
        if !((level == FRONTIER_MAX_LEVEL_50 || level == 20)
            && monId > FRONTIER_MONS_HIGH_TIER as u32)
        {
            break;
        }
    }
    monId as u16
}
unsafe fn FillFactoryTrainerParty() {
    ZeroEnemyPartyMons();
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
        FillFactoryFrontierTrainerParty(gTrainerBattleOpponent_A, 0);
    } else {
        FillFactoryTentTrainerParty(gTrainerBattleOpponent_A, 0);
    }
}
unsafe fn FillFactoryFrontierTrainerParty(trainerId: u16, firstMonId: u8) {
    let mut i: u8 = 0;
    let mut friendship: u8 = 0;
    let mut fixedIV: u8 = 0;
    if trainerId < FRONTIER_TRAINERS_COUNT {
        let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
        let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
        let challengeNum: u8 =
            ((*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][0] as i32 / 7) as u8;
        if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum < 6 {
            fixedIV = GetFactoryMonFixedIV(challengeNum, FALSE);
        } else {
            fixedIV = GetFactoryMonFixedIV(challengeNum, TRUE);
        }
    } else if trainerId == TRAINER_EREADER {
        i = firstMonId;
        while (i as i32) < firstMonId as i32 + FRONTIER_PARTY_SIZE {
            CreateBattleTowerMon(
                &raw mut gEnemyParty[i],
                &raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer.party
                    [i as i32 - firstMonId as i32],
            );
            i += 1;
        }
        return;
    } else if trainerId == TRAINER_FRONTIER_BRAIN {
        FillFactoryBrainParty();
        return;
    } else {
        fixedIV = MAX_PER_STAT_IVS;
    }
    let level: u8 = SetFacilityPtrsGetLevel();
    let otID: u32 = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let monId: u16 = gFrontierTempParty[i];
        CreateMonWithEVSpreadNatureOTID(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            fixedIV,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otID,
        );
        friendship = 0;
        for j in 0..(MAX_MON_MOVES as u8) {
            SetMonMoveAvoidReturn(
                &raw mut gEnemyParty[firstMonId as i32 + i as i32],
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
        }
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
unsafe fn FillFactoryTentTrainerParty(trainerId: u16, firstMonId: u8) {
    let mut friendship: u8 = 0;
    let level: u8 = TENT_MIN_LEVEL;
    let fixedIV: u8 = 0;
    let otID: u32 = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        let monId: u16 = gFrontierTempParty[i];
        CreateMonWithEVSpreadNatureOTID(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            fixedIV,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otID,
        );
        friendship = 0;
        for j in 0..(MAX_MON_MOVES as u8) {
            SetMonMoveAvoidReturn(
                &raw mut gEnemyParty[firstMonId as i32 + i as i32],
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
            if (*gFacilityTrainerMons.at(monId)).moves[j] == MOVE_FRUSTRATION {
                friendship = 0;
            }
        }
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[firstMonId as i32 + i as i32],
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FrontierSpeechToString(words: *mut u16) {
    ConvertEasyChatWordsToString(gStringVar4.as_mut_ptr(), words, 3, 2);
    if GetStringWidth(FONT_NORMAL, gStringVar4.as_mut_ptr(), -1) > 204 {
        let mut i: i32 = 0;
        ConvertEasyChatWordsToString(gStringVar4.as_mut_ptr(), words, 2, 3);
        while gStringVar4[{
            let t1 = i;
            i += 1;
            t1
        }] != CHAR_NEWLINE
        {}
        while gStringVar4[i] != CHAR_NEWLINE {
            i += 1;
        }
        gStringVar4[i] = CHAR_PROMPT_SCROLL;
    }
}
pub(crate) unsafe fn GetOpponentIntroSpeech() {
    let mut trainerId: u16 = 0;
    SetFacilityPtrsGetLevel();
    if gSpecialVar_0x8005 != 0 {
        trainerId = gTrainerBattleOpponent_B;
    } else {
        trainerId = gTrainerBattleOpponent_A;
    }
    if trainerId == TRAINER_EREADER {
        FrontierSpeechToString(
            (*gSaveBlock2Ptr)
                .frontier
                .ereaderTrainer
                .greeting
                .as_mut_ptr(),
        );
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechBefore.as_mut_ptr());
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        FrontierSpeechToString(
            (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .greeting
                .as_mut_ptr(),
        );
    } else {
        BufferApprenticeChallengeText(trainerId as u8 - TRAINER_RECORD_MIXING_APPRENTICE as u8);
    }
}
pub(crate) unsafe fn HandleSpecialTrainerBattleEnd() {
    RecordedBattle_SaveBattleOutcome();
    match gBattleScripting.specialTrainerBattleType {
        SPECIAL_BATTLE_TOWER
        | SPECIAL_BATTLE_DOME
        | SPECIAL_BATTLE_PALACE
        | SPECIAL_BATTLE_ARENA
        | SPECIAL_BATTLE_FACTORY
        | SPECIAL_BATTLE_PIKE_SINGLE
        | SPECIAL_BATTLE_PIKE_DOUBLE
        | SPECIAL_BATTLE_PYRAMID => {
            if (*gSaveBlock2Ptr).frontier.battlesCount < 0xFFFFFF {
                (*gSaveBlock2Ptr).frontier.battlesCount += 1;
                if (*gSaveBlock2Ptr).frontier.battlesCount % 20 == 0 {
                    UpdateGymLeaderRematch();
                }
            } else {
                (*gSaveBlock2Ptr).frontier.battlesCount = 0xFFFFFF;
            }
        }
        SPECIAL_BATTLE_SECRET_BASE => {
            for i in 0..PARTY_SIZE {
                let mut itemBefore: u16 = GetMonData2(
                    &raw mut (*gSaveBlock1Ptr).playerParty[i],
                    MON_DATA_HELD_ITEM,
                ) as u16;
                SetMonData(
                    &raw mut gPlayerParty[i],
                    MON_DATA_HELD_ITEM,
                    &raw mut itemBefore as *mut c_void,
                );
            }
        }
        SPECIAL_BATTLE_EREADER => {
            CopyEReaderTrainerFarewellMessage();
        }
        _ => {}
    }
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
pub(crate) unsafe fn Task_StartBattleAfterTransition(taskId: u8) {
    if IsBattleTransitionDone() == TRUE {
        gMain.savedCallback = Some(HandleSpecialTrainerBattleEnd);
        SetMainCallback2(Some(CB2_InitBattle));
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DoSpecialTrainerBattle() {
    gBattleScripting.specialTrainerBattleType = gSpecialVar_0x8004 as u8;
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {
            gBattleTypeFlags = 264;
            match VarGet(VAR_FRONTIER_BATTLE_MODE) {
                0 => {
                    FillFrontierTrainerParty(FRONTIER_PARTY_SIZE as u8);
                }
                FRONTIER_MODE_DOUBLES => {
                    FillFrontierTrainerParty(FRONTIER_DOUBLES_PARTY_SIZE);
                    gBattleTypeFlags |= BATTLE_TYPE_DOUBLE;
                }
                FRONTIER_MODE_MULTIS => {
                    FillFrontierTrainersParties(FRONTIER_MULTI_PARTY_SIZE as u8);
                    gPartnerTrainerId = (*gSaveBlock2Ptr).frontier.trainerIds[17];
                    FillPartnerParty(gPartnerTrainerId);
                    gBattleTypeFlags |= 0x408041;
                }
                FRONTIER_MODE_LINK_MULTIS => {
                    gBattleTypeFlags |= 0x800043;
                    FillFrontierTrainersParties(FRONTIER_MULTI_PARTY_SIZE as u8);
                }
                _ => {}
            }
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_TOWER));
        }
        1 => {
            for i in 0..PARTY_SIZE {
                let mut itemBefore: u16 =
                    GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
                SetMonData(
                    &raw mut (*gSaveBlock1Ptr).playerParty[i],
                    MON_DATA_HELD_ITEM,
                    &raw mut itemBefore as *mut c_void,
                );
            }
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(
                B_TRANSITION_GROUP_SECRET_BASE,
            ));
        }
        2 => {
            ZeroEnemyPartyMons();
            for i in 0..3i32 {
                CreateBattleTowerMon(
                    &raw mut gEnemyParty[i],
                    &raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer.party[i],
                );
            }
            gBattleTypeFlags = 2056;
            gTrainerBattleOpponent_A = 0;
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_E_READER));
        }
        3 => {
            gBattleTypeFlags = 0x10008;
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                gBattleTypeFlags |= BATTLE_TYPE_DOUBLE;
            }
            if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
                FillFrontierTrainerParty(DOME_BATTLE_PARTY_SIZE as u8);
            }
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            CreateTask_PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_DOME));
        }
        4 => {
            gBattleTypeFlags = 0x20008;
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                gBattleTypeFlags |= BATTLE_TYPE_DOUBLE;
            }
            if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
                FillFrontierTrainerParty(FRONTIER_PARTY_SIZE as u8);
            } else {
                FillTentTrainerParty(FRONTIER_PARTY_SIZE as u8);
            }
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_PALACE));
        }
        5 => {
            gBattleTypeFlags = 0x40008;
            if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
                FillFrontierTrainerParty(FRONTIER_PARTY_SIZE as u8);
            } else {
                FillTentTrainerParty(FRONTIER_PARTY_SIZE as u8);
            }
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_ARENA));
        }
        6 => {
            gBattleTypeFlags = 0x80008;
            if VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_DOUBLES {
                gBattleTypeFlags |= BATTLE_TYPE_DOUBLE;
            }
            FillFactoryTrainerParty();
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_FACTORY));
        }
        7 => {
            gBattleTypeFlags = 264;
            FillFrontierTrainerParty(FRONTIER_PARTY_SIZE as u8);
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_PIKE));
        }
        10 => {
            gBattleTypeFlags = 0x200008;
            FillFrontierTrainerParty(FRONTIER_PARTY_SIZE as u8);
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_PYRAMID));
        }
        9 => {
            gBattleTypeFlags = 33033;
            FillFrontierTrainersParties(1);
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(GetSpecialBattleTransition(B_TRANSITION_GROUP_B_PIKE));
        }
        SPECIAL_BATTLE_STEVEN => {
            gBattleTypeFlags = 0x408049;
            FillPartnerParty(TRAINER_STEVEN_PARTNER);
            gApproachingTrainerId = 0;
            BattleSetup_ConfigureTrainerBattle(
                (*crate::asmdata::MossdeepCity_SpaceCenter_2F_EventScript_MaxieTrainer
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
                .at(1),
            );
            gApproachingTrainerId = 1;
            BattleSetup_ConfigureTrainerBattle(
                (*crate::asmdata::MossdeepCity_SpaceCenter_2F_EventScript_TabithaTrainer
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
                .at(1),
            );
            gPartnerTrainerId = TRAINER_STEVEN_PARTNER;
            CreateTask(Some(Task_StartBattleAfterTransition), 1);
            PlayMapChosenOrBattleBGM(0);
            BattleTransition_StartOnField(B_TRANSITION_MAGMA);
        }
        _ => {}
    }
}
unsafe fn SaveCurrentWinStreak() {
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let winStreak: u16 = GetCurrentBattleTowerWinStreak(lvlMode, battleMode);
    if (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] < winStreak {
        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] = winStreak;
    }
}
unsafe fn SaveBattleTowerRecord() {
    let mut lvlMode: u8 = 0;
    let mut class: u8 = 0;
    let playerRecord: *mut EmeraldBattleTowerRecord =
        &raw mut (*gSaveBlock2Ptr).frontier.towerPlayer;
    ClearBattleTowerRecord(playerRecord);
    lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    if (*gSaveBlock2Ptr).playerGender != MALE {
        class = gTowerFemaleFacilityClasses[((*gSaveBlock2Ptr).playerTrainerId[0] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[1] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[2] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[3] as u32)
            % 20];
    } else {
        class = gTowerMaleFacilityClasses[((*gSaveBlock2Ptr).playerTrainerId[0] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[1] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[2] as u32
            + (*gSaveBlock2Ptr).playerTrainerId[3] as u32)
            % 30];
    }
    (*playerRecord).lvlMode = lvlMode;
    (*playerRecord).facilityClass = class;
    CopyTrainerId(
        (*playerRecord).trainerId.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr(),
    );
    StringCopy_PlayerName(
        (*playerRecord).name.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*playerRecord).winStreak = GetCurrentBattleTowerWinStreak(lvlMode, battleMode);
    for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
        (*playerRecord).greeting[i] = (*gSaveBlock1Ptr).easyChatBattleStart[i];
        (*playerRecord).speechWon[i] = (*gSaveBlock1Ptr).easyChatBattleWon[i];
        (*playerRecord).speechLost[i] = (*gSaveBlock1Ptr).easyChatBattleLost[i];
    }
    let mut i: i32 = 0;
    while i
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        if (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] != 0 {
            ConvertPokemonToBattleTowerPokemon(
                &raw mut gPlayerParty[(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
                &raw mut (*playerRecord).party[i],
            );
        }
        i += 1;
    }
    (*playerRecord).language = gGameLanguage;
    CalcEmeraldBattleTowerChecksum(&raw mut (*gSaveBlock2Ptr).frontier.towerPlayer);
    SaveCurrentWinStreak();
}
pub(crate) unsafe fn SaveTowerChallenge() {
    let lvlMode: u16 = (*gSaveBlock2Ptr).frontier.lvlMode() as u16;
    let battleMode: u16 = VarGet(VAR_FRONTIER_BATTLE_MODE);
    let challengeNum: i32 =
        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as i32 / 7;
    if gSpecialVar_0x8005 == 0
        && (challengeNum > 1 || (*gSaveBlock2Ptr).frontier.curChallengeBattleNum != 0)
    {
        SaveBattleTowerRecord();
    }
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveGameFrontier();
}
pub(crate) fn BattleTowerNop1() {}
pub(crate) fn BattleTowerNop2() {}
unsafe fn GetApprenticeMultiPartnerParty(trainerId: u16) {
    let mut validSpecies: CArray<u32, 3> = zeroed();
    let species1: u16 = GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut()) as u16;
    let species2: u16 = GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut()) as u16;
    let mut count: i32 = 0;
    for i in 0..MULTI_PARTY_SIZE {
        let apprenticeSpecies: u16 = (*gSaveBlock2Ptr).apprentices
            [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
            .party[i]
            .species;
        if apprenticeSpecies != species1 && apprenticeSpecies != species2 {
            validSpecies[count] = i as u32;
            count += 1;
        }
    }
    gFrontierTempParty[0] = validSpecies[rem_i32(Random() as i32, count)] as u16;
    loop {
        gFrontierTempParty[1] = validSpecies[rem_i32(Random() as i32, count)] as u16;
        if gFrontierTempParty[0] != gFrontierTempParty[1] {
            break;
        }
    }
}
unsafe fn GetRecordMixFriendMultiPartnerParty(trainerId: u16) {
    let mut validSpecies: CArray<u32, 3> = zeroed();
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let species1: u16 = GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut()) as u16;
    let species2: u16 = GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut()) as u16;
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    while i
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        if (*gSaveBlock2Ptr).frontier.towerRecords[trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
            .party[i]
            .species
            != species1
            && (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[i]
                .species
                != species2
            && (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[i]
                .level
                <= GetFrontierEnemyMonLevel(lvlMode as u8)
            && (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[i]
                .species
                != SPECIES_NONE
        {
            validSpecies[count] = i as u32;
            count += 1;
        }
        i += 1;
    }
    gFrontierTempParty[2] = validSpecies[rem_i32(Random() as i32, count)] as u16;
    loop {
        gFrontierTempParty[3] = validSpecies[rem_i32(Random() as i32, count)] as u16;
        if gFrontierTempParty[2] != gFrontierTempParty[3] {
            break;
        }
    }
}
pub(crate) unsafe fn LoadMultiPartnerCandidatesData() {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut spArray: CArray<u32, 5> = zeroed();
    let mut trainerId: u16 = 0;
    let mut monId: u16 = 0;
    let mut lvlMode: u32 = 0;
    let objEventTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    lvlMode = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    let challengeNum: i32 =
        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as i32 / 7;
    let species1: u32 = GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut());
    let species2: u32 = GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut());
    let level: u32 = SetFacilityPtrsGetLevel() as u32;
    let mut j: i32 = 0;
    loop {
        loop {
            trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8, 0);
            i = 0;
            while i < j {
                if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                    break;
                }
                if (*gFacilityTrainers.at((*gSaveBlock2Ptr).frontier.trainerIds[i])).facilityClass
                    == (*gFacilityTrainers.at(trainerId)).facilityClass
                {
                    break;
                }
                i += 1;
            }
            if i == j {
                break;
            }
        }
        (*gSaveBlock2Ptr).frontier.trainerIds[j] = trainerId;
        j += 1;
        if j >= 6 {
            break;
        }
    }
    let mut r10: i32 = 8;
    for i in 0..6i32 {
        trainerId = (*gSaveBlock2Ptr).frontier.trainerIds[i];
        (*objEventTemplates.at(i + 1)).graphicsId = GetBattleFacilityTrainerGfxId(trainerId);
        for j in 0..2i32 {
            loop {
                monId = GetRandomFrontierMonFromSet(trainerId);
                if j % 2 != 0
                    && (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.trainerIds[r10 - 1]))
                        .itemTableId
                        == (*gFacilityTrainerMons.at(monId)).itemTableId
                {
                    continue;
                }
                k = 8;
                while k < r10 {
                    if (*gFacilityTrainerMons.at((*gSaveBlock2Ptr).frontier.trainerIds[k])).species
                        == (*gFacilityTrainerMons.at(monId)).species
                    {
                        break;
                    }
                    if species1 == (*gFacilityTrainerMons.at(monId)).species as u32 {
                        break;
                    }
                    if species2 == (*gFacilityTrainerMons.at(monId)).species as u32 {
                        break;
                    }
                    k += 1;
                }
                if k == r10 {
                    break;
                }
            }
            (*gSaveBlock2Ptr).frontier.trainerIds[r10] = monId;
            r10 += 1;
        }
    }
    r10 = 0;
    ValidateApprenticesChecksums();
    i = 0;
    while i < APPRENTICE_COUNT {
        if (*gSaveBlock2Ptr).apprentices[i].lvlMode() != 0
            && sApprenticeChallengeThreshold[(*gSaveBlock2Ptr).apprentices[i].numQuestions] as i32
                / 7
                <= challengeNum
            && (*gSaveBlock2Ptr).apprentices[i].lvlMode() as u32 - 1 == lvlMode
        {
            k = 0;
            for j in 0..MULTI_PARTY_SIZE {
                if species1 != (*gSaveBlock2Ptr).apprentices[i].party[j].species as u32
                    && species2 != (*gSaveBlock2Ptr).apprentices[i].party[j].species as u32
                {
                    k += 1;
                }
            }
            if k > 2 {
                spArray[r10] = i as u32 + TRAINER_RECORD_MIXING_APPRENTICE as u32;
                r10 += 1;
            }
        }
        i += 1;
    }
    if r10 != 0 {
        (*gSaveBlock2Ptr).frontier.trainerIds[6] = spArray[rem_i32(Random() as i32, r10)] as u16;
        (*objEventTemplates.at(7)).graphicsId =
            GetBattleFacilityTrainerGfxId((*gSaveBlock2Ptr).frontier.trainerIds[6]);
        FlagClear(FLAG_HIDE_BATTLE_TOWER_MULTI_BATTLE_PARTNER_ALT_1);
        GetApprenticeMultiPartnerParty((*gSaveBlock2Ptr).frontier.trainerIds[6]);
    }
    r10 = 0;
    for i in 0..BATTLE_TOWER_RECORD_COUNT {
        let record: *mut u32 = &raw mut (*gSaveBlock2Ptr).frontier.towerRecords[i] as *mut u32;
        let mut recordHasData: u32 = 0;
        let mut checksum: u32 = 0;
        j = 0;
        while j < 58 {
            recordHasData |= *record.at(j);
            checksum += *record.at(j);
            j += 1;
        }
        if (*gSaveBlock2Ptr).frontier.towerRecords[i].winStreak as i32 / 7 <= challengeNum
            && (*gSaveBlock2Ptr).frontier.towerRecords[i].lvlMode as u32 == lvlMode
            && recordHasData != 0
            && (*gSaveBlock2Ptr).frontier.towerRecords[i].checksum == checksum
        {
            k = 0;
            j = 0;
            while j
                < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
                    3
                } else {
                    if 4 >= 2 { 4 } else { 2 }
                })
            {
                if species1 != (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].species as u32
                    && species2
                        != (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].species as u32
                    && (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].level
                        <= GetFrontierEnemyMonLevel(lvlMode as u8)
                    && (*gSaveBlock2Ptr).frontier.towerRecords[i].party[j].species != SPECIES_NONE
                {
                    k += 1;
                }
                j += 1;
            }
            if k > 1 {
                spArray[r10] = i as u32 + TRAINER_RECORD_MIXING_FRIEND as u32;
                r10 += 1;
            }
        }
    }
    if r10 != 0 {
        (*gSaveBlock2Ptr).frontier.trainerIds[7] = spArray[rem_i32(Random() as i32, r10)] as u16;
        (*objEventTemplates.at(8)).graphicsId =
            GetBattleFacilityTrainerGfxId((*gSaveBlock2Ptr).frontier.trainerIds[7]);
        FlagClear(FLAG_HIDE_BATTLE_TOWER_MULTI_BATTLE_PARTNER_ALT_2);
        GetRecordMixFriendMultiPartnerParty((*gSaveBlock2Ptr).frontier.trainerIds[7]);
    }
}
unsafe fn GetPotentialPartnerMoveAndSpecies(trainerId: u16, monId: u16) {
    let mut r#move: u16 = MOVE_NONE;
    let mut species: u16 = 0;
    SetFacilityPtrsGetLevel();
    if trainerId != TRAINER_EREADER {
        if trainerId < FRONTIER_TRAINERS_COUNT {
            r#move = (*gFacilityTrainerMons.at(monId)).moves[0];
            species = (*gFacilityTrainerMons.at(monId)).species;
        } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
            r#move = (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[gFrontierTempParty[gSpecialVar_0x8005 as i32 + 1]]
                .moves[0];
            species = (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .party[gFrontierTempParty[gSpecialVar_0x8005 as i32 + 1]]
                .species;
        } else {
            r#move = (*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .party[gFrontierTempParty[gSpecialVar_0x8005 as i32 - 1]]
                .moves[0];
            species = (*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .party[gFrontierTempParty[gSpecialVar_0x8005 as i32 - 1]]
                .species;
            let mut i: i32 = 0;
            while i < PLAYER_NAME_LENGTH {
                gStringVar3[i] = (*gSaveBlock2Ptr).apprentices
                    [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                    .playerName[i];
                i += 1;
            }
            gStringVar3[i] = EOS;
            ConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*gSaveBlock2Ptr).apprentices[trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                    .language,
            );
        }
    }
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [r#move]
            .as_ptr()
            .cast_mut(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [species]
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn ShowPartnerCandidateMessage() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut monId: i32 = 0;
    let level: i32 = SetFacilityPtrsGetLevel() as i32;
    let winStreak: u16 = GetCurrentFacilityWinStreak() as u16;
    let challengeNum: i32 = winStreak as i32 / 7;
    let k: i32 = gSpecialVar_LastTalked as i32 - 2;
    let trainerId: i32 = (*gSaveBlock2Ptr).frontier.trainerIds[k] as i32;
    let mut partnerId: i32 = 0;
    while partnerId < 50 {
        if sPartnerTrainerTextTables[partnerId].facilityClass
            == GetFrontierTrainerFacilityClass(trainerId as u16) as u32
        {
            break;
        }
        partnerId += 1;
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        PARTNER_MSGID_INTRO => {
            if trainerId == TRAINER_EREADER as i32 {
                return;
            }
            if trainerId < FRONTIER_TRAINERS_COUNT as i32 {
                GetFrontierTrainerName(gStringVar1.as_mut_ptr(), trainerId as u16);
            } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE {
                GetFrontierTrainerName(gStringVar1.as_mut_ptr(), trainerId as u16);
            } else {
                let mut i: i32 = 0;
                while i < PLAYER_NAME_LENGTH {
                    gStringVar1[i] = (*gSaveBlock2Ptr).apprentices
                        [trainerId - TRAINER_RECORD_MIXING_APPRENTICE]
                        .playerName[i];
                    i += 1;
                }
                gStringVar1[i] = EOS;
                ConvertInternationalString(
                    gStringVar1.as_mut_ptr(),
                    (*gSaveBlock2Ptr).apprentices[trainerId - TRAINER_RECORD_MIXING_APPRENTICE]
                        .language,
                );
                ConvertIntToDecimalStringN(
                    gStringVar2.as_mut_ptr(),
                    (*gSaveBlock2Ptr).apprentices[trainerId - TRAINER_RECORD_MIXING_APPRENTICE]
                        .number as i32,
                    STR_CONV_MODE_LEFT_ALIGN,
                    3,
                );
                GetFrontierTrainerName(gStringVar3.as_mut_ptr(), trainerId as u16);
            }
        }
        PARTNER_MSGID_MON1 => {
            monId = (*gSaveBlock2Ptr).frontier.trainerIds[8 + k * 2] as i32;
            GetPotentialPartnerMoveAndSpecies(trainerId as u16, monId as u16);
        }
        PARTNER_MSGID_MON2_ASK => {
            monId = (*gSaveBlock2Ptr).frontier.trainerIds[9 + k * 2] as i32;
            GetPotentialPartnerMoveAndSpecies(trainerId as u16, monId as u16);
        }
        PARTNER_MSGID_ACCEPT => {
            gPartnerTrainerId = trainerId as u16;
            if trainerId < FRONTIER_TRAINERS_COUNT as i32 {
                (*gSaveBlock2Ptr).frontier.trainerIds[18] =
                    (*gSaveBlock2Ptr).frontier.trainerIds[8 + k * 2];
                (*gSaveBlock2Ptr).frontier.trainerIds[19] =
                    (*gSaveBlock2Ptr).frontier.trainerIds[9 + k * 2];
            } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE {
                (*gSaveBlock2Ptr).frontier.trainerIds[18] = gFrontierTempParty[2];
                (*gSaveBlock2Ptr).frontier.trainerIds[19] = gFrontierTempParty[3];
            } else {
                (*gSaveBlock2Ptr).frontier.trainerIds[18] = gFrontierTempParty[0];
                (*gSaveBlock2Ptr).frontier.trainerIds[19] = gFrontierTempParty[1];
            }
            for k in 0..14i32 {
                loop {
                    i = GetRandomScaledFrontierTrainerId(challengeNum as u8, (k / 2) as u8) as i32;
                    if gPartnerTrainerId as i32 == i {
                        continue;
                    }
                    j = 0;
                    while j < k {
                        if (*gSaveBlock2Ptr).frontier.trainerIds[j] as i32 == i {
                            break;
                        }
                        j += 1;
                    }
                    if j == k {
                        break;
                    }
                }
                (*gSaveBlock2Ptr).frontier.trainerIds[k] = i as u16;
            }
            (*gSaveBlock2Ptr).frontier.trainerIds[17] = trainerId as u16;
        }
        PARTNER_MSGID_REJECT => {}
        _ => {}
    }
    if trainerId == TRAINER_EREADER as i32 {
        return;
    }
    if trainerId < FRONTIER_TRAINERS_COUNT as i32 {
        ShowFieldMessage(
            *sPartnerTrainerTextTables[partnerId].strings.at(
                *(&raw const crate::ffi::gSpecialVar_0x8005)
                    .cast::<u16>()
                    .cast_mut(),
            ),
        );
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE {
        ShowFieldMessage(
            *sPartnerTrainerTextTables[partnerId].strings.at(
                *(&raw const crate::ffi::gSpecialVar_0x8005)
                    .cast::<u16>()
                    .cast_mut(),
            ),
        );
    } else {
        let apprenticeId: u8 =
            (*gSaveBlock2Ptr).apprentices[trainerId - TRAINER_RECORD_MIXING_APPRENTICE].id();
        ShowFieldMessage(
            *sPartnerApprenticeTextTables[apprenticeId].at(
                *(&raw const crate::ffi::gSpecialVar_0x8005)
                    .cast::<u16>()
                    .cast_mut(),
            ),
        );
    }
}
pub(crate) unsafe fn LoadLinkMultiOpponentsData() {
    let mut challengeNum: i32 = 0;
    let mut j: i32 = 0;
    let mut trainerId: i32 = 0;
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let battleMode: u32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u32;
    let battleNum: u32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u32;
    GetMultiplayerId();
    match *(&raw const crate::ffi::gSpecialVar_Result)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {
            if battleMode == FRONTIER_MODE_LINK_MULTIS as u32 {
                challengeNum =
                    (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as i32 / 7;
                if IsLinkTaskFinished() != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        &raw mut challengeNum as *mut c_void,
                        4,
                    );
                    gSpecialVar_Result = 1;
                }
            } else {
                gSpecialVar_Result = 6;
            }
        }
        1 => {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                if gBlockRecvBuffer[0][0] > gBlockRecvBuffer[1][0] {
                    challengeNum = gBlockRecvBuffer[0][0] as i32;
                } else {
                    challengeNum = gBlockRecvBuffer[1][0] as i32;
                }
                for i in 0..14i32 {
                    loop {
                        trainerId =
                            GetRandomScaledFrontierTrainerId(challengeNum as u8, (i / 2) as u8)
                                as i32;
                        j = 0;
                        while j < i {
                            if (*gSaveBlock2Ptr).frontier.trainerIds[j] as i32 == trainerId {
                                break;
                            }
                            j += 1;
                        }
                        if i == j {
                            break;
                        }
                    }
                    if i == j {
                        (*gSaveBlock2Ptr).frontier.trainerIds[i] = trainerId as u16;
                    }
                }
                gSpecialVar_Result = 2;
            }
        }
        2 => {
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    &raw mut (*gSaveBlock2Ptr).frontier.trainerIds as *mut c_void,
                    40,
                );
                gSpecialVar_Result = 3;
            }
        }
        3 => {
            if GetBlockReceivedStatus() as i32 & 3 == 3 {
                ResetBlockReceivedFlags();
                memcpy(
                    &raw mut (*gSaveBlock2Ptr).frontier.trainerIds as *mut u8,
                    gBlockRecvBuffer.as_mut_ptr() as *mut u8,
                    40,
                );
                gTrainerBattleOpponent_A = (*gSaveBlock2Ptr).frontier.trainerIds[battleNum * 2];
                gTrainerBattleOpponent_B = (*gSaveBlock2Ptr).frontier.trainerIds[battleNum * 2 + 1];
                SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
                SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_B, 1);
                if gReceivedRemoteLinkPlayers != 0 && gWirelessCommType == 0 {
                    gSpecialVar_Result = 4;
                } else {
                    gSpecialVar_Result = 6;
                }
            }
        }
        4 => {
            SetCloseLinkCallback();
            gSpecialVar_Result = 5;
        }
        5 => {
            if gReceivedRemoteLinkPlayers == 0 {
                gSpecialVar_Result = 6;
            }
        }
        6 => {}
        _ => {}
    }
}
pub(crate) unsafe fn TowerTryCloseLink() {
    if gWirelessCommType != 0 {
        SetCloseLinkCallback();
    }
}
pub(crate) unsafe fn SetMultiPartnerGfx() {
    SetBattleFacilityTrainerGfxId((*gSaveBlock2Ptr).frontier.trainerIds[17], 0xF);
}
pub(crate) unsafe fn SetTowerInterviewData() {
    let mut text: CArray<u8, 32> = zeroed();
    if VarGet(VAR_FRONTIER_BATTLE_MODE) != FRONTIER_MODE_SINGLES as u16 {
        return;
    }
    GetFrontierTrainerName(text.as_mut_ptr(), gTrainerBattleOpponent_A);
    StripExtCtrlCodes(text.as_mut_ptr());
    StringCopy(
        (*gSaveBlock2Ptr)
            .frontier
            .towerInterview
            .opponentName
            .as_mut_ptr(),
        text.as_mut_ptr(),
    );
    GetBattleTowerTrainerLanguage(
        &raw mut (*gSaveBlock2Ptr).frontier.towerInterview.opponentLanguage,
        gTrainerBattleOpponent_A,
    );
    (*gSaveBlock2Ptr).frontier.towerInterview.opponentSpecies = GetMonData3(
        &raw mut gEnemyParty[gBattlerPartyIndexes[1]],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    (*gSaveBlock2Ptr).frontier.towerInterview.playerSpecies = GetMonData3(
        &raw mut gPlayerParty[gBattlerPartyIndexes[0]],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    for i in 0..11i32 {
        (*gSaveBlock2Ptr)
            .frontier
            .towerInterview
            .opponentMonNickname[i] = gBattleMons[0].nickname[i];
    }
    (*gSaveBlock2Ptr).frontier.towerBattleOutcome = gBattleOutcome;
}
unsafe fn ValidateBattleTowerRecordChecksums() {
    let mut record: *mut u32 = &raw mut (*gSaveBlock2Ptr).frontier.towerPlayer as *mut u32;
    let mut checksum: u32 = 0;
    let mut j: i32 = 0;
    while j < 58 {
        checksum += *record.at(j);
        j += 1;
    }
    if (*gSaveBlock2Ptr).frontier.towerPlayer.checksum != checksum {
        ClearBattleTowerRecord(&raw mut (*gSaveBlock2Ptr).frontier.towerPlayer);
    }
    for i in 0..BATTLE_TOWER_RECORD_COUNT {
        record = &raw mut (*gSaveBlock2Ptr).frontier.towerRecords[i] as *mut u32;
        checksum = 0;
        for j in 0..58i32 {
            checksum += *record.at(j);
        }
        if (*gSaveBlock2Ptr).frontier.towerRecords[i].checksum != checksum {
            ClearBattleTowerRecord(&raw mut (*gSaveBlock2Ptr).frontier.towerRecords[i]);
        }
    }
}
pub unsafe fn CalcEmeraldBattleTowerChecksum(record: *mut EmeraldBattleTowerRecord) {
    (*record).checksum = 0;
    for i in 0..58u32 {
        (*record).checksum += *(record as *mut u32).at(i);
    }
}
pub unsafe fn CalcRubyBattleTowerChecksum(record: *mut RSBattleTowerRecord) {
    (*record).checksum = 0;
    for i in 0..40u32 {
        (*record).checksum += *(record as *mut u32).at(i);
    }
}
unsafe fn ClearBattleTowerRecord(record: *mut EmeraldBattleTowerRecord) {
    for i in 0..59u32 {
        *(record as *mut u32).at(i) = 0;
    }
}
pub unsafe fn GetCurrentBattleTowerWinStreak(lvlMode: u8, battleMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetMonCountForBattleMode(battleMode: u8) -> u8 {
    let mut partySizes: CArray<u8, 4> = zeroed();
    memcpy(
        partySizes.as_mut_ptr(),
        sBattleTowerPartySizes.as_ptr().cast_mut(),
        4,
    );
    if battleMode < 4 {
        return partySizes[battleMode];
    } else {
        return FRONTIER_PARTY_SIZE as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn AwardBattleTowerRibbons() {
    let mut partyIndex: u32 = 0;
    let mut ribbons: CArray<RibbonCounter, 3> = zeroed();
    let mut ribbonType: u8 = 0;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let monCount: u8 = GetMonCountForBattleMode(battleMode);
    if lvlMode != FRONTIER_LVL_50 {
        ribbonType = MON_DATA_VICTORY_RIBBON as u8;
    } else {
        ribbonType = MON_DATA_WINNING_RIBBON as u8;
    }
    gSpecialVar_Result = FALSE as u16;
    if GetCurrentBattleTowerWinStreak(lvlMode, battleMode) > 55 {
        for i in 0..(monCount as i32) {
            partyIndex = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as u32 - 1;
            ribbons[i].partyIndex = partyIndex as u8;
            ribbons[i].count = 0;
            if GetMonData2(
                &raw mut (*gSaveBlock1Ptr).playerParty[partyIndex],
                ribbonType as i32,
            ) == 0
            {
                gSpecialVar_Result = TRUE as u16;
                SetMonData(
                    &raw mut (*gSaveBlock1Ptr).playerParty[partyIndex],
                    ribbonType as i32,
                    &raw mut gSpecialVar_Result as *mut c_void,
                );
                ribbons[i].count =
                    GetRibbonCount(&raw mut (*gSaveBlock1Ptr).playerParty[partyIndex]);
            }
        }
    }
    if gSpecialVar_Result != 0 {
        IncrementGameStat(GAME_STAT_RECEIVED_RIBBONS);
        for i in 1..(monCount as i32) {
            if ribbons[i].count > ribbons[0].count {
                let prevBest: RibbonCounter = ribbons[0];
                ribbons[0] = ribbons[i];
                ribbons[i] = prevBest;
            }
        }
        if ribbons[0].count > NUM_CUTIES_RIBBONS {
            TryPutSpotTheCutiesOnAir(
                &raw mut (*gSaveBlock1Ptr).playerParty[ribbons[0].partyIndex],
                ribbonType,
            );
        }
    }
}
unsafe fn FillEReaderTrainerWithPlayerData() {
    let ereaderTrainer: *mut BattleTowerEReaderTrainer =
        &raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer;
    if (*gSaveBlock2Ptr).playerGender != MALE {
        (*ereaderTrainer).facilityClass =
            gTowerFemaleFacilityClasses[((*gSaveBlock2Ptr).playerTrainerId[0] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[1] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[2] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[3] as u32)
                % 20];
    } else {
        (*ereaderTrainer).facilityClass =
            gTowerMaleFacilityClasses[((*gSaveBlock2Ptr).playerTrainerId[0] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[1] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[2] as u32
                + (*gSaveBlock2Ptr).playerTrainerId[3] as u32)
                % 30];
    }
    CopyTrainerId(
        (*ereaderTrainer).trainerId.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr(),
    );
    StringCopy_PlayerName(
        (*ereaderTrainer).name.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*ereaderTrainer).winStreak = 1;
    let mut j: i32 = 7;
    for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
        (*ereaderTrainer).greeting[i] = (*gSaveBlock1Ptr).easyChatBattleStart[i];
        (*ereaderTrainer).farewellPlayerLost[i] = j as u16;
        (*ereaderTrainer).farewellPlayerWon[i] = j as u16 + 6;
        j += 1;
    }
    for i in 0..3i32 {
        ConvertPokemonToBattleTowerPokemon(
            &raw mut gPlayerParty[i],
            &raw mut (*ereaderTrainer).party[i],
        );
    }
    SetEReaderTrainerChecksum(ereaderTrainer);
}
pub unsafe fn GetEreaderTrainerFrontSpriteId() -> u8 {
    (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex).cast::<CArray<u8, 0>>())
        [(*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass]
}
pub unsafe fn GetEreaderTrainerClassId() -> u8 {
    (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass).cast::<CArray<u8, 0>>())
        [(*gSaveBlock2Ptr).frontier.ereaderTrainer.facilityClass]
}
pub unsafe fn GetEreaderTrainerName(dst: *mut u8) {
    let mut i: i32 = 0;
    while i < 5 {
        *dst.at(i) = (*gSaveBlock2Ptr).frontier.ereaderTrainer.name[i];
        i += 1;
    }
    *dst.at(i) = EOS;
}
#[unsafe(no_mangle)]
pub unsafe fn ValidateEReaderTrainer() {
    let mut ereaderTrainer: *mut BattleTowerEReaderTrainer = null_mut();
    gSpecialVar_Result = FALSE as u16;
    ereaderTrainer = &raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer;
    let mut checksum: u32 = 0;
    for i in 0..46u32 {
        checksum |= *(ereaderTrainer as *mut u32).at(i);
    }
    if checksum == 0 {
        gSpecialVar_Result = TRUE as u16;
        return;
    }
    checksum = 0;
    for i in 0..46u32 {
        checksum += *(ereaderTrainer as *mut u32).at(i);
    }
    if (*gSaveBlock2Ptr).frontier.ereaderTrainer.checksum != checksum {
        ClearEReaderTrainer(&raw mut (*gSaveBlock2Ptr).frontier.ereaderTrainer);
        gSpecialVar_Result = TRUE as u16;
    }
}
unsafe fn SetEReaderTrainerChecksum(ereaderTrainer: *mut BattleTowerEReaderTrainer) {
    (*ereaderTrainer).checksum = 0;
    for i in 0..46i32 {
        (*ereaderTrainer).checksum += *(ereaderTrainer as *mut u32).at(i);
    }
}
pub unsafe fn ClearEReaderTrainer(ereaderTrainer: *mut BattleTowerEReaderTrainer) {
    for i in 0..47u32 {
        *(ereaderTrainer as *mut u32).at(i) = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CopyEReaderTrainerGreeting() {
    FrontierSpeechToString(
        (*gSaveBlock2Ptr)
            .frontier
            .ereaderTrainer
            .greeting
            .as_mut_ptr(),
    );
}
unsafe fn CopyEReaderTrainerFarewellMessage() {
    if gBattleOutcome == B_OUTCOME_DREW {
        gStringVar4[0] = EOS;
    } else if gBattleOutcome == B_OUTCOME_WON {
        FrontierSpeechToString(
            (*gSaveBlock2Ptr)
                .frontier
                .ereaderTrainer
                .farewellPlayerWon
                .as_mut_ptr(),
        );
    } else {
        FrontierSpeechToString(
            (*gSaveBlock2Ptr)
                .frontier
                .ereaderTrainer
                .farewellPlayerLost
                .as_mut_ptr(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn TryHideBattleTowerReporter() {
    if (*gSaveBlock2Ptr).frontier.challengeStatus == CHALLENGE_STATUS_SAVING {
        HideBattleTowerReporter();
    }
    if FlagGet(FLAG_CANCEL_BATTLE_ROOM_CHALLENGE) == TRUE {
        HideBattleTowerReporter();
        FlagClear(FLAG_CANCEL_BATTLE_ROOM_CHALLENGE);
    }
}
unsafe fn FillPartnerParty(mut trainerId: u16) {
    let mut j: i32 = 0;
    let mut ivs: u32 = 0;
    let mut level: u32 = 0;
    let mut friendship: u32 = 0;
    let mut monId: u16 = 0;
    let mut otID: u32 = 0;
    let mut trainerName: CArray<u8, 8> = zeroed();
    SetFacilityPtrsGetLevel();
    if trainerId == TRAINER_STEVEN_PARTNER {
        for i in 0..MULTI_PARTY_SIZE {
            loop {
                j = Random() as i32 | (Random() as i32) << 16;
                if !(IsShinyOtIdPersonality(STEVEN_OTID, j as u32) != 0
                    || sStevenMons[i].nature != GetNatureFromPersonality(j as u32))
                {
                    break;
                }
            }
            CreateMon(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                sStevenMons[i].species,
                sStevenMons[i].level,
                sStevenMons[i].fixedIV,
                TRUE,
                i as u32,
                OT_ID_PRESET,
                STEVEN_OTID,
            );
            j = 0;
            while j < PARTY_SIZE {
                SetMonData(
                    &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                    MON_DATA_HP_EV + j,
                    (&raw const sStevenMons[i].evs[j]).cast_mut() as *mut c_void,
                );
                j += 1;
            }
            j = 0;
            while j < MAX_MON_MOVES {
                SetMonMoveSlot(
                    &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                    sStevenMons[i].moves[j],
                    j as u8,
                );
                j += 1;
            }
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_NAME,
                (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
                    [804]
                    .trainerName
                    .as_ptr()
                    .cast_mut() as *mut c_void,
            );
            j = MALE as i32;
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_GENDER,
                &raw mut j as *mut c_void,
            );
            CalculateMonStats(&raw mut gPlayerParty[MULTI_PARTY_SIZE + i]);
        }
    } else if trainerId == TRAINER_EREADER {
        trainerName[0] = gGameLanguage;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        level = SetFacilityPtrsGetLevel() as u32;
        ivs = GetFrontierTrainerFixedIvs(trainerId) as u32;
        otID = Random() as u32 | (Random() as u32) << 16;
        for i in 0..FRONTIER_MULTI_PARTY_SIZE {
            monId = (*gSaveBlock2Ptr).frontier.trainerIds[i + 18];
            CreateMonWithEVSpreadNatureOTID(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                (*gFacilityTrainerMons.at(monId)).species,
                level as u8,
                (*gFacilityTrainerMons.at(monId)).nature,
                ivs as u8,
                (*gFacilityTrainerMons.at(monId)).evSpread,
                otID,
            );
            friendship = MAX_FRIENDSHIP as u32;
            j = 0;
            while j < MAX_MON_MOVES {
                SetMonMoveSlot(
                    &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                    (*gFacilityTrainerMons.at(monId)).moves[j],
                    j as u8,
                );
                if (*gFacilityTrainerMons.at(monId)).moves[j] == MOVE_FRUSTRATION {
                    friendship = 0;
                }
                j += 1;
            }
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_FRIENDSHIP,
                &raw mut friendship as *mut c_void,
            );
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_HELD_ITEM,
                (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                    .cast_mut() as *mut c_void,
            );
            j = 0;
            while j < 8 {
                trainerName[j] = (*gFacilityTrainers.at(trainerId)).trainerName[j];
                j += 1;
            }
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_NAME,
                &raw mut trainerName as *mut c_void,
            );
            j = IsFrontierTrainerFemale(trainerId) as i32;
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_GENDER,
                &raw mut j as *mut c_void,
            );
        }
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        trainerId -= TRAINER_RECORD_MIXING_FRIEND as u16;
        for i in 0..FRONTIER_MULTI_PARTY_SIZE {
            let record: *mut EmeraldBattleTowerRecord =
                &raw mut (*gSaveBlock2Ptr).frontier.towerRecords[trainerId];
            let mut monData: BattleTowerPokemon =
                (*record).party[(*gSaveBlock2Ptr).frontier.trainerIds[18 + i]];
            StringCopy(trainerName.as_mut_ptr(), (*record).name.as_mut_ptr());
            if (*record).language == LANGUAGE_JAPANESE {
                if monData.nickname[0] != EXT_CTRL_CODE_BEGIN
                    || monData.nickname[1] != EXT_CTRL_CODE_JPN
                {
                    monData.nickname[5] = EOS;
                    ConvertInternationalString(monData.nickname.as_mut_ptr(), LANGUAGE_JAPANESE);
                }
            } else {
                if monData.nickname[0] == EXT_CTRL_CODE_BEGIN
                    && monData.nickname[1] == EXT_CTRL_CODE_JPN
                {
                    trainerName[5] = EOS;
                }
            }
            CreateBattleTowerMon_HandleLevel(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                &raw mut monData,
                TRUE,
            );
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_NAME,
                trainerName.as_mut_ptr() as *mut c_void,
            );
            j = IsFrontierTrainerFemale(trainerId + TRAINER_RECORD_MIXING_FRIEND as u16) as i32;
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_GENDER,
                &raw mut j as *mut c_void,
            );
        }
    } else {
        trainerId -= TRAINER_RECORD_MIXING_APPRENTICE as u16;
        for i in 0..FRONTIER_MULTI_PARTY_SIZE {
            CreateApprenticeMon(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                &raw mut (*gSaveBlock2Ptr).apprentices[trainerId],
                (*gSaveBlock2Ptr).frontier.trainerIds[18 + i] as u8,
            );
            j = IsFrontierTrainerFemale(trainerId + TRAINER_RECORD_MIXING_APPRENTICE as u16) as i32;
            SetMonData(
                &raw mut gPlayerParty[MULTI_PARTY_SIZE + i],
                MON_DATA_OT_GENDER,
                &raw mut j as *mut c_void,
            );
        }
    }
}
pub unsafe fn RubyBattleTowerRecordToEmerald(
    src: *mut RSBattleTowerRecord,
    dst: *mut EmeraldBattleTowerRecord,
) -> u32 {
    let mut validMons: i32 = 0;
    let mut i: i32 = 0;
    while i < FRONTIER_PARTY_SIZE {
        if (*src).party[i].species != 0 {
            validMons += 1;
        }
        i += 1;
    }
    if validMons != FRONTIER_PARTY_SIZE {
        memset(dst as *mut u8, 0, 236);
        return FALSE as u32;
    } else {
        (*dst).lvlMode = (*src).lvlMode;
        (*dst).winStreak = (*src).winStreak;
        i = 0;
        while i < 75 {
            if sRubyFacilityClassToEmerald[i][0] == (*src).facilityClass {
                break;
            }
            i += 1;
        }
        if i != FACILITY_CLASSES_COUNT {
            (*dst).facilityClass = sRubyFacilityClassToEmerald[i][1];
        } else {
            (*dst).facilityClass = FACILITY_CLASS_YOUNGSTER;
        }
        for i in 0..8i32 {
            (*dst).name[i] = (*src).name[i];
        }
        for i in 0..(TRAINER_ID_LENGTH as i32) {
            (*dst).trainerId[i] = (*src).trainerId[i];
        }
        for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
            (*dst).greeting[i] = (*src).greeting[i];
        }
        for i in 0..EASY_CHAT_BATTLE_WORDS_COUNT {
            (*dst).speechWon[i] = sRecordTrainerSpeechWon[i];
        }
        i = 0;
        while i < EASY_CHAT_BATTLE_WORDS_COUNT {
            (*dst).speechLost[i] = sRecordTrainerSpeechLost[i];
            i += 1;
        }
        for i in 0..FRONTIER_PARTY_SIZE {
            (*dst).party[i] = (*src).party[i];
        }
        {
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    &raw mut (*dst).party[3] as *mut c_void,
                    0x500000b,
                );
            }
        }
        CalcEmeraldBattleTowerChecksum(dst);
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn EmeraldBattleTowerRecordToRuby(
    src: *mut EmeraldBattleTowerRecord,
    dst: *mut RSBattleTowerRecord,
) -> u32 {
    let mut validMons: i32 = 0;
    let mut i: i32 = 0;
    while i < FRONTIER_PARTY_SIZE {
        if (*src).party[i].species != 0 {
            validMons += 1;
        }
        i += 1;
    }
    if validMons != FRONTIER_PARTY_SIZE {
        memset(dst as *mut u8, 0, 164);
        return FALSE as u32;
    } else {
        (*dst).lvlMode = (*src).lvlMode;
        (*dst).winStreak = (*src).winStreak;
        i = 0;
        while i < 75 {
            if sRubyFacilityClassToEmerald[i][1] == (*src).facilityClass {
                break;
            }
            i += 1;
        }
        if i != FACILITY_CLASSES_COUNT {
            (*dst).facilityClass = sRubyFacilityClassToEmerald[i][0];
        } else {
            (*dst).facilityClass = RS_FACILITY_CLASS_YOUNGSTER;
        }
        for i in 0..8i32 {
            (*dst).name[i] = (*src).name[i];
        }
        for i in 0..(TRAINER_ID_LENGTH as i32) {
            (*dst).trainerId[i] = (*src).trainerId[i];
        }
        i = 0;
        while i < EASY_CHAT_BATTLE_WORDS_COUNT {
            (*dst).greeting[i] = (*src).greeting[i];
            i += 1;
        }
        for i in 0..FRONTIER_PARTY_SIZE {
            (*dst).party[i] = (*src).party[i];
        }
        CalcRubyBattleTowerChecksum(dst);
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CalcApprenticeChecksum(apprentice: *mut Apprentice) {
    (*apprentice).checksum = 0;
    for i in 0..16i32 {
        (*apprentice).checksum += *(apprentice as *mut u32).at(i);
    }
}
unsafe fn ClearApprentice(apprentice: *mut Apprentice) {
    for i in 0..17i32 {
        *(apprentice as *mut u32).at(i) = 0;
    }
    ResetApprenticeStruct(apprentice);
}
unsafe fn ValidateApprenticesChecksums() {
    for i in 0..APPRENTICE_COUNT {
        let data: *mut u32 = &raw mut (*gSaveBlock2Ptr).apprentices[i] as *mut u32;
        let mut checksum: u32 = 0;
        for j in 0..16i32 {
            checksum += *data.at(j);
        }
        if (*gSaveBlock2Ptr).apprentices[i].checksum != checksum {
            ClearApprentice(&raw mut (*gSaveBlock2Ptr).apprentices[i]);
        }
    }
}
pub unsafe fn GetBattleTowerTrainerLanguage(dst: *mut u8, trainerId: u16) {
    if trainerId == TRAINER_EREADER {
        *dst = gGameLanguage;
    } else if trainerId < FRONTIER_TRAINERS_COUNT {
        *dst = gGameLanguage;
    } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            *dst = GetRecordedBattleRecordMixFriendLanguage();
        } else {
            *dst = (*gSaveBlock2Ptr).frontier.towerRecords
                [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                .language;
        }
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            *dst = GetRecordedBattleApprenticeLanguage();
        } else {
            *dst = (*gSaveBlock2Ptr).apprentices
                [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                .language;
        }
    }
}
pub unsafe fn SetFacilityPtrsGetLevel() -> u8 {
    if (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_TENT {
        return SetTentPtrsGetLevel();
    } else {
        gFacilityTrainers = gBattleFrontierTrainers.as_ptr().cast_mut();
        gFacilityTrainerMons = gBattleFrontierMons.as_ptr().cast_mut();
        return GetFrontierEnemyMonLevel((*gSaveBlock2Ptr).frontier.lvlMode());
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetFrontierEnemyMonLevel(lvlMode: u8) -> u8 {
    let mut level: u8 = 0;
    match lvlMode {
        FRONTIER_LVL_OPEN => {
            level = GetHighestLevelInPlayerParty() as u8;
            if level < FRONTIER_MIN_LEVEL_OPEN as u8 {
                level = FRONTIER_MIN_LEVEL_OPEN as u8;
            }
        }
        _ => {
            level = FRONTIER_MAX_LEVEL_50;
        }
    }
    level
}
pub unsafe fn GetHighestLevelInPlayerParty() -> i32 {
    let mut highestLevel: i32 = 0;
    for i in 0..PARTY_SIZE {
        if GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) != 0
            && GetMonData3(
                &raw mut gPlayerParty[i],
                MON_DATA_SPECIES_OR_EGG,
                null_mut(),
            ) != SPECIES_EGG
        {
            let level: i32 =
                GetMonData3(&raw mut gPlayerParty[i], MON_DATA_LEVEL, null_mut()) as i32;
            if level > highestLevel {
                highestLevel = level;
            }
        }
    }
    highestLevel
}
fn GetFrontierTrainerFixedIvs(trainerId: u16) -> u8 {
    let mut fixedIv: u8 = 0;
    if trainerId <= 99 {
        fixedIv = 3;
    } else if trainerId <= 119 {
        fixedIv = 6;
    } else if trainerId <= 139 {
        fixedIv = 9;
    } else if trainerId <= 159 {
        fixedIv = 12;
    } else if trainerId <= 179 {
        fixedIv = 15;
    } else if trainerId <= 199 {
        fixedIv = 18;
    } else if trainerId <= 219 {
        fixedIv = 21;
    } else {
        fixedIv = MAX_PER_STAT_IVS;
    }
    fixedIv
}
unsafe fn GetBattleTentTrainerId() -> u16 {
    let facility: u32 = VarGet(VAR_FRONTIER_FACILITY) as u32;
    if facility == FRONTIER_FACILITY_PALACE as u32 {
        return (Random() as i32 % 30) as u16;
    } else if facility == FRONTIER_FACILITY_ARENA as u32 {
        return (Random() as i32 % 30) as u16;
    } else if facility == FRONTIER_FACILITY_FACTORY as u32 {
        return (Random() as i32 % 30) as u16;
    } else if facility == FRONTIER_FACILITY_TOWER as u32 {
        return 0;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SetTentPtrsGetLevel() -> u8 {
    let facility: u32 = VarGet(VAR_FRONTIER_FACILITY) as u32;
    if facility == FRONTIER_FACILITY_FACTORY as u32 {
        gFacilityTrainers = gSlateportBattleTentTrainers.as_ptr().cast_mut();
        gFacilityTrainerMons = gSlateportBattleTentMons.as_ptr().cast_mut();
    } else if facility == FRONTIER_FACILITY_PALACE as u32 {
        gFacilityTrainers = gVerdanturfBattleTentTrainers.as_ptr().cast_mut();
        gFacilityTrainerMons = gVerdanturfBattleTentMons.as_ptr().cast_mut();
    } else if facility == FRONTIER_FACILITY_ARENA as u32 {
        gFacilityTrainers = gFallarborBattleTentTrainers.as_ptr().cast_mut();
        gFacilityTrainerMons = gFallarborBattleTentMons.as_ptr().cast_mut();
    } else {
        gFacilityTrainers = gBattleFrontierTrainers.as_ptr().cast_mut();
        gFacilityTrainerMons = gBattleFrontierMons.as_ptr().cast_mut();
    }
    let mut level: u8 = GetHighestLevelInPlayerParty() as u8;
    if level < TENT_MIN_LEVEL {
        level = TENT_MIN_LEVEL;
    }
    level
}
unsafe fn SetNextBattleTentOpponent() {
    let mut i: i32 = 0;
    let mut trainerId: u16 = 0;
    loop {
        trainerId = GetBattleTentTrainerId();
        i = 0;
        while i < (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                break;
            }
            i += 1;
        }
        if i == (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 {
            break;
        }
    }
    gTrainerBattleOpponent_A = trainerId;
    SetBattleFacilityTrainerGfxId(gTrainerBattleOpponent_A, 0);
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32 + 1 < TENT_STAGES_PER_CHALLENGE {
        (*gSaveBlock2Ptr).frontier.trainerIds[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum] =
            gTrainerBattleOpponent_A;
    }
}
unsafe fn FillTentTrainerParty_(trainerId: u16, firstMonId: u8, monCount: u8) {
    let mut j: i32 = 0;
    let mut chosenMonIndices: CArray<u16, 4> = zeroed();
    let mut friendship: u8 = 0;
    let level: u8 = SetTentPtrsGetLevel();
    let fixedIV: u8 = 0;
    let mut monSet: *mut u16 = null_mut();
    monSet = (*gFacilityTrainers.at(gTrainerBattleOpponent_A)).monSet;
    let mut bfMonCount: u8 = 0;
    let mut monId: u16 = *monSet.at(bfMonCount);
    while monId != 0xFFFF {
        bfMonCount += 1;
        monId = *monSet.at(bfMonCount);
        if monId == 0xFFFF {
            break;
        }
    }
    let mut i: i32 = 0;
    let otID: u32 = Random() as u32 | (Random() as u32) << 16;
    while i != monCount as i32 {
        let monId: u16 = *monSet.at(rem_i32(Random() as i32, bfMonCount as i32));
        j = 0;
        while j < i + firstMonId as i32 {
            if GetMonData3(&raw mut gEnemyParty[j], MON_DATA_SPECIES, null_mut())
                == (*gFacilityTrainerMons.at(monId)).species as u32
            {
                break;
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        j = 0;
        while j < i + firstMonId as i32 {
            if GetMonData3(&raw mut gEnemyParty[j], MON_DATA_HELD_ITEM, null_mut())
                != ITEM_NONE as u32
                && GetMonData3(&raw mut gEnemyParty[j], MON_DATA_HELD_ITEM, null_mut())
                    == gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId]
                        as u32
            {
                break;
            }
            j += 1;
        }
        if j != i + firstMonId as i32 {
            continue;
        }
        j = 0;
        while j < i {
            if chosenMonIndices[j] == monId {
                break;
            }
            j += 1;
        }
        if j != i {
            continue;
        }
        chosenMonIndices[i] = monId;
        CreateMonWithEVSpreadNatureOTID(
            &raw mut gEnemyParty[i + firstMonId as i32],
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            fixedIV,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otID,
        );
        friendship = MAX_FRIENDSHIP;
        for j in 0..MAX_MON_MOVES {
            SetMonMoveSlot(
                &raw mut gEnemyParty[i + firstMonId as i32],
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j as u8,
            );
            if (*gFacilityTrainerMons.at(monId)).moves[j] == MOVE_FRUSTRATION {
                friendship = 0;
            }
        }
        SetMonData(
            &raw mut gEnemyParty[i + firstMonId as i32],
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut gEnemyParty[i + firstMonId as i32],
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
        i += 1;
    }
}
pub unsafe fn FacilityClassToGraphicsId(facilityClass: u8) -> u8 {
    let mut trainerObjectGfxId: u8 = 0;
    let mut i: u8 = 0;
    while i < 30 {
        if gTowerMaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 30 {
        trainerObjectGfxId = gTowerMaleTrainerGfxIds[i];
        return trainerObjectGfxId;
    }
    i = 0;
    while i < 20 {
        if gTowerFemaleFacilityClasses[i] == facilityClass {
            break;
        }
        i += 1;
    }
    if i != 20 {
        trainerObjectGfxId = gTowerFemaleTrainerGfxIds[i];
        return trainerObjectGfxId;
    } else {
        return OBJ_EVENT_GFX_BOY_1 as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ValidateBattleTowerRecord(recordId: u8) -> u32 {
    let record: *mut u32 = &raw mut (*gSaveBlock2Ptr).frontier.towerRecords[recordId] as *mut u32;
    let mut checksum: u32 = 0;
    let mut hasData: u32 = 0;
    for i in 0..58i32 {
        checksum += *record.at(i);
        hasData |= *record.at(i);
    }
    if checksum == 0 && hasData == 0 {
        return FALSE as u32;
    } else if (*gSaveBlock2Ptr).frontier.towerRecords[recordId].checksum != checksum {
        ClearBattleTowerRecord(&raw mut (*gSaveBlock2Ptr).frontier.towerRecords[recordId]);
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn TrySetLinkBattleTowerEnemyPartyLevel() {
    if gBattleTypeFlags & 0x2000002 != 0 {
        let enemyLevel: u8 = SetFacilityPtrsGetLevel();
        for i in 0..PARTY_SIZE {
            let species: u32 = GetMonData3(&raw mut gEnemyParty[i], MON_DATA_SPECIES, null_mut());
            if species != 0 {
                SetMonData(
                    &raw mut gEnemyParty[i],
                    MON_DATA_EXP,
                    (&raw const (*(&raw const crate::data::pokemon::gExperienceTables)
                        .cast::<CArray<CArray<u32, 101>, 0>>())
                        [(*(&raw const crate::data::pokemon::gSpeciesInfo)
                            .cast::<CArray<SpeciesInfo, 0>>())[species]
                            .growthRate][enemyLevel])
                        .cast_mut() as *mut c_void,
                );
                CalculateMonStats(&raw mut gEnemyParty[i]);
            }
        }
    }
}
