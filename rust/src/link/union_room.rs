//! Translated from `src/union_room.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sText_EmptyString sText_Colon sText_ID sText_PleaseStartOver sText_WirelessSearchCanceled sText_AwaitingCommunucation2 sText_AwaitingCommunication sText_AwaitingLinkPressStart sJPText_SingleBattle sJPText_DoubleBattle sJPText_MultiBattle sJPText_TradePokemon sJPText_Chat sJPText_DistWonderCard sJPText_DistWonderNews sJPText_DistMysteryEvent sJPText_HoldPokemonJump sJPText_HoldBerryCrush sJPText_HoldBerryPicking sJPText_HoldSpinTrade sJPText_HoldSpinShop sJPLinkGroupActionTexts sText_1PlayerNeeded sText_2PlayersNeeded sText_3PlayersNeeded sText_4PlayersNeeded sText_2PlayerMode sText_3PlayerMode sText_4PlayerMode sText_5PlayerMode sPlayersNeededOrModeTexts sText_BButtonCancel sJPText_SearchingForParticipants sText_PlayerContactedYouForXAccept sText_PlayerContactedYouShareX sText_PlayerContactedYouAddToMembers sText_AreTheseMembersOK sText_CancelModeWithTheseMembers sText_AnOKWasSentToPlayer sText_OtherTrainerUnavailableNow sText_CantTransmitTrainerTooFar sText_TrainersNotReadyYet sCantTransmitToTrainerTexts sText_ModeWithTheseMembersWillBeCanceled sText_MemberNoLongerAvailable sPlayerUnavailableTexts sText_TrainerAppearsUnavailable sText_PlayerSentBackOK sText_PlayerOKdRegistration sText_PlayerRepliedNo sText_AwaitingOtherMembers sText_QuitBeingMember sText_StoppedBeingMember sPlayerDisconnectedTexts sText_WirelessLinkEstablished sText_WirelessLinkDropped sText_LinkWithFriendDropped sText_PlayerRepliedNo2 sLinkDroppedTexts sText_DoYouWantXMode sText_DoYouWantXMode2 sDoYouWantModeTexts sText_CommunicatingPleaseWait sText_AwaitingPlayersResponseAboutTrade sText_Communicating sText_CommunicatingWithPlayer sText_PleaseWaitAWhile sCommunicatingWaitTexts sText_HiDoSomethingMale sText_HiDoSomethingFemale sText_HiDoSomethingAgainMale sText_HiDoSomethingAgainFemale sHiDoSomethingTexts sText_DoSomethingMale sText_DoSomethingFemale sText_DoSomethingAgainMale sText_DoSomethingAgainFemale sDoSomethingTexts sText_SomebodyHasContactedYou sText_PlayerHasContactedYou sPlayerContactedYouTexts sText_AwaitingResponseFromTrainer sText_AwaitingResponseFromPlayer sAwaitingResponseTexts sText_ShowTrainerCard sText_BattleChallenge sText_ChatInvitation sText_OfferToTradeMon sText_OfferToTradeEgg sText_ChatDropped sText_OfferDeclined1 sText_OfferDeclined2 sText_ChatEnded sInvitationTexts sText_JoinChatMale sText_PlayerJoinChatMale sText_JoinChatFemale sText_PlayerJoinChatFemale sJoinChatTexts sText_TrainerAppearsBusy sText_WaitForBattleMale sText_WaitForChatMale sText_ShowTrainerCardMale sText_WaitForBattleFemale sText_WaitForChatFemale sText_ShowTrainerCardFemale sText_WaitOrShowCardTexts sText_WaitForChatMale2 sText_DoneWaitingBattleMale sText_DoneWaitingChatMale sText_DoneWaitingBattleFemale sText_DoneWaitingChatFemale sText_TradeWillBeStarted sText_BattleWillBeStarted sText_EnteringChat sStartActivityTexts sText_BattleDeclinedMale sText_BattleDeclinedFemale sBattleDeclinedTexts sText_ShowTrainerCardDeclinedMale sText_ShowTrainerCardDeclinedFemale sShowTrainerCardDeclinedTexts sText_IfYouWantToDoSomethingMale sText_IfYouWantToDoSomethingFemale sIfYouWantToDoSomethingTexts sText_TrainerBattleBusy sText_NeedTwoMonsOfLevel30OrLower1 sText_NeedTwoMonsOfLevel30OrLower2 sText_DeclineChatMale stext_DeclineChatFemale sDeclineChatTexts sText_ChatDeclinedMale sText_ChatDeclinedFemale sChatDeclinedTexts sText_YoureToughMale sText_UsedGoodMoveMale sText_BattleSurpriseMale sText_SwitchedMonsMale sText_YoureToughFemale sText_UsedGoodMoveFemale sText_BattleSurpriseFemale sText_SwitchedMonsFemale sBattleReactionTexts sText_LearnedSomethingMale sText_ThatsFunnyMale sText_RandomChatMale1 sText_RandomChatMale2 sText_LearnedSomethingFemale sText_ThatsFunnyFemale sText_RandomChatFemale1 sText_RandomChatFemale2 sChatReactionTexts sText_ShowedTrainerCardMale1 sText_ShowedTrainerCardMale2 sText_ShowedTrainerCardFemale1 sText_ShowedTrainerCardFemale2 sTrainerCardReactionTexts sText_MaleTraded1 sText_MaleTraded2 sText_FemaleTraded1 sText_FemaleTraded2 sTradeReactionTexts sText_XCheckedTradingBoard sText_RegisterMonAtTradingBoard sText_TradingBoardInfo sText_ThankYouForRegistering sText_NobodyHasRegistered sText_ChooseRequestedMonType sText_WhichMonWillYouOffer sText_RegistrationCanceled sText_RegistrationCompleted sText_TradeCanceled sText_CancelRegistrationOfMon sText_CancelRegistrationOfEgg sText_RegistrationCanceled2 sText_TradeTrainersWillBeListed sText_ChooseTrainerToTradeWith2 sText_AskTrainerToMakeTrade sText_AwaitingResponseFromTrainer2 sText_NotRegisteredAMonForTrade sText_DontHaveTypeTrainerWants sText_DontHaveEggTrainerWants sText_PlayerCantTradeForYourMon sText_CantTradeForPartnersMon sCantTradeMonTexts sText_TradeOfferRejected sText_EggTrade sText_ChooseJoinCancel sText_ChooseTrainer sText_ChooseTrainerSingleBattle sText_ChooseTrainerDoubleBattle sText_ChooseLeaderMultiBattle sText_ChooseTrainerToTradeWith sText_ChooseTrainerToShareWonderCards sText_ChooseTrainerToShareWonderNews sText_ChooseLeaderPokemonJump sText_ChooseLeaderBerryCrush sText_ChooseLeaderBerryPicking sText_ChooseLeaderBerryBlender sText_ChooseLeaderRecordCorner sText_ChooseLeaderCoolContest sText_ChooseLeaderBeautyContest sText_ChooseLeaderCuteContest sText_ChooseLeaderSmartContest sText_ChooseLeaderToughContest sText_ChooseLeaderBattleTowerLv50 sText_ChooseLeaderBattleTowerOpenLv sChooseTrainerTexts sText_SearchingForWirelessSystemWait sText_MustHaveTwoMonsForDoubleBattle sText_AwaitingPlayersResponse sText_PlayerHasBeenAskedToRegisterYouPleaseWait sText_AwaitingResponseFromWirelessSystem sText_PleaseWaitForOtherTrainersToGather sText_NoCardsSharedRightNow sText_NoNewsSharedRightNow sNoWonderSharedTexts sText_Battle sText_Chat2 sText_Greetings sText_Exit sText_Exit2 sText_Info sText_NameWantedOfferLv sText_SingleBattle sText_DoubleBattle sText_MultiBattle sText_PokemonTrades sText_Chat sText_Cards sText_WonderCards sText_WonderNews sText_PokemonJump sText_BerryCrush sText_BerryPicking sText_Search sText_BerryBlender sText_RecordCorner sText_CoolContest sText_BeautyContest sText_CuteContest sText_SmartContest sText_ToughContest sText_BattleTowerLv50 sText_BattleTowerOpenLv sText_ItsNormalCard sText_ItsBronzeCard sText_ItsCopperCard sText_ItsSilverCard sText_ItsGoldCard sCardColorTexts sText_TrainerCardInfoPage1 sText_TrainerCardInfoPage2 sText_GladToMeetYouMale sText_GladToMeetYouFemale sGladToMeetYouTexts sText_FinishedCheckingPlayersTrainerCard sLinkGroupActivityNameTexts sWindowTemplate_BButtonCancel sLinkGroupToActivityAndCapacity sWindowTemplate_PlayerList sWindowTemplate_5PlayerList sWindowTemplate_NumPlayerMode sPossibleGroupMembersListMenuItems sListMenuTemplate_PossibleGroupMembers sWindowTemplate_GroupList sWindowTemplate_PlayerNameAndId sUnionRoomGroupsMenuItems sListMenuTemplate_UnionRoomGroups sWindowTemplate_InviteToActivity sInviteToActivityMenuItems sListMenuTemplate_InviteToActivity sWindowTemplate_RegisterForTrade sRegisterForTradeListMenuItems sListMenuTemplate_RegisterForTrade sWindowTemplate_TradingBoardRequestType sTradingBoardTypes sMenuTemplate_TradingBoardRequestType sWindowTemplate_TradingBoardHeader sWindowTemplate_TradingBoardMain sTradeBoardListMenuItems sTradeBoardListMenuTemplate sWindowTemplate_Unused sEmptyListMenuItems sEmptyListMenuTemplate sUnionRoomPlayer_DummyRfu sAcceptedActivityIds_SingleBattle sAcceptedActivityIds_DoubleBattle sAcceptedActivityIds_MultiBattle sAcceptedActivityIds_Trade sAcceptedActivityIds_PokemonJump sAcceptedActivityIds_BerryCrush sAcceptedActivityIds_BerryPicking sAcceptedActivityIds_WonderCard sAcceptedActivityIds_WonderNews sAcceptedActivityIds_Resume sAcceptedActivityIds_Init sAcceptedActivityIds_Unk11 sAcceptedActivityIds_RecordCorner sAcceptedActivityIds_BerryBlender sAcceptedActivityIds_CoolContest sAcceptedActivityIds_BeautyContest sAcceptedActivityIds_CuteContest sAcceptedActivityIds_SmartContest sAcceptedActivityIds_ToughContest sAcceptedActivityIds_BattleTower sAcceptedActivityIds_BattleTowerOpen sAcceptedActivityIds sLinkGroupToURoomActivity
#[allow(unused_imports)]
use crate::data::union_room::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionRoomPlayerName: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerCurrActivity: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerActivityGroupSize: u8 = 0u8;
pub(crate) static mut sWirelessLinkMain: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gRfuPartnerCompatibilityData: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnionRoomOfferedSpecies: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gUnionRoomRequestedMonType: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionRoomTrade: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
pub(crate) static mut sLeader: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sGroup: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sURoom: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleTypeFlags: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gDecompressionBuffer: u8;
    static mut gEnemyParty: u8;
    static mut gFieldCallback: u8;
    static mut gFieldLinkPlayerCount: u8;
    static mut gLinkPlayers: u8;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gRfuLinkStatus: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedOrderFromParty: u8;
    static mut gSelectedTradeMonPositions: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gTradeMail: u8;
    static mut gTrainerCards: u8;
    static mut gTrainerClassNames: u8;
    static mut gTypeNames: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AddTextPrinterForMessage_2(a0: u8);
    fn AddTextPrinterWithCustomSpeedForMessage(a0: u8, a1: u8);
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn CopyHostRfuGameDataAndUsername(a0: *mut u8, a1: *mut u8);
    fn CopyTrainerCardData(a0: *mut u8, a1: *mut u8, a2: u8);
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
    fn Free(a0: *mut u8);
    fn FreezeObjects_WaitForPlayer();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetHostRfuGameData() -> *mut u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetLinkPlayerInfoFlags(a0: i32) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetMysteryGiftBaseBlock() -> u16;
    fn GetOtherPlayersInfoFlags();
    fn GetPartyMenuType() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetUnionRoomTrainerClass() -> u16;
    fn GetWonderCardFlagID() -> u16;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn HandleUnionRoomPlayerRefresh(a0: *mut u8);
    fn HasTrainerLeftPartnersList(a0: u16, a1: *mut u8) -> u32;
    fn HealPlayerParty();
    fn IncrementGameStat(a0: u8);
    fn InitChooseHalfPartyForBattle(a0: u8);
    fn InitUnionRoomPlayerObjects(a0: *mut u8) -> u8;
    fn InitializeRfuLinkManager_EnterUnionRoom();
    fn InitializeRfuLinkManager_JoinGroup();
    fn InitializeRfuLinkManager_LinkLeader(a0: u32);
    fn Intl_GetListMenuWidth(a0: *mut u8) -> i32;
    fn IsLinkTaskFinished() -> u8;
    fn IsRfuCommunicatingWithAllChildren() -> u32;
    fn IsUnionRoomListenTaskActive() -> u32;
    fn LinkRfu_CreateConnectionAsParent();
    fn LinkRfu_Shutdown();
    fn LinkRfu_StopManagerAndFinalizeSlots();
    fn LinkRfu_StopManagerBeforeEnteringChat();
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
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
    fn Rfu_GetCompatiblePlayerData(a0: *mut u8, a1: *mut u8, a2: u8) -> u8;
    fn Rfu_GetWonderDistributorPlayerData(a0: *mut u8, a1: *mut u8, a2: u8) -> u8;
    fn Rfu_SendPacket(a0: *mut u8);
    fn RunTasks();
    fn RunTextPrinters();
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn SaveLinkTrainerNames();
    fn SavePlayerParty();
    fn ScheduleUnionRoomPlayerRefresh(a0: *mut u8);
    fn ScriptContext_Enable();
    fn ScriptContext_IsEnabled() -> u8;
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
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
    fn TrainerCard_GenerateCardForLinkPlayer(a0: *mut u8);
    fn TryConnectToUnionRoomParent(a0: *mut u8, a1: *mut u8, a2: u8);
    fn TryInteractWithUnionRoomMember(a0: *mut u8, a1: *mut i16, a2: *mut i16, a3: *mut u8) -> u32;
    fn UnionRoom_UnlockPlayerAndChatPartner();
    fn UnlockPlayerFieldControls();
    fn UpdateGameData_GroupLockedIn(a0: u8);
    fn UpdateGameData_SetActivity(a0: u8, a1: u32, a2: u32);
    fn UpdatePaletteFade() -> u8;
    fn UpdateUnionRoomMemberFacing(a0: u32, a1: u32, a2: *mut u8);
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
    unsafe {
        let mut windowId = windowId;
        let mut capacityCode = capacityCode;
        let mut stringId = stringId;
        FillWindowPixelBuffer(windowId, 17u8);
        'l1: {
            let __sw1 = (((capacityCode) as i32) << 8);
            if __sw1 == 512i32 {
                PrintUnionRoomText(
                    windowId,
                    1u8,
                    (((((&raw const sPlayersNeededOrModeTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringId) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 1024i32 {
                PrintUnionRoomText(
                    windowId,
                    1u8,
                    ((((((&raw const sPlayersNeededOrModeTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(20))
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringId) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 9472i32 {
                PrintUnionRoomText(
                    windowId,
                    1u8,
                    ((((((&raw const sPlayersNeededOrModeTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(40))
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringId) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 13568i32 {
                PrintUnionRoomText(
                    windowId,
                    1u8,
                    ((((((&raw const sPlayersNeededOrModeTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(60))
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringId) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 9216i32 {
                PrintUnionRoomText(
                    windowId,
                    1u8,
                    ((((((&raw const sPlayersNeededOrModeTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(80))
                    .cast::<*mut u8>())
                    .wrapping_offset((((stringId) as i32).wrapping_sub(1i32)) as isize))
                    .read(),
                    0u8,
                    1u8,
                    0u8,
                );
                break 'l1;
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNameAndIdOnWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut text = crate::ffi::Align4([0u8; 30]);
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        PrintUnionRoomText(
            windowId,
            1u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            0u8,
            1u8,
            0u8,
        );
        txtPtr = StringCopy(
            (&raw mut text).cast::<u8>(),
            ((&raw const sText_ID).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            txtPtr,
            ((ReadAsU16(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>(),
            )) as i32),
            2i32,
            5u8,
        );
        PrintUnionRoomText(windowId, 1u8, (&raw mut text).cast::<u8>(), 0u8, 17u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn GetAwaitingCommunicationText(dst: *mut u8, activity: u8) {
    unsafe {
        let mut dst = dst;
        let mut activity = activity;
        'l1: {
            let __sw1 = ((activity) as i32);
            if __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 28i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 21i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
            {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_AwaitingCommunication)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsActivityWithVariableGroupSize(activity: u32) -> u32 {
    unsafe {
        let mut activity = activity;
        'l1: {
            let __sw1 = activity;
            let __matched = __sw1 == 9u32
                || __sw1 == 10u32
                || __sw1 == 11u32
                || __sw1 == 15u32
                || __sw1 == 16u32
                || __sw1 == 23u32
                || __sw1 == 24u32
                || __sw1 == 25u32
                || __sw1 == 26u32
                || __sw1 == 27u32;
            if __sw1 == 9u32
                || __sw1 == 10u32
                || __sw1 == 11u32
                || __sw1 == 15u32
                || __sw1 == 16u32
                || __sw1 == 23u32
                || __sw1 == 24u32
                || __sw1 == 25u32
                || __sw1 == 26u32
                || __sw1 == 27u32
            {
                return 1u32;
            }
            if !__matched {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBecomeLinkLeader() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_TryBecomeLinkLeader), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((&raw mut sLeader).cast::<u8>().cast::<*mut u8>()).write(data);
        ((data).wrapping_add(12)).write(0u8);
        ((data).wrapping_add(13)).write(0u8);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_TryBecomeLinkLeader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: u32 = 0u32;
        let mut val: u32 = 0u32;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 20i32)
                    && (((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 1i32)
                {
                    let __p2 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(
                    ((((((&raw const sLinkGroupToActivityAndCapacity)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read()) as u8),
                );
                ((&raw mut sPlayerActivityGroupSize)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(
                    ((((((&raw const sLinkGroupToActivityAndCapacity)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read()
                        >> 8) as u8),
                );
                SetHostRfuGameData(
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                    0u32,
                    0u32,
                );
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_LinkLeader(
                    ((((((&raw mut sPlayerActivityGroupSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        & 15i32) as u32),
                );
                ((data).wrapping_add(12)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(160u32));
                ((data).wrapping_add(8).cast::<*mut u8>()).write(AllocZeroed(160u32));
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 5u8);
                CopyHostRfuGameDataAndUsername(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()),
                    (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(16))
                        .cast::<u8>(),
                );
                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_add(24)
                    .cast::<u16>())
                .write(0u16);
                crate::c::bf_write(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(26),
                    0,
                    2,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(26),
                    2,
                    1,
                    (0u8) as i32,
                );
                (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(27)).write(0u8);
                ((data).wrapping_add(23)).write(CreateTask_ListenForCompatiblePartners(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    255u32,
                ));
                ((data).wrapping_add(16)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_BButtonCancel)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                'l2: {
                    let __sw3 = (((((&raw mut sPlayerActivityGroupSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        & 15i32);
                    if __sw3 == 2i32 || __sw3 == 3i32 || __sw3 == 4i32 {
                        ((data).wrapping_add(15)).write(
                            ((AddWindow(
                                (&raw const sWindowTemplate_PlayerList)
                                    .cast::<u8>()
                                    .cast_mut(),
                            )) as u8),
                        );
                        break 'l2;
                    }
                    if __sw3 == 5i32 {
                        ((data).wrapping_add(15)).write(
                            ((AddWindow(
                                (&raw const sWindowTemplate_5PlayerList)
                                    .cast::<u8>()
                                    .cast_mut(),
                            )) as u8),
                        );
                        break 'l2;
                    }
                }
                ((data).wrapping_add(17)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_NumPlayerMode)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                FillWindowPixelBuffer(((data).wrapping_add(16)).read(), 34u8);
                PrintUnionRoomText(
                    ((data).wrapping_add(16)).read(),
                    0u8,
                    ((&raw const sText_BButtonCancel).cast::<u8>().cast_mut()).cast::<u8>(),
                    8u8,
                    1u8,
                    4u8,
                );
                PutWindowTilemap(((data).wrapping_add(16)).read());
                CopyWindowToVram(((data).wrapping_add(16)).read(), 2u8);
                DrawStdWindowFrame(((data).wrapping_add(15)).read(), 0u8);
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sListMenuTemplate_PossibleGroupMembers)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write(((data).wrapping_add(15)).read());
                ((data).wrapping_add(18)).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    0u16,
                ));
                DrawStdWindowFrame(((data).wrapping_add(17)).read(), 0u8);
                PutWindowTilemap(((data).wrapping_add(17)).read());
                CopyWindowToVram(((data).wrapping_add(17)).read(), 2u8);
                CopyBgTilemapBufferToVram(0u8);
                ((data).wrapping_add(19)).write(1u8);
                ((data).wrapping_add(12)).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((&raw const sLinkGroupActivityNameTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                    .read(),
                );
                if (((((&raw mut sPlayerActivityGroupSize)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32)
                    >> 4)
                    != 0i32
                {
                    if (((((data).wrapping_add(19)).read()) as i32)
                        > (((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            >> 4)
                            .wrapping_sub(1i32))
                        && ((((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            & 15i32)
                            != 0i32)
                    {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw const sText_AwaitingLinkPressStart)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    } else {
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw const sText_AwaitingCommunication)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    }
                } else {
                    GetAwaitingCommunicationText(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                    );
                }
                PrintNumPlayersWaitingForMsg(
                    ((data).wrapping_add(17)).read(),
                    ((&raw mut sPlayerActivityGroupSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read(),
                    ((data).wrapping_add(19)).read(),
                );
                ((data).wrapping_add(12)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (PrintOnTextbox((data).wrapping_add(13), (&raw mut gStringVar4).cast::<u8>()))
                    != 0
                {
                    ((data).wrapping_add(12)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                Leader_SetStateIfMemberListChanged(data, 7u32, 10u32);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    if ((((data).wrapping_add(19)).read()) as i32) == 1i32 {
                        ((data).wrapping_add(12)).write(23u8);
                    } else {
                        if (((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            & 240i32)
                            != 0i32
                        {
                            ((data).wrapping_add(12)).write(30u8);
                        } else {
                            ((data).wrapping_add(12)).write(19u8);
                        }
                    }
                }
                if (((((((((&raw mut sPlayerActivityGroupSize)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32)
                    >> 4)
                    != 0i32)
                    && (((((data).wrapping_add(19)).read()) as i32)
                        > (((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            >> 4)
                            .wrapping_sub(1i32)))
                    && ((((((&raw mut sPlayerActivityGroupSize)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        & 15i32)
                        != 0i32))
                    && ((IsRfuCommunicatingWithAllChildren()) != 0))
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 8i32)
                        != 0)
                {
                    ((data).wrapping_add(12)).write(15u8);
                    LinkRfu_StopManagerAndFinalizeSlots();
                }
                if (((((data).wrapping_add(12)).read()) as i32) == 6i32)
                    && ((RfuTryDisconnectLeavingChildren()) != 0)
                {
                    ((data).wrapping_add(12)).write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if !((RfuTryDisconnectLeavingChildren()) != 0) {
                    ((data).wrapping_add(12)).write(6u8);
                    ((data).wrapping_add(19))
                        .write(LeaderPrunePlayerList(((data).cast::<*mut u8>()).read()));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                id = ((if (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                    as i32)
                    & 15i32)
                    == 2i32
                {
                    1i32
                } else {
                    0i32
                }) as u32);
                if (PrintOnTextbox(
                    (data).wrapping_add(13),
                    ((((&raw const sPlayerUnavailableTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((id) as i32) as isize))
                    .read(),
                )) != 0
                {
                    ((data).wrapping_add(19))
                        .write(LeaderPrunePlayerList(((data).cast::<*mut u8>()).read()));
                    RedrawListMenu(((data).wrapping_add(18)).read());
                    ((data).wrapping_add(12)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 29i32 {
                id = ((if (((((&raw mut sPlayerActivityGroupSize)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32)
                    & 15i32)
                    == 2i32
                {
                    0i32
                } else {
                    1i32
                }) as u32);
                if (PrintOnTextbox(
                    (data).wrapping_add(13),
                    ((((&raw const sPlayerUnavailableTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((id) as i32) as isize))
                    .read(),
                )) != 0
                {
                    ((data).wrapping_add(12)).write(21u8);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (PrintOnTextbox((data).wrapping_add(13), (&raw mut gStringVar4).cast::<u8>()))
                    != 0
                {
                    ((data).wrapping_add(12)).write(11u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                'l3: {
                    let __sw4 = ((UnionRoomHandleYesNo(
                        (data).wrapping_add(13),
                        HasTrainerLeftPartnersList(
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                        ),
                    )) as i32);
                    if __sw4 == 0i32 {
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                        ((data).wrapping_add(25)).write(5u8);
                        SendRfuStatusToPartner(
                            ((data).wrapping_add(25)).read(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                        );
                        ((data).wrapping_add(12)).write(12u8);
                        break 'l3;
                    }
                    if __sw4 == 1i32 || __sw4 == (-1i32) {
                        ((data).wrapping_add(25)).write(6u8);
                        SendRfuStatusToPartner(
                            ((data).wrapping_add(25)).read(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                        );
                        ((data).wrapping_add(12)).write(12u8);
                        break 'l3;
                    }
                    if __sw4 == (-3i32) {
                        ((data).wrapping_add(12)).write(9u8);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                val = WaitSendRfuStatusToPartner(
                    ReadAsU16(
                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                        ))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    ),
                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(16))
                    .cast::<u8>(),
                );
                if val == 1u32 {
                    if ((((data).wrapping_add(25)).read()) as i32) == 5i32 {
                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                        ))
                        .wrapping_add(27))
                        .write(0u8);
                        RedrawListMenu(((data).wrapping_add(18)).read());
                        let __p5 = (data).wrapping_add(19);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        if ((((data).wrapping_add(19)).read()) as i32)
                            == (((((&raw mut sPlayerActivityGroupSize)
                                .cast::<u8>()
                                .cast::<u8>())
                            .read()) as i32)
                                & 15i32)
                        {
                            if ((((((&raw mut sPlayerActivityGroupSize)
                                .cast::<u8>()
                                .cast::<u8>())
                            .read()) as i32)
                                & 240i32)
                                != 0i32)
                                || (((((data).wrapping_add(19)).read()) as i32) == 4i32)
                            {
                                ((data).wrapping_add(12)).write(15u8);
                            } else {
                                CopyAndTranslatePlayerName(
                                    (&raw mut gStringVar1).cast::<u8>(),
                                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(
                                            (((((data).wrapping_add(19)).read()) as i32)
                                                .wrapping_sub(1i32))
                                                as isize
                                                * 32,
                                        ),
                                );
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    ((&raw const sText_AnOKWasSentToPlayer)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                                ((data).wrapping_add(12)).write(13u8);
                            }
                            LinkRfu_StopManagerAndFinalizeSlots();
                            PrintNumPlayersWaitingForMsg(
                                ((data).wrapping_add(17)).read(),
                                ((&raw mut sPlayerActivityGroupSize)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .read(),
                                ((data).wrapping_add(19)).read(),
                            );
                        } else {
                            ((data).wrapping_add(12)).write(4u8);
                        }
                    } else {
                        RequestDisconnectSlotByTrainerNameAndId(
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                        );
                        crate::c::bf_write(
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(26),
                            0,
                            2,
                            (0u8) as i32,
                        );
                        LeaderPrunePlayerList(((data).cast::<*mut u8>()).read());
                        RedrawListMenu(((data).wrapping_add(18)).read());
                        ((data).wrapping_add(12)).write(4u8);
                    }
                    ((data).wrapping_add(25)).write(0u8);
                } else {
                    if val == 2u32 {
                        RfuSetStatus(0u8, 0u16);
                        ((data).wrapping_add(12)).write(4u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if (PrintOnTextbox((data).wrapping_add(13), (&raw mut gStringVar4).cast::<u8>()))
                    != 0
                {
                    ((data).wrapping_add(12)).write(14u8);
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (({
                    let __p6 = (data).wrapping_add(14);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 120i32
                {
                    ((data).wrapping_add(12)).write(17u8);
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (PrintOnTextbox(
                    (data).wrapping_add(13),
                    ((&raw const sText_AreTheseMembersOK).cast::<u8>().cast_mut()).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(12)).write(16u8);
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                'l4: {
                    let __sw8 = ((UnionRoomHandleYesNo((data).wrapping_add(13), 0u32)) as i32);
                    if __sw8 == 0i32 {
                        ((data).wrapping_add(12)).write(17u8);
                        break 'l4;
                    }
                    if __sw8 == 1i32 || __sw8 == (-1i32) {
                        if (((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            & 240i32)
                            != 0i32
                        {
                            ((data).wrapping_add(12)).write(30u8);
                        } else {
                            ((data).wrapping_add(12)).write(19u8);
                        }
                        break 'l4;
                    }
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if (PrintOnTextbox(
                    (data).wrapping_add(13),
                    ((&raw const sText_CancelModeWithTheseMembers)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(12)).write(20u8);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                'l5: {
                    let __sw9 = ((UnionRoomHandleYesNo((data).wrapping_add(13), 0u32)) as i32);
                    if __sw9 == 0i32 {
                        ((data).wrapping_add(12)).write(23u8);
                        break 'l5;
                    }
                    if __sw9 == 1i32 || __sw9 == (-1i32) {
                        if (((((&raw mut sPlayerActivityGroupSize)
                            .cast::<u8>()
                            .cast::<u8>())
                        .read()) as i32)
                            & 240i32)
                            != 0i32
                        {
                            ((data).wrapping_add(12)).write(15u8);
                        } else {
                            if ((((data).wrapping_add(19)).read()) as i32)
                                == (((((&raw mut sPlayerActivityGroupSize)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .read()) as i32)
                                    & 15i32)
                            {
                                ((data).wrapping_add(12)).write(15u8);
                            } else {
                                ((data).wrapping_add(12)).write(4u8);
                            }
                        }
                        break 'l5;
                    }
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                if !((Leader_SetStateIfMemberListChanged(data, 7u32, 29u32)) != 0) {
                    ((data).wrapping_add(12)).write(18u8);
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                if (LmanAcceptSlotFlagIsNotZero()) != 0 {
                    if (WaitRfuState(0u32)) != 0 {
                        ((data).wrapping_add(12)).write(26u8);
                    } else {
                        if (({
                            let __p10 = (data).wrapping_add(26).cast::<u16>();
                            let __t11 = ((__p10).read()).wrapping_add(1);
                            (__p10).write(__t11);
                            __t11
                        }) as i32)
                            > 300i32
                        {
                            ((data).wrapping_add(12)).write(29u8);
                            ((data).wrapping_add(13)).write(0u8);
                        }
                    }
                } else {
                    ((data).wrapping_add(12)).write(29u8);
                    ((data).wrapping_add(13)).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                if (PrintOnTextbox(
                    (data).wrapping_add(13),
                    ((&raw const sText_ModeWithTheseMembersWillBeCanceled)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(12)).write(23u8);
                }
                break 'l1;
            }
            if __sw1 == 21i32 || __sw1 == 23i32 {
                DestroyWirelessStatusIndicatorSprite();
                LinkRfu_Shutdown();
                Leader_DestroyResources(data);
                let __p12 = (data).wrapping_add(12);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 24i32 {
                ScriptContext_Enable();
                DestroyTask(taskId);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                break 'l1;
            }
            if __sw1 == 22i32 {
                ScriptContext_Enable();
                DestroyTask(taskId);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(8u16);
                break 'l1;
            }
            if __sw1 == 26i32 {
                if (RfuHasErrored()) != 0 {
                    ((data).wrapping_add(12)).write(29u8);
                } else {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        if (IsActivityWithVariableGroupSize(
                            ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                                as u32),
                        )) != 0
                        {
                            GetOtherPlayersInfoFlags();
                        }
                        UpdateGameData_GroupLockedIn(1u8);
                        CreateTask_RunScriptAndFadeToActivity();
                        Leader_DestroyResources(data);
                        DestroyTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Leader_DestroyResources(data: *mut u8) {
    unsafe {
        let mut data = data;
        ClearWindowTilemap(((data).wrapping_add(17)).read());
        ClearStdWindowAndFrame(((data).wrapping_add(17)).read(), 0u8);
        DestroyListMenuTask(
            ((data).wrapping_add(18)).read(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        ClearWindowTilemap(((data).wrapping_add(16)).read());
        ClearStdWindowAndFrame(((data).wrapping_add(15)).read(), 0u8);
        CopyBgTilemapBufferToVram(0u8);
        RemoveWindow(((data).wrapping_add(17)).read());
        RemoveWindow(((data).wrapping_add(15)).read());
        RemoveWindow(((data).wrapping_add(16)).read());
        DestroyTask(((data).wrapping_add(23)).read());
        Free(((data).wrapping_add(8).cast::<*mut u8>()).read());
        Free(((data).cast::<*mut u8>()).read());
        Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn Leader_GetAcceptNewMemberPrompt(dst: *mut u8, activity: u8) {
    unsafe {
        let mut dst = dst;
        let mut activity = activity;
        'l1: {
            let __sw1 = ((activity) as i32);
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 4i32 || __sw1 == 14i32 || __sw1 == 28i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerContactedYouForXAccept)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 21i32 || __sw1 == 22i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerContactedYouShareX)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
            {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerContactedYouAddToMembers)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetYouDeclinedTheOfferMessage(dst: *mut u8, activity: u8) {
    unsafe {
        let mut dst = dst;
        let mut activity = activity;
        'l1: {
            let __sw1 = ((activity) as i32);
            if __sw1 == 65i32 || __sw1 == 68i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_OfferDeclined1).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 69i32 || __sw1 == 72i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_OfferDeclined2).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetYouAskedToJoinGroupPleaseWaitMessage(
    dst: *mut u8,
    activity: u8,
) {
    unsafe {
        let mut dst = dst;
        let mut activity = activity;
        'l1: {
            let __sw1 = ((activity) as i32);
            if __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 4i32
                || __sw1 == 28i32
                || __sw1 == 14i32
                || __sw1 == 21i32
                || __sw1 == 22i32
            {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_AwaitingPlayersResponse)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
            {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerHasBeenAskedToRegisterYouPleaseWait)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetGroupLeaderSentAnOKMessage(dst: *mut u8, activity: u8) {
    unsafe {
        let mut dst = dst;
        let mut activity = activity;
        'l1: {
            let __sw1 = ((activity) as i32);
            if __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 4i32
                || __sw1 == 28i32
                || __sw1 == 14i32
                || __sw1 == 21i32
                || __sw1 == 22i32
            {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerSentBackOK).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
            {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_PlayerOKdRegistration)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Leader_SetStateIfMemberListChanged(
    data: *mut u8,
    joinedState: u32,
    droppedState: u32,
) -> u8 {
    unsafe {
        let mut data = data;
        let mut joinedState = joinedState;
        let mut droppedState = droppedState;
        'l1: {
            let __sw1 = ((LeaderUpdateGroupMembership(((data).cast::<*mut u8>()).read())) as i32);
            if __sw1 == 1i32 {
                PlaySE(2u16);
                RedrawListMenu(((data).wrapping_add(18)).read());
                CopyAndTranslatePlayerName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((((data).wrapping_add(19)).read()) as i32) as isize * 32),
                );
                Leader_GetAcceptNewMemberPrompt(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                );
                ((data).wrapping_add(12)).write(((joinedState) as u8));
                break 'l1;
            }
            if __sw1 == 2i32 {
                RfuSetStatus(0u8, 0u16);
                RedrawListMenu(((data).wrapping_add(18)).read());
                ((data).wrapping_add(12)).write(((droppedState) as u8));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ItemPrintFunc_PossibleGroupMembers(windowId: u8, id: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut id = id;
        let mut y = y;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut colorIdx: u8 = 0u8;
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(26),
                0,
                2,
                false,
            ) as u8) as i32);
            if __sw1 == 1i32 {
                if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(27))
                .read()) as i32)
                    != 0i32
                {
                    colorIdx = 2u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                colorIdx = 1u8;
                break 'l1;
            }
        }
        PrintGroupCandidateOnWindow(
            windowId,
            0u8,
            y,
            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32),
            colorIdx,
            ((id) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn LeaderUpdateGroupMembership(list: *mut u8) -> u8 {
    unsafe {
        let mut list = list;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut ret: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut id: i32 = 0i32;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut var: u16 = ((crate::c::bf_read(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as u16);
                    if ((var) as i32) == 1i32 {
                        id = ((GetNewIncomingPlayerId(
                            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32),
                            (((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>(),
                        )) as i32);
                        if id != 255i32 {
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (((((data).wrapping_add(4).cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset((id) as isize * 28))
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                            );
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .write(1u16);
                        } else {
                            crate::c::bf_write(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                (2u8) as i32,
                            );
                            ret = 2u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            id = 0i32;
            'l3: loop {
                if !(id < 4i32) {
                    break 'l3;
                }
                'l4: {
                    TryAddIncomingPlayerToList(
                        (((data).cast::<*mut u8>()).read()).cast::<u8>(),
                        ((((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((id) as isize * 28),
                        5u8,
                    );
                }
                id = (id).wrapping_add(1);
            }
        }
        if ((ret) as i32) != 2i32 {
            {
                id = 0i32;
                'l5: loop {
                    if !(id < 5i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((id) as isize * 32))
                        .wrapping_add(27))
                        .read()) as i32)
                            != 0i32
                        {
                            ret = 1u8;
                        }
                    }
                    id = (id).wrapping_add(1);
                }
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn LeaderPrunePlayerList(list: *mut u8) -> u8 {
    unsafe {
        let mut list = list;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut copiedCount: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut playerCount: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    ((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32)
                        .cast::<crate::c::Rec4<32>>()
                        .write_unaligned(
                            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset((i) as isize * 32)
                                .cast::<crate::c::Rec4<32>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        copiedCount = 0u8;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 5i32) {
                    break 'l3;
                }
                'l4: {
                    if ((crate::c::bf_read(
                        (((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 1i32
                    {
                        ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((copiedCount) as i32) as isize * 32)
                            .cast::<crate::c::Rec4<32>>()
                            .write_unaligned(
                                ((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 32)
                                    .cast::<crate::c::Rec4<32>>()
                                    .read_unaligned(),
                            );
                        copiedCount = (copiedCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        playerCount = copiedCount;
        {
            'l5: loop {
                if !(((copiedCount) as i32) < 5i32) {
                    break 'l5;
                }
                'l6: {
                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((copiedCount) as i32) as isize * 32))
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sUnionRoomPlayer_DummyRfu)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((copiedCount) as i32) as isize * 32))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .write(0u16);
                    crate::c::bf_write(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((copiedCount) as i32) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((copiedCount) as i32) as isize * 32))
                        .wrapping_add(26),
                        2,
                        1,
                        (0u8) as i32,
                    );
                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((copiedCount) as i32) as isize * 32))
                    .wrapping_add(27))
                    .write(0u8);
                }
                copiedCount = (copiedCount).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l7: loop {
                if !(i < 5i32) {
                    break 'l7;
                }
                'l8: {
                    if ((crate::c::bf_read(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 1i32
                    {
                        break 'l8;
                    }
                    if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset((i) as isize * 32))
                    .wrapping_add(27))
                    .read()) as i32)
                        != 64i32
                    {
                        break 'l8;
                    }
                    playerCount = ((i) as u8);
                    break 'l7;
                }
                i = (i).wrapping_add(1);
            }
        }
        return playerCount;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryJoinLinkGroup() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_TryJoinLinkGroup), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((&raw mut sGroup).cast::<u8>().cast::<*mut u8>()).write(data);
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_TryJoinLinkGroup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: i32 = 0i32;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 20i32)
                    && (((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 1i32)
                {
                    let __p2 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(
                    ((((&raw const sLinkGroupToURoomActivity)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read(),
                );
                SetHostRfuGameData(
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                    0u32,
                    0u32,
                );
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_JoinGroup();
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(512u32));
                ((data).wrapping_add(8)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PrintOnTextbox(
                    (data).wrapping_add(9),
                    ((((&raw const sChooseTrainerTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
                    ))
                    .read(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 16u8);
                ((data).wrapping_add(17)).write(CreateTask_ListenForCompatiblePartners(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u32),
                ));
                ((data).wrapping_add(12)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_BButtonCancel)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                ((data).wrapping_add(11)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_GroupList)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                ((data).wrapping_add(13)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_PlayerNameAndId)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                FillWindowPixelBuffer(((data).wrapping_add(12)).read(), 34u8);
                PrintUnionRoomText(
                    ((data).wrapping_add(12)).read(),
                    0u8,
                    ((&raw const sText_ChooseJoinCancel).cast::<u8>().cast_mut()).cast::<u8>(),
                    8u8,
                    1u8,
                    4u8,
                );
                PutWindowTilemap(((data).wrapping_add(12)).read());
                CopyWindowToVram(((data).wrapping_add(12)).read(), 2u8);
                DrawStdWindowFrame(((data).wrapping_add(11)).read(), 0u8);
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sListMenuTemplate_UnionRoomGroups)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write(((data).wrapping_add(11)).read());
                ((data).wrapping_add(14)).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    0u16,
                ));
                DrawStdWindowFrame(((data).wrapping_add(13)).read(), 0u8);
                PutWindowTilemap(((data).wrapping_add(13)).read());
                PrintPlayerNameAndIdOnWindow(((data).wrapping_add(13)).read());
                CopyWindowToVram(((data).wrapping_add(13)).read(), 2u8);
                CopyBgTilemapBufferToVram(0u8);
                ((data).wrapping_add(15)).write(0u8);
                ((data).wrapping_add(8)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                id = ((GetNewLeaderCandidate()) as i32);
                'l2: {
                    let __sw3 = id;
                    let __matched = __sw3 == 1i32 || __sw3 == 0i32;
                    if __sw3 == 1i32 {
                        PlaySE(2u16);
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l2;
                    }
                    if __sw3 == 0i32 {
                        id = ListMenu_ProcessInput(((data).wrapping_add(14)).read());
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0)
                            && (id != (-1i32))
                        {
                            let mut activity: u32 = ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((id) as isize * 32))
                                .wrapping_add(10),
                                0,
                                7,
                                false,
                            ) as u8) as u32);
                            if (((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((id) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                == 1i32)
                                && (!((crate::c::bf_read(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset((id) as isize * 32))
                                    .wrapping_add(10),
                                    7,
                                    1,
                                    false,
                                ) as u8)
                                    != 0))
                            {
                                let mut readyStatus: u32 =
                                    IsTryingToTradeAcrossVersionTooSoon(data, id);
                                if readyStatus == 0u32 {
                                    AskToJoinRfuGroup(data, id);
                                    ((data).wrapping_add(8)).write(5u8);
                                    PlaySE(110u16);
                                } else {
                                    StringCopy(
                                        (&raw mut gStringVar4).cast::<u8>(),
                                        ((((&raw const sCantTransmitToTrainerTexts)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                        .wrapping_offset(
                                            (((readyStatus).wrapping_sub(1u32)) as i32) as isize,
                                        ))
                                        .read(),
                                    );
                                    ((data).wrapping_add(8)).write(18u8);
                                    PlaySE(110u16);
                                }
                            } else {
                                PlaySE(7u16);
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                ((data).wrapping_add(8)).write(10u8);
                            }
                        }
                        break 'l2;
                    }
                    if !__matched {
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                GetYouAskedToJoinGroupPleaseWaitMessage(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                );
                if (PrintOnTextbox((data).wrapping_add(9), (&raw mut gStringVar4).cast::<u8>()))
                    != 0
                {
                    CopyAndTranslatePlayerName(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(15)).read()) as i32) as isize * 32,
                        ),
                    );
                    ((data).wrapping_add(8)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(
                        (crate::c::bf_read(
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(15)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8),
                    );
                    RfuSetStatus(0u8, 0u16);
                    'l3: {
                        let __sw4 = ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                            .read()) as i32);
                        if __sw4 == 1i32
                            || __sw4 == 2i32
                            || __sw4 == 3i32
                            || __sw4 == 4i32
                            || __sw4 == 5i32
                            || __sw4 == 9i32
                            || __sw4 == 10i32
                            || __sw4 == 11i32
                            || __sw4 == 13i32
                            || __sw4 == 28i32
                            || __sw4 == 14i32
                            || __sw4 == 15i32
                            || __sw4 == 16i32
                            || __sw4 == 21i32
                            || __sw4 == 22i32
                            || __sw4 == 23i32
                            || __sw4 == 24i32
                            || __sw4 == 25i32
                            || __sw4 == 26i32
                            || __sw4 == 27i32
                        {
                            ((data).wrapping_add(8)).write(20u8);
                            return;
                        }
                    }
                }
                'l4: {
                    let __sw5 = ((RfuGetStatus()) as i32);
                    if __sw5 == 1i32 {
                        ((data).wrapping_add(8)).write(12u8);
                        break 'l4;
                    }
                    if __sw5 == 2i32 || __sw5 == 6i32 || __sw5 == 9i32 {
                        ((data).wrapping_add(8)).write(14u8);
                        break 'l4;
                    }
                    if __sw5 == 5i32 {
                        GetGroupLeaderSentAnOKMessage(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                        );
                        if (PrintOnTextbox(
                            (data).wrapping_add(9),
                            (&raw mut gStringVar4).cast::<u8>(),
                        )) != 0
                        {
                            if (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                == 28i32)
                                || (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    == 14i32)
                            {
                                RfuSetStatus(12u8, 0u16);
                            } else {
                                RfuSetStatus(7u8, 0u16);
                                StringCopy(
                                    (&raw mut gStringVar1).cast::<u8>(),
                                    ((((&raw const sLinkGroupActivityNameTexts)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(
                                        ((((&raw mut gPlayerCurrActivity)
                                            .cast::<u8>()
                                            .cast::<u8>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    ((&raw const sText_AwaitingOtherMembers)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                            }
                        }
                        break 'l4;
                    }
                    if __sw5 == 7i32 {
                        if ((((data).wrapping_add(21)).read()) as i32) > 240i32 {
                            if (PrintOnTextbox(
                                (data).wrapping_add(9),
                                (&raw mut gStringVar4).cast::<u8>(),
                            )) != 0
                            {
                                RfuSetStatus(12u8, 0u16);
                                ((data).wrapping_add(21)).write(0u8);
                            }
                        } else {
                            'l5: {
                                let __sw6 =
                                    ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                        .read()) as i32);
                                let __matched = __sw6 == 1i32
                                    || __sw6 == 2i32
                                    || __sw6 == 4i32
                                    || __sw6 == 28i32
                                    || __sw6 == 14i32;
                                if __sw6 == 1i32
                                    || __sw6 == 2i32
                                    || __sw6 == 4i32
                                    || __sw6 == 28i32
                                    || __sw6 == 14i32
                                {
                                    break 'l5;
                                }
                                if !__matched {
                                    let __p7 = (data).wrapping_add(21);
                                    (__p7).write(((__p7).read()).wrapping_add(1));
                                    break 'l5;
                                }
                            }
                        }
                        break 'l4;
                    }
                }
                if (((RfuGetStatus()) as i32) == 0i32)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0)
                {
                    ((data).wrapping_add(8)).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (PrintOnTextbox(
                    (data).wrapping_add(9),
                    ((&raw const sText_QuitBeingMember).cast::<u8>().cast_mut()).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(8u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                'l6: {
                    let __sw8 =
                        ((UnionRoomHandleYesNo((data).wrapping_add(9), ((RfuGetStatus()) as u32)))
                            as i32);
                    if __sw8 == 0i32 {
                        SendLeaveGroupNotice();
                        ((data).wrapping_add(8)).write(9u8);
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l6;
                    }
                    if __sw8 == 1i32 || __sw8 == (-1i32) {
                        ((data).wrapping_add(8)).write(5u8);
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l6;
                    }
                    if __sw8 == (-3i32) {
                        ((data).wrapping_add(8)).write(6u8);
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (RfuGetStatus()) != 0 {
                    ((data).wrapping_add(8)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32
                || __sw1 == 12i32
                || __sw1 == 14i32
                || __sw1 == 18i32
                || __sw1 == 20i32
            {
                ClearWindowTilemap(((data).wrapping_add(13)).read());
                ClearStdWindowAndFrame(((data).wrapping_add(13)).read(), 0u8);
                DestroyListMenuTask(
                    ((data).wrapping_add(14)).read(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                ClearWindowTilemap(((data).wrapping_add(12)).read());
                ClearStdWindowAndFrame(((data).wrapping_add(11)).read(), 0u8);
                CopyBgTilemapBufferToVram(0u8);
                RemoveWindow(((data).wrapping_add(13)).read());
                RemoveWindow(((data).wrapping_add(11)).read());
                RemoveWindow(((data).wrapping_add(12)).read());
                DestroyTask(((data).wrapping_add(17)).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                let __p9 = (data).wrapping_add(8);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                DestroyWirelessStatusIndicatorSprite();
                if (PrintOnTextbox(
                    (data).wrapping_add(9),
                    ((((&raw const sPlayerDisconnectedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((RfuGetStatus()) as i32) as isize))
                    .read(),
                )) != 0
                {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
                    ((data).wrapping_add(8)).write(23u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                DestroyWirelessStatusIndicatorSprite();
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                ((data).wrapping_add(8)).write(23u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                DestroyWirelessStatusIndicatorSprite();
                if (PrintOnTextbox(
                    (data).wrapping_add(9),
                    ((((&raw const sPlayerDisconnectedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((RfuGetStatus()) as i32) as isize))
                    .read(),
                )) != 0
                {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(8u16);
                    ((data).wrapping_add(8)).write(23u8);
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if (PrintOnTextbox((data).wrapping_add(9), (&raw mut gStringVar4).cast::<u8>()))
                    != 0
                {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(8u16);
                    ((data).wrapping_add(8)).write(23u8);
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                DestroyTask(taskId);
                JoinGroup_EnableScriptContexts();
                LinkRfu_Shutdown();
                break 'l1;
            }
            if __sw1 == 21i32 {
                CreateTask_RunScriptAndFadeToActivity();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsTryingToTradeAcrossVersionTooSoon(data: *mut u8, id: i32) -> u32 {
    unsafe {
        let mut data = data;
        let mut id = id;
        let mut partner: *mut u8 =
            ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset((id) as isize * 32);
        if (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32) == 4i32)
            && (((crate::c::bf_read((partner).wrapping_add(1), 2, 4, false) as u16) as i32) != 3i32)
        {
            if !((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                as i32)
                & 128i32)
                != 0)
            {
                return 1u32;
            } else {
                if (crate::c::bf_read((partner).wrapping_add(0), 7, 1, false) as u16) != 0 {
                    return 0u32;
                }
            }
        } else {
            return 0u32;
        }
        return 2u32;
    }
}
pub(crate) unsafe extern "C" fn AskToJoinRfuGroup(data: *mut u8, id: i32) {
    unsafe {
        let mut data = data;
        let mut id = id;
        ((data).wrapping_add(15)).write(((id) as u8));
        LoadWirelessStatusIndicatorSpriteGfx();
        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
        RedrawListMenu(((data).wrapping_add(14)).read());
        CopyAndTranslatePlayerName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(15)).read()) as i32) as isize * 32),
        );
        UpdateGameData_SetActivity(
            ((((&raw const sLinkGroupToURoomActivity)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
            ))
            .read(),
            0u32,
            1u32,
        );
        CreateTask_RfuReconnectWithParent(
            ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((((data).wrapping_add(15)).read()) as i32) as isize * 32))
            .wrapping_add(16))
            .cast::<u8>(),
            ReadAsU16(
                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_add(15)).read()) as i32) as isize * 32))
                .wrapping_add(2))
                .cast::<u8>(),
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_ListenToWireless() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_ListenToWireless), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((&raw mut sGroup).cast::<u8>().cast::<*mut u8>()).write(data);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_ListenToWireless(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                SetHostRfuGameData(0u8, 0u32, 0u32);
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_JoinGroup();
                RfuSetIgnoreError(1u32);
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(512u32));
                ((data).wrapping_add(8)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 16u8);
                ((data).wrapping_add(17)).write(CreateTask_ListenForCompatiblePartners(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    255u32,
                ));
                ((data).wrapping_add(15)).write(0u8);
                ((data).wrapping_add(8)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetNewLeaderCandidate()) as i32) == 1i32 {
                    PlaySE(2u16);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32)
                    == 255i32
                {
                    ((data).wrapping_add(8)).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                DestroyTask(((data).wrapping_add(17)).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                LinkRfu_Shutdown();
                let __p2 = (data).wrapping_add(8);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                LinkRfu_Shutdown();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsPartnerActivityAcceptable(activity: u32, linkGroup: u32) -> u32 {
    unsafe {
        let mut activity = activity;
        let mut linkGroup = linkGroup;
        if linkGroup == 255u32 {
            return 1u32;
        }
        if linkGroup < crate::c::div_u32(88u32, 4u32) {
            let mut bytes: *mut u8 = ((((&raw const sAcceptedActivityIds)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((linkGroup) as i32) as isize))
            .read();
            'l1: loop {
                if !((((bytes).read()) as i32) != 255i32) {
                    break 'l1;
                }
                if (((bytes).read()) as u32) == activity {
                    return 1u32;
                }
                bytes = (bytes).wrapping_offset(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetGroupListTextColor(data: *mut u8, id: u32) -> u8 {
    unsafe {
        let mut data = data;
        let mut id = id;
        if ((crate::c::bf_read(
            (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
            .wrapping_add(26),
            0,
            2,
            false,
        ) as u8) as i32)
            == 1i32
        {
            if (crate::c::bf_read(
                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(10),
                7,
                1,
                false,
            ) as u8)
                != 0
            {
                return 3u8;
            } else {
                if (crate::c::bf_read(
                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 32))
                    .wrapping_add(26),
                    2,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    return 1u8;
                } else {
                    if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 32))
                    .wrapping_add(27))
                    .read()) as i32)
                        != 0i32
                    {
                        return 2u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ListMenuItemPrintFunc_UnionRoomGroups(
    windowId: u8,
    id: u32,
    y: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut id = id;
        let mut y = y;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut colorId: u8 = GetGroupListTextColor(data, id);
        PrintGroupMemberOnWindow(
            windowId,
            8u8,
            y,
            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32),
            colorId,
            ((id) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn GetNewLeaderCandidate() -> u8 {
    unsafe {
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut ret: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut id: i32 = 0i32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32
                    {
                        id = ((GetNewIncomingPlayerId(
                            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32),
                            (((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>(),
                        )) as i32);
                        if id != 255i32 {
                            if ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                == 1i32
                            {
                                if (ArePlayerDataDifferent(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32)),
                                    (((((data).wrapping_add(4).cast::<*mut u8>()).read())
                                        .cast::<u8>())
                                    .wrapping_offset((id) as isize * 28)),
                                )) != 0
                                {
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .cast::<crate::c::Rec4<24>>()
                                    .write_unaligned(
                                        (((((data).wrapping_add(4).cast::<*mut u8>()).read())
                                            .cast::<u8>())
                                        .wrapping_offset((id) as isize * 28))
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                    );
                                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(27))
                                    .write(64u8);
                                    ret = 1u8;
                                } else {
                                    if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(27))
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        let __p1 = (((((data).cast::<*mut u8>()).read())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(27);
                                        (__p1).write(((__p1).read()).wrapping_sub(1));
                                        if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(27))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            ret = 2u8;
                                        }
                                    }
                                }
                            } else {
                                crate::c::bf_write(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(26),
                                    0,
                                    2,
                                    (1u8) as i32,
                                );
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(27))
                                .write(64u8);
                                ret = 1u8;
                            }
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .write(0u16);
                        } else {
                            if ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                != 2i32
                            {
                                let __p2 = (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(24)
                                .cast::<u16>();
                                (__p2).write(((__p2).read()).wrapping_add(1));
                                if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(24)
                                .cast::<u16>())
                                .read()) as i32)
                                    >= 300i32
                                {
                                    crate::c::bf_write(
                                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(26),
                                        0,
                                        2,
                                        (2u8) as i32,
                                    );
                                    ret = 2u8;
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            id = 0i32;
            'l3: loop {
                if !(id < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if ((TryAddIncomingPlayerToList(
                        (((data).cast::<*mut u8>()).read()).cast::<u8>(),
                        ((((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((id) as isize * 28),
                        16u8,
                    )) as i32)
                        != 255i32
                    {
                        ret = 1u8;
                    }
                }
                id = (id).wrapping_add(1);
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn Task_CreateTradeMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CB2_StartCreateTradeMenu();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_CreateTradeMenu() -> u8 {
    unsafe {
        return CreateTask(Some(Task_CreateTradeMenu), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_StartUnionRoomTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut monId: u32 = GetPartyPositionOfRegisteredMon(
            (&raw mut sUnionRoomTrade).cast::<u8>(),
            GetMultiplayerId(),
        );
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                SendBlock(
                    0u8,
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    100u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((GetBlockReceivedStatus()) as i32) == 3i32 {
                    (&raw mut gEnemyParty)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                                (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 256,
                            ))
                            .cast::<u16>())
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<100>>()
                            .read_unaligned(),
                        );
                    IncrementGameStat(50u8);
                    ResetBlockReceivedFlags();
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                        .cast::<u8>(),
                    220u32,
                );
                if (SendBlock(0u8, (&raw mut gBlockSendBuffer).cast::<u8>(), 220u16)) != 0 {
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetBlockReceivedStatus()) as i32) == 3i32 {
                    crate::c::memcpy(
                        (&raw mut gTradeMail).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        216u32,
                    );
                    ResetBlockReceivedFlags();
                    ((&raw mut gSelectedTradeMonPositions).cast::<u8>()).write(((monId) as u8));
                    (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).wrapping_offset(1))
                        .write(6u8);
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_ReturnToField));
                    SetMainCallback2(Some(CB2_LinkTrade));
                    ResetUnionRoomTrade((&raw mut sUnionRoomTrade).cast::<u8>());
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExchangeCards(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    SendBlockRequest(2u8);
                }
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((GetBlockReceivedStatus()) as i32) == ((GetLinkPlayerCountAsBitFlags()) as i32)
                {
                    let mut i: i32 = 0i32;
                    let mut recvBuff: *mut u16 = core::ptr::null_mut();
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < ((GetLinkPlayerCount()) as i32)) {
                                break 'l2;
                            }
                            'l3: {
                                recvBuff = (((&raw mut gBlockRecvBuffer).cast::<u8>())
                                    .wrapping_offset((i) as isize * 256))
                                .cast::<u16>();
                                CopyTrainerCardData(
                                    ((&raw mut gTrainerCards).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    (recvBuff).cast::<u8>(),
                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .cast::<u16>())
                                    .read()) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((GetLinkPlayerCount()) as i32) == 2i32 {
                        recvBuff = (((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 256,
                        ))
                        .cast::<u16>();
                        MysteryGift_TryEnableStatsByFlagId(
                            (((recvBuff).cast::<u8>()).wrapping_add(96).cast::<u16>()).read(),
                        );
                    } else {
                        MysteryGift_DisableStats();
                    }
                    ResetBlockReceivedFlags();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowCard() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                CreateTask(Some(Task_ExchangeCards), 5u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FuncIsActiveTask(Some(Task_ExchangeCards))) != 0) {
                    ShowTrainerCardInLink(
                        ((((GetMultiplayerId()) as i32) ^ 1i32) as u8),
                        Some(CB2_ReturnToField),
                    );
                }
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartUnionRoomBattle(battleFlags: u16) {
    unsafe {
        let mut battleFlags = battleFlags;
        HealPlayerParty();
        SavePlayerParty();
        LoadPlayerBag();
        (((&raw mut gLinkPlayers).cast::<u8>())
            .wrapping_add(20)
            .cast::<u32>())
        .write(8721u32);
        ((((&raw mut gLinkPlayers).cast::<u8>())
            .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
        .wrapping_add(24)
        .cast::<u16>())
        .write(((GetMultiplayerId()) as u16));
        ((((&raw mut gLinkPlayers).cast::<u8>())
            .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28))
        .wrapping_add(24)
        .cast::<u16>())
        .write(((((GetMultiplayerId()) as i32) ^ 1i32) as u16));
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ReturnFromCableClubBattle));
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(((battleFlags) as u32));
        PlayBattleBGM();
    }
}
pub(crate) unsafe extern "C" fn WarpForWirelessMinigame(linkService: u16, x: u16, y: u16) {
    unsafe {
        let mut linkService = linkService;
        let mut x = x;
        let mut y = y;
        VarSet(16519u16, linkService);
        SetWarpDestination(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
            ((x) as i8),
            ((y) as i8),
        );
        SetDynamicWarpWithCoords(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
            ((x) as i8),
            ((y) as i8),
        );
        WarpIntoMap();
    }
}
pub(crate) unsafe extern "C" fn WarpForCableClubActivity(
    mapGroup: i8,
    mapNum: i8,
    x: i32,
    y: i32,
    linkService: u16,
) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut x = x;
        let mut y = y;
        let mut linkService = linkService;
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(linkService);
        VarSet(16519u16, linkService);
        ((&raw mut gFieldLinkPlayerCount).cast::<u8>()).write(GetLinkPlayerCount());
        ((&raw mut gLocalLinkPlayerId).cast::<u8>()).write(GetMultiplayerId());
        SetCableClubWarp();
        SetWarpDestination(mapGroup, mapNum, (-1i8), ((x) as i8), ((y) as i8));
        WarpIntoMap();
    }
}
pub(crate) unsafe extern "C" fn CB2_TransitionToCableClub() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                CreateTask(Some(Task_ExchangeCards), 5u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FuncIsActiveTask(Some(Task_ExchangeCards))) != 0) {
                    SetMainCallback2(Some(CB2_ReturnToFieldCableClub));
                }
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
    }
}
pub(crate) unsafe extern "C" fn CreateTrainerCardInBuffer(dest: *mut u8, setWonderCard: u32) {
    unsafe {
        let mut dest = dest;
        let mut setWonderCard = setWonderCard;
        let mut card: *mut u8 = dest;
        TrainerCard_GenerateCardForLinkPlayer(card);
        if (setWonderCard) != 0 {
            ((card).wrapping_add(96).cast::<u16>()).write(GetWonderCardFlagID());
        } else {
            ((card).wrapping_add(96).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartActivity(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        MysteryGift_DisableStats();
        'l1: {
            let __sw1 =
                ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 13i32
                || __sw1 == 15i32
            {
                SaveLinkTrainerNames();
                break 'l1;
            }
        }
        'l2: {
            let __sw2 =
                ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw2 == 65i32 || __sw2 == 81i32 {
                CleanupOverworldWindowsAndTilemaps();
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_UnionRoomBattle));
                InitChooseHalfPartyForBattle(3u8);
                break 'l2;
            }
            if __sw2 == 1i32 {
                CleanupOverworldWindowsAndTilemaps();
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                HealPlayerParty();
                SavePlayerParty();
                LoadPlayerBag();
                WarpForCableClubActivity(25i8, 24i8, 6i32, 8i32, 1u16);
                SetMainCallback2(Some(CB2_TransitionToCableClub));
                break 'l2;
            }
            if __sw2 == 2i32 {
                CleanupOverworldWindowsAndTilemaps();
                HealPlayerParty();
                SavePlayerParty();
                LoadPlayerBag();
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                WarpForCableClubActivity(25i8, 24i8, 6i32, 8i32, 2u16);
                SetMainCallback2(Some(CB2_TransitionToCableClub));
                break 'l2;
            }
            if __sw2 == 3i32 {
                CleanupOverworldWindowsAndTilemaps();
                HealPlayerParty();
                SavePlayerParty();
                LoadPlayerBag();
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                WarpForCableClubActivity(25i8, 27i8, 5i32, 8i32, 5u16);
                SetMainCallback2(Some(CB2_TransitionToCableClub));
                break 'l2;
            }
            if __sw2 == 4i32 {
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                CleanupOverworldWindowsAndTilemaps();
                WarpForCableClubActivity(25i8, 25i8, 5i32, 8i32, 3u16);
                SetMainCallback2(Some(CB2_TransitionToCableClub));
                break 'l2;
            }
            if __sw2 == 15i32 {
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                CleanupOverworldWindowsAndTilemaps();
                WarpForCableClubActivity(25i8, 26i8, 8i32, 9i32, 4u16);
                SetMainCallback2(Some(CB2_TransitionToCableClub));
                break 'l2;
            }
            if __sw2 == 68i32 {
                CleanupOverworldWindowsAndTilemaps();
                CreateTask(Some(Task_StartUnionRoomTrade), 0u8);
                break 'l2;
            }
            if __sw2 == 5i32 || __sw2 == 69i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    LinkRfu_CreateConnectionAsParent();
                } else {
                    LinkRfu_StopManagerBeforeEnteringChat();
                    SetHostRfuGameData(69u8, 0u32, 1u32);
                }
                EnterUnionRoomChat();
                break 'l2;
            }
            if __sw2 == 8i32 || __sw2 == 72i32 {
                CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 0u32);
                SetMainCallback2(Some(CB2_ShowCard));
                break 'l2;
            }
            if __sw2 == 9i32 {
                WarpForWirelessMinigame(8u16, 5u16, 1u16);
                StartPokemonJump(((GetCursorSelectionMonId()) as u16), Some(CB2_LoadMap));
                break 'l2;
            }
            if __sw2 == 10i32 {
                WarpForWirelessMinigame(7u16, 9u16, 1u16);
                StartBerryCrush(Some(CB2_LoadMap));
                break 'l2;
            }
            if __sw2 == 11i32 {
                WarpForWirelessMinigame(8u16, 5u16, 1u16);
                StartDodrioBerryPicking(((GetCursorSelectionMonId()) as u16), Some(CB2_LoadMap));
                break 'l2;
            }
        }
        DestroyTask(taskId);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32) != 68i32 {
            UnlockPlayerFieldControls();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RunScriptAndFadeToActivity(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut sendBuff: *mut u16 = ((&raw mut gBlockSendBuffer).cast::<u8>()).cast::<u16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                'l2: {
                    let __sw2 = ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                        as i32);
                    let __matched = __sw2 == 28i32
                        || __sw2 == 14i32
                        || __sw2 == 16i32
                        || __sw2 == 23i32
                        || __sw2 == 24i32
                        || __sw2 == 25i32
                        || __sw2 == 26i32
                        || __sw2 == 27i32;
                    let mut __fall = false;
                    if __sw2 == 28i32 || __sw2 == 14i32 {
                        __fall = true;
                        (((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(20)
                            .cast::<u32>())
                        .write(8721u32);
                        (((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(0u16);
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(2u16);
                        (sendBuff).write(
                            ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gSelectedOrderFromParty).cast::<u8>()).read())
                                        as i32)
                                        .wrapping_sub(1i32))
                                        as isize
                                        * 100,
                                ),
                                11i32,
                            )) as u16),
                        );
                        ((sendBuff).wrapping_offset(1)).write(
                            ((GetMonData3(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                        .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_sub(1i32))
                                        as isize
                                        * 100,
                                ),
                                11i32,
                                core::ptr::null_mut(),
                            )) as u16),
                        );
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .write(None);
                        (data).write(4i16);
                        SaveLinkTrainerNames();
                        ResetBlockReceivedFlags();
                        break 'l2;
                    }
                    if __sw2 == 16i32
                        || __sw2 == 23i32
                        || __sw2 == 24i32
                        || __sw2 == 25i32
                        || __sw2 == 26i32
                        || __sw2 == 27i32
                    {
                        __fall = true;
                        SaveLinkTrainerNames();
                        DestroyTask(taskId);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        ScriptContext_Enable();
                        (data).write(1i16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((ScriptContext_IsEnabled()) != 0) {
                    FadeScreen(1u8, 0i8);
                    (data).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 29i32
                    {
                        DestroyTask(taskId);
                        SetMainCallback2(Some(CB2_StartCreateTradeMenu));
                    } else {
                        SetLinkStandbyCallback();
                        (data).write(3i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    DestroyTask(taskId);
                    CreateTask_StartActivity();
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (SendBlock(0u8, (&raw mut gBlockSendBuffer).cast::<u8>(), 14u16)) != 0 {
                    (data).write(5i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((GetBlockReceivedStatus()) as i32) == 3i32 {
                    ResetBlockReceivedFlags();
                    if (AreBattleTowerLinkSpeciesSame(
                        ((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>(),
                        (((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256))
                            .cast::<u16>(),
                    )) != 0
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(11u16);
                        (data).write(7i16);
                    } else {
                        (data).write(6i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                ScriptContext_Enable();
                DestroyTask(taskId);
                break 'l1;
            }
            if __sw1 == 7i32 {
                SetCloseLinkCallback();
                (data).write(8i16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 0i32 {
                    DestroyWirelessStatusIndicatorSprite();
                    ScriptContext_Enable();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_RunScriptAndFadeToActivity() {
    unsafe {
        CreateTask(Some(Task_RunScriptAndFadeToActivity), 0u8);
    }
}
pub(crate) unsafe extern "C" fn CreateTask_StartActivity() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_StartActivity), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_SendMysteryGift(activity: u32) {
    unsafe {
        let mut activity = activity;
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_SendMysteryGift), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((data).wrapping_add(12)).write(0u8);
        ((data).wrapping_add(13)).write(0u8);
        ((data).wrapping_add(24)).write(((activity) as u8));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_SendMysteryGift(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        let mut val: i32 = 0i32;
        'l1: {
            let __sw1 = ((((data).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                    .write(((data).wrapping_add(24)).read());
                ((&raw mut sPlayerActivityGroupSize)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(2u8);
                SetHostRfuGameData(((data).wrapping_add(24)).read(), 0u32, 0u32);
                SetHostRfuWonderFlags(0u32, 0u32);
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_LinkLeader(2u32);
                ((data).wrapping_add(12)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(160u32));
                ((data).wrapping_add(8).cast::<*mut u8>()).write(AllocZeroed(160u32));
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 5u8);
                CopyHostRfuGameDataAndUsername(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()),
                    (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(16))
                        .cast::<u8>(),
                );
                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_add(24)
                    .cast::<u16>())
                .write(0u16);
                crate::c::bf_write(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(26),
                    0,
                    2,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(26),
                    2,
                    1,
                    (0u8) as i32,
                );
                (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(27)).write(0u8);
                ((data).wrapping_add(23)).write(CreateTask_ListenForCompatiblePartners(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    255u32,
                ));
                (&raw mut winTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_PlayerList)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                (((&raw mut winTemplate).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(GetMysteryGiftBaseBlock());
                (((&raw mut winTemplate).cast::<u8>()).wrapping_add(5)).write(12u8);
                ((data).wrapping_add(15))
                    .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
                MG_DrawTextBorder(((data).wrapping_add(15)).read());
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sListMenuTemplate_PossibleGroupMembers)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write(((data).wrapping_add(15)).read());
                ((data).wrapping_add(18)).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    0u16,
                ));
                CopyBgTilemapBufferToVram(0u8);
                ((data).wrapping_add(19)).write(1u8);
                ((data).wrapping_add(12)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((&raw const sLinkGroupActivityNameTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(
                        ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32)
                            as isize,
                    ))
                    .read(),
                );
                GetAwaitingCommunicationText(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                );
                ((data).wrapping_add(12)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                MG_AddMessageTextPrinter((&raw mut gStringVar4).cast::<u8>());
                ((data).wrapping_add(12)).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                Leader_SetStateIfMemberListChanged(data, 5u32, 6u32);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    ((data).wrapping_add(12)).write(13u8);
                    DestroyWirelessStatusIndicatorSprite();
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(13),
                    ((&raw const sText_LinkWithFriendDropped)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(19))
                        .write(LeaderPrunePlayerList(((data).cast::<*mut u8>()).read()));
                    RedrawListMenu(((data).wrapping_add(18)).read());
                    ((data).wrapping_add(12)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((data).wrapping_add(12)).write(7u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                'l2: {
                    let __sw2 = ((DoMysteryGiftYesNo(
                        (data).wrapping_add(13),
                        (data).wrapping_add(20).cast::<u16>(),
                        0u8,
                        (&raw mut gStringVar4).cast::<u8>(),
                    )) as i32);
                    if __sw2 == 0i32 {
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                        ))
                        .wrapping_add(27))
                        .write(0u8);
                        RedrawListMenu(((data).wrapping_add(18)).read());
                        ((data).wrapping_add(25)).write(5u8);
                        SendRfuStatusToPartner(
                            ((data).wrapping_add(25)).read(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                        );
                        ((data).wrapping_add(12)).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 1i32 || __sw2 == (-1i32) {
                        ((data).wrapping_add(25)).write(6u8);
                        SendRfuStatusToPartner(
                            ((data).wrapping_add(25)).read(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                        );
                        ((data).wrapping_add(12)).write(8u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                val = ((WaitSendRfuStatusToPartner(
                    ReadAsU16(
                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                        ))
                        .wrapping_add(2))
                        .cast::<u8>(),
                    ),
                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(16))
                    .cast::<u8>(),
                )) as i32);
                if val == 1i32 {
                    if ((((data).wrapping_add(25)).read()) as i32) == 5i32 {
                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                        ))
                        .wrapping_add(27))
                        .write(0u8);
                        RedrawListMenu(((data).wrapping_add(18)).read());
                        let __p3 = (data).wrapping_add(19);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        CopyAndTranslatePlayerName(
                            (&raw mut gStringVar1).cast::<u8>(),
                            ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                (((((data).wrapping_add(19)).read()) as i32).wrapping_sub(1i32))
                                    as isize
                                    * 32,
                            ),
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw const sText_AnOKWasSentToPlayer)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                        ((data).wrapping_add(12)).write(9u8);
                        LinkRfu_StopManagerAndFinalizeSlots();
                    } else {
                        RequestDisconnectSlotByTrainerNameAndId(
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                            ReadAsU16(
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(
                                        ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                                    ))
                                .wrapping_add(2))
                                .cast::<u8>(),
                            ),
                        );
                        crate::c::bf_write(
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(19)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(26),
                            0,
                            2,
                            (0u8) as i32,
                        );
                        LeaderPrunePlayerList(((data).cast::<*mut u8>()).read());
                        RedrawListMenu(((data).wrapping_add(18)).read());
                        ((data).wrapping_add(12)).write(2u8);
                    }
                    ((data).wrapping_add(25)).write(0u8);
                } else {
                    if val == 2i32 {
                        RfuSetStatus(0u8, 0u16);
                        ((data).wrapping_add(12)).write(2u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                MG_AddMessageTextPrinter((&raw mut gStringVar4).cast::<u8>());
                ((data).wrapping_add(12)).write(10u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (({
                    let __p4 = (data).wrapping_add(14);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 120i32
                {
                    ((data).wrapping_add(12)).write(11u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if !((Leader_SetStateIfMemberListChanged(data, 5u32, 6u32)) != 0) {
                    ((data).wrapping_add(12)).write(12u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (LmanAcceptSlotFlagIsNotZero()) != 0 {
                    WaitRfuState(0u32);
                    ((data).wrapping_add(12)).write(15u8);
                } else {
                    ((data).wrapping_add(12)).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                DestroyWirelessStatusIndicatorSprite();
                LinkRfu_Shutdown();
                DestroyListMenuTask(
                    ((data).wrapping_add(18)).read(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                CopyBgTilemapBufferToVram(0u8);
                RemoveWindow(((data).wrapping_add(15)).read());
                DestroyTask(((data).wrapping_add(23)).read());
                Free(((data).wrapping_add(8).cast::<*mut u8>()).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                let __p6 = (data).wrapping_add(12);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(13),
                    ((&raw const sText_PleaseStartOver).cast::<u8>().cast_mut()).cast::<u8>(),
                )) != 0
                {
                    DestroyTask(taskId);
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (((RfuGetStatus()) as i32) == 1i32) || (((RfuGetStatus()) as i32) == 2i32) {
                    ((data).wrapping_add(12)).write(13u8);
                } else {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        UpdateGameData_GroupLockedIn(1u8);
                        let __p7 = (data).wrapping_add(12);
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                DestroyListMenuTask(
                    ((data).wrapping_add(18)).read(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                CopyBgTilemapBufferToVram(0u8);
                RemoveWindow(((data).wrapping_add(15)).read());
                DestroyTask(((data).wrapping_add(23)).read());
                Free(((data).wrapping_add(8).cast::<*mut u8>()).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                SetLinkStandbyCallback();
                let __p8 = (data).wrapping_add(12);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                if (IsLinkTaskFinished()) != 0 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_LinkMysteryGiftWithFriend(activity: u32) {
    unsafe {
        let mut activity = activity;
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_CardOrNewsWithFriend), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((&raw mut sGroup).cast::<u8>().cast::<*mut u8>()).write(data);
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((data).wrapping_add(18)).write((((activity).wrapping_sub(21u32)) as u8));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_CardOrNewsWithFriend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: i32 = 0i32;
        let mut listWinTemplate = crate::ffi::Align4([0u8; 8]);
        let mut playerNameWinTemplate = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                SetHostRfuGameData(
                    ((((((data).wrapping_add(18)).read()) as i32).wrapping_add(21i32)) as u8),
                    0u32,
                    0u32,
                );
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_JoinGroup();
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(512u32));
                ((data).wrapping_add(8)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                MG_AddMessageTextPrinter(
                    ((&raw const sText_ChooseTrainer).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                ((data).wrapping_add(8)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 16u8);
                ((data).wrapping_add(17)).write(CreateTask_ListenForCompatiblePartners(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    ((((((data).wrapping_add(18)).read()) as i32).wrapping_add(7i32)) as u32),
                ));
                (&raw mut listWinTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_GroupList)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                (((&raw mut listWinTemplate).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(GetMysteryGiftBaseBlock());
                (((&raw mut listWinTemplate).cast::<u8>()).wrapping_add(5)).write(12u8);
                ((data).wrapping_add(11))
                    .write(((AddWindow((&raw mut listWinTemplate).cast::<u8>())) as u8));
                (&raw mut playerNameWinTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sWindowTemplate_PlayerNameAndId)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                (((&raw mut playerNameWinTemplate).cast::<u8>()).wrapping_add(5)).write(12u8);
                ((data).wrapping_add(13))
                    .write(((AddWindow((&raw mut playerNameWinTemplate).cast::<u8>())) as u8));
                MG_DrawTextBorder(((data).wrapping_add(11)).read());
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sListMenuTemplate_UnionRoomGroups)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write(((data).wrapping_add(11)).read());
                ((data).wrapping_add(14)).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    0u16,
                ));
                MG_DrawTextBorder(((data).wrapping_add(13)).read());
                FillWindowPixelBuffer(((data).wrapping_add(13)).read(), 17u8);
                PutWindowTilemap(((data).wrapping_add(13)).read());
                PrintPlayerNameAndIdOnWindow(((data).wrapping_add(13)).read());
                CopyWindowToVram(((data).wrapping_add(13)).read(), 2u8);
                CopyBgTilemapBufferToVram(0u8);
                ((data).wrapping_add(15)).write(0u8);
                ((data).wrapping_add(8)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                id = ((GetNewLeaderCandidate()) as i32);
                'l2: {
                    let __sw2 = id;
                    let __matched = __sw2 == 1i32 || __sw2 == 0i32;
                    let mut __fall = false;
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(2u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        RedrawListMenu(((data).wrapping_add(14)).read());
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        __fall = true;
                        id = ListMenu_ProcessInput(((data).wrapping_add(14)).read());
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0)
                            && (id != (-1i32))
                        {
                            let mut activity: u32 = ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((id) as isize * 32))
                                .wrapping_add(10),
                                0,
                                7,
                                false,
                            ) as u8) as u32);
                            if (((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((id) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                == 1i32)
                                && (!((crate::c::bf_read(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset((id) as isize * 32))
                                    .wrapping_add(10),
                                    7,
                                    1,
                                    false,
                                ) as u8)
                                    != 0))
                            {
                                ((data).wrapping_add(15)).write(((id) as u8));
                                LoadWirelessStatusIndicatorSpriteGfx();
                                CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                                RedrawListMenu(((data).wrapping_add(14)).read());
                                CopyAndTranslatePlayerName(
                                    (&raw mut gStringVar1).cast::<u8>(),
                                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((data).wrapping_add(15)).read()) as i32) as isize
                                                * 32,
                                        ),
                                );
                                CreateTask_RfuReconnectWithParent(
                                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(
                                            ((((data).wrapping_add(15)).read()) as i32) as isize
                                                * 32,
                                        ))
                                    .wrapping_add(16))
                                    .cast::<u8>(),
                                    ReadAsU16(
                                        ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset(
                                                ((((data).wrapping_add(15)).read()) as i32)
                                                    as isize
                                                    * 32,
                                            ))
                                        .wrapping_add(2))
                                        .cast::<u8>(),
                                    ),
                                );
                                PlaySE(110u16);
                                ((data).wrapping_add(8)).write(4u8);
                            } else {
                                PlaySE(7u16);
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                ((data).wrapping_add(8)).write(6u8);
                            }
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                MG_AddMessageTextPrinter(
                    ((&raw const sText_AwaitingPlayersResponse)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                CopyAndTranslatePlayerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((((data).wrapping_add(15)).read()) as i32) as isize * 32),
                );
                ((data).wrapping_add(8)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(
                        (crate::c::bf_read(
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(15)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8),
                    );
                    ((data).wrapping_add(8)).write(10u8);
                }
                'l3: {
                    let __sw3 = ((RfuGetStatus()) as i32);
                    if __sw3 == 1i32 || __sw3 == 2i32 || __sw3 == 6i32 {
                        ((data).wrapping_add(8)).write(8u8);
                        break 'l3;
                    }
                    if __sw3 == 5i32 {
                        MG_AddMessageTextPrinter(
                            ((&raw const sText_PlayerSentBackOK).cast::<u8>().cast_mut())
                                .cast::<u8>(),
                        );
                        RfuSetStatus(0u8, 0u16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 || __sw1 == 8i32 || __sw1 == 10i32 {
                DestroyListMenuTask(
                    ((data).wrapping_add(14)).read(),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                CopyBgTilemapBufferToVram(0u8);
                RemoveWindow(((data).wrapping_add(13)).read());
                RemoveWindow(((data).wrapping_add(11)).read());
                DestroyTask(((data).wrapping_add(17)).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                let __p4 = (data).wrapping_add(8);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    ((((&raw const sLinkDroppedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((RfuGetStatus()) as i32) as isize))
                    .read(),
                )) != 0
                {
                    DestroyWirelessStatusIndicatorSprite();
                    DestroyTask(taskId);
                    LinkRfu_Shutdown();
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                DestroyWirelessStatusIndicatorSprite();
                MG_AddMessageTextPrinter(
                    ((&raw const sText_PleaseStartOver).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DestroyTask(taskId);
                LinkRfu_Shutdown();
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                break 'l1;
            }
            if __sw1 == 11i32 {
                let __p5 = (data).wrapping_add(8);
                (__p5).write(((__p5).read()).wrapping_add(1));
                SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (IsLinkTaskFinished()) != 0 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_LinkMysteryGiftOverWireless(activity: u32) {
    unsafe {
        let mut activity = activity;
        let mut taskId: u8 = 0u8;
        let mut data: *mut u8 = core::ptr::null_mut();
        taskId = CreateTask(Some(Task_CardOrNewsOverWireless), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .cast::<u8>();
            data = __v1;
            __v1
        });
        ((&raw mut sGroup).cast::<u8>().cast::<*mut u8>()).write(data);
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((data).wrapping_add(18)).write((((activity).wrapping_sub(21u32)) as u8));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_CardOrNewsOverWireless(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: i32 = 0i32;
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                SetHostRfuGameData(0u8, 0u32, 0u32);
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_JoinGroup();
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((data).cast::<*mut u8>()).write(AllocZeroed(512u32));
                ((data).wrapping_add(8)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                MG_AddMessageTextPrinter(
                    ((&raw const sText_SearchingForWirelessSystemWait)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((data).wrapping_add(8)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 16u8);
                ((data).wrapping_add(17)).write(CreateTask_ListenForWonderDistributor(
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    ((((((data).wrapping_add(18)).read()) as i32).wrapping_add(7i32)) as u32),
                ));
                if (((data).wrapping_add(19)).read()) != 0 {
                    (&raw mut winTemplate)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<8>>()
                        .write_unaligned(
                            (&raw const sWindowTemplate_GroupList)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<8>>()
                                .read_unaligned(),
                        );
                    (((&raw mut winTemplate).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .write(GetMysteryGiftBaseBlock());
                    ((data).wrapping_add(11))
                        .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
                    MG_DrawTextBorder(((data).wrapping_add(11)).read());
                    (&raw mut gMultiuseListMenuTemplate)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sListMenuTemplate_UnionRoomGroups)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                        .write(((data).wrapping_add(11)).read());
                    ((data).wrapping_add(14)).write(ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        0u16,
                        0u16,
                    ));
                    CopyBgTilemapBufferToVram(0u8);
                }
                ((data).wrapping_add(15)).write(0u8);
                ((data).wrapping_add(8)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                id = ((GetNewLeaderCandidate()) as i32);
                'l2: {
                    let __sw2 = id;
                    let __matched = __sw2 == 1i32 || __sw2 == 0i32;
                    let mut __fall = false;
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(2u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        if (((data).wrapping_add(19)).read()) != 0 {
                            RedrawListMenu(((data).wrapping_add(14)).read());
                        }
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        __fall = true;
                        if (((data).wrapping_add(19)).read()) != 0 {
                            id = ListMenu_ProcessInput(((data).wrapping_add(14)).read());
                        }
                        if ((((data).wrapping_add(20)).read()) as i32) > 120i32 {
                            if (((crate::c::bf_read(
                                ((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                == 1i32)
                                && (!((crate::c::bf_read(
                                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_add(10),
                                    7,
                                    1,
                                    false,
                                ) as u8)
                                    != 0))
                            {
                                if (HasWonderCardOrNewsByLinkGroup(
                                    ((((data).cast::<*mut u8>()).read()).cast::<u8>()),
                                    ((((((data).wrapping_add(18)).read()) as i32)
                                        .wrapping_add(7i32))
                                        as i16),
                                )) != 0
                                {
                                    ((data).wrapping_add(15)).write(0u8);
                                    ((data).wrapping_add(20)).write(0u8);
                                    LoadWirelessStatusIndicatorSpriteGfx();
                                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                                    CreateTask_RfuReconnectWithParent(
                                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_add(16))
                                        .cast::<u8>(),
                                        ReadAsU16(
                                            (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                                .wrapping_add(2))
                                            .cast::<u8>(),
                                        ),
                                    );
                                    PlaySE(110u16);
                                    ((data).wrapping_add(8)).write(4u8);
                                } else {
                                    PlaySE(22u16);
                                    ((data).wrapping_add(8)).write(10u8);
                                }
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                ((data).wrapping_add(8)).write(6u8);
                                ((data).wrapping_add(20)).write(0u8);
                            }
                        }
                        let __p3 = (data).wrapping_add(20);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                MG_AddMessageTextPrinter(
                    ((&raw const sText_AwaitingResponseFromWirelessSystem)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                CopyAndTranslatePlayerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((((data).wrapping_add(15)).read()) as i32) as isize * 32),
                );
                ((data).wrapping_add(8)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(
                        (crate::c::bf_read(
                            (((((data).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_add(15)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8),
                    );
                    ((data).wrapping_add(8)).write(12u8);
                }
                'l3: {
                    let __sw4 = ((RfuGetStatus()) as i32);
                    if __sw4 == 1i32 || __sw4 == 2i32 || __sw4 == 6i32 {
                        ((data).wrapping_add(8)).write(8u8);
                        break 'l3;
                    }
                    if __sw4 == 5i32 {
                        MG_AddMessageTextPrinter(
                            ((&raw const sText_WirelessLinkEstablished)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                        RfuSetStatus(0u8, 0u16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 || __sw1 == 8i32 || __sw1 == 10i32 || __sw1 == 12i32 {
                if (((data).wrapping_add(19)).read()) != 0 {
                    DestroyListMenuTask(
                        ((data).wrapping_add(14)).read(),
                        core::ptr::null_mut(),
                        core::ptr::null_mut(),
                    );
                    CopyBgTilemapBufferToVram(0u8);
                    RemoveWindow(((data).wrapping_add(11)).read());
                }
                DestroyTask(((data).wrapping_add(17)).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                let __p5 = (data).wrapping_add(8);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    ((&raw const sText_WirelessLinkDropped)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    DestroyWirelessStatusIndicatorSprite();
                    DestroyTask(taskId);
                    LinkRfu_Shutdown();
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    ((&raw const sText_WirelessSearchCanceled)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    DestroyWirelessStatusIndicatorSprite();
                    DestroyTask(taskId);
                    LinkRfu_Shutdown();
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    ((((&raw const sNoWonderSharedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((((data).wrapping_add(18)).read()) as i32) as isize))
                    .read(),
                )) != 0
                {
                    DestroyWirelessStatusIndicatorSprite();
                    DestroyTask(taskId);
                    LinkRfu_Shutdown();
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                let __p6 = (data).wrapping_add(8);
                (__p6).write(((__p6).read()).wrapping_add(1));
                SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (IsLinkTaskFinished()) != 0 {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunUnionRoom() {
    unsafe {
        let mut uroom: *mut u8 = core::ptr::null_mut();
        ResetHostRfuGameData();
        CreateTask(Some(Task_RunUnionRoom), 10u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>())
            .write((((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read());
        uroom = AllocZeroed(620u32);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write(uroom);
        ((&raw mut sURoom).cast::<u8>().cast::<*mut u8>()).write(uroom);
        ((uroom).wrapping_add(20)).write(0u8);
        ((uroom).wrapping_add(22)).write(0u8);
        ((uroom).wrapping_add(16).cast::<u16>()).write(0u16);
        ((uroom).wrapping_add(18).cast::<u16>()).write(0u16);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        ListMenuLoadStdPalAt(208u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn ReadAsU16(ptr: *mut u8) -> u16 {
    unsafe {
        let mut ptr = ptr;
        return (((((((ptr).wrapping_offset(1)).read()) as i32) << 8) | (((ptr).read()) as i32))
            as u16);
    }
}
pub(crate) unsafe extern "C" fn ScheduleFieldMessageWithFollowupState(
    nextState: u32,
    src: *mut u8,
) {
    unsafe {
        let mut nextState = nextState;
        let mut src = src;
        let mut uroom: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        ((uroom).wrapping_add(20)).write(8u8);
        ((uroom).wrapping_add(21)).write(((nextState) as u8));
        if ((src) as usize) != (((&raw mut gStringVar4).cast::<u8>()) as usize) {
            StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), src);
        }
    }
}
pub(crate) unsafe extern "C" fn ScheduleFieldMessageAndExit(src: *mut u8) {
    unsafe {
        let mut src = src;
        let mut uroom: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        ((uroom).wrapping_add(20)).write(26u8);
        if ((src) as usize) != (((&raw mut gStringVar4).cast::<u8>()) as usize) {
            StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), src);
        }
    }
}
pub(crate) unsafe extern "C" fn CopyPlayerListToBuffer(uroom: *mut u8) {
    unsafe {
        let mut uroom = uroom;
        crate::c::memcpy(
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(16128),
            ((uroom).cast::<*mut u8>()).read(),
            256u32,
        );
    }
}
pub(crate) unsafe extern "C" fn CopyPlayerListFromBuffer(uroom: *mut u8) {
    unsafe {
        let mut uroom = uroom;
        crate::c::memcpy(
            ((uroom).cast::<*mut u8>()).read(),
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(16128),
            256u32,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_RunUnionRoom(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut id: u32 = 0u32;
        let mut input: i32 = 0i32;
        let mut playerGender: i32 = 0i32;
        let mut uroom: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut taskData: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((uroom).wrapping_add(20)).read()) as i32);
            if __sw1 == 0i32 {
                ((uroom).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((uroom).wrapping_add(12).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ((uroom).cast::<*mut u8>()).write(AllocZeroed(256u32));
                ((uroom).wrapping_add(8).cast::<*mut u8>()).write(AllocZeroed(32u32));
                ClearRfuPlayerList((((uroom).cast::<*mut u8>()).read()).cast::<u8>(), 8u8);
                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(64u8);
                ((uroom).wrapping_add(32)).write(CreateTask_SearchForChildOrParent(
                    ((uroom).wrapping_add(12).cast::<*mut u8>()).read(),
                    ((uroom).wrapping_add(4).cast::<*mut u8>()).read(),
                    9u32,
                ));
                InitUnionRoomPlayerObjects(((uroom).wrapping_add(160)).cast::<u8>());
                SetTilesAroundUnionRoomPlayersPassable();
                ((uroom).wrapping_add(20)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                CreateUnionRoomPlayerSprites(
                    ((uroom).wrapping_add(33)).cast::<u8>(),
                    (((taskData).read()) as i32),
                );
                if (({
                    let __t2 = ((taskData).read()).wrapping_add(1);
                    (taskData).write(__t2);
                    __t2
                }) as i32)
                    == 8i32
                {
                    ((uroom).wrapping_add(20)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetHostRfuGameData(64u8, 0u32, 0u32);
                SetTradeBoardRegisteredMonInfo(
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as u32),
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as u32),
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read()) as u32),
                );
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_EnterUnionRoom();
                ClearRfuPlayerList(
                    (((uroom).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>(),
                    1u8,
                );
                ClearIncomingPlayerList(((uroom).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ClearIncomingPlayerList(((uroom).wrapping_add(12).cast::<*mut u8>()).read(), 4u8);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                ((uroom).wrapping_add(20)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((GetPartyMenuType()) as i32) == 8i32)
                    || (((GetPartyMenuType()) as i32) == 9i32))
                    && ((((((&raw mut sUnionRoomTrade).cast::<u8>()).cast::<u16>()).read()) as i32)
                        != 0i32)
                {
                    id = ((GetCursorSelectionMonId()) as u32);
                    'l2: {
                        let __sw3 = (((((&raw mut sUnionRoomTrade).cast::<u8>()).cast::<u16>())
                            .read()) as i32);
                        if __sw3 == 1i32 {
                            UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                            if id >= 6u32 {
                                ResetUnionRoomTrade((&raw mut sUnionRoomTrade).cast::<u8>());
                                SetTradeBoardRegisteredMonInfo(0u32, 0u32, 0u32);
                                ScheduleFieldMessageAndExit(
                                    ((&raw const sText_RegistrationCanceled)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                            } else {
                                if !((RegisterTradeMonAndGetIsEgg(
                                    ((GetCursorSelectionMonId()) as u32),
                                    (&raw mut sUnionRoomTrade).cast::<u8>(),
                                )) != 0)
                                {
                                    ScheduleFieldMessageWithFollowupState(
                                        52u32,
                                        ((&raw const sText_ChooseRequestedMonType)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>(),
                                    );
                                } else {
                                    ((uroom).wrapping_add(20)).write(55u8);
                                }
                            }
                            break 'l2;
                        }
                        if __sw3 == 2i32 {
                            CopyPlayerListFromBuffer(uroom);
                            ((taskData).wrapping_offset(1)).write(
                                (((((&raw mut sUnionRoomTrade).cast::<u8>()).wrapping_add(8))
                                    .read()) as i16),
                            );
                            if id >= 6u32 {
                                ScheduleFieldMessageAndExit(
                                    ((&raw const sText_TradeCanceled).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                );
                            } else {
                                UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                    .write(68u8);
                                RegisterTradeMon(
                                    ((GetCursorSelectionMonId()) as u32),
                                    (&raw mut sUnionRoomTrade).cast::<u8>(),
                                );
                                ((uroom).wrapping_add(20)).write(51u8);
                            }
                            break 'l2;
                        }
                    }
                    (((&raw mut sUnionRoomTrade).cast::<u8>()).cast::<u16>()).write(0u16);
                } else {
                    ((uroom).wrapping_add(20)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 0i32 {
                    if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 9i32 {
                        UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                        PlaySE(2u16);
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                        );
                        ((uroom).wrapping_add(20)).write(42u8);
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                    } else {
                        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 11i32
                        {
                            UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                            ((uroom).wrapping_add(20)).write(23u8);
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                        } else {
                            (taskData).write(0i16);
                            ((taskData).wrapping_offset(1)).write(
                                ((((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32)
                                    .wrapping_sub(1i32)) as i16),
                            );
                            ((uroom).wrapping_add(20)).write(24u8);
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                        }
                    }
                } else {
                    if ((ArePlayerFieldControlsLocked()) as i32) != 1i32 {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0
                        {
                            if (TryInteractWithUnionRoomMember(
                                ((uroom).cast::<*mut u8>()).read(),
                                taskData,
                                (taskData).wrapping_offset(1),
                                ((uroom).wrapping_add(33)).cast::<u8>(),
                            )) != 0
                            {
                                PlaySE(5u16);
                                StartScriptInteraction();
                                ((uroom).wrapping_add(20)).write(24u8);
                                break 'l1;
                            } else {
                                if (IsPlayerFacingTradingBoard()) != 0 {
                                    UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                                    PlaySE(2u16);
                                    StartScriptInteraction();
                                    StringCopy(
                                        (&raw mut gStringVar1).cast::<u8>(),
                                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                            .cast::<u8>(),
                                    );
                                    ((uroom).wrapping_add(20)).write(45u8);
                                    break 'l1;
                                }
                            }
                        }
                        'l3: {
                            let __sw4 = ((HandlePlayerListUpdate()) as i32);
                            let mut __fall = false;
                            if __sw4 == 1i32 {
                                __fall = true;
                                PlaySE(2u16);
                            }
                            if __fall || __sw4 == 2i32 {
                                __fall = true;
                                ScheduleUnionRoomPlayerRefresh(uroom);
                                break 'l3;
                            }
                            if __sw4 == 4i32 {
                                __fall = true;
                                ((uroom).wrapping_add(20)).write(11u8);
                                StartScriptInteraction();
                                SetTradeBoardRegisteredMonInfo(0u32, 0u32, 0u32);
                                UpdateGameData_SetActivity(
                                    83u8,
                                    ((GetActivePartnersInfo(uroom)) as u32),
                                    0u32,
                                );
                                break 'l3;
                            }
                        }
                        HandleUnionRoomPlayerRefresh(uroom);
                    }
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                if !((FuncIsActiveTask(Some(Task_ShowStartMenu))) != 0) {
                    UpdateGameData_SetActivity(64u8, 0u32, 0u32);
                    ((uroom).wrapping_add(20)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 24i32 {
                UR_RunTextPrinters();
                playerGender = GetUnionRoomPlayerGender(
                    ((((taskData).wrapping_offset(1)).read()) as i32),
                    ((uroom).cast::<*mut u8>()).read(),
                );
                UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                'l4: {
                    let __sw5 = UnionRoomGetPlayerInteractionResponse(
                        ((uroom).cast::<*mut u8>()).read(),
                        (((taskData).read()) as u8),
                        ((((taskData).wrapping_offset(1)).read()) as u8),
                        ((playerGender) as u32),
                    );
                    if __sw5 == 0i32 {
                        ((uroom).wrapping_add(20)).write(26u8);
                        break 'l4;
                    }
                    if __sw5 == 1i32 {
                        TryConnectToUnionRoomParent(
                            ((((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(
                                    ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                                ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                            (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                            )),
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                        );
                        ((uroom).wrapping_add(18).cast::<u16>()).write(((id) as u16));
                        ((uroom).wrapping_add(20)).write(25u8);
                        break 'l4;
                    }
                    if __sw5 == 2i32 {
                        ScheduleFieldMessageWithFollowupState(
                            19u32,
                            (&raw mut gStringVar4).cast::<u8>(),
                        );
                        break 'l4;
                    }
                }
                break 'l1;
            }
            if __sw1 == 25i32 {
                UR_RunTextPrinters();
                'l5: {
                    let __sw6 = ((RfuGetStatus()) as i32);
                    if __sw6 == 4i32 {
                        HandleCancelActivity(1u32);
                        ((uroom).wrapping_add(20)).write(4u8);
                        break 'l5;
                    }
                    if __sw6 == 1i32 || __sw6 == 2i32 {
                        if IsUnionRoomListenTaskActive() == 1u32 {
                            ScheduleFieldMessageAndExit(
                                ((&raw const sText_TrainerAppearsBusy)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                        } else {
                            ScheduleFieldMessageWithFollowupState(
                                30u32,
                                ((&raw const sText_TrainerAppearsBusy)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                        }
                        ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(64u8);
                        break 'l5;
                    }
                }
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                    CreateTask(Some(Task_ExchangeCards), 5u8);
                    ((uroom).wrapping_add(20)).write(38u8);
                }
                break 'l1;
            }
            if __sw1 == 38i32 {
                if !((FuncIsActiveTask(Some(Task_ExchangeCards))) != 0) {
                    if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 68i32
                    {
                        ScheduleFieldMessageWithFollowupState(
                            31u32,
                            ((&raw const sText_AwaitingPlayersResponseAboutTrade)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    } else {
                        ((uroom).wrapping_add(20)).write(5u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    HandleCancelActivity(0u32);
                    UpdateUnionRoomMemberFacing(
                        (((taskData).read()) as u32),
                        ((((taskData).wrapping_offset(1)).read()) as u32),
                        ((uroom).cast::<*mut u8>()).read(),
                    );
                    ((uroom).wrapping_add(20)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                id = ConvPartnerUnameAndGetWhetherMetAlready(
                    ((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                    ),
                );
                playerGender = GetUnionRoomPlayerGender(
                    ((((taskData).wrapping_offset(1)).read()) as i32),
                    ((uroom).cast::<*mut u8>()).read(),
                );
                ScheduleFieldMessageWithFollowupState(
                    6u32,
                    ((((((&raw const sHiDoSomethingTexts).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .wrapping_offset((playerGender) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                input = ListMenuHandler_AllItemsAvailable(
                    (uroom).wrapping_add(22),
                    (uroom).wrapping_add(27),
                    (uroom).wrapping_add(28),
                    (&raw const sWindowTemplate_InviteToActivity)
                        .cast::<u8>()
                        .cast_mut(),
                    (&raw const sListMenuTemplate_InviteToActivity)
                        .cast::<u8>()
                        .cast_mut(),
                );
                if input != (-1i32) {
                    if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                        ((uroom).wrapping_add(20)).write(28u8);
                    } else {
                        ((uroom).wrapping_add(152).cast::<u16>()).write(0u16);
                        playerGender = GetUnionRoomPlayerGender(
                            ((((taskData).wrapping_offset(1)).read()) as i32),
                            ((uroom).cast::<*mut u8>()).read(),
                        );
                        if (input == (-2i32)) || (input == 64i32) {
                            (((uroom).wrapping_add(76)).cast::<u16>()).write(64u16);
                            Rfu_SendPacket((((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>());
                            StringCopy(
                                (&raw mut gStringVar4).cast::<u8>(),
                                ((((&raw const sIfYouWantToDoSomethingTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(
                                    (((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(19))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read(),
                            );
                            ((uroom).wrapping_add(20)).write(32u8);
                        } else {
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                .write(((input) as u8));
                            ((&raw mut sPlayerActivityGroupSize)
                                .cast::<u8>()
                                .cast::<u8>())
                            .write(((((input) as u32) >> 8) as u8));
                            if (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                == 65i32)
                                && (!((HasAtLeastTwoMonsOfLevel30OrLower()) != 0))
                            {
                                ScheduleFieldMessageWithFollowupState(
                                    5u32,
                                    ((&raw const sText_NeedTwoMonsOfLevel30OrLower1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                            } else {
                                (((uroom).wrapping_add(76)).cast::<u16>()).write(
                                    ((((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                        .read()) as i32)
                                        | 64i32) as u16),
                                );
                                Rfu_SendPacket(
                                    (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                                );
                                ((uroom).wrapping_add(20)).write(27u8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 28i32 {
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_TrainerBattleBusy).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                ((uroom).wrapping_add(20)).write(36u8);
                break 'l1;
            }
            if __sw1 == 27i32 {
                PollPartnerYesNoResponse(uroom);
                playerGender = GetUnionRoomPlayerGender(
                    ((((taskData).wrapping_offset(1)).read()) as i32),
                    ((uroom).cast::<*mut u8>()).read(),
                );
                id = GetResponseIdx_InviteToURoomActivity(
                    ((((((uroom).wrapping_add(76)).cast::<u16>()).read()) as i32) & 63i32),
                );
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    ((((((&raw const sText_WaitOrShowCardTexts)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((playerGender) as isize * 16))
                    .cast::<*mut u8>())
                    .wrapping_offset(((id) as i32) as isize))
                    .read(),
                )) != 0
                {
                    ((taskData).wrapping_offset(3)).write(0i16);
                    ((uroom).wrapping_add(20)).write(29u8);
                }
                break 'l1;
            }
            if __sw1 == 32i32 {
                SetCloseLinkCallback();
                ((uroom).wrapping_add(20)).write(36u8);
                break 'l1;
            }
            if __sw1 == 31i32 {
                (((uroom).wrapping_add(76)).cast::<u16>()).write(68u16);
                ((((uroom).wrapping_add(76)).cast::<u16>()).wrapping_offset(1)).write(
                    (((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(14)
                        .cast::<u16>())
                    .read(),
                );
                ((((uroom).wrapping_add(76)).cast::<u16>()).wrapping_offset(2)).write(
                    (((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(16)
                        .cast::<u16>())
                    .read(),
                );
                Rfu_SendPacket((((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>());
                ((uroom).wrapping_add(20)).write(29u8);
                break 'l1;
            }
            if __sw1 == 29i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    StringCopy(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((&raw const sText_TrainerBattleBusy).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ((uroom).wrapping_add(20)).write(28u8);
                } else {
                    PollPartnerYesNoResponse(uroom);
                    if ((((uroom).wrapping_add(152).cast::<u16>()).read()) as i32) == 81i32 {
                        if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            == 8i32
                        {
                            ViewURoomPartnerTrainerCard(
                                (&raw mut gStringVar4).cast::<u8>(),
                                uroom,
                                0u8,
                            );
                            ((uroom).wrapping_add(20)).write(40u8);
                        } else {
                            ((uroom).wrapping_add(20)).write(13u8);
                        }
                    } else {
                        if ((((uroom).wrapping_add(152).cast::<u16>()).read()) as i32) == 82i32 {
                            ((uroom).wrapping_add(20)).write(32u8);
                            GetURoomActivityRejectMsg(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    | 64i32),
                                (((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(19)).read())
                                    as u32),
                            );
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(0u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                id = ConvPartnerUnameAndGetWhetherMetAlready(
                    ((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                    ),
                );
                playerGender = GetUnionRoomPlayerGender(
                    ((((taskData).wrapping_offset(1)).read()) as i32),
                    ((uroom).cast::<*mut u8>()).read(),
                );
                ScheduleFieldMessageWithFollowupState(
                    6u32,
                    ((((((&raw const sHiDoSomethingTexts).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .wrapping_offset((playerGender) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 40i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    (&raw mut gStringVar4).cast::<u8>(),
                )) != 0
                {
                    ((uroom).wrapping_add(20)).write(41u8);
                    SetLinkStandbyCallback();
                    ((uroom).wrapping_add(152).cast::<u16>()).write(0u16);
                    (((uroom).wrapping_add(154)).cast::<u16>()).write(0u16);
                }
                break 'l1;
            }
            if __sw1 == 41i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28,
                            ))
                            .wrapping_add(8))
                            .cast::<u8>(),
                        );
                        id = PlayerHasMetTrainerBefore(
                            ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                                .wrapping_add(4)
                                .cast::<u32>())
                            .read()) as u16),
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                                .wrapping_add(8))
                            .cast::<u8>(),
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((((&raw const sAwaitingResponseTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(((id) as i32) as isize))
                            .read(),
                        );
                        ((uroom).wrapping_add(20)).write(33u8);
                    } else {
                        ((uroom).wrapping_add(20)).write(7u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                'l6: {
                    let __sw7 = ((UnionRoomHandleYesNo((uroom).wrapping_add(22), 0u32)) as i32);
                    if __sw7 == 0i32 {
                        CopyBgTilemapBufferToVram(0u8);
                        ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(69u8);
                        UpdateGameData_SetActivity(69u8, 0u32, 1u32);
                        TryConnectToUnionRoomParent(
                            ((((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(
                                    ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                                ))
                            .wrapping_add(16))
                            .cast::<u8>(),
                            (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                            )),
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                        );
                        ((uroom).wrapping_add(18).cast::<u16>())
                            .write(((((taskData).wrapping_offset(1)).read()) as u16));
                        ((uroom).wrapping_add(20)).write(20u8);
                        ((taskData).wrapping_offset(3)).write(0i16);
                        break 'l6;
                    }
                    if __sw7 == 1i32 || __sw7 == (-1i32) {
                        playerGender = GetUnionRoomPlayerGender(
                            ((((taskData).wrapping_offset(1)).read()) as i32),
                            ((uroom).cast::<*mut u8>()).read(),
                        );
                        ScheduleFieldMessageAndExit(
                            ((((&raw const sDeclineChatTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((playerGender) as isize))
                            .read(),
                        );
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (({
                    let __p8 = (taskData).wrapping_offset(2);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 60i32
                {
                    ((uroom).wrapping_add(20)).write(21u8);
                    ((taskData).wrapping_offset(2)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                'l7: {
                    let __sw10 = ((RfuGetStatus()) as i32);
                    if __sw10 == 4i32 {
                        HandleCancelActivity(1u32);
                        ((uroom).wrapping_add(20)).write(4u8);
                        break 'l7;
                    }
                    if __sw10 == 1i32 || __sw10 == 2i32 {
                        playerGender = GetUnionRoomPlayerGender(
                            ((((taskData).wrapping_offset(1)).read()) as i32),
                            ((uroom).cast::<*mut u8>()).read(),
                        );
                        UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                        if IsUnionRoomListenTaskActive() == 1u32 {
                            ScheduleFieldMessageAndExit(
                                ((((&raw const sChatDeclinedTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset((playerGender) as isize))
                                .read(),
                            );
                        } else {
                            ScheduleFieldMessageWithFollowupState(
                                30u32,
                                ((((&raw const sChatDeclinedTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset((playerGender) as isize))
                                .read(),
                            );
                        }
                        break 'l7;
                    }
                    if __sw10 == 3i32 {
                        ((uroom).wrapping_add(20)).write(22u8);
                        break 'l7;
                    }
                }
                let __p11 = (taskData).wrapping_offset(3);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                if (RfuHasErrored()) != 0 {
                    playerGender = GetUnionRoomPlayerGender(
                        ((((taskData).wrapping_offset(1)).read()) as i32),
                        ((uroom).cast::<*mut u8>()).read(),
                    );
                    UpdateGameData_SetActivity(84u8, 0u32, 1u32);
                    if IsUnionRoomListenTaskActive() == 1u32 {
                        ScheduleFieldMessageAndExit(
                            ((((&raw const sChatDeclinedTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((playerGender) as isize))
                            .read(),
                        );
                    } else {
                        ScheduleFieldMessageWithFollowupState(
                            30u32,
                            ((((&raw const sChatDeclinedTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((playerGender) as isize))
                            .read(),
                        );
                    }
                }
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    ((uroom).wrapping_add(20)).write(16u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                PlaySE(73u16);
                StopUnionRoomLinkManager();
                ((uroom).wrapping_add(20)).write(12u8);
                (((uroom).wrapping_add(154)).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (RfuHasErrored()) != 0 {
                    HandleCancelActivity(0u32);
                    ((uroom).wrapping_add(20)).write(2u8);
                } else {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        CreateTrainerCardInBuffer((&raw mut gBlockSendBuffer).cast::<u8>(), 1u32);
                        CreateTask(Some(Task_ExchangeCards), 5u8);
                        ((uroom).wrapping_add(20)).write(39u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 39i32 {
                ReceiveUnionRoomActivityPacket(uroom);
                if !((FuncIsActiveTask(Some(Task_ExchangeCards))) != 0) {
                    ((uroom).wrapping_add(20)).write(33u8);
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(8))
                        .cast::<u8>(),
                    );
                    id = PlayerHasMetTrainerBefore(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(4)
                            .cast::<u32>())
                        .read()) as u16),
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(8))
                        .cast::<u8>(),
                    );
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((&raw const sPlayerContactedYouTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((id) as i32) as isize))
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 33i32 {
                ReceiveUnionRoomActivityPacket(uroom);
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    (&raw mut gStringVar4).cast::<u8>(),
                )) != 0
                {
                    ((uroom).wrapping_add(20)).write(34u8);
                }
                break 'l1;
            }
            if __sw1 == 34i32 {
                ReceiveUnionRoomActivityPacket(uroom);
                if ((HandleContactFromOtherPlayer(uroom)) != 0)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0)
                {
                    Rfu_DisconnectPlayerById(1u32);
                    StringCopy(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((&raw const sText_ChatEnded).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ((uroom).wrapping_add(20)).write(36u8);
                }
                break 'l1;
            }
            if __sw1 == 35i32 {
                ScheduleFieldMessageWithFollowupState(9u32, (&raw mut gStringVar4).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 9i32 {
                'l8: {
                    let __sw12 = ((UnionRoomHandleYesNo((uroom).wrapping_add(22), 0u32)) as i32);
                    if __sw12 == 0i32 {
                        (((uroom).wrapping_add(76)).cast::<u16>()).write(81u16);
                        if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            == 69i32
                        {
                            UpdateGameData_SetActivity(
                                ((((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    | 64i32) as u8),
                                ((GetLinkPlayerInfoFlags(1i32)) as u32),
                                0u32,
                            );
                        } else {
                            UpdateGameData_SetActivity(
                                ((((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    | 64i32) as u8),
                                ((GetLinkPlayerInfoFlags(1i32)) as u32),
                                1u32,
                            );
                        }
                        (((((uroom).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_add(27))
                        .write(0u8);
                        ((taskData).wrapping_offset(3)).write(0i16);
                        if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            == 65i32
                        {
                            if !((HasAtLeastTwoMonsOfLevel30OrLower()) != 0) {
                                (((uroom).wrapping_add(76)).cast::<u16>()).write(82u16);
                                Rfu_SendPacket(
                                    (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                                );
                                ((uroom).wrapping_add(20)).write(10u8);
                                StringCopy(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    ((&raw const sText_NeedTwoMonsOfLevel30OrLower2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                            } else {
                                Rfu_SendPacket(
                                    (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                                );
                                ((uroom).wrapping_add(20)).write(13u8);
                            }
                        } else {
                            if ((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                == 72i32
                            {
                                Rfu_SendPacket(
                                    (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                                );
                                ViewURoomPartnerTrainerCard(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    uroom,
                                    1u8,
                                );
                                ((uroom).wrapping_add(20)).write(40u8);
                            } else {
                                Rfu_SendPacket(
                                    (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                                );
                                ((uroom).wrapping_add(20)).write(13u8);
                            }
                        }
                        break 'l8;
                    }
                    if __sw12 == 1i32 || __sw12 == (-1i32) {
                        (((uroom).wrapping_add(76)).cast::<u16>()).write(82u16);
                        Rfu_SendPacket((((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>());
                        ((uroom).wrapping_add(20)).write(10u8);
                        GetYouDeclinedTheOfferMessage(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                        );
                        break 'l8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetCloseLinkCallback();
                ((uroom).wrapping_add(20)).write(36u8);
                break 'l1;
            }
            if __sw1 == 36i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(64u8);
                    ScheduleFieldMessageWithFollowupState(
                        37u32,
                        (&raw mut gStringVar4).cast::<u8>(),
                    );
                    crate::c::memset(
                        (((uroom).wrapping_add(76)).cast::<u16>()).cast::<u8>(),
                        0i32,
                        12u32,
                    );
                    (((uroom).wrapping_add(154)).cast::<u16>()).write(0u16);
                    ((uroom).wrapping_add(152).cast::<u16>()).write(0u16);
                }
                break 'l1;
            }
            if __sw1 == 37i32 {
                ((uroom).wrapping_add(20)).write(2u8);
                HandleCancelActivity(0u32);
                break 'l1;
            }
            if __sw1 == 13i32 {
                GetURoomActivityStartMsg(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read()) as i32)
                        | 64i32) as u8),
                );
                ScheduleFieldMessageWithFollowupState(14u32, (&raw mut gStringVar4).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 14i32 {
                SetLinkStandbyCallback();
                ((uroom).wrapping_add(20)).write(15u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (IsLinkTaskFinished()) != 0 {
                    ((uroom).wrapping_add(20)).write(16u8);
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                Free(((uroom).wrapping_add(8).cast::<*mut u8>()).read());
                Free(((uroom).cast::<*mut u8>()).read());
                Free(((uroom).wrapping_add(12).cast::<*mut u8>()).read());
                Free(((uroom).wrapping_add(4).cast::<*mut u8>()).read());
                DestroyTask(((uroom).wrapping_add(32)).read());
                DestroyUnionRoomPlayerSprites(((uroom).wrapping_add(33)).cast::<u8>());
                ((uroom).wrapping_add(20)).write(17u8);
                break 'l1;
            }
            if __sw1 == 17i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((uroom).wrapping_add(20)).write(18u8);
                break 'l1;
            }
            if __sw1 == 18i32 {
                if !((UpdatePaletteFade()) != 0) {
                    DestroyUnionRoomPlayerObjects();
                    DestroyTask(taskId);
                    Free((((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read());
                    CreateTask_StartActivity();
                }
                break 'l1;
            }
            if __sw1 == 42i32 {
                if ((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(8), 0, 10, false) as u16)
                    as i32)
                    == 0i32
                {
                    ((uroom).wrapping_add(20)).write(43u8);
                } else {
                    if ((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(8), 0, 10, false)
                        as u16) as i32)
                        == 412i32
                    {
                        StringCopy(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw const sText_CancelRegistrationOfEgg)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                                ((crate::c::bf_read(
                                    (GetHostRfuGameData()).wrapping_add(8),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32) as isize
                                    * 11,
                            ))
                            .cast::<u8>(),
                        );
                        ConvertIntToDecimalStringN(
                            (&raw mut gStringVar2).cast::<u8>(),
                            ((crate::c::bf_read(
                                (GetHostRfuGameData()).wrapping_add(11),
                                1,
                                7,
                                false,
                            ) as u8) as i32),
                            0i32,
                            3u8,
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            ((&raw const sText_CancelRegistrationOfMon)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    }
                    ScheduleFieldMessageWithFollowupState(
                        44u32,
                        (&raw mut gStringVar4).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 43i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    ((&raw const sText_RegisterMonAtTradingBoard)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    ((uroom).wrapping_add(20)).write(47u8);
                }
                break 'l1;
            }
            if __sw1 == 47i32 {
                input = ListMenuHandler_AllItemsAvailable(
                    (uroom).wrapping_add(22),
                    (uroom).wrapping_add(29),
                    (uroom).wrapping_add(30),
                    (&raw const sWindowTemplate_RegisterForTrade)
                        .cast::<u8>()
                        .cast_mut(),
                    (&raw const sListMenuTemplate_RegisterForTrade)
                        .cast::<u8>()
                        .cast_mut(),
                );
                if input != (-1i32) {
                    if (input == (-2i32)) || (input == 3i32) {
                        ((uroom).wrapping_add(20)).write(4u8);
                        HandleCancelActivity(1u32);
                    } else {
                        'l9: {
                            let __sw13 = input;
                            if __sw13 == 1i32 {
                                ScheduleFieldMessageWithFollowupState(
                                    53u32,
                                    ((&raw const sText_WhichMonWillYouOffer)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>(),
                                );
                                break 'l9;
                            }
                            if __sw13 == 2i32 {
                                ScheduleFieldMessageWithFollowupState(
                                    47u32,
                                    ((&raw const sText_TradingBoardInfo).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                );
                                break 'l9;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 53i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((uroom).wrapping_add(20)).write(54u8);
                break 'l1;
            }
            if __sw1 == 54i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut sUnionRoomTrade).cast::<u8>()).cast::<u16>()).write(1u16);
                    ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(FieldCB_ContinueScriptUnionRoom));
                    ChooseMonForTradingBoard(8u8, Some(CB2_ReturnToField));
                }
                break 'l1;
            }
            if __sw1 == 52i32 {
                input = ListMenuHandler_AllItemsAvailable(
                    (uroom).wrapping_add(22),
                    (uroom).wrapping_add(29),
                    (uroom).wrapping_add(30),
                    (&raw const sWindowTemplate_TradingBoardRequestType)
                        .cast::<u8>()
                        .cast_mut(),
                    (&raw const sMenuTemplate_TradingBoardRequestType)
                        .cast::<u8>()
                        .cast_mut(),
                );
                if input != (-1i32) {
                    'l10: {
                        let __sw14 = input;
                        let __matched = __sw14 == (-2i32) || __sw14 == 18i32;
                        if __sw14 == (-2i32) || __sw14 == 18i32 {
                            ResetUnionRoomTrade((&raw mut sUnionRoomTrade).cast::<u8>());
                            SetTradeBoardRegisteredMonInfo(0u32, 0u32, 0u32);
                            ScheduleFieldMessageAndExit(
                                ((&raw const sText_RegistrationCanceled)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                            break 'l10;
                        }
                        if !__matched {
                            (((&raw mut sUnionRoomTrade).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<u16>())
                            .write(((input) as u16));
                            ((uroom).wrapping_add(20)).write(55u8);
                            break 'l10;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 55i32 {
                SetTradeBoardRegisteredMonInfo(
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as u32),
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as u32),
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read()) as u32),
                );
                ScheduleFieldMessageAndExit(
                    ((&raw const sText_RegistrationCompleted)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 44i32 {
                'l11: {
                    let __sw15 = ((UnionRoomHandleYesNo((uroom).wrapping_add(22), 0u32)) as i32);
                    if __sw15 == 0i32 {
                        ((uroom).wrapping_add(20)).write(56u8);
                        break 'l11;
                    }
                    if __sw15 == 1i32 || __sw15 == (-1i32) {
                        HandleCancelActivity(1u32);
                        ((uroom).wrapping_add(20)).write(4u8);
                        break 'l11;
                    }
                }
                break 'l1;
            }
            if __sw1 == 56i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    ((&raw const sText_RegistrationCanceled2)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    SetTradeBoardRegisteredMonInfo(0u32, 0u32, 0u32);
                    ResetUnionRoomTrade((&raw mut sUnionRoomTrade).cast::<u8>());
                    HandleCancelActivity(1u32);
                    ((uroom).wrapping_add(20)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 45i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    ((&raw const sText_XCheckedTradingBoard)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    ((uroom).wrapping_add(20)).write(46u8);
                }
                break 'l1;
            }
            if __sw1 == 46i32 {
                UR_ClearBg0();
                ((uroom).wrapping_add(20)).write(48u8);
                break 'l1;
            }
            if __sw1 == 48i32 {
                input = TradeBoardMenuHandler(
                    (uroom).wrapping_add(22),
                    (uroom).wrapping_add(29),
                    (uroom).wrapping_add(74),
                    (uroom).wrapping_add(30),
                    (&raw const sWindowTemplate_TradingBoardMain)
                        .cast::<u8>()
                        .cast_mut(),
                    (&raw const sTradeBoardListMenuTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    ((uroom).cast::<*mut u8>()).read(),
                );
                if input != (-1i32) {
                    'l12: {
                        let __sw16 = input;
                        let __matched = __sw16 == (-2i32) || __sw16 == 8i32;
                        if __sw16 == (-2i32) || __sw16 == 8i32 {
                            HandleCancelActivity(1u32);
                            ((uroom).wrapping_add(20)).write(4u8);
                            break 'l12;
                        }
                        if !__matched {
                            UR_ClearBg0();
                            'l13: {
                                let __sw17 = IsRequestedTradeInPlayerParty(
                                    ((crate::c::bf_read(
                                        (((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset((input) as isize * 32))
                                        .wrapping_add(9),
                                        2,
                                        6,
                                        false,
                                    ) as u16) as u32),
                                    ((crate::c::bf_read(
                                        (((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset((input) as isize * 32))
                                        .wrapping_add(8),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as u32),
                                );
                                if __sw17 == 0i32 {
                                    CopyAndTranslatePlayerName(
                                        (&raw mut gStringVar1).cast::<u8>(),
                                        ((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset((input) as isize * 32),
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        49u32,
                                        ((&raw const sText_AskTrainerToMakeTrade)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>(),
                                    );
                                    ((taskData).wrapping_offset(1)).write(((input) as i16));
                                    break 'l13;
                                }
                                if __sw17 == 1i32 {
                                    CopyAndTranslatePlayerName(
                                        (&raw mut gStringVar1).cast::<u8>(),
                                        ((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset((input) as isize * 32),
                                    );
                                    StringCopy(
                                        (&raw mut gStringVar2).cast::<u8>(),
                                        (((&raw mut gTypeNames).cast::<u8>()).wrapping_offset(
                                            ((crate::c::bf_read(
                                                (((((uroom).cast::<*mut u8>()).read())
                                                    .cast::<u8>())
                                                .wrapping_offset((input) as isize * 32))
                                                .wrapping_add(9),
                                                2,
                                                6,
                                                false,
                                            ) as u16)
                                                as i32)
                                                as isize
                                                * 7,
                                        ))
                                        .cast::<u8>(),
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        46u32,
                                        ((&raw const sText_DontHaveTypeTrainerWants)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>(),
                                    );
                                    break 'l13;
                                }
                                if __sw17 == 2i32 {
                                    CopyAndTranslatePlayerName(
                                        (&raw mut gStringVar1).cast::<u8>(),
                                        ((((uroom).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset((input) as isize * 32),
                                    );
                                    StringCopy(
                                        (&raw mut gStringVar2).cast::<u8>(),
                                        (((&raw mut gTypeNames).cast::<u8>()).wrapping_offset(
                                            ((crate::c::bf_read(
                                                (((((uroom).cast::<*mut u8>()).read())
                                                    .cast::<u8>())
                                                .wrapping_offset((input) as isize * 32))
                                                .wrapping_add(9),
                                                2,
                                                6,
                                                false,
                                            ) as u16)
                                                as i32)
                                                as isize
                                                * 7,
                                        ))
                                        .cast::<u8>(),
                                    );
                                    ScheduleFieldMessageWithFollowupState(
                                        46u32,
                                        ((&raw const sText_DontHaveEggTrainerWants)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>(),
                                    );
                                    break 'l13;
                                }
                            }
                            break 'l12;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 49i32 {
                'l14: {
                    let __sw18 = ((UnionRoomHandleYesNo((uroom).wrapping_add(22), 0u32)) as i32);
                    if __sw18 == 0i32 {
                        ((uroom).wrapping_add(20)).write(50u8);
                        break 'l14;
                    }
                    if __sw18 == (-1i32) || __sw18 == 1i32 {
                        HandleCancelActivity(1u32);
                        ((uroom).wrapping_add(20)).write(4u8);
                        break 'l14;
                    }
                }
                break 'l1;
            }
            if __sw1 == 50i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    ((&raw const sText_WhichMonWillYouOffer)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) != 0
                {
                    (((&raw mut sUnionRoomTrade).cast::<u8>()).cast::<u16>()).write(2u16);
                    crate::c::memcpy(
                        (&raw mut gRfuPartnerCompatibilityData).cast::<u8>(),
                        (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                            ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                        )),
                        4u32,
                    );
                    ((&raw mut gUnionRoomRequestedMonType)
                        .cast::<u8>()
                        .cast::<u8>())
                    .write(
                        ((crate::c::bf_read(
                            (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(9),
                            2,
                            6,
                            false,
                        ) as u16) as u8),
                    );
                    ((&raw mut gUnionRoomOfferedSpecies)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(
                        (crate::c::bf_read(
                            (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                                ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                            ))
                            .wrapping_add(8),
                            0,
                            10,
                            false,
                        ) as u16),
                    );
                    ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(FieldCB_ContinueScriptUnionRoom));
                    ChooseMonForTradingBoard(9u8, Some(CB2_ReturnToField));
                    CopyPlayerListToBuffer(uroom);
                    (((&raw mut sUnionRoomTrade).cast::<u8>()).wrapping_add(8))
                        .write(((((taskData).wrapping_offset(1)).read()) as u8));
                }
                break 'l1;
            }
            if __sw1 == 51i32 {
                ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(68u8);
                TryConnectToUnionRoomParent(
                    ((((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(16))
                    .cast::<u8>(),
                    (((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                    )),
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).read(),
                );
                CopyAndTranslatePlayerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((uroom).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(
                        ((((taskData).wrapping_offset(1)).read()) as i32) as isize * 32,
                    ),
                );
                UR_PrintFieldMessage(
                    ((((&raw const sCommunicatingWaitTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(2))
                    .read(),
                );
                ((uroom).wrapping_add(20)).write(25u8);
                break 'l1;
            }
            if __sw1 == 26i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    (&raw mut gStringVar4).cast::<u8>(),
                )) != 0
                {
                    HandleCancelActivity(1u32);
                    UpdateUnionRoomMemberFacing(
                        (((taskData).read()) as u32),
                        ((((taskData).wrapping_offset(1)).read()) as u32),
                        ((uroom).cast::<*mut u8>()).read(),
                    );
                    ((uroom).wrapping_add(20)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (PrintOnTextbox(
                    (uroom).wrapping_add(22),
                    (&raw mut gStringVar4).cast::<u8>(),
                )) != 0
                {
                    ((uroom).wrapping_add(20)).write(((uroom).wrapping_add(21)).read());
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUsingUnionRoomStartMenu() {
    unsafe {
        if InUnionRoom() == 1u32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(11u16);
        }
    }
}
pub(crate) unsafe extern "C" fn ReceiveUnionRoomActivityPacket(data: *mut u8) {
    unsafe {
        let mut data = data;
        if ((((((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
            .wrapping_offset(1))
        .read()) as i32)
            != 0i32)
            && ((((((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>()).read())
                as i32)
                & 65280i32)
                == 12032i32)
        {
            (((data).wrapping_add(154)).cast::<u16>()).write(
                (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                    .wrapping_offset(1))
                .read(),
            );
            if (((((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                .wrapping_offset(1))
            .read()) as i32)
                == 68i32
            {
                ((((data).wrapping_add(154)).cast::<u16>()).wrapping_offset(1)).write(
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                        .wrapping_offset(2))
                    .read(),
                );
                ((((data).wrapping_add(154)).cast::<u16>()).wrapping_offset(2)).write(
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                        .wrapping_offset(3))
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleContactFromOtherPlayer(uroom: *mut u8) -> u32 {
    unsafe {
        let mut uroom = uroom;
        if (((((uroom).wrapping_add(154)).cast::<u16>()).read()) as i32) != 0i32 {
            let mut id: i32 = GetChatLeaderActionRequestMessage(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28)).wrapping_add(19))
                    .read()) as u32),
                ((uroom).wrapping_add(154)).cast::<u16>(),
                uroom,
            );
            if id == 0i32 {
                return 1u32;
            } else {
                if id == 1i32 {
                    ((uroom).wrapping_add(20)).write(35u8);
                    ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>())
                        .write((((((uroom).wrapping_add(154)).cast::<u16>()).read()) as u8));
                    return 0u32;
                } else {
                    if id == 2i32 {
                        ((uroom).wrapping_add(20)).write(36u8);
                        SetCloseLinkCallback();
                        return 0u32;
                    }
                }
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoom() {
    unsafe {
        let mut data: *mut u8 = core::ptr::null_mut();
        (((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>()).write(255u8);
        CreateTask(Some(Task_InitUnionRoom), 0u8);
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>())
            .write((((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read());
        (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).write({
            let __v1 = AllocZeroed(620u32);
            data = __v1;
            __v1
        });
        ((&raw mut sURoom).cast::<u8>().cast::<*mut u8>())
            .write((((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read());
        ((data).wrapping_add(20)).write(0u8);
        ((data).wrapping_add(22)).write(0u8);
        ((data).wrapping_add(16).cast::<u16>()).write(0u16);
        ((data).wrapping_add(18).cast::<u16>()).write(0u16);
        (((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>()).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn Task_InitUnionRoom(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        'l1: {
            let __sw1 = ((((data).wrapping_add(20)).read()) as i32);
            if __sw1 == 0i32 {
                ((data).wrapping_add(20)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetHostRfuGameData(12u8, 0u32, 0u32);
                SetWirelessCommType1();
                OpenLink();
                InitializeRfuLinkManager_EnterUnionRoom();
                RfuSetIgnoreError(1u32);
                ((data).wrapping_add(20)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((data).wrapping_add(4).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ClearIncomingPlayerList(((data).wrapping_add(4).cast::<*mut u8>()).read(), 4u8);
                ((data).wrapping_add(12).cast::<*mut u8>()).write(AllocZeroed(112u32));
                ClearIncomingPlayerList(((data).wrapping_add(12).cast::<*mut u8>()).read(), 4u8);
                ((data).cast::<*mut u8>()).write(AllocZeroed(256u32));
                ClearRfuPlayerList((((data).cast::<*mut u8>()).read()).cast::<u8>(), 8u8);
                ((data).wrapping_add(8).cast::<*mut u8>()).write(AllocZeroed(32u32));
                ClearRfuPlayerList(
                    (((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>(),
                    1u8,
                );
                ((data).wrapping_add(32)).write(CreateTask_SearchForChildOrParent(
                    ((data).wrapping_add(12).cast::<*mut u8>()).read(),
                    ((data).wrapping_add(4).cast::<*mut u8>()).read(),
                    10u32,
                ));
                ((data).wrapping_add(20)).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l2: {
                    let __sw2 = ((HandlePlayerListUpdate()) as i32);
                    if __sw2 == 1i32 || __sw2 == 2i32 {
                        if (((((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>()).read())
                            as i32)
                            == 255i32
                        {
                            {
                                i = 0i32;
                                'l3: loop {
                                    if !(i < 8i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if ((crate::c::bf_read(
                                            (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                                .wrapping_offset((i) as isize * 32))
                                            .wrapping_add(26),
                                            0,
                                            2,
                                            false,
                                        ) as u8) as i32)
                                            == 1i32
                                        {
                                            CopyAndTranslatePlayerName(
                                                (&raw mut text).cast::<u8>(),
                                                ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                                    .wrapping_offset((i) as isize * 32),
                                            );
                                            if (PlayerHasMetTrainerBefore(
                                                ReadAsU16(
                                                    ((((((data).cast::<*mut u8>()).read())
                                                        .cast::<u8>())
                                                    .wrapping_offset((i) as isize * 32))
                                                    .wrapping_add(2))
                                                    .cast::<u8>(),
                                                ),
                                                (&raw mut text).cast::<u8>(),
                                            )) != 0
                                            {
                                                StringCopy(
                                                    ((&raw mut sUnionRoomPlayerName).cast::<u8>())
                                                        .cast::<u8>(),
                                                    (&raw mut text).cast::<u8>(),
                                                );
                                                break 'l3;
                                            }
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                        }
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                Free(((data).wrapping_add(8).cast::<*mut u8>()).read());
                Free(((data).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(12).cast::<*mut u8>()).read());
                Free(((data).wrapping_add(4).cast::<*mut u8>()).read());
                DestroyTask(((data).wrapping_add(32)).read());
                Free((((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read());
                LinkRfu_Shutdown();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferUnionRoomPlayerName() -> u16 {
    unsafe {
        if (((((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>()).read()) as i32) != 255i32
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>(),
            );
            (((&raw mut sUnionRoomPlayerName).cast::<u8>()).cast::<u8>()).write(255u8);
            return 1u16;
        } else {
            return 0u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn HandlePlayerListUpdate() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: u8 = 0u8;
        let mut data: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut retVal: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((ArePlayersDifferent(
                        (((((data).wrapping_add(12).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28)),
                        (&raw const sUnionRoomPlayer_DummyRfu)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as i32)
                        == 1i32
                    {
                        ((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (((((data).wrapping_add(12).cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                            );
                        (((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_add(24)
                            .cast::<u16>())
                        .write(0u16);
                        crate::c::bf_write(
                            ((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_add(26),
                            0,
                            2,
                            (1u8) as i32,
                        );
                        (((((data).wrapping_add(8).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_add(27))
                        .write(1u8);
                        return 4u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0u8;
            'l3: loop {
                if !(((j) as i32) < 8i32) {
                    break 'l3;
                }
                'l4: {
                    if ((crate::c::bf_read(
                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize * 32))
                        .wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        != 0i32
                    {
                        i = ((GetNewIncomingPlayerId(
                            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 32),
                            (((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>(),
                        )) as i32);
                        if i != 255i32 {
                            if ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                == 1i32
                            {
                                if (ArePlayerDataDifferent(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32)),
                                    (((((data).wrapping_add(4).cast::<*mut u8>()).read())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize * 28)),
                                )) != 0
                                {
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .cast::<crate::c::Rec4<24>>()
                                    .write_unaligned(
                                        (((((data).wrapping_add(4).cast::<*mut u8>()).read())
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                    );
                                    ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(27))
                                    .write(64u8);
                                    retVal = 1i32;
                                } else {
                                    if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(27))
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        let __p1 = (((((data).cast::<*mut u8>()).read())
                                            .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                        .wrapping_add(27);
                                        (__p1).write(((__p1).read()).wrapping_sub(1));
                                        if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 32))
                                        .wrapping_add(27))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            retVal = 2i32;
                                        }
                                    }
                                }
                            } else {
                                crate::c::bf_write(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(26),
                                    0,
                                    2,
                                    (1u8) as i32,
                                );
                                ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 32))
                                .wrapping_add(27))
                                .write(0u8);
                                retVal = 2i32;
                            }
                            ((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize * 32))
                            .wrapping_add(24)
                            .cast::<u16>())
                            .write(0u16);
                        } else {
                            if ((crate::c::bf_read(
                                (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 32))
                                .wrapping_add(26),
                                0,
                                2,
                                false,
                            ) as u8) as i32)
                                != 2i32
                            {
                                let __p2 = (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 32))
                                .wrapping_add(24)
                                .cast::<u16>();
                                (__p2).write(((__p2).read()).wrapping_add(1));
                                if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 32))
                                .wrapping_add(24)
                                .cast::<u16>())
                                .read()) as i32)
                                    >= 600i32
                                {
                                    crate::c::bf_write(
                                        (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 32))
                                        .wrapping_add(26),
                                        0,
                                        2,
                                        (2u8) as i32,
                                    );
                                    retVal = 2i32;
                                }
                            } else {
                                if ((crate::c::bf_read(
                                    (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(26),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32)
                                    == 2i32
                                {
                                    let __p3 = (((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(24)
                                    .cast::<u16>();
                                    (__p3).write(((__p3).read()).wrapping_add(1));
                                    if ((((((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 32))
                                    .wrapping_add(24)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        >= 900i32
                                    {
                                        ClearRfuPlayerList(
                                            ((((data).cast::<*mut u8>()).read()).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize * 32),
                                            1u8,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if ((TryAddIncomingPlayerToList(
                        (((data).cast::<*mut u8>()).read()).cast::<u8>(),
                        ((((data).wrapping_add(4).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28),
                        8u8,
                    )) as i32)
                        != 255i32
                    {
                        retVal = 1i32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((retVal) as u8);
    }
}
pub(crate) unsafe extern "C" fn Task_SearchForChildOrParent(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut rfu = crate::ffi::Align4([0u8; 24]);
        let mut list: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        let mut isParent: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    isParent = Rfu_GetCompatiblePlayerData(
                        ((&raw mut rfu).cast::<u8>()),
                        (((&raw mut rfu).cast::<u8>()).wrapping_add(16)).cast::<u8>(),
                        ((i) as u8),
                    );
                    if !((IsPartnerActivityAcceptable(
                        ((crate::c::bf_read(
                            ((&raw mut rfu).cast::<u8>()).wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8) as u32),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as u32),
                    )) != 0)
                    {
                        (&raw mut rfu)
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (&raw const sUnionRoomPlayer_DummyRfu)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<24>>()
                                    .read_unaligned(),
                            );
                    }
                    if ((crate::c::bf_read(
                        ((&raw mut rfu).cast::<u8>()).wrapping_add(0),
                        0,
                        4,
                        false,
                    ) as u16) as i32)
                        == 1i32
                    {
                        (&raw mut rfu)
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (&raw const sUnionRoomPlayer_DummyRfu)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<24>>()
                                    .read_unaligned(),
                            );
                    }
                    if !((isParent) != 0) {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < i) {
                                    break 'l3;
                                }
                                'l4: {
                                    if !((ArePlayersDifferent(
                                        (((((list).wrapping_offset(1)).read()).cast::<u8>())
                                            .wrapping_offset((j) as isize * 28)),
                                        (&raw mut rfu).cast::<u8>(),
                                    )) != 0)
                                    {
                                        (&raw mut rfu)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<24>>()
                                            .write_unaligned(
                                                (&raw const sUnionRoomPlayer_DummyRfu)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<crate::c::Rec4<24>>()
                                                    .read_unaligned(),
                                            );
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        (((((list).wrapping_offset(1)).read()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw mut rfu)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                        crate::c::bf_write(
                            (((((list).wrapping_offset(1)).read()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .wrapping_add(24),
                            0,
                            1,
                            (ArePlayersDifferent(
                                (((((list).wrapping_offset(1)).read()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28)),
                                (&raw const sUnionRoomPlayer_DummyRfu)
                                    .cast::<u8>()
                                    .cast_mut(),
                            )) as i32,
                        );
                    } else {
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (&raw mut rfu)
                                    .cast::<u8>()
                                    .cast::<crate::c::Rec4<24>>()
                                    .read_unaligned(),
                            );
                        crate::c::bf_write(
                            ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                                .wrapping_add(24),
                            0,
                            1,
                            (ArePlayersDifferent(
                                ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                                (&raw const sUnionRoomPlayer_DummyRfu)
                                    .cast::<u8>()
                                    .cast_mut(),
                            )) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_SearchForChildOrParent(
    parentList: *mut u8,
    childList: *mut u8,
    linkGroup: u32,
) -> u8 {
    unsafe {
        let mut parentList = parentList;
        let mut childList = childList;
        let mut linkGroup = linkGroup;
        let mut taskId: u8 = CreateTask(Some(Task_SearchForChildOrParent), 0u8);
        let mut data: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        (data).write(parentList);
        ((data).wrapping_offset(1)).write(childList);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((linkGroup) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_ListenForCompatiblePartners(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut list: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    Rfu_GetCompatiblePlayerData(
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                        (((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .wrapping_add(16))
                        .cast::<u8>(),
                        ((i) as u8),
                    );
                    if !((IsPartnerActivityAcceptable(
                        ((crate::c::bf_read(
                            ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                                .wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8) as u32),
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u32),
                    )) != 0)
                    {
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .cast::<crate::c::Rec4<24>>()
                            .write_unaligned(
                                (&raw const sUnionRoomPlayer_DummyRfu)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<crate::c::Rec4<24>>()
                                    .read_unaligned(),
                            );
                    }
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < i) {
                                break 'l3;
                            }
                            'l4: {
                                if !((ArePlayersDifferent(
                                    ((((list).read()).cast::<u8>())
                                        .wrapping_offset((j) as isize * 28)),
                                    ((((list).read()).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28)),
                                )) != 0)
                                {
                                    ((((list).read()).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .cast::<crate::c::Rec4<24>>()
                                    .write_unaligned(
                                        (&raw const sUnionRoomPlayer_DummyRfu)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<crate::c::Rec4<24>>()
                                            .read_unaligned(),
                                    );
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    crate::c::bf_write(
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .wrapping_add(24),
                        0,
                        1,
                        (ArePlayersDifferent(
                            ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                            (&raw const sUnionRoomPlayer_DummyRfu)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HasWonderCardOrNewsByLinkGroup(
    data: *mut u8,
    linkGroup: i16,
) -> u32 {
    unsafe {
        let mut data = data;
        let mut linkGroup = linkGroup;
        if ((linkGroup) as i32) == 7i32 {
            if !((crate::c::bf_read((data).wrapping_add(0), 5, 1, false) as u16) != 0) {
                return 0u32;
            } else {
                return 1u32;
            }
        } else {
            if ((linkGroup) as i32) == 8i32 {
                if !((crate::c::bf_read((data).wrapping_add(0), 4, 1, false) as u16) != 0) {
                    return 0u32;
                } else {
                    return 1u32;
                }
            } else {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ListenForWonderDistributor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut list: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (Rfu_GetWonderDistributorPlayerData(
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                        (((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .wrapping_add(16))
                        .cast::<u8>(),
                        ((i) as u8),
                    )) != 0
                    {
                        HasWonderCardOrNewsByLinkGroup(
                            ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read(),
                        );
                    }
                    crate::c::bf_write(
                        ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28))
                            .wrapping_add(24),
                        0,
                        1,
                        (ArePlayersDifferent(
                            ((((list).read()).cast::<u8>()).wrapping_offset((i) as isize * 28)),
                            (&raw const sUnionRoomPlayer_DummyRfu)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_ListenForCompatiblePartners(
    list: *mut u8,
    linkGroup: u32,
) -> u8 {
    unsafe {
        let mut list = list;
        let mut linkGroup = linkGroup;
        let mut taskId: u8 = CreateTask(Some(Task_ListenForCompatiblePartners), 0u8);
        let mut oldList: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        (oldList).write(list);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((linkGroup) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn CreateTask_ListenForWonderDistributor(
    list: *mut u8,
    linkGroup: u32,
) -> u8 {
    unsafe {
        let mut list = list;
        let mut linkGroup = linkGroup;
        let mut taskId: u8 = CreateTask(Some(Task_ListenForWonderDistributor), 0u8);
        let mut oldList: *mut *mut u8 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>())
        .cast::<*mut u8>();
        (oldList).write(list);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((linkGroup) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn UR_PrintFieldMessage(src: *mut u8) -> u32 {
    unsafe {
        let mut src = src;
        LoadMessageBoxAndBorderGfx();
        DrawDialogueFrame(0u8, 1u8);
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), src);
        AddTextPrinterWithCustomSpeedForMessage(0u8, 1u8);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UR_RunTextPrinters() -> u32 {
    unsafe {
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn PrintOnTextbox(textState: *mut u8, str: *mut u8) -> u8 {
    unsafe {
        let mut textState = textState;
        let mut str = str;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                LoadMessageBoxAndBorderGfx();
                DrawDialogueFrame(0u8, 1u8);
                StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str);
                AddTextPrinterForMessage_2(1u8);
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
                    (textState).write(0u8);
                    return 1u8;
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UnionRoomHandleYesNo(state: *mut u8, noDraw: u32) -> i8 {
    unsafe {
        let mut state = state;
        let mut noDraw = noDraw;
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                if (noDraw) != 0 {
                    return (-3i8);
                }
                DisplayYesNoMenuDefaultYes();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (noDraw) != 0 {
                    EraseYesNoWindow();
                    (state).write(0u8);
                    return (-3i8);
                }
                input = Menu_ProcessInputNoWrapClearOnChoose();
                if ((((input) as i32) == (-1i32)) || (((input) as i32) == 0i32))
                    || (((input) as i32) == 1i32)
                {
                    (state).write(0u8);
                    return input;
                }
                break 'l1;
            }
        }
        return (-2i8);
    }
}
pub(crate) unsafe extern "C" fn CreateTradeBoardWindow(template: *mut u8) -> u8 {
    unsafe {
        let mut template = template;
        let mut windowId: u8 = ((AddWindow(template)) as u8);
        DrawStdWindowFrame(windowId, 0u8);
        FillWindowPixelBuffer(windowId, 255u8);
        PrintUnionRoomText(
            windowId,
            1u8,
            ((&raw const sText_NameWantedOfferLv).cast::<u8>().cast_mut()).cast::<u8>(),
            8u8,
            1u8,
            6u8,
        );
        CopyWindowToVram(windowId, 2u8);
        PutWindowTilemap(windowId);
        return windowId;
    }
}
pub(crate) unsafe extern "C" fn DeleteTradeBoardWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        RemoveWindow(windowId);
    }
}
pub(crate) unsafe extern "C" fn ListMenuHandler_AllItemsAvailable(
    state: *mut u8,
    windowId: *mut u8,
    listMenuId: *mut u8,
    winTemplate: *mut u8,
    menuTemplate: *mut u8,
) -> i32 {
    unsafe {
        let mut state = state;
        let mut windowId = windowId;
        let mut listMenuId = listMenuId;
        let mut winTemplate = winTemplate;
        let mut menuTemplate = menuTemplate;
        let mut maxWidth: i32 = 0i32;
        let mut input: i32 = 0i32;
        let mut winTemplateCopy = crate::ffi::Align4([0u8; 8]);
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                (&raw mut winTemplateCopy)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(winTemplate.cast::<crate::c::Rec4<8>>().read_unaligned());
                maxWidth = Intl_GetListMenuWidth(menuTemplate);
                if (((((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(3)).read()) as i32)
                    > maxWidth
                {
                    (((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(3))
                        .write(((maxWidth) as u8));
                }
                if (((((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(1)).read()) as i32)
                    .wrapping_add(
                        (((((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(3)).read())
                            as i32),
                    )
                    >= crate::c::div_i32(240i32, 8i32)
                {
                    (((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(1)).write(
                        ((if ((crate::c::div_i32(240i32, 8i32)).wrapping_sub(1i32)).wrapping_sub(
                            (((((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(3)).read())
                                as i32),
                        ) >= 0i32
                        {
                            ((crate::c::div_i32(240i32, 8i32)).wrapping_sub(1i32)).wrapping_sub(
                                (((((&raw mut winTemplateCopy).cast::<u8>()).wrapping_add(3))
                                    .read()) as i32),
                            )
                        } else {
                            0i32
                        }) as u8),
                    );
                }
                (windowId).write(((AddWindow((&raw mut winTemplateCopy).cast::<u8>())) as u8));
                DrawStdWindowFrame((windowId).read(), 0u8);
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(menuTemplate.cast::<crate::c::Rec4<24>>().read_unaligned());
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write((windowId).read());
                (listMenuId).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    0u16,
                ));
                CopyWindowToVram((windowId).read(), 1u8);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                input = ListMenu_ProcessInput((listMenuId).read());
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    DestroyListMenuTask(
                        (listMenuId).read(),
                        core::ptr::null_mut(),
                        core::ptr::null_mut(),
                    );
                    ClearStdWindowAndFrame((windowId).read(), 1u8);
                    RemoveWindow((windowId).read());
                    (state).write(0u8);
                    return input;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        DestroyListMenuTask(
                            (listMenuId).read(),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                        );
                        ClearStdWindowAndFrame((windowId).read(), 1u8);
                        RemoveWindow((windowId).read());
                        (state).write(0u8);
                        return (-2i32);
                    }
                }
                break 'l1;
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn TradeBoardMenuHandler(
    state: *mut u8,
    mainWindowId: *mut u8,
    listMenuId: *mut u8,
    headerWindowId: *mut u8,
    winTemplate: *mut u8,
    menuTemplate: *mut u8,
    list: *mut u8,
) -> i32 {
    unsafe {
        let mut state = state;
        let mut mainWindowId = mainWindowId;
        let mut listMenuId = listMenuId;
        let mut headerWindowId = headerWindowId;
        let mut winTemplate = winTemplate;
        let mut menuTemplate = menuTemplate;
        let mut list = list;
        let mut input: i32 = 0i32;
        let mut idx: i32 = 0i32;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                (headerWindowId).write(CreateTradeBoardWindow(
                    (&raw const sWindowTemplate_TradingBoardHeader)
                        .cast::<u8>()
                        .cast_mut(),
                ));
                (mainWindowId).write(((AddWindow(winTemplate)) as u8));
                DrawStdWindowFrame((mainWindowId).read(), 0u8);
                (&raw mut gMultiuseListMenuTemplate)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(menuTemplate.cast::<crate::c::Rec4<24>>().read_unaligned());
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
                    .write((mainWindowId).read());
                (listMenuId).write(ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    0u16,
                    1u16,
                ));
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyWindowToVram((mainWindowId).read(), 1u8);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = ListMenu_ProcessInput((listMenuId).read());
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    if (input == 8i32)
                        || (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 2i32)
                            != 0)
                    {
                        DestroyListMenuTask(
                            (listMenuId).read(),
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                        );
                        RemoveWindow((mainWindowId).read());
                        DeleteTradeBoardWindow((headerWindowId).read());
                        (state).write(0u8);
                        return (-2i32);
                    } else {
                        idx = GetIndexOfNthTradeBoardOffer((list).cast::<u8>(), input);
                        if idx >= 0i32 {
                            DestroyListMenuTask(
                                (listMenuId).read(),
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            );
                            RemoveWindow((mainWindowId).read());
                            DeleteTradeBoardWindow((headerWindowId).read());
                            (state).write(0u8);
                            return idx;
                        } else {
                            PlaySE(7u16);
                        }
                    }
                }
                break 'l1;
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn UR_ClearBg0() {
    unsafe {
        FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 32u8, 0u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn JoinGroup_EnableScriptContexts() {
    unsafe {
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn PrintUnionRoomText(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    colorIdx: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut colorIdx = colorIdx;
        let mut printerTemplate = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printerTemplate).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(6)).write(x);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(7)).write(y);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(8)).write(x);
        (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(9)).write(y);
        crate::c::bf_write(
            ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            1,
            1,
            (0u8) as i32,
        );
        'l1: {
            let __sw1 = ((colorIdx) as i32);
            if __sw1 == 0i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (2u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (3u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (4u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (5u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (6u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (7u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (3u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (2u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (3u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (7u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (15u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (9u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(10)).write(0u8);
                (((&raw mut printerTemplate).cast::<u8>()).wrapping_add(11)).write(0u8);
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(12),
                    4,
                    4,
                    (14u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    0,
                    4,
                    (15u8) as i32,
                );
                crate::c::bf_write(
                    ((&raw mut printerTemplate).cast::<u8>()).wrapping_add(13),
                    4,
                    4,
                    (9u8) as i32,
                );
                break 'l1;
            }
        }
        AddTextPrinter((&raw mut printerTemplate).cast::<u8>(), 255u8, None);
    }
}
pub(crate) unsafe extern "C" fn ClearRfuPlayerList(players: *mut u8, count: u8) {
    unsafe {
        let mut players = players;
        let mut count = count;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((players).wrapping_offset((i) as isize * 32))
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sUnionRoomPlayer_DummyRfu)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    (((players).wrapping_offset((i) as isize * 32))
                        .wrapping_add(24)
                        .cast::<u16>())
                    .write(255u16);
                    crate::c::bf_write(
                        ((players).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                        0,
                        2,
                        (0u8) as i32,
                    );
                    crate::c::bf_write(
                        ((players).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                        2,
                        1,
                        (0u8) as i32,
                    );
                    (((players).wrapping_offset((i) as isize * 32)).wrapping_add(27)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearIncomingPlayerList(list: *mut u8, count: u8) {
    unsafe {
        let mut list = list;
        let mut count = count;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((list).cast::<u8>()).wrapping_offset((i) as isize * 28))
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sUnionRoomPlayer_DummyRfu)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    crate::c::bf_write(
                        (((list).cast::<u8>()).wrapping_offset((i) as isize * 28)).wrapping_add(24),
                        0,
                        1,
                        (0u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ArePlayersDifferent(player1: *mut u8, player2: *mut u8) -> u8 {
    unsafe {
        let mut player1 = player1;
        let mut player2 = player2;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((player1).wrapping_add(2)).cast::<u8>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        != ((((((player2).wrapping_add(2)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((((player1).wrapping_add(16)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != ((((((player2).wrapping_add(16)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
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
pub(crate) unsafe extern "C" fn ArePlayerDataDifferent(player1: *mut u8, player2: *mut u8) -> u32 {
    unsafe {
        let mut player1 = player1;
        let mut player2 = player2;
        let mut i: i32 = 0i32;
        if ((crate::c::bf_read((player1).wrapping_add(10), 0, 7, false) as u8) as i32)
            != ((crate::c::bf_read((player2).wrapping_add(10), 0, 7, false) as u8) as i32)
        {
            return 1u32;
        }
        if ((crate::c::bf_read((player1).wrapping_add(10), 7, 1, false) as u8) as i32)
            != ((crate::c::bf_read((player2).wrapping_add(10), 7, 1, false) as u8) as i32)
        {
            return 1u32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((player1).wrapping_add(4)).cast::<u8>()).wrapping_offset((i) as isize))
                        .read()) as i32)
                        != ((((((player2).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((crate::c::bf_read((player1).wrapping_add(8), 0, 10, false) as u16) as i32)
            != ((crate::c::bf_read((player2).wrapping_add(8), 0, 10, false) as u16) as i32)
        {
            return 1u32;
        }
        if ((crate::c::bf_read((player1).wrapping_add(9), 2, 6, false) as u16) as i32)
            != ((crate::c::bf_read((player2).wrapping_add(9), 2, 6, false) as u16) as i32)
        {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetNewIncomingPlayerId(
    player: *mut u8,
    incomingPlayer: *mut u8,
) -> u32 {
    unsafe {
        let mut player = player;
        let mut incomingPlayer = incomingPlayer;
        let mut result: u8 = 255u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        ((incomingPlayer).wrapping_offset((i) as isize * 28)).wrapping_add(24),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0)
                        && (!((ArePlayersDifferent(
                            (player),
                            ((incomingPlayer).wrapping_offset((i) as isize * 28)),
                        )) != 0))
                    {
                        result = ((i) as u8);
                        crate::c::bf_write(
                            ((incomingPlayer).wrapping_offset((i) as isize * 28)).wrapping_add(24),
                            0,
                            1,
                            (0u8) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((result) as u32);
    }
}
pub(crate) unsafe extern "C" fn TryAddIncomingPlayerToList(
    players: *mut u8,
    incomingPlayer: *mut u8,
    max: u8,
) -> u8 {
    unsafe {
        let mut players = players;
        let mut incomingPlayer = incomingPlayer;
        let mut max = max;
        let mut i: i32 = 0i32;
        if (crate::c::bf_read((incomingPlayer).wrapping_add(24), 0, 1, false) as u8) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((max) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if ((crate::c::bf_read(
                            ((players).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                            == 0i32
                        {
                            ((players).wrapping_offset((i) as isize * 32))
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (incomingPlayer)
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            (((players).wrapping_offset((i) as isize * 32))
                                .wrapping_add(24)
                                .cast::<u16>())
                            .write(0u16);
                            crate::c::bf_write(
                                ((players).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                                0,
                                2,
                                (1u8) as i32,
                            );
                            (((players).wrapping_offset((i) as isize * 32)).wrapping_add(27))
                                .write(64u8);
                            crate::c::bf_write(
                                (incomingPlayer).wrapping_add(24),
                                0,
                                1,
                                (0u8) as i32,
                            );
                            return ((i) as u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 255u8;
    }
}
pub(crate) unsafe extern "C" fn PrintGroupMemberOnWindow(
    windowId: u8,
    x: u8,
    y: u8,
    player: *mut u8,
    colorIdx: u8,
    id: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut x = x;
        let mut y = y;
        let mut player = player;
        let mut colorIdx = colorIdx;
        let mut id = id;
        let mut activity: u8 = 0u8;
        let mut trainerId = crate::ffi::Align4([0u8; 6]);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar4).cast::<u8>(),
            ((id) as i32).wrapping_add(1i32),
            2i32,
            2u8,
        );
        StringAppend(
            (&raw mut gStringVar4).cast::<u8>(),
            ((&raw const sText_Colon).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        PrintUnionRoomText(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x,
            y,
            0u8,
        );
        x = ((((x) as i32).wrapping_add(18i32)) as u8);
        activity = (crate::c::bf_read((player).wrapping_add(10), 0, 7, false) as u8);
        if (((crate::c::bf_read((player).wrapping_add(26), 0, 2, false) as u8) as i32) == 1i32)
            && (!((((activity) as i32) & 64i32) != 0))
        {
            CopyAndTranslatePlayerName((&raw mut gStringVar4).cast::<u8>(), player);
            PrintUnionRoomText(
                windowId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                x,
                y,
                colorIdx,
            );
            ConvertIntToDecimalStringN(
                (&raw mut trainerId).cast::<u8>(),
                ((((((player).wrapping_add(2)).cast::<u8>()).read()) as i32)
                    | (((((((player).wrapping_add(2)).cast::<u8>()).wrapping_offset(1)).read())
                        as i32)
                        << 8)),
                2i32,
                5u8,
            );
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                ((&raw const sText_ID).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            StringAppend(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut trainerId).cast::<u8>(),
            );
            PrintUnionRoomText(
                windowId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 136i32))
                    as u8),
                y,
                colorIdx,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintGroupCandidateOnWindow(
    windowId: u8,
    x: u8,
    y: u8,
    player: *mut u8,
    colorIdx: u8,
    id: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut x = x;
        let mut y = y;
        let mut player = player;
        let mut colorIdx = colorIdx;
        let mut id = id;
        let mut trainerId = crate::ffi::Align4([0u8; 6]);
        if ((crate::c::bf_read((player).wrapping_add(26), 0, 2, false) as u8) as i32) == 1i32 {
            CopyAndTranslatePlayerName((&raw mut gStringVar4).cast::<u8>(), player);
            PrintUnionRoomText(
                windowId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                x,
                y,
                colorIdx,
            );
            ConvertIntToDecimalStringN(
                (&raw mut trainerId).cast::<u8>(),
                ((((((player).wrapping_add(2)).cast::<u8>()).read()) as i32)
                    | (((((((player).wrapping_add(2)).cast::<u8>()).wrapping_offset(1)).read())
                        as i32)
                        << 8)),
                2i32,
                5u8,
            );
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                ((&raw const sText_ID).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            StringAppend(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut trainerId).cast::<u8>(),
            );
            PrintUnionRoomText(
                windowId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 104i32))
                    as u8),
                y,
                colorIdx,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsPlayerFacingTradingBoard() -> u32 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        if ((x) as i32) != 9i32 {
            return 0u32;
        }
        if ((y) as i32) != 8i32 {
            return 0u32;
        }
        if ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 2i32)
            || ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 0i32)
        {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetResponseIdx_InviteToURoomActivity(activity: i32) -> u32 {
    unsafe {
        let mut activity = activity;
        'l1: {
            let __sw1 = activity;
            let __matched = __sw1 == 5i32 || __sw1 == 4i32 || __sw1 == 8i32 || __sw1 == 3i32;
            if __sw1 == 5i32 {
                return 1u32;
            }
            if __sw1 == 4i32 {
                return 2u32;
            }
            if __sw1 == 8i32 {
                return 3u32;
            }
            if __sw1 == 3i32 || !__matched {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ConvPartnerUnameAndGetWhetherMetAlready(player: *mut u8) -> u32 {
    unsafe {
        let mut player = player;
        let mut name = crate::ffi::Align4([0u8; 30]);
        CopyAndTranslatePlayerName((&raw mut name).cast::<u8>(), player);
        return PlayerHasMetTrainerBefore(
            ReadAsU16(((player).wrapping_add(2)).cast::<u8>()),
            (&raw mut name).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn UnionRoomGetPlayerInteractionResponse(
    list: *mut u8,
    overrideGender: u8,
    playerIdx: u8,
    playerGender: u32,
) -> i32 {
    unsafe {
        let mut list = list;
        let mut overrideGender = overrideGender;
        let mut playerIdx = playerIdx;
        let mut playerGender = playerGender;
        let mut metBefore: u32 = 0u32;
        let mut player: *mut u8 =
            ((list).cast::<u8>()).wrapping_offset(((playerIdx) as i32) as isize * 32);
        if (!((crate::c::bf_read((player).wrapping_add(10), 7, 1, false) as u8) != 0))
            && (!((overrideGender) != 0))
        {
            CopyAndTranslatePlayerName((&raw mut gStringVar1).cast::<u8>(), player);
            metBefore = PlayerHasMetTrainerBefore(
                ReadAsU16(((player).wrapping_add(2)).cast::<u8>()),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            if ((crate::c::bf_read((player).wrapping_add(10), 0, 7, false) as u8) as i32) == 69i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((((&raw const sJoinChatTexts).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((metBefore) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .wrapping_offset(((playerGender) as i32) as isize))
                    .read(),
                );
                return 2i32;
            } else {
                UR_PrintFieldMessage(
                    ((((&raw const sCommunicatingWaitTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((metBefore) as i32) as isize))
                    .read(),
                );
                return 1i32;
            }
        } else {
            CopyAndTranslatePlayerName((&raw mut gStringVar1).cast::<u8>(), player);
            if (overrideGender) != 0 {
                playerGender = (((((((((player).wrapping_add(2)).cast::<u8>())
                    .wrapping_offset((((overrideGender) as i32).wrapping_add(1i32)) as isize))
                .read()) as i32)
                    >> 3)
                    & 1i32) as u32);
            }
            'l1: {
                let __sw1 = (((crate::c::bf_read((player).wrapping_add(10), 0, 7, false) as u8)
                    as i32)
                    & 63i32);
                let __matched = __sw1 == 1i32 || __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 8i32;
                if __sw1 == 1i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((((&raw const sBattleReactionTexts).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((playerGender) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((crate::c::rem_u32(
                                ((Random()) as u32),
                                crate::c::div_u32(16u32, 4u32),
                            )) as i32) as isize,
                        ))
                        .read(),
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((((&raw const sTradeReactionTexts).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((playerGender) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .wrapping_offset((crate::c::rem_i32(((Random()) as i32), 2i32)) as isize))
                        .read(),
                    );
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((((&raw const sChatReactionTexts).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((playerGender) as i32) as isize * 16))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((crate::c::rem_u32(
                                ((Random()) as u32),
                                crate::c::div_u32(16u32, 4u32),
                            )) as i32) as isize,
                        ))
                        .read(),
                    );
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((((&raw const sTrainerCardReactionTexts)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((playerGender) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(8u32, 4u32)))
                                as i32) as isize,
                        ))
                        .read(),
                    );
                    break 'l1;
                }
                if !__matched {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((&raw const sText_TrainerAppearsBusy)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    break 'l1;
                }
            }
            return 0i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn ItemPrintFunc_EmptyList(windowId: u8, itemId: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemId = itemId;
        let mut y = y;
    }
}
pub(crate) unsafe extern "C" fn TradeBoardPrintItemInfo(
    windowId: u8,
    y: u8,
    data: *mut u8,
    playerName: *mut u8,
    colorIdx: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut y = y;
        let mut data = data;
        let mut playerName = playerName;
        let mut colorIdx = colorIdx;
        let mut levelStr = crate::ffi::Align4([0u8; 4]);
        let mut species: u16 = (crate::c::bf_read((data).wrapping_add(8), 0, 10, false) as u16);
        let mut r#type: u8 =
            ((crate::c::bf_read((data).wrapping_add(9), 2, 6, false) as u16) as u8);
        let mut level: u8 = (crate::c::bf_read((data).wrapping_add(11), 1, 7, false) as u8);
        PrintUnionRoomText(windowId, 1u8, playerName, 8u8, y, colorIdx);
        if ((species) as i32) == 412i32 {
            PrintUnionRoomText(
                windowId,
                1u8,
                ((&raw const sText_EggTrade).cast::<u8>().cast_mut()).cast::<u8>(),
                68u8,
                y,
                colorIdx,
            );
        } else {
            BlitMenuInfoIcon(
                windowId,
                ((((r#type) as i32).wrapping_add(1i32)) as u8),
                68u16,
                ((y) as u16),
            );
            PrintUnionRoomText(
                windowId,
                1u8,
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
                118u8,
                y,
                colorIdx,
            );
            ConvertIntToDecimalStringN(
                (&raw mut levelStr).cast::<u8>(),
                ((level) as i32),
                1i32,
                3u8,
            );
            PrintUnionRoomText(
                windowId,
                1u8,
                (&raw mut levelStr).cast::<u8>(),
                198u8,
                y,
                colorIdx,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TradeBoardListMenuItemPrintFunc(windowId: u8, itemId: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemId = itemId;
        let mut y = y;
        let mut leader: *mut u8 =
            (((&raw mut sWirelessLinkMain).cast::<u8>()).cast::<*mut u8>()).read();
        let mut gameData: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut playerName = crate::ffi::Align4([0u8; 9]);
        if (itemId == 4294967293u32)
            && (((y) as i32)
                == ((crate::c::bf_read(
                    ((&raw const sTradeBoardListMenuTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .wrapping_add(20),
                    0,
                    4,
                    false,
                ) as u8) as i32))
        {
            gameData = GetHostRfuGameData();
            if ((crate::c::bf_read((gameData).wrapping_add(8), 0, 10, false) as u16) as i32) != 0i32
            {
                TradeBoardPrintItemInfo(
                    windowId,
                    y,
                    gameData,
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    5u8,
                );
            }
        } else {
            j = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((crate::c::bf_read(
                            (((((leader).cast::<*mut u8>()).read()).cast::<u8>())
                                .wrapping_offset((i) as isize * 32))
                            .wrapping_add(26),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                            == 1i32)
                            && (((crate::c::bf_read(
                                (((((leader).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 32))
                                .wrapping_add(8),
                                0,
                                10,
                                false,
                            ) as u16) as i32)
                                != 0i32)
                        {
                            j = (j).wrapping_add(1);
                        }
                        if ((j) as u32) == (itemId).wrapping_add(1u32) {
                            CopyAndTranslatePlayerName(
                                (&raw mut playerName).cast::<u8>(),
                                ((((leader).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 32),
                            );
                            TradeBoardPrintItemInfo(
                                windowId,
                                y,
                                (((((leader).cast::<*mut u8>()).read()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 32)),
                                (&raw mut playerName).cast::<u8>(),
                                6u8,
                            );
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetIndexOfNthTradeBoardOffer(players: *mut u8, n: i32) -> i32 {
    unsafe {
        let mut players = players;
        let mut n = n;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (((crate::c::bf_read(
                        ((players).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 1i32)
                        && (((crate::c::bf_read(
                            ((players).wrapping_offset((i) as isize * 32)).wrapping_add(8),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            != 0i32)
                    {
                        j = (j).wrapping_add(1);
                    }
                    if j == (n).wrapping_add(1i32) {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerGender(playerIdx: i32, list: *mut u8) -> i32 {
    unsafe {
        let mut playerIdx = playerIdx;
        let mut list = list;
        return ((crate::c::bf_read(
            (((list).cast::<u8>()).wrapping_offset((playerIdx) as isize * 32)).wrapping_add(11),
            0,
            1,
            false,
        ) as u8) as i32);
    }
}
pub(crate) unsafe extern "C" fn IsRequestedTradeInPlayerParty(r#type: u32, species: u32) -> i32 {
    unsafe {
        let mut r#type = r#type;
        let mut species = species;
        let mut i: i32 = 0i32;
        if species == 412u32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        species = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        );
                        if species == 412u32 {
                            return 0i32;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return 2i32;
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        species = GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        );
                        if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .read()) as u32)
                            == r#type)
                            || (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 28))
                            .wrapping_add(6))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as u32)
                                == r#type)
                        {
                            return 0i32;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return 1i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetURoomActivityRejectMsg(
    dst: *mut u8,
    acitivty: i32,
    playerGender: u32,
) {
    unsafe {
        let mut dst = dst;
        let mut acitivty = acitivty;
        let mut playerGender = playerGender;
        'l1: {
            let __sw1 = acitivty;
            if __sw1 == 65i32 {
                StringExpandPlaceholders(
                    dst,
                    ((((&raw const sBattleDeclinedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((playerGender) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 69i32 {
                StringExpandPlaceholders(
                    dst,
                    ((((&raw const sChatDeclinedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((playerGender) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 68i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_TradeOfferRejected)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 72i32 {
                StringExpandPlaceholders(
                    dst,
                    ((((&raw const sShowTrainerCardDeclinedTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((playerGender) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetURoomActivityStartMsg(dst: *mut u8, acitivty: u8) {
    unsafe {
        let mut dst = dst;
        let mut acitivty = acitivty;
        let mut mpId: u8 = GetMultiplayerId();
        let mut gender: u8 = ((((&raw mut gLinkPlayers).cast::<u8>())
            .wrapping_offset((((mpId) as i32) ^ 1i32) as isize * 28))
        .wrapping_add(19))
        .read();
        'l1: {
            let __sw1 = ((acitivty) as i32);
            if __sw1 == 65i32 {
                StringCopy(
                    dst,
                    (((((((&raw const sStartActivityTexts).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((mpId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(((gender) as i32) as isize * 12))
                    .cast::<*mut u8>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 68i32 {
                StringCopy(
                    dst,
                    ((((((((&raw const sStartActivityTexts).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((mpId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(((gender) as i32) as isize * 12))
                    .cast::<*mut u8>())
                    .wrapping_offset(2))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 69i32 {
                StringCopy(
                    dst,
                    ((((((((&raw const sStartActivityTexts).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((mpId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(((gender) as i32) as isize * 12))
                    .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetChatLeaderActionRequestMessage(
    dst: *mut u8,
    gender: u32,
    activityData: *mut u16,
    uroom: *mut u8,
) -> i32 {
    unsafe {
        let mut dst = dst;
        let mut gender = gender;
        let mut activityData = activityData;
        let mut uroom = uroom;
        let mut result: i32 = 0i32;
        let mut species: u16 = 0u16;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((activityData).read()) as i32);
            if __sw1 == 65i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_BattleChallenge).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                result = 1i32;
                break 'l1;
            }
            if __sw1 == 69i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_ChatInvitation).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                result = 1i32;
                break 'l1;
            }
            if __sw1 == 68i32 {
                ConvertIntToDecimalStringN(
                    (((uroom).wrapping_add(88)).cast::<u8>()).cast::<u8>(),
                    (((((&raw mut sUnionRoomTrade).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read()) as i32),
                    0i32,
                    3u8,
                );
                StringCopy(
                    ((((uroom).wrapping_add(88)).cast::<u8>()).wrapping_offset(16)).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((&raw mut sUnionRoomTrade).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read()) as i32)
                                == 2i32
                            {
                                ConvertIntToDecimalStringN(
                                    ((((uroom).wrapping_add(88)).cast::<u8>()).wrapping_offset(32))
                                        .cast::<u8>(),
                                    ((((activityData).wrapping_offset(2)).read()) as i32),
                                    0i32,
                                    3u8,
                                );
                                StringCopy(
                                    ((((uroom).wrapping_add(88)).cast::<u8>()).wrapping_offset(48))
                                        .cast::<u8>(),
                                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                                        ((((activityData).wrapping_offset(1)).read()) as i32)
                                            as isize
                                            * 11,
                                    ))
                                    .cast::<u8>(),
                                );
                                species = ((activityData).wrapping_offset(1)).read();
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((species) as i32) == 412i32 {
                    StringCopy(
                        dst,
                        ((&raw const sText_OfferToTradeEgg).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                } else {
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                                    ((i) as u8),
                                    ((((uroom).wrapping_add(88)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<u8>(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        dst,
                        ((&raw const sText_OfferToTradeMon).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                }
                result = 1i32;
                break 'l1;
            }
            if __sw1 == 72i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_ShowTrainerCard).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                result = 1i32;
                break 'l1;
            }
            if __sw1 == 64i32 {
                StringExpandPlaceholders(
                    dst,
                    ((&raw const sText_ChatDropped).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                result = 2i32;
                break 'l1;
            }
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn PollPartnerYesNoResponse(data: *mut u8) -> u32 {
    unsafe {
        let mut data = data;
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).read())
            as i32)
            != 0i32
        {
            if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).read())
                as i32)
                == 81i32
            {
                ((data).wrapping_add(152).cast::<u16>()).write(81u16);
                return 1u32;
            } else {
                if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1))
                    .read()) as i32)
                    == 82i32
                {
                    ((data).wrapping_add(152).cast::<u16>()).write(82u16);
                    return 1u32;
                }
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InUnionRoom() -> u32 {
    unsafe {
        return ((if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 25i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 60i32)
        {
            1i32
        } else {
            0i32
        }) as u32);
    }
}
pub(crate) unsafe extern "C" fn HasAtLeastTwoMonsOfLevel30OrLower() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        56i32,
                    ) <= 30u32)
                        && (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100),
                            65i32,
                        ) != 412u32)
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count > 1i32 {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ResetUnionRoomTrade(trade: *mut u8) {
    unsafe {
        let mut trade = trade;
        ((trade).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(2).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(4).cast::<u32>()).write(0u32);
        ((trade).wrapping_add(10).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(12).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(14).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(16).cast::<u16>()).write(0u16);
        ((trade).wrapping_add(20).cast::<u32>()).write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_ResetUnionRoomTrade() {
    unsafe {
        ResetUnionRoomTrade((&raw mut sUnionRoomTrade).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RegisterTradeMonAndGetIsEgg(monId: u32, trade: *mut u8) -> u32 {
    unsafe {
        let mut monId = monId;
        let mut trade = trade;
        ((trade).wrapping_add(10).cast::<u16>()).write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                65i32,
            )) as u16),
        );
        ((trade).wrapping_add(12).cast::<u16>()).write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                56i32,
            )) as u16),
        );
        ((trade).wrapping_add(4).cast::<u32>()).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((monId) as i32) as isize * 100),
            0i32,
        ));
        if ((((trade).wrapping_add(10).cast::<u16>()).read()) as i32) == 412i32 {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn RegisterTradeMon(monId: u32, trade: *mut u8) {
    unsafe {
        let mut monId = monId;
        let mut trade = trade;
        ((trade).wrapping_add(14).cast::<u16>()).write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                65i32,
            )) as u16),
        );
        ((trade).wrapping_add(16).cast::<u16>()).write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                56i32,
            )) as u16),
        );
        ((trade).wrapping_add(20).cast::<u32>()).write(GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((monId) as i32) as isize * 100),
            0i32,
        ));
    }
}
pub(crate) unsafe extern "C" fn GetPartyPositionOfRegisteredMon(
    trade: *mut u8,
    multiplayerId: u8,
) -> u32 {
    unsafe {
        let mut trade = trade;
        let mut multiplayerId = multiplayerId;
        let mut response: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut cur_personality: u32 = 0u32;
        let mut cur_species: u16 = 0u16;
        let mut i: i32 = 0i32;
        if ((multiplayerId) as i32) == 0i32 {
            species = ((trade).wrapping_add(10).cast::<u16>()).read();
            personality = ((trade).wrapping_add(4).cast::<u32>()).read();
        } else {
            species = ((trade).wrapping_add(14).cast::<u16>()).read();
            personality = ((trade).wrapping_add(20).cast::<u32>()).read();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    cur_personality = GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        0i32,
                    );
                    if cur_personality != personality {
                        break 'l2;
                    }
                    cur_species = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset((i) as isize * 100),
                        65i32,
                    )) as u16);
                    if ((cur_species) as i32) != ((species) as i32) {
                        break 'l2;
                    }
                    response = ((i) as u16);
                    break 'l1;
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((response) as u32);
    }
}
pub(crate) unsafe extern "C" fn HandleCancelActivity(setData: u32) {
    unsafe {
        let mut setData = setData;
        UR_ClearBg0();
        UnlockPlayerFieldControls();
        UnionRoom_UnlockPlayerAndChatPartner();
        ((&raw mut gPlayerCurrActivity).cast::<u8>().cast::<u8>()).write(0u8);
        if (setData) != 0 {
            SetTradeBoardRegisteredMonInfo(
                (((((&raw mut sUnionRoomTrade).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as u32),
                (((((&raw mut sUnionRoomTrade).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as u32),
                (((((&raw mut sUnionRoomTrade).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<u16>())
                .read()) as u32),
            );
            UpdateGameData_SetActivity(64u8, 0u32, 0u32);
        }
    }
}
pub(crate) unsafe extern "C" fn StartScriptInteraction() {
    unsafe {
        LockPlayerFieldControls();
        FreezeObjects_WaitForPlayer();
    }
}
pub(crate) unsafe extern "C" fn GetActivePartnersInfo(data: *mut u8) -> u8 {
    unsafe {
        let mut data = data;
        let mut retVal: u8 = 128u8;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::bf_read(
                        (((((data).wrapping_add(12).cast::<*mut u8>()).read()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(24),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        retVal = ((((retVal) as i32)
                            | (((crate::c::bf_read(
                                (((((data).wrapping_add(12).cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(11),
                                0,
                                1,
                                false,
                            ) as u8) as i32)
                                << 3)) as u8);
                        retVal = ((((retVal) as i32)
                            | ((((((((((data).wrapping_add(12).cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(2))
                            .cast::<u8>())
                            .read()) as i32)
                                & 7i32)) as u8);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn ViewURoomPartnerTrainerCard(
    unused: *mut u8,
    data: *mut u8,
    isParent: u8,
) {
    unsafe {
        let mut unused = unused;
        let mut data = data;
        let mut isParent = isParent;
        let mut trainerCard: *mut u8 = ((&raw mut gTrainerCards).cast::<u8>())
            .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 100);
        let mut i: i32 = 0i32;
        let mut n: i32 = 0i32;
        DynamicPlaceholderTextUtil_Reset();
        StringCopy(
            (((data).wrapping_add(192)).cast::<u8>()).cast::<u8>(),
            (((&raw mut gTrainerClassNames).cast::<u8>())
                .wrapping_offset(((GetUnionRoomTrainerClass()) as i32) as isize * 13))
            .cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            (((data).wrapping_add(192)).cast::<u8>()).cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            1u8,
            ((trainerCard).wrapping_add(48)).cast::<u8>(),
        );
        StringCopy(
            ((data).wrapping_add(372)).cast::<u8>(),
            ((((&raw const sCardColorTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((((trainerCard).wrapping_add(1)).read()) as i32) as isize))
            .read(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(2u8, ((data).wrapping_add(372)).cast::<u8>());
        ConvertIntToDecimalStringN(
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(30)).cast::<u8>(),
            ((((trainerCard).wrapping_add(12).cast::<u16>()).read()) as i32),
            0i32,
            3u8,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            3u8,
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(30)).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(45)).cast::<u8>(),
            ((((trainerCard).wrapping_add(16).cast::<u16>()).read()) as i32),
            0i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(60)).cast::<u8>(),
            ((((trainerCard).wrapping_add(18).cast::<u16>()).read()) as i32),
            2i32,
            2u8,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            4u8,
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(45)).cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            5u8,
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(60)).cast::<u8>(),
        );
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((data).wrapping_add(420)).cast::<u8>(),
            ((&raw const sText_TrainerCardInfoPage1)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar4).cast::<u8>(),
            ((data).wrapping_add(420)).cast::<u8>(),
        );
        n = ((((trainerCard).wrapping_add(20).cast::<u16>()).read()) as i32);
        if n > 9999i32 {
            n = 9999i32;
        }
        ConvertIntToDecimalStringN(
            (((data).wrapping_add(192)).cast::<u8>()).cast::<u8>(),
            n,
            0i32,
            4u8,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            0u8,
            (((data).wrapping_add(192)).cast::<u8>()).cast::<u8>(),
        );
        n = ((((trainerCard).wrapping_add(22).cast::<u16>()).read()) as i32);
        if n > 9999i32 {
            n = 9999i32;
        }
        ConvertIntToDecimalStringN(
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(15)).cast::<u8>(),
            n,
            0i32,
            4u8,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            2u8,
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(15)).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(30)).cast::<u8>(),
            ((((trainerCard).wrapping_add(32).cast::<u16>()).read()) as i32),
            0i32,
            5u8,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(
            3u8,
            ((((data).wrapping_add(192)).cast::<u8>()).wrapping_offset(30)).cast::<u8>(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    CopyEasyChatWord(
                        ((((data).wrapping_add(192)).cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(3i32)) as isize * 15))
                        .cast::<u8>(),
                        ((((trainerCard).wrapping_add(40)).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                        (((i).wrapping_add(4i32)) as u8),
                        ((((data).wrapping_add(192)).cast::<u8>())
                            .wrapping_offset(((i).wrapping_add(3i32)) as isize * 15))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DynamicPlaceholderTextUtil_ExpandPlaceholders(
            ((data).wrapping_add(420)).cast::<u8>(),
            ((&raw const sText_TrainerCardInfoPage2)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        StringAppend(
            (&raw mut gStringVar4).cast::<u8>(),
            ((data).wrapping_add(420)).cast::<u8>(),
        );
        if ((isParent) as i32) == 1i32 {
            DynamicPlaceholderTextUtil_ExpandPlaceholders(
                ((data).wrapping_add(420)).cast::<u8>(),
                ((&raw const sText_FinishedCheckingPlayersTrainerCard)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            StringAppend(
                (&raw mut gStringVar4).cast::<u8>(),
                ((data).wrapping_add(420)).cast::<u8>(),
            );
        } else {
            if ((isParent) as i32) == 0i32 {
                DynamicPlaceholderTextUtil_ExpandPlaceholders(
                    ((data).wrapping_add(420)).cast::<u8>(),
                    ((((&raw const sGladToMeetYouTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((((trainerCard).read()) as i32) as isize))
                    .read(),
                );
                StringAppend(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((data).wrapping_add(420)).cast::<u8>(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CopyAndTranslatePlayerName(dest: *mut u8, player: *mut u8) {
    unsafe {
        let mut dest = dest;
        let mut player = player;
        StringCopy_PlayerName(dest, ((player).wrapping_add(16)).cast::<u8>());
        ConvertInternationalString(
            dest,
            ((crate::c::bf_read((player).wrapping_add(0), 0, 4, false) as u16) as u8),
        );
    }
}
