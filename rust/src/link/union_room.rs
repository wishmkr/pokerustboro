//! Translated from `src/union_room.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sText_EmptyString sText_Colon sText_ID sText_PleaseStartOver sText_WirelessSearchCanceled sText_AwaitingCommunucation2 sText_AwaitingCommunication sText_AwaitingLinkPressStart sJPText_SingleBattle sJPText_DoubleBattle sJPText_MultiBattle sJPText_TradePokemon sJPText_Chat sJPText_DistWonderCard sJPText_DistWonderNews sJPText_DistMysteryEvent sJPText_HoldPokemonJump sJPText_HoldBerryCrush sJPText_HoldBerryPicking sJPText_HoldSpinTrade sJPText_HoldSpinShop sJPLinkGroupActionTexts sText_1PlayerNeeded sText_2PlayersNeeded sText_3PlayersNeeded sText_4PlayersNeeded sText_2PlayerMode sText_3PlayerMode sText_4PlayerMode sText_5PlayerMode sPlayersNeededOrModeTexts sText_BButtonCancel sJPText_SearchingForParticipants sText_PlayerContactedYouForXAccept sText_PlayerContactedYouShareX sText_PlayerContactedYouAddToMembers sText_AreTheseMembersOK sText_CancelModeWithTheseMembers sText_AnOKWasSentToPlayer sText_OtherTrainerUnavailableNow sText_CantTransmitTrainerTooFar sText_TrainersNotReadyYet sCantTransmitToTrainerTexts sText_ModeWithTheseMembersWillBeCanceled sText_MemberNoLongerAvailable sPlayerUnavailableTexts sText_TrainerAppearsUnavailable sText_PlayerSentBackOK sText_PlayerOKdRegistration sText_PlayerRepliedNo sText_AwaitingOtherMembers sText_QuitBeingMember sText_StoppedBeingMember sPlayerDisconnectedTexts sText_WirelessLinkEstablished sText_WirelessLinkDropped sText_LinkWithFriendDropped sText_PlayerRepliedNo2 sLinkDroppedTexts sText_DoYouWantXMode sText_DoYouWantXMode2 sDoYouWantModeTexts sText_CommunicatingPleaseWait sText_AwaitingPlayersResponseAboutTrade sText_Communicating sText_CommunicatingWithPlayer sText_PleaseWaitAWhile sCommunicatingWaitTexts sText_HiDoSomethingMale sText_HiDoSomethingFemale sText_HiDoSomethingAgainMale sText_HiDoSomethingAgainFemale sHiDoSomethingTexts sText_DoSomethingMale sText_DoSomethingFemale sText_DoSomethingAgainMale sText_DoSomethingAgainFemale sDoSomethingTexts sText_SomebodyHasContactedYou sText_PlayerHasContactedYou sPlayerContactedYouTexts sText_AwaitingResponseFromTrainer sText_AwaitingResponseFromPlayer sAwaitingResponseTexts sText_ShowTrainerCard sText_BattleChallenge sText_ChatInvitation sText_OfferToTradeMon sText_OfferToTradeEgg sText_ChatDropped sText_OfferDeclined1 sText_OfferDeclined2 sText_ChatEnded sInvitationTexts sText_JoinChatMale sText_PlayerJoinChatMale sText_JoinChatFemale sText_PlayerJoinChatFemale sJoinChatTexts sText_TrainerAppearsBusy sText_WaitForBattleMale sText_WaitForChatMale sText_ShowTrainerCardMale sText_WaitForBattleFemale sText_WaitForChatFemale sText_ShowTrainerCardFemale sText_WaitOrShowCardTexts sText_WaitForChatMale2 sText_DoneWaitingBattleMale sText_DoneWaitingChatMale sText_DoneWaitingBattleFemale sText_DoneWaitingChatFemale sText_TradeWillBeStarted sText_BattleWillBeStarted sText_EnteringChat sStartActivityTexts sText_BattleDeclinedMale sText_BattleDeclinedFemale sBattleDeclinedTexts sText_ShowTrainerCardDeclinedMale sText_ShowTrainerCardDeclinedFemale sShowTrainerCardDeclinedTexts sText_IfYouWantToDoSomethingMale sText_IfYouWantToDoSomethingFemale sIfYouWantToDoSomethingTexts sText_TrainerBattleBusy sText_NeedTwoMonsOfLevel30OrLower1 sText_NeedTwoMonsOfLevel30OrLower2 sText_DeclineChatMale stext_DeclineChatFemale sDeclineChatTexts sText_ChatDeclinedMale sText_ChatDeclinedFemale sChatDeclinedTexts sText_YoureToughMale sText_UsedGoodMoveMale sText_BattleSurpriseMale sText_SwitchedMonsMale sText_YoureToughFemale sText_UsedGoodMoveFemale sText_BattleSurpriseFemale sText_SwitchedMonsFemale sBattleReactionTexts sText_LearnedSomethingMale sText_ThatsFunnyMale sText_RandomChatMale1 sText_RandomChatMale2 sText_LearnedSomethingFemale sText_ThatsFunnyFemale sText_RandomChatFemale1 sText_RandomChatFemale2 sChatReactionTexts sText_ShowedTrainerCardMale1 sText_ShowedTrainerCardMale2 sText_ShowedTrainerCardFemale1 sText_ShowedTrainerCardFemale2 sTrainerCardReactionTexts sText_MaleTraded1 sText_MaleTraded2 sText_FemaleTraded1 sText_FemaleTraded2 sTradeReactionTexts sText_XCheckedTradingBoard sText_RegisterMonAtTradingBoard sText_TradingBoardInfo sText_ThankYouForRegistering sText_NobodyHasRegistered sText_ChooseRequestedMonType sText_WhichMonWillYouOffer sText_RegistrationCanceled sText_RegistrationCompleted sText_TradeCanceled sText_CancelRegistrationOfMon sText_CancelRegistrationOfEgg sText_RegistrationCanceled2 sText_TradeTrainersWillBeListed sText_ChooseTrainerToTradeWith2 sText_AskTrainerToMakeTrade sText_AwaitingResponseFromTrainer2 sText_NotRegisteredAMonForTrade sText_DontHaveTypeTrainerWants sText_DontHaveEggTrainerWants sText_PlayerCantTradeForYourMon sText_CantTradeForPartnersMon sCantTradeMonTexts sText_TradeOfferRejected sText_EggTrade sText_ChooseJoinCancel sText_ChooseTrainer sText_ChooseTrainerSingleBattle sText_ChooseTrainerDoubleBattle sText_ChooseLeaderMultiBattle sText_ChooseTrainerToTradeWith sText_ChooseTrainerToShareWonderCards sText_ChooseTrainerToShareWonderNews sText_ChooseLeaderPokemonJump sText_ChooseLeaderBerryCrush sText_ChooseLeaderBerryPicking sText_ChooseLeaderBerryBlender sText_ChooseLeaderRecordCorner sText_ChooseLeaderCoolContest sText_ChooseLeaderBeautyContest sText_ChooseLeaderCuteContest sText_ChooseLeaderSmartContest sText_ChooseLeaderToughContest sText_ChooseLeaderBattleTowerLv50 sText_ChooseLeaderBattleTowerOpenLv sChooseTrainerTexts sText_SearchingForWirelessSystemWait sText_MustHaveTwoMonsForDoubleBattle sText_AwaitingPlayersResponse sText_PlayerHasBeenAskedToRegisterYouPleaseWait sText_AwaitingResponseFromWirelessSystem sText_PleaseWaitForOtherTrainersToGather sText_NoCardsSharedRightNow sText_NoNewsSharedRightNow sNoWonderSharedTexts sText_Battle sText_Chat2 sText_Greetings sText_Exit sText_Exit2 sText_Info sText_NameWantedOfferLv sText_SingleBattle sText_DoubleBattle sText_MultiBattle sText_PokemonTrades sText_Chat sText_Cards sText_WonderCards sText_WonderNews sText_PokemonJump sText_BerryCrush sText_BerryPicking sText_Search sText_BerryBlender sText_RecordCorner sText_CoolContest sText_BeautyContest sText_CuteContest sText_SmartContest sText_ToughContest sText_BattleTowerLv50 sText_BattleTowerOpenLv sText_ItsNormalCard sText_ItsBronzeCard sText_ItsCopperCard sText_ItsSilverCard sText_ItsGoldCard sCardColorTexts sText_TrainerCardInfoPage1 sText_TrainerCardInfoPage2 sText_GladToMeetYouMale sText_GladToMeetYouFemale sGladToMeetYouTexts sText_FinishedCheckingPlayersTrainerCard sLinkGroupActivityNameTexts sWindowTemplate_BButtonCancel sLinkGroupToActivityAndCapacity sWindowTemplate_PlayerList sWindowTemplate_5PlayerList sWindowTemplate_NumPlayerMode sPossibleGroupMembersListMenuItems sListMenuTemplate_PossibleGroupMembers sWindowTemplate_GroupList sWindowTemplate_PlayerNameAndId sUnionRoomGroupsMenuItems sListMenuTemplate_UnionRoomGroups sWindowTemplate_InviteToActivity sInviteToActivityMenuItems sListMenuTemplate_InviteToActivity sWindowTemplate_RegisterForTrade sRegisterForTradeListMenuItems sListMenuTemplate_RegisterForTrade sWindowTemplate_TradingBoardRequestType sTradingBoardTypes sMenuTemplate_TradingBoardRequestType sWindowTemplate_TradingBoardHeader sWindowTemplate_TradingBoardMain sTradeBoardListMenuItems sTradeBoardListMenuTemplate sWindowTemplate_Unused sEmptyListMenuItems sEmptyListMenuTemplate sUnionRoomPlayer_DummyRfu sAcceptedActivityIds_SingleBattle sAcceptedActivityIds_DoubleBattle sAcceptedActivityIds_MultiBattle sAcceptedActivityIds_Trade sAcceptedActivityIds_PokemonJump sAcceptedActivityIds_BerryCrush sAcceptedActivityIds_BerryPicking sAcceptedActivityIds_WonderCard sAcceptedActivityIds_WonderNews sAcceptedActivityIds_Resume sAcceptedActivityIds_Init sAcceptedActivityIds_Unk11 sAcceptedActivityIds_RecordCorner sAcceptedActivityIds_BerryBlender sAcceptedActivityIds_CoolContest sAcceptedActivityIds_BeautyContest sAcceptedActivityIds_CuteContest sAcceptedActivityIds_SmartContest sAcceptedActivityIds_ToughContest sAcceptedActivityIds_BattleTower sAcceptedActivityIds_BattleTowerOpen sAcceptedActivityIds sLinkGroupToURoomActivity

/// `__typeof__(sWirelessLinkMain)`
#[repr(C)]
#[derive(Clone, Copy)]
pub union sWirelessLinkMain_t {
    pub leader: *mut WirelessLink_Leader,
    pub group: *mut WirelessLink_Group,
    pub uRoom: *mut WirelessLink_URoom,
}

unsafe impl Sync for sWirelessLinkMain_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sWirelessLinkMain_t>() == 4);
};

const LG_STATE_ASK_JOIN_GROUP: u8 = 5;
const LG_STATE_ASK_LEAVE_GROUP: u8 = 7;
const LG_STATE_ASK_LEAVE_GROUP_HANDLE_INPUT: u8 = 8;
const LG_STATE_CANCELED: u8 = 11;
const LG_STATE_CANCEL_CHOOSE_LEADER: u8 = 10;
const LG_STATE_CHOOSE_LEADER_HANDLE_INPUT: u8 = 3;
const LG_STATE_CHOOSE_LEADER_MSG: u8 = 1;
const LG_STATE_DISCONNECTED: u8 = 14;
const LG_STATE_INIT: u8 = 0;
const LG_STATE_INIT_WINDOWS: u8 = 2;
const LG_STATE_MAIN: u8 = 6;
const LG_STATE_READY_START_ACTIVITY: u8 = 20;
const LG_STATE_RETRY_CONNECTION: u8 = 15;
const LG_STATE_RFU_ERROR: u8 = 12;
const LG_STATE_RFU_ERROR_SHUTDOWN: u8 = 13;
const LG_STATE_SHUTDOWN: u8 = 23;
const LG_STATE_START_ACTIVITY: u8 = 21;
const LG_STATE_TRADE_NOT_READY: u8 = 18;
const LG_STATE_TRADE_NOT_READY_RETRY: u8 = 19;
const LG_STATE_WAIT_LEAVE_GROUP: u8 = 9;
const LL_STATE_ACCEPTED_FINAL_MEMBER: u8 = 13;
const LL_STATE_ACCEPT_NEW_MEMBER_PROMPT: u32 = 7;
const LL_STATE_ACCEPT_NEW_MEMBER_PROMPT_HANDLE_INPUT: u8 = 11;
const LL_STATE_AWAIT_PLAYERS: u8 = 6;
const LL_STATE_CANCEL_PROMPT: u8 = 19;
const LL_STATE_CANCEL_PROMPT_HANDLE_INPUT: u8 = 20;
const LL_STATE_CANCEL_WITH_MSG: u8 = 30;
const LL_STATE_CONFIRMED_MEMBERS: u8 = 17;
const LL_STATE_FAILED: u8 = 24;
const LL_STATE_FINAL_MEMBER_CHECK: u8 = 18;
const LL_STATE_GET_AWAITING_PLAYERS_TEXT: u8 = 4;
const LL_STATE_INIT: u8 = 0;
const LL_STATE_INIT2: u8 = 3;
const LL_STATE_MEMBERS_OK_PROMPT: u8 = 15;
const LL_STATE_MEMBERS_OK_PROMPT_HANDLE_INPUT: u8 = 16;
const LL_STATE_MEMBER_DISCONNECTED: u8 = 29;
const LL_STATE_MEMBER_LEFT: u32 = 10;
const LL_STATE_PRINT_AWAITING_PLAYERS: u8 = 5;
const LL_STATE_RETRY: u8 = 22;
const LL_STATE_SHUTDOWN_AND_FAIL: u8 = 23;
const LL_STATE_SHUTDOWN_AND_RETRY: u8 = 21;
const LL_STATE_TRY_START_ACTIVITY: u8 = 26;
const LL_STATE_UPDATE_AFTER_JOIN_REQUEST: u8 = 12;
const LL_STATE_WAIT_AND_CONFIRM_MEMBERS: u8 = 14;
const LL_STATE_WAIT_DISCONNECT_CHILD: u8 = 9;
const PLIST_CONTACTED: u8 = 4;
const PLIST_NEW_PLAYER: u8 = 1;
const PLIST_NONE: i32 = 0;
const PLIST_RECENT_UPDATE: i32 = 2;
const PLIST_UNUSED: u8 = 3;
const URTRADE_STATE_NONE: u16 = 0;
const URTRADE_STATE_OFFERING: u16 = 2;
const URTRADE_STATE_REGISTERING: u16 = 1;
const UR_COLOR_CANCEL: u8 = 4;
const UR_COLOR_DEFAULT: u8 = 0;
const UR_COLOR_GREEN: u8 = 2;
const UR_COLOR_RED: u8 = 1;
const UR_COLOR_TRADE_BOARD_OTHER: u8 = 6;
const UR_COLOR_TRADE_BOARD_SELF: u8 = 5;
const UR_COLOR_WHITE: u8 = 3;
const UR_STATE_ACCEPT_CHAT_REQUEST: u8 = 22;
const UR_STATE_CANCEL_ACTIVITY_LINK_ERROR: u32 = 30;
const UR_STATE_CANCEL_REGISTRATION: u8 = 56;
const UR_STATE_CANCEL_REGISTRATION_PROMPT: u32 = 44;
const UR_STATE_CANCEL_REQUEST_PRINT_MSG: u8 = 36;
const UR_STATE_CANCEL_REQUEST_RESTART_LINK: u32 = 37;
const UR_STATE_CHECK_SELECTING_MON: u8 = 3;
const UR_STATE_CHECK_TRADING_BOARD: u8 = 45;
const UR_STATE_COMMUNICATING_WAIT_FOR_DATA: u8 = 38;
const UR_STATE_DECLINE_ACTIVITY_REQUEST: u8 = 10;
const UR_STATE_DO_SOMETHING_PROMPT: u8 = 5;
const UR_STATE_DO_SOMETHING_PROMPT_2: u8 = 7;
const UR_STATE_HANDLE_ACTIVITY_REQUEST: u32 = 9;
const UR_STATE_HANDLE_CONTACT_DATA: u8 = 34;
const UR_STATE_HANDLE_DO_SOMETHING_PROMPT_INPUT: u32 = 6;
const UR_STATE_INIT: u8 = 0;
const UR_STATE_INIT_LINK: u8 = 2;
const UR_STATE_INIT_OBJECTS: u8 = 1;
const UR_STATE_INTERACT_WITH_ATTENDANT: u8 = 42;
const UR_STATE_INTERACT_WITH_PLAYER: u8 = 24;
const UR_STATE_MAIN: u8 = 4;
const UR_STATE_PLAYER_CONTACTED_YOU: u8 = 11;
const UR_STATE_PRINT_AND_EXIT: u8 = 26;
const UR_STATE_PRINT_CARD_INFO: u8 = 40;
const UR_STATE_PRINT_CONTACT_MSG: u8 = 33;
const UR_STATE_PRINT_MSG: u8 = 8;
const UR_STATE_PRINT_START_ACTIVITY_MSG: u8 = 13;
const UR_STATE_RECV_ACTIVITY_REQUEST: u8 = 35;
const UR_STATE_RECV_CONTACT_DATA: u8 = 12;
const UR_STATE_RECV_JOIN_CHAT_REQUEST: u32 = 19;
const UR_STATE_REGISTER_COMPLETE: u8 = 55;
const UR_STATE_REGISTER_PROMPT: u8 = 43;
const UR_STATE_REGISTER_PROMPT_HANDLE_INPUT: u8 = 47;
const UR_STATE_REGISTER_REQUEST_TYPE: u32 = 52;
const UR_STATE_REGISTER_SELECT_MON: u8 = 54;
const UR_STATE_REGISTER_SELECT_MON_FADE: u32 = 53;
const UR_STATE_REQUEST_DECLINED: u8 = 32;
const UR_STATE_SEND_ACTIVITY_REQUEST: u8 = 27;
const UR_STATE_SEND_TRADE_REQUST: u32 = 31;
const UR_STATE_START_ACTIVITY: u8 = 18;
const UR_STATE_START_ACTIVITY_FADE: u8 = 17;
const UR_STATE_START_ACTIVITY_FREE_UROOM: u8 = 16;
const UR_STATE_START_ACTIVITY_LINK: u32 = 14;
const UR_STATE_START_ACTIVITY_WAIT_FOR_LINK: u8 = 15;
const UR_STATE_TRADE_OFFER_MON: u8 = 51;
const UR_STATE_TRADE_PROMPT: u32 = 49;
const UR_STATE_TRADE_SELECT_MON: u8 = 50;
const UR_STATE_TRADING_BOARD_HANDLE_INPUT: u8 = 48;
const UR_STATE_TRADING_BOARD_LOAD: u8 = 46;
const UR_STATE_TRAINER_APPEARS_BUSY: u8 = 28;
const UR_STATE_TRY_ACCEPT_CHAT_REQUEST: u8 = 21;
const UR_STATE_TRY_ACCEPT_CHAT_REQUEST_DELAY: u8 = 20;
const UR_STATE_TRY_COMMUNICATING: u8 = 25;
const UR_STATE_WAIT_FINISH_READING_CARD: u8 = 41;
const UR_STATE_WAIT_FOR_CONTACT_DATA: u8 = 39;
const UR_STATE_WAIT_FOR_RESPONSE_TO_REQUEST: u8 = 29;
const UR_STATE_WAIT_FOR_START_MENU: u8 = 23;

static sAcceptedActivityIds: Table<CArray<*mut u8, 22>> =
    Table((&raw const crate::data::union_room::sAcceptedActivityIds).cast());
static sAwaitingResponseTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sAwaitingResponseTexts).cast());
static sBattleDeclinedTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sBattleDeclinedTexts).cast());
static sBattleReactionTexts: Table<CArray<CArray<*mut u8, 4>, 2>> =
    Table((&raw const crate::data::union_room::sBattleReactionTexts).cast());
static sCantTransmitToTrainerTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sCantTransmitToTrainerTexts).cast());
static sCardColorTexts: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::union_room::sCardColorTexts).cast());
static sChatDeclinedTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sChatDeclinedTexts).cast());
static sChatReactionTexts: Table<CArray<CArray<*mut u8, 4>, 2>> =
    Table((&raw const crate::data::union_room::sChatReactionTexts).cast());
static sChooseTrainerTexts: Table<CArray<*mut u8, 22>> =
    Table((&raw const crate::data::union_room::sChooseTrainerTexts).cast());
static sCommunicatingWaitTexts: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::union_room::sCommunicatingWaitTexts).cast());
static sDeclineChatTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sDeclineChatTexts).cast());
static sGladToMeetYouTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sGladToMeetYouTexts).cast());
static sHiDoSomethingTexts: Table<CArray<CArray<*mut u8, 2>, 2>> =
    Table((&raw const crate::data::union_room::sHiDoSomethingTexts).cast());
static sIfYouWantToDoSomethingTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sIfYouWantToDoSomethingTexts).cast());
static sJoinChatTexts: Table<CArray<CArray<*mut u8, 2>, 2>> =
    Table((&raw const crate::data::union_room::sJoinChatTexts).cast());
static sLinkDroppedTexts: Table<CArray<*mut u8, 10>> =
    Table((&raw const crate::data::union_room::sLinkDroppedTexts).cast());
static sLinkGroupActivityNameTexts: Table<CArray<*mut u8, 29>> =
    Table((&raw const crate::data::union_room::sLinkGroupActivityNameTexts).cast());
static sLinkGroupToActivityAndCapacity: Table<CArray<u32, 22>> =
    Table((&raw const crate::data::union_room::sLinkGroupToActivityAndCapacity).cast());
static sLinkGroupToURoomActivity: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::union_room::sLinkGroupToURoomActivity).cast());
static sListMenuTemplate_InviteToActivity: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sListMenuTemplate_InviteToActivity).cast());
static sListMenuTemplate_PossibleGroupMembers: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sListMenuTemplate_PossibleGroupMembers).cast());
static sListMenuTemplate_RegisterForTrade: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sListMenuTemplate_RegisterForTrade).cast());
static sListMenuTemplate_UnionRoomGroups: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sListMenuTemplate_UnionRoomGroups).cast());
static sMenuTemplate_TradingBoardRequestType: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sMenuTemplate_TradingBoardRequestType).cast());
static sNoWonderSharedTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sNoWonderSharedTexts).cast());
static sPlayerContactedYouTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sPlayerContactedYouTexts).cast());
static sPlayerDisconnectedTexts: Table<CArray<*mut u8, 10>> =
    Table((&raw const crate::data::union_room::sPlayerDisconnectedTexts).cast());
static sPlayerUnavailableTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sPlayerUnavailableTexts).cast());
static sPlayersNeededOrModeTexts: Table<CArray<CArray<*mut u8, 5>, 5>> =
    Table((&raw const crate::data::union_room::sPlayersNeededOrModeTexts).cast());
static sShowTrainerCardDeclinedTexts: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::union_room::sShowTrainerCardDeclinedTexts).cast());
static sStartActivityTexts: Table<CArray<CArray<CArray<*mut u8, 3>, 2>, 2>> =
    Table((&raw const crate::data::union_room::sStartActivityTexts).cast());
static sText_AnOKWasSentToPlayer: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::union_room::sText_AnOKWasSentToPlayer).cast());
static sText_AreTheseMembersOK: Table<CArray<u8, 26>> =
    Table((&raw const crate::data::union_room::sText_AreTheseMembersOK).cast());
static sText_AskTrainerToMakeTrade: Table<CArray<u8, 42>> =
    Table((&raw const crate::data::union_room::sText_AskTrainerToMakeTrade).cast());
static sText_AwaitingCommunication: Table<CArray<u8, 48>> =
    Table((&raw const crate::data::union_room::sText_AwaitingCommunication).cast());
static sText_AwaitingLinkPressStart: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::union_room::sText_AwaitingLinkPressStart).cast());
static sText_AwaitingOtherMembers: Table<CArray<u8, 28>> =
    Table((&raw const crate::data::union_room::sText_AwaitingOtherMembers).cast());
static sText_AwaitingPlayersResponse: Table<CArray<u8, 24>> =
    Table((&raw const crate::data::union_room::sText_AwaitingPlayersResponse).cast());
static sText_AwaitingPlayersResponseAboutTrade: Table<CArray<u8, 40>> =
    Table((&raw const crate::data::union_room::sText_AwaitingPlayersResponseAboutTrade).cast());
static sText_AwaitingResponseFromWirelessSystem: Table<CArray<u8, 60>> =
    Table((&raw const crate::data::union_room::sText_AwaitingResponseFromWirelessSystem).cast());
static sText_BButtonCancel: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::union_room::sText_BButtonCancel).cast());
static sText_BattleChallenge: Table<CArray<u8, 82>> =
    Table((&raw const crate::data::union_room::sText_BattleChallenge).cast());
static sText_CancelModeWithTheseMembers: Table<CArray<u8, 35>> =
    Table((&raw const crate::data::union_room::sText_CancelModeWithTheseMembers).cast());
static sText_CancelRegistrationOfEgg: Table<CArray<u8, 37>> =
    Table((&raw const crate::data::union_room::sText_CancelRegistrationOfEgg).cast());
static sText_CancelRegistrationOfMon: Table<CArray<u8, 43>> =
    Table((&raw const crate::data::union_room::sText_CancelRegistrationOfMon).cast());
static sText_ChatDropped: Table<CArray<u8, 28>> =
    Table((&raw const crate::data::union_room::sText_ChatDropped).cast());
static sText_ChatEnded: Table<CArray<u8, 21>> =
    Table((&raw const crate::data::union_room::sText_ChatEnded).cast());
static sText_ChatInvitation: Table<CArray<u8, 76>> =
    Table((&raw const crate::data::union_room::sText_ChatInvitation).cast());
static sText_ChooseJoinCancel: Table<CArray<u8, 27>> =
    Table((&raw const crate::data::union_room::sText_ChooseJoinCancel).cast());
static sText_ChooseRequestedMonType: Table<CArray<u8, 69>> =
    Table((&raw const crate::data::union_room::sText_ChooseRequestedMonType).cast());
static sText_ChooseTrainer: Table<CArray<u8, 25>> =
    Table((&raw const crate::data::union_room::sText_ChooseTrainer).cast());
static sText_Colon: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::union_room::sText_Colon).cast());
static sText_DontHaveEggTrainerWants: Table<CArray<u8, 38>> =
    Table((&raw const crate::data::union_room::sText_DontHaveEggTrainerWants).cast());
static sText_DontHaveTypeTrainerWants: Table<CArray<u8, 49>> =
    Table((&raw const crate::data::union_room::sText_DontHaveTypeTrainerWants).cast());
static sText_EggTrade: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::union_room::sText_EggTrade).cast());
static sText_FinishedCheckingPlayersTrainerCard: Table<CArray<u8, 40>> =
    Table((&raw const crate::data::union_room::sText_FinishedCheckingPlayersTrainerCard).cast());
static sText_ID: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::union_room::sText_ID).cast());
static sText_LinkWithFriendDropped: Table<CArray<u8, 44>> =
    Table((&raw const crate::data::union_room::sText_LinkWithFriendDropped).cast());
static sText_ModeWithTheseMembersWillBeCanceled: Table<CArray<u8, 52>> =
    Table((&raw const crate::data::union_room::sText_ModeWithTheseMembersWillBeCanceled).cast());
static sText_NameWantedOfferLv: Table<CArray<u8, 28>> =
    Table((&raw const crate::data::union_room::sText_NameWantedOfferLv).cast());
static sText_NeedTwoMonsOfLevel30OrLower1: Table<CArray<u8, 68>> =
    Table((&raw const crate::data::union_room::sText_NeedTwoMonsOfLevel30OrLower1).cast());
static sText_NeedTwoMonsOfLevel30OrLower2: Table<CArray<u8, 59>> =
    Table((&raw const crate::data::union_room::sText_NeedTwoMonsOfLevel30OrLower2).cast());
static sText_OfferDeclined1: Table<CArray<u8, 25>> =
    Table((&raw const crate::data::union_room::sText_OfferDeclined1).cast());
static sText_OfferDeclined2: Table<CArray<u8, 25>> =
    Table((&raw const crate::data::union_room::sText_OfferDeclined2).cast());
static sText_OfferToTradeEgg: Table<CArray<u8, 82>> =
    Table((&raw const crate::data::union_room::sText_OfferToTradeEgg).cast());
static sText_OfferToTradeMon: Table<CArray<u8, 116>> =
    Table((&raw const crate::data::union_room::sText_OfferToTradeMon).cast());
static sText_PlayerContactedYouAddToMembers: Table<CArray<u8, 38>> =
    Table((&raw const crate::data::union_room::sText_PlayerContactedYouAddToMembers).cast());
static sText_PlayerContactedYouForXAccept: Table<CArray<u8, 33>> =
    Table((&raw const crate::data::union_room::sText_PlayerContactedYouForXAccept).cast());
static sText_PlayerContactedYouShareX: Table<CArray<u8, 37>> =
    Table((&raw const crate::data::union_room::sText_PlayerContactedYouShareX).cast());
static sText_PlayerHasBeenAskedToRegisterYouPleaseWait: Table<CArray<u8, 60>> = Table(
    (&raw const crate::data::union_room::sText_PlayerHasBeenAskedToRegisterYouPleaseWait).cast(),
);
static sText_PlayerOKdRegistration: Table<CArray<u8, 39>> =
    Table((&raw const crate::data::union_room::sText_PlayerOKdRegistration).cast());
static sText_PlayerSentBackOK: Table<CArray<u8, 22>> =
    Table((&raw const crate::data::union_room::sText_PlayerSentBackOK).cast());
static sText_PleaseStartOver: Table<CArray<u8, 38>> =
    Table((&raw const crate::data::union_room::sText_PleaseStartOver).cast());
static sText_QuitBeingMember: Table<CArray<u8, 21>> =
    Table((&raw const crate::data::union_room::sText_QuitBeingMember).cast());
static sText_RegisterMonAtTradingBoard: Table<CArray<u8, 137>> =
    Table((&raw const crate::data::union_room::sText_RegisterMonAtTradingBoard).cast());
static sText_RegistrationCanceled: Table<CArray<u8, 33>> =
    Table((&raw const crate::data::union_room::sText_RegistrationCanceled).cast());
static sText_RegistrationCanceled2: Table<CArray<u8, 37>> =
    Table((&raw const crate::data::union_room::sText_RegistrationCanceled2).cast());
static sText_RegistrationCompleted: Table<CArray<u8, 34>> =
    Table((&raw const crate::data::union_room::sText_RegistrationCompleted).cast());
static sText_SearchingForWirelessSystemWait: Table<CArray<u8, 55>> =
    Table((&raw const crate::data::union_room::sText_SearchingForWirelessSystemWait).cast());
static sText_ShowTrainerCard: Table<CArray<u8, 91>> =
    Table((&raw const crate::data::union_room::sText_ShowTrainerCard).cast());
static sText_TradeCanceled: Table<CArray<u8, 30>> =
    Table((&raw const crate::data::union_room::sText_TradeCanceled).cast());
static sText_TradeOfferRejected: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::union_room::sText_TradeOfferRejected).cast());
static sText_TradingBoardInfo: Table<CArray<u8, 313>> =
    Table((&raw const crate::data::union_room::sText_TradingBoardInfo).cast());
static sText_TrainerAppearsBusy: Table<CArray<u8, 36>> =
    Table((&raw const crate::data::union_room::sText_TrainerAppearsBusy).cast());
static sText_TrainerBattleBusy: Table<CArray<u8, 69>> =
    Table((&raw const crate::data::union_room::sText_TrainerBattleBusy).cast());
static sText_TrainerCardInfoPage1: Table<CArray<u8, 61>> =
    Table((&raw const crate::data::union_room::sText_TrainerCardInfoPage1).cast());
static sText_TrainerCardInfoPage2: Table<CArray<u8, 56>> =
    Table((&raw const crate::data::union_room::sText_TrainerCardInfoPage2).cast());
static sText_WaitOrShowCardTexts: Table<CArray<CArray<*mut u8, 4>, 2>> =
    Table((&raw const crate::data::union_room::sText_WaitOrShowCardTexts).cast());
static sText_WhichMonWillYouOffer: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::union_room::sText_WhichMonWillYouOffer).cast());
static sText_WirelessLinkDropped: Table<CArray<u8, 57>> =
    Table((&raw const crate::data::union_room::sText_WirelessLinkDropped).cast());
static sText_WirelessLinkEstablished: Table<CArray<u8, 61>> =
    Table((&raw const crate::data::union_room::sText_WirelessLinkEstablished).cast());
static sText_WirelessSearchCanceled: Table<CArray<u8, 60>> =
    Table((&raw const crate::data::union_room::sText_WirelessSearchCanceled).cast());
static sText_XCheckedTradingBoard: Table<CArray<u8, 31>> =
    Table((&raw const crate::data::union_room::sText_XCheckedTradingBoard).cast());
static sTradeBoardListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::union_room::sTradeBoardListMenuTemplate).cast());
static sTradeReactionTexts: Table<CArray<CArray<*mut u8, 4>, 2>> =
    Table((&raw const crate::data::union_room::sTradeReactionTexts).cast());
static sTrainerCardReactionTexts: Table<CArray<CArray<*mut u8, 2>, 2>> =
    Table((&raw const crate::data::union_room::sTrainerCardReactionTexts).cast());
static sUnionRoomPlayer_DummyRfu: Table<RfuPlayerData> =
    Table((&raw const crate::data::union_room::sUnionRoomPlayer_DummyRfu).cast());
static sWindowTemplate_5PlayerList: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_5PlayerList).cast());
static sWindowTemplate_BButtonCancel: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_BButtonCancel).cast());
static sWindowTemplate_GroupList: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_GroupList).cast());
static sWindowTemplate_InviteToActivity: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_InviteToActivity).cast());
static sWindowTemplate_NumPlayerMode: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_NumPlayerMode).cast());
static sWindowTemplate_PlayerList: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_PlayerList).cast());
static sWindowTemplate_PlayerNameAndId: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_PlayerNameAndId).cast());
static sWindowTemplate_RegisterForTrade: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_RegisterForTrade).cast());
static sWindowTemplate_TradingBoardHeader: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_TradingBoardHeader).cast());
static sWindowTemplate_TradingBoardMain: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_TradingBoardMain).cast());
static sWindowTemplate_TradingBoardRequestType: Table<WindowTemplate> =
    Table((&raw const crate::data::union_room::sWindowTemplate_TradingBoardRequestType).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionRoomPlayerName: Aligned<CArray<u8, 12>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerCurrActivity: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerActivityGroupSize: u8 = 0;
pub(crate) static mut sWirelessLinkMain: sWirelessLinkMain_t = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRfuPartnerCompatibilityData: RfuGameCompatibilityData = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnionRoomOfferedSpecies: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnionRoomRequestedMonType: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionRoomTrade: UnionRoomTrade = unsafe { zeroed() };
pub(crate) static mut sLeader: *mut WirelessLink_Leader = null_mut();
pub(crate) static mut sGroup: *mut WirelessLink_Group = null_mut();
pub(crate) static mut sURoom: *mut WirelessLink_URoom = null_mut();

unsafe extern "C" {
    static mut gBattleTypeFlags: u32;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gBlockSendBuffer: CArray<u8, 256>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gFieldLinkPlayerCount: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: Main;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: CArray<CArray<u16, 8>, 5>;
    static mut gRfuLinkStatus: *mut RfuLinkStatus;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedOrderFromParty: CArray<u8, 4>;
    static mut gSelectedTradeMonPositions: CArray<u8, 2>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static mut gTradeMail: CArray<Mail, 6>;
    static mut gTrainerCards: CArray<TrainerCard, 4>;
    static gTrainerClassNames: CArray<CArray<u8, 13>, 0>;
    static gTypeNames: CArray<CArray<u8, 7>, 18>;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterForMessage_2(a0: u8);
    fn AddTextPrinterWithCustomSpeedForMessage(a0: u8, a1: u8);
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn AreBattleTowerLinkSpeciesSame(a0: *mut u16, a1: *mut u16) -> u32;
    fn ArePlayerFieldControlsLocked() -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlitMenuInfoIcon(a0: u8, a1: u8, a2: u16, a3: u16);
    fn BuildOamBuffer();
    fn CB2_LinkTrade();
    fn CB2_LoadMap();
    fn CB2_ReturnFromCableClubBattle();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldCableClub();
    fn CB2_StartCreateTradeMenu();
    fn CB2_UnionRoomBattle();
    fn ChooseMonForTradingBoard(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn CopyHostRfuGameDataAndUsername(a0: *mut RfuGameData, a1: *mut u8);
    fn CopyTrainerCardData(a0: *mut TrainerCard, a1: *mut TrainerCard, a2: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_RfuReconnectWithParent(a0: *mut u8, a1: u16);
    fn CreateUnionRoomPlayerSprites(a0: *mut u8, a1: i32);
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DestroyUnionRoomPlayerObjects();
    fn DestroyUnionRoomPlayerSprites(a0: *mut u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DisplayYesNoMenuDefaultYes();
    fn DoMysteryGiftYesNo(a0: *mut u8, a1: *mut u16, a2: u8, a3: *mut u8) -> i8;
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn EnterUnionRoomChat();
    fn EraseYesNoWindow();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptUnionRoom();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreezeObjects_WaitForPlayer();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetHostRfuGameData() -> *mut RfuGameData;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetLinkPlayerInfoFlags(a0: i32) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetMysteryGiftBaseBlock() -> u16;
    fn GetOtherPlayersInfoFlags();
    fn GetPartyMenuType() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetUnionRoomTrainerClass() -> u16;
    fn GetWonderCardFlagID() -> u16;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn HandleUnionRoomPlayerRefresh(a0: *mut WirelessLink_URoom);
    fn HasTrainerLeftPartnersList(a0: u16, a1: *mut u8) -> u32;
    fn HealPlayerParty();
    fn IncrementGameStat(a0: u8);
    fn InitChooseHalfPartyForBattle(a0: u8);
    fn InitUnionRoomPlayerObjects(a0: *mut UnionRoomObject) -> u8;
    fn InitializeRfuLinkManager_EnterUnionRoom();
    fn InitializeRfuLinkManager_JoinGroup();
    fn InitializeRfuLinkManager_LinkLeader(a0: u32);
    fn Intl_GetListMenuWidth(a0: *mut ListMenuTemplate) -> i32;
    fn IsLinkTaskFinished() -> u8;
    fn IsRfuCommunicatingWithAllChildren() -> u32;
    fn IsUnionRoomListenTaskActive() -> u32;
    fn LinkRfu_CreateConnectionAsParent();
    fn LinkRfu_Shutdown();
    fn LinkRfu_StopManagerAndFinalizeSlots();
    fn LinkRfu_StopManagerBeforeEnteringChat();
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenuLoadStdPalAt(a0: u8, a1: u8);
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LmanAcceptSlotFlagIsNotZero() -> u8;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadPlayerBag();
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn MG_AddMessageTextPrinter(a0: *mut u8);
    fn MG_DrawTextBorder(a0: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MysteryGift_DisableStats();
    fn MysteryGift_TryEnableStatsByFlagId(a0: u16) -> u32;
    fn OpenLink();
    fn PlayBattleBGM();
    fn PlaySE(a0: u16);
    fn PlayerHasMetTrainerBefore(a0: u16, a1: *mut u8) -> u32;
    fn PrintMysteryGiftMenuMessage(a0: *mut u8, a1: *mut u8) -> u32;
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RedrawListMenu(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDisconnectSlotByTrainerNameAndId(a0: *mut u8, a1: u16);
    fn ResetBlockReceivedFlags();
    fn ResetHostRfuGameData();
    fn RfuGetStatus() -> u8;
    fn RfuHasErrored() -> u32;
    fn RfuSetIgnoreError(a0: u32);
    fn RfuSetStatus(a0: u8, a1: u16);
    fn RfuTryDisconnectLeavingChildren() -> u32;
    fn Rfu_DisconnectPlayerById(a0: u32);
    fn Rfu_GetCompatiblePlayerData(a0: *mut RfuGameData, a1: *mut u8, a2: u8) -> u8;
    fn Rfu_GetWonderDistributorPlayerData(a0: *mut RfuGameData, a1: *mut u8, a2: u8) -> u8;
    fn Rfu_SendPacket(a0: *mut c_void);
    fn RunTasks();
    fn RunTextPrinters();
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn SaveLinkTrainerNames();
    fn SavePlayerParty();
    fn ScheduleUnionRoomPlayerRefresh(a0: *mut WirelessLink_URoom);
    fn ScriptContext_Enable();
    fn ScriptContext_IsEnabled() -> u8;
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SendLeaveGroupNotice();
    fn SendRfuStatusToPartner(a0: u8, a1: u16, a2: *mut u8);
    fn SetCableClubWarp() -> i32;
    fn SetCloseLinkCallback();
    fn SetDynamicWarpWithCoords(a0: i32, a1: i8, a2: i8, a3: i8, a4: i8, a5: i8);
    fn SetHostRfuGameData(a0: u8, a1: u32, a2: u32);
    fn SetHostRfuWonderFlags(a0: u32, a1: u32);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTilesAroundUnionRoomPlayersPassable();
    fn SetTradeBoardRegisteredMonInfo(a0: u32, a1: u32, a2: u32);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetWirelessCommType1();
    fn ShowTrainerCardInLink(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn StartBerryCrush(a0: Option<unsafe extern "C" fn()>);
    fn StartDodrioBerryPicking(a0: u16, a1: Option<unsafe extern "C" fn()>);
    fn StartPokemonJump(a0: u16, a1: Option<unsafe extern "C" fn()>);
    fn StopUnionRoomLinkManager();
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_PlayerName(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_ShowStartMenu(a0: u8);
    fn TrainerCard_GenerateCardForLinkPlayer(a0: *mut TrainerCard);
    fn TryConnectToUnionRoomParent(a0: *mut u8, a1: *mut RfuGameData, a2: u8);
    fn TryInteractWithUnionRoomMember(
        a0: *mut RfuPlayerList,
        a1: *mut i16,
        a2: *mut i16,
        a3: *mut u8,
    ) -> u32;
    fn UnionRoom_UnlockPlayerAndChatPartner();
    fn UnlockPlayerFieldControls();
    fn UpdateGameData_GroupLockedIn(a0: u8);
    fn UpdateGameData_SetActivity(a0: u8, a1: u32, a2: u32);
    fn UpdatePaletteFade() -> u8;
    fn UpdateUnionRoomMemberFacing(a0: u32, a1: u32, a2: *mut RfuPlayerList);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WaitRfuState(a0: u32) -> u32;
    fn WaitSendRfuStatusToPartner(a0: u16, a1: *mut u8) -> u32;
    fn WarpIntoMap();
}

pub(crate) unsafe extern "C" fn PrintNumPlayersWaitingForMsg(
    windowId: u8,
    capacityCode: u8,
    stringId: u8,
) {
    FillWindowPixelBuffer(windowId, 17);
    match (capacityCode as i32) << 8 {
        512 => {
            PrintUnionRoomText(
                windowId,
                FONT_NORMAL,
                sPlayersNeededOrModeTexts[0][stringId as i32 - 1],
                0,
                1,
                UR_COLOR_DEFAULT,
            );
        }
        1024 => {
            PrintUnionRoomText(
                windowId,
                FONT_NORMAL,
                sPlayersNeededOrModeTexts[1][stringId as i32 - 1],
                0,
                1,
                UR_COLOR_DEFAULT,
            );
        }
        9472 => {
            PrintUnionRoomText(
                windowId,
                FONT_NORMAL,
                sPlayersNeededOrModeTexts[2][stringId as i32 - 1],
                0,
                1,
                UR_COLOR_DEFAULT,
            );
        }
        13568 => {
            PrintUnionRoomText(
                windowId,
                FONT_NORMAL,
                sPlayersNeededOrModeTexts[3][stringId as i32 - 1],
                0,
                1,
                UR_COLOR_DEFAULT,
            );
        }
        9216 => {
            PrintUnionRoomText(
                windowId,
                FONT_NORMAL,
                sPlayersNeededOrModeTexts[4][stringId as i32 - 1],
                0,
                1,
                UR_COLOR_DEFAULT,
            );
        }
        _ => {}
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn PrintPlayerNameAndIdOnWindow(windowId: u8) {
    let mut text: CArray<u8, 30> = zeroed();
    let mut txtPtr: *mut u8 = null_mut();
    PrintUnionRoomText(
        windowId,
        FONT_NORMAL,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        0,
        1,
        UR_COLOR_DEFAULT,
    );
    txtPtr = StringCopy(text.as_mut_ptr(), sText_ID.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(
        txtPtr,
        ReadAsU16((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr()) as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        5,
    );
    PrintUnionRoomText(
        windowId,
        FONT_NORMAL,
        text.as_mut_ptr(),
        0,
        17,
        UR_COLOR_DEFAULT,
    );
}
pub(crate) unsafe extern "C" fn GetAwaitingCommunicationText(dst: *mut u8, activity: u8) {
    match activity {
        ACTIVITY_BATTLE_SINGLE
        | ACTIVITY_BATTLE_DOUBLE
        | ACTIVITY_BATTLE_MULTI
        | ACTIVITY_TRADE
        | ACTIVITY_POKEMON_JUMP
        | ACTIVITY_BERRY_CRUSH
        | ACTIVITY_BERRY_PICK
        | ACTIVITY_BATTLE_TOWER
        | ACTIVITY_BATTLE_TOWER_OPEN
        | ACTIVITY_RECORD_CORNER
        | ACTIVITY_BERRY_BLENDER
        | ACTIVITY_WONDER_CARD
        | ACTIVITY_WONDER_NEWS
        | ACTIVITY_CONTEST_COOL
        | ACTIVITY_CONTEST_BEAUTY
        | ACTIVITY_CONTEST_CUTE
        | ACTIVITY_CONTEST_SMART
        | ACTIVITY_CONTEST_TOUGH => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                sText_AwaitingCommunication.as_ptr().cast_mut(),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn IsActivityWithVariableGroupSize(activity: u32) -> u32 {
    match activity {
        9 | 10 | 11 | 15 | 16 | 23 | 24 | 25 | 26 | 27 => {
            return TRUE as u32;
        }
        _ => {
            return FALSE as u32;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBecomeLinkLeader() {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Leader = null_mut();
    taskId = CreateTask(Some(Task_TryBecomeLinkLeader), 0);
    sWirelessLinkMain.leader = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Leader;
        data
    };
    sLeader = data;
    (*data).state = LL_STATE_INIT;
    (*data).textState = 0;
    gSpecialVar_Result = LINKUP_ONGOING;
}
pub(crate) unsafe extern "C" fn Task_TryBecomeLinkLeader(taskId: u8) {
    let mut id: u32 = 0;
    let mut val: u32 = 0;
    let mut data: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    match (*data).state {
        LL_STATE_INIT => {
            if gSpecialVar_0x8004 == LINK_GROUP_BATTLE_TOWER
                && (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_OPEN
            {
                gSpecialVar_0x8004 += 1;
            }
            gPlayerCurrActivity = sLinkGroupToActivityAndCapacity[gSpecialVar_0x8004] as u8;
            sPlayerActivityGroupSize =
                (sLinkGroupToActivityAndCapacity[gSpecialVar_0x8004] >> 8) as u8;
            SetHostRfuGameData(gPlayerCurrActivity, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_LinkLeader(sPlayerActivityGroupSize as u32 & 0x0F);
            (*data).state = LL_STATE_INIT2;
        }
        LL_STATE_INIT2 => {
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(160) as *mut RfuPlayerList;
            (*data).playerListBackup = AllocZeroed(160) as *mut RfuPlayerList;
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYERS as u8,
            );
            CopyHostRfuGameDataAndUsername(
                &raw mut (*(*data).playerList).players[0].rfu.data,
                (*(*data).playerList).players[0].rfu.name.as_mut_ptr(),
            );
            (*(*data).playerList).players[0].timeoutCounter = 0;
            (*(*data).playerList).players[0].set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
            (*(*data).playerList).players[0].set_useRedText(0);
            (*(*data).playerList).players[0].newPlayerCountdown = 0;
            (*data).listenTaskId =
                CreateTask_ListenForCompatiblePartners((*data).incomingPlayerList, 0xFF);
            (*data).bButtonCancelWindowId =
                AddWindow((&raw const *sWindowTemplate_BButtonCancel).cast_mut()) as u8;
            match sPlayerActivityGroupSize as i32 & 0x0F {
                2 | 3 | 4 => {
                    (*data).listWindowId =
                        AddWindow((&raw const *sWindowTemplate_PlayerList).cast_mut()) as u8;
                }
                5 => {
                    (*data).listWindowId =
                        AddWindow((&raw const *sWindowTemplate_5PlayerList).cast_mut()) as u8;
                }
                _ => {}
            }
            (*data).nPlayerModeWindowId =
                AddWindow((&raw const *sWindowTemplate_NumPlayerMode).cast_mut()) as u8;
            FillWindowPixelBuffer((*data).bButtonCancelWindowId, 34);
            PrintUnionRoomText(
                (*data).bButtonCancelWindowId,
                FONT_SMALL,
                sText_BButtonCancel.as_ptr().cast_mut(),
                8,
                1,
                UR_COLOR_CANCEL,
            );
            PutWindowTilemap((*data).bButtonCancelWindowId);
            CopyWindowToVram((*data).bButtonCancelWindowId, COPYWIN_GFX);
            DrawStdWindowFrame((*data).listWindowId, FALSE);
            gMultiuseListMenuTemplate = *sListMenuTemplate_PossibleGroupMembers;
            gMultiuseListMenuTemplate.windowId = (*data).listWindowId;
            (*data).listTaskId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            DrawStdWindowFrame((*data).nPlayerModeWindowId, FALSE);
            PutWindowTilemap((*data).nPlayerModeWindowId);
            CopyWindowToVram((*data).nPlayerModeWindowId, COPYWIN_GFX);
            CopyBgTilemapBufferToVram(0);
            (*data).playerCount = 1;
            (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
        }
        LL_STATE_GET_AWAITING_PLAYERS_TEXT => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                sLinkGroupActivityNameTexts[gPlayerCurrActivity],
            );
            if sPlayerActivityGroupSize >> 4 != 0 {
                if (*data).playerCount as i32 > (sPlayerActivityGroupSize >> 4) as i32 - 1
                    && sPlayerActivityGroupSize as i32 & 0x0F != 0
                {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        sText_AwaitingLinkPressStart.as_ptr().cast_mut(),
                    );
                } else {
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        sText_AwaitingCommunication.as_ptr().cast_mut(),
                    );
                }
            } else {
                GetAwaitingCommunicationText(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
            }
            PrintNumPlayersWaitingForMsg(
                (*data).nPlayerModeWindowId,
                sPlayerActivityGroupSize,
                (*data).playerCount,
            );
            (*data).state = LL_STATE_PRINT_AWAITING_PLAYERS;
        }
        LL_STATE_PRINT_AWAITING_PLAYERS => {
            if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                (*data).state = LL_STATE_AWAIT_PLAYERS;
            }
        }
        LL_STATE_AWAIT_PLAYERS => {
            Leader_SetStateIfMemberListChanged(
                data,
                LL_STATE_ACCEPT_NEW_MEMBER_PROMPT,
                LL_STATE_MEMBER_LEFT,
            );
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                if (*data).playerCount == 1 {
                    (*data).state = LL_STATE_SHUTDOWN_AND_FAIL;
                } else if sPlayerActivityGroupSize as i32 & 0xF0 != 0 {
                    (*data).state = LL_STATE_CANCEL_WITH_MSG;
                } else {
                    (*data).state = LL_STATE_CANCEL_PROMPT;
                }
            }
            if sPlayerActivityGroupSize >> 4 != 0
                && (*data).playerCount as i32 > (sPlayerActivityGroupSize >> 4) as i32 - 1
                && sPlayerActivityGroupSize as i32 & 0x0F != 0
                && IsRfuCommunicatingWithAllChildren() != 0
                && gMain.newKeys as i32 & START_BUTTON != 0
            {
                (*data).state = LL_STATE_MEMBERS_OK_PROMPT;
                LinkRfu_StopManagerAndFinalizeSlots();
            }
            if (*data).state == LL_STATE_AWAIT_PLAYERS && RfuTryDisconnectLeavingChildren() != 0 {
                (*data).state = LL_STATE_WAIT_DISCONNECT_CHILD;
            }
        }
        LL_STATE_WAIT_DISCONNECT_CHILD => {
            if RfuTryDisconnectLeavingChildren() == 0 {
                (*data).state = LL_STATE_AWAIT_PLAYERS;
                (*data).playerCount = LeaderPrunePlayerList((*data).playerList);
            }
        }
        10 => {
            id = (if gPlayerCurrActivity as i32 & 0x0F == 2 {
                1
            } else {
                0
            }) as u32;
            if PrintOnTextbox(&raw mut (*data).textState, sPlayerUnavailableTexts[id]) != 0 {
                (*data).playerCount = LeaderPrunePlayerList((*data).playerList);
                RedrawListMenu((*data).listTaskId);
                (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
            }
        }
        LL_STATE_MEMBER_DISCONNECTED => {
            id = (if sPlayerActivityGroupSize as i32 & 0x0F == 2 {
                0
            } else {
                1
            }) as u32;
            if PrintOnTextbox(&raw mut (*data).textState, sPlayerUnavailableTexts[id]) != 0 {
                (*data).state = LL_STATE_SHUTDOWN_AND_RETRY;
            }
        }
        7 => {
            if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                (*data).state = LL_STATE_ACCEPT_NEW_MEMBER_PROMPT_HANDLE_INPUT;
            }
        }
        LL_STATE_ACCEPT_NEW_MEMBER_PROMPT_HANDLE_INPUT => {
            match UnionRoomHandleYesNo(
                &raw mut (*data).textState,
                HasTrainerLeftPartnersList(
                    ReadAsU16(
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .data
                            .compatibility
                            .playerTrainerId
                            .as_mut_ptr(),
                    ),
                    (*(*data).playerList).players[(*data).playerCount]
                        .rfu
                        .name
                        .as_mut_ptr(),
                ),
            ) {
                0 => {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                    (*data).joinRequestAnswer = RFU_STATUS_JOIN_GROUP_OK;
                    SendRfuStatusToPartner(
                        (*data).joinRequestAnswer,
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                    );
                    (*data).state = LL_STATE_UPDATE_AFTER_JOIN_REQUEST;
                }
                1 | MENU_B_PRESSED => {
                    (*data).joinRequestAnswer = RFU_STATUS_JOIN_GROUP_NO;
                    SendRfuStatusToPartner(
                        (*data).joinRequestAnswer,
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                    );
                    (*data).state = LL_STATE_UPDATE_AFTER_JOIN_REQUEST;
                }
                -3 => {
                    (*data).state = LL_STATE_WAIT_DISCONNECT_CHILD;
                }
                _ => {}
            }
        }
        LL_STATE_UPDATE_AFTER_JOIN_REQUEST => {
            val = WaitSendRfuStatusToPartner(
                ReadAsU16(
                    (*(*data).playerList).players[(*data).playerCount]
                        .rfu
                        .data
                        .compatibility
                        .playerTrainerId
                        .as_mut_ptr(),
                ),
                (*(*data).playerList).players[(*data).playerCount]
                    .rfu
                    .name
                    .as_mut_ptr(),
            );
            if val == 1 {
                if (*data).joinRequestAnswer == RFU_STATUS_JOIN_GROUP_OK {
                    (*(*data).playerList).players[(*data).playerCount].newPlayerCountdown = 0;
                    RedrawListMenu((*data).listTaskId);
                    (*data).playerCount += 1;
                    if (*data).playerCount as i32 == sPlayerActivityGroupSize as i32 & 0x0F {
                        if sPlayerActivityGroupSize as i32 & 0xF0 != 0
                            || (*data).playerCount == RFU_CHILD_MAX
                        {
                            (*data).state = LL_STATE_MEMBERS_OK_PROMPT;
                        } else {
                            CopyAndTranslatePlayerName(
                                gStringVar1.as_mut_ptr(),
                                &raw mut (*(*data).playerList).players
                                    [(*data).playerCount as i32 - 1],
                            );
                            StringExpandPlaceholders(
                                gStringVar4.as_mut_ptr(),
                                sText_AnOKWasSentToPlayer.as_ptr().cast_mut(),
                            );
                            (*data).state = LL_STATE_ACCEPTED_FINAL_MEMBER;
                        }
                        LinkRfu_StopManagerAndFinalizeSlots();
                        PrintNumPlayersWaitingForMsg(
                            (*data).nPlayerModeWindowId,
                            sPlayerActivityGroupSize,
                            (*data).playerCount,
                        );
                    } else {
                        (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
                    }
                } else {
                    RequestDisconnectSlotByTrainerNameAndId(
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                    );
                    (*(*data).playerList).players[(*data).playerCount]
                        .set_groupScheduledAnim(UNION_ROOM_SPAWN_NONE);
                    LeaderPrunePlayerList((*data).playerList);
                    RedrawListMenu((*data).listTaskId);
                    (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
                }
                (*data).joinRequestAnswer = 0;
            } else if val == 2 {
                RfuSetStatus(0, 0);
                (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
            }
        }
        LL_STATE_ACCEPTED_FINAL_MEMBER => {
            if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                (*data).state = LL_STATE_WAIT_AND_CONFIRM_MEMBERS;
            }
        }
        LL_STATE_WAIT_AND_CONFIRM_MEMBERS => {
            if ({
                (*data).delayTimerAfterOk += 1;
                (*data).delayTimerAfterOk
            }) > 120
            {
                (*data).state = LL_STATE_CONFIRMED_MEMBERS;
            }
        }
        LL_STATE_MEMBERS_OK_PROMPT => {
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sText_AreTheseMembersOK.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = LL_STATE_MEMBERS_OK_PROMPT_HANDLE_INPUT;
            }
        }
        LL_STATE_MEMBERS_OK_PROMPT_HANDLE_INPUT => {
            match UnionRoomHandleYesNo(&raw mut (*data).textState, FALSE as u32) {
                0 => {
                    (*data).state = LL_STATE_CONFIRMED_MEMBERS;
                }
                1 | MENU_B_PRESSED => {
                    if sPlayerActivityGroupSize as i32 & 0xF0 != 0 {
                        (*data).state = LL_STATE_CANCEL_WITH_MSG;
                    } else {
                        (*data).state = LL_STATE_CANCEL_PROMPT;
                    }
                }
                _ => {}
            }
        }
        LL_STATE_CANCEL_PROMPT => {
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sText_CancelModeWithTheseMembers.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = LL_STATE_CANCEL_PROMPT_HANDLE_INPUT;
            }
        }
        LL_STATE_CANCEL_PROMPT_HANDLE_INPUT => {
            match UnionRoomHandleYesNo(&raw mut (*data).textState, FALSE as u32) {
                0 => {
                    (*data).state = LL_STATE_SHUTDOWN_AND_FAIL;
                }
                1 | MENU_B_PRESSED => {
                    if sPlayerActivityGroupSize as i32 & 0xF0 != 0 {
                        (*data).state = LL_STATE_MEMBERS_OK_PROMPT;
                    } else if (*data).playerCount as i32 == sPlayerActivityGroupSize as i32 & 0x0F {
                        (*data).state = LL_STATE_MEMBERS_OK_PROMPT;
                    } else {
                        (*data).state = LL_STATE_GET_AWAITING_PLAYERS_TEXT;
                    }
                }
                _ => {}
            }
        }
        LL_STATE_CONFIRMED_MEMBERS => {
            if Leader_SetStateIfMemberListChanged(
                data,
                LL_STATE_ACCEPT_NEW_MEMBER_PROMPT,
                LL_STATE_MEMBER_DISCONNECTED as u32,
            ) == 0
            {
                (*data).state = LL_STATE_FINAL_MEMBER_CHECK;
            }
        }
        LL_STATE_FINAL_MEMBER_CHECK => {
            if LmanAcceptSlotFlagIsNotZero() != 0 {
                if WaitRfuState(FALSE as u32) != 0 {
                    (*data).state = LL_STATE_TRY_START_ACTIVITY;
                } else {
                    if ({
                        (*data).memberConfirmTimeout += 1;
                        (*data).memberConfirmTimeout
                    }) > 300
                    {
                        (*data).state = LL_STATE_MEMBER_DISCONNECTED;
                        (*data).textState = 0;
                    }
                }
            } else {
                (*data).state = LL_STATE_MEMBER_DISCONNECTED;
                (*data).textState = 0;
            }
        }
        LL_STATE_CANCEL_WITH_MSG => {
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sText_ModeWithTheseMembersWillBeCanceled.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = LL_STATE_SHUTDOWN_AND_FAIL;
            }
        }
        LL_STATE_SHUTDOWN_AND_RETRY | LL_STATE_SHUTDOWN_AND_FAIL => {
            DestroyWirelessStatusIndicatorSprite();
            LinkRfu_Shutdown();
            Leader_DestroyResources(data);
            (*data).state += 1;
        }
        LL_STATE_FAILED => {
            ScriptContext_Enable();
            DestroyTask(taskId);
            gSpecialVar_Result = LINKUP_FAILED;
        }
        LL_STATE_RETRY => {
            ScriptContext_Enable();
            DestroyTask(taskId);
            gSpecialVar_Result = LINKUP_RETRY_ROLE_ASSIGN;
        }
        LL_STATE_TRY_START_ACTIVITY => {
            if RfuHasErrored() != 0 {
                (*data).state = LL_STATE_MEMBER_DISCONNECTED;
            } else {
                if gReceivedRemoteLinkPlayers != 0 {
                    if IsActivityWithVariableGroupSize(gPlayerCurrActivity as u32) != 0 {
                        GetOtherPlayersInfoFlags();
                    }
                    UpdateGameData_GroupLockedIn(TRUE);
                    CreateTask_RunScriptAndFadeToActivity();
                    Leader_DestroyResources(data);
                    DestroyTask(taskId);
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Leader_DestroyResources(data: *mut WirelessLink_Leader) {
    ClearWindowTilemap((*data).nPlayerModeWindowId);
    ClearStdWindowAndFrame((*data).nPlayerModeWindowId, FALSE);
    DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
    ClearWindowTilemap((*data).bButtonCancelWindowId);
    ClearStdWindowAndFrame((*data).listWindowId, FALSE);
    CopyBgTilemapBufferToVram(0);
    RemoveWindow((*data).nPlayerModeWindowId);
    RemoveWindow((*data).listWindowId);
    RemoveWindow((*data).bButtonCancelWindowId);
    DestroyTask((*data).listenTaskId);
    Free((*data).playerListBackup as *mut c_void);
    Free((*data).playerList as *mut c_void);
    Free((*data).incomingPlayerList as *mut c_void);
}
pub(crate) unsafe extern "C" fn Leader_GetAcceptNewMemberPrompt(dst: *mut u8, activity: u8) {
    match activity {
        ACTIVITY_BATTLE_SINGLE
        | ACTIVITY_BATTLE_DOUBLE
        | ACTIVITY_TRADE
        | ACTIVITY_BATTLE_TOWER_OPEN
        | ACTIVITY_BATTLE_TOWER => {
            StringExpandPlaceholders(dst, sText_PlayerContactedYouForXAccept.as_ptr().cast_mut());
        }
        ACTIVITY_WONDER_CARD | ACTIVITY_WONDER_NEWS => {
            StringExpandPlaceholders(dst, sText_PlayerContactedYouShareX.as_ptr().cast_mut());
        }
        ACTIVITY_BATTLE_MULTI
        | ACTIVITY_POKEMON_JUMP
        | ACTIVITY_BERRY_CRUSH
        | ACTIVITY_BERRY_PICK
        | ACTIVITY_RECORD_CORNER
        | ACTIVITY_BERRY_BLENDER
        | ACTIVITY_CONTEST_COOL
        | ACTIVITY_CONTEST_BEAUTY
        | ACTIVITY_CONTEST_CUTE
        | ACTIVITY_CONTEST_SMART
        | ACTIVITY_CONTEST_TOUGH => {
            StringExpandPlaceholders(
                dst,
                sText_PlayerContactedYouAddToMembers.as_ptr().cast_mut(),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetYouDeclinedTheOfferMessage(dst: *mut u8, activity: u8) {
    match activity {
        65 | 68 => {
            StringExpandPlaceholders(dst, sText_OfferDeclined1.as_ptr().cast_mut());
        }
        69 | 72 => {
            StringExpandPlaceholders(dst, sText_OfferDeclined2.as_ptr().cast_mut());
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetYouAskedToJoinGroupPleaseWaitMessage(
    dst: *mut u8,
    activity: u8,
) {
    match activity {
        ACTIVITY_BATTLE_SINGLE
        | ACTIVITY_BATTLE_DOUBLE
        | ACTIVITY_TRADE
        | ACTIVITY_BATTLE_TOWER
        | ACTIVITY_BATTLE_TOWER_OPEN
        | ACTIVITY_WONDER_CARD
        | ACTIVITY_WONDER_NEWS => {
            StringExpandPlaceholders(dst, sText_AwaitingPlayersResponse.as_ptr().cast_mut());
        }
        ACTIVITY_BATTLE_MULTI
        | ACTIVITY_POKEMON_JUMP
        | ACTIVITY_BERRY_CRUSH
        | ACTIVITY_BERRY_PICK
        | ACTIVITY_RECORD_CORNER
        | ACTIVITY_BERRY_BLENDER
        | ACTIVITY_CONTEST_COOL
        | ACTIVITY_CONTEST_BEAUTY
        | ACTIVITY_CONTEST_CUTE
        | ACTIVITY_CONTEST_SMART
        | ACTIVITY_CONTEST_TOUGH => {
            StringExpandPlaceholders(
                dst,
                sText_PlayerHasBeenAskedToRegisterYouPleaseWait
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetGroupLeaderSentAnOKMessage(dst: *mut u8, activity: u8) {
    match activity {
        ACTIVITY_BATTLE_SINGLE
        | ACTIVITY_BATTLE_DOUBLE
        | ACTIVITY_TRADE
        | ACTIVITY_BATTLE_TOWER
        | ACTIVITY_BATTLE_TOWER_OPEN
        | ACTIVITY_WONDER_CARD
        | ACTIVITY_WONDER_NEWS => {
            StringExpandPlaceholders(dst, sText_PlayerSentBackOK.as_ptr().cast_mut());
        }
        ACTIVITY_BATTLE_MULTI
        | ACTIVITY_POKEMON_JUMP
        | ACTIVITY_BERRY_CRUSH
        | ACTIVITY_BERRY_PICK
        | ACTIVITY_RECORD_CORNER
        | ACTIVITY_BERRY_BLENDER
        | ACTIVITY_CONTEST_COOL
        | ACTIVITY_CONTEST_BEAUTY
        | ACTIVITY_CONTEST_CUTE
        | ACTIVITY_CONTEST_SMART
        | ACTIVITY_CONTEST_TOUGH => {
            StringExpandPlaceholders(dst, sText_PlayerOKdRegistration.as_ptr().cast_mut());
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Leader_SetStateIfMemberListChanged(
    data: *mut WirelessLink_Leader,
    joinedState: u32,
    droppedState: u32,
) -> u8 {
    match LeaderUpdateGroupMembership((*data).playerList) {
        UNION_ROOM_SPAWN_IN => {
            PlaySE(SE_PC_LOGIN);
            RedrawListMenu((*data).listTaskId);
            CopyAndTranslatePlayerName(
                gStringVar2.as_mut_ptr(),
                &raw mut (*(*data).playerList).players[(*data).playerCount],
            );
            Leader_GetAcceptNewMemberPrompt(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
            (*data).state = joinedState as u8;
        }
        UNION_ROOM_SPAWN_OUT => {
            RfuSetStatus(0, 0);
            RedrawListMenu((*data).listTaskId);
            (*data).state = droppedState as u8;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ItemPrintFunc_PossibleGroupMembers(windowId: u8, id: u32, y: u8) {
    let mut data: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    let mut colorIdx: u8 = UR_COLOR_DEFAULT;
    match (*(*data).playerList).players[id].groupScheduledAnim() {
        UNION_ROOM_SPAWN_IN => {
            if (*(*data).playerList).players[id].newPlayerCountdown != 0 {
                colorIdx = UR_COLOR_GREEN;
            }
        }
        UNION_ROOM_SPAWN_OUT => {
            colorIdx = UR_COLOR_RED;
        }
        _ => {}
    }
    PrintGroupCandidateOnWindow(
        windowId,
        0,
        y,
        &raw mut (*(*data).playerList).players[id],
        colorIdx,
        id as u8,
    );
}
pub(crate) unsafe extern "C" fn LeaderUpdateGroupMembership(list: *mut RfuPlayerList) -> u8 {
    let mut data: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    let mut ret: u8 = UNION_ROOM_SPAWN_NONE;
    let mut i: u8 = 0;
    let mut id: i32 = 0;
    i = 1;
    while i < MAX_RFU_PLAYERS as u8 {
        let mut var: u16 = (*(*data).playerList).players[i].groupScheduledAnim() as u16;
        if var == UNION_ROOM_SPAWN_IN as u16 {
            id = GetNewIncomingPlayerId(
                &raw mut (*(*data).playerList).players[i],
                (*(*data).incomingPlayerList).players.as_mut_ptr(),
            ) as i32;
            if id != 0xFF {
                (*(*data).playerList).players[i].rfu =
                    (*(*data).incomingPlayerList).players[id].rfu;
                (*(*data).playerList).players[i].timeoutCounter = 1;
            } else {
                (*(*data).playerList).players[i].set_groupScheduledAnim(UNION_ROOM_SPAWN_OUT);
                ret = UNION_ROOM_SPAWN_OUT;
            }
        }
        i += 1;
    }
    id = 0;
    while id < RFU_CHILD_MAX as i32 {
        TryAddIncomingPlayerToList(
            (*(*data).playerList).players.as_mut_ptr(),
            &raw mut (*(*data).incomingPlayerList).players[id],
            MAX_RFU_PLAYERS as u8,
        );
        id += 1;
    }
    if ret != UNION_ROOM_SPAWN_OUT {
        id = 0;
        while id < MAX_RFU_PLAYERS {
            if (*(*data).playerList).players[id].newPlayerCountdown != 0 {
                ret = UNION_ROOM_SPAWN_IN;
            }
            id += 1;
        }
    }
    return ret;
}
pub(crate) unsafe extern "C" fn LeaderPrunePlayerList(list: *mut RfuPlayerList) -> u8 {
    let mut data: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    let mut copiedCount: u8 = 0;
    let mut i: i32 = 0;
    let mut playerCount: u8 = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS {
        (*(*data).playerListBackup).players[i] = (*(*data).playerList).players[i];
        i += 1;
    }
    copiedCount = 0;
    i = 0;
    while i < MAX_RFU_PLAYERS {
        if (*(*data).playerListBackup).players[i].groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
            (*(*data).playerList).players[copiedCount] = (*(*data).playerListBackup).players[i];
            copiedCount += 1;
        }
        i += 1;
    }
    playerCount = copiedCount;
    while copiedCount < MAX_RFU_PLAYERS as u8 {
        (*(*data).playerList).players[copiedCount].rfu = *sUnionRoomPlayer_DummyRfu;
        (*(*data).playerList).players[copiedCount].timeoutCounter = 0;
        (*(*data).playerList).players[copiedCount].set_groupScheduledAnim(UNION_ROOM_SPAWN_NONE);
        (*(*data).playerList).players[copiedCount].set_useRedText(FALSE);
        (*(*data).playerList).players[copiedCount].newPlayerCountdown = 0;
        copiedCount += 1;
    }
    i = 0;
    'l5: while i < MAX_RFU_PLAYERS {
        'l4: {
            if (*(*data).playerList).players[i].groupScheduledAnim() != UNION_ROOM_SPAWN_IN {
                break 'l4;
            }
            if (*(*data).playerList).players[i].newPlayerCountdown != 64 {
                break 'l4;
            }
            playerCount = i as u8;
            break 'l5;
        }
        i += 1;
    }
    return playerCount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryJoinLinkGroup() {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Group = null_mut();
    taskId = CreateTask(Some(Task_TryJoinLinkGroup), 0);
    sWirelessLinkMain.group = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Group;
        data
    };
    sGroup = data;
    (*data).state = LG_STATE_INIT;
    (*data).textState = 0;
    gSpecialVar_Result = LINKUP_ONGOING;
}
pub(crate) unsafe extern "C" fn Task_TryJoinLinkGroup(taskId: u8) {
    let mut id: i32 = 0;
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    match (*data).state {
        LG_STATE_INIT => {
            if gSpecialVar_0x8004 == LINK_GROUP_BATTLE_TOWER
                && (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_OPEN
            {
                gSpecialVar_0x8004 += 1;
            }
            gPlayerCurrActivity = sLinkGroupToURoomActivity[gSpecialVar_0x8004];
            SetHostRfuGameData(gPlayerCurrActivity, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_JoinGroup();
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(512) as *mut RfuPlayerList;
            (*data).state = LG_STATE_CHOOSE_LEADER_MSG;
        }
        LG_STATE_CHOOSE_LEADER_MSG => {
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sChooseTrainerTexts[gSpecialVar_0x8004],
            ) != 0
            {
                (*data).state = LG_STATE_INIT_WINDOWS;
            }
        }
        LG_STATE_INIT_WINDOWS => {
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYER_LIST_SIZE,
            );
            (*data).listenTaskId = CreateTask_ListenForCompatiblePartners(
                (*data).incomingPlayerList,
                gSpecialVar_0x8004 as u32,
            );
            (*data).bButtonCancelWindowId =
                AddWindow((&raw const *sWindowTemplate_BButtonCancel).cast_mut()) as u8;
            (*data).listWindowId =
                AddWindow((&raw const *sWindowTemplate_GroupList).cast_mut()) as u8;
            (*data).playerNameAndIdWindowId =
                AddWindow((&raw const *sWindowTemplate_PlayerNameAndId).cast_mut()) as u8;
            FillWindowPixelBuffer((*data).bButtonCancelWindowId, 34);
            PrintUnionRoomText(
                (*data).bButtonCancelWindowId,
                FONT_SMALL,
                sText_ChooseJoinCancel.as_ptr().cast_mut(),
                8,
                1,
                UR_COLOR_CANCEL,
            );
            PutWindowTilemap((*data).bButtonCancelWindowId);
            CopyWindowToVram((*data).bButtonCancelWindowId, COPYWIN_GFX);
            DrawStdWindowFrame((*data).listWindowId, FALSE);
            gMultiuseListMenuTemplate = *sListMenuTemplate_UnionRoomGroups;
            gMultiuseListMenuTemplate.windowId = (*data).listWindowId;
            (*data).listTaskId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            DrawStdWindowFrame((*data).playerNameAndIdWindowId, FALSE);
            PutWindowTilemap((*data).playerNameAndIdWindowId);
            PrintPlayerNameAndIdOnWindow((*data).playerNameAndIdWindowId);
            CopyWindowToVram((*data).playerNameAndIdWindowId, COPYWIN_GFX);
            CopyBgTilemapBufferToVram(0);
            (*data).leaderId = 0;
            (*data).state = LG_STATE_CHOOSE_LEADER_HANDLE_INPUT;
        }
        LG_STATE_CHOOSE_LEADER_HANDLE_INPUT => {
            id = GetNewLeaderCandidate() as i32;
            match id {
                1 => {
                    PlaySE(SE_PC_LOGIN);
                    RedrawListMenu((*data).listTaskId);
                }
                0 => {
                    id = ListMenu_ProcessInput((*data).listTaskId);
                    if gMain.newKeys as i32 & A_BUTTON != 0 && id != LIST_NOTHING_CHOSEN {
                        let mut activity: u32 =
                            (*(*data).playerList).players[id].rfu.data.activity() as u32;
                        if (*(*data).playerList).players[id].groupScheduledAnim()
                            == UNION_ROOM_SPAWN_IN
                            && (*(*data).playerList).players[id].rfu.data.startedActivity() == 0
                        {
                            let mut readyStatus: u32 =
                                IsTryingToTradeAcrossVersionTooSoon(data, id);
                            if readyStatus == UR_TRADE_READY {
                                AskToJoinRfuGroup(data, id);
                                (*data).state = LG_STATE_ASK_JOIN_GROUP;
                                PlaySE(SE_POKENAV_ON);
                            } else {
                                StringCopy(
                                    gStringVar4.as_mut_ptr(),
                                    sCantTransmitToTrainerTexts[readyStatus - 1],
                                );
                                (*data).state = LG_STATE_TRADE_NOT_READY;
                                PlaySE(SE_POKENAV_ON);
                            }
                        } else {
                            PlaySE(SE_WALL_HIT);
                        }
                    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                        (*data).state = LG_STATE_CANCEL_CHOOSE_LEADER;
                    }
                }
                _ => {
                    RedrawListMenu((*data).listTaskId);
                }
            }
        }
        LG_STATE_ASK_JOIN_GROUP => {
            GetYouAskedToJoinGroupPleaseWaitMessage(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
            if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                CopyAndTranslatePlayerName(
                    gStringVar1.as_mut_ptr(),
                    &raw mut (*(*data).playerList).players[(*data).leaderId],
                );
                (*data).state = LG_STATE_MAIN;
            }
        }
        LG_STATE_MAIN => {
            if gReceivedRemoteLinkPlayers != 0 {
                gPlayerCurrActivity = (*(*data).playerList).players[(*data).leaderId]
                    .rfu
                    .data
                    .activity();
                RfuSetStatus(0, 0);
                match gPlayerCurrActivity {
                    ACTIVITY_BATTLE_SINGLE
                    | ACTIVITY_BATTLE_DOUBLE
                    | ACTIVITY_BATTLE_MULTI
                    | ACTIVITY_TRADE
                    | ACTIVITY_CHAT
                    | ACTIVITY_POKEMON_JUMP
                    | ACTIVITY_BERRY_CRUSH
                    | ACTIVITY_BERRY_PICK
                    | ACTIVITY_SPIN_TRADE
                    | ACTIVITY_BATTLE_TOWER
                    | ACTIVITY_BATTLE_TOWER_OPEN
                    | ACTIVITY_RECORD_CORNER
                    | ACTIVITY_BERRY_BLENDER
                    | ACTIVITY_WONDER_CARD
                    | ACTIVITY_WONDER_NEWS
                    | ACTIVITY_CONTEST_COOL
                    | ACTIVITY_CONTEST_BEAUTY
                    | ACTIVITY_CONTEST_CUTE
                    | ACTIVITY_CONTEST_SMART
                    | ACTIVITY_CONTEST_TOUGH => {
                        (*data).state = LG_STATE_READY_START_ACTIVITY;
                        return;
                    }
                    _ => {}
                }
            }
            match RfuGetStatus() {
                RFU_STATUS_FATAL_ERROR => {
                    (*data).state = LG_STATE_RFU_ERROR;
                }
                RFU_STATUS_CONNECTION_ERROR | RFU_STATUS_JOIN_GROUP_NO | RFU_STATUS_LEAVE_GROUP => {
                    (*data).state = LG_STATE_DISCONNECTED;
                }
                RFU_STATUS_JOIN_GROUP_OK => {
                    GetGroupLeaderSentAnOKMessage(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
                    if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                        if gPlayerCurrActivity == ACTIVITY_BATTLE_TOWER
                            || gPlayerCurrActivity == ACTIVITY_BATTLE_TOWER_OPEN
                        {
                            RfuSetStatus(RFU_STATUS_ACK_JOIN_GROUP, 0);
                        } else {
                            RfuSetStatus(RFU_STATUS_WAIT_ACK_JOIN_GROUP, 0);
                            StringCopy(
                                gStringVar1.as_mut_ptr(),
                                sLinkGroupActivityNameTexts[gPlayerCurrActivity],
                            );
                            StringExpandPlaceholders(
                                gStringVar4.as_mut_ptr(),
                                sText_AwaitingOtherMembers.as_ptr().cast_mut(),
                            );
                        }
                    }
                }
                RFU_STATUS_WAIT_ACK_JOIN_GROUP => {
                    if (*data).delayBeforePrint > 240 {
                        if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0
                        {
                            RfuSetStatus(RFU_STATUS_ACK_JOIN_GROUP, 0);
                            (*data).delayBeforePrint = 0;
                        }
                    } else {
                        match gPlayerCurrActivity {
                            ACTIVITY_BATTLE_SINGLE
                            | ACTIVITY_BATTLE_DOUBLE
                            | ACTIVITY_TRADE
                            | ACTIVITY_BATTLE_TOWER
                            | ACTIVITY_BATTLE_TOWER_OPEN => {}
                            _ => {
                                (*data).delayBeforePrint += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
            if RfuGetStatus() == RFU_STATUS_OK && gMain.newKeys as i32 & B_BUTTON != 0 {
                (*data).state = LG_STATE_ASK_LEAVE_GROUP;
            }
        }
        LG_STATE_ASK_LEAVE_GROUP => {
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sText_QuitBeingMember.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = LG_STATE_ASK_LEAVE_GROUP_HANDLE_INPUT;
            }
        }
        LG_STATE_ASK_LEAVE_GROUP_HANDLE_INPUT => {
            match UnionRoomHandleYesNo(&raw mut (*data).textState, RfuGetStatus() as u32) {
                0 => {
                    SendLeaveGroupNotice();
                    (*data).state = LG_STATE_WAIT_LEAVE_GROUP;
                    RedrawListMenu((*data).listTaskId);
                }
                1 | MENU_B_PRESSED => {
                    (*data).state = LG_STATE_ASK_JOIN_GROUP;
                    RedrawListMenu((*data).listTaskId);
                }
                -3 => {
                    (*data).state = LG_STATE_MAIN;
                    RedrawListMenu((*data).listTaskId);
                }
                _ => {}
            }
        }
        LG_STATE_WAIT_LEAVE_GROUP => {
            if RfuGetStatus() != 0 {
                (*data).state = LG_STATE_MAIN;
            }
        }
        LG_STATE_CANCEL_CHOOSE_LEADER
        | LG_STATE_RFU_ERROR
        | LG_STATE_DISCONNECTED
        | LG_STATE_TRADE_NOT_READY
        | LG_STATE_READY_START_ACTIVITY => {
            ClearWindowTilemap((*data).playerNameAndIdWindowId);
            ClearStdWindowAndFrame((*data).playerNameAndIdWindowId, FALSE);
            DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
            ClearWindowTilemap((*data).bButtonCancelWindowId);
            ClearStdWindowAndFrame((*data).listWindowId, FALSE);
            CopyBgTilemapBufferToVram(0);
            RemoveWindow((*data).playerNameAndIdWindowId);
            RemoveWindow((*data).listWindowId);
            RemoveWindow((*data).bButtonCancelWindowId);
            DestroyTask((*data).listenTaskId);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            (*data).state += 1;
        }
        LG_STATE_RFU_ERROR_SHUTDOWN => {
            DestroyWirelessStatusIndicatorSprite();
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sPlayerDisconnectedTexts[RfuGetStatus()],
            ) != 0
            {
                gSpecialVar_Result = LINKUP_CONNECTION_ERROR;
                (*data).state = LG_STATE_SHUTDOWN;
            }
        }
        LG_STATE_CANCELED => {
            DestroyWirelessStatusIndicatorSprite();
            gSpecialVar_Result = LINKUP_FAILED;
            (*data).state = LG_STATE_SHUTDOWN;
        }
        LG_STATE_RETRY_CONNECTION => {
            DestroyWirelessStatusIndicatorSprite();
            if PrintOnTextbox(
                &raw mut (*data).textState,
                sPlayerDisconnectedTexts[RfuGetStatus()],
            ) != 0
            {
                gSpecialVar_Result = LINKUP_RETRY_ROLE_ASSIGN;
                (*data).state = LG_STATE_SHUTDOWN;
            }
        }
        LG_STATE_TRADE_NOT_READY_RETRY => {
            if PrintOnTextbox(&raw mut (*data).textState, gStringVar4.as_mut_ptr()) != 0 {
                gSpecialVar_Result = LINKUP_RETRY_ROLE_ASSIGN;
                (*data).state = LG_STATE_SHUTDOWN;
            }
        }
        LG_STATE_SHUTDOWN => {
            DestroyTask(taskId);
            JoinGroup_EnableScriptContexts();
            LinkRfu_Shutdown();
        }
        LG_STATE_START_ACTIVITY => {
            CreateTask_RunScriptAndFadeToActivity();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn IsTryingToTradeAcrossVersionTooSoon(
    data: *mut WirelessLink_Group,
    id: i32,
) -> u32 {
    let mut partner: *mut RfuPlayer = &raw mut (*(*data).playerList).players[id];
    if gPlayerCurrActivity == ACTIVITY_TRADE
        && (*partner).rfu.data.compatibility.version() != VERSION_EMERALD as u16
    {
        if (*gSaveBlock2Ptr).specialSaveWarpFlags as i32 & CHAMPION_SAVEWARP == 0 {
            return UR_TRADE_PLAYER_NOT_READY;
        } else if (*partner).rfu.data.compatibility.canLinkNationally() != 0 {
            return UR_TRADE_READY;
        }
    } else {
        return UR_TRADE_READY;
    }
    return UR_TRADE_PARTNER_NOT_READY;
}
pub(crate) unsafe extern "C" fn AskToJoinRfuGroup(data: *mut WirelessLink_Group, id: i32) {
    (*data).leaderId = id as u8;
    LoadWirelessStatusIndicatorSpriteGfx();
    CreateWirelessStatusIndicatorSprite(0, 0);
    RedrawListMenu((*data).listTaskId);
    CopyAndTranslatePlayerName(
        gStringVar1.as_mut_ptr(),
        &raw mut (*(*data).playerList).players[(*data).leaderId],
    );
    UpdateGameData_SetActivity(
        sLinkGroupToURoomActivity[gSpecialVar_0x8004],
        0,
        TRUE as u32,
    );
    CreateTask_RfuReconnectWithParent(
        (*(*data).playerList).players[(*data).leaderId]
            .rfu
            .name
            .as_mut_ptr(),
        ReadAsU16(
            (*(*data).playerList).players[(*data).leaderId]
                .rfu
                .data
                .compatibility
                .playerTrainerId
                .as_mut_ptr(),
        ),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_ListenToWireless() -> u8 {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Group = null_mut();
    taskId = CreateTask(Some(Task_ListenToWireless), 0);
    sWirelessLinkMain.group = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Group;
        data
    };
    (*data).state = 0;
    (*data).textState = 0;
    sGroup = data;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_ListenToWireless(taskId: u8) {
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    match (*data).state {
        0 => {
            SetHostRfuGameData(0, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_JoinGroup();
            RfuSetIgnoreError(TRUE as u32);
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(512) as *mut RfuPlayerList;
            (*data).state = 2;
        }
        2 => {
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYER_LIST_SIZE,
            );
            (*data).listenTaskId =
                CreateTask_ListenForCompatiblePartners((*data).incomingPlayerList, 0xFF);
            (*data).leaderId = 0;
            (*data).state = 3;
        }
        3 => {
            if GetNewLeaderCandidate() == 1 {
                PlaySE(SE_PC_LOGIN);
            }
            if gTasks[taskId].data[15] == 0xFF {
                (*data).state = 10;
            }
        }
        10 => {
            DestroyTask((*data).listenTaskId);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            LinkRfu_Shutdown();
            (*data).state += 1;
        }
        11 => {
            LinkRfu_Shutdown();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn IsPartnerActivityAcceptable(activity: u32, linkGroup: u32) -> u32 {
    if linkGroup == 0xFF {
        return TRUE as u32;
    }
    if linkGroup < 22 {
        let mut bytes: *mut u8 = sAcceptedActivityIds[linkGroup];
        while *bytes != 0xFF {
            if *bytes as u32 == activity {
                return TRUE as u32;
            }
            bytes = bytes.at(1);
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetGroupListTextColor(
    data: *mut WirelessLink_Group,
    id: u32,
) -> u8 {
    if (*(*data).playerList).players[id].groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
        if (*(*data).playerList).players[id].rfu.data.startedActivity() != 0 {
            return UR_COLOR_WHITE;
        } else if (*(*data).playerList).players[id].useRedText() != 0 {
            return UR_COLOR_RED;
        } else if (*(*data).playerList).players[id].newPlayerCountdown != 0 {
            return UR_COLOR_GREEN;
        }
    }
    return UR_COLOR_DEFAULT;
}
pub(crate) unsafe extern "C" fn ListMenuItemPrintFunc_UnionRoomGroups(
    windowId: u8,
    id: u32,
    y: u8,
) {
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    let mut colorId: u8 = GetGroupListTextColor(data, id);
    PrintGroupMemberOnWindow(
        windowId,
        8,
        y,
        &raw mut (*(*data).playerList).players[id],
        colorId,
        id as u8,
    );
}
pub(crate) unsafe extern "C" fn GetNewLeaderCandidate() -> u8 {
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    let mut ret: u8 = 0;
    let mut i: u8 = 0;
    let mut id: i32 = 0;
    i = 0;
    while i < MAX_RFU_PLAYER_LIST_SIZE {
        if (*(*data).playerList).players[i].groupScheduledAnim() != UNION_ROOM_SPAWN_NONE {
            id = GetNewIncomingPlayerId(
                &raw mut (*(*data).playerList).players[i],
                (*(*data).incomingPlayerList).players.as_mut_ptr(),
            ) as i32;
            if id != 0xFF {
                if (*(*data).playerList).players[i].groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
                    if ArePlayerDataDifferent(
                        &raw mut (*(*data).playerList).players[i].rfu,
                        &raw mut (*(*data).incomingPlayerList).players[id].rfu,
                    ) != 0
                    {
                        (*(*data).playerList).players[i].rfu =
                            (*(*data).incomingPlayerList).players[id].rfu;
                        (*(*data).playerList).players[i].newPlayerCountdown = 64;
                        ret = 1;
                    } else {
                        if (*(*data).playerList).players[i].newPlayerCountdown != 0 {
                            (*(*data).playerList).players[i].newPlayerCountdown -= 1;
                            if (*(*data).playerList).players[i].newPlayerCountdown == 0 {
                                ret = 2;
                            }
                        }
                    }
                } else {
                    (*(*data).playerList).players[i].set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
                    (*(*data).playerList).players[i].newPlayerCountdown = 64;
                    ret = 1;
                }
                (*(*data).playerList).players[i].timeoutCounter = 0;
            } else {
                if (*(*data).playerList).players[i].groupScheduledAnim() != UNION_ROOM_SPAWN_OUT {
                    (*(*data).playerList).players[i].timeoutCounter += 1;
                    if (*(*data).playerList).players[i].timeoutCounter >= 300 {
                        (*(*data).playerList).players[i]
                            .set_groupScheduledAnim(UNION_ROOM_SPAWN_OUT);
                        ret = 2;
                    }
                }
            }
        }
        i += 1;
    }
    id = 0;
    while id < RFU_CHILD_MAX as i32 {
        if TryAddIncomingPlayerToList(
            (*(*data).playerList).players.as_mut_ptr(),
            &raw mut (*(*data).incomingPlayerList).players[id],
            MAX_RFU_PLAYER_LIST_SIZE,
        ) != 0xFF
        {
            ret = 1;
        }
        id += 1;
    }
    return ret;
}
pub(crate) unsafe extern "C" fn Task_CreateTradeMenu(taskId: u8) {
    CB2_StartCreateTradeMenu();
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_CreateTradeMenu() -> u8 {
    return CreateTask(Some(Task_CreateTradeMenu), 0);
}
pub(crate) unsafe extern "C" fn Task_StartUnionRoomTrade(taskId: u8) {
    let mut monId: u32 =
        GetPartyPositionOfRegisteredMon(&raw mut sUnionRoomTrade, GetMultiplayerId());
    match gTasks[taskId].data[0] {
        0 => {
            gTasks[taskId].data[0] += 1;
            SendBlock(0, &raw mut gPlayerParty[monId] as *mut c_void, 100);
        }
        1 => {
            if GetBlockReceivedStatus() == 3 {
                gEnemyParty[0] =
                    *(gBlockRecvBuffer[GetMultiplayerId() as i32 ^ 1].as_mut_ptr() as *mut Pokemon);
                IncrementGameStat(GAME_STAT_NUM_UNION_ROOM_BATTLES);
                ResetBlockReceivedFlags();
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            memcpy(
                gBlockSendBuffer.as_mut_ptr(),
                (*gSaveBlock1Ptr).mail.as_mut_ptr() as *mut u8,
                220,
            );
            if SendBlock(0, gBlockSendBuffer.as_mut_ptr() as *mut c_void, 220) != 0 {
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            if GetBlockReceivedStatus() == 3 {
                memcpy(
                    gTradeMail.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[GetMultiplayerId() as i32 ^ 1].as_mut_ptr() as *mut u8,
                    216,
                );
                ResetBlockReceivedFlags();
                gSelectedTradeMonPositions[0] = monId as u8;
                gSelectedTradeMonPositions[1] = PARTY_SIZE as u8;
                gMain.savedCallback = Some(CB2_ReturnToField);
                SetMainCallback2(Some(CB2_LinkTrade));
                ResetUnionRoomTrade(&raw mut sUnionRoomTrade);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ExchangeCards(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if GetMultiplayerId() == 0 {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
            }
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if GetBlockReceivedStatus() == GetLinkPlayerCountAsBitFlags() {
                let mut i: i32 = 0;
                let mut recvBuff: *mut u16 = null_mut();
                i = 0;
                while i < GetLinkPlayerCount() as i32 {
                    recvBuff = gBlockRecvBuffer[i].as_mut_ptr();
                    CopyTrainerCardData(
                        &raw mut gTrainerCards[i],
                        recvBuff as *mut TrainerCard,
                        gLinkPlayers[i].version as u8,
                    );
                    i += 1;
                }
                if GetLinkPlayerCount() == 2 {
                    recvBuff = gBlockRecvBuffer[GetMultiplayerId() as i32 ^ 1].as_mut_ptr();
                    MysteryGift_TryEnableStatsByFlagId(
                        (*(recvBuff as *mut TrainerCard)).hasAllFrontierSymbols,
                    );
                } else {
                    MysteryGift_DisableStats();
                }
                ResetBlockReceivedFlags();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowCard() {
    match gMain.state {
        0 => {
            CreateTask(Some(Task_ExchangeCards), 5);
            gMain.state += 1;
        }
        1 => {
            if FuncIsActiveTask(Some(Task_ExchangeCards)) == 0 {
                ShowTrainerCardInLink(GetMultiplayerId() ^ 1, Some(CB2_ReturnToField));
            }
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartUnionRoomBattle(battleFlags: u16) {
    HealPlayerParty();
    SavePlayerParty();
    LoadPlayerBag();
    gLinkPlayers[0].linkType = LINKTYPE_BATTLE as u32;
    gLinkPlayers[GetMultiplayerId()].id = GetMultiplayerId() as u16;
    gLinkPlayers[GetMultiplayerId() as i32 ^ 1].id = GetMultiplayerId() as u16 ^ 1;
    gMain.savedCallback = Some(CB2_ReturnFromCableClubBattle);
    gBattleTypeFlags = battleFlags as u32;
    PlayBattleBGM();
}
pub(crate) unsafe extern "C" fn WarpForWirelessMinigame(linkService: u16, x: u16, y: u16) {
    VarSet(VAR_CABLE_CLUB_STATE, linkService);
    SetWarpDestination(
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
        x as i8,
        y as i8,
    );
    SetDynamicWarpWithCoords(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
        x as i8,
        y as i8,
    );
    WarpIntoMap();
}
pub(crate) unsafe extern "C" fn WarpForCableClubActivity(
    mapGroup: i8,
    mapNum: i8,
    x: i32,
    y: i32,
    linkService: u16,
) {
    gSpecialVar_0x8004 = linkService;
    VarSet(VAR_CABLE_CLUB_STATE, linkService);
    gFieldLinkPlayerCount = GetLinkPlayerCount();
    gLocalLinkPlayerId = GetMultiplayerId();
    SetCableClubWarp();
    SetWarpDestination(mapGroup, mapNum, WARP_ID_NONE, x as i8, y as i8);
    WarpIntoMap();
}
pub(crate) unsafe extern "C" fn CB2_TransitionToCableClub() {
    match gMain.state {
        0 => {
            CreateTask(Some(Task_ExchangeCards), 5);
            gMain.state += 1;
        }
        1 => {
            if FuncIsActiveTask(Some(Task_ExchangeCards)) == 0 {
                SetMainCallback2(Some(CB2_ReturnToFieldCableClub));
            }
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
}
pub(crate) unsafe extern "C" fn CreateTrainerCardInBuffer(dest: *mut c_void, setWonderCard: u32) {
    let mut card: *mut TrainerCard = dest as *mut TrainerCard;
    TrainerCard_GenerateCardForLinkPlayer(card);
    if setWonderCard != 0 {
        (*card).hasAllFrontierSymbols = GetWonderCardFlagID();
    } else {
        (*card).hasAllFrontierSymbols = 0;
    }
}
pub(crate) unsafe extern "C" fn Task_StartActivity(taskId: u8) {
    MysteryGift_DisableStats();
    match gPlayerCurrActivity {
        ACTIVITY_BATTLE_SINGLE
        | ACTIVITY_BATTLE_DOUBLE
        | ACTIVITY_BATTLE_MULTI
        | ACTIVITY_TRADE
        | ACTIVITY_POKEMON_JUMP
        | ACTIVITY_BERRY_CRUSH
        | ACTIVITY_BERRY_PICK
        | ACTIVITY_SPIN_TRADE
        | ACTIVITY_RECORD_CORNER => {
            SaveLinkTrainerNames();
        }
        _ => {}
    }
    match gPlayerCurrActivity {
        65 | 81 => {
            CleanupOverworldWindowsAndTilemaps();
            gMain.savedCallback = Some(CB2_UnionRoomBattle);
            InitChooseHalfPartyForBattle(3);
        }
        ACTIVITY_BATTLE_SINGLE => {
            CleanupOverworldWindowsAndTilemaps();
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, TRUE as u32);
            HealPlayerParty();
            SavePlayerParty();
            LoadPlayerBag();
            WarpForCableClubActivity(25, 24, 6, 8, USING_SINGLE_BATTLE);
            SetMainCallback2(Some(CB2_TransitionToCableClub));
        }
        ACTIVITY_BATTLE_DOUBLE => {
            CleanupOverworldWindowsAndTilemaps();
            HealPlayerParty();
            SavePlayerParty();
            LoadPlayerBag();
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, TRUE as u32);
            WarpForCableClubActivity(25, 24, 6, 8, USING_DOUBLE_BATTLE);
            SetMainCallback2(Some(CB2_TransitionToCableClub));
        }
        ACTIVITY_BATTLE_MULTI => {
            CleanupOverworldWindowsAndTilemaps();
            HealPlayerParty();
            SavePlayerParty();
            LoadPlayerBag();
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, TRUE as u32);
            WarpForCableClubActivity(25, 27, 5, 8, 5);
            SetMainCallback2(Some(CB2_TransitionToCableClub));
        }
        ACTIVITY_TRADE => {
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, TRUE as u32);
            CleanupOverworldWindowsAndTilemaps();
            WarpForCableClubActivity(25, 25, 5, 8, USING_TRADE_CENTER);
            SetMainCallback2(Some(CB2_TransitionToCableClub));
        }
        ACTIVITY_RECORD_CORNER => {
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, TRUE as u32);
            CleanupOverworldWindowsAndTilemaps();
            WarpForCableClubActivity(25, 26, 8, 9, USING_RECORD_CORNER);
            SetMainCallback2(Some(CB2_TransitionToCableClub));
        }
        68 => {
            CleanupOverworldWindowsAndTilemaps();
            CreateTask(Some(Task_StartUnionRoomTrade), 0);
        }
        ACTIVITY_CHAT | 69 => {
            if GetMultiplayerId() == 0 {
                LinkRfu_CreateConnectionAsParent();
            } else {
                LinkRfu_StopManagerBeforeEnteringChat();
                SetHostRfuGameData(69, 0, TRUE as u32);
            }
            EnterUnionRoomChat();
        }
        ACTIVITY_CARD | 72 => {
            CreateTrainerCardInBuffer(gBlockSendBuffer.as_mut_ptr() as *mut c_void, FALSE as u32);
            SetMainCallback2(Some(CB2_ShowCard));
        }
        ACTIVITY_POKEMON_JUMP => {
            WarpForWirelessMinigame(USING_MINIGAME, 5, 1);
            StartPokemonJump(GetCursorSelectionMonId() as u16, Some(CB2_LoadMap));
        }
        ACTIVITY_BERRY_CRUSH => {
            WarpForWirelessMinigame(USING_BERRY_CRUSH, 9, 1);
            StartBerryCrush(Some(CB2_LoadMap));
        }
        ACTIVITY_BERRY_PICK => {
            WarpForWirelessMinigame(USING_MINIGAME, 5, 1);
            StartDodrioBerryPicking(GetCursorSelectionMonId() as u16, Some(CB2_LoadMap));
        }
        _ => {}
    }
    DestroyTask(taskId);
    gSpecialVar_Result = LINKUP_SUCCESS;
    if gPlayerCurrActivity != 68 {
        UnlockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn Task_RunScriptAndFadeToActivity(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut sendBuff: *mut u16 = gBlockSendBuffer.as_mut_ptr() as *mut u16;
    match *data {
        0 => {
            gSpecialVar_Result = LINKUP_SUCCESS;
            'l2: {
                let sw1: u8 = gPlayerCurrActivity;
                let matched = sw1 == ACTIVITY_BATTLE_TOWER
                    || sw1 == ACTIVITY_BATTLE_TOWER_OPEN
                    || sw1 == ACTIVITY_BERRY_BLENDER
                    || sw1 == ACTIVITY_CONTEST_COOL
                    || sw1 == ACTIVITY_CONTEST_BEAUTY
                    || sw1 == ACTIVITY_CONTEST_CUTE
                    || sw1 == ACTIVITY_CONTEST_SMART
                    || sw1 == ACTIVITY_CONTEST_TOUGH;
                let mut fall = false;
                if sw1 == ACTIVITY_BATTLE_TOWER || sw1 == ACTIVITY_BATTLE_TOWER_OPEN {
                    fall = true;
                    gLinkPlayers[0].linkType = LINKTYPE_BATTLE as u32;
                    gLinkPlayers[0].id = 0;
                    gLinkPlayers[1].id = 2;
                    *sendBuff = GetMonData2(
                        &raw mut gPlayerParty[gSelectedOrderFromParty[0] as i32 - 1],
                        MON_DATA_SPECIES,
                    ) as u16;
                    *sendBuff.at(1) = GetMonData3(
                        &raw mut gPlayerParty[gSelectedOrderFromParty[1] as i32 - 1],
                        MON_DATA_SPECIES,
                        null_mut(),
                    ) as u16;
                    gMain.savedCallback = None;
                    *data = 4;
                    SaveLinkTrainerNames();
                    ResetBlockReceivedFlags();
                    break 'l2;
                }
                if sw1 == ACTIVITY_BERRY_BLENDER
                    || sw1 == ACTIVITY_CONTEST_COOL
                    || sw1 == ACTIVITY_CONTEST_BEAUTY
                    || sw1 == ACTIVITY_CONTEST_CUTE
                    || sw1 == ACTIVITY_CONTEST_SMART
                    || sw1 == ACTIVITY_CONTEST_TOUGH
                {
                    fall = true;
                    SaveLinkTrainerNames();
                    DestroyTask(taskId);
                }
                if fall || !matched {
                    fall = true;
                    ScriptContext_Enable();
                    *data = 1;
                    break 'l2;
                }
            }
        }
        1 => {
            if ScriptContext_IsEnabled() == 0 {
                FadeScreen(FADE_TO_BLACK, 0);
                *data = 2;
            }
        }
        2 => {
            if gPaletteFade.active() == 0 {
                if gPlayerCurrActivity == ACTIVITY_29 {
                    DestroyTask(taskId);
                    SetMainCallback2(Some(CB2_StartCreateTradeMenu));
                } else {
                    SetLinkStandbyCallback();
                    *data = 3;
                }
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                DestroyTask(taskId);
                CreateTask_StartActivity();
            }
        }
        4 => {
            if SendBlock(0, gBlockSendBuffer.as_mut_ptr() as *mut c_void, 0xE) != 0 {
                *data = 5;
            }
        }
        5 => {
            if GetBlockReceivedStatus() == 3 {
                ResetBlockReceivedFlags();
                if AreBattleTowerLinkSpeciesSame(
                    gBlockRecvBuffer[0].as_mut_ptr(),
                    gBlockRecvBuffer[1].as_mut_ptr(),
                ) != 0
                {
                    gSpecialVar_Result = LINKUP_FAILED_BATTLE_TOWER;
                    *data = 7;
                } else {
                    *data = 6;
                }
            }
        }
        6 => {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        7 => {
            SetCloseLinkCallback();
            *data = 8;
        }
        8 => {
            if gReceivedRemoteLinkPlayers == 0 {
                DestroyWirelessStatusIndicatorSprite();
                ScriptContext_Enable();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateTask_RunScriptAndFadeToActivity() {
    CreateTask(Some(Task_RunScriptAndFadeToActivity), 0);
}
pub(crate) unsafe extern "C" fn CreateTask_StartActivity() {
    let mut taskId: u8 = CreateTask(Some(Task_StartActivity), 0);
    gTasks[taskId].data[0] = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_SendMysteryGift(activity: u32) {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Leader = null_mut();
    taskId = CreateTask(Some(Task_SendMysteryGift), 0);
    sWirelessLinkMain.leader = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Leader;
        data
    };
    (*data).state = 0;
    (*data).textState = 0;
    (*data).activity = activity as u8;
    gSpecialVar_Result = LINKUP_ONGOING;
}
pub(crate) unsafe extern "C" fn Task_SendMysteryGift(taskId: u8) {
    let mut data: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    let mut winTemplate: WindowTemplate = zeroed();
    let mut val: i32 = 0;
    match (*data).state {
        0 => {
            gPlayerCurrActivity = (*data).activity;
            sPlayerActivityGroupSize = 2;
            SetHostRfuGameData((*data).activity, 0, 0);
            SetHostRfuWonderFlags(FALSE as u32, FALSE as u32);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_LinkLeader(2);
            (*data).state = 1;
        }
        1 => {
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(160) as *mut RfuPlayerList;
            (*data).playerListBackup = AllocZeroed(160) as *mut RfuPlayerList;
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYERS as u8,
            );
            CopyHostRfuGameDataAndUsername(
                &raw mut (*(*data).playerList).players[0].rfu.data,
                (*(*data).playerList).players[0].rfu.name.as_mut_ptr(),
            );
            (*(*data).playerList).players[0].timeoutCounter = 0;
            (*(*data).playerList).players[0].set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
            (*(*data).playerList).players[0].set_useRedText(0);
            (*(*data).playerList).players[0].newPlayerCountdown = 0;
            (*data).listenTaskId =
                CreateTask_ListenForCompatiblePartners((*data).incomingPlayerList, 0xFF);
            winTemplate = *sWindowTemplate_PlayerList;
            winTemplate.baseBlock = GetMysteryGiftBaseBlock();
            winTemplate.paletteNum = 12;
            (*data).listWindowId = AddWindow(&raw mut winTemplate) as u8;
            MG_DrawTextBorder((*data).listWindowId);
            gMultiuseListMenuTemplate = *sListMenuTemplate_PossibleGroupMembers;
            gMultiuseListMenuTemplate.windowId = (*data).listWindowId;
            (*data).listTaskId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            CopyBgTilemapBufferToVram(0);
            (*data).playerCount = 1;
            (*data).state = 2;
        }
        2 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                sLinkGroupActivityNameTexts[gPlayerCurrActivity],
            );
            GetAwaitingCommunicationText(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
            (*data).state = 3;
        }
        3 => {
            MG_AddMessageTextPrinter(gStringVar4.as_mut_ptr());
            (*data).state = 4;
        }
        4 => {
            Leader_SetStateIfMemberListChanged(data, 5, 6);
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                (*data).state = 13;
                DestroyWirelessStatusIndicatorSprite();
            }
        }
        6 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sText_LinkWithFriendDropped.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).playerCount = LeaderPrunePlayerList((*data).playerList);
                RedrawListMenu((*data).listTaskId);
                (*data).state = 2;
            }
        }
        5 => {
            (*data).state = 7;
        }
        7 => {
            match DoMysteryGiftYesNo(
                &raw mut (*data).textState,
                &raw mut (*data).yesNoWindowId,
                FALSE,
                gStringVar4.as_mut_ptr(),
            ) {
                0 => {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                    (*(*data).playerList).players[(*data).playerCount].newPlayerCountdown = 0;
                    RedrawListMenu((*data).listTaskId);
                    (*data).joinRequestAnswer = RFU_STATUS_JOIN_GROUP_OK;
                    SendRfuStatusToPartner(
                        (*data).joinRequestAnswer,
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                    );
                    (*data).state = 8;
                }
                1 | MENU_B_PRESSED => {
                    (*data).joinRequestAnswer = RFU_STATUS_JOIN_GROUP_NO;
                    SendRfuStatusToPartner(
                        (*data).joinRequestAnswer,
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                    );
                    (*data).state = 8;
                }
                _ => {}
            }
        }
        8 => {
            val = WaitSendRfuStatusToPartner(
                ReadAsU16(
                    (*(*data).playerList).players[(*data).playerCount]
                        .rfu
                        .data
                        .compatibility
                        .playerTrainerId
                        .as_mut_ptr(),
                ),
                (*(*data).playerList).players[(*data).playerCount]
                    .rfu
                    .name
                    .as_mut_ptr(),
            ) as i32;
            if val == 1 {
                if (*data).joinRequestAnswer == RFU_STATUS_JOIN_GROUP_OK {
                    (*(*data).playerList).players[(*data).playerCount].newPlayerCountdown = 0;
                    RedrawListMenu((*data).listTaskId);
                    (*data).playerCount += 1;
                    CopyAndTranslatePlayerName(
                        gStringVar1.as_mut_ptr(),
                        &raw mut (*(*data).playerList).players[(*data).playerCount as i32 - 1],
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        sText_AnOKWasSentToPlayer.as_ptr().cast_mut(),
                    );
                    (*data).state = 9;
                    LinkRfu_StopManagerAndFinalizeSlots();
                } else {
                    RequestDisconnectSlotByTrainerNameAndId(
                        (*(*data).playerList).players[(*data).playerCount]
                            .rfu
                            .name
                            .as_mut_ptr(),
                        ReadAsU16(
                            (*(*data).playerList).players[(*data).playerCount]
                                .rfu
                                .data
                                .compatibility
                                .playerTrainerId
                                .as_mut_ptr(),
                        ),
                    );
                    (*(*data).playerList).players[(*data).playerCount]
                        .set_groupScheduledAnim(UNION_ROOM_SPAWN_NONE);
                    LeaderPrunePlayerList((*data).playerList);
                    RedrawListMenu((*data).listTaskId);
                    (*data).state = 2;
                }
                (*data).joinRequestAnswer = 0;
            } else if val == 2 {
                RfuSetStatus(0, 0);
                (*data).state = 2;
            }
        }
        9 => {
            MG_AddMessageTextPrinter(gStringVar4.as_mut_ptr());
            (*data).state = 10;
        }
        10 => {
            if ({
                (*data).delayTimerAfterOk += 1;
                (*data).delayTimerAfterOk
            }) > 120
            {
                (*data).state = 11;
            }
        }
        11 => {
            if Leader_SetStateIfMemberListChanged(data, 5, 6) == 0 {
                (*data).state = 12;
            }
        }
        12 => {
            if LmanAcceptSlotFlagIsNotZero() != 0 {
                WaitRfuState(FALSE as u32);
                (*data).state = 15;
            } else {
                (*data).state = 6;
            }
        }
        13 => {
            DestroyWirelessStatusIndicatorSprite();
            LinkRfu_Shutdown();
            DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
            CopyBgTilemapBufferToVram(0);
            RemoveWindow((*data).listWindowId);
            DestroyTask((*data).listenTaskId);
            Free((*data).playerListBackup as *mut c_void);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            (*data).state += 1;
        }
        14 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sText_PleaseStartOver.as_ptr().cast_mut(),
            ) != 0
            {
                DestroyTask(taskId);
                gSpecialVar_Result = LINKUP_FAILED;
            }
        }
        15 => {
            if RfuGetStatus() == RFU_STATUS_FATAL_ERROR
                || RfuGetStatus() == RFU_STATUS_CONNECTION_ERROR
            {
                (*data).state = 13;
            } else if gReceivedRemoteLinkPlayers != 0 {
                UpdateGameData_GroupLockedIn(TRUE);
                (*data).state += 1;
            }
        }
        16 => {
            DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
            CopyBgTilemapBufferToVram(0);
            RemoveWindow((*data).listWindowId);
            DestroyTask((*data).listenTaskId);
            Free((*data).playerListBackup as *mut c_void);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            SetLinkStandbyCallback();
            (*data).state += 1;
        }
        17 => {
            if IsLinkTaskFinished() != 0 {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_LinkMysteryGiftWithFriend(activity: u32) {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Group = null_mut();
    taskId = CreateTask(Some(Task_CardOrNewsWithFriend), 0);
    sWirelessLinkMain.group = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Group;
        data
    };
    sGroup = data;
    (*data).state = 0;
    (*data).textState = 0;
    (*data).isWonderNews = activity as u8 - ACTIVITY_WONDER_CARD;
    gSpecialVar_Result = LINKUP_ONGOING;
}
pub(crate) unsafe extern "C" fn Task_CardOrNewsWithFriend(taskId: u8) {
    let mut id: i32 = 0;
    let mut listWinTemplate: WindowTemplate = zeroed();
    let mut playerNameWinTemplate: WindowTemplate = zeroed();
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    match (*data).state {
        0 => {
            SetHostRfuGameData((*data).isWonderNews + ACTIVITY_WONDER_CARD, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_JoinGroup();
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(512) as *mut RfuPlayerList;
            (*data).state = 1;
        }
        1 => {
            MG_AddMessageTextPrinter(sText_ChooseTrainer.as_ptr().cast_mut());
            (*data).state = 2;
        }
        2 => {
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYER_LIST_SIZE,
            );
            (*data).listenTaskId = CreateTask_ListenForCompatiblePartners(
                (*data).incomingPlayerList,
                (*data).isWonderNews as u32 + LINK_GROUP_WONDER_CARD,
            );
            listWinTemplate = *sWindowTemplate_GroupList;
            listWinTemplate.baseBlock = GetMysteryGiftBaseBlock();
            listWinTemplate.paletteNum = 12;
            (*data).listWindowId = AddWindow(&raw mut listWinTemplate) as u8;
            playerNameWinTemplate = *sWindowTemplate_PlayerNameAndId;
            playerNameWinTemplate.paletteNum = 12;
            (*data).playerNameAndIdWindowId = AddWindow(&raw mut playerNameWinTemplate) as u8;
            MG_DrawTextBorder((*data).listWindowId);
            gMultiuseListMenuTemplate = *sListMenuTemplate_UnionRoomGroups;
            gMultiuseListMenuTemplate.windowId = (*data).listWindowId;
            (*data).listTaskId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            MG_DrawTextBorder((*data).playerNameAndIdWindowId);
            FillWindowPixelBuffer((*data).playerNameAndIdWindowId, 17);
            PutWindowTilemap((*data).playerNameAndIdWindowId);
            PrintPlayerNameAndIdOnWindow((*data).playerNameAndIdWindowId);
            CopyWindowToVram((*data).playerNameAndIdWindowId, COPYWIN_GFX);
            CopyBgTilemapBufferToVram(0);
            (*data).leaderId = 0;
            (*data).state = 3;
        }
        3 => {
            id = GetNewLeaderCandidate() as i32;
            'l2: {
                let sw1: i32 = id;
                let matched = sw1 == 1 || sw1 == 0;
                let mut fall = false;
                if sw1 == 1 {
                    fall = true;
                    PlaySE(SE_PC_LOGIN);
                }
                if fall || !matched {
                    fall = true;
                    RedrawListMenu((*data).listTaskId);
                    break 'l2;
                }
                if sw1 == 0 {
                    fall = true;
                    id = ListMenu_ProcessInput((*data).listTaskId);
                    if gMain.newKeys as i32 & A_BUTTON != 0 && id != LIST_NOTHING_CHOSEN {
                        let mut activity: u32 =
                            (*(*data).playerList).players[id].rfu.data.activity() as u32;
                        if (*(*data).playerList).players[id].groupScheduledAnim()
                            == UNION_ROOM_SPAWN_IN
                            && (*(*data).playerList).players[id].rfu.data.startedActivity() == 0
                        {
                            (*data).leaderId = id as u8;
                            LoadWirelessStatusIndicatorSpriteGfx();
                            CreateWirelessStatusIndicatorSprite(0, 0);
                            RedrawListMenu((*data).listTaskId);
                            CopyAndTranslatePlayerName(
                                gStringVar1.as_mut_ptr(),
                                &raw mut (*(*data).playerList).players[(*data).leaderId],
                            );
                            CreateTask_RfuReconnectWithParent(
                                (*(*data).playerList).players[(*data).leaderId]
                                    .rfu
                                    .name
                                    .as_mut_ptr(),
                                ReadAsU16(
                                    (*(*data).playerList).players[(*data).leaderId]
                                        .rfu
                                        .data
                                        .compatibility
                                        .playerTrainerId
                                        .as_mut_ptr(),
                                ),
                            );
                            PlaySE(SE_POKENAV_ON);
                            (*data).state = 4;
                        } else {
                            PlaySE(SE_WALL_HIT);
                        }
                    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                        (*data).state = 6;
                    }
                    break 'l2;
                }
            }
        }
        4 => {
            MG_AddMessageTextPrinter(sText_AwaitingPlayersResponse.as_ptr().cast_mut());
            CopyAndTranslatePlayerName(
                gStringVar1.as_mut_ptr(),
                &raw mut (*(*data).playerList).players[(*data).leaderId],
            );
            (*data).state = 5;
        }
        5 => {
            if gReceivedRemoteLinkPlayers != 0 {
                gPlayerCurrActivity = (*(*data).playerList).players[(*data).leaderId]
                    .rfu
                    .data
                    .activity();
                (*data).state = 10;
            }
            match RfuGetStatus() {
                RFU_STATUS_FATAL_ERROR | RFU_STATUS_CONNECTION_ERROR | RFU_STATUS_JOIN_GROUP_NO => {
                    (*data).state = 8;
                }
                RFU_STATUS_JOIN_GROUP_OK => {
                    MG_AddMessageTextPrinter(sText_PlayerSentBackOK.as_ptr().cast_mut());
                    RfuSetStatus(0, 0);
                }
                _ => {}
            }
        }
        6 | 8 | 10 => {
            DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
            CopyBgTilemapBufferToVram(0);
            RemoveWindow((*data).playerNameAndIdWindowId);
            RemoveWindow((*data).listWindowId);
            DestroyTask((*data).listenTaskId);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            (*data).state += 1;
        }
        9 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sLinkDroppedTexts[RfuGetStatus()],
            ) != 0
            {
                DestroyWirelessStatusIndicatorSprite();
                DestroyTask(taskId);
                LinkRfu_Shutdown();
                gSpecialVar_Result = LINKUP_FAILED;
            }
        }
        7 => {
            DestroyWirelessStatusIndicatorSprite();
            MG_AddMessageTextPrinter(sText_PleaseStartOver.as_ptr().cast_mut());
            DestroyTask(taskId);
            LinkRfu_Shutdown();
            gSpecialVar_Result = LINKUP_FAILED;
        }
        11 => {
            (*data).state += 1;
            SetLinkStandbyCallback();
        }
        12 => {
            if IsLinkTaskFinished() != 0 {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_LinkMysteryGiftOverWireless(activity: u32) {
    let mut taskId: u8 = 0;
    let mut data: *mut WirelessLink_Group = null_mut();
    taskId = CreateTask(Some(Task_CardOrNewsOverWireless), 0);
    sWirelessLinkMain.group = {
        data = gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut WirelessLink_Group;
        data
    };
    sGroup = data;
    (*data).state = 0;
    (*data).textState = 0;
    (*data).isWonderNews = activity as u8 - ACTIVITY_WONDER_CARD;
    gSpecialVar_Result = LINKUP_ONGOING;
}
pub(crate) unsafe extern "C" fn Task_CardOrNewsOverWireless(taskId: u8) {
    let mut id: i32 = 0;
    let mut winTemplate: WindowTemplate = zeroed();
    let mut data: *mut WirelessLink_Group = sWirelessLinkMain.group;
    match (*data).state {
        0 => {
            SetHostRfuGameData(0, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_JoinGroup();
            (*data).incomingPlayerList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            (*data).playerList = AllocZeroed(512) as *mut RfuPlayerList;
            (*data).state = 1;
        }
        1 => {
            MG_AddMessageTextPrinter(sText_SearchingForWirelessSystemWait.as_ptr().cast_mut());
            (*data).state = 2;
        }
        2 => {
            ClearIncomingPlayerList((*data).incomingPlayerList, RFU_CHILD_MAX);
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_RFU_PLAYER_LIST_SIZE,
            );
            (*data).listenTaskId = CreateTask_ListenForWonderDistributor(
                (*data).incomingPlayerList,
                (*data).isWonderNews as u32 + LINK_GROUP_WONDER_CARD,
            );
            if (*data).showListMenu != 0 {
                winTemplate = *sWindowTemplate_GroupList;
                winTemplate.baseBlock = GetMysteryGiftBaseBlock();
                (*data).listWindowId = AddWindow(&raw mut winTemplate) as u8;
                MG_DrawTextBorder((*data).listWindowId);
                gMultiuseListMenuTemplate = *sListMenuTemplate_UnionRoomGroups;
                gMultiuseListMenuTemplate.windowId = (*data).listWindowId;
                (*data).listTaskId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
                CopyBgTilemapBufferToVram(0);
            }
            (*data).leaderId = 0;
            (*data).state = 3;
        }
        3 => {
            id = GetNewLeaderCandidate() as i32;
            'l2: {
                let sw1: i32 = id;
                let matched = sw1 == 1 || sw1 == 0;
                let mut fall = false;
                if sw1 == 1 {
                    fall = true;
                    PlaySE(SE_PC_LOGIN);
                }
                if fall || !matched {
                    fall = true;
                    if (*data).showListMenu != 0 {
                        RedrawListMenu((*data).listTaskId);
                    }
                    break 'l2;
                }
                if sw1 == 0 {
                    fall = true;
                    if (*data).showListMenu != 0 {
                        id = ListMenu_ProcessInput((*data).listTaskId);
                    }
                    if (*data).refreshTimer > 120 {
                        if (*(*data).playerList).players[0].groupScheduledAnim()
                            == UNION_ROOM_SPAWN_IN
                            && (*(*data).playerList).players[0].rfu.data.startedActivity() == 0
                        {
                            if HasWonderCardOrNewsByLinkGroup(
                                &raw mut (*(*data).playerList).players[0].rfu.data,
                                (*data).isWonderNews as i16 + LINK_GROUP_WONDER_CARD as i16,
                            ) != 0
                            {
                                (*data).leaderId = 0;
                                (*data).refreshTimer = 0;
                                LoadWirelessStatusIndicatorSpriteGfx();
                                CreateWirelessStatusIndicatorSprite(0, 0);
                                CreateTask_RfuReconnectWithParent(
                                    (*(*data).playerList).players[0].rfu.name.as_mut_ptr(),
                                    ReadAsU16(
                                        (*(*data).playerList).players[0]
                                            .rfu
                                            .data
                                            .compatibility
                                            .playerTrainerId
                                            .as_mut_ptr(),
                                    ),
                                );
                                PlaySE(SE_POKENAV_ON);
                                (*data).state = 4;
                            } else {
                                PlaySE(SE_BOO);
                                (*data).state = 10;
                            }
                        }
                    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                        (*data).state = 6;
                        (*data).refreshTimer = 0;
                    }
                    (*data).refreshTimer += 1;
                    break 'l2;
                }
            }
        }
        4 => {
            MG_AddMessageTextPrinter(sText_AwaitingResponseFromWirelessSystem.as_ptr().cast_mut());
            CopyAndTranslatePlayerName(
                gStringVar1.as_mut_ptr(),
                &raw mut (*(*data).playerList).players[(*data).leaderId],
            );
            (*data).state = 5;
        }
        5 => {
            if gReceivedRemoteLinkPlayers != 0 {
                gPlayerCurrActivity = (*(*data).playerList).players[(*data).leaderId]
                    .rfu
                    .data
                    .activity();
                (*data).state = 12;
            }
            match RfuGetStatus() {
                RFU_STATUS_FATAL_ERROR | RFU_STATUS_CONNECTION_ERROR | RFU_STATUS_JOIN_GROUP_NO => {
                    (*data).state = 8;
                }
                RFU_STATUS_JOIN_GROUP_OK => {
                    MG_AddMessageTextPrinter(sText_WirelessLinkEstablished.as_ptr().cast_mut());
                    RfuSetStatus(0, 0);
                }
                _ => {}
            }
        }
        6 | 8 | 10 | 12 => {
            if (*data).showListMenu != 0 {
                DestroyListMenuTask((*data).listTaskId, null_mut(), null_mut());
                CopyBgTilemapBufferToVram(0);
                RemoveWindow((*data).listWindowId);
            }
            DestroyTask((*data).listenTaskId);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingPlayerList as *mut c_void);
            (*data).state += 1;
        }
        9 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sText_WirelessLinkDropped.as_ptr().cast_mut(),
            ) != 0
            {
                DestroyWirelessStatusIndicatorSprite();
                DestroyTask(taskId);
                LinkRfu_Shutdown();
                gSpecialVar_Result = LINKUP_FAILED;
            }
        }
        7 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sText_WirelessSearchCanceled.as_ptr().cast_mut(),
            ) != 0
            {
                DestroyWirelessStatusIndicatorSprite();
                DestroyTask(taskId);
                LinkRfu_Shutdown();
                gSpecialVar_Result = LINKUP_FAILED;
            }
        }
        11 => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                sNoWonderSharedTexts[(*data).isWonderNews],
            ) != 0
            {
                DestroyWirelessStatusIndicatorSprite();
                DestroyTask(taskId);
                LinkRfu_Shutdown();
                gSpecialVar_Result = LINKUP_FAILED;
            }
        }
        13 => {
            (*data).state += 1;
            SetLinkStandbyCallback();
        }
        14 => {
            if IsLinkTaskFinished() != 0 {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunUnionRoom() {
    let mut uroom: *mut WirelessLink_URoom = null_mut();
    ResetHostRfuGameData();
    CreateTask(Some(Task_RunUnionRoom), 10);
    sWirelessLinkMain.uRoom = sWirelessLinkMain.uRoom;
    uroom = AllocZeroed(620) as *mut WirelessLink_URoom;
    sWirelessLinkMain.uRoom = uroom;
    sURoom = uroom;
    (*uroom).state = UR_STATE_INIT;
    (*uroom).textState = 0;
    (*uroom).unknown = 0;
    (*uroom).unreadPlayerId = 0;
    gSpecialVar_Result = 0;
    ListMenuLoadStdPalAt(208, 1);
}
pub(crate) unsafe extern "C" fn ReadAsU16(ptr: *mut u8) -> u16 {
    return (*ptr.at(1) as u16) << 8 | *ptr as u16;
}
pub(crate) unsafe extern "C" fn ScheduleFieldMessageWithFollowupState(
    nextState: u32,
    src: *mut u8,
) {
    let mut uroom: *mut WirelessLink_URoom = sWirelessLinkMain.uRoom;
    (*uroom).state = UR_STATE_PRINT_MSG;
    (*uroom).stateAfterPrint = nextState as u8;
    if src != gStringVar4.as_mut_ptr() {
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    }
}
pub(crate) unsafe extern "C" fn ScheduleFieldMessageAndExit(src: *mut u8) {
    let mut uroom: *mut WirelessLink_URoom = sWirelessLinkMain.uRoom;
    (*uroom).state = UR_STATE_PRINT_AND_EXIT;
    if src != gStringVar4.as_mut_ptr() {
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    }
}
pub(crate) unsafe extern "C" fn CopyPlayerListToBuffer(uroom: *mut WirelessLink_URoom) {
    memcpy(
        &raw mut gDecompressionBuffer[16128],
        (*uroom).playerList as *mut u8,
        256,
    );
}
pub(crate) unsafe extern "C" fn CopyPlayerListFromBuffer(uroom: *mut WirelessLink_URoom) {
    memcpy(
        (*uroom).playerList as *mut u8,
        &raw mut gDecompressionBuffer[16128],
        256,
    );
}
pub(crate) unsafe extern "C" fn Task_RunUnionRoom(taskId: u8) {
    let mut id: u32 = 0;
    let mut input: i32 = 0;
    let mut playerGender: i32 = MALE as i32;
    let mut uroom: *mut WirelessLink_URoom = sWirelessLinkMain.uRoom;
    let mut taskData: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        match (*uroom).state {
            UR_STATE_INIT => {
                (*uroom).incomingChildList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
                (*uroom).incomingParentList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
                (*uroom).playerList = AllocZeroed(256) as *mut RfuPlayerList;
                (*uroom).spawnPlayer = AllocZeroed(32) as *mut RfuPlayerList;
                ClearRfuPlayerList(
                    (*(*uroom).playerList).players.as_mut_ptr(),
                    MAX_UNION_ROOM_LEADERS as u8,
                );
                gPlayerCurrActivity = IN_UNION_ROOM;
                (*uroom).searchTaskId = CreateTask_SearchForChildOrParent(
                    (*uroom).incomingParentList,
                    (*uroom).incomingChildList,
                    LINK_GROUP_UNION_ROOM_RESUME,
                );
                InitUnionRoomPlayerObjects((*uroom).objects.as_mut_ptr());
                SetTilesAroundUnionRoomPlayersPassable();
                (*uroom).state = UR_STATE_INIT_OBJECTS;
            }
            UR_STATE_INIT_OBJECTS => {
                CreateUnionRoomPlayerSprites((*uroom).spriteIds.as_mut_ptr(), *taskData as i32);
                if ({
                    *taskData += 1;
                    *taskData
                }) == MAX_UNION_ROOM_LEADERS as i16
                {
                    (*uroom).state = UR_STATE_INIT_LINK;
                }
            }
            UR_STATE_INIT_LINK => {
                SetHostRfuGameData(IN_UNION_ROOM, 0, 0);
                SetTradeBoardRegisteredMonInfo(
                    sUnionRoomTrade.r#type as u32,
                    sUnionRoomTrade.playerSpecies as u32,
                    sUnionRoomTrade.playerLevel as u32,
                );
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_EnterUnionRoom();
                ClearRfuPlayerList(&raw mut (*(*uroom).spawnPlayer).players[0], 1);
                ClearIncomingPlayerList((*uroom).incomingChildList, RFU_CHILD_MAX);
                ClearIncomingPlayerList((*uroom).incomingParentList, RFU_CHILD_MAX);
                gSpecialVar_Result = 0;
                (*uroom).state = UR_STATE_CHECK_SELECTING_MON;
            }
            UR_STATE_CHECK_SELECTING_MON => {
                if (GetPartyMenuType() == PARTY_MENU_TYPE_UNION_ROOM_REGISTER
                    || GetPartyMenuType() == PARTY_MENU_TYPE_UNION_ROOM_TRADE)
                    && sUnionRoomTrade.state != URTRADE_STATE_NONE
                {
                    id = GetCursorSelectionMonId() as u32;
                    match sUnionRoomTrade.state {
                        URTRADE_STATE_REGISTERING => {
                            UpdateGameData_SetActivity(84, 0, TRUE as u32);
                            if id >= PARTY_SIZE as u32 {
                                ResetUnionRoomTrade(&raw mut sUnionRoomTrade);
                                SetTradeBoardRegisteredMonInfo(0, 0, 0);
                                ScheduleFieldMessageAndExit(
                                    sText_RegistrationCanceled.as_ptr().cast_mut(),
                                );
                            } else if RegisterTradeMonAndGetIsEgg(
                                GetCursorSelectionMonId() as u32,
                                &raw mut sUnionRoomTrade,
                            ) == 0
                            {
                                ScheduleFieldMessageWithFollowupState(
                                    UR_STATE_REGISTER_REQUEST_TYPE,
                                    sText_ChooseRequestedMonType.as_ptr().cast_mut(),
                                );
                            } else {
                                (*uroom).state = UR_STATE_REGISTER_COMPLETE;
                            }
                        }
                        URTRADE_STATE_OFFERING => {
                            CopyPlayerListFromBuffer(uroom);
                            *taskData.at(1) = sUnionRoomTrade.offerPlayerId as i16;
                            if id >= PARTY_SIZE as u32 {
                                ScheduleFieldMessageAndExit(
                                    sText_TradeCanceled.as_ptr().cast_mut(),
                                );
                            } else {
                                UpdateGameData_SetActivity(84, 0, TRUE as u32);
                                gPlayerCurrActivity = 68;
                                RegisterTradeMon(
                                    GetCursorSelectionMonId() as u32,
                                    &raw mut sUnionRoomTrade,
                                );
                                (*uroom).state = UR_STATE_TRADE_OFFER_MON;
                            }
                        }
                        _ => {}
                    }
                    sUnionRoomTrade.state = URTRADE_STATE_NONE;
                } else {
                    (*uroom).state = UR_STATE_MAIN;
                }
            }
            UR_STATE_MAIN => {
                if gSpecialVar_Result != 0 {
                    if gSpecialVar_Result == UR_INTERACT_ATTENDANT {
                        UpdateGameData_SetActivity(84, 0, TRUE as u32);
                        PlaySE(SE_PC_LOGIN);
                        StringCopy(
                            gStringVar1.as_mut_ptr(),
                            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                        );
                        (*uroom).state = UR_STATE_INTERACT_WITH_ATTENDANT;
                        gSpecialVar_Result = 0;
                    } else if gSpecialVar_Result == UR_INTERACT_START_MENU {
                        UpdateGameData_SetActivity(84, 0, TRUE as u32);
                        (*uroom).state = UR_STATE_WAIT_FOR_START_MENU;
                        gSpecialVar_Result = 0;
                    } else {
                        *taskData = 0;
                        *taskData.at(1) = gSpecialVar_Result as i16 - 1;
                        (*uroom).state = UR_STATE_INTERACT_WITH_PLAYER;
                        gSpecialVar_Result = 0;
                    }
                } else if ArePlayerFieldControlsLocked() != TRUE {
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        if TryInteractWithUnionRoomMember(
                            (*uroom).playerList,
                            taskData,
                            taskData.at(1),
                            (*uroom).spriteIds.as_mut_ptr(),
                        ) != 0
                        {
                            PlaySE(SE_SELECT);
                            StartScriptInteraction();
                            (*uroom).state = UR_STATE_INTERACT_WITH_PLAYER;
                            break 'l1;
                        } else if IsPlayerFacingTradingBoard() != 0 {
                            UpdateGameData_SetActivity(84, 0, TRUE as u32);
                            PlaySE(SE_PC_LOGIN);
                            StartScriptInteraction();
                            StringCopy(
                                gStringVar1.as_mut_ptr(),
                                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                            );
                            (*uroom).state = UR_STATE_CHECK_TRADING_BOARD;
                            break 'l1;
                        }
                    }
                    'l3: {
                        let sw2: u8 = HandlePlayerListUpdate();
                        let mut fall = false;
                        if sw2 == PLIST_NEW_PLAYER {
                            fall = true;
                            PlaySE(SE_PC_LOGIN);
                        }
                        if fall || sw2 == 2 {
                            fall = true;
                            ScheduleUnionRoomPlayerRefresh(uroom);
                            break 'l3;
                        }
                        if sw2 == PLIST_CONTACTED {
                            fall = true;
                            (*uroom).state = UR_STATE_PLAYER_CONTACTED_YOU;
                            StartScriptInteraction();
                            SetTradeBoardRegisteredMonInfo(0, 0, 0);
                            UpdateGameData_SetActivity(
                                83,
                                GetActivePartnersInfo(uroom) as u32,
                                FALSE as u32,
                            );
                            break 'l3;
                        }
                    }
                    HandleUnionRoomPlayerRefresh(uroom);
                }
            }
            UR_STATE_WAIT_FOR_START_MENU => {
                if FuncIsActiveTask(Some(Task_ShowStartMenu)) == 0 {
                    UpdateGameData_SetActivity(IN_UNION_ROOM, 0, 0);
                    (*uroom).state = UR_STATE_MAIN;
                }
            }
            UR_STATE_INTERACT_WITH_PLAYER => {
                UR_RunTextPrinters();
                playerGender =
                    GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                UpdateGameData_SetActivity(84, 0, TRUE as u32);
                match UnionRoomGetPlayerInteractionResponse(
                    (*uroom).playerList,
                    *taskData as u8,
                    *taskData.at(1) as u8,
                    playerGender as u32,
                ) {
                    0 => {
                        (*uroom).state = UR_STATE_PRINT_AND_EXIT;
                    }
                    1 => {
                        TryConnectToUnionRoomParent(
                            (*(*uroom).playerList).players[*taskData.at(1)]
                                .rfu
                                .name
                                .as_mut_ptr(),
                            &raw mut (*(*uroom).playerList).players[*taskData.at(1)].rfu.data,
                            gPlayerCurrActivity,
                        );
                        (*uroom).unreadPlayerId = id as u16;
                        (*uroom).state = UR_STATE_TRY_COMMUNICATING;
                    }
                    2 => {
                        ScheduleFieldMessageWithFollowupState(
                            UR_STATE_RECV_JOIN_CHAT_REQUEST,
                            gStringVar4.as_mut_ptr(),
                        );
                    }
                    _ => {}
                }
            }
            UR_STATE_TRY_COMMUNICATING => {
                UR_RunTextPrinters();
                match RfuGetStatus() {
                    RFU_STATUS_NEW_CHILD_DETECTED => {
                        HandleCancelActivity(TRUE as u32);
                        (*uroom).state = UR_STATE_MAIN;
                    }
                    RFU_STATUS_FATAL_ERROR | RFU_STATUS_CONNECTION_ERROR => {
                        if IsUnionRoomListenTaskActive() == TRUE as u32 {
                            ScheduleFieldMessageAndExit(
                                sText_TrainerAppearsBusy.as_ptr().cast_mut(),
                            );
                        } else {
                            ScheduleFieldMessageWithFollowupState(
                                UR_STATE_CANCEL_ACTIVITY_LINK_ERROR,
                                sText_TrainerAppearsBusy.as_ptr().cast_mut(),
                            );
                        }
                        gPlayerCurrActivity = IN_UNION_ROOM;
                    }
                    _ => {}
                }
                if gReceivedRemoteLinkPlayers != 0 {
                    CreateTrainerCardInBuffer(
                        gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                        TRUE as u32,
                    );
                    CreateTask(Some(Task_ExchangeCards), 5);
                    (*uroom).state = UR_STATE_COMMUNICATING_WAIT_FOR_DATA;
                }
            }
            UR_STATE_COMMUNICATING_WAIT_FOR_DATA => {
                if FuncIsActiveTask(Some(Task_ExchangeCards)) == 0 {
                    if gPlayerCurrActivity == 68 {
                        ScheduleFieldMessageWithFollowupState(
                            UR_STATE_SEND_TRADE_REQUST,
                            sText_AwaitingPlayersResponseAboutTrade.as_ptr().cast_mut(),
                        );
                    } else {
                        (*uroom).state = UR_STATE_DO_SOMETHING_PROMPT;
                    }
                }
            }
            30 => {
                if gReceivedRemoteLinkPlayers == 0 {
                    HandleCancelActivity(FALSE as u32);
                    UpdateUnionRoomMemberFacing(
                        *taskData as u32,
                        *taskData.at(1) as u32,
                        (*uroom).playerList,
                    );
                    (*uroom).state = UR_STATE_INIT_LINK;
                }
            }
            UR_STATE_DO_SOMETHING_PROMPT => {
                id = ConvPartnerUnameAndGetWhetherMetAlready(
                    &raw mut (*(*uroom).playerList).players[*taskData.at(1)],
                );
                playerGender =
                    GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                ScheduleFieldMessageWithFollowupState(
                    UR_STATE_HANDLE_DO_SOMETHING_PROMPT_INPUT,
                    sHiDoSomethingTexts[id][playerGender],
                );
            }
            6 => {
                input = ListMenuHandler_AllItemsAvailable(
                    &raw mut (*uroom).textState,
                    &raw mut (*uroom).topListMenuWindowId,
                    &raw mut (*uroom).topListMenuId,
                    (&raw const *sWindowTemplate_InviteToActivity).cast_mut(),
                    (&raw const *sListMenuTemplate_InviteToActivity).cast_mut(),
                );
                if input != LIST_NOTHING_CHOSEN {
                    if gReceivedRemoteLinkPlayers == 0 {
                        (*uroom).state = UR_STATE_TRAINER_APPEARS_BUSY;
                    } else {
                        (*uroom).partnerYesNoResponse = 0;
                        playerGender =
                            GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                        if input == LIST_CANCEL || input == IN_UNION_ROOM as i32 {
                            (*uroom).playerSendBuffer[0] = IN_UNION_ROOM as u16;
                            Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                            StringCopy(
                                gStringVar4.as_mut_ptr(),
                                sIfYouWantToDoSomethingTexts[gLinkPlayers[0].gender],
                            );
                            (*uroom).state = UR_STATE_REQUEST_DECLINED;
                        } else {
                            gPlayerCurrActivity = input as u8;
                            sPlayerActivityGroupSize = (input as u32 >> 8) as u8;
                            if gPlayerCurrActivity == 65 && HasAtLeastTwoMonsOfLevel30OrLower() == 0
                            {
                                ScheduleFieldMessageWithFollowupState(
                                    UR_STATE_DO_SOMETHING_PROMPT as u32,
                                    sText_NeedTwoMonsOfLevel30OrLower1.as_ptr().cast_mut(),
                                );
                            } else {
                                (*uroom).playerSendBuffer[0] =
                                    gPlayerCurrActivity as u16 | IN_UNION_ROOM as u16;
                                Rfu_SendPacket(
                                    (*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void
                                );
                                (*uroom).state = UR_STATE_SEND_ACTIVITY_REQUEST;
                            }
                        }
                    }
                }
            }
            UR_STATE_TRAINER_APPEARS_BUSY => {
                StringCopy(
                    gStringVar4.as_mut_ptr(),
                    sText_TrainerBattleBusy.as_ptr().cast_mut(),
                );
                (*uroom).state = UR_STATE_CANCEL_REQUEST_PRINT_MSG;
            }
            UR_STATE_SEND_ACTIVITY_REQUEST => {
                PollPartnerYesNoResponse(uroom);
                playerGender =
                    GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                id = GetResponseIdx_InviteToURoomActivity(
                    (*uroom).playerSendBuffer[0] as i32 & 0x3F,
                );
                if PrintOnTextbox(
                    &raw mut (*uroom).textState,
                    sText_WaitOrShowCardTexts[playerGender][id],
                ) != 0
                {
                    *taskData.at(3) = 0;
                    (*uroom).state = UR_STATE_WAIT_FOR_RESPONSE_TO_REQUEST;
                }
            }
            UR_STATE_REQUEST_DECLINED => {
                SetCloseLinkCallback();
                (*uroom).state = UR_STATE_CANCEL_REQUEST_PRINT_MSG;
            }
            31 => {
                (*uroom).playerSendBuffer[0] = 68;
                (*uroom).playerSendBuffer[1] = sUnionRoomTrade.species;
                (*uroom).playerSendBuffer[2] = sUnionRoomTrade.level;
                Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                (*uroom).state = UR_STATE_WAIT_FOR_RESPONSE_TO_REQUEST;
            }
            UR_STATE_WAIT_FOR_RESPONSE_TO_REQUEST => {
                if gReceivedRemoteLinkPlayers == 0 {
                    StringCopy(
                        gStringVar4.as_mut_ptr(),
                        sText_TrainerBattleBusy.as_ptr().cast_mut(),
                    );
                    (*uroom).state = UR_STATE_TRAINER_APPEARS_BUSY;
                } else {
                    PollPartnerYesNoResponse(uroom);
                    if (*uroom).partnerYesNoResponse == 81 {
                        if gPlayerCurrActivity == ACTIVITY_CARD {
                            ViewURoomPartnerTrainerCard(gStringVar4.as_mut_ptr(), uroom, FALSE);
                            (*uroom).state = UR_STATE_PRINT_CARD_INFO;
                        } else {
                            (*uroom).state = UR_STATE_PRINT_START_ACTIVITY_MSG;
                        }
                    } else if (*uroom).partnerYesNoResponse == 82 {
                        (*uroom).state = UR_STATE_REQUEST_DECLINED;
                        GetURoomActivityRejectMsg(
                            gStringVar4.as_mut_ptr(),
                            gPlayerCurrActivity as i32 | IN_UNION_ROOM as i32,
                            gLinkPlayers[0].gender as u32,
                        );
                        gPlayerCurrActivity = ACTIVITY_NONE;
                    }
                }
            }
            UR_STATE_DO_SOMETHING_PROMPT_2 => {
                id = ConvPartnerUnameAndGetWhetherMetAlready(
                    &raw mut (*(*uroom).playerList).players[*taskData.at(1)],
                );
                playerGender =
                    GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                ScheduleFieldMessageWithFollowupState(
                    UR_STATE_HANDLE_DO_SOMETHING_PROMPT_INPUT,
                    sHiDoSomethingTexts[id][playerGender],
                );
            }
            UR_STATE_PRINT_CARD_INFO => {
                if PrintOnTextbox(&raw mut (*uroom).textState, gStringVar4.as_mut_ptr()) != 0 {
                    (*uroom).state = UR_STATE_WAIT_FINISH_READING_CARD;
                    SetLinkStandbyCallback();
                    (*uroom).partnerYesNoResponse = 0;
                    (*uroom).recvActivityRequest[0] = 0;
                }
            }
            UR_STATE_WAIT_FINISH_READING_CARD => {
                if IsLinkTaskFinished() != 0 {
                    if GetMultiplayerId() == 0 {
                        StringCopy(
                            gStringVar1.as_mut_ptr(),
                            gLinkPlayers[GetMultiplayerId() as i32 ^ 1]
                                .name
                                .as_mut_ptr(),
                        );
                        id = PlayerHasMetTrainerBefore(
                            gLinkPlayers[1].trainerId as u16,
                            gLinkPlayers[1].name.as_mut_ptr(),
                        );
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            sAwaitingResponseTexts[id],
                        );
                        (*uroom).state = UR_STATE_PRINT_CONTACT_MSG;
                    } else {
                        (*uroom).state = UR_STATE_DO_SOMETHING_PROMPT_2;
                    }
                }
            }
            19 => match UnionRoomHandleYesNo(&raw mut (*uroom).textState, FALSE as u32) {
                0 => {
                    CopyBgTilemapBufferToVram(0);
                    gPlayerCurrActivity = 69;
                    UpdateGameData_SetActivity(69, 0, TRUE as u32);
                    TryConnectToUnionRoomParent(
                        (*(*uroom).playerList).players[*taskData.at(1)]
                            .rfu
                            .name
                            .as_mut_ptr(),
                        &raw mut (*(*uroom).playerList).players[*taskData.at(1)].rfu.data,
                        gPlayerCurrActivity,
                    );
                    (*uroom).unreadPlayerId = *taskData.at(1) as u16;
                    (*uroom).state = UR_STATE_TRY_ACCEPT_CHAT_REQUEST_DELAY;
                    *taskData.at(3) = 0;
                }
                1 | MENU_B_PRESSED => {
                    playerGender =
                        GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                    ScheduleFieldMessageAndExit(sDeclineChatTexts[playerGender]);
                }
                _ => {}
            },
            UR_STATE_TRY_ACCEPT_CHAT_REQUEST_DELAY => {
                if ({
                    *taskData.at(2) += 1;
                    *taskData.at(2)
                }) > 60
                {
                    (*uroom).state = UR_STATE_TRY_ACCEPT_CHAT_REQUEST;
                    *taskData.at(2) = 0;
                }
            }
            UR_STATE_TRY_ACCEPT_CHAT_REQUEST => {
                match RfuGetStatus() {
                    RFU_STATUS_NEW_CHILD_DETECTED => {
                        HandleCancelActivity(TRUE as u32);
                        (*uroom).state = UR_STATE_MAIN;
                    }
                    RFU_STATUS_FATAL_ERROR | RFU_STATUS_CONNECTION_ERROR => {
                        playerGender =
                            GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                        UpdateGameData_SetActivity(84, 0, TRUE as u32);
                        if IsUnionRoomListenTaskActive() == TRUE as u32 {
                            ScheduleFieldMessageAndExit(sChatDeclinedTexts[playerGender]);
                        } else {
                            ScheduleFieldMessageWithFollowupState(
                                UR_STATE_CANCEL_ACTIVITY_LINK_ERROR,
                                sChatDeclinedTexts[playerGender],
                            );
                        }
                    }
                    RFU_STATUS_CHILD_SEND_COMPLETE => {
                        (*uroom).state = UR_STATE_ACCEPT_CHAT_REQUEST;
                    }
                    _ => {}
                }
                *taskData.at(3) += 1;
            }
            UR_STATE_ACCEPT_CHAT_REQUEST => {
                if RfuHasErrored() != 0 {
                    playerGender =
                        GetUnionRoomPlayerGender(*taskData.at(1) as i32, (*uroom).playerList);
                    UpdateGameData_SetActivity(84, 0, TRUE as u32);
                    if IsUnionRoomListenTaskActive() == TRUE as u32 {
                        ScheduleFieldMessageAndExit(sChatDeclinedTexts[playerGender]);
                    } else {
                        ScheduleFieldMessageWithFollowupState(
                            UR_STATE_CANCEL_ACTIVITY_LINK_ERROR,
                            sChatDeclinedTexts[playerGender],
                        );
                    }
                }
                if gReceivedRemoteLinkPlayers != 0 {
                    (*uroom).state = UR_STATE_START_ACTIVITY_FREE_UROOM;
                }
            }
            UR_STATE_PLAYER_CONTACTED_YOU => {
                PlaySE(SE_DING_DONG);
                StopUnionRoomLinkManager();
                (*uroom).state = UR_STATE_RECV_CONTACT_DATA;
                (*uroom).recvActivityRequest[0] = 0;
            }
            UR_STATE_RECV_CONTACT_DATA => {
                if RfuHasErrored() != 0 {
                    HandleCancelActivity(FALSE as u32);
                    (*uroom).state = UR_STATE_INIT_LINK;
                } else if gReceivedRemoteLinkPlayers != 0 {
                    CreateTrainerCardInBuffer(
                        gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                        TRUE as u32,
                    );
                    CreateTask(Some(Task_ExchangeCards), 5);
                    (*uroom).state = UR_STATE_WAIT_FOR_CONTACT_DATA;
                }
            }
            UR_STATE_WAIT_FOR_CONTACT_DATA => {
                ReceiveUnionRoomActivityPacket(uroom);
                if FuncIsActiveTask(Some(Task_ExchangeCards)) == 0 {
                    (*uroom).state = UR_STATE_PRINT_CONTACT_MSG;
                    StringCopy(gStringVar1.as_mut_ptr(), gLinkPlayers[1].name.as_mut_ptr());
                    id = PlayerHasMetTrainerBefore(
                        gLinkPlayers[1].trainerId as u16,
                        gLinkPlayers[1].name.as_mut_ptr(),
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        sPlayerContactedYouTexts[id],
                    );
                }
            }
            UR_STATE_PRINT_CONTACT_MSG => {
                ReceiveUnionRoomActivityPacket(uroom);
                if PrintOnTextbox(&raw mut (*uroom).textState, gStringVar4.as_mut_ptr()) != 0 {
                    (*uroom).state = UR_STATE_HANDLE_CONTACT_DATA;
                }
            }
            UR_STATE_HANDLE_CONTACT_DATA => {
                ReceiveUnionRoomActivityPacket(uroom);
                if HandleContactFromOtherPlayer(uroom) != 0 && gMain.newKeys as i32 & B_BUTTON != 0
                {
                    Rfu_DisconnectPlayerById(1);
                    StringCopy(
                        gStringVar4.as_mut_ptr(),
                        sText_ChatEnded.as_ptr().cast_mut(),
                    );
                    (*uroom).state = UR_STATE_CANCEL_REQUEST_PRINT_MSG;
                }
            }
            UR_STATE_RECV_ACTIVITY_REQUEST => {
                ScheduleFieldMessageWithFollowupState(
                    UR_STATE_HANDLE_ACTIVITY_REQUEST,
                    gStringVar4.as_mut_ptr(),
                );
            }
            9 => match UnionRoomHandleYesNo(&raw mut (*uroom).textState, FALSE as u32) {
                0 => {
                    (*uroom).playerSendBuffer[0] = 81;
                    if gPlayerCurrActivity == 69 {
                        UpdateGameData_SetActivity(
                            gPlayerCurrActivity | IN_UNION_ROOM,
                            GetLinkPlayerInfoFlags(1) as u32,
                            FALSE as u32,
                        );
                    } else {
                        UpdateGameData_SetActivity(
                            gPlayerCurrActivity | IN_UNION_ROOM,
                            GetLinkPlayerInfoFlags(1) as u32,
                            1,
                        );
                    }
                    (*(*uroom).spawnPlayer).players[0].newPlayerCountdown = 0;
                    *taskData.at(3) = 0;
                    if gPlayerCurrActivity == 65 {
                        if HasAtLeastTwoMonsOfLevel30OrLower() == 0 {
                            (*uroom).playerSendBuffer[0] = 82;
                            Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                            (*uroom).state = UR_STATE_DECLINE_ACTIVITY_REQUEST;
                            StringCopy(
                                gStringVar4.as_mut_ptr(),
                                sText_NeedTwoMonsOfLevel30OrLower2.as_ptr().cast_mut(),
                            );
                        } else {
                            Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                            (*uroom).state = UR_STATE_PRINT_START_ACTIVITY_MSG;
                        }
                    } else if gPlayerCurrActivity == 72 {
                        Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                        ViewURoomPartnerTrainerCard(gStringVar4.as_mut_ptr(), uroom, TRUE);
                        (*uroom).state = UR_STATE_PRINT_CARD_INFO;
                    } else {
                        Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                        (*uroom).state = UR_STATE_PRINT_START_ACTIVITY_MSG;
                    }
                }
                1 | MENU_B_PRESSED => {
                    (*uroom).playerSendBuffer[0] = 82;
                    Rfu_SendPacket((*uroom).playerSendBuffer.as_mut_ptr() as *mut c_void);
                    (*uroom).state = UR_STATE_DECLINE_ACTIVITY_REQUEST;
                    GetYouDeclinedTheOfferMessage(gStringVar4.as_mut_ptr(), gPlayerCurrActivity);
                }
                _ => {}
            },
            UR_STATE_DECLINE_ACTIVITY_REQUEST => {
                SetCloseLinkCallback();
                (*uroom).state = UR_STATE_CANCEL_REQUEST_PRINT_MSG;
            }
            UR_STATE_CANCEL_REQUEST_PRINT_MSG => {
                if gReceivedRemoteLinkPlayers == 0 {
                    gPlayerCurrActivity = IN_UNION_ROOM;
                    ScheduleFieldMessageWithFollowupState(
                        UR_STATE_CANCEL_REQUEST_RESTART_LINK,
                        gStringVar4.as_mut_ptr(),
                    );
                    memset((*uroom).playerSendBuffer.as_mut_ptr() as *mut u8, 0, 12);
                    (*uroom).recvActivityRequest[0] = 0;
                    (*uroom).partnerYesNoResponse = 0;
                }
            }
            37 => {
                (*uroom).state = UR_STATE_INIT_LINK;
                HandleCancelActivity(FALSE as u32);
            }
            UR_STATE_PRINT_START_ACTIVITY_MSG => {
                GetURoomActivityStartMsg(
                    gStringVar4.as_mut_ptr(),
                    gPlayerCurrActivity | IN_UNION_ROOM,
                );
                ScheduleFieldMessageWithFollowupState(
                    UR_STATE_START_ACTIVITY_LINK,
                    gStringVar4.as_mut_ptr(),
                );
            }
            14 => {
                SetLinkStandbyCallback();
                (*uroom).state = UR_STATE_START_ACTIVITY_WAIT_FOR_LINK;
            }
            UR_STATE_START_ACTIVITY_WAIT_FOR_LINK => {
                if IsLinkTaskFinished() != 0 {
                    (*uroom).state = UR_STATE_START_ACTIVITY_FREE_UROOM;
                }
            }
            UR_STATE_START_ACTIVITY_FREE_UROOM => {
                Free((*uroom).spawnPlayer as *mut c_void);
                Free((*uroom).playerList as *mut c_void);
                Free((*uroom).incomingParentList as *mut c_void);
                Free((*uroom).incomingChildList as *mut c_void);
                DestroyTask((*uroom).searchTaskId);
                DestroyUnionRoomPlayerSprites((*uroom).spriteIds.as_mut_ptr());
                (*uroom).state = UR_STATE_START_ACTIVITY_FADE;
            }
            UR_STATE_START_ACTIVITY_FADE => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                (*uroom).state = UR_STATE_START_ACTIVITY;
            }
            UR_STATE_START_ACTIVITY => {
                if UpdatePaletteFade() == 0 {
                    DestroyUnionRoomPlayerObjects();
                    DestroyTask(taskId);
                    Free(sWirelessLinkMain.uRoom as *mut c_void);
                    CreateTask_StartActivity();
                }
            }
            UR_STATE_INTERACT_WITH_ATTENDANT => {
                if (*GetHostRfuGameData()).tradeSpecies() == SPECIES_NONE {
                    (*uroom).state = UR_STATE_REGISTER_PROMPT;
                } else {
                    if (*GetHostRfuGameData()).tradeSpecies() == SPECIES_EGG as u16 {
                        StringCopy(
                            gStringVar4.as_mut_ptr(),
                            sText_CancelRegistrationOfEgg.as_ptr().cast_mut(),
                        );
                    } else {
                        StringCopy(
                            gStringVar1.as_mut_ptr(),
                            gSpeciesNames[(*GetHostRfuGameData()).tradeSpecies()]
                                .as_ptr()
                                .cast_mut(),
                        );
                        ConvertIntToDecimalStringN(
                            gStringVar2.as_mut_ptr(),
                            (*GetHostRfuGameData()).tradeLevel() as i32,
                            STR_CONV_MODE_LEFT_ALIGN,
                            3,
                        );
                        StringExpandPlaceholders(
                            gStringVar4.as_mut_ptr(),
                            sText_CancelRegistrationOfMon.as_ptr().cast_mut(),
                        );
                    }
                    ScheduleFieldMessageWithFollowupState(
                        UR_STATE_CANCEL_REGISTRATION_PROMPT,
                        gStringVar4.as_mut_ptr(),
                    );
                }
            }
            UR_STATE_REGISTER_PROMPT => {
                if PrintOnTextbox(
                    &raw mut (*uroom).textState,
                    sText_RegisterMonAtTradingBoard.as_ptr().cast_mut(),
                ) != 0
                {
                    (*uroom).state = UR_STATE_REGISTER_PROMPT_HANDLE_INPUT;
                }
            }
            UR_STATE_REGISTER_PROMPT_HANDLE_INPUT => {
                input = ListMenuHandler_AllItemsAvailable(
                    &raw mut (*uroom).textState,
                    &raw mut (*uroom).tradeBoardMainWindowId,
                    &raw mut (*uroom).tradeBoardHeaderWindowId,
                    (&raw const *sWindowTemplate_RegisterForTrade).cast_mut(),
                    (&raw const *sListMenuTemplate_RegisterForTrade).cast_mut(),
                );
                if input != LIST_NOTHING_CHOSEN {
                    if input == LIST_CANCEL || input == 3 {
                        (*uroom).state = UR_STATE_MAIN;
                        HandleCancelActivity(TRUE as u32);
                    } else {
                        match input {
                            1 => {
                                ScheduleFieldMessageWithFollowupState(
                                    UR_STATE_REGISTER_SELECT_MON_FADE,
                                    sText_WhichMonWillYouOffer.as_ptr().cast_mut(),
                                );
                            }
                            2 => {
                                ScheduleFieldMessageWithFollowupState(
                                    UR_STATE_REGISTER_PROMPT_HANDLE_INPUT as u32,
                                    sText_TradingBoardInfo.as_ptr().cast_mut(),
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
            53 => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                (*uroom).state = UR_STATE_REGISTER_SELECT_MON;
            }
            UR_STATE_REGISTER_SELECT_MON => {
                if gPaletteFade.active() == 0 {
                    sUnionRoomTrade.state = URTRADE_STATE_REGISTERING;
                    gFieldCallback = Some(FieldCB_ContinueScriptUnionRoom);
                    ChooseMonForTradingBoard(
                        PARTY_MENU_TYPE_UNION_ROOM_REGISTER,
                        Some(CB2_ReturnToField),
                    );
                }
            }
            52 => {
                input = ListMenuHandler_AllItemsAvailable(
                    &raw mut (*uroom).textState,
                    &raw mut (*uroom).tradeBoardMainWindowId,
                    &raw mut (*uroom).tradeBoardHeaderWindowId,
                    (&raw const *sWindowTemplate_TradingBoardRequestType).cast_mut(),
                    (&raw const *sMenuTemplate_TradingBoardRequestType).cast_mut(),
                );
                if input != LIST_NOTHING_CHOSEN {
                    match input {
                        LIST_CANCEL | 18 => {
                            ResetUnionRoomTrade(&raw mut sUnionRoomTrade);
                            SetTradeBoardRegisteredMonInfo(0, 0, 0);
                            ScheduleFieldMessageAndExit(
                                sText_RegistrationCanceled.as_ptr().cast_mut(),
                            );
                        }
                        _ => {
                            sUnionRoomTrade.r#type = input as u16;
                            (*uroom).state = UR_STATE_REGISTER_COMPLETE;
                        }
                    }
                }
            }
            UR_STATE_REGISTER_COMPLETE => {
                SetTradeBoardRegisteredMonInfo(
                    sUnionRoomTrade.r#type as u32,
                    sUnionRoomTrade.playerSpecies as u32,
                    sUnionRoomTrade.playerLevel as u32,
                );
                ScheduleFieldMessageAndExit(sText_RegistrationCompleted.as_ptr().cast_mut());
            }
            44 => match UnionRoomHandleYesNo(&raw mut (*uroom).textState, FALSE as u32) {
                0 => {
                    (*uroom).state = UR_STATE_CANCEL_REGISTRATION;
                }
                1 | MENU_B_PRESSED => {
                    HandleCancelActivity(TRUE as u32);
                    (*uroom).state = UR_STATE_MAIN;
                }
                _ => {}
            },
            UR_STATE_CANCEL_REGISTRATION => {
                if PrintOnTextbox(
                    &raw mut (*uroom).textState,
                    sText_RegistrationCanceled2.as_ptr().cast_mut(),
                ) != 0
                {
                    SetTradeBoardRegisteredMonInfo(0, 0, 0);
                    ResetUnionRoomTrade(&raw mut sUnionRoomTrade);
                    HandleCancelActivity(TRUE as u32);
                    (*uroom).state = UR_STATE_MAIN;
                }
            }
            UR_STATE_CHECK_TRADING_BOARD => {
                if PrintOnTextbox(
                    &raw mut (*uroom).textState,
                    sText_XCheckedTradingBoard.as_ptr().cast_mut(),
                ) != 0
                {
                    (*uroom).state = UR_STATE_TRADING_BOARD_LOAD;
                }
            }
            UR_STATE_TRADING_BOARD_LOAD => {
                UR_ClearBg0();
                (*uroom).state = UR_STATE_TRADING_BOARD_HANDLE_INPUT;
            }
            UR_STATE_TRADING_BOARD_HANDLE_INPUT => {
                input = TradeBoardMenuHandler(
                    &raw mut (*uroom).textState,
                    &raw mut (*uroom).tradeBoardMainWindowId,
                    &raw mut (*uroom).tradeBoardListMenuId,
                    &raw mut (*uroom).tradeBoardHeaderWindowId,
                    (&raw const *sWindowTemplate_TradingBoardMain).cast_mut(),
                    (&raw const *sTradeBoardListMenuTemplate).cast_mut(),
                    (*uroom).playerList,
                );
                if input != LIST_NOTHING_CHOSEN {
                    match input {
                        LIST_CANCEL | 8 => {
                            HandleCancelActivity(TRUE as u32);
                            (*uroom).state = UR_STATE_MAIN;
                        }
                        _ => {
                            UR_ClearBg0();
                            match IsRequestedTradeInPlayerParty(
                                (*(*uroom).playerList).players[input].rfu.data.tradeType() as u32,
                                (*(*uroom).playerList).players[input]
                                    .rfu
                                    .data
                                    .tradeSpecies() as u32,
                            ) {
                                UR_TRADE_MATCH => {
                                    CopyAndTranslatePlayerName(
                                        gStringVar1.as_mut_ptr(),
                                        &raw mut (*(*uroom).playerList).players[input],
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        UR_STATE_TRADE_PROMPT,
                                        sText_AskTrainerToMakeTrade.as_ptr().cast_mut(),
                                    );
                                    *taskData.at(1) = input as i16;
                                }
                                UR_TRADE_NOTYPE => {
                                    CopyAndTranslatePlayerName(
                                        gStringVar1.as_mut_ptr(),
                                        &raw mut (*(*uroom).playerList).players[input],
                                    );
                                    StringCopy(
                                        gStringVar2.as_mut_ptr(),
                                        gTypeNames[(*(*uroom).playerList).players[input]
                                            .rfu
                                            .data
                                            .tradeType()]
                                        .as_ptr()
                                        .cast_mut(),
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        UR_STATE_TRADING_BOARD_LOAD as u32,
                                        sText_DontHaveTypeTrainerWants.as_ptr().cast_mut(),
                                    );
                                }
                                UR_TRADE_NOEGG => {
                                    CopyAndTranslatePlayerName(
                                        gStringVar1.as_mut_ptr(),
                                        &raw mut (*(*uroom).playerList).players[input],
                                    );
                                    StringCopy(
                                        gStringVar2.as_mut_ptr(),
                                        gTypeNames[(*(*uroom).playerList).players[input]
                                            .rfu
                                            .data
                                            .tradeType()]
                                        .as_ptr()
                                        .cast_mut(),
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        UR_STATE_TRADING_BOARD_LOAD as u32,
                                        sText_DontHaveEggTrainerWants.as_ptr().cast_mut(),
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            49 => match UnionRoomHandleYesNo(&raw mut (*uroom).textState, FALSE as u32) {
                0 => {
                    (*uroom).state = UR_STATE_TRADE_SELECT_MON;
                }
                MENU_B_PRESSED | 1 => {
                    HandleCancelActivity(TRUE as u32);
                    (*uroom).state = UR_STATE_MAIN;
                }
                _ => {}
            },
            UR_STATE_TRADE_SELECT_MON => {
                if PrintOnTextbox(
                    &raw mut (*uroom).textState,
                    sText_WhichMonWillYouOffer.as_ptr().cast_mut(),
                ) != 0
                {
                    sUnionRoomTrade.state = URTRADE_STATE_OFFERING;
                    memcpy(
                        &raw mut gRfuPartnerCompatibilityData as *mut u8,
                        &raw mut (*(*uroom).playerList).players[*taskData.at(1)]
                            .rfu
                            .data
                            .compatibility as *mut u8,
                        4,
                    );
                    gUnionRoomRequestedMonType = (*(*uroom).playerList).players[*taskData.at(1)]
                        .rfu
                        .data
                        .tradeType() as u8;
                    gUnionRoomOfferedSpecies = (*(*uroom).playerList).players[*taskData.at(1)]
                        .rfu
                        .data
                        .tradeSpecies();
                    gFieldCallback = Some(FieldCB_ContinueScriptUnionRoom);
                    ChooseMonForTradingBoard(
                        PARTY_MENU_TYPE_UNION_ROOM_TRADE,
                        Some(CB2_ReturnToField),
                    );
                    CopyPlayerListToBuffer(uroom);
                    sUnionRoomTrade.offerPlayerId = *taskData.at(1) as u8;
                }
            }
            UR_STATE_TRADE_OFFER_MON => {
                gPlayerCurrActivity = 68;
                TryConnectToUnionRoomParent(
                    (*(*uroom).playerList).players[*taskData.at(1)]
                        .rfu
                        .name
                        .as_mut_ptr(),
                    &raw mut (*(*uroom).playerList).players[*taskData.at(1)].rfu.data,
                    gPlayerCurrActivity,
                );
                CopyAndTranslatePlayerName(
                    gStringVar1.as_mut_ptr(),
                    &raw mut (*(*uroom).playerList).players[*taskData.at(1)],
                );
                UR_PrintFieldMessage(sCommunicatingWaitTexts[2]);
                (*uroom).state = UR_STATE_TRY_COMMUNICATING;
            }
            UR_STATE_PRINT_AND_EXIT => {
                if PrintOnTextbox(&raw mut (*uroom).textState, gStringVar4.as_mut_ptr()) != 0 {
                    HandleCancelActivity(TRUE as u32);
                    UpdateUnionRoomMemberFacing(
                        *taskData as u32,
                        *taskData.at(1) as u32,
                        (*uroom).playerList,
                    );
                    (*uroom).state = UR_STATE_MAIN;
                }
            }
            UR_STATE_PRINT_MSG => {
                if PrintOnTextbox(&raw mut (*uroom).textState, gStringVar4.as_mut_ptr()) != 0 {
                    (*uroom).state = (*uroom).stateAfterPrint;
                }
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUsingUnionRoomStartMenu() {
    if InUnionRoom() == TRUE as u32 {
        gSpecialVar_Result = UR_INTERACT_START_MENU;
    }
}
pub(crate) unsafe extern "C" fn ReceiveUnionRoomActivityPacket(data: *mut WirelessLink_URoom) {
    if gRecvCmds[1][1] != 0 && gRecvCmds[1][0] as i32 & RFUCMD_MASK == RFUCMD_SEND_PACKET {
        (*data).recvActivityRequest[0] = gRecvCmds[1][1];
        if gRecvCmds[1][1] == 68 {
            (*data).recvActivityRequest[1] = gRecvCmds[1][2];
            (*data).recvActivityRequest[2] = gRecvCmds[1][3];
        }
    }
}
pub(crate) unsafe extern "C" fn HandleContactFromOtherPlayer(
    uroom: *mut WirelessLink_URoom,
) -> u32 {
    if (*uroom).recvActivityRequest[0] != 0 {
        let mut id: i32 = GetChatLeaderActionRequestMessage(
            gStringVar4.as_mut_ptr(),
            gLinkPlayers[1].gender as u32,
            &raw mut (*uroom).recvActivityRequest[0],
            uroom,
        );
        if id == 0 {
            return TRUE as u32;
        } else if id == 1 {
            (*uroom).state = UR_STATE_RECV_ACTIVITY_REQUEST;
            gPlayerCurrActivity = (*uroom).recvActivityRequest[0] as u8;
            return FALSE as u32;
        } else if id == 2 {
            (*uroom).state = UR_STATE_CANCEL_REQUEST_PRINT_MSG;
            SetCloseLinkCallback();
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoom() {
    let mut data: *mut WirelessLink_URoom = null_mut();
    sUnionRoomPlayerName[0] = EOS;
    CreateTask(Some(Task_InitUnionRoom), 0);
    sWirelessLinkMain.uRoom = sWirelessLinkMain.uRoom;
    sWirelessLinkMain.uRoom = {
        data = AllocZeroed(620) as *mut WirelessLink_URoom;
        data
    };
    sURoom = sWirelessLinkMain.uRoom;
    (*data).state = 0;
    (*data).textState = 0;
    (*data).unknown = 0;
    (*data).unreadPlayerId = 0;
    sUnionRoomPlayerName[0] = EOS;
}
pub(crate) unsafe extern "C" fn Task_InitUnionRoom(taskId: u8) {
    let mut i: i32 = 0;
    let mut text: CArray<u8, 32> = zeroed();
    let mut data: *mut WirelessLink_URoom = sWirelessLinkMain.uRoom;
    match (*data).state {
        0 => {
            (*data).state = 1;
        }
        1 => {
            SetHostRfuGameData(ACTIVITY_SEARCH, 0, 0);
            SetWirelessCommType1();
            OpenLink();
            InitializeRfuLinkManager_EnterUnionRoom();
            RfuSetIgnoreError(TRUE as u32);
            (*data).state = 2;
        }
        2 => {
            (*data).incomingChildList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            ClearIncomingPlayerList((*data).incomingChildList, RFU_CHILD_MAX);
            (*data).incomingParentList = AllocZeroed(112) as *mut RfuIncomingPlayerList;
            ClearIncomingPlayerList((*data).incomingParentList, RFU_CHILD_MAX);
            (*data).playerList = AllocZeroed(256) as *mut RfuPlayerList;
            ClearRfuPlayerList(
                (*(*data).playerList).players.as_mut_ptr(),
                MAX_UNION_ROOM_LEADERS as u8,
            );
            (*data).spawnPlayer = AllocZeroed(32) as *mut RfuPlayerList;
            ClearRfuPlayerList(&raw mut (*(*data).spawnPlayer).players[0], 1);
            (*data).searchTaskId = CreateTask_SearchForChildOrParent(
                (*data).incomingParentList,
                (*data).incomingChildList,
                LINK_GROUP_UNION_ROOM_INIT,
            );
            (*data).state = 3;
        }
        3 => match HandlePlayerListUpdate() {
            PLIST_NEW_PLAYER | 2 => {
                if sUnionRoomPlayerName[0] == EOS {
                    i = 0;
                    while i < MAX_UNION_ROOM_LEADERS {
                        if (*(*data).playerList).players[i].groupScheduledAnim()
                            == UNION_ROOM_SPAWN_IN
                        {
                            CopyAndTranslatePlayerName(
                                text.as_mut_ptr(),
                                &raw mut (*(*data).playerList).players[i],
                            );
                            if PlayerHasMetTrainerBefore(
                                ReadAsU16(
                                    (*(*data).playerList).players[i]
                                        .rfu
                                        .data
                                        .compatibility
                                        .playerTrainerId
                                        .as_mut_ptr(),
                                ),
                                text.as_mut_ptr(),
                            ) != 0
                            {
                                StringCopy(sUnionRoomPlayerName.as_mut_ptr(), text.as_mut_ptr());
                                break;
                            }
                        }
                        i += 1;
                    }
                }
            }
            PLIST_UNUSED => {}
            _ => {}
        },
        4 => {
            Free((*data).spawnPlayer as *mut c_void);
            Free((*data).playerList as *mut c_void);
            Free((*data).incomingParentList as *mut c_void);
            Free((*data).incomingChildList as *mut c_void);
            DestroyTask((*data).searchTaskId);
            Free(sWirelessLinkMain.uRoom as *mut c_void);
            LinkRfu_Shutdown();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferUnionRoomPlayerName() -> u16 {
    if sUnionRoomPlayerName[0] != EOS {
        StringCopy(gStringVar1.as_mut_ptr(), sUnionRoomPlayerName.as_mut_ptr());
        sUnionRoomPlayerName[0] = EOS;
        return TRUE as u16;
    } else {
        return FALSE as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandlePlayerListUpdate() -> u8 {
    let mut i: i32 = 0;
    let mut j: u8 = 0;
    let mut data: *mut WirelessLink_URoom = sWirelessLinkMain.uRoom;
    let mut retVal: i32 = PLIST_NONE;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        if ArePlayersDifferent(
            &raw mut (*(*data).incomingParentList).players[i].rfu,
            (&raw const *sUnionRoomPlayer_DummyRfu).cast_mut(),
        ) == TRUE
        {
            (*(*data).spawnPlayer).players[0].rfu = (*(*data).incomingParentList).players[i].rfu;
            (*(*data).spawnPlayer).players[0].timeoutCounter = 0;
            (*(*data).spawnPlayer).players[0].set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
            (*(*data).spawnPlayer).players[0].newPlayerCountdown = 1;
            return PLIST_CONTACTED;
        }
        i += 1;
    }
    j = 0;
    while j < MAX_UNION_ROOM_LEADERS as u8 {
        if (*(*data).playerList).players[j].groupScheduledAnim() != UNION_ROOM_SPAWN_NONE {
            i = GetNewIncomingPlayerId(
                &raw mut (*(*data).playerList).players[j],
                &raw mut (*(*data).incomingChildList).players[0],
            ) as i32;
            if i != 0xFF {
                if (*(*data).playerList).players[j].groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
                    if ArePlayerDataDifferent(
                        &raw mut (*(*data).playerList).players[j].rfu,
                        &raw mut (*(*data).incomingChildList).players[i].rfu,
                    ) != 0
                    {
                        (*(*data).playerList).players[j].rfu =
                            (*(*data).incomingChildList).players[i].rfu;
                        (*(*data).playerList).players[j].newPlayerCountdown = 64;
                        retVal = PLIST_NEW_PLAYER as i32;
                    } else if (*(*data).playerList).players[j].newPlayerCountdown != 0 {
                        (*(*data).playerList).players[j].newPlayerCountdown -= 1;
                        if (*(*data).playerList).players[j].newPlayerCountdown == 0 {
                            retVal = PLIST_RECENT_UPDATE;
                        }
                    }
                } else {
                    (*(*data).playerList).players[j].set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
                    (*(*data).playerList).players[j].newPlayerCountdown = 0;
                    retVal = PLIST_RECENT_UPDATE;
                }
                (*(*data).playerList).players[j].timeoutCounter = 0;
            } else if (*(*data).playerList).players[j].groupScheduledAnim() != UNION_ROOM_SPAWN_OUT
            {
                (*(*data).playerList).players[j].timeoutCounter += 1;
                if (*(*data).playerList).players[j].timeoutCounter >= 600 {
                    (*(*data).playerList).players[j].set_groupScheduledAnim(UNION_ROOM_SPAWN_OUT);
                    retVal = PLIST_RECENT_UPDATE;
                }
            } else if (*(*data).playerList).players[j].groupScheduledAnim() == UNION_ROOM_SPAWN_OUT
            {
                (*(*data).playerList).players[j].timeoutCounter += 1;
                if (*(*data).playerList).players[j].timeoutCounter >= 900 {
                    ClearRfuPlayerList(&raw mut (*(*data).playerList).players[j], 1);
                }
            }
        }
        j += 1;
    }
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        if TryAddIncomingPlayerToList(
            &raw mut (*(*data).playerList).players[0],
            &raw mut (*(*data).incomingChildList).players[i],
            MAX_UNION_ROOM_LEADERS as u8,
        ) != 0xFF
        {
            retVal = PLIST_NEW_PLAYER as i32;
        }
        i += 1;
    }
    return retVal as u8;
}
pub(crate) unsafe extern "C" fn Task_SearchForChildOrParent(taskId: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut rfu: RfuPlayerData = zeroed();
    let mut list: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    let mut isParent: u8 = 0;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        isParent = Rfu_GetCompatiblePlayerData(&raw mut rfu.data, rfu.name.as_mut_ptr(), i as u8);
        if IsPartnerActivityAcceptable(rfu.data.activity() as u32, gTasks[taskId].data[4] as u32)
            == 0
        {
            rfu = *sUnionRoomPlayer_DummyRfu;
        }
        if rfu.data.compatibility.language() == LANGUAGE_JAPANESE as u16 {
            rfu = *sUnionRoomPlayer_DummyRfu;
        }
        if isParent == 0 {
            j = 0;
            while j < i {
                if ArePlayersDifferent(&raw mut (*(*list.at(1))).players[j].rfu, &raw mut rfu) == 0
                {
                    rfu = *sUnionRoomPlayer_DummyRfu;
                }
                j += 1;
            }
            (*(*list.at(1))).players[i].rfu = rfu;
            (*(*list.at(1))).players[i].set_active(ArePlayersDifferent(
                &raw mut (*(*list.at(1))).players[i].rfu,
                (&raw const *sUnionRoomPlayer_DummyRfu).cast_mut(),
            ));
        } else {
            (*(*list)).players[i].rfu = rfu;
            (*(*list)).players[i].set_active(ArePlayersDifferent(
                &raw mut (*(*list)).players[i].rfu,
                (&raw const *sUnionRoomPlayer_DummyRfu).cast_mut(),
            ));
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateTask_SearchForChildOrParent(
    parentList: *mut RfuIncomingPlayerList,
    childList: *mut RfuIncomingPlayerList,
    linkGroup: u32,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_SearchForChildOrParent), 0);
    let mut data: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    *data = parentList;
    *data.at(1) = childList;
    gTasks[taskId].data[4] = linkGroup as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_ListenForCompatiblePartners(taskId: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut list: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        Rfu_GetCompatiblePlayerData(
            &raw mut (*(*list)).players[i].rfu.data,
            (*(*list)).players[i].rfu.name.as_mut_ptr(),
            i as u8,
        );
        if IsPartnerActivityAcceptable(
            (*(*list)).players[i].rfu.data.activity() as u32,
            gTasks[taskId].data[2] as u32,
        ) == 0
        {
            (*(*list)).players[i].rfu = *sUnionRoomPlayer_DummyRfu;
        }
        j = 0;
        while j < i {
            if ArePlayersDifferent(
                &raw mut (*(*list)).players[j].rfu,
                &raw mut (*(*list)).players[i].rfu,
            ) == 0
            {
                (*(*list)).players[i].rfu = *sUnionRoomPlayer_DummyRfu;
            }
            j += 1;
        }
        (*(*list)).players[i].set_active(ArePlayersDifferent(
            &raw mut (*(*list)).players[i].rfu,
            (&raw const *sUnionRoomPlayer_DummyRfu).cast_mut(),
        ));
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn HasWonderCardOrNewsByLinkGroup(
    data: *mut RfuGameData,
    linkGroup: i16,
) -> u32 {
    if linkGroup == LINK_GROUP_WONDER_CARD as i16 {
        if (*data).compatibility.hasCard() == 0 {
            return FALSE as u32;
        } else {
            return TRUE as u32;
        }
    } else if linkGroup == LINK_GROUP_WONDER_NEWS {
        if (*data).compatibility.hasNews() == 0 {
            return FALSE as u32;
        } else {
            return TRUE as u32;
        }
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_ListenForWonderDistributor(taskId: u8) {
    let mut i: i32 = 0;
    let mut list: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        if Rfu_GetWonderDistributorPlayerData(
            &raw mut (*(*list)).players[i].rfu.data,
            (*(*list)).players[i].rfu.name.as_mut_ptr(),
            i as u8,
        ) != 0
        {
            HasWonderCardOrNewsByLinkGroup(
                &raw mut (*(*list)).players[i].rfu.data,
                gTasks[taskId].data[2],
            );
        }
        (*(*list)).players[i].set_active(ArePlayersDifferent(
            &raw mut (*(*list)).players[i].rfu,
            (&raw const *sUnionRoomPlayer_DummyRfu).cast_mut(),
        ));
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateTask_ListenForCompatiblePartners(
    list: *mut RfuIncomingPlayerList,
    linkGroup: u32,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_ListenForCompatiblePartners), 0);
    let mut oldList: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    *oldList = list;
    gTasks[taskId].data[2] = linkGroup as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn CreateTask_ListenForWonderDistributor(
    list: *mut RfuIncomingPlayerList,
    linkGroup: u32,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_ListenForWonderDistributor), 0);
    let mut oldList: *mut *mut RfuIncomingPlayerList =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuIncomingPlayerList;
    *oldList = list;
    gTasks[taskId].data[2] = linkGroup as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn UR_PrintFieldMessage(src: *mut u8) -> u32 {
    LoadMessageBoxAndBorderGfx();
    DrawDialogueFrame(0, TRUE);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    AddTextPrinterWithCustomSpeedForMessage(FALSE, 1);
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn UR_RunTextPrinters() -> u32 {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PrintOnTextbox(textState: *mut u8, str: *mut u8) -> u8 {
    match *textState {
        0 => {
            LoadMessageBoxAndBorderGfx();
            DrawDialogueFrame(0, TRUE);
            StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
            AddTextPrinterForMessage_2(TRUE);
            *textState += 1;
        }
        1 => {
            if RunTextPrintersAndIsPrinter0Active() == 0 {
                *textState = 0;
                return TRUE;
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn UnionRoomHandleYesNo(state: *mut u8, noDraw: u32) -> i8 {
    let mut input: i8 = 0;
    match *state {
        0 => {
            if noDraw != 0 {
                return -3;
            }
            DisplayYesNoMenuDefaultYes();
            *state += 1;
        }
        1 => {
            if noDraw != 0 {
                EraseYesNoWindow();
                *state = 0;
                return -3;
            }
            input = Menu_ProcessInputNoWrapClearOnChoose();
            if input == MENU_B_PRESSED || input == 0 || input == 1 {
                *state = 0;
                return input;
            }
        }
        _ => {}
    }
    return MENU_NOTHING_CHOSEN;
}
pub(crate) unsafe extern "C" fn CreateTradeBoardWindow(template: *mut WindowTemplate) -> u8 {
    let mut windowId: u8 = AddWindow(template) as u8;
    DrawStdWindowFrame(windowId, FALSE);
    FillWindowPixelBuffer(windowId, 255);
    PrintUnionRoomText(
        windowId,
        FONT_NORMAL,
        sText_NameWantedOfferLv.as_ptr().cast_mut(),
        8,
        1,
        UR_COLOR_TRADE_BOARD_OTHER,
    );
    CopyWindowToVram(windowId, COPYWIN_GFX);
    PutWindowTilemap(windowId);
    return windowId;
}
pub(crate) unsafe extern "C" fn DeleteTradeBoardWindow(windowId: u8) {
    RemoveWindow(windowId);
}
pub(crate) unsafe extern "C" fn ListMenuHandler_AllItemsAvailable(
    state: *mut u8,
    windowId: *mut u8,
    listMenuId: *mut u8,
    winTemplate: *mut WindowTemplate,
    menuTemplate: *mut ListMenuTemplate,
) -> i32 {
    let mut maxWidth: i32 = 0;
    let mut input: i32 = 0;
    let mut winTemplateCopy: WindowTemplate = zeroed();
    match *state {
        0 => {
            winTemplateCopy = *winTemplate;
            maxWidth = Intl_GetListMenuWidth(menuTemplate);
            if winTemplateCopy.width as i32 > maxWidth {
                winTemplateCopy.width = maxWidth as u8;
            }
            if winTemplateCopy.tilemapLeft as i32 + winTemplateCopy.width as i32
                >= DISPLAY_TILE_WIDTH as i32
            {
                winTemplateCopy.tilemapLeft = (if 29 - winTemplateCopy.width as i32 >= 0 {
                    29 - winTemplateCopy.width as i32
                } else {
                    0
                }) as u8;
            }
            *windowId = AddWindow(&raw mut winTemplateCopy) as u8;
            DrawStdWindowFrame(*windowId, FALSE);
            gMultiuseListMenuTemplate = *menuTemplate;
            gMultiuseListMenuTemplate.windowId = *windowId;
            *listMenuId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0);
            CopyWindowToVram(*windowId, COPYWIN_MAP);
            *state += 1;
        }
        1 => {
            input = ListMenu_ProcessInput(*listMenuId);
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                DestroyListMenuTask(*listMenuId, null_mut(), null_mut());
                ClearStdWindowAndFrame(*windowId, TRUE);
                RemoveWindow(*windowId);
                *state = 0;
                return input;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                DestroyListMenuTask(*listMenuId, null_mut(), null_mut());
                ClearStdWindowAndFrame(*windowId, TRUE);
                RemoveWindow(*windowId);
                *state = 0;
                return LIST_CANCEL;
            }
        }
        _ => {}
    }
    return LIST_NOTHING_CHOSEN;
}
pub(crate) unsafe extern "C" fn TradeBoardMenuHandler(
    state: *mut u8,
    mainWindowId: *mut u8,
    listMenuId: *mut u8,
    headerWindowId: *mut u8,
    winTemplate: *mut WindowTemplate,
    menuTemplate: *mut ListMenuTemplate,
    list: *mut RfuPlayerList,
) -> i32 {
    let mut input: i32 = 0;
    let mut idx: i32 = 0;
    match *state {
        0 => {
            *headerWindowId =
                CreateTradeBoardWindow((&raw const *sWindowTemplate_TradingBoardHeader).cast_mut());
            *mainWindowId = AddWindow(winTemplate) as u8;
            DrawStdWindowFrame(*mainWindowId, FALSE);
            gMultiuseListMenuTemplate = *menuTemplate;
            gMultiuseListMenuTemplate.windowId = *mainWindowId;
            *listMenuId = ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 1);
            *state += 1;
        }
        1 => {
            CopyWindowToVram(*mainWindowId, COPYWIN_MAP);
            *state += 1;
        }
        2 => {
            input = ListMenu_ProcessInput(*listMenuId);
            if gMain.newKeys as i32 & 3 != 0 {
                if input == 8 || gMain.newKeys as i32 & B_BUTTON != 0 {
                    DestroyListMenuTask(*listMenuId, null_mut(), null_mut());
                    RemoveWindow(*mainWindowId);
                    DeleteTradeBoardWindow(*headerWindowId);
                    *state = 0;
                    return LIST_CANCEL;
                } else {
                    idx = GetIndexOfNthTradeBoardOffer((*list).players.as_mut_ptr(), input);
                    if idx >= 0 {
                        DestroyListMenuTask(*listMenuId, null_mut(), null_mut());
                        RemoveWindow(*mainWindowId);
                        DeleteTradeBoardWindow(*headerWindowId);
                        *state = 0;
                        return idx;
                    } else {
                        PlaySE(SE_WALL_HIT);
                    }
                }
            }
        }
        _ => {}
    }
    return LIST_NOTHING_CHOSEN;
}
pub(crate) unsafe extern "C" fn UR_ClearBg0() {
    FillBgTilemapBufferRect(0, 0, 0, 0, 32, 32, 0);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn JoinGroup_EnableScriptContexts() {
    ScriptContext_Enable();
}
pub(crate) unsafe extern "C" fn PrintUnionRoomText(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    colorIdx: u8,
) {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    printerTemplate.currentChar = str;
    printerTemplate.windowId = windowId;
    printerTemplate.fontId = fontId;
    printerTemplate.x = x;
    printerTemplate.y = y;
    printerTemplate.currentX = x;
    printerTemplate.currentY = y;
    printerTemplate.set_unk(0);
    gTextFlags.set_useAlternateDownArrow(FALSE);
    match colorIdx {
        UR_COLOR_DEFAULT => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_DARK_GRAY);
            printerTemplate.set_bgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_GRAY);
        }
        UR_COLOR_RED => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_RED);
            printerTemplate.set_bgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_RED);
        }
        UR_COLOR_GREEN => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_GREEN);
            printerTemplate.set_bgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_GREEN);
        }
        UR_COLOR_WHITE => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_bgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_GRAY);
        }
        UR_COLOR_CANCEL => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_WHITE);
            printerTemplate.set_bgColor(TEXT_COLOR_DARK_GRAY);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_GRAY);
        }
        UR_COLOR_TRADE_BOARD_SELF => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_COLOR_LIGHT_GREEN);
            printerTemplate.set_bgColor(TEXT_DYNAMIC_COLOR_6);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_BLUE);
        }
        UR_COLOR_TRADE_BOARD_OTHER => {
            printerTemplate.letterSpacing = 0;
            printerTemplate.lineSpacing = 0;
            printerTemplate.set_fgColor(TEXT_DYNAMIC_COLOR_5);
            printerTemplate.set_bgColor(TEXT_DYNAMIC_COLOR_6);
            printerTemplate.set_shadowColor(TEXT_COLOR_LIGHT_BLUE);
        }
        _ => {}
    }
    AddTextPrinter(&raw mut printerTemplate, TEXT_SKIP_DRAW, None);
}
pub(crate) unsafe extern "C" fn ClearRfuPlayerList(mut players: *mut RfuPlayer, count: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < count as i32 {
        (*players.at(i)).rfu = *sUnionRoomPlayer_DummyRfu;
        (*players.at(i)).timeoutCounter = 255;
        (*players.at(i)).set_groupScheduledAnim(UNION_ROOM_SPAWN_NONE);
        (*players.at(i)).set_useRedText(FALSE);
        (*players.at(i)).newPlayerCountdown = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearIncomingPlayerList(
    list: *mut RfuIncomingPlayerList,
    count: u8,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        (*list).players[i].rfu = *sUnionRoomPlayer_DummyRfu;
        (*list).players[i].set_active(FALSE);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ArePlayersDifferent(
    player1: *mut RfuPlayerData,
    player2: *mut RfuPlayerData,
) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < 2 {
        if (*player1).data.compatibility.playerTrainerId[i]
            != (*player2).data.compatibility.playerTrainerId[i]
        {
            return TRUE;
        }
        i += 1;
    }
    i = 0;
    while i < 8 {
        if (*player1).name[i] != (*player2).name[i] {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ArePlayerDataDifferent(
    player1: *mut RfuPlayerData,
    player2: *mut RfuPlayerData,
) -> u32 {
    let mut i: i32 = 0;
    if (*player1).data.activity() != (*player2).data.activity() {
        return TRUE as u32;
    }
    if (*player1).data.startedActivity() != (*player2).data.startedActivity() {
        return TRUE as u32;
    }
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        if (*player1).data.partnerInfo[i] != (*player2).data.partnerInfo[i] {
            return TRUE as u32;
        }
        i += 1;
    }
    if (*player1).data.tradeSpecies() != (*player2).data.tradeSpecies() {
        return TRUE as u32;
    }
    if (*player1).data.tradeType() != (*player2).data.tradeType() {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetNewIncomingPlayerId(
    player: *mut RfuPlayer,
    mut incomingPlayer: *mut RfuIncomingPlayer,
) -> u32 {
    let mut result: u8 = 0xFF;
    let mut i: i32 = 0;
    i = 0;
    while i < RFU_CHILD_MAX as i32 {
        if (*incomingPlayer.at(i)).active() != 0
            && ArePlayersDifferent(&raw mut (*player).rfu, &raw mut (*incomingPlayer.at(i)).rfu)
                == 0
        {
            result = i as u8;
            (*incomingPlayer.at(i)).set_active(FALSE);
        }
        i += 1;
    }
    return result as u32;
}
pub(crate) unsafe extern "C" fn TryAddIncomingPlayerToList(
    mut players: *mut RfuPlayer,
    incomingPlayer: *mut RfuIncomingPlayer,
    max: u8,
) -> u8 {
    let mut i: i32 = 0;
    if (*incomingPlayer).active() != 0 {
        i = 0;
        while i < max as i32 {
            if (*players.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_NONE {
                (*players.at(i)).rfu = (*incomingPlayer).rfu;
                (*players.at(i)).timeoutCounter = 0;
                (*players.at(i)).set_groupScheduledAnim(UNION_ROOM_SPAWN_IN);
                (*players.at(i)).newPlayerCountdown = 64;
                (*incomingPlayer).set_active(FALSE);
                return i as u8;
            }
            i += 1;
        }
    }
    return 0xFF;
}
pub(crate) unsafe extern "C" fn PrintGroupMemberOnWindow(
    windowId: u8,
    mut x: u8,
    y: u8,
    player: *mut RfuPlayer,
    colorIdx: u8,
    id: u8,
) {
    let mut activity: u8 = 0;
    let mut trainerId: CArray<u8, 6> = zeroed();
    ConvertIntToDecimalStringN(
        gStringVar4.as_mut_ptr(),
        id as i32 + 1,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringAppend(gStringVar4.as_mut_ptr(), sText_Colon.as_ptr().cast_mut());
    PrintUnionRoomText(
        windowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x,
        y,
        UR_COLOR_DEFAULT,
    );
    x += 18;
    activity = (*player).rfu.data.activity();
    if (*player).groupScheduledAnim() == UNION_ROOM_SPAWN_IN
        && activity as i32 & IN_UNION_ROOM as i32 == 0
    {
        CopyAndTranslatePlayerName(gStringVar4.as_mut_ptr(), player);
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            x,
            y,
            colorIdx,
        );
        ConvertIntToDecimalStringN(
            trainerId.as_mut_ptr(),
            (*player).rfu.data.compatibility.playerTrainerId[0] as i32
                | ((*player).rfu.data.compatibility.playerTrainerId[1] as i32) << 8,
            STR_CONV_MODE_LEADING_ZEROS,
            5,
        );
        StringCopy(gStringVar4.as_mut_ptr(), sText_ID.as_ptr().cast_mut());
        StringAppend(gStringVar4.as_mut_ptr(), trainerId.as_mut_ptr());
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x88) as u8,
            y,
            colorIdx,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintGroupCandidateOnWindow(
    windowId: u8,
    x: u8,
    y: u8,
    player: *mut RfuPlayer,
    colorIdx: u8,
    id: u8,
) {
    let mut trainerId: CArray<u8, 6> = zeroed();
    if (*player).groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
        CopyAndTranslatePlayerName(gStringVar4.as_mut_ptr(), player);
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            x,
            y,
            colorIdx,
        );
        ConvertIntToDecimalStringN(
            trainerId.as_mut_ptr(),
            (*player).rfu.data.compatibility.playerTrainerId[0] as i32
                | ((*player).rfu.data.compatibility.playerTrainerId[1] as i32) << 8,
            STR_CONV_MODE_LEADING_ZEROS,
            5,
        );
        StringCopy(gStringVar4.as_mut_ptr(), sText_ID.as_ptr().cast_mut());
        StringAppend(gStringVar4.as_mut_ptr(), trainerId.as_mut_ptr());
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x68) as u8,
            y,
            colorIdx,
        );
    }
}
pub(crate) unsafe extern "C" fn IsPlayerFacingTradingBoard() -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    if x != 9 {
        return FALSE as u32;
    }
    if y != 8 {
        return FALSE as u32;
    }
    if gPlayerAvatar.tileTransitionState == T_TILE_CENTER
        || gPlayerAvatar.tileTransitionState == T_NOT_MOVING
    {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetResponseIdx_InviteToURoomActivity(activity: i32) -> u32 {
    match activity {
        5 => {
            return 1;
        }
        4 => {
            return 2;
        }
        8 => {
            return 3;
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ConvPartnerUnameAndGetWhetherMetAlready(
    player: *mut RfuPlayer,
) -> u32 {
    let mut name: CArray<u8, 30> = zeroed();
    CopyAndTranslatePlayerName(name.as_mut_ptr(), player);
    return PlayerHasMetTrainerBefore(
        ReadAsU16(
            (*player)
                .rfu
                .data
                .compatibility
                .playerTrainerId
                .as_mut_ptr(),
        ),
        name.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn UnionRoomGetPlayerInteractionResponse(
    list: *mut RfuPlayerList,
    overrideGender: u8,
    playerIdx: u8,
    mut playerGender: u32,
) -> i32 {
    let mut metBefore: u32 = 0;
    let mut player: *mut RfuPlayer = &raw mut (*list).players[playerIdx];
    if (*player).rfu.data.startedActivity() == 0 && overrideGender == 0 {
        CopyAndTranslatePlayerName(gStringVar1.as_mut_ptr(), player);
        metBefore = PlayerHasMetTrainerBefore(
            ReadAsU16(
                (*player)
                    .rfu
                    .data
                    .compatibility
                    .playerTrainerId
                    .as_mut_ptr(),
            ),
            gStringVar1.as_mut_ptr(),
        );
        if (*player).rfu.data.activity() == 69 {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                sJoinChatTexts[metBefore][playerGender],
            );
            return 2;
        } else {
            UR_PrintFieldMessage(sCommunicatingWaitTexts[metBefore]);
            return 1;
        }
    } else {
        CopyAndTranslatePlayerName(gStringVar1.as_mut_ptr(), player);
        if overrideGender != 0 {
            playerGender = ((*player).rfu.data.compatibility.playerTrainerId
                [overrideGender as i32 + 1]
                >> 3) as u32
                & 1;
        }
        match (*player).rfu.data.activity() as i32 & 0x3F {
            1 => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    sBattleReactionTexts[playerGender][Random() % 4],
                );
            }
            4 => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    sTradeReactionTexts[playerGender][Random() as i32 % 2],
                );
            }
            5 => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    sChatReactionTexts[playerGender][Random() % 4],
                );
            }
            8 => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    sTrainerCardReactionTexts[playerGender][Random() % 2],
                );
            }
            _ => {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    sText_TrainerAppearsBusy.as_ptr().cast_mut(),
                );
            }
        }
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ItemPrintFunc_EmptyList(windowId: u8, itemId: u32, y: u8) {}
pub(crate) unsafe extern "C" fn TradeBoardPrintItemInfo(
    windowId: u8,
    y: u8,
    data: *mut RfuGameData,
    playerName: *mut u8,
    colorIdx: u8,
) {
    let mut levelStr: CArray<u8, 4> = zeroed();
    let mut species: u16 = (*data).tradeSpecies();
    let mut r#type: u8 = (*data).tradeType() as u8;
    let mut level: u8 = (*data).tradeLevel();
    PrintUnionRoomText(windowId, FONT_NORMAL, playerName, 8, y, colorIdx);
    if species == SPECIES_EGG as u16 {
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            sText_EggTrade.as_ptr().cast_mut(),
            68,
            y,
            colorIdx,
        );
    } else {
        BlitMenuInfoIcon(windowId, r#type + 1, 68, y as u16);
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            gSpeciesNames[species].as_ptr().cast_mut(),
            118,
            y,
            colorIdx,
        );
        ConvertIntToDecimalStringN(
            levelStr.as_mut_ptr(),
            level as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            3,
        );
        PrintUnionRoomText(
            windowId,
            FONT_NORMAL,
            levelStr.as_mut_ptr(),
            198,
            y,
            colorIdx,
        );
    }
}
pub(crate) unsafe extern "C" fn TradeBoardListMenuItemPrintFunc(windowId: u8, itemId: u32, y: u8) {
    let mut leader: *mut WirelessLink_Leader = sWirelessLinkMain.leader;
    let mut gameData: *mut RfuGameData = null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut playerName: CArray<u8, 9> = zeroed();
    if itemId == LIST_HEADER as u32 && y == (*sTradeBoardListMenuTemplate).upText_Y() {
        gameData = GetHostRfuGameData();
        if (*gameData).tradeSpecies() != SPECIES_NONE {
            TradeBoardPrintItemInfo(
                windowId,
                y,
                gameData,
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                UR_COLOR_TRADE_BOARD_SELF,
            );
        }
    } else {
        j = 0;
        i = 0;
        while i < MAX_UNION_ROOM_LEADERS {
            if (*(*leader).playerList).players[i].groupScheduledAnim() == UNION_ROOM_SPAWN_IN
                && (*(*leader).playerList).players[i].rfu.data.tradeSpecies() != SPECIES_NONE
            {
                j += 1;
            }
            if j as u32 == itemId + 1 {
                CopyAndTranslatePlayerName(
                    playerName.as_mut_ptr(),
                    &raw mut (*(*leader).playerList).players[i],
                );
                TradeBoardPrintItemInfo(
                    windowId,
                    y,
                    &raw mut (*(*leader).playerList).players[i].rfu.data,
                    playerName.as_mut_ptr(),
                    UR_COLOR_TRADE_BOARD_OTHER,
                );
                break;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn GetIndexOfNthTradeBoardOffer(
    players: *mut RfuPlayer,
    n: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < MAX_UNION_ROOM_LEADERS {
        if (*players.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_IN
            && (*players.at(i)).rfu.data.tradeSpecies() != SPECIES_NONE
        {
            j += 1;
        }
        if j == n + 1 {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerGender(
    playerIdx: i32,
    list: *mut RfuPlayerList,
) -> i32 {
    return (*list).players[playerIdx].rfu.data.playerGender() as i32;
}
pub(crate) unsafe extern "C" fn IsRequestedTradeInPlayerParty(
    r#type: u32,
    mut species: u32,
) -> i32 {
    let mut i: i32 = 0;
    if species == SPECIES_EGG {
        i = 0;
        while i < gPlayerPartyCount as i32 {
            species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG);
            if species == SPECIES_EGG {
                return UR_TRADE_MATCH;
            }
            i += 1;
        }
        return UR_TRADE_NOEGG;
    } else {
        i = 0;
        while i < gPlayerPartyCount as i32 {
            species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG);
            if gSpeciesInfo[species].types[0] as u32 == r#type
                || gSpeciesInfo[species].types[1] as u32 == r#type
            {
                return UR_TRADE_MATCH;
            }
            i += 1;
        }
        return UR_TRADE_NOTYPE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetURoomActivityRejectMsg(
    dst: *mut u8,
    acitivty: i32,
    playerGender: u32,
) {
    match acitivty {
        65 => {
            StringExpandPlaceholders(dst, sBattleDeclinedTexts[playerGender]);
        }
        69 => {
            StringExpandPlaceholders(dst, sChatDeclinedTexts[playerGender]);
        }
        68 => {
            StringExpandPlaceholders(dst, sText_TradeOfferRejected.as_ptr().cast_mut());
        }
        72 => {
            StringExpandPlaceholders(dst, sShowTrainerCardDeclinedTexts[playerGender]);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetURoomActivityStartMsg(dst: *mut u8, acitivty: u8) {
    let mut mpId: u8 = GetMultiplayerId();
    let mut gender: u8 = gLinkPlayers[mpId as i32 ^ 1].gender;
    match acitivty {
        65 => {
            StringCopy(dst, sStartActivityTexts[mpId][gender][0]);
        }
        68 => {
            StringCopy(dst, sStartActivityTexts[mpId][gender][2]);
        }
        69 => {
            StringCopy(dst, sStartActivityTexts[mpId][gender][1]);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetChatLeaderActionRequestMessage(
    dst: *mut u8,
    gender: u32,
    activityData: *mut u16,
    uroom: *mut WirelessLink_URoom,
) -> i32 {
    let mut result: i32 = 0;
    let mut species: u16 = SPECIES_NONE;
    let mut i: i32 = 0;
    match *activityData {
        65 => {
            StringExpandPlaceholders(dst, sText_BattleChallenge.as_ptr().cast_mut());
            result = 1;
        }
        69 => {
            StringExpandPlaceholders(dst, sText_ChatInvitation.as_ptr().cast_mut());
            result = 1;
        }
        68 => {
            ConvertIntToDecimalStringN(
                (*uroom).activityRequestStrbufs[0].as_mut_ptr(),
                sUnionRoomTrade.playerLevel as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                3,
            );
            StringCopy(
                (*uroom).activityRequestStrbufs[1].as_mut_ptr(),
                gSpeciesNames[sUnionRoomTrade.playerSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            i = 0;
            while i < RFU_CHILD_MAX as i32 {
                if (*gRfuLinkStatus).partner[i].serialNo == RFU_SERIAL_GAME {
                    ConvertIntToDecimalStringN(
                        (*uroom).activityRequestStrbufs[2].as_mut_ptr(),
                        *activityData.at(2) as i32,
                        STR_CONV_MODE_LEFT_ALIGN,
                        3,
                    );
                    StringCopy(
                        (*uroom).activityRequestStrbufs[3].as_mut_ptr(),
                        gSpeciesNames[*activityData.at(1)].as_ptr().cast_mut(),
                    );
                    species = *activityData.at(1);
                    break;
                }
                i += 1;
            }
            if species == SPECIES_EGG as u16 {
                StringCopy(dst, sText_OfferToTradeEgg.as_ptr().cast_mut());
            } else {
                i = 0;
                while i < RFU_CHILD_MAX as i32 {
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                        i as u8,
                        (*uroom).activityRequestStrbufs[i].as_mut_ptr(),
                    );
                    i += 1;
                }
                DynamicPlaceholderTextUtil_ExpandPlaceholders(
                    dst,
                    sText_OfferToTradeMon.as_ptr().cast_mut(),
                );
            }
            result = 1;
        }
        72 => {
            StringExpandPlaceholders(dst, sText_ShowTrainerCard.as_ptr().cast_mut());
            result = 1;
        }
        64 => {
            StringExpandPlaceholders(dst, sText_ChatDropped.as_ptr().cast_mut());
            result = 2;
        }
        _ => {}
    }
    return result;
}
pub(crate) unsafe extern "C" fn PollPartnerYesNoResponse(data: *mut WirelessLink_URoom) -> u32 {
    if gRecvCmds[0][1] != 0 {
        if gRecvCmds[0][1] == 81 {
            (*data).partnerYesNoResponse = 81;
            return TRUE as u32;
        } else if gRecvCmds[0][1] == 82 {
            (*data).partnerYesNoResponse = 82;
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InUnionRoom() -> u32 {
    return (if (*gSaveBlock1Ptr).location.mapGroup == 25 && (*gSaveBlock1Ptr).location.mapNum == 60
    {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u32;
}
pub(crate) unsafe extern "C" fn HasAtLeastTwoMonsOfLevel30OrLower() -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    i = 0;
    while i < gPlayerPartyCount as i32 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) <= UNION_ROOM_MAX_LEVEL as u32
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) != SPECIES_EGG
        {
            count += 1;
        }
        i += 1;
    }
    if count > 1 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ResetUnionRoomTrade(trade: *mut UnionRoomTrade) {
    (*trade).state = URTRADE_STATE_NONE;
    (*trade).r#type = 0;
    (*trade).playerPersonality = 0;
    (*trade).playerSpecies = SPECIES_NONE;
    (*trade).playerLevel = 0;
    (*trade).species = SPECIES_NONE;
    (*trade).level = 0;
    (*trade).personality = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_ResetUnionRoomTrade() {
    ResetUnionRoomTrade(&raw mut sUnionRoomTrade);
}
pub(crate) unsafe extern "C" fn RegisterTradeMonAndGetIsEgg(
    monId: u32,
    trade: *mut UnionRoomTrade,
) -> u32 {
    (*trade).playerSpecies =
        GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES_OR_EGG) as u16;
    (*trade).playerLevel = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u16;
    (*trade).playerPersonality = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
    if (*trade).playerSpecies == SPECIES_EGG as u16 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn RegisterTradeMon(monId: u32, trade: *mut UnionRoomTrade) {
    (*trade).species = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES_OR_EGG) as u16;
    (*trade).level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u16;
    (*trade).personality = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_PERSONALITY);
}
pub(crate) unsafe extern "C" fn GetPartyPositionOfRegisteredMon(
    trade: *mut UnionRoomTrade,
    multiplayerId: u8,
) -> u32 {
    let mut response: u16 = 0;
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut cur_personality: u32 = 0;
    let mut cur_species: u16 = 0;
    let mut i: i32 = 0;
    if multiplayerId == 0 {
        species = (*trade).playerSpecies;
        personality = (*trade).playerPersonality;
    } else {
        species = (*trade).species;
        personality = (*trade).personality;
    }
    i = 0;
    'l2: while i < gPlayerPartyCount as i32 {
        'l1: {
            cur_personality = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_PERSONALITY);
            if cur_personality != personality {
                break 'l1;
            }
            cur_species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
            if cur_species != species {
                break 'l1;
            }
            response = i as u16;
            break 'l2;
        }
        i += 1;
    }
    return response as u32;
}
pub(crate) unsafe extern "C" fn HandleCancelActivity(setData: u32) {
    UR_ClearBg0();
    UnlockPlayerFieldControls();
    UnionRoom_UnlockPlayerAndChatPartner();
    gPlayerCurrActivity = ACTIVITY_NONE;
    if setData != 0 {
        SetTradeBoardRegisteredMonInfo(
            sUnionRoomTrade.r#type as u32,
            sUnionRoomTrade.playerSpecies as u32,
            sUnionRoomTrade.playerLevel as u32,
        );
        UpdateGameData_SetActivity(IN_UNION_ROOM, 0, 0);
    }
}
pub(crate) unsafe extern "C" fn StartScriptInteraction() {
    LockPlayerFieldControls();
    FreezeObjects_WaitForPlayer();
}
pub(crate) unsafe extern "C" fn GetActivePartnersInfo(data: *mut WirelessLink_URoom) -> u8 {
    let mut retVal: u8 = PINFO_ACTIVE_FLAG;
    let mut i: u8 = 0;
    i = 0;
    while i < RFU_CHILD_MAX {
        if (*(*data).incomingParentList).players[i].active() != 0 {
            retVal |= (*(*data).incomingParentList).players[i]
                .rfu
                .data
                .playerGender()
                << 3;
            retVal |= (*(*data).incomingParentList).players[i]
                .rfu
                .data
                .compatibility
                .playerTrainerId[0]
                & PINFO_TID_MASK;
            break;
        }
        i += 1;
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn ViewURoomPartnerTrainerCard(
    unused: *mut u8,
    data: *mut WirelessLink_URoom,
    isParent: u8,
) {
    let mut trainerCard: *mut TrainerCard = &raw mut gTrainerCards[GetMultiplayerId() as i32 ^ 1];
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    DynamicPlaceholderTextUtil_Reset();
    StringCopy(
        (*data).trainerCardStrBuffer[0].as_mut_ptr(),
        gTrainerClassNames[GetUnionRoomTrainerClass()]
            .as_ptr()
            .cast_mut(),
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*data).trainerCardStrBuffer[0].as_mut_ptr());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(1, (*trainerCard).playerName.as_mut_ptr());
    StringCopy(
        (*data).trainerCardColorStrBuffer.as_mut_ptr(),
        sCardColorTexts[(*trainerCard).stars],
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(2, (*data).trainerCardColorStrBuffer.as_mut_ptr());
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[2].as_mut_ptr(),
        (*trainerCard).caughtMonsCount as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, (*data).trainerCardStrBuffer[2].as_mut_ptr());
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[3].as_mut_ptr(),
        (*trainerCard).playTimeHours as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[4].as_mut_ptr(),
        (*trainerCard).playTimeMinutes as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(4, (*data).trainerCardStrBuffer[3].as_mut_ptr());
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(5, (*data).trainerCardStrBuffer[4].as_mut_ptr());
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
        sText_TrainerCardInfoPage1.as_ptr().cast_mut(),
    );
    StringCopy(
        gStringVar4.as_mut_ptr(),
        (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
    );
    n = (*trainerCard).linkBattleWins as i32;
    if n > 9999 {
        n = 9999;
    }
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[0].as_mut_ptr(),
        n,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*data).trainerCardStrBuffer[0].as_mut_ptr());
    n = (*trainerCard).linkBattleLosses as i32;
    if n > 9999 {
        n = 9999;
    }
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[1].as_mut_ptr(),
        n,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(2, (*data).trainerCardStrBuffer[1].as_mut_ptr());
    ConvertIntToDecimalStringN(
        (*data).trainerCardStrBuffer[2].as_mut_ptr(),
        (*trainerCard).pokemonTrades as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, (*data).trainerCardStrBuffer[2].as_mut_ptr());
    i = 0;
    while i < TRAINER_CARD_PROFILE_LENGTH as i32 {
        CopyEasyChatWord(
            (*data).trainerCardStrBuffer[i + 3].as_mut_ptr(),
            (*trainerCard).easyChatProfile[i],
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            i as u8 + 4,
            (*data).trainerCardStrBuffer[i + 3].as_mut_ptr(),
        );
        i += 1;
    }
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
        sText_TrainerCardInfoPage2.as_ptr().cast_mut(),
    );
    StringAppend(
        gStringVar4.as_mut_ptr(),
        (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
    );
    if isParent == TRUE {
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
            sText_FinishedCheckingPlayersTrainerCard.as_ptr().cast_mut(),
        );
        StringAppend(
            gStringVar4.as_mut_ptr(),
            (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
        );
    } else if isParent == FALSE {
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
            sGladToMeetYouTexts[(*trainerCard).gender],
        );
        StringAppend(
            gStringVar4.as_mut_ptr(),
            (*data).trainerCardMsgStrBuffer.as_mut_ptr(),
        );
    }
}
pub(crate) unsafe extern "C" fn CopyAndTranslatePlayerName(dest: *mut u8, player: *mut RfuPlayer) {
    StringCopy_PlayerName(dest, (*player).rfu.name.as_mut_ptr());
    ConvertInternationalString(dest, (*player).rfu.data.compatibility.language() as u8);
}
