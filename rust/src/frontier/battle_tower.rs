//! Translated from `src/battle_tower.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gBattleFrontierHeldItems gBattleFrontierTrainerMons_Brady gBattleFrontierTrainerMons_Conner gBattleFrontierTrainerMons_Bradley gBattleFrontierTrainerMons_Cybil gBattleFrontierTrainerMons_Rodette gBattleFrontierTrainerMons_Peggy gBattleFrontierTrainerMons_Keith gBattleFrontierTrainerMons_Grayson gBattleFrontierTrainerMons_Glenn gBattleFrontierTrainerMons_Liliana gBattleFrontierTrainerMons_Elise gBattleFrontierTrainerMons_Zoey gBattleFrontierTrainerMons_Manuel gBattleFrontierTrainerMons_Russ gBattleFrontierTrainerMons_Dustin gBattleFrontierTrainerMons_Tina gBattleFrontierTrainerMons_Gillian gBattleFrontierTrainerMons_Zoe gBattleFrontierTrainerMons_Chen gBattleFrontierTrainerMons_Al gBattleFrontierTrainerMons_Mitch gBattleFrontierTrainerMons_Anne gBattleFrontierTrainerMons_Alize gBattleFrontierTrainerMons_Lauren gBattleFrontierTrainerMons_Kipp gBattleFrontierTrainerMons_Jason gBattleFrontierTrainerMons_John gBattleFrontierTrainerMons_Ann gBattleFrontierTrainerMons_Eileen gBattleFrontierTrainerMons_Carlie gBattleFrontierTrainerMons_Gordon gBattleFrontierTrainerMons_Ayden gBattleFrontierTrainerMons_Marco gBattleFrontierTrainerMons_Cierra gBattleFrontierTrainerMons_Marcy gBattleFrontierTrainerMons_Kathy gBattleFrontierTrainerMons_Peyton gBattleFrontierTrainerMons_Julian gBattleFrontierTrainerMons_Quinn gBattleFrontierTrainerMons_Haylee gBattleFrontierTrainerMons_Amanda gBattleFrontierTrainerMons_Stacy gBattleFrontierTrainerMons_Rafael gBattleFrontierTrainerMons_Oliver gBattleFrontierTrainerMons_Payton gBattleFrontierTrainerMons_Pamela gBattleFrontierTrainerMons_Eliza gBattleFrontierTrainerMons_Marisa gBattleFrontierTrainerMons_Lewis gBattleFrontierTrainerMons_Yoshi gBattleFrontierTrainerMons_Destin gBattleFrontierTrainerMons_Keon gBattleFrontierTrainerMons_Stuart gBattleFrontierTrainerMons_Nestor gBattleFrontierTrainerMons_Derrick gBattleFrontierTrainerMons_Bryson gBattleFrontierTrainerMons_Clayton gBattleFrontierTrainerMons_Trenton gBattleFrontierTrainerMons_Jenson gBattleFrontierTrainerMons_Wesley gBattleFrontierTrainerMons_Anton gBattleFrontierTrainerMons_Lawson gBattleFrontierTrainerMons_Sammy gBattleFrontierTrainerMons_Arnie gBattleFrontierTrainerMons_Adrian gBattleFrontierTrainerMons_Tristan gBattleFrontierTrainerMons_Juliana gBattleFrontierTrainerMons_Rylee gBattleFrontierTrainerMons_Chelsea gBattleFrontierTrainerMons_Danela gBattleFrontierTrainerMons_Lizbeth gBattleFrontierTrainerMons_Amelia gBattleFrontierTrainerMons_Jillian gBattleFrontierTrainerMons_Abbie gBattleFrontierTrainerMons_Briana gBattleFrontierTrainerMons_Antonio gBattleFrontierTrainerMons_Jaden gBattleFrontierTrainerMons_Dakota gBattleFrontierTrainerMons_Brayden gBattleFrontierTrainerMons_Corson gBattleFrontierTrainerMons_Trevin gBattleFrontierTrainerMons_Patrick gBattleFrontierTrainerMons_Kaden gBattleFrontierTrainerMons_Maxwell gBattleFrontierTrainerMons_Daryl gBattleFrontierTrainerMons_Kenneth gBattleFrontierTrainerMons_Rich gBattleFrontierTrainerMons_Caden gBattleFrontierTrainerMons_Marlon gBattleFrontierTrainerMons_Nash gBattleFrontierTrainerMons_Robby gBattleFrontierTrainerMons_Reece gBattleFrontierTrainerMons_Kathryn gBattleFrontierTrainerMons_Ellen gBattleFrontierTrainerMons_Ramon gBattleFrontierTrainerMons_Arthur gBattleFrontierTrainerMons_Alondra gBattleFrontierTrainerMons_Adriana gBattleFrontierTrainerMons_Malik gBattleFrontierTrainerMons_Jill gBattleFrontierTrainerMons_Erik gBattleFrontierTrainerMons_Yazmin gBattleFrontierTrainerMons_Jamal gBattleFrontierTrainerMons_Leslie gBattleFrontierTrainerMons_Dave gBattleFrontierTrainerMons_Carlo gBattleFrontierTrainerMons_Emilia gBattleFrontierTrainerMons_Dalia gBattleFrontierTrainerMons_Hitomi gBattleFrontierTrainerMons_Ricardo gBattleFrontierTrainerMons_Shizuka gBattleFrontierTrainerMons_Joana gBattleFrontierTrainerMons_Kelly gBattleFrontierTrainerMons_Rayna gBattleFrontierTrainerMons_Evan gBattleFrontierTrainerMons_Jordan gBattleFrontierTrainerMons_Joel gBattleFrontierTrainerMons_Kristen gBattleFrontierTrainerMons_Selphy gBattleFrontierTrainerMons_Chloe gBattleFrontierTrainerMons_Norton gBattleFrontierTrainerMons_Lukas gBattleFrontierTrainerMons_Zach gBattleFrontierTrainerMons_Kaitlyn gBattleFrontierTrainerMons_Breanna gBattleFrontierTrainerMons_Kendra gBattleFrontierTrainerMons_Molly gBattleFrontierTrainerMons_Jazmin gBattleFrontierTrainerMons_Kelsey gBattleFrontierTrainerMons_Jalen gBattleFrontierTrainerMons_Griffen gBattleFrontierTrainerMons_Xander gBattleFrontierTrainerMons_Marvin gBattleFrontierTrainerMons_Brennan gBattleFrontierTrainerMons_Baley gBattleFrontierTrainerMons_Zackary gBattleFrontierTrainerMons_Gabriel gBattleFrontierTrainerMons_Emily gBattleFrontierTrainerMons_Jordyn gBattleFrontierTrainerMons_Sofia gBattleFrontierTrainerMons_Braden gBattleFrontierTrainerMons_Kayden gBattleFrontierTrainerMons_Cooper gBattleFrontierTrainerMons_Julia gBattleFrontierTrainerMons_Amara gBattleFrontierTrainerMons_Lynn gBattleFrontierTrainerMons_Jovan gBattleFrontierTrainerMons_Dominic gBattleFrontierTrainerMons_Nikolas gBattleFrontierTrainerMons_Valeria gBattleFrontierTrainerMons_Delaney gBattleFrontierTrainerMons_Meghan gBattleFrontierTrainerMons_Roberto gBattleFrontierTrainerMons_Damian gBattleFrontierTrainerMons_Brody gBattleFrontierTrainerMons_Graham gBattleFrontierTrainerMons_Tylor gBattleFrontierTrainerMons_Jaren gBattleFrontierTrainerMons_Cordell gBattleFrontierTrainerMons_Jazlyn gBattleFrontierTrainerMons_Zachery gBattleFrontierTrainerMons_Johan gBattleFrontierTrainerMons_Shea gBattleFrontierTrainerMons_Kaila gBattleFrontierTrainerMons_Isiah gBattleFrontierTrainerMons_Garrett gBattleFrontierTrainerMons_Haylie gBattleFrontierTrainerMons_Megan gBattleFrontierTrainerMons_Issac gBattleFrontierTrainerMons_Quinton gBattleFrontierTrainerMons_Salma gBattleFrontierTrainerMons_Ansley gBattleFrontierTrainerMons_Holden gBattleFrontierTrainerMons_Luca gBattleFrontierTrainerMons_Jamison gBattleFrontierTrainerMons_Gunnar gBattleFrontierTrainerMons_Craig gBattleFrontierTrainerMons_Pierce gBattleFrontierTrainerMons_Regina gBattleFrontierTrainerMons_Alison gBattleFrontierTrainerMons_Hank gBattleFrontierTrainerMons_Earl gBattleFrontierTrainerMons_Ramiro gBattleFrontierTrainerMons_Hunter gBattleFrontierTrainerMons_Aiden gBattleFrontierTrainerMons_Xavier gBattleFrontierTrainerMons_Clinton gBattleFrontierTrainerMons_Jesse gBattleFrontierTrainerMons_Eduardo gBattleFrontierTrainerMons_Hal gBattleFrontierTrainerMons_Gage gBattleFrontierTrainerMons_Arnold gBattleFrontierTrainerMons_Jarrett gBattleFrontierTrainerMons_Garett gBattleFrontierTrainerMons_Emanuel gBattleFrontierTrainerMons_Gustavo gBattleFrontierTrainerMons_Kameron gBattleFrontierTrainerMons_Alfredo gBattleFrontierTrainerMons_Ruben gBattleFrontierTrainerMons_Lamar gBattleFrontierTrainerMons_Jaxon gBattleFrontierTrainerMons_Logan gBattleFrontierTrainerMons_Emilee gBattleFrontierTrainerMons_Josie gBattleFrontierTrainerMons_Armando gBattleFrontierTrainerMons_Skyler gBattleFrontierTrainerMons_Ruth gBattleFrontierTrainerMons_Melody gBattleFrontierTrainerMons_Pedro gBattleFrontierTrainerMons_Erick gBattleFrontierTrainerMons_Elaine gBattleFrontierTrainerMons_Joyce gBattleFrontierTrainerMons_Todd gBattleFrontierTrainerMons_Gavin gBattleFrontierTrainerMons_Malory gBattleFrontierTrainerMons_Esther gBattleFrontierTrainerMons_Oscar gBattleFrontierTrainerMons_Wilson gBattleFrontierTrainerMons_Clare gBattleFrontierTrainerMons_Tess gBattleFrontierTrainerMons_Leon gBattleFrontierTrainerMons_Alonzo gBattleFrontierTrainerMons_Vince gBattleFrontierTrainerMons_Bryon gBattleFrontierTrainerMons_Ava gBattleFrontierTrainerMons_Miriam gBattleFrontierTrainerMons_Carrie gBattleFrontierTrainerMons_Gillian2 gBattleFrontierTrainerMons_Tyler gBattleFrontierTrainerMons_Chaz gBattleFrontierTrainerMons_Nelson gBattleFrontierTrainerMons_Shania gBattleFrontierTrainerMons_Stella gBattleFrontierTrainerMons_Dorine gBattleFrontierTrainerMons_Maddox gBattleFrontierTrainerMons_Davin gBattleFrontierTrainerMons_Trevon gBattleFrontierTrainerMons_Mateo gBattleFrontierTrainerMons_Bret gBattleFrontierTrainerMons_Raul gBattleFrontierTrainerMons_Kay gBattleFrontierTrainerMons_Elena gBattleFrontierTrainerMons_Alana gBattleFrontierTrainerMons_Alexas gBattleFrontierTrainerMons_Weston gBattleFrontierTrainerMons_Jasper gBattleFrontierTrainerMons_Nadia gBattleFrontierTrainerMons_Miranda gBattleFrontierTrainerMons_Emma gBattleFrontierTrainerMons_Rolando gBattleFrontierTrainerMons_Stanly gBattleFrontierTrainerMons_Dario gBattleFrontierTrainerMons_Karlee gBattleFrontierTrainerMons_Jaylin gBattleFrontierTrainerMons_Ingrid gBattleFrontierTrainerMons_Delilah gBattleFrontierTrainerMons_Carly gBattleFrontierTrainerMons_Lexie gBattleFrontierTrainerMons_Miller gBattleFrontierTrainerMons_Marv gBattleFrontierTrainerMons_Layton gBattleFrontierTrainerMons_Brooks gBattleFrontierTrainerMons_Gregory gBattleFrontierTrainerMons_Reese gBattleFrontierTrainerMons_Mason gBattleFrontierTrainerMons_Toby gBattleFrontierTrainerMons_Dorothy gBattleFrontierTrainerMons_Piper gBattleFrontierTrainerMons_Finn gBattleFrontierTrainerMons_Samir gBattleFrontierTrainerMons_Fiona gBattleFrontierTrainerMons_Gloria gBattleFrontierTrainerMons_Nico gBattleFrontierTrainerMons_Jeremy gBattleFrontierTrainerMons_Caitlin gBattleFrontierTrainerMons_Reena gBattleFrontierTrainerMons_Avery gBattleFrontierTrainerMons_Liam gBattleFrontierTrainerMons_Theo gBattleFrontierTrainerMons_Bailey gBattleFrontierTrainerMons_Hugo gBattleFrontierTrainerMons_Bryce gBattleFrontierTrainerMons_Gideon gBattleFrontierTrainerMons_Triston gBattleFrontierTrainerMons_Charles gBattleFrontierTrainerMons_Raymond gBattleFrontierTrainerMons_Dirk gBattleFrontierTrainerMons_Harold gBattleFrontierTrainerMons_Omar gBattleFrontierTrainerMons_Peter gBattleFrontierTrainerMons_Dev gBattleFrontierTrainerMons_Corey gBattleFrontierTrainerMons_Andre gBattleFrontierTrainerMons_Ferris gBattleFrontierTrainerMons_Alivia gBattleFrontierTrainerMons_Paige gBattleFrontierTrainerMons_Anya gBattleFrontierTrainerMons_Dawn gBattleFrontierTrainerMons_Abby gBattleFrontierTrainerMons_Gretel gBattleFrontierTrainers gBattleFrontierMons gTowerMaleFacilityClasses gTowerFemaleFacilityClasses gTowerMaleTrainerGfxIds gTowerFemaleTrainerGfxIds sRubyFacilityClassToEmerald sPartnerApprenticeTexts1 sPartnerApprenticeTexts2 sPartnerApprenticeTexts3 sPartnerApprenticeTexts4 sPartnerApprenticeTexts5 sPartnerApprenticeTexts6 sPartnerApprenticeTexts7 sPartnerApprenticeTexts8 sPartnerApprenticeTexts9 sPartnerApprenticeTexts10 sPartnerApprenticeTexts11 sPartnerApprenticeTexts12 sPartnerApprenticeTexts13 sPartnerApprenticeTexts14 sPartnerApprenticeTexts15 sPartnerApprenticeTexts16 sPartnerTextsLass sPartnerTextsYoungster sPartnerTextsHiker sPartnerTextsBeauty sPartnerTextsFisherman sPartnerTextsLady sPartnerTextsCyclingTriathleteF sPartnerTextsBugCatcher sPartnerTextsSchoolKidM sPartnerTextsRichBoy sPartnerTextsBlackBelt sPartnerTextsTuberF sPartnerTextsHexManiac sPartnerTextsPkmnBreederM sPartnerTextsRunningTriathleteF sPartnerTextsRunningTriathleteM sPartnerTextsBattleGirl sPartnerTextsCyclingTriathleteM sPartnerTextsTuberM sPartnerTextsGuitarist sPartnerTextsGentleman sPartnerTextsPokefanM sPartnerTextsExpertM sPartnerTextsExpertF sPartnerTextsDragonTamer sPartnerTextsBirdKeeper sPartnerTextsNinjaBoy sPartnerTextsParasolLady sPartnerTextsBugManiac sPartnerTextsSailor sPartnerTextsCollector sPartnerTextsPkmnRangerM sPartnerTextsPkmnRangerF sPartnerTextsAromaLady sPartnerTextsRuinManiac sPartnerTextsCoolTrainerM sPartnerTextsCoolTrainerF sPartnerTextsPokemaniac sPartnerTextsKindler sPartnerTextsCamper sPartnerTextsPicnicker sPartnerTextsPsychicM sPartnerTextsPsychicF sPartnerTextsSchoolKidF sPartnerTextsPkmnBreederF sPartnerTextsPokefanF sPartnerTextsSwimmerF sPartnerTextsSwimmingTriathleteM sPartnerTextsSwimmingTriathleteF sPartnerTextsSwimmerM sPartnerTrainerTextTables sPartnerApprenticeTextTables sStevenMons gSlateportBattleTentTrainerMons_Jolie gSlateportBattleTentTrainerMons_Malachi gSlateportBattleTentTrainerMons_Kelsie gSlateportBattleTentTrainerMons_Davon gSlateportBattleTentTrainerMons_Glenda gSlateportBattleTentTrainerMons_Helena gSlateportBattleTentTrainerMons_Rodolfo gSlateportBattleTentTrainerMons_Davion gSlateportBattleTentTrainerMons_Kendall gSlateportBattleTentTrainerMons_Colten gSlateportBattleTentTrainerMons_Irvin gSlateportBattleTentTrainerMons_Shaun gSlateportBattleTentTrainerMons_Kyler gSlateportBattleTentTrainerMons_Maggie gSlateportBattleTentTrainerMons_Stephon gSlateportBattleTentTrainerMons_Rebecca gSlateportBattleTentTrainerMons_Reggie gSlateportBattleTentTrainerMons_Janae gSlateportBattleTentTrainerMons_Caiden gSlateportBattleTentTrainerMons_Kirsten gSlateportBattleTentTrainerMons_Kurtis gSlateportBattleTentTrainerMons_Stefan gSlateportBattleTentTrainerMons_Avery gSlateportBattleTentTrainerMons_Dwane gSlateportBattleTentTrainerMons_Mckenna gSlateportBattleTentTrainerMons_Camryn gSlateportBattleTentTrainerMons_Natasha gSlateportBattleTentTrainerMons_Austyn gSlateportBattleTentTrainerMons_Donovan gSlateportBattleTentTrainerMons_Tamia gSlateportBattleTentTrainers gSlateportBattleTentMons gVerdanturfBattleTentTrainerMons_Brenna gVerdanturfBattleTentTrainerMons_Dilan gVerdanturfBattleTentTrainerMons_Eliana gVerdanturfBattleTentTrainerMons_Markus gVerdanturfBattleTentTrainerMons_Caitlyn gVerdanturfBattleTentTrainerMons_Desiree gVerdanturfBattleTentTrainerMons_Ronald gVerdanturfBattleTentTrainerMons_Ashten gVerdanturfBattleTentTrainerMons_Gerard gVerdanturfBattleTentTrainerMons_Bradly gVerdanturfBattleTentTrainerMons_Dennis gVerdanturfBattleTentTrainerMons_Prestin gVerdanturfBattleTentTrainerMons_Ernesto gVerdanturfBattleTentTrainerMons_Nala gVerdanturfBattleTentTrainerMons_Darnell gVerdanturfBattleTentTrainerMons_Ashlyn gVerdanturfBattleTentTrainerMons_Addison gVerdanturfBattleTentTrainerMons_Justine gVerdanturfBattleTentTrainerMons_Tyson gVerdanturfBattleTentTrainerMons_Laila gVerdanturfBattleTentTrainerMons_Waren gVerdanturfBattleTentTrainerMons_Tobias gVerdanturfBattleTentTrainerMons_Josiah gVerdanturfBattleTentTrainerMons_Dion gVerdanturfBattleTentTrainerMons_Kenzie gVerdanturfBattleTentTrainerMons_Lillian gVerdanturfBattleTentTrainerMons_Lesley gVerdanturfBattleTentTrainerMons_Marquis gVerdanturfBattleTentTrainerMons_Freddy gVerdanturfBattleTentTrainerMons_Cecilia gVerdanturfBattleTentTrainers gVerdanturfBattleTentMons gFallarborBattleTentTrainerMons_Amber gFallarborBattleTentTrainerMons_Javier gFallarborBattleTentTrainerMons_Natalie gFallarborBattleTentTrainerMons_Treve gFallarborBattleTentTrainerMons_Arianna gFallarborBattleTentTrainerMons_Jadyn gFallarborBattleTentTrainerMons_Gerardo gFallarborBattleTentTrainerMons_Jonn gFallarborBattleTentTrainerMons_Esteban gFallarborBattleTentTrainerMons_Jameson gFallarborBattleTentTrainerMons_Alanzo gFallarborBattleTentTrainerMons_Howard gFallarborBattleTentTrainerMons_Conrad gFallarborBattleTentTrainerMons_Makenna gFallarborBattleTentTrainerMons_Brayan gFallarborBattleTentTrainerMons_Mariana gFallarborBattleTentTrainerMons_Sheldon gFallarborBattleTentTrainerMons_Gianna gFallarborBattleTentTrainerMons_Yahir gFallarborBattleTentTrainerMons_Britney gFallarborBattleTentTrainerMons_Hecter gFallarborBattleTentTrainerMons_Tannor gFallarborBattleTentTrainerMons_Benji gFallarborBattleTentTrainerMons_Rory gFallarborBattleTentTrainerMons_Eleanor gFallarborBattleTentTrainerMons_Evelyn gFallarborBattleTentTrainerMons_Arielle gFallarborBattleTentTrainerMons_Connar gFallarborBattleTentTrainerMons_Maurice gFallarborBattleTentTrainerMons_Kianna gFallarborBattleTentTrainers gFallarborBattleTentMons sBattleTowerFuncs sWinStreakFlags sWinStreakMasks sApprenticeChallengeThreshold sBattleTowerPartySizes2 sFrontierTrainerIdRanges sFrontierTrainerIdRangesHard sUnused sBattleTowerPartySizes sRecordTrainerSpeechWon sRecordTrainerSpeechLost
#[allow(unused_imports)]
use crate::data::battle_tower::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gFacilityTrainers: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gFacilityTrainerMons: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFrontierTempParty: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut MossdeepCity_SpaceCenter_2F_EventScript_MaxieTrainer: u8;
    static mut MossdeepCity_SpaceCenter_2F_EventScript_TabithaTrainer: u8;
    static mut gApprentices: u8;
    static mut gApproachingTrainerId: u8;
    static mut gBattleMons: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleScripting: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gFacilityClassToPicIndex: u8;
    static mut gFacilityClassToTrainerClass: u8;
    static mut gGameLanguage: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gPartnerTrainerId: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerBattleOpponent_B: u8;
    static mut gTrainers: u8;
    static mut gWirelessCommType: u8;
    fn BattleSetup_ConfigureTrainerBattle(a0: *mut u8) -> *mut u8;
    fn BattleTransition_StartOnField(a0: u8);
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BufferApprenticeChallengeText(a0: u8);
    fn CB2_InitBattle();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CalculateMonStats(a0: *mut u8);
    fn ConvertEasyChatWordsToString(a0: *mut u8, a1: *mut u16, a2: u16, a3: u16) -> *mut u8;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPokemonToBattleTowerPokemon(a0: *mut u8, a1: *mut u8);
    fn CopyFrontierBrainTrainerName(a0: *mut u8);
    fn CopyTrainerId(a0: *mut u8, a1: *mut u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateApprenticeMon(a0: *mut u8, a1: *mut u8, a2: u8);
    fn CreateBattleTowerMon(a0: *mut u8, a1: *mut u8);
    fn CreateBattleTowerMon_HandleLevel(a0: *mut u8, a1: *mut u8, a2: u8);
    fn CreateFrontierBrainPokemon();
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonWithEVSpread(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8);
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_PlayMapChosenOrBattleBGM(a0: u16);
    fn DestroyTask(a0: u8);
    fn FillFactoryBrainParty();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn GetApprenticeNameInLanguage(a0: u32, a1: i32) -> *mut u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCurrentFacilityWinStreak() -> u32;
    fn GetFactoryMonFixedIV(a0: u8, a1: u8) -> u8;
    fn GetFrontierBrainTrainerClass() -> u8;
    fn GetFrontierBrainTrainerPicIndex() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetRecordedBattleApprenticeId() -> u8;
    fn GetRecordedBattleApprenticeLanguage() -> u8;
    fn GetRecordedBattleRecordMixFriendClass() -> u8;
    fn GetRecordedBattleRecordMixFriendLanguage() -> u8;
    fn GetRecordedBattleRecordMixFriendName(a0: *mut u8);
    fn GetRibbonCount(a0: *mut u8) -> u8;
    fn GetSpecialBattleTransition(a0: i32) -> u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HideBattleTowerReporter();
    fn IncrementGameStat(a0: u8);
    fn IsBattleTransitionDone() -> u8;
    fn IsFrontierBrainFemale() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsShinyOtIdPersonality(a0: u32, a1: u32) -> u8;
    fn PlayMapChosenOrBattleBGM(a0: u16);
    fn Random() -> u16;
    fn RecordedBattle_SaveBattleOutcome();
    fn ResetApprenticeStruct(a0: *mut u8);
    fn ResetBlockReceivedFlags();
    fn ResetFrontierTrainerIds();
    fn SaveGameFrontier();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetCloseLinkCallback();
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetFrontierBrainObjEventGfx_2();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveAvoidReturn(a0: *mut u8, a1: u16, a2: u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_PlayerName(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn TryPutSpotTheCutiesOnAir(a0: *mut u8, a1: u8);
    fn UpdateGymLeaderRematch();
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattleTowerFunc() {
    unsafe {
        (((((&raw const sBattleTowerFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn InitTowerChallenge() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(1u8);
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .write(0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (0u8) as i32,
        );
        ResetFrontierTrainerIds();
        if !(((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .read()
            & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 8))
            .cast::<u32>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read())
            != 0)
        {
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1684))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(0u16);
        }
        ValidateBattleTowerRecordChecksums();
        SetDynamicWarp(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
        );
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetTowerData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    GetCurrentBattleTowerWinStreak(((lvlMode) as u8), ((battleMode) as u8)),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>())
                    .read()
                        & ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((battleMode) as i32) as isize * 8))
                        .cast::<u32>())
                        .wrapping_offset(((lvlMode) as i32) as isize))
                        .read())
                        != 0u32) as u16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1723))
                .write(
                    (crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTowerData() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(((lvlMode) as i32) as isize))
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) != 0 {
                    let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p2).write(
                        ((__p2).read()
                            | ((((((&raw const sWinStreakFlags).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((battleMode) as i32) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset(((lvlMode) as i32) as isize))
                            .read()),
                    );
                } else {
                    let __p3 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1680)
                    .cast::<u32>();
                    (__p3).write(
                        ((__p3).read()
                            & ((((((&raw const sWinStreakMasks).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((battleMode) as i32) as isize * 8))
                            .cast::<u32>())
                            .wrapping_offset(((lvlMode) as i32) as isize))
                            .read()),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1723))
                .write(
                    (crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTowerBattleWon() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 500i32 {
            ClearEReaderTrainer(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1440),
            );
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1720)
            .cast::<u16>())
        .read()) as i32)
            < 9999i32
        {
            let __p1 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1720)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        SaveCurrentWinStreak();
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ChooseSpecialBattleTowerTrainer() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut validMons: i32 = 0i32;
        let mut trainerIds = crate::ffi::Align4([0u8; 36]);
        let mut idsCount: i32 = 0i32;
        let mut winStreak: i32 = 0i32;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        if ((VarGet(16591u16)) as i32) != 0i32 {
            return 0u8;
        }
        winStreak = ((GetCurrentBattleTowerWinStreak(lvlMode, battleMode)) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut record: *mut u32 =
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .cast::<u32>();
                    let mut recordHasData: u32 = 0u32;
                    let mut checksum: u32 = 0u32;
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(232u32, 4u32)) {
                                break 'l3;
                            }
                            'l4: {
                                recordHasData = (recordHasData
                                    | ((record).wrapping_offset((j) as isize)).read());
                                checksum = (checksum)
                                    .wrapping_add(((record).wrapping_offset((j) as isize)).read());
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    validMons = 0i32;
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j
                                < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                                    3i32
                                } else {
                                    (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                                }))
                            {
                                break 'l5;
                            }
                            'l6: {
                                if ((((((((((((((&raw mut gSaveBlock2Ptr)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 236))
                                .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 44))
                                .cast::<u16>())
                                .read()) as i32)
                                    != 0i32)
                                    && ((((((((((((((&raw mut gSaveBlock2Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 236))
                                    .wrapping_add(52))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 44))
                                    .wrapping_add(12))
                                    .read()) as i32)
                                        <= ((GetFrontierEnemyMonLevel(lvlMode)) as i32))
                                {
                                    validMons = (validMons).wrapping_add(1);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((((validMons
                        >= ((((((&raw const sBattleTowerPartySizes2).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((battleMode) as i32) as isize))
                        .read()) as i32))
                        && (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            == winStreak))
                        && ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .read()) as i32)
                            == ((lvlMode) as i32)))
                        && ((recordHasData) != 0))
                        && (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .wrapping_add(232)
                        .cast::<u32>())
                        .read()
                            == checksum)
                    {
                        (((&raw mut trainerIds).cast::<i32>())
                            .wrapping_offset((idsCount) as isize))
                        .write((i).wrapping_add(300i32));
                        idsCount = (idsCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((battleMode) as i32) == 0i32 {
            ValidateApprenticesChecksums();
            {
                i = 0i32;
                'l7: loop {
                    if !(i < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        if ((((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 68))
                            .wrapping_add(0),
                            5,
                            2,
                            false,
                        ) as u8) as i32)
                            != 0i32)
                            && (((((((&raw const sApprenticeChallengeThreshold)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 68))
                                .wrapping_add(1))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                == winStreak))
                            && (((crate::c::bf_read(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 68))
                                .wrapping_add(0),
                                5,
                                2,
                                false,
                            ) as u8) as i32)
                                .wrapping_sub(1i32)
                                == ((lvlMode) as i32))
                        {
                            (((&raw mut trainerIds).cast::<i32>())
                                .wrapping_offset((idsCount) as isize))
                            .write((i).wrapping_add(400i32));
                            idsCount = (idsCount).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if idsCount != 0i32 {
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(
                (((((&raw mut trainerIds).cast::<i32>())
                    .wrapping_offset((crate::c::rem_i32(((Random()) as i32), idsCount)) as isize))
                .read()) as u16),
            );
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SetNextFacilityOpponent() {
    unsafe {
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        if lvlMode == 2u32 {
            SetNextBattleTentOpponent();
        } else {
            let mut id: u16 = 0u16;
            let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
            let mut winStreak: u16 = ((GetCurrentFacilityWinStreak()) as u16);
            let mut challengeNum: u32 = ((crate::c::div_i32(((winStreak) as i32), 7i32)) as u32);
            SetFacilityPtrsGetLevel();
            if (battleMode == 2u32) || (battleMode == 3u32) {
                id = (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read();
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset((((id) as i32).wrapping_mul(2i32)) as isize))
                    .read(),
                );
                ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((id) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as isize,
                    ))
                    .read(),
                );
                SetBattleFacilityTrainerGfxId(
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                    0u8,
                );
                SetBattleFacilityTrainerGfxId(
                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                    1u8,
                );
            } else {
                if (ChooseSpecialBattleTowerTrainer()) != 0 {
                    SetBattleFacilityTrainerGfxId(
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                        0u8,
                    );
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                        .read()) as i32) as isize,
                    ))
                    .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
                } else {
                    let mut i: i32 = 0i32;
                    'l1: loop {
                        if !((1i32) != 0) {
                            break 'l1;
                        }
                        id = GetRandomScaledFrontierTrainerId(
                            ((challengeNum) as u8),
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as u8),
                        );
                        {
                            i = 0i32;
                            'l2: loop {
                                if !(i
                                    < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1638)
                                    .cast::<u16>())
                                    .read()) as i32))
                                {
                                    break 'l2;
                                }
                                'l3: {
                                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1640))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        == ((id) as i32)
                                    {
                                        break 'l2;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if i == (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                        .read()) as i32)
                        {
                            break 'l1;
                        }
                    }
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(id);
                    SetBattleFacilityTrainerGfxId(
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                        0u8,
                    );
                    if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(1i32)
                        < 7i32
                    {
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32) as isize,
                        ))
                        .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomScaledFrontierTrainerId(challengeNum: u8, battleNum: u8) -> u16 {
    unsafe {
        let mut challengeNum = challengeNum;
        let mut battleNum = battleNum;
        let mut trainerId: u16 = 0u16;
        if ((challengeNum) as i32) <= 7i32 {
            if ((battleNum) as i32) == 6i32 {
                trainerId = (((((((((((&raw const sFrontierTrainerIdRangesHard)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw const sFrontierTrainerIdRangesHard)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((challengeNum) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                .wrapping_add(1i32)) as u16);
                trainerId = (((((((((&raw const sFrontierTrainerIdRangesHard)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(crate::c::rem_i32(((Random()) as i32), ((trainerId) as i32))))
                    as u16);
            } else {
                trainerId = (((((((((((&raw const sFrontierTrainerIdRanges)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw const sFrontierTrainerIdRanges)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((challengeNum) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                .wrapping_add(1i32)) as u16);
                trainerId = (((((((((&raw const sFrontierTrainerIdRanges)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(crate::c::rem_i32(((Random()) as i32), ((trainerId) as i32))))
                    as u16);
            }
        } else {
            trainerId = (((((((((((&raw const sFrontierTrainerIdRanges)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(28))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_sub(
                    (((((((&raw const sFrontierTrainerIdRanges)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(28))
                    .cast::<u16>())
                    .read()) as i32),
                ))
            .wrapping_add(1i32)) as u16);
            trainerId = (((((((((&raw const sFrontierTrainerIdRanges)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(28))
            .cast::<u16>())
            .read()) as i32)
                .wrapping_add(crate::c::rem_i32(((Random()) as i32), ((trainerId) as i32))))
                as u16);
        }
        return trainerId;
    }
}
pub(crate) unsafe extern "C" fn GetRandomScaledFrontierTrainerIdRange(
    challengeNum: u8,
    battleNum: u8,
    trainerIdPtr: *mut u16,
    rangePtr: *mut u8,
) {
    unsafe {
        let mut challengeNum = challengeNum;
        let mut battleNum = battleNum;
        let mut trainerIdPtr = trainerIdPtr;
        let mut rangePtr = rangePtr;
        let mut trainerId: u16 = 0u16;
        let mut range: u16 = 0u16;
        if ((challengeNum) as i32) <= 7i32 {
            if ((battleNum) as i32) == 6i32 {
                range = (((((((((((&raw const sFrontierTrainerIdRangesHard)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw const sFrontierTrainerIdRangesHard)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((challengeNum) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                .wrapping_add(1i32)) as u16);
                trainerId = (((((&raw const sFrontierTrainerIdRangesHard)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .read();
            } else {
                range = (((((((((((&raw const sFrontierTrainerIdRanges)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_sub(
                        (((((((&raw const sFrontierTrainerIdRanges)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((challengeNum) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32),
                    ))
                .wrapping_add(1i32)) as u16);
                trainerId = (((((&raw const sFrontierTrainerIdRanges)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((challengeNum) as i32) as isize * 4))
                .cast::<u16>())
                .read();
            }
        } else {
            range = (((((((((((&raw const sFrontierTrainerIdRanges)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(28))
            .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_sub(
                    (((((((&raw const sFrontierTrainerIdRanges)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(28))
                    .cast::<u16>())
                    .read()) as i32),
                ))
            .wrapping_add(1i32)) as u16);
            trainerId = (((((&raw const sFrontierTrainerIdRanges)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(28))
            .cast::<u16>())
            .read();
        }
        (trainerIdPtr).write(trainerId);
        (rangePtr).write(((range) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleFacilityTrainerGfxId(trainerId: u16, tempVarId: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut tempVarId = tempVarId;
        let mut i: u32 = 0u32;
        let mut facilityClass: u8 = 0u8;
        let mut trainerObjectGfxId: u8 = 0u8;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            facilityClass = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1440))
            .wrapping_add(1))
            .read();
        } else {
            if ((trainerId) as i32) == 1022i32 {
                SetFrontierBrainObjEventGfx_2();
                return;
            } else {
                if ((trainerId) as i32) < 300i32 {
                    facilityClass =
                        ((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((trainerId) as i32) as isize * 52))
                        .read();
                } else {
                    if ((trainerId) as i32) < 400i32 {
                        facilityClass = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(1))
                        .read();
                    } else {
                        facilityClass = ((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                            ((crate::c::bf_read(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                                ))
                                .wrapping_add(0),
                                0,
                                5,
                                false,
                            ) as u8) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(50))
                        .read();
                    }
                }
            }
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(30u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const gTowerMaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != crate::c::div_u32(30u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerMaleTrainerGfxIds).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            'l3: {
                let __sw1 = ((tempVarId) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 15i32;
                if __sw1 == 0i32 || !__matched {
                    VarSet(16400u16, ((trainerObjectGfxId) as u16));
                    return;
                }
                if __sw1 == 1i32 {
                    VarSet(16401u16, ((trainerObjectGfxId) as u16));
                    return;
                }
                if __sw1 == 15i32 {
                    VarSet(16414u16, ((trainerObjectGfxId) as u16));
                    return;
                }
            }
        }
        {
            i = 0u32;
            'l4: loop {
                if !(i < crate::c::div_u32(20u32, 1u32)) {
                    break 'l4;
                }
                'l5: {
                    if ((((((&raw const gTowerFemaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l4;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != crate::c::div_u32(20u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerFemaleTrainerGfxIds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            'l6: {
                let __sw2 = ((tempVarId) as i32);
                let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 15i32;
                if __sw2 == 0i32 || !__matched {
                    VarSet(16400u16, ((trainerObjectGfxId) as u16));
                    return;
                }
                if __sw2 == 1i32 {
                    VarSet(16401u16, ((trainerObjectGfxId) as u16));
                    return;
                }
                if __sw2 == 15i32 {
                    VarSet(16414u16, ((trainerObjectGfxId) as u16));
                    return;
                }
            }
        }
        'l7: {
            let __sw3 = ((tempVarId) as i32);
            let __matched = __sw3 == 0i32 || __sw3 == 1i32 || __sw3 == 15i32;
            if __sw3 == 0i32 || !__matched {
                VarSet(16400u16, 7u16);
                return;
            }
            if __sw3 == 1i32 {
                VarSet(16401u16, 7u16);
                return;
            }
            if __sw3 == 15i32 {
                VarSet(16414u16, 7u16);
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetEReaderTrainerGfxId() {
    unsafe {
        SetBattleFacilityTrainerGfxId(500u16, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleFacilityTrainerGfxId(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: u32 = 0u32;
        let mut facilityClass: u8 = 0u8;
        let mut trainerObjectGfxId: u8 = 0u8;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            facilityClass = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1440))
            .wrapping_add(1))
            .read();
        } else {
            if ((trainerId) as i32) < 300i32 {
                facilityClass = ((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
                .read();
            } else {
                if ((trainerId) as i32) < 400i32 {
                    facilityClass = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset((((trainerId) as i32).wrapping_sub(300i32)) as isize * 236))
                    .wrapping_add(1))
                    .read();
                } else {
                    facilityClass = ((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                        ((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                            ))
                            .wrapping_add(0),
                            0,
                            5,
                            false,
                        ) as u8) as i32) as isize
                            * 88,
                    ))
                    .wrapping_add(50))
                    .read();
                }
            }
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(30u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const gTowerMaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != crate::c::div_u32(30u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerMaleTrainerGfxIds).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            return trainerObjectGfxId;
        }
        {
            i = 0u32;
            'l3: loop {
                if !(i < crate::c::div_u32(20u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw const gTowerFemaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l3;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != crate::c::div_u32(20u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerFemaleTrainerGfxIds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            return trainerObjectGfxId;
        } else {
            return 7u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutNewBattleTowerRecord(newRecordEm: *mut u8) {
    unsafe {
        let mut newRecordEm = newRecordEm;
        let mut slotValues = crate::ffi::Align4([0u8; 12]);
        let mut slotIds = crate::ffi::Align4([0u8; 12]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut slotsCount: i32 = 0i32;
        let mut newRecord: *mut u8 = newRecordEm;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    k = 0i32;
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 236))
                                .wrapping_add(12))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    != ((((((newRecord).wrapping_add(12)).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if j == 4i32 {
                        {
                            k = 0i32;
                            'l5: loop {
                                if !(k < 7i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((((((((((((&raw mut gSaveBlock2Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 236))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        != ((((((newRecord).wrapping_add(4)).cast::<u8>())
                                            .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l5;
                                    }
                                    if ((((((newRecord).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        == 255i32
                                    {
                                        k = 7i32;
                                        break 'l5;
                                    }
                                }
                                k = (k).wrapping_add(1);
                            }
                        }
                    }
                    if k == 7i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i < 5i32 {
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(236))
            .cast::<u8>())
            .wrapping_offset((i) as isize * 236)
            .cast::<crate::c::Rec4<236>>()
            .write_unaligned(newRecord.cast::<crate::c::Rec4<236>>().read_unaligned());
            return;
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 5i32) {
                    break 'l7;
                }
                'l8: {
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 236))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l7;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i < 5i32 {
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(236))
            .cast::<u8>())
            .wrapping_offset((i) as isize * 236)
            .cast::<crate::c::Rec4<236>>()
            .write_unaligned(newRecord.cast::<crate::c::Rec4<236>>().read_unaligned());
            return;
        }
        ((&raw mut slotValues).cast::<u16>()).write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(236))
            .cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        ((&raw mut slotIds).cast::<u16>()).write(0u16);
        slotsCount = (slotsCount).wrapping_add(1);
        {
            i = 1i32;
            'l9: loop {
                if !(i < 5i32) {
                    break 'l9;
                }
                'l10: {
                    {
                        j = 0i32;
                        'l11: loop {
                            if !(j < slotsCount) {
                                break 'l11;
                            }
                            'l12: {
                                if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(236))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 236))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32)
                                    < (((((&raw mut slotValues).cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                {
                                    j = 0i32;
                                    slotsCount = 1i32;
                                    ((&raw mut slotValues).cast::<u16>()).write(
                                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 236))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .read(),
                                    );
                                    ((&raw mut slotIds).cast::<u16>()).write(((i) as u16));
                                    break 'l11;
                                } else {
                                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 236))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        > (((((&raw mut slotValues).cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                    {
                                        break 'l11;
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if j == slotsCount {
                        (((&raw mut slotValues).cast::<u16>())
                            .wrapping_offset((slotsCount) as isize))
                        .write(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(236))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 236))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        );
                        (((&raw mut slotIds).cast::<u16>()).wrapping_offset((slotsCount) as isize))
                            .write(((i) as u16));
                        slotsCount = (slotsCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        i = crate::c::rem_i32(((Random()) as i32), slotsCount);
        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(236))
        .cast::<u8>())
        .wrapping_offset(
            (((((&raw mut slotIds).cast::<u16>()).wrapping_offset((i) as isize)).read()) as i32)
                as isize
                * 236,
        )
        .cast::<crate::c::Rec4<236>>()
        .write_unaligned(newRecord.cast::<crate::c::Rec4<236>>().read_unaligned());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierTrainerFrontSpriteId(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            return (((&raw mut gFacilityClassToPicIndex).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1440))
                .wrapping_add(1))
                .read()) as i32) as isize,
            ))
            .read();
        } else {
            if ((trainerId) as i32) == 1022i32 {
                return GetFrontierBrainTrainerPicIndex();
            } else {
                if ((trainerId) as i32) < 300i32 {
                    return (((&raw mut gFacilityClassToPicIndex).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((trainerId) as i32) as isize * 52))
                        .read()) as i32) as isize,
                    ))
                    .read();
                } else {
                    if ((trainerId) as i32) < 400i32 {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                            return (((&raw mut gFacilityClassToPicIndex).cast::<u8>())
                                .wrapping_offset(
                                    ((GetRecordedBattleRecordMixFriendClass()) as i32) as isize,
                                ))
                            .read();
                        } else {
                            return (((&raw mut gFacilityClassToPicIndex).cast::<u8>())
                                .wrapping_offset(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                                    ))
                                    .wrapping_add(1))
                                    .read()) as i32) as isize,
                                ))
                            .read();
                        }
                    } else {
                        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                            return (((&raw mut gFacilityClassToPicIndex).cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                                        ((GetRecordedBattleApprenticeId()) as i32) as isize * 88,
                                    ))
                                    .wrapping_add(50))
                                    .read()) as i32) as isize,
                                ))
                            .read();
                        } else {
                            return (((&raw mut gFacilityClassToPicIndex).cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                                        ((crate::c::bf_read(
                                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(220))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((trainerId) as i32).wrapping_sub(400i32))
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(0),
                                            0,
                                            5,
                                            false,
                                        ) as u8) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(50))
                                    .read()) as i32) as isize,
                                ))
                            .read();
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierOpponentClass(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut trainerClass: u8 = 0u8;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            trainerClass = (((&raw mut gFacilityClassToTrainerClass).cast::<u8>())
                .wrapping_offset(
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1440))
                    .wrapping_add(1))
                    .read()) as i32) as isize,
                ))
            .read();
        } else {
            if ((trainerId) as i32) == 1022i32 {
                return GetFrontierBrainTrainerClass();
            } else {
                if ((trainerId) as i32) == 3075i32 {
                    trainerClass = ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32160))
                        .wrapping_add(1))
                    .read();
                } else {
                    if ((trainerId) as i32) < 300i32 {
                        trainerClass = (((&raw mut gFacilityClassToTrainerClass).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_offset(((trainerId) as i32) as isize * 52))
                                .read()) as i32) as isize,
                            ))
                        .read();
                    } else {
                        if ((trainerId) as i32) < 400i32 {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32)
                                != 0
                            {
                                trainerClass = (((&raw mut gFacilityClassToTrainerClass)
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((GetRecordedBattleRecordMixFriendClass()) as i32) as isize,
                                ))
                                .read();
                            } else {
                                trainerClass = (((&raw mut gFacilityClassToTrainerClass)
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                                    ))
                                    .wrapping_add(1))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                            }
                        } else {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32)
                                != 0
                            {
                                trainerClass = (((&raw mut gFacilityClassToTrainerClass)
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                                        ((GetRecordedBattleApprenticeId()) as i32) as isize * 88,
                                    ))
                                    .wrapping_add(50))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                            } else {
                                trainerClass = (((&raw mut gFacilityClassToTrainerClass)
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                                        ((crate::c::bf_read(
                                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(220))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((trainerId) as i32).wrapping_sub(400i32))
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(0),
                                            0,
                                            5,
                                            false,
                                        ) as u8) as i32)
                                            as isize
                                            * 88,
                                    ))
                                    .wrapping_add(50))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                            }
                        }
                    }
                }
            }
        }
        return trainerClass;
    }
}
pub(crate) unsafe extern "C" fn GetFrontierTrainerFacilityClass(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut facilityClass: u8 = 0u8;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            facilityClass = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1440))
            .wrapping_add(1))
            .read();
        } else {
            if ((trainerId) as i32) < 300i32 {
                facilityClass = ((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
                .read();
            } else {
                if ((trainerId) as i32) < 400i32 {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                        facilityClass = GetRecordedBattleRecordMixFriendClass();
                    } else {
                        facilityClass = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(1))
                        .read();
                    }
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                        facilityClass = ((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                            ((GetRecordedBattleApprenticeId()) as i32) as isize * 88,
                        ))
                        .wrapping_add(50))
                        .read();
                    } else {
                        facilityClass = ((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                            ((crate::c::bf_read(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                                ))
                                .wrapping_add(0),
                                0,
                                5,
                                false,
                            ) as u8) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(50))
                        .read();
                    }
                }
            }
        }
        return facilityClass;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierTrainerName(dst: *mut u8, trainerId: u16) {
    unsafe {
        let mut dst = dst;
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((dst).wrapping_offset((i) as isize)).write(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1440))
                            .wrapping_add(4))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            if ((trainerId) as i32) == 1022i32 {
                CopyFrontierBrainTrainerName(dst);
                return;
            } else {
                if ((trainerId) as i32) == 3075i32 {
                    {
                        i = 0i32;
                        'l3: loop {
                            if !(i < 7i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((dst).wrapping_offset((i) as isize)).write(
                                    ((((((&raw mut gTrainers).cast::<u8>())
                                        .wrapping_offset(32160))
                                    .wrapping_add(4))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    if ((trainerId) as i32) < 300i32 {
                        {
                            i = 0i32;
                            'l5: loop {
                                if !(i < 7i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((dst).wrapping_offset((i) as isize)).write(
                                        (((((((&raw mut gFacilityTrainers)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((trainerId) as i32) as isize * 52))
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    } else {
                        if ((trainerId) as i32) < 400i32 {
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32)
                                != 0
                            {
                                GetRecordedBattleRecordMixFriendName(dst);
                                return;
                            } else {
                                let mut record: *mut u8 =
                                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                                    );
                                TVShowConvertInternationalString(
                                    dst,
                                    ((record).wrapping_add(4)).cast::<u8>(),
                                    ((((record).wrapping_add(228)).read()) as i32),
                                );
                                return;
                            }
                        } else {
                            let mut id: u8 = 0u8;
                            let mut language: u8 = 0u8;
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32)
                                != 0
                            {
                                id = GetRecordedBattleApprenticeId();
                                language = GetRecordedBattleApprenticeLanguage();
                            } else {
                                let mut apprentice: *mut u8 =
                                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(220))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                                    );
                                id = (crate::c::bf_read((apprentice).wrapping_add(0), 0, 5, false)
                                    as u8);
                                language = ((apprentice).wrapping_add(63)).read();
                            }
                            TVShowConvertInternationalString(
                                dst,
                                GetApprenticeNameInLanguage(((id) as u32), ((language) as i32)),
                                ((language) as i32),
                            );
                            return;
                        }
                    }
                }
            }
        }
        ((dst).wrapping_offset((i) as isize)).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn IsFrontierTrainerFemale(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: u32 = 0u32;
        let mut facilityClass: u8 = 0u8;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 500i32 {
            facilityClass = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1440))
            .wrapping_add(1))
            .read();
        } else {
            if ((trainerId) as i32) == 1022i32 {
                return IsFrontierBrainFemale();
            } else {
                if ((trainerId) as i32) < 300i32 {
                    facilityClass =
                        ((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((trainerId) as i32) as isize * 52))
                        .read();
                } else {
                    if ((trainerId) as i32) < 400i32 {
                        facilityClass = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(1))
                        .read();
                    } else {
                        facilityClass = ((((&raw mut gApprentices).cast::<u8>()).wrapping_offset(
                            ((crate::c::bf_read(
                                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                                ))
                                .wrapping_add(0),
                                0,
                                5,
                                false,
                            ) as u8) as i32) as isize
                                * 88,
                        ))
                        .wrapping_add(50))
                        .read();
                    }
                }
            }
        }
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(20u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const gTowerFemaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if i != crate::c::div_u32(20u32, 1u32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillFrontierTrainerParty(monsCount: u8) {
    unsafe {
        let mut monsCount = monsCount;
        ZeroEnemyPartyMons();
        FillTrainerParty(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
            monsCount,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillFrontierTrainersParties(monsCount: u8) {
    unsafe {
        let mut monsCount = monsCount;
        ZeroEnemyPartyMons();
        FillTrainerParty(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
            monsCount,
        );
        FillTrainerParty(
            ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
            3u8,
            monsCount,
        );
    }
}
pub(crate) unsafe extern "C" fn FillTentTrainerParty(monsCount: u8) {
    unsafe {
        let mut monsCount = monsCount;
        ZeroEnemyPartyMons();
        FillTentTrainerParty_(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
            monsCount,
        );
    }
}
pub(crate) unsafe extern "C" fn FillTrainerParty(trainerId: u16, firstMonId: u8, monCount: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut monCount = monCount;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut chosenMonIndices = crate::ffi::Align4([0u8; 8]);
        let mut friendship: u8 = 255u8;
        let mut level: u8 = SetFacilityPtrsGetLevel();
        let mut fixedIV: u8 = 0u8;
        let mut bfMonCount: u8 = 0u8;
        let mut monSet: *mut u16 = core::ptr::null_mut();
        let mut otID: u32 = 0u32;
        if ((trainerId) as i32) < 300i32 {
            fixedIV = GetFrontierTrainerFixedIvs(trainerId);
            monSet = (((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(
                    ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize
                        * 52,
                ))
            .wrapping_add(48)
            .cast::<*mut u16>())
            .read();
        } else {
            if ((trainerId) as i32) == 500i32 {
                {
                    i = ((firstMonId) as i32);
                    'l1: loop {
                        if !(i < ((firstMonId) as i32).wrapping_add(3i32)) {
                            break 'l1;
                        }
                        'l2: {
                            CreateBattleTowerMon(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1440))
                                .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((i).wrapping_sub(((firstMonId) as i32))) as isize * 44,
                                ),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return;
            } else {
                if ((trainerId) as i32) == 1022i32 {
                    CreateFrontierBrainPokemon();
                    return;
                } else {
                    if ((trainerId) as i32) < 400i32 {
                        {
                            j = 0i32;
                            i = ((firstMonId) as i32);
                            'l3: loop {
                                if !(i < ((firstMonId) as i32).wrapping_add(((monCount) as i32))) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((((((((&raw mut gSaveBlock2Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                                    ))
                                    .wrapping_add(52))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 44))
                                    .cast::<u16>())
                                    .read()) as i32)
                                        != 0i32)
                                        && ((((((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(300i32)) as isize
                                                * 236,
                                        ))
                                        .wrapping_add(52))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 44))
                                        .wrapping_add(12))
                                        .read())
                                            as i32)
                                            <= ((level) as i32))
                                    {
                                        CreateBattleTowerMon_HandleLevel(
                                            ((&raw mut gEnemyParty).cast::<u8>())
                                                .wrapping_offset((i) as isize * 100),
                                            (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(236))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                (((trainerId) as i32).wrapping_sub(300i32))
                                                    as isize
                                                    * 236,
                                            ))
                                            .wrapping_add(52))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 44),
                                            0u8,
                                        );
                                    }
                                }
                                j = (j).wrapping_add(1);
                                i = (i).wrapping_add(1);
                            }
                        }
                        return;
                    } else {
                        {
                            i = ((firstMonId) as i32);
                            'l5: loop {
                                if !(i < ((firstMonId) as i32).wrapping_add(3i32)) {
                                    break 'l5;
                                }
                                'l6: {
                                    CreateApprenticeMon(
                                        ((&raw mut gEnemyParty).cast::<u8>())
                                            .wrapping_offset((i) as isize * 100),
                                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(400i32)) as isize
                                                * 68,
                                        ),
                                        (((i).wrapping_sub(((firstMonId) as i32))) as u8),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        return;
                    }
                }
            }
        }
        {
            bfMonCount = 0u8;
            'l7: loop {
                if !(((((monSet).wrapping_offset(((bfMonCount) as i32) as isize)).read()) as i32)
                    != 65535i32)
                {
                    break 'l7;
                }
                'l8: {}
                bfMonCount = (bfMonCount).wrapping_add(1);
            }
        }
        i = 0i32;
        otID = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
        'l9: loop {
            if !(i != ((monCount) as i32)) {
                break 'l9;
            }
            let mut monId: u16 = ((monSet).wrapping_offset(
                (crate::c::rem_i32(((Random()) as i32), ((bfMonCount) as i32))) as isize,
            ))
            .read();
            if ((((level) as i32) == 50i32) || (((level) as i32) == 20i32))
                && (((monId) as i32) > 849i32)
            {
                continue 'l9;
            }
            {
                j = 0i32;
                'l10: loop {
                    if !(j < (i).wrapping_add(((firstMonId) as i32))) {
                        break 'l10;
                    }
                    'l11: {
                        if GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((j) as isize * 100),
                            11i32,
                            core::ptr::null_mut(),
                        ) == (((((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read()) as u32)
                        {
                            break 'l10;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l9;
            }
            {
                j = 0i32;
                'l12: loop {
                    if !(j < (i).wrapping_add(((firstMonId) as i32))) {
                        break 'l12;
                    }
                    'l13: {
                        if (GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((j) as isize * 100),
                            12i32,
                            core::ptr::null_mut(),
                        ) != 0u32)
                            && (GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((j) as isize * 100),
                                12i32,
                                core::ptr::null_mut(),
                            ) == ((((((&raw const gBattleFrontierHeldItems)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                            .read()) as u32))
                        {
                            break 'l12;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l9;
            }
            {
                j = 0i32;
                'l14: loop {
                    if !(j < i) {
                        break 'l14;
                    }
                    'l15: {
                        if (((((&raw mut chosenMonIndices).cast::<u16>())
                            .wrapping_offset((j) as isize))
                        .read()) as i32)
                            == ((monId) as i32)
                        {
                            break 'l14;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != i {
                continue 'l9;
            }
            (((&raw mut chosenMonIndices).cast::<u16>()).wrapping_offset((i) as isize))
                .write(monId);
            CreateMonWithEVSpreadNatureOTID(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
                level,
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(12))
                .read(),
                fixedIV,
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(11))
                .read(),
                otID,
            );
            friendship = 255u8;
            {
                j = 0i32;
                'l16: loop {
                    if !(j < 4i32) {
                        break 'l16;
                    }
                    'l17: {
                        SetMonMoveSlot(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((i).wrapping_add(((firstMonId) as i32))) as isize * 100,
                            ),
                            (((((((&raw mut gFacilityTrainerMons)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(2))
                            .cast::<u16>())
                            .wrapping_offset((j) as isize))
                            .read(),
                            ((j) as u8),
                        );
                        if (((((((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset((j) as isize))
                        .read()) as i32)
                            == 218i32
                        {
                            friendship = 0u8;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                32i32,
                &raw mut friendship,
            );
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                12i32,
                ((((&raw const gBattleFrontierHeldItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((((&raw mut gFacilityTrainerMons)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                    .wrapping_add(10))
                    .read()) as i32) as isize,
                ))
                .cast::<u8>(),
            );
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn Unused_CreateApprenticeMons(trainerId: u16, firstMonId: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut friendship: u8 = 255u8;
        let mut level: u8 = 0u8;
        let mut fixedIV: u8 = 0u8;
        let mut apprentice: *mut u8 =
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220)).cast::<u8>();
        if ((((apprentice).wrapping_add(1)).read()) as i32) < 5i32 {
            fixedIV = 6u8;
        } else {
            fixedIV = 9u8;
        }
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 0i32
        {
            level = 100u8;
        } else {
            level = 50u8;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i != 3i32) {
                    break 'l1;
                }
                'l2: {
                    CreateMonWithEVSpread(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(i)) as isize * 100,
                        ),
                        (((((apprentice).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .cast::<u16>())
                        .read(),
                        level,
                        fixedIV,
                        8u8,
                    );
                    friendship = 255u8;
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((apprentice).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 12))
                                .wrapping_add(2))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == 218i32
                                {
                                    friendship = 0u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(i)) as isize * 100,
                        ),
                        32i32,
                        &raw mut friendship,
                    );
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(i)) as isize * 100,
                        ),
                        12i32,
                        (((((apprentice).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .wrapping_add(10)
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomFrontierMonFromSet(trainerId: u16) -> u16 {
    unsafe {
        let mut trainerId = trainerId;
        let mut level: u8 = SetFacilityPtrsGetLevel();
        let mut monSet: *mut u16 =
            (((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((trainerId) as i32) as isize * 52))
            .wrapping_add(48)
            .cast::<*mut u16>())
            .read();
        let mut numMons: u8 = 0u8;
        let mut monId: u32 =
            ((((monSet).wrapping_offset(((numMons) as i32) as isize)).read()) as u32);
        'l1: loop {
            if !(monId != 65535u32) {
                break 'l1;
            }
            numMons = (numMons).wrapping_add(1);
            monId = ((((monSet).wrapping_offset(((numMons) as i32) as isize)).read()) as u32);
            if monId == 65535u32 {
                break 'l1;
            }
        }
        'l2: loop {
            'l3: {
                monId = ((((monSet).wrapping_offset(
                    (crate::c::rem_i32(((Random()) as i32), ((numMons) as i32))) as isize,
                ))
                .read()) as u32);
            }
            if !(((((level) as i32) == 50i32) || (((level) as i32) == 20i32)) && (monId > 849u32)) {
                break 'l2;
            }
        }
        return ((monId) as u16);
    }
}
pub(crate) unsafe extern "C" fn FillFactoryTrainerParty() {
    unsafe {
        ZeroEnemyPartyMons();
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            != 2i32
        {
            FillFactoryFrontierTrainerParty(
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                0u8,
            );
        } else {
            FillFactoryTentTrainerParty(
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                0u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FillFactoryFrontierTrainerParty(trainerId: u16, firstMonId: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut friendship: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut fixedIV: u8 = 0u8;
        let mut otID: u32 = 0u32;
        if ((trainerId) as i32) < 300i32 {
            let mut lvlMode: u8 = (crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8);
            let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
            let mut challengeNum: u8 = ((crate::c::div_i32(
                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32),
                7i32,
            )) as u8);
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1638)
                .cast::<u16>())
            .read()) as i32)
                < 6i32
            {
                fixedIV = GetFactoryMonFixedIV(challengeNum, 0u8);
            } else {
                fixedIV = GetFactoryMonFixedIV(challengeNum, 1u8);
            }
        } else {
            if ((trainerId) as i32) == 500i32 {
                {
                    i = firstMonId;
                    'l1: loop {
                        if !(((i) as i32) < ((firstMonId) as i32).wrapping_add(3i32)) {
                            break 'l1;
                        }
                        'l2: {
                            CreateBattleTowerMon(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1440))
                                .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((i) as i32).wrapping_sub(((firstMonId) as i32))) as isize
                                        * 44,
                                ),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return;
            } else {
                if ((trainerId) as i32) == 1022i32 {
                    FillFactoryBrainParty();
                    return;
                } else {
                    fixedIV = 31u8;
                }
            }
        }
        level = SetFacilityPtrsGetLevel();
        otID = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut monId: u16 =
                        ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    CreateMonWithEVSpreadNatureOTID(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                        level,
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(12))
                        .read(),
                        fixedIV,
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read(),
                        otID,
                    );
                    friendship = 0u8;
                    {
                        j = 0u8;
                        'l5: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                SetMonMoveAvoidReturn(
                                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize
                                            * 100,
                                    ),
                                    (((((((&raw mut gFacilityTrainerMons)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                    j,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        32i32,
                        &raw mut friendship,
                    );
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        12i32,
                        ((((&raw const gBattleFrontierHeldItems)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut gFacilityTrainerMons)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(10))
                            .read()) as i32) as isize,
                        ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillFactoryTentTrainerParty(trainerId: u16, firstMonId: u8) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut friendship: u8 = 0u8;
        let mut level: u8 = 30u8;
        let mut fixedIV: u8 = 0u8;
        let mut otID: u32 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10))
        .cast::<u8>())
        .read()) as i32)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32)
                << 24)) as u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut monId: u16 =
                        ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read();
                    CreateMonWithEVSpreadNatureOTID(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read(),
                        level,
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(12))
                        .read(),
                        fixedIV,
                        (((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read(),
                        otID,
                    );
                    friendship = 0u8;
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                SetMonMoveAvoidReturn(
                                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize
                                            * 100,
                                    ),
                                    (((((((&raw mut gFacilityTrainerMons)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                    j,
                                );
                                if (((((((((&raw mut gFacilityTrainerMons)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                                .wrapping_add(2))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .read()) as i32)
                                    == 218i32
                                {
                                    friendship = 0u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        32i32,
                        &raw mut friendship,
                    );
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (((firstMonId) as i32).wrapping_add(((i) as i32))) as isize * 100,
                        ),
                        12i32,
                        ((((&raw const gBattleFrontierHeldItems)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            (((((((&raw mut gFacilityTrainerMons)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(10))
                            .read()) as i32) as isize,
                        ))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FrontierSpeechToString(words: *mut u16) {
    unsafe {
        let mut words = words;
        ConvertEasyChatWordsToString((&raw mut gStringVar4).cast::<u8>(), words, 3u16, 2u16);
        if ((GetStringWidth(1u8, (&raw mut gStringVar4).cast::<u8>(), (-1i16))) as u32) > 204u32 {
            let mut i: i32 = 0i32;
            ConvertEasyChatWordsToString((&raw mut gStringVar4).cast::<u8>(), words, 2u16, 3u16);
            'l1: loop {
                if !((((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                    ({
                        let __t1 = i;
                        i = (i).wrapping_add(1);
                        __t1
                    }) as isize,
                ))
                .read()) as i32)
                    != 254i32)
                {
                    break 'l1;
                }
            }
            'l2: loop {
                if !((((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset((i) as isize)).read())
                    as i32)
                    != 254i32)
                {
                    break 'l2;
                }
                i = (i).wrapping_add(1);
            }
            (((&raw mut gStringVar4).cast::<u8>()).wrapping_offset((i) as isize)).write(250u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetOpponentIntroSpeech() {
    unsafe {
        let mut trainerId: u16 = 0u16;
        SetFacilityPtrsGetLevel();
        if (((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) != 0 {
            trainerId = ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read();
        } else {
            trainerId = ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read();
        }
        if ((trainerId) as i32) == 500i32 {
            FrontierSpeechToString(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1440))
                .wrapping_add(16))
                .cast::<u16>(),
            );
        } else {
            if ((trainerId) as i32) < 300i32 {
                FrontierSpeechToString(
                    (((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((trainerId) as i32) as isize * 52))
                    .wrapping_add(12))
                    .cast::<u16>(),
                );
            } else {
                if ((trainerId) as i32) < 400i32 {
                    FrontierSpeechToString(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(16))
                        .cast::<u16>(),
                    );
                } else {
                    BufferApprenticeChallengeText(
                        ((((trainerId) as i32).wrapping_sub(400i32)) as u8),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleSpecialTrainerBattleEnd() {
    unsafe {
        let mut i: i32 = 0i32;
        RecordedBattle_SaveBattleOutcome();
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(38)).read()) as i32);
            if __sw1 == 0i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 9i32
                || __sw1 == 10i32
            {
                if (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2160)
                    .cast::<u32>())
                .read()
                    < 16777215u32
                {
                    let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2160)
                    .cast::<u32>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    if crate::c::rem_u32(
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2160)
                        .cast::<u32>())
                        .read(),
                        20u32,
                    ) == 0u32
                    {
                        UpdateGymLeaderRematch();
                    }
                } else {
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(2160)
                        .cast::<u32>())
                    .write(16777215u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut itemBefore: u16 = ((GetMonData2(
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(568))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                                12i32,
                            )) as u16);
                            SetMonData(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                12i32,
                                (&raw mut itemBefore).cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                CopyEReaderTrainerFarewellMessage();
                break 'l1;
            }
        }
        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
pub(crate) unsafe extern "C" fn Task_StartBattleAfterTransition(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsBattleTransitionDone()) as i32) == 1i32 {
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(HandleSpecialTrainerBattleEnd));
            SetMainCallback2(Some(CB2_InitBattle));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSpecialTrainerBattle() {
    unsafe {
        let mut i: i32 = 0i32;
        (((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(38))
            .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(264u32);
                'l2: {
                    let __sw2 = ((VarGet(16590u16)) as i32);
                    if __sw2 == 0i32 {
                        FillFrontierTrainerParty(3u8);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        FillFrontierTrainerParty(4u8);
                        let __p3 = (&raw mut gBattleTypeFlags).cast::<u32>();
                        (__p3).write(((__p3).read() | 1u32));
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        FillFrontierTrainersParties(2u8);
                        ((&raw mut gPartnerTrainerId).cast::<u16>()).write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1640))
                            .cast::<u16>())
                            .wrapping_offset(17))
                            .read(),
                        );
                        FillPartnerParty(((&raw mut gPartnerTrainerId).cast::<u16>()).read());
                        let __p4 = (&raw mut gBattleTypeFlags).cast::<u32>();
                        (__p4).write(((__p4).read() | 4227137u32));
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        let __p5 = (&raw mut gBattleTypeFlags).cast::<u32>();
                        (__p5).write(((__p5).read() | 8388675u32));
                        FillFrontierTrainersParties(2u8);
                        break 'l2;
                    }
                }
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(0i32));
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 6i32) {
                            break 'l3;
                        }
                        'l4: {
                            let mut itemBefore: u16 = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                12i32,
                            )) as u16);
                            SetMonData(
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(568))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                                12i32,
                                (&raw mut itemBefore).cast::<u8>(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(12i32));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ZeroEnemyPartyMons();
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i < ((crate::c::div_u32(132u32, 44u32)) as i32)) {
                            break 'l5;
                        }
                        'l6: {
                            CreateBattleTowerMon(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1440))
                                .wrapping_add(52))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 44),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(2056u32);
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(0u16);
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(13i32));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(65544u32);
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    let __p6 = (&raw mut gBattleTypeFlags).cast::<u32>();
                    (__p6).write(((__p6).read() | 1u32));
                }
                if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32
                {
                    FillFrontierTrainerParty(2u8);
                }
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                CreateTask_PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(3i32));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(131080u32);
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    let __p7 = (&raw mut gBattleTypeFlags).cast::<u32>();
                    (__p7).write(((__p7).read() | 1u32));
                }
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    != 2i32
                {
                    FillFrontierTrainerParty(3u8);
                } else {
                    FillTentTrainerParty(3u8);
                }
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(4i32));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(262152u32);
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    != 2i32
                {
                    FillFrontierTrainerParty(3u8);
                } else {
                    FillTentTrainerParty(3u8);
                }
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(5i32));
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(524296u32);
                if ((VarGet(16590u16)) as i32) == 1i32 {
                    let __p8 = (&raw mut gBattleTypeFlags).cast::<u32>();
                    (__p8).write(((__p8).read() | 1u32));
                }
                FillFactoryTrainerParty();
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(6i32));
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(264u32);
                FillFrontierTrainerParty(3u8);
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(7i32));
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(2097160u32);
                FillFrontierTrainerParty(3u8);
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(10i32));
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(33033u32);
                FillFrontierTrainersParties(1u8);
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(GetSpecialBattleTransition(7i32));
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(4227145u32);
                FillPartnerParty(3075u16);
                ((&raw mut gApproachingTrainerId).cast::<u8>()).write(0u8);
                BattleSetup_ConfigureTrainerBattle(
                    ((&raw mut MossdeepCity_SpaceCenter_2F_EventScript_MaxieTrainer).cast::<u8>())
                        .wrapping_offset(1),
                );
                ((&raw mut gApproachingTrainerId).cast::<u8>()).write(1u8);
                BattleSetup_ConfigureTrainerBattle(
                    ((&raw mut MossdeepCity_SpaceCenter_2F_EventScript_TabithaTrainer)
                        .cast::<u8>())
                    .wrapping_offset(1),
                );
                ((&raw mut gPartnerTrainerId).cast::<u16>()).write(3075u16);
                CreateTask(Some(Task_StartBattleAfterTransition), 1u8);
                PlayMapChosenOrBattleBGM(0u16);
                BattleTransition_StartOnField(18u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveCurrentWinStreak() {
    unsafe {
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        let mut winStreak: u16 = GetCurrentBattleTowerWinStreak(lvlMode, battleMode);
        if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1684))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read()) as i32)
            < ((winStreak) as i32)
        {
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1684))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .write(winStreak);
        }
    }
}
pub(crate) unsafe extern "C" fn SaveBattleTowerRecord() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut lvlMode: u8 = 0u8;
        let mut battleMode: u8 = 0u8;
        let mut class: u8 = 0u8;
        let mut playerRecord: *mut u8 =
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612));
        ClearBattleTowerRecord(playerRecord);
        lvlMode = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        battleMode = ((VarGet(16590u16)) as u8);
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            != 0i32
        {
            class = ((((&raw const gTowerFemaleFacilityClasses)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::rem_u32(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        ))
                    .wrapping_add(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    ))
                    .wrapping_add(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(3))
                        .read()) as i32),
                    )) as u32),
                    crate::c::div_u32(20u32, 1u32),
                )) as i32) as isize,
            ))
            .read();
        } else {
            class = ((((&raw const gTowerMaleFacilityClasses)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::rem_u32(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        ))
                    .wrapping_add(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    ))
                    .wrapping_add(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(3))
                        .read()) as i32),
                    )) as u32),
                    crate::c::div_u32(30u32, 1u32),
                )) as i32) as isize,
            ))
            .read();
        }
        (playerRecord).write(lvlMode);
        ((playerRecord).wrapping_add(1)).write(class);
        CopyTrainerId(
            ((playerRecord).wrapping_add(12)).cast::<u8>(),
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10)).cast::<u8>(),
        );
        StringCopy_PlayerName(
            ((playerRecord).wrapping_add(4)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((playerRecord).wrapping_add(2).cast::<u16>())
            .write(GetCurrentBattleTowerWinStreak(lvlMode, battleMode));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((playerRecord).wrapping_add(16)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11196))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((playerRecord).wrapping_add(28)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11208))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((playerRecord).wrapping_add(40)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11220))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l3;
                }
                'l4: {
                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        ConvertPokemonToBattleTowerPokemon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            (((playerRecord).wrapping_add(52)).cast::<u8>())
                                .wrapping_offset((i) as isize * 44),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((playerRecord).wrapping_add(228)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        CalcEmeraldBattleTowerChecksum(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)),
        );
        SaveCurrentWinStreak();
    }
}
pub(crate) unsafe extern "C" fn SaveTowerChallenge() {
    unsafe {
        let mut lvlMode: u16 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u16);
        let mut battleMode: u16 = VarGet(16590u16);
        let mut challengeNum: i32 = crate::c::div_i32(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1684))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        );
        if (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 0i32)
            && ((challengeNum > 1i32)
                || ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32))
        {
            SaveBattleTowerRecord();
        }
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
        VarSet(16384u16, 0u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            2,
            1,
            (1u8) as i32,
        );
        SaveGameFrontier();
    }
}
pub(crate) unsafe extern "C" fn BattleTowerNop1() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn BattleTowerNop2() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn GetApprenticeMultiPartnerParty(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut validSpecies = crate::ffi::Align4([0u8; 12]);
        let mut species1: u16 = ((GetMonData3(
            (&raw mut gPlayerParty).cast::<u8>(),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        let mut species2: u16 = ((GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        count = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < crate::c::div_i32(6i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut apprenticeSpecies: u16 =
                        ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                        ))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                        .cast::<u16>())
                        .read();
                    if (((apprenticeSpecies) as i32) != ((species1) as i32))
                        && (((apprenticeSpecies) as i32) != ((species2) as i32))
                    {
                        (((&raw mut validSpecies).cast::<u32>()).wrapping_offset((count) as isize))
                            .write(((i) as u32));
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>()).write(
            (((((&raw mut validSpecies).cast::<u32>())
                .wrapping_offset((crate::c::rem_i32(((Random()) as i32), count)) as isize))
            .read()) as u16),
        );
        'l3: loop {
            'l4: {
                ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(
                    (((((&raw mut validSpecies).cast::<u32>())
                        .wrapping_offset((crate::c::rem_i32(((Random()) as i32), count)) as isize))
                    .read()) as u16),
                );
            }
            if !((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                .read()) as i32)
                == ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .read()) as i32))
            {
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRecordMixFriendMultiPartnerParty(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut validSpecies = crate::ffi::Align4([0u8; 12]);
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut species1: u16 = ((GetMonData3(
            (&raw mut gPlayerParty).cast::<u8>(),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        let mut species2: u16 = ((GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
            11i32,
            core::ptr::null_mut(),
        )) as u16);
        count = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                    ))
                    .wrapping_add(52))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 44))
                    .cast::<u16>())
                    .read()) as i32)
                        != ((species1) as i32))
                        && ((((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(52))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 44))
                        .cast::<u16>())
                        .read()) as i32)
                            != ((species2) as i32)))
                        && ((((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(52))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 44))
                        .wrapping_add(12))
                        .read()) as i32)
                            <= ((GetFrontierEnemyMonLevel(((lvlMode) as u8))) as i32)))
                        && ((((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                        ))
                        .wrapping_add(52))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 44))
                        .cast::<u16>())
                        .read()) as i32)
                            != 0i32)
                    {
                        (((&raw mut validSpecies).cast::<u32>()).wrapping_offset((count) as isize))
                            .write(((i) as u32));
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
            .wrapping_offset(2))
        .write(
            (((((&raw mut validSpecies).cast::<u32>())
                .wrapping_offset((crate::c::rem_i32(((Random()) as i32), count)) as isize))
            .read()) as u16),
        );
        'l3: loop {
            'l4: {
                ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(3))
                .write(
                    (((((&raw mut validSpecies).cast::<u32>())
                        .wrapping_offset((crate::c::rem_i32(((Random()) as i32), count)) as isize))
                    .read()) as u16),
                );
            }
            if !(((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                .wrapping_offset(2))
            .read()) as i32)
                == ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(3))
                .read()) as i32))
            {
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMultiPartnerCandidatesData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut spArray = crate::ffi::Align4([0u8; 20]);
        let mut r10: i32 = 0i32;
        let mut trainerId: u16 = 0u16;
        let mut monId: u16 = 0u16;
        let mut lvlMode: u32 = 0u32;
        let mut battleMode: u32 = 0u32;
        let mut challengeNum: i32 = 0i32;
        let mut species1: u32 = 0u32;
        let mut species2: u32 = 0u32;
        let mut level: u32 = 0u32;
        let mut objEventTemplates: *mut u8 = core::ptr::null_mut();
        objEventTemplates = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        lvlMode = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        battleMode = ((VarGet(16590u16)) as u32);
        challengeNum = crate::c::div_i32(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1684))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(((lvlMode) as i32) as isize))
            .read()) as i32),
            7i32,
        );
        species1 = GetMonData3(
            (&raw mut gPlayerParty).cast::<u8>(),
            11i32,
            core::ptr::null_mut(),
        );
        species2 = GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
            11i32,
            core::ptr::null_mut(),
        );
        level = ((SetFacilityPtrsGetLevel()) as u32);
        j = 0i32;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        trainerId = GetRandomScaledFrontierTrainerId(((challengeNum) as u8), 0u8);
                        {
                            i = 0i32;
                            'l5: loop {
                                if !(i < j) {
                                    break 'l5;
                                }
                                'l6: {
                                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1640))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        == ((trainerId) as i32)
                                    {
                                        break 'l5;
                                    }
                                    if ((((((&raw mut gFacilityTrainers)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1640))
                                        .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 52,
                                    ))
                                    .read()) as i32)
                                        == ((((((&raw mut gFacilityTrainers)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((trainerId) as i32) as isize * 52))
                                        .read()) as i32)
                                    {
                                        break 'l5;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    if !(i != j) {
                        break 'l3;
                    }
                }
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset((j) as isize))
                .write(trainerId);
                j = (j).wrapping_add(1);
            }
            if !(j < 6i32) {
                break 'l1;
            }
        }
        r10 = 8i32;
        {
            i = 0i32;
            'l7: loop {
                if !(i < 6i32) {
                    break 'l7;
                }
                'l8: {
                    trainerId = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read();
                    (((objEventTemplates).wrapping_offset(((i).wrapping_add(1i32)) as isize * 24))
                        .wrapping_add(1))
                    .write(GetBattleFacilityTrainerGfxId(trainerId));
                    {
                        j = 0i32;
                        'l9: loop {
                            if !(j < 2i32) {
                                break 'l9;
                            }
                            'l10: {
                                'l11: loop {
                                    if !((1i32) != 0) {
                                        break 'l11;
                                    }
                                    monId = GetRandomFrontierMonFromSet(trainerId);
                                    if (crate::c::rem_i32(j, 2i32) != 0i32)
                                        && ((((((((&raw mut gFacilityTrainerMons)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(
                                            (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1640))
                                            .cast::<u16>())
                                            .wrapping_offset(((r10).wrapping_sub(1i32)) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 16,
                                        ))
                                        .wrapping_add(10))
                                        .read())
                                            as i32)
                                            == (((((((&raw mut gFacilityTrainerMons)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((monId) as i32) as isize * 16))
                                            .wrapping_add(10))
                                            .read())
                                                as i32))
                                    {
                                        break 'l11;
                                    }
                                    {
                                        k = 8i32;
                                        'l12: loop {
                                            if !(k < r10) {
                                                break 'l12;
                                            }
                                            'l13: {
                                                if (((((((&raw mut gFacilityTrainerMons)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    (((((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(1612))
                                                    .wrapping_add(1640))
                                                    .cast::<u16>())
                                                    .wrapping_offset((k) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 16,
                                                ))
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    == (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        ((monId) as i32) as isize * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                {
                                                    break 'l12;
                                                }
                                                if species1
                                                    == (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        ((monId) as i32) as isize * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as u32)
                                                {
                                                    break 'l12;
                                                }
                                                if species2
                                                    == (((((((&raw mut gFacilityTrainerMons)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        ((monId) as i32) as isize * 16,
                                                    ))
                                                    .cast::<u16>())
                                                    .read())
                                                        as u32)
                                                {
                                                    break 'l12;
                                                }
                                            }
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                    if k == r10 {
                                        break 'l11;
                                    }
                                }
                                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1640))
                                .cast::<u16>())
                                .wrapping_offset((r10) as isize))
                                .write(monId);
                                r10 = (r10).wrapping_add(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        r10 = 0i32;
        ValidateApprenticesChecksums();
        {
            i = 0i32;
            'l14: loop {
                if !(i < 4i32) {
                    break 'l14;
                }
                'l15: {
                    if ((((crate::c::bf_read(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 68))
                        .wrapping_add(0),
                        5,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32)
                        && (crate::c::div_i32(
                            ((((((&raw const sApprenticeChallengeThreshold)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(220))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 68))
                                .wrapping_add(1))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32),
                            7i32,
                        ) <= challengeNum))
                        && (((((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 68))
                            .wrapping_add(0),
                            5,
                            2,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32)) as u32)
                            == lvlMode)
                    {
                        k = 0i32;
                        {
                            j = 0i32;
                            'l16: loop {
                                if !(j < crate::c::div_i32(6i32, 2i32)) {
                                    break 'l16;
                                }
                                'l17: {
                                    if (species1
                                        != ((((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 68))
                                        .wrapping_add(4))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 12))
                                        .cast::<u16>())
                                        .read())
                                            as u32))
                                        && (species2
                                            != ((((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(220))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 68))
                                            .wrapping_add(4))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 12))
                                            .cast::<u16>())
                                            .read())
                                                as u32))
                                    {
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if k > 2i32 {
                            (((&raw mut spArray).cast::<u32>()).wrapping_offset((r10) as isize))
                                .write((((i).wrapping_add(400i32)) as u32));
                            r10 = (r10).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if r10 != 0i32 {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(6))
            .write(
                (((((&raw mut spArray).cast::<u32>())
                    .wrapping_offset((crate::c::rem_i32(((Random()) as i32), r10)) as isize))
                .read()) as u16),
            );
            (((objEventTemplates).wrapping_offset(168)).wrapping_add(1)).write(
                GetBattleFacilityTrainerGfxId(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(6))
                    .read(),
                ),
            );
            FlagClear(864u16);
            GetApprenticeMultiPartnerParty(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(6))
                .read(),
            );
        }
        r10 = 0i32;
        {
            i = 0i32;
            'l18: loop {
                if !(i < 5i32) {
                    break 'l18;
                }
                'l19: {
                    let mut record: *mut u32 =
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .cast::<u32>();
                    let mut recordHasData: u32 = 0u32;
                    let mut checksum: u32 = 0u32;
                    {
                        j = 0i32;
                        'l20: loop {
                            if !(((j) as u32) < crate::c::div_u32(232u32, 4u32)) {
                                break 'l20;
                            }
                            'l21: {
                                recordHasData = (recordHasData
                                    | ((record).wrapping_offset((j) as isize)).read());
                                checksum = (checksum)
                                    .wrapping_add(((record).wrapping_offset((j) as isize)).read());
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if (((crate::c::div_i32(
                        ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32),
                        7i32,
                    ) <= challengeNum)
                        && ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .read()) as u32)
                            == lvlMode))
                        && ((recordHasData) != 0))
                        && (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(236))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 236))
                        .wrapping_add(232)
                        .cast::<u32>())
                        .read()
                            == checksum)
                    {
                        k = 0i32;
                        {
                            j = 0i32;
                            'l22: loop {
                                if !(j
                                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                                        3i32
                                    } else {
                                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                                    }))
                                {
                                    break 'l22;
                                }
                                'l23: {
                                    if (((species1
                                        != (((((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 236))
                                        .wrapping_add(52))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 44))
                                        .cast::<u16>())
                                        .read())
                                            as u32))
                                        && (species2
                                            != (((((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(236))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 236))
                                            .wrapping_add(52))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 44))
                                            .cast::<u16>())
                                            .read())
                                                as u32)))
                                        && ((((((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 236))
                                        .wrapping_add(52))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 44))
                                        .wrapping_add(12))
                                        .read())
                                            as i32)
                                            <= ((GetFrontierEnemyMonLevel(((lvlMode) as u8)))
                                                as i32)))
                                        && ((((((((((((((&raw mut gSaveBlock2Ptr)
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 236))
                                        .wrapping_add(52))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize * 44))
                                        .cast::<u16>())
                                        .read())
                                            as i32)
                                            != 0i32)
                                    {
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        if k > 1i32 {
                            (((&raw mut spArray).cast::<u32>()).wrapping_offset((r10) as isize))
                                .write((((i).wrapping_add(300i32)) as u32));
                            r10 = (r10).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if r10 != 0i32 {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(7))
            .write(
                (((((&raw mut spArray).cast::<u32>())
                    .wrapping_offset((crate::c::rem_i32(((Random()) as i32), r10)) as isize))
                .read()) as u16),
            );
            (((objEventTemplates).wrapping_offset(192)).wrapping_add(1)).write(
                GetBattleFacilityTrainerGfxId(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(7))
                    .read(),
                ),
            );
            FlagClear(865u16);
            GetRecordMixFriendMultiPartnerParty(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(7))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetPotentialPartnerMoveAndSpecies(trainerId: u16, monId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        let mut monId = monId;
        let mut r#move: u16 = 0u16;
        let mut species: u16 = 0u16;
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) != 500i32 {
            if ((trainerId) as i32) < 300i32 {
                r#move = ((((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(2))
                .cast::<u16>())
                .read();
                species = (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read();
            } else {
                if ((trainerId) as i32) < 400i32 {
                    r#move = ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                    ))
                    .wrapping_add(52))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                .wrapping_add(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 44,
                    ))
                    .wrapping_add(4))
                    .cast::<u16>())
                    .read();
                    species = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                    ))
                    .wrapping_add(52))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                .wrapping_add(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 44,
                    ))
                    .cast::<u16>())
                    .read();
                } else {
                    let mut i: i32 = 0i32;
                    r#move = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                    ))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .wrapping_add(2))
                    .cast::<u16>())
                    .read();
                    species = ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                    ))
                    .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                            .cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 12,
                    ))
                    .cast::<u16>())
                    .read();
                    {
                        i = 0i32;
                        'l1: loop {
                            if !(i < 7i32) {
                                break 'l1;
                            }
                            'l2: {
                                (((&raw mut gStringVar3).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(220))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                                    ))
                                    .wrapping_add(56))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    (((&raw mut gStringVar3).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(255u8);
                    ConvertInternationalString(
                        (&raw mut gStringVar3).cast::<u8>(),
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(220))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                        ))
                        .wrapping_add(63))
                        .read(),
                    );
                }
            }
        }
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 11))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn ShowPartnerCandidateMessage() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut partnerId: i32 = 0i32;
        let mut monId: i32 = 0i32;
        let mut level: i32 = ((SetFacilityPtrsGetLevel()) as i32);
        let mut winStreak: u16 = ((GetCurrentFacilityWinStreak()) as u16);
        let mut challengeNum: i32 = crate::c::div_i32(((winStreak) as i32), 7i32);
        let mut k: i32 =
            ((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as i32).wrapping_sub(2i32);
        let mut trainerId: i32 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1640))
        .cast::<u16>())
        .wrapping_offset((k) as isize))
        .read()) as i32);
        {
            partnerId = 0i32;
            'l1: loop {
                if !(((partnerId) as u32) < crate::c::div_u32(400u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw const sPartnerTrainerTextTables)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((partnerId) as isize * 8))
                    .cast::<u32>())
                    .read()
                        == ((GetFrontierTrainerFacilityClass(((trainerId) as u16))) as u32)
                    {
                        break 'l1;
                    }
                }
                partnerId = (partnerId).wrapping_add(1);
            }
        }
        'l3: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                if trainerId == 500i32 {
                    return;
                }
                if trainerId < 300i32 {
                    GetFrontierTrainerName(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((trainerId) as u16),
                    );
                } else {
                    if trainerId < 400i32 {
                        GetFrontierTrainerName(
                            (&raw mut gStringVar1).cast::<u8>(),
                            ((trainerId) as u16),
                        );
                    } else {
                        let mut i: i32 = 0i32;
                        {
                            i = 0i32;
                            'l4: loop {
                                if !(i < 7i32) {
                                    break 'l4;
                                }
                                'l5: {
                                    (((&raw mut gStringVar1).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(
                                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            ((trainerId).wrapping_sub(400i32)) as isize * 68,
                                        ))
                                        .wrapping_add(56))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        (((&raw mut gStringVar1).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(255u8);
                        ConvertInternationalString(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(((trainerId).wrapping_sub(400i32)) as isize * 68))
                            .wrapping_add(63))
                            .read(),
                        );
                        ConvertIntToDecimalStringN(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(((trainerId).wrapping_sub(400i32)) as isize * 68))
                            .wrapping_add(2))
                            .read()) as i32),
                            0i32,
                            3u8,
                        );
                        GetFrontierTrainerName(
                            (&raw mut gStringVar3).cast::<u8>(),
                            ((trainerId) as u16),
                        );
                    }
                }
                break 'l3;
            }
            if __sw1 == 1i32 {
                monId = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(((8i32).wrapping_add((k).wrapping_mul(2i32))) as isize))
                .read()) as i32);
                GetPotentialPartnerMoveAndSpecies(((trainerId) as u16), ((monId) as u16));
                break 'l3;
            }
            if __sw1 == 2i32 {
                monId = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(((9i32).wrapping_add((k).wrapping_mul(2i32))) as isize))
                .read()) as i32);
                GetPotentialPartnerMoveAndSpecies(((trainerId) as u16), ((monId) as u16));
                break 'l3;
            }
            if __sw1 == 3i32 {
                ((&raw mut gPartnerTrainerId).cast::<u16>()).write(((trainerId) as u16));
                if trainerId < 300i32 {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(18))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(((8i32).wrapping_add((k).wrapping_mul(2i32))) as isize))
                        .read(),
                    );
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset(19))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(((9i32).wrapping_add((k).wrapping_mul(2i32))) as isize))
                        .read(),
                    );
                } else {
                    if trainerId < 400i32 {
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(18))
                        .write(
                            ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .read(),
                        );
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(19))
                        .write(
                            ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(3))
                            .read(),
                        );
                    } else {
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(18))
                        .write(
                            (((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .read(),
                        );
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(19))
                        .write(
                            ((((&raw mut gFrontierTempParty).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(1))
                            .read(),
                        );
                    }
                }
                {
                    k = 0i32;
                    'l6: loop {
                        if !(k < 14i32) {
                            break 'l6;
                        }
                        'l7: {
                            'l8: loop {
                                if !((1i32) != 0) {
                                    break 'l8;
                                }
                                i = ((GetRandomScaledFrontierTrainerId(
                                    ((challengeNum) as u8),
                                    ((crate::c::div_i32(k, 2i32)) as u8),
                                )) as i32);
                                if ((((&raw mut gPartnerTrainerId).cast::<u16>()).read()) as i32)
                                    == i
                                {
                                    break 'l8;
                                }
                                {
                                    j = 0i32;
                                    'l9: loop {
                                        if !(j < k) {
                                            break 'l9;
                                        }
                                        'l10: {
                                            if (((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(1612))
                                            .wrapping_add(1640))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                == i
                                            {
                                                break 'l9;
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                if j == k {
                                    break 'l8;
                                }
                            }
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1640))
                            .cast::<u16>())
                            .wrapping_offset((k) as isize))
                            .write(((i) as u16));
                        }
                        k = (k).wrapping_add(1);
                    }
                }
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1640))
                .cast::<u16>())
                .wrapping_offset(17))
                .write(((trainerId) as u16));
                break 'l3;
            }
            if __sw1 == 4i32 {
                break 'l3;
            }
        }
        if trainerId == 500i32 {
            return;
        }
        if trainerId < 300i32 {
            ShowFieldMessage(
                (((((((&raw const sPartnerTrainerTextTables)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((partnerId) as isize * 8))
                .wrapping_add(4)
                .cast::<*mut *mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
                ))
                .read(),
            );
        } else {
            if trainerId < 400i32 {
                ShowFieldMessage(
                    (((((((&raw const sPartnerTrainerTextTables)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((partnerId) as isize * 8))
                    .wrapping_add(4)
                    .cast::<*mut *mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
            } else {
                let mut apprenticeId: u8 = (crate::c::bf_read(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(220))
                        .cast::<u8>())
                    .wrapping_offset(((trainerId).wrapping_sub(400i32)) as isize * 68))
                    .wrapping_add(0),
                    0,
                    5,
                    false,
                ) as u8);
                ShowFieldMessage(
                    ((((((&raw const sPartnerApprenticeTextTables)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut *mut u8>())
                    .cast::<*mut *mut u8>())
                    .wrapping_offset(((apprenticeId) as i32) as isize))
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadLinkMultiOpponentsData() {
    unsafe {
        let mut challengeNum: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut trainerId: i32 = 0i32;
        let mut lvlMode: u32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as u32);
        let mut battleMode: u32 = ((VarGet(16590u16)) as u32);
        let mut battleNum: u32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1638)
        .cast::<u16>())
        .read()) as u32);
        GetMultiplayerId();
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                if battleMode == 3u32 {
                    challengeNum = crate::c::div_i32(
                        (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1684))
                        .cast::<u8>())
                        .wrapping_offset(((battleMode) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(((lvlMode) as i32) as isize))
                        .read()) as i32),
                        7i32,
                    );
                    if (IsLinkTaskFinished()) != 0 {
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            (&raw mut challengeNum).cast::<u8>(),
                            4u16,
                        );
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                    }
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32)
                        > ((((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256))
                            .cast::<u16>())
                        .read()) as i32)
                    {
                        challengeNum = (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>())
                            .read()) as i32);
                    } else {
                        challengeNum = ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(256))
                        .cast::<u16>())
                        .read()) as i32);
                    }
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 14i32) {
                                break 'l2;
                            }
                            'l3: {
                                'l4: loop {
                                    'l5: {
                                        trainerId = ((GetRandomScaledFrontierTrainerId(
                                            ((challengeNum) as u8),
                                            ((crate::c::div_i32(i, 2i32)) as u8),
                                        ))
                                            as i32);
                                        {
                                            j = 0i32;
                                            'l6: loop {
                                                if !(j < i) {
                                                    break 'l6;
                                                }
                                                'l7: {
                                                    if (((((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(1612))
                                                    .wrapping_add(1640))
                                                    .cast::<u16>())
                                                    .wrapping_offset((j) as isize))
                                                    .read())
                                                        as i32)
                                                        == trainerId
                                                    {
                                                        break 'l6;
                                                    }
                                                }
                                                j = (j).wrapping_add(1);
                                            }
                                        }
                                    }
                                    if !(i != j) {
                                        break 'l4;
                                    }
                                }
                                if i == j {
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1640))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .write(((trainerId) as u16));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .cast::<u8>(),
                        40u16,
                    );
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((GetBlockReceivedStatus()) as i32) & 3i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    crate::c::memcpy(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .cast::<u8>(),
                        (&raw mut gBlockRecvBuffer).cast::<u8>(),
                        40u32,
                    );
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset((((battleNum).wrapping_mul(2u32)) as i32) as isize))
                        .read(),
                    );
                    ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1640))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((battleNum).wrapping_mul(2u32)).wrapping_add(1u32)) as i32) as isize,
                        ))
                        .read(),
                    );
                    SetBattleFacilityTrainerGfxId(
                        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                        0u8,
                    );
                    SetBattleFacilityTrainerGfxId(
                        ((&raw mut gTrainerBattleOpponent_B).cast::<u16>()).read(),
                        1u8,
                    );
                    if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                        && (((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32)
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(4u16);
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetCloseLinkCallback();
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                return;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TowerTryCloseLink() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            SetCloseLinkCallback();
        }
    }
}
pub(crate) unsafe extern "C" fn SetMultiPartnerGfx() {
    unsafe {
        SetBattleFacilityTrainerGfxId(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(17))
            .read(),
            15u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetTowerInterviewData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut text = crate::ffi::Align4([0u8; 32]);
        if ((VarGet(16590u16)) as i32) != 0i32 {
            return;
        }
        GetFrontierTrainerName(
            (&raw mut text).cast::<u8>(),
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
        );
        StripExtCtrlCodes((&raw mut text).cast::<u8>());
        StringCopy(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .wrapping_add(4))
            .cast::<u8>(),
            (&raw mut text).cast::<u8>(),
        );
        GetBattleTowerTrainerLanguage(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .wrapping_add(23),
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
        );
        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1416))
        .wrapping_add(2)
        .cast::<u16>())
        .write(
            ((GetMonData3(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 100,
                ),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1416))
        .cast::<u16>())
        .write(
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).read())
                        as i32) as isize
                        * 100,
                ),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 11i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1416))
                    .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(
                        (((((&raw mut gBattleMons).cast::<u8>()).wrapping_add(48)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1722))
        .write(((&raw mut gBattleOutcome).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn ValidateBattleTowerRecordChecksums() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut record: *mut u32 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .cast::<u32>();
        let mut checksum: u32 = 0u32;
        {
            j = 0i32;
            'l1: loop {
                if !(((j) as u32) < crate::c::div_u32(232u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    checksum =
                        (checksum).wrapping_add(((record).wrapping_offset((j) as isize)).read());
                }
                j = (j).wrapping_add(1);
            }
        }
        if (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(232)
            .cast::<u32>())
        .read()
            != checksum
        {
            ClearBattleTowerRecord(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612)),
            );
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 5i32) {
                    break 'l3;
                }
                'l4: {
                    record = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 236))
                    .cast::<u32>();
                    checksum = 0u32;
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(((j) as u32) < crate::c::div_u32(232u32, 4u32)) {
                                break 'l5;
                            }
                            'l6: {
                                checksum = (checksum)
                                    .wrapping_add(((record).wrapping_offset((j) as isize)).read());
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 236))
                    .wrapping_add(232)
                    .cast::<u32>())
                    .read()
                        != checksum
                    {
                        ClearBattleTowerRecord(
                            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(236))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 236),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalcEmeraldBattleTowerChecksum(record: *mut u8) {
    unsafe {
        let mut record = record;
        let mut i: u32 = 0u32;
        ((record).wrapping_add(232).cast::<u32>()).write(0u32);
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(232u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (record).wrapping_add(232).cast::<u32>();
                    (__p1).write(((__p1).read()).wrapping_add(
                        (((record).cast::<u32>()).wrapping_offset(((i) as i32) as isize)).read(),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalcRubyBattleTowerChecksum(record: *mut u8) {
    unsafe {
        let mut record = record;
        let mut i: u32 = 0u32;
        ((record).wrapping_add(160).cast::<u32>()).write(0u32);
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(160u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (record).wrapping_add(160).cast::<u32>();
                    (__p1).write(((__p1).read()).wrapping_add(
                        (((record).cast::<u32>()).wrapping_offset(((i) as i32) as isize)).read(),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearBattleTowerRecord(record: *mut u8) {
    unsafe {
        let mut record = record;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(236u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    (((record).cast::<u32>()).wrapping_offset(((i) as i32) as isize)).write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentBattleTowerWinStreak(lvlMode: u8, battleMode: u8) -> u16 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut battleMode = battleMode;
        let mut winStreak: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1684))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonCountForBattleMode(battleMode: u8) -> u8 {
    unsafe {
        let mut battleMode = battleMode;
        let mut partySizes = crate::ffi::Align4([0u8; 4]);
        crate::c::memcpy(
            (&raw mut partySizes).cast::<u8>(),
            ((&raw const sBattleTowerPartySizes).cast::<u8>().cast_mut()).cast::<u8>(),
            4u32,
        );
        if ((battleMode) as u32) < crate::c::div_u32(4u32, 1u32) {
            return (((&raw mut partySizes).cast::<u8>())
                .wrapping_offset(((battleMode) as i32) as isize))
            .read();
        } else {
            return 3u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AwardBattleTowerRibbons() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut partyIndex: u32 = 0u32;
        let mut ribbons = crate::ffi::Align4([0u8; 12]);
        let mut ribbonType: u8 = 0u8;
        let mut lvlMode: u8 = (crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8);
        let mut battleMode: u8 = ((VarGet(16590u16)) as u8);
        let mut monCount: u8 = GetMonCountForBattleMode(battleMode);
        if ((lvlMode) as i32) != 0i32 {
            ribbonType = 69u8;
        } else {
            ribbonType = 68u8;
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        if ((GetCurrentBattleTowerWinStreak(lvlMode, battleMode)) as i32) > 55i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((monCount) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        partyIndex = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                            .read())
                        .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u32);
                        (((&raw mut ribbons).cast::<u8>()).wrapping_offset((i) as isize * 4))
                            .write(((partyIndex) as u8));
                        ((((&raw mut ribbons).cast::<u8>()).wrapping_offset((i) as isize * 4))
                            .wrapping_add(1))
                        .write(0u8);
                        if !((GetMonData2(
                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(568))
                            .cast::<u8>())
                            .wrapping_offset(((partyIndex) as i32) as isize * 100),
                            ((ribbonType) as i32),
                        )) != 0)
                        {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                            SetMonData(
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(568))
                                .cast::<u8>())
                                .wrapping_offset(((partyIndex) as i32) as isize * 100),
                                ((ribbonType) as i32),
                                ((&raw mut gSpecialVar_Result).cast::<u16>()).cast::<u8>(),
                            );
                            ((((&raw mut ribbons).cast::<u8>()).wrapping_offset((i) as isize * 4))
                                .wrapping_add(1))
                            .write(GetRibbonCount(
                                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(568))
                                .cast::<u8>())
                                .wrapping_offset(((partyIndex) as i32) as isize * 100),
                            ));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0 {
            IncrementGameStat(42u8);
            {
                i = 1i32;
                'l3: loop {
                    if !(i < ((monCount) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((&raw mut ribbons).cast::<u8>())
                            .wrapping_offset((i) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32)
                            > (((((&raw mut ribbons).cast::<u8>()).wrapping_add(1)).read()) as i32)
                        {
                            let mut prevBest = crate::ffi::Align4([0u8; 4]);
                            (&raw mut prevBest)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<4>>()
                                .write_unaligned(
                                    (&raw mut ribbons)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                );
                            (&raw mut ribbons)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<4>>()
                                .write_unaligned(
                                    ((&raw mut ribbons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 4)
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                );
                            ((&raw mut ribbons).cast::<u8>())
                                .wrapping_offset((i) as isize * 4)
                                .cast::<crate::c::Rec4<4>>()
                                .write_unaligned(
                                    (&raw mut prevBest)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<4>>()
                                        .read_unaligned(),
                                );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((&raw mut ribbons).cast::<u8>()).wrapping_add(1)).read()) as i32) > 4i32 {
                TryPutSpotTheCutiesOnAir(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(568))
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut ribbons).cast::<u8>()).read()) as i32) as isize * 100,
                    ),
                    ribbonType,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillEReaderTrainerWithPlayerData() {
    unsafe {
        let mut ereaderTrainer: *mut u8 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1440);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            != 0i32
        {
            ((ereaderTrainer).wrapping_add(1)).write(
                ((((&raw const gTowerFemaleFacilityClasses)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::rem_u32(
                        (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            ))
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32),
                        ))
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .read()) as i32),
                        )) as u32),
                        crate::c::div_u32(20u32, 1u32),
                    )) as i32) as isize,
                ))
                .read(),
            );
        } else {
            ((ereaderTrainer).wrapping_add(1)).write(
                ((((&raw const gTowerMaleFacilityClasses)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::rem_u32(
                        (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            ))
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32),
                        ))
                        .wrapping_add(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .read()) as i32),
                        )) as u32),
                        crate::c::div_u32(30u32, 1u32),
                    )) as i32) as isize,
                ))
                .read(),
            );
        }
        CopyTrainerId(
            ((ereaderTrainer).wrapping_add(12)).cast::<u8>(),
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10)).cast::<u8>(),
        );
        StringCopy_PlayerName(
            ((ereaderTrainer).wrapping_add(4)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((ereaderTrainer).wrapping_add(2).cast::<u16>()).write(1u16);
        j = 7i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((ereaderTrainer).wrapping_add(16)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11196))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                    ((((ereaderTrainer).wrapping_add(28)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(((j) as u16));
                    ((((ereaderTrainer).wrapping_add(40)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write((((j).wrapping_add(6i32)) as u16));
                    j = (j).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((crate::c::div_u32(132u32, 44u32)) as i32)) {
                    break 'l3;
                }
                'l4: {
                    ConvertPokemonToBattleTowerPokemon(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        (((ereaderTrainer).wrapping_add(52)).cast::<u8>())
                            .wrapping_offset((i) as isize * 44),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetEReaderTrainerChecksum(ereaderTrainer);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEreaderTrainerFrontSpriteId() -> u8 {
    unsafe {
        return (((&raw mut gFacilityClassToPicIndex).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1440))
            .wrapping_add(1))
            .read()) as i32) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEreaderTrainerClassId() -> u8 {
    unsafe {
        return (((&raw mut gFacilityClassToTrainerClass).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1440))
            .wrapping_add(1))
            .read()) as i32) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetEreaderTrainerName(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset((i) as isize)).write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1440))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((dst).wrapping_offset((i) as isize)).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateEReaderTrainer() {
    unsafe {
        let mut i: u32 = 0u32;
        let mut checksum: u32 = 0u32;
        let mut ereaderTrainer: *mut u8 = core::ptr::null_mut();
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        ereaderTrainer = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1440);
        checksum = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(184u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    checksum = (checksum
                        | (((ereaderTrainer).cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read());
                }
                i = (i).wrapping_add(1);
            }
        }
        if checksum == 0u32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            return;
        }
        checksum = 0u32;
        {
            i = 0u32;
            'l3: loop {
                if !(i < crate::c::div_u32(184u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    checksum = (checksum).wrapping_add(
                        (((ereaderTrainer).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1440))
        .wrapping_add(184)
        .cast::<u32>())
        .read()
            != checksum
        {
            ClearEReaderTrainer(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1440),
            );
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SetEReaderTrainerChecksum(ereaderTrainer: *mut u8) {
    unsafe {
        let mut ereaderTrainer = ereaderTrainer;
        let mut i: i32 = 0i32;
        ((ereaderTrainer).wrapping_add(184).cast::<u32>()).write(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(184u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (ereaderTrainer).wrapping_add(184).cast::<u32>();
                    (__p1).write(((__p1).read()).wrapping_add(
                        (((ereaderTrainer).cast::<u32>()).wrapping_offset((i) as isize)).read(),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearEReaderTrainer(ereaderTrainer: *mut u8) {
    unsafe {
        let mut ereaderTrainer = ereaderTrainer;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(188u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    (((ereaderTrainer).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyEReaderTrainerGreeting() {
    unsafe {
        FrontierSpeechToString(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1440))
            .wrapping_add(16))
            .cast::<u16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CopyEReaderTrainerFarewellMessage() {
    unsafe {
        if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 3i32 {
            ((&raw mut gStringVar4).cast::<u8>()).write(255u8);
        } else {
            if ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32 {
                FrontierSpeechToString(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1440))
                    .wrapping_add(40))
                    .cast::<u16>(),
                );
            } else {
                FrontierSpeechToString(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1440))
                    .wrapping_add(28))
                    .cast::<u16>(),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryHideBattleTowerReporter() {
    unsafe {
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .read()) as i32)
            == 1i32
        {
            HideBattleTowerReporter();
        }
        if ((FlagGet(119u16)) as i32) == 1i32 {
            HideBattleTowerReporter();
            FlagClear(119u16);
        }
    }
}
pub(crate) unsafe extern "C" fn FillPartnerParty(trainerId: u16) {
    unsafe {
        let mut trainerId = trainerId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut ivs: u32 = 0u32;
        let mut level: u32 = 0u32;
        let mut friendship: u32 = 0u32;
        let mut monId: u16 = 0u16;
        let mut otID: u32 = 0u32;
        let mut trainerName = crate::ffi::Align4([0u8; 8]);
        SetFacilityPtrsGetLevel();
        if ((trainerId) as i32) == 3075i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < crate::c::div_i32(6i32, 2i32)) {
                        break 'l1;
                    }
                    'l2: {
                        'l3: loop {
                            'l4: {
                                j = (((Random()) as i32) | (((Random()) as i32) << 16));
                            }
                            if !(((IsShinyOtIdPersonality(61226u32, ((j) as u32))) != 0)
                                || ((((((((&raw const sStevenMons).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(4))
                                .read()) as i32)
                                    != ((GetNatureFromPersonality(((j) as u32))) as i32)))
                            {
                                break 'l3;
                            }
                        }
                        CreateMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize * 100,
                            ),
                            (((((&raw const sStevenMons).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                            .cast::<u16>())
                            .read(),
                            (((((&raw const sStevenMons).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                            .wrapping_add(3))
                            .read(),
                            (((((&raw const sStevenMons).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                            .wrapping_add(2))
                            .read(),
                            1u8,
                            ((i) as u32),
                            1u8,
                            61226u32,
                        );
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 6i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        (26i32).wrapping_add(j),
                                        ((((((&raw const sStevenMons).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize * 20))
                                        .wrapping_add(5))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        {
                            j = 0i32;
                            'l7: loop {
                                if !(j < 4i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    SetMonMoveSlot(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        (((((((&raw const sStevenMons)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 20))
                                        .wrapping_add(12))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read(),
                                        ((j) as u8),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize * 100,
                            ),
                            7i32,
                            ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(32160))
                                .wrapping_add(4))
                            .cast::<u8>(),
                        );
                        j = 0i32;
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize * 100,
                            ),
                            49i32,
                            (&raw mut j).cast::<u8>(),
                        );
                        CalculateMonStats(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize * 100,
                        ));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            if ((trainerId) as i32) == 500i32 {
                ((&raw mut trainerName).cast::<u8>())
                    .write(((&raw mut gGameLanguage).cast::<u8>()).read());
            } else {
                if ((trainerId) as i32) < 300i32 {
                    level = ((SetFacilityPtrsGetLevel()) as u32);
                    ivs = ((GetFrontierTrainerFixedIvs(trainerId)) as u32);
                    otID = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
                    {
                        i = 0i32;
                        'l9: loop {
                            if !(i < 2i32) {
                                break 'l9;
                            }
                            'l10: {
                                monId = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1612))
                                .wrapping_add(1640))
                                .cast::<u16>())
                                .wrapping_offset(((i).wrapping_add(18i32)) as isize))
                                .read();
                                CreateMonWithEVSpreadNatureOTID(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize
                                            * 100,
                                    ),
                                    (((((&raw mut gFacilityTrainerMons)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .read(),
                                    ((level) as u8),
                                    (((((&raw mut gFacilityTrainerMons)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(12))
                                    .read(),
                                    ((ivs) as u8),
                                    (((((&raw mut gFacilityTrainerMons)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((monId) as i32) as isize * 16))
                                    .wrapping_add(11))
                                    .read(),
                                    otID,
                                );
                                friendship = 255u32;
                                {
                                    j = 0i32;
                                    'l11: loop {
                                        if !(j < 4i32) {
                                            break 'l11;
                                        }
                                        'l12: {
                                            SetMonMoveSlot(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((crate::c::div_i32(6i32, 2i32))
                                                            .wrapping_add(i))
                                                            as isize
                                                            * 100,
                                                    ),
                                                (((((((&raw mut gFacilityTrainerMons)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    ((monId) as i32) as isize * 16,
                                                ))
                                                .wrapping_add(2))
                                                .cast::<u16>())
                                                .wrapping_offset((j) as isize))
                                                .read(),
                                                ((j) as u8),
                                            );
                                            if (((((((((&raw mut gFacilityTrainerMons)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((monId) as i32) as isize * 16))
                                            .wrapping_add(2))
                                            .cast::<u16>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                                as i32)
                                                == 218i32
                                            {
                                                friendship = 0u32;
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize
                                            * 100,
                                    ),
                                    32i32,
                                    (&raw mut friendship).cast::<u8>(),
                                );
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize
                                            * 100,
                                    ),
                                    12i32,
                                    ((((&raw const gBattleFrontierHeldItems)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u16>())
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        (((((((&raw mut gFacilityTrainerMons)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((monId) as i32) as isize * 16))
                                        .wrapping_add(10))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .cast::<u8>(),
                                );
                                {
                                    j = 0i32;
                                    'l13: loop {
                                        if !(j < 8i32) {
                                            break 'l13;
                                        }
                                        'l14: {
                                            (((&raw mut trainerName).cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                            .write(
                                                (((((((&raw mut gFacilityTrainers)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    ((trainerId) as i32) as isize * 52,
                                                ))
                                                .wrapping_add(4))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize))
                                                .read(),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize
                                            * 100,
                                    ),
                                    7i32,
                                    ((&raw mut trainerName).cast::<u8>()).cast::<u8>(),
                                );
                                j = ((IsFrontierTrainerFemale(trainerId)) as i32);
                                SetMonData(
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i)) as isize
                                            * 100,
                                    ),
                                    49i32,
                                    (&raw mut j).cast::<u8>(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    if ((trainerId) as i32) < 400i32 {
                        trainerId = ((((trainerId) as i32).wrapping_sub(300i32)) as u16);
                        {
                            i = 0i32;
                            'l15: loop {
                                if !(i < 2i32) {
                                    break 'l15;
                                }
                                'l16: {
                                    let mut record: *mut u8 = ((((((&raw mut gSaveBlock2Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(((trainerId) as i32) as isize * 236);
                                    let mut monData = crate::ffi::Align4([0u8; 44]);
                                    (&raw mut monData)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<44>>()
                                        .write_unaligned(
                                            (((record).wrapping_add(52)).cast::<u8>())
                                                .wrapping_offset(
                                                    (((((((((&raw mut gSaveBlock2Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(1612))
                                                    .wrapping_add(1640))
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        ((18i32).wrapping_add(i)) as isize,
                                                    ))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 44,
                                                )
                                                .cast::<crate::c::Rec4<44>>()
                                                .read_unaligned(),
                                        );
                                    StringCopy(
                                        (&raw mut trainerName).cast::<u8>(),
                                        ((record).wrapping_add(4)).cast::<u8>(),
                                    );
                                    if ((((record).wrapping_add(228)).read()) as i32) == 1i32 {
                                        if (((((((&raw mut monData).cast::<u8>())
                                            .wrapping_add(32))
                                        .cast::<u8>())
                                        .read())
                                            as i32)
                                            != 252i32)
                                            || ((((((((&raw mut monData).cast::<u8>())
                                                .wrapping_add(32))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                != 21i32)
                                        {
                                            (((((&raw mut monData).cast::<u8>())
                                                .wrapping_add(32))
                                            .cast::<u8>())
                                            .wrapping_offset(5))
                                            .write(255u8);
                                            ConvertInternationalString(
                                                (((&raw mut monData).cast::<u8>())
                                                    .wrapping_add(32))
                                                .cast::<u8>(),
                                                1u8,
                                            );
                                        }
                                    } else {
                                        if (((((((&raw mut monData).cast::<u8>())
                                            .wrapping_add(32))
                                        .cast::<u8>())
                                        .read())
                                            as i32)
                                            == 252i32)
                                            && ((((((((&raw mut monData).cast::<u8>())
                                                .wrapping_add(32))
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                == 21i32)
                                        {
                                            (((&raw mut trainerName).cast::<u8>())
                                                .wrapping_offset(5))
                                            .write(255u8);
                                        }
                                    }
                                    CreateBattleTowerMon_HandleLevel(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        (&raw mut monData).cast::<u8>(),
                                        1u8,
                                    );
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        7i32,
                                        (&raw mut trainerName).cast::<u8>(),
                                    );
                                    j = ((IsFrontierTrainerFemale(
                                        ((((trainerId) as i32).wrapping_add(300i32)) as u16),
                                    )) as i32);
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        49i32,
                                        (&raw mut j).cast::<u8>(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    } else {
                        trainerId = ((((trainerId) as i32).wrapping_sub(400i32)) as u16);
                        {
                            i = 0i32;
                            'l17: loop {
                                if !(i < 2i32) {
                                    break 'l17;
                                }
                                'l18: {
                                    CreateApprenticeMon(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(((trainerId) as i32) as isize * 68),
                                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(1640))
                                        .cast::<u16>())
                                        .wrapping_offset(((18i32).wrapping_add(i)) as isize))
                                        .read()) as u8),
                                    );
                                    j = ((IsFrontierTrainerFemale(
                                        ((((trainerId) as i32).wrapping_add(400i32)) as u16),
                                    )) as i32);
                                    SetMonData(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((crate::c::div_i32(6i32, 2i32)).wrapping_add(i))
                                                as isize
                                                * 100,
                                        ),
                                        49i32,
                                        (&raw mut j).cast::<u8>(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RubyBattleTowerRecordToEmerald(src: *mut u8, dst: *mut u8) -> u32 {
    unsafe {
        let mut src = src;
        let mut dst = dst;
        let mut i: i32 = 0i32;
        let mut validMons: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((src).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset((i) as isize * 44))
                    .cast::<u16>())
                    .read())
                        != 0
                    {
                        validMons = (validMons).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if validMons != 3i32 {
            crate::c::memset(dst, 0i32, 236u32);
            return 0u32;
        } else {
            (dst).write((src).read());
            ((dst).wrapping_add(2).cast::<u16>())
                .write(((src).wrapping_add(2).cast::<u16>()).read());
            {
                i = 0i32;
                'l3: loop {
                    if !(((i) as u32) < crate::c::div_u32(150u32, 2u32)) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((&raw const sRubyFacilityClassToEmerald)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .read()) as i32)
                            == ((((src).wrapping_add(1)).read()) as i32)
                        {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i != 82i32 {
                ((dst).wrapping_add(1)).write(
                    ((((((&raw const sRubyFacilityClassToEmerald)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
            } else {
                ((dst).wrapping_add(1)).write(43u8);
            }
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((((dst).wrapping_add(4)).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l7: loop {
                    if !(i < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        ((((dst).wrapping_add(12)).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l9: loop {
                    if !(i < 6i32) {
                        break 'l9;
                    }
                    'l10: {
                        ((((dst).wrapping_add(16)).cast::<u16>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(16)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l11: loop {
                    if !(i < 6i32) {
                        break 'l11;
                    }
                    'l12: {
                        ((((dst).wrapping_add(28)).cast::<u16>()).wrapping_offset((i) as isize))
                            .write(
                                ((((&raw const sRecordTrainerSpeechWon)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l13: loop {
                    if !(i < 6i32) {
                        break 'l13;
                    }
                    'l14: {
                        ((((dst).wrapping_add(40)).cast::<u16>()).wrapping_offset((i) as isize))
                            .write(
                                ((((&raw const sRecordTrainerSpeechLost)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l15: loop {
                    if !(i < 3i32) {
                        break 'l15;
                    }
                    'l16: {
                        (((dst).wrapping_add(52)).cast::<u8>())
                            .wrapping_offset((i) as isize * 44)
                            .cast::<crate::c::Rec4<44>>()
                            .write_unaligned(
                                (((src).wrapping_add(28)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 44)
                                    .cast::<crate::c::Rec4<44>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            'l17: loop {
                'l18: {
                    {
                        let mut tmp: u32 = 0u32;
                        (&raw mut tmp).write_volatile(0u32);
                        'l19: loop {
                            'l20: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((dst).wrapping_add(52)).cast::<u8>()).wrapping_offset(132),
                                    (83886080u32
                                        | (crate::c::div_u32(
                                            44u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l19;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l17;
                }
            }
            CalcEmeraldBattleTowerChecksum(dst);
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EmeraldBattleTowerRecordToRuby(src: *mut u8, dst: *mut u8) -> u32 {
    unsafe {
        let mut src = src;
        let mut dst = dst;
        let mut i: i32 = 0i32;
        let mut validMons: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((src).wrapping_add(52)).cast::<u8>())
                        .wrapping_offset((i) as isize * 44))
                    .cast::<u16>())
                    .read())
                        != 0
                    {
                        validMons = (validMons).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if validMons != 3i32 {
            crate::c::memset(dst, 0i32, 164u32);
            return 0u32;
        } else {
            (dst).write((src).read());
            ((dst).wrapping_add(2).cast::<u16>())
                .write(((src).wrapping_add(2).cast::<u16>()).read());
            {
                i = 0i32;
                'l3: loop {
                    if !(((i) as u32) < crate::c::div_u32(150u32, 2u32)) {
                        break 'l3;
                    }
                    'l4: {
                        if ((((((((&raw const sRubyFacilityClassToEmerald)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            == ((((src).wrapping_add(1)).read()) as i32)
                        {
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i != 82i32 {
                ((dst).wrapping_add(1)).write(
                    (((((&raw const sRubyFacilityClassToEmerald)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 2))
                    .cast::<u8>())
                    .read(),
                );
            } else {
                ((dst).wrapping_add(1)).write(36u8);
            }
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 8i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((((dst).wrapping_add(4)).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l7: loop {
                    if !(i < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        ((((dst).wrapping_add(12)).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l9: loop {
                    if !(i < 6i32) {
                        break 'l9;
                    }
                    'l10: {
                        ((((dst).wrapping_add(16)).cast::<u16>()).wrapping_offset((i) as isize))
                            .write(
                                ((((src).wrapping_add(16)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l11: loop {
                    if !(i < 3i32) {
                        break 'l11;
                    }
                    'l12: {
                        (((dst).wrapping_add(28)).cast::<u8>())
                            .wrapping_offset((i) as isize * 44)
                            .cast::<crate::c::Rec4<44>>()
                            .write_unaligned(
                                (((src).wrapping_add(52)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 44)
                                    .cast::<crate::c::Rec4<44>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CalcRubyBattleTowerChecksum(dst);
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CalcApprenticeChecksum(apprentice: *mut u8) {
    unsafe {
        let mut apprentice = apprentice;
        let mut i: i32 = 0i32;
        ((apprentice).wrapping_add(64).cast::<u32>()).write(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(64u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = (apprentice).wrapping_add(64).cast::<u32>();
                    (__p1).write(((__p1).read()).wrapping_add(
                        (((apprentice).cast::<u32>()).wrapping_offset((i) as isize)).read(),
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearApprentice(apprentice: *mut u8) {
    unsafe {
        let mut apprentice = apprentice;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(68u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    (((apprentice).cast::<u32>()).wrapping_offset((i) as isize)).write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        ResetApprenticeStruct(apprentice);
    }
}
pub(crate) unsafe extern "C" fn ValidateApprenticesChecksums() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut data: *mut u32 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 68))
                    .cast::<u32>();
                    let mut checksum: u32 = 0u32;
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(64u32, 4u32)) {
                                break 'l3;
                            }
                            'l4: {
                                checksum = (checksum)
                                    .wrapping_add(((data).wrapping_offset((j) as isize)).read());
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(220))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 68))
                    .wrapping_add(64)
                    .cast::<u32>())
                    .read()
                        != checksum
                    {
                        ClearApprentice(
                            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 68),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleTowerTrainerLanguage(dst: *mut u8, trainerId: u16) {
    unsafe {
        let mut dst = dst;
        let mut trainerId = trainerId;
        if ((trainerId) as i32) == 500i32 {
            (dst).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        } else {
            if ((trainerId) as i32) < 300i32 {
                (dst).write(((&raw mut gGameLanguage).cast::<u8>()).read());
            } else {
                if ((trainerId) as i32) < 400i32 {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                        (dst).write(GetRecordedBattleRecordMixFriendLanguage());
                    } else {
                        (dst).write(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(236))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                            ))
                            .wrapping_add(228))
                            .read(),
                        );
                    }
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
                        (dst).write(GetRecordedBattleApprenticeLanguage());
                    } else {
                        (dst).write(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(220))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((trainerId) as i32).wrapping_sub(400i32)) as isize * 68,
                            ))
                            .wrapping_add(63))
                            .read(),
                        );
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFacilityPtrsGetLevel() -> u8 {
    unsafe {
        if ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32)
            == 2i32
        {
            return SetTentPtrsGetLevel();
        } else {
            ((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>())
                .write(((&raw const gBattleFrontierTrainers).cast::<u8>().cast_mut()).cast::<u8>());
            ((&raw mut gFacilityTrainerMons)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(((&raw const gBattleFrontierMons).cast::<u8>().cast_mut()).cast::<u8>());
            return GetFrontierEnemyMonLevel(
                (crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8),
            );
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierEnemyMonLevel(lvlMode: u8) -> u8 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut level: u8 = 0u8;
        'l1: {
            let __sw1 = ((lvlMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                level = 50u8;
                break 'l1;
            }
            if __sw1 == 1i32 {
                level = ((GetHighestLevelInPlayerParty()) as u8);
                if ((level) as i32) < 60i32 {
                    level = 60u8;
                }
                break 'l1;
            }
        }
        return level;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHighestLevelInPlayerParty() -> i32 {
    unsafe {
        let mut highestLevel: i32 = 0i32;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) != 0)
                        && (GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                            core::ptr::null_mut(),
                        ) != 412u32)
                    {
                        let mut level: i32 = ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            56i32,
                            core::ptr::null_mut(),
                        )) as i32);
                        if level > highestLevel {
                            highestLevel = level;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return highestLevel;
    }
}
pub(crate) unsafe extern "C" fn GetFrontierTrainerFixedIvs(trainerId: u16) -> u8 {
    unsafe {
        let mut trainerId = trainerId;
        let mut fixedIv: u8 = 0u8;
        if ((trainerId) as i32) <= 99i32 {
            fixedIv = 3u8;
        } else {
            if ((trainerId) as i32) <= 119i32 {
                fixedIv = 6u8;
            } else {
                if ((trainerId) as i32) <= 139i32 {
                    fixedIv = 9u8;
                } else {
                    if ((trainerId) as i32) <= 159i32 {
                        fixedIv = 12u8;
                    } else {
                        if ((trainerId) as i32) <= 179i32 {
                            fixedIv = 15u8;
                        } else {
                            if ((trainerId) as i32) <= 199i32 {
                                fixedIv = 18u8;
                            } else {
                                if ((trainerId) as i32) <= 219i32 {
                                    fixedIv = 21u8;
                                } else {
                                    fixedIv = 31u8;
                                }
                            }
                        }
                    }
                }
            }
        }
        return fixedIv;
    }
}
pub(crate) unsafe extern "C" fn GetBattleTentTrainerId() -> u16 {
    unsafe {
        let mut facility: u32 = ((VarGet(16591u16)) as u32);
        if facility == 2u32 {
            return ((crate::c::rem_i32(((Random()) as i32), 30i32)) as u16);
        } else {
            if facility == 3u32 {
                return ((crate::c::rem_i32(((Random()) as i32), 30i32)) as u16);
            } else {
                if facility == 4u32 {
                    return ((crate::c::rem_i32(((Random()) as i32), 30i32)) as u16);
                } else {
                    if facility == 0u32 {
                        return 0u16;
                    } else {
                        return 0u16;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn SetTentPtrsGetLevel() -> u8 {
    unsafe {
        let mut level: u8 = 30u8;
        let mut facility: u32 = ((VarGet(16591u16)) as u32);
        if facility == 4u32 {
            ((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).write(
                ((&raw const gSlateportBattleTentTrainers)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((&raw mut gFacilityTrainerMons)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(
                ((&raw const gSlateportBattleTentMons)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
        } else {
            if facility == 2u32 {
                ((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).write(
                    ((&raw const gVerdanturfBattleTentTrainers)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(
                    ((&raw const gVerdanturfBattleTentMons)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
            } else {
                if facility == 3u32 {
                    ((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).write(
                        ((&raw const gFallarborBattleTentTrainers)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((&raw mut gFacilityTrainerMons)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const gFallarborBattleTentMons)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                } else {
                    ((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).write(
                        ((&raw const gBattleFrontierTrainers).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ((&raw mut gFacilityTrainerMons)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(((&raw const gBattleFrontierMons).cast::<u8>().cast_mut()).cast::<u8>());
                }
            }
        }
        level = ((GetHighestLevelInPlayerParty()) as u8);
        if ((level) as i32) < 30i32 {
            level = 30u8;
        }
        return level;
    }
}
pub(crate) unsafe extern "C" fn SetNextBattleTentOpponent() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut trainerId: u16 = 0u16;
        'l1: loop {
            'l2: {
                trainerId = GetBattleTentTrainerId();
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i
                            < (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1638)
                            .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1640))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == ((trainerId) as i32)
                            {
                                break 'l3;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if !(i
                != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32))
            {
                break 'l1;
            }
        }
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(trainerId);
        SetBattleFacilityTrainerGfxId(
            ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
            0u8,
        );
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(1i32)
            < 3i32
        {
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1640))
            .cast::<u16>())
            .wrapping_offset(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32) as isize,
            ))
            .write(((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn FillTentTrainerParty_(
    trainerId: u16,
    firstMonId: u8,
    monCount: u8,
) {
    unsafe {
        let mut trainerId = trainerId;
        let mut firstMonId = firstMonId;
        let mut monCount = monCount;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut chosenMonIndices = crate::ffi::Align4([0u8; 8]);
        let mut friendship: u8 = 0u8;
        let mut level: u8 = SetTentPtrsGetLevel();
        let mut fixedIV: u8 = 0u8;
        let mut bfMonCount: u8 = 0u8;
        let mut monSet: *mut u16 = core::ptr::null_mut();
        let mut otID: u32 = 0u32;
        let mut monId: u16 = 0u16;
        monSet = (((((&raw mut gFacilityTrainers).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(
                ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) as isize * 52,
            ))
        .wrapping_add(48)
        .cast::<*mut u16>())
        .read();
        bfMonCount = 0u8;
        monId = ((monSet).wrapping_offset(((bfMonCount) as i32) as isize)).read();
        'l1: loop {
            if !(((monId) as i32) != 65535i32) {
                break 'l1;
            }
            bfMonCount = (bfMonCount).wrapping_add(1);
            monId = ((monSet).wrapping_offset(((bfMonCount) as i32) as isize)).read();
            if ((monId) as i32) == 65535i32 {
                break 'l1;
            }
        }
        i = 0i32;
        otID = ((((Random()) as i32) | (((Random()) as i32) << 16)) as u32);
        'l2: loop {
            if !(i != ((monCount) as i32)) {
                break 'l2;
            }
            let mut monId: u16 = ((monSet).wrapping_offset(
                (crate::c::rem_i32(((Random()) as i32), ((bfMonCount) as i32))) as isize,
            ))
            .read();
            {
                j = 0i32;
                'l3: loop {
                    if !(j < (i).wrapping_add(((firstMonId) as i32))) {
                        break 'l3;
                    }
                    'l4: {
                        if GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((j) as isize * 100),
                            11i32,
                            core::ptr::null_mut(),
                        ) == (((((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .cast::<u16>())
                        .read()) as u32)
                        {
                            break 'l3;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l2;
            }
            {
                j = 0i32;
                'l5: loop {
                    if !(j < (i).wrapping_add(((firstMonId) as i32))) {
                        break 'l5;
                    }
                    'l6: {
                        if (GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((j) as isize * 100),
                            12i32,
                            core::ptr::null_mut(),
                        ) != 0u32)
                            && (GetMonData3(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((j) as isize * 100),
                                12i32,
                                core::ptr::null_mut(),
                            ) == ((((((&raw const gBattleFrontierHeldItems)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                (((((((&raw mut gFacilityTrainerMons)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((monId) as i32) as isize * 16))
                                .wrapping_add(10))
                                .read()) as i32) as isize,
                            ))
                            .read()) as u32))
                        {
                            break 'l5;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != (i).wrapping_add(((firstMonId) as i32)) {
                continue 'l2;
            }
            {
                j = 0i32;
                'l7: loop {
                    if !(j < i) {
                        break 'l7;
                    }
                    'l8: {
                        if (((((&raw mut chosenMonIndices).cast::<u16>())
                            .wrapping_offset((j) as isize))
                        .read()) as i32)
                            == ((monId) as i32)
                        {
                            break 'l7;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            if j != i {
                continue 'l2;
            }
            (((&raw mut chosenMonIndices).cast::<u16>()).wrapping_offset((i) as isize))
                .write(monId);
            CreateMonWithEVSpreadNatureOTID(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .cast::<u16>())
                .read(),
                level,
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(12))
                .read(),
                fixedIV,
                (((((&raw mut gFacilityTrainerMons)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((monId) as i32) as isize * 16))
                .wrapping_add(11))
                .read(),
                otID,
            );
            friendship = 255u8;
            {
                j = 0i32;
                'l9: loop {
                    if !(j < 4i32) {
                        break 'l9;
                    }
                    'l10: {
                        SetMonMoveSlot(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((i).wrapping_add(((firstMonId) as i32))) as isize * 100,
                            ),
                            (((((((&raw mut gFacilityTrainerMons)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((monId) as i32) as isize * 16))
                            .wrapping_add(2))
                            .cast::<u16>())
                            .wrapping_offset((j) as isize))
                            .read(),
                            ((j) as u8),
                        );
                        if (((((((((&raw mut gFacilityTrainerMons)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((monId) as i32) as isize * 16))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset((j) as isize))
                        .read()) as i32)
                            == 218i32
                        {
                            friendship = 0u8;
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                32i32,
                &raw mut friendship,
            );
            SetMonData(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((i).wrapping_add(((firstMonId) as i32))) as isize * 100),
                12i32,
                ((((&raw const gBattleFrontierHeldItems)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((((&raw mut gFacilityTrainerMons)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((monId) as i32) as isize * 16))
                    .wrapping_add(10))
                    .read()) as i32) as isize,
                ))
                .cast::<u8>(),
            );
            i = (i).wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FacilityClassToGraphicsId(facilityClass: u8) -> u8 {
    unsafe {
        let mut facilityClass = facilityClass;
        let mut trainerObjectGfxId: u8 = 0u8;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(30u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const gTowerMaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(30u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerMaleTrainerGfxIds).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            return trainerObjectGfxId;
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(20u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    if ((((((&raw const gTowerFemaleFacilityClasses)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((facilityClass) as i32)
                    {
                        break 'l3;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as u32) != crate::c::div_u32(20u32, 1u32) {
            trainerObjectGfxId = ((((&raw const gTowerFemaleTrainerGfxIds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .read();
            return trainerObjectGfxId;
        } else {
            return 7u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateBattleTowerRecord(recordId: u8) -> u32 {
    unsafe {
        let mut recordId = recordId;
        let mut i: i32 = 0i32;
        let mut record: *mut u32 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(236))
        .cast::<u8>())
        .wrapping_offset(((recordId) as i32) as isize * 236))
        .cast::<u32>();
        let mut checksum: u32 = 0u32;
        let mut hasData: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(232u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    checksum =
                        (checksum).wrapping_add(((record).wrapping_offset((i) as isize)).read());
                    hasData = (hasData | ((record).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        if (checksum == 0u32) && (hasData == 0u32) {
            return 0u32;
        } else {
            if ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(236))
            .cast::<u8>())
            .wrapping_offset(((recordId) as i32) as isize * 236))
            .wrapping_add(232)
            .cast::<u32>())
            .read()
                != checksum
            {
                ClearBattleTowerRecord(
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(236))
                    .cast::<u8>())
                    .wrapping_offset(((recordId) as i32) as isize * 236),
                );
                return 0u32;
            } else {
                return 1u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetLinkBattleTowerEnemyPartyLevel() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 33554434u32) != 0 {
            let mut i: i32 = 0i32;
            let mut enemyLevel: u8 = SetFacilityPtrsGetLevel();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut species: u32 = GetMonData3(
                            ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            11i32,
                            core::ptr::null_mut(),
                        );
                        if (species) != 0 {
                            SetMonData(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                25i32,
                                (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                                        .wrapping_offset(((species) as i32) as isize * 28))
                                    .wrapping_add(19))
                                    .read()) as i32) as isize
                                        * 404,
                                ))
                                .cast::<u32>())
                                .wrapping_offset(((enemyLevel) as i32) as isize))
                                .cast::<u8>(),
                            );
                            CalculateMonStats(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
