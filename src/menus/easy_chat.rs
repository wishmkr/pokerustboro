//! Translated from `src/easy_chat.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sQuizLadyEasyChatScreens sEasyChatScreenTemplates sAlphabetGroupIdMap sMysteryGiftPhrase sBerryMasterWifePhrases sTriangleCursor_Pal sTriangleCursor_Gfx sScrollIndicator_Gfx sStartSelectButtons_Gfx sRSInterviewFrame_Pal sRSInterviewFrame_Gfx sTextInputFrameOrange_Pal sTextInputFrameGreen_Pal sTextInputFrame_Gfx sTitleText_Pal sText_Pal sPhraseFrameDimensions sEasyChatBgTemplates sEasyChatWindowTemplates sEasyChatYesNoWindowTemplate sText_Clear17 sEasyChatKeyboardAlphabet sSpriteSheets sSpritePalettes sCompressedSpriteSheets sAlphabetKeyboardColumnOffsets sOamData_TriangleCursor sSpriteTemplate_TriangleCursor sOamData_RectangleCursor sAnim_RectangleCursor_OnGroup sAnim_RectangleCursor_OnButton sAnim_RectangleCursor_OnOthers sAnim_RectangleCursor_OnLetter sAnims_RectangleCursor sSpriteTemplate_RectangleCursor sOamData_ModeWindow sAnim_ModeWindow_Hidden sAnim_ModeWindow_ToGroup sAnim_ModeWindow_ToAlphabet sAnim_ModeWindow_ToHidden sAnim_ModeWindow_Transition sAnims_ModeWindow sSpriteTemplate_ModeWindow sOamData_ButtonWindow sSpriteTemplate_ButtonWindow sOamData_StartSelectButton sOamData_ScrollIndicator sAnim_Frame0 sAnim_Frame1 sAnims_TwoFrame sSpriteTemplate_StartSelectButton sSpriteTemplate_ScrollIndicator sFooterOptionXOffsets sFooterTextOptions gEasyChatGroup_Pokemon gEasyChatWord_IChooseYou gEasyChatWord_Gotcha gEasyChatWord_Trade gEasyChatWord_Sapphire gEasyChatWord_Evolve gEasyChatWord_Encyclopedia gEasyChatWord_Nature gEasyChatWord_Center gEasyChatWord_Egg gEasyChatWord_Link gEasyChatWord_SpAbility gEasyChatWord_Trainer gEasyChatWord_Version gEasyChatWord_Pokenav gEasyChatWord_Pokemon gEasyChatWord_Get gEasyChatWord_Pokedex gEasyChatWord_Ruby gEasyChatWord_Level gEasyChatWord_Red gEasyChatWord_Green gEasyChatWord_Bag gEasyChatWord_Flame gEasyChatWord_Gold gEasyChatWord_Leaf gEasyChatWord_Silver gEasyChatWord_Emerald gEasyChatGroup_Trainer gEasyChatWord_Dark gEasyChatWord_Stench gEasyChatWord_ThickFat gEasyChatWord_RainDish gEasyChatWord_Drizzle gEasyChatWord_ArenaTrap gEasyChatWord_Intimidate gEasyChatWord_RockHead gEasyChatWord_Color gEasyChatWord_AltColor gEasyChatWord_Rock gEasyChatWord_Beautiful gEasyChatWord_Beauty gEasyChatWord_AirLock gEasyChatWord_Psychic gEasyChatWord_HyperCutter gEasyChatWord_Fighting gEasyChatWord_ShadowTag gEasyChatWord_Smart gEasyChatWord_Smartness gEasyChatWord_SpeedBoost gEasyChatWord_Cool gEasyChatWord_Coolness gEasyChatWord_BattleArmor gEasyChatWord_Cute gEasyChatWord_Cuteness gEasyChatWord_Sturdy gEasyChatWord_SuctionCups gEasyChatWord_Grass gEasyChatWord_ClearBody gEasyChatWord_Torrent gEasyChatWord_Ghost gEasyChatWord_Ice gEasyChatWord_Guts gEasyChatWord_RoughSkin gEasyChatWord_ShellArmor gEasyChatWord_NaturalCure gEasyChatWord_Damp gEasyChatWord_Ground gEasyChatWord_Limber gEasyChatWord_MagnetPull gEasyChatWord_WhiteSmoke gEasyChatWord_Synchronize gEasyChatWord_Overgrow gEasyChatWord_SwiftSwim gEasyChatWord_SandStream gEasyChatWord_SandVeil gEasyChatWord_KeenEye gEasyChatWord_InnerFocus gEasyChatWord_Static gEasyChatWord_Type gEasyChatWord_Tough gEasyChatWord_Toughness gEasyChatWord_ShedSkin gEasyChatWord_HugePower gEasyChatWord_VoltAbsorb gEasyChatWord_WaterAbsorb gEasyChatWord_Electric gEasyChatWord_Forecast gEasyChatWord_SereneGrace gEasyChatWord_Poison gEasyChatWord_PoisonPoint gEasyChatWord_Dragon gEasyChatWord_Trace gEasyChatWord_Oblivious gEasyChatWord_Truant gEasyChatWord_RunAway gEasyChatWord_StickyHold gEasyChatWord_CloudNine gEasyChatWord_Normal gEasyChatWord_Steel gEasyChatWord_Illuminate gEasyChatWord_EarlyBird gEasyChatWord_Hustle gEasyChatWord_Shine gEasyChatWord_Flying gEasyChatWord_Drought gEasyChatWord_Lightningrod gEasyChatWord_Compoundeyes gEasyChatWord_MarvelScale gEasyChatWord_WonderGuard gEasyChatWord_Insomnia gEasyChatWord_Levitate gEasyChatWord_Plus gEasyChatWord_Pressure gEasyChatWord_LiquidOoze gEasyChatWord_ColorChange gEasyChatWord_Soundproof gEasyChatWord_EffectSpore gEasyChatWord_Pkrs gEasyChatWord_Fire gEasyChatWord_FlameBody gEasyChatWord_Minus gEasyChatWord_OwnTempo gEasyChatWord_MagmaArmor gEasyChatWord_Water gEasyChatWord_WaterVeil gEasyChatWord_Bug gEasyChatWord_Swarm gEasyChatWord_CuteCharm gEasyChatWord_Immunity gEasyChatWord_Blaze gEasyChatWord_Pickup gEasyChatWord_Pattern gEasyChatWord_FlashFire gEasyChatWord_VitalSpirit gEasyChatWord_Chlorophyll gEasyChatWord_PurePower gEasyChatWord_ShieldDust gEasyChatGroup_Status gEasyChatWord_MatchUp gEasyChatWord_Go gEasyChatWord_No1 gEasyChatWord_Decide gEasyChatWord_LetMeWin gEasyChatWord_Wins gEasyChatWord_Win gEasyChatWord_Won gEasyChatWord_IfIWin gEasyChatWord_WhenIWin gEasyChatWord_CantWin gEasyChatWord_CanWin gEasyChatWord_NoMatch gEasyChatWord_Spirit gEasyChatWord_Decided gEasyChatWord_TrumpCard gEasyChatWord_TakeThat gEasyChatWord_ComeOn gEasyChatWord_Attack gEasyChatWord_Surrender gEasyChatWord_Gutsy gEasyChatWord_Talent gEasyChatWord_Strategy gEasyChatWord_Smite gEasyChatWord_Match gEasyChatWord_Victory gEasyChatWord_Offensive gEasyChatWord_Sense gEasyChatWord_Versus gEasyChatWord_Fights gEasyChatWord_Power gEasyChatWord_Challenge gEasyChatWord_Strong gEasyChatWord_TooStrong gEasyChatWord_GoEasy gEasyChatWord_Foe gEasyChatWord_Genius gEasyChatWord_Legend gEasyChatWord_Escape gEasyChatWord_Aim gEasyChatWord_Battle gEasyChatWord_Fight gEasyChatWord_Resuscitate gEasyChatWord_Points gEasyChatWord_Serious gEasyChatWord_GiveUp gEasyChatWord_Loss gEasyChatWord_IfILose gEasyChatWord_Lost gEasyChatWord_Lose gEasyChatWord_Guard gEasyChatWord_Partner gEasyChatWord_Reject gEasyChatWord_Accept gEasyChatWord_Invincible gEasyChatWord_Received gEasyChatWord_Easy gEasyChatWord_Weak gEasyChatWord_TooWeak gEasyChatWord_Pushover gEasyChatWord_Leader gEasyChatWord_Rule gEasyChatWord_Move gEasyChatGroup_Battle gEasyChatWord_Thanks gEasyChatWord_Yes gEasyChatWord_HereGoes gEasyChatWord_HereICome gEasyChatWord_HereItIs gEasyChatWord_Yeah gEasyChatWord_Welcome gEasyChatWord_Oi gEasyChatWord_HowDo gEasyChatWord_Congrats gEasyChatWord_GiveMe gEasyChatWord_Sorry gEasyChatWord_Apologize gEasyChatWord_Forgive gEasyChatWord_HeyThere gEasyChatWord_Hello gEasyChatWord_GoodBye gEasyChatWord_ThankYou gEasyChatWord_IveArrived gEasyChatWord_Pardon gEasyChatWord_Excuse gEasyChatWord_SeeYa gEasyChatWord_ExcuseMe gEasyChatWord_WellThen gEasyChatWord_GoAhead gEasyChatWord_Appreciate gEasyChatWord_HeyQues gEasyChatWord_WhatsUpQues gEasyChatWord_HuhQues gEasyChatWord_No gEasyChatWord_Hi gEasyChatWord_YeahYeah gEasyChatWord_ByeBye gEasyChatWord_MeetYou gEasyChatWord_Hey gEasyChatWord_Smell gEasyChatWord_Listening gEasyChatWord_HooHah gEasyChatWord_Yahoo gEasyChatWord_Yo gEasyChatWord_ComeOver gEasyChatWord_CountOn gEasyChatGroup_Greetings gEasyChatWord_Opponent gEasyChatWord_I gEasyChatWord_You gEasyChatWord_Yours gEasyChatWord_Son gEasyChatWord_Your gEasyChatWord_Youre gEasyChatWord_Youve gEasyChatWord_Mother gEasyChatWord_Grandfather gEasyChatWord_Uncle gEasyChatWord_Father gEasyChatWord_Boy gEasyChatWord_Adult gEasyChatWord_Brother gEasyChatWord_Sister gEasyChatWord_Grandmother gEasyChatWord_Aunt gEasyChatWord_Parent gEasyChatWord_Man gEasyChatWord_Me gEasyChatWord_Girl gEasyChatWord_Babe gEasyChatWord_Family gEasyChatWord_Her gEasyChatWord_Him gEasyChatWord_He gEasyChatWord_Place gEasyChatWord_Daughter gEasyChatWord_His gEasyChatWord_Hes gEasyChatWord_Arent gEasyChatWord_Siblings gEasyChatWord_Kid gEasyChatWord_Children gEasyChatWord_Mr gEasyChatWord_Mrs gEasyChatWord_Myself gEasyChatWord_IWas gEasyChatWord_ToMe gEasyChatWord_My gEasyChatWord_IAm gEasyChatWord_Ive gEasyChatWord_Who gEasyChatWord_Someone gEasyChatWord_WhoWas gEasyChatWord_ToWhom gEasyChatWord_Whose gEasyChatWord_WhoIs gEasyChatWord_Its gEasyChatWord_Lady gEasyChatWord_Friend gEasyChatWord_Ally gEasyChatWord_Person gEasyChatWord_Dude gEasyChatWord_They gEasyChatWord_TheyWere gEasyChatWord_ToThem gEasyChatWord_Their gEasyChatWord_Theyre gEasyChatWord_Theyve gEasyChatWord_We gEasyChatWord_Been gEasyChatWord_ToUs gEasyChatWord_Our gEasyChatWord_WeRe gEasyChatWord_Rival gEasyChatWord_Weve gEasyChatWord_Woman gEasyChatWord_She gEasyChatWord_SheWas gEasyChatWord_ToHer gEasyChatWord_Hers gEasyChatWord_SheIs gEasyChatWord_Some gEasyChatGroup_People gEasyChatWord_Excl gEasyChatWord_ExclExcl gEasyChatWord_QuesExcl gEasyChatWord_Ques gEasyChatWord_Ellipsis gEasyChatWord_EllipsisExcl gEasyChatWord_EllipsisEllipsisEllipsis gEasyChatWord_Dash gEasyChatWord_DashDashDash gEasyChatWord_UhOh gEasyChatWord_Waaah gEasyChatWord_Ahaha gEasyChatWord_OhQues gEasyChatWord_Nope gEasyChatWord_Urgh gEasyChatWord_Hmm gEasyChatWord_Whoah gEasyChatWord_WroooaarExcl gEasyChatWord_Wow gEasyChatWord_Giggle gEasyChatWord_Sigh gEasyChatWord_Unbelievable gEasyChatWord_Cries gEasyChatWord_Agree gEasyChatWord_EhQues gEasyChatWord_Cry gEasyChatWord_Ehehe gEasyChatWord_OiOiOi gEasyChatWord_OhYeah gEasyChatWord_Oh gEasyChatWord_Oops gEasyChatWord_Shocked gEasyChatWord_Eek gEasyChatWord_Graaah gEasyChatWord_Gwahahaha gEasyChatWord_Way gEasyChatWord_Tch gEasyChatWord_Hehe gEasyChatWord_Hah gEasyChatWord_Yup gEasyChatWord_Hahaha gEasyChatWord_Aiyeeh gEasyChatWord_Hiyah gEasyChatWord_Fufufu gEasyChatWord_Lol gEasyChatWord_Snort gEasyChatWord_Humph gEasyChatWord_Hehehe gEasyChatWord_Heh gEasyChatWord_Hohoho gEasyChatWord_UhHuh gEasyChatWord_OhDear gEasyChatWord_Arrgh gEasyChatWord_Mufufu gEasyChatWord_Mmm gEasyChatWord_OhKay gEasyChatWord_Okay gEasyChatWord_Lalala gEasyChatWord_Yay gEasyChatWord_Aww gEasyChatWord_Wowee gEasyChatWord_Gwah gEasyChatWord_Wahahaha gEasyChatGroup_Voices gEasyChatWord_Listen gEasyChatWord_NotVery gEasyChatWord_Mean gEasyChatWord_Lie gEasyChatWord_Lay gEasyChatWord_Recommend gEasyChatWord_Nitwit gEasyChatWord_Quite gEasyChatWord_From gEasyChatWord_Feeling gEasyChatWord_But gEasyChatWord_However gEasyChatWord_Case gEasyChatWord_The gEasyChatWord_Miss gEasyChatWord_How gEasyChatWord_Hit gEasyChatWord_Enough gEasyChatWord_ALot gEasyChatWord_ALittle gEasyChatWord_Absolutely gEasyChatWord_And gEasyChatWord_Only gEasyChatWord_Around gEasyChatWord_Probably gEasyChatWord_If gEasyChatWord_Very gEasyChatWord_ATinyBit gEasyChatWord_Wild gEasyChatWord_Thats gEasyChatWord_Just gEasyChatWord_EvenSo gEasyChatWord_MustBe gEasyChatWord_Naturally gEasyChatWord_ForNow gEasyChatWord_Understood gEasyChatWord_Joking gEasyChatWord_Ready gEasyChatWord_Something gEasyChatWord_Somehow gEasyChatWord_Although gEasyChatWord_Also gEasyChatWord_Perfect gEasyChatWord_AsMuchAs gEasyChatWord_Really gEasyChatWord_Truly gEasyChatWord_Seriously gEasyChatWord_Totally gEasyChatWord_Until gEasyChatWord_AsIf gEasyChatWord_Mood gEasyChatWord_Rather gEasyChatWord_Awfully gEasyChatWord_Mode gEasyChatWord_More gEasyChatWord_TooLate gEasyChatWord_Finally gEasyChatWord_Any gEasyChatWord_Instead gEasyChatWord_Fantastic gEasyChatGroup_Speech gEasyChatWord_Will gEasyChatWord_WillBeHere gEasyChatWord_Or gEasyChatWord_Times gEasyChatWord_Wonder gEasyChatWord_IsItQues gEasyChatWord_Be gEasyChatWord_Gimme gEasyChatWord_Could gEasyChatWord_LikelyTo gEasyChatWord_Would gEasyChatWord_Is gEasyChatWord_IsntItQues gEasyChatWord_Lets gEasyChatWord_Other gEasyChatWord_Are gEasyChatWord_Was gEasyChatWord_Were gEasyChatWord_Those gEasyChatWord_Isnt gEasyChatWord_Wont gEasyChatWord_Cant gEasyChatWord_Can gEasyChatWord_Dont gEasyChatWord_Do gEasyChatWord_Does gEasyChatWord_Whom gEasyChatWord_Which gEasyChatWord_Wasnt gEasyChatWord_Werent gEasyChatWord_Have gEasyChatWord_Havent gEasyChatWord_A gEasyChatWord_An gEasyChatWord_Not gEasyChatWord_There gEasyChatWord_OkQues gEasyChatWord_So gEasyChatWord_Maybe gEasyChatWord_About gEasyChatWord_Over gEasyChatWord_It gEasyChatWord_All gEasyChatWord_For gEasyChatWord_On gEasyChatWord_Off gEasyChatWord_As gEasyChatWord_To gEasyChatWord_With gEasyChatWord_Better gEasyChatWord_Ever gEasyChatWord_Since gEasyChatWord_Of gEasyChatWord_BelongsTo gEasyChatWord_At gEasyChatWord_In gEasyChatWord_Out gEasyChatWord_Too gEasyChatWord_Like gEasyChatWord_Did gEasyChatWord_Didnt gEasyChatWord_Doesnt gEasyChatWord_Without gEasyChatWord_After gEasyChatWord_Before gEasyChatWord_While gEasyChatWord_Than gEasyChatWord_Once gEasyChatWord_Anywhere gEasyChatGroup_Endings gEasyChatWord_Meet gEasyChatWord_Play gEasyChatWord_Hurried gEasyChatWord_Goes gEasyChatWord_Giddy gEasyChatWord_Happy gEasyChatWord_Happiness gEasyChatWord_Excite gEasyChatWord_Important gEasyChatWord_Funny gEasyChatWord_Got gEasyChatWord_GoHome gEasyChatWord_Disappointed gEasyChatWord_Disappoints gEasyChatWord_Sad gEasyChatWord_Try gEasyChatWord_Tries gEasyChatWord_Hears gEasyChatWord_Think gEasyChatWord_Hear gEasyChatWord_Wants gEasyChatWord_Misheard gEasyChatWord_Dislike gEasyChatWord_Angry gEasyChatWord_Anger gEasyChatWord_Scary gEasyChatWord_Lonesome gEasyChatWord_Disappoint gEasyChatWord_Joy gEasyChatWord_Gets gEasyChatWord_Never gEasyChatWord_Darn gEasyChatWord_Downcast gEasyChatWord_Incredible gEasyChatWord_Likes gEasyChatWord_Dislikes gEasyChatWord_Boring gEasyChatWord_Care gEasyChatWord_Cares gEasyChatWord_AllRight gEasyChatWord_Adore gEasyChatWord_Disaster gEasyChatWord_Enjoy gEasyChatWord_Enjoys gEasyChatWord_Eat gEasyChatWord_Lacking gEasyChatWord_Bad gEasyChatWord_Hard gEasyChatWord_Terrible gEasyChatWord_Should gEasyChatWord_Nice gEasyChatWord_Drink gEasyChatWord_Surprise gEasyChatWord_Fear gEasyChatWord_Want gEasyChatWord_Wait gEasyChatWord_Satisfied gEasyChatWord_See gEasyChatWord_Rare gEasyChatWord_Negative gEasyChatWord_Done gEasyChatWord_Danger gEasyChatWord_Defeated gEasyChatWord_Beat gEasyChatWord_Great gEasyChatWord_Romantic gEasyChatWord_Question gEasyChatWord_Understand gEasyChatWord_Understands gEasyChatGroup_Feelings gEasyChatWord_Hot gEasyChatWord_Exists gEasyChatWord_Excess gEasyChatWord_Approved gEasyChatWord_Has gEasyChatWord_Good gEasyChatWord_Less gEasyChatWord_Momentum gEasyChatWord_Going gEasyChatWord_Weird gEasyChatWord_Busy gEasyChatWord_Together gEasyChatWord_Full gEasyChatWord_Absent gEasyChatWord_Being gEasyChatWord_Need gEasyChatWord_Tasty gEasyChatWord_Skilled gEasyChatWord_Noisy gEasyChatWord_Big gEasyChatWord_Late gEasyChatWord_Close gEasyChatWord_Docile gEasyChatWord_Amusing gEasyChatWord_Entertaining gEasyChatWord_Perfection gEasyChatWord_Pretty gEasyChatWord_Healthy gEasyChatWord_Excellent gEasyChatWord_UpsideDown gEasyChatWord_Cold gEasyChatWord_Refreshing gEasyChatWord_Unavoidable gEasyChatWord_Much gEasyChatWord_Overwhelming gEasyChatWord_Fabulous gEasyChatWord_Else gEasyChatWord_Expensive gEasyChatWord_Correct gEasyChatWord_Impossible gEasyChatWord_Small gEasyChatWord_Different gEasyChatWord_Tired gEasyChatWord_Skill gEasyChatWord_Top gEasyChatWord_NonStop gEasyChatWord_Preposterous gEasyChatWord_None gEasyChatWord_Nothing gEasyChatWord_Natural gEasyChatWord_Becomes gEasyChatWord_Lukewarm gEasyChatWord_Fast gEasyChatWord_Low gEasyChatWord_Awful gEasyChatWord_Alone gEasyChatWord_Bored gEasyChatWord_Secret gEasyChatWord_Mystery gEasyChatWord_Lacks gEasyChatWord_Best gEasyChatWord_Lousy gEasyChatWord_Mistake gEasyChatWord_Kind gEasyChatWord_Well gEasyChatWord_Weakened gEasyChatWord_Simple gEasyChatWord_Seems gEasyChatWord_Badly gEasyChatGroup_Conditions gEasyChatWord_Meets gEasyChatWord_Concede gEasyChatWord_Give gEasyChatWord_Gives gEasyChatWord_Played gEasyChatWord_Plays gEasyChatWord_Collect gEasyChatWord_Walking gEasyChatWord_Walks gEasyChatWord_Says gEasyChatWord_Went gEasyChatWord_Said gEasyChatWord_WakeUp gEasyChatWord_WakesUp gEasyChatWord_Angers gEasyChatWord_Teach gEasyChatWord_Teaches gEasyChatWord_Please gEasyChatWord_Learn gEasyChatWord_Change gEasyChatWord_Story gEasyChatWord_Trust gEasyChatWord_Lavish gEasyChatWord_Listens gEasyChatWord_Hearing gEasyChatWord_Trains gEasyChatWord_Choose gEasyChatWord_Come gEasyChatWord_Came gEasyChatWord_Search gEasyChatWord_Make gEasyChatWord_Cause gEasyChatWord_Know gEasyChatWord_Knows gEasyChatWord_Refuse gEasyChatWord_Stores gEasyChatWord_Brag gEasyChatWord_Ignorant gEasyChatWord_Thinks gEasyChatWord_Believe gEasyChatWord_Slide gEasyChatWord_Eats gEasyChatWord_Use gEasyChatWord_Uses gEasyChatWord_Using gEasyChatWord_Couldnt gEasyChatWord_Capable gEasyChatWord_Disappear gEasyChatWord_Appear gEasyChatWord_Throw gEasyChatWord_Worry gEasyChatWord_Slept gEasyChatWord_Sleep gEasyChatWord_Release gEasyChatWord_Drinks gEasyChatWord_Runs gEasyChatWord_Run gEasyChatWord_Works gEasyChatWord_Working gEasyChatWord_Talking gEasyChatWord_Talk gEasyChatWord_Sink gEasyChatWord_Smack gEasyChatWord_Pretend gEasyChatWord_Praise gEasyChatWord_Overdo gEasyChatWord_Show gEasyChatWord_Looks gEasyChatWord_Sees gEasyChatWord_Seek gEasyChatWord_Own gEasyChatWord_Take gEasyChatWord_Allow gEasyChatWord_Forget gEasyChatWord_Forgets gEasyChatWord_Appears gEasyChatWord_Faint gEasyChatWord_Fainted gEasyChatGroup_Actions gEasyChatWord_Chores gEasyChatWord_Home gEasyChatWord_Money gEasyChatWord_Allowance gEasyChatWord_Bath gEasyChatWord_Conversation gEasyChatWord_School gEasyChatWord_Commemorate gEasyChatWord_Habit gEasyChatWord_Group gEasyChatWord_Word gEasyChatWord_Store gEasyChatWord_Service gEasyChatWord_Work gEasyChatWord_System gEasyChatWord_Train gEasyChatWord_Class gEasyChatWord_Lessons gEasyChatWord_Information gEasyChatWord_Living gEasyChatWord_Teacher gEasyChatWord_Tournament gEasyChatWord_Letter gEasyChatWord_Event gEasyChatWord_Digital gEasyChatWord_Test gEasyChatWord_DeptStore gEasyChatWord_Television gEasyChatWord_Phone gEasyChatWord_Item gEasyChatWord_Name gEasyChatWord_News gEasyChatWord_Popular gEasyChatWord_Party gEasyChatWord_Study gEasyChatWord_Machine gEasyChatWord_Mail gEasyChatWord_Message gEasyChatWord_Promise gEasyChatWord_Dream gEasyChatWord_Kindergarten gEasyChatWord_Life gEasyChatWord_Radio gEasyChatWord_Rental gEasyChatWord_World gEasyChatGroup_Lifestyle gEasyChatWord_Idol gEasyChatWord_Anime gEasyChatWord_Song gEasyChatWord_Movie gEasyChatWord_Sweets gEasyChatWord_Chat gEasyChatWord_ChildsPlay gEasyChatWord_Toys gEasyChatWord_Music gEasyChatWord_Cards gEasyChatWord_Shopping gEasyChatWord_Camera gEasyChatWord_Viewing gEasyChatWord_Spectator gEasyChatWord_Gourmet gEasyChatWord_Game gEasyChatWord_Rpg gEasyChatWord_Collection gEasyChatWord_Complete gEasyChatWord_Magazine gEasyChatWord_Walk gEasyChatWord_Bike gEasyChatWord_Hobby gEasyChatWord_Sports gEasyChatWord_Software gEasyChatWord_Songs gEasyChatWord_Diet gEasyChatWord_Treasure gEasyChatWord_Travel gEasyChatWord_Dance gEasyChatWord_Channel gEasyChatWord_Making gEasyChatWord_Fishing gEasyChatWord_Date gEasyChatWord_Design gEasyChatWord_Locomotive gEasyChatWord_PlushDoll gEasyChatWord_Pc gEasyChatWord_Flowers gEasyChatWord_Hero gEasyChatWord_Nap gEasyChatWord_Heroine gEasyChatWord_Fashion gEasyChatWord_Adventure gEasyChatWord_Board gEasyChatWord_Ball gEasyChatWord_Book gEasyChatWord_Festival gEasyChatWord_Comics gEasyChatWord_Holiday gEasyChatWord_Plans gEasyChatWord_Trendy gEasyChatWord_Vacation gEasyChatWord_Look gEasyChatGroup_Hobbies gEasyChatWord_Fall gEasyChatWord_Morning gEasyChatWord_Tomorrow gEasyChatWord_Last gEasyChatWord_Day gEasyChatWord_Sometime gEasyChatWord_Always gEasyChatWord_Current gEasyChatWord_Forever gEasyChatWord_Days gEasyChatWord_End gEasyChatWord_Tuesday gEasyChatWord_Yesterday gEasyChatWord_Today gEasyChatWord_Friday gEasyChatWord_Monday gEasyChatWord_Later gEasyChatWord_Earlier gEasyChatWord_Another gEasyChatWord_Time gEasyChatWord_Finish gEasyChatWord_Wednesday gEasyChatWord_Soon gEasyChatWord_Start gEasyChatWord_Month gEasyChatWord_Stop gEasyChatWord_Now gEasyChatWord_Final gEasyChatWord_Next gEasyChatWord_Age gEasyChatWord_Saturday gEasyChatWord_Summer gEasyChatWord_Sunday gEasyChatWord_Beginning gEasyChatWord_Spring gEasyChatWord_Daytime gEasyChatWord_Winter gEasyChatWord_Daily gEasyChatWord_Olden gEasyChatWord_Almost gEasyChatWord_Nearly gEasyChatWord_Thursday gEasyChatWord_Nighttime gEasyChatWord_Night gEasyChatWord_Week gEasyChatGroup_Time gEasyChatWord_Highs gEasyChatWord_Lows gEasyChatWord_Um gEasyChatWord_Rear gEasyChatWord_Things gEasyChatWord_Thing gEasyChatWord_Below gEasyChatWord_Above gEasyChatWord_Back gEasyChatWord_High gEasyChatWord_Here gEasyChatWord_Inside gEasyChatWord_Outside gEasyChatWord_Beside gEasyChatWord_ThisIsItExcl gEasyChatWord_This gEasyChatWord_Every gEasyChatWord_These gEasyChatWord_TheseWere gEasyChatWord_Down gEasyChatWord_That gEasyChatWord_ThoseAre gEasyChatWord_ThoseWere gEasyChatWord_ThatsItExcl gEasyChatWord_Am gEasyChatWord_ThatWas gEasyChatWord_Front gEasyChatWord_Up gEasyChatWord_Choice gEasyChatWord_Far gEasyChatWord_Away gEasyChatWord_Near gEasyChatWord_Where gEasyChatWord_When gEasyChatWord_What gEasyChatWord_Deep gEasyChatWord_Shallow gEasyChatWord_Why gEasyChatWord_Confused gEasyChatWord_Opposite gEasyChatWord_Left gEasyChatWord_Right gEasyChatGroup_Misc gEasyChatWord_Wandering gEasyChatWord_Rickety gEasyChatWord_RockSolid gEasyChatWord_Hungry gEasyChatWord_Tight gEasyChatWord_Ticklish gEasyChatWord_Twirling gEasyChatWord_Spiraling gEasyChatWord_Thirsty gEasyChatWord_Lolling gEasyChatWord_Silky gEasyChatWord_Sadly gEasyChatWord_Hopeless gEasyChatWord_Useless gEasyChatWord_Drooling gEasyChatWord_Exciting gEasyChatWord_Thick gEasyChatWord_Smooth gEasyChatWord_Slimy gEasyChatWord_Thin gEasyChatWord_Break gEasyChatWord_Voracious gEasyChatWord_Scatter gEasyChatWord_Awesome gEasyChatWord_Wimpy gEasyChatWord_Wobbly gEasyChatWord_Shaky gEasyChatWord_Ripped gEasyChatWord_Shredded gEasyChatWord_Increasing gEasyChatWord_Yet gEasyChatWord_Destroyed gEasyChatWord_Fiery gEasyChatWord_LoveyDovey gEasyChatWord_Happily gEasyChatWord_Anticipation gEasyChatGroup_Adjectives gEasyChatWord_Appeal gEasyChatWord_Events gEasyChatWord_StayAtHome gEasyChatWord_Berry gEasyChatWord_Contest gEasyChatWord_Mc gEasyChatWord_Judge gEasyChatWord_Super gEasyChatWord_Stage gEasyChatWord_HallOfFame gEasyChatWord_Evolution gEasyChatWord_Hyper gEasyChatWord_BattleTower gEasyChatWord_Leaders gEasyChatWord_BattleRoom gEasyChatWord_Hidden gEasyChatWord_SecretBase gEasyChatWord_Blend gEasyChatWord_POKEBLOCK gEasyChatWord_Master gEasyChatWord_Rank gEasyChatWord_Ribbon gEasyChatWord_Crush gEasyChatWord_Direct gEasyChatWord_Tower gEasyChatWord_Union gEasyChatWord_Room gEasyChatWord_Wireless gEasyChatWord_Frontier gEasyChatGroup_Events gEasyChatGroup_Move1 gEasyChatGroup_Move2 gEasyChatWord_KthxBye gEasyChatWord_YesSirExcl gEasyChatWord_AvantGarde gEasyChatWord_Couple gEasyChatWord_MuchObliged gEasyChatWord_YeehawExcl gEasyChatWord_Mega gEasyChatWord_1HitKOExcl gEasyChatWord_Destiny gEasyChatWord_Cancel gEasyChatWord_New gEasyChatWord_Flatten gEasyChatWord_Kidding gEasyChatWord_Loser gEasyChatWord_Losing gEasyChatWord_Happening gEasyChatWord_HipAnd gEasyChatWord_Shake gEasyChatWord_Shady gEasyChatWord_Upbeat gEasyChatWord_Modern gEasyChatWord_SmellYa gEasyChatWord_Bang gEasyChatWord_Knockout gEasyChatWord_Hassle gEasyChatWord_Winner gEasyChatWord_Fever gEasyChatWord_Wannabe gEasyChatWord_Baby gEasyChatWord_Heart gEasyChatWord_Old gEasyChatWord_Young gEasyChatWord_Ugly gEasyChatGroup_TrendySaying gEasyChatGroup_Pokemon2 gEasyChatGroups gEasyChatWordsByLetter_Others gEasyChatWordsByLetter_A gEasyChatWordsByLetter_B gEasyChatWordsByLetter_C gEasyChatWordsByLetter_D gEasyChatWordsByLetter_E gEasyChatWordsByLetter_F gEasyChatWordsByLetter_G gEasyChatWordsByLetter_H gEasyChatWordsByLetter_I gEasyChatWordsByLetter_J gEasyChatWordsByLetter_K gEasyChatWordsByLetter_L gEasyChatWordsByLetter_M gEasyChatWordsByLetter_N gEasyChatWordsByLetter_O gEasyChatWordsByLetter_P gEasyChatWordsByLetter_Q gEasyChatWordsByLetter_R gEasyChatWordsByLetter_S gEasyChatWordsByLetter_T gEasyChatWordsByLetter_U gEasyChatWordsByLetter_V gEasyChatWordsByLetter_W gEasyChatWordsByLetter_X gEasyChatWordsByLetter_Y gEasyChatWordsByLetter_Z gEasyChatWordsByLetter_UnusedJapaneseHi gEasyChatWordsByLetter_UnusedJapaneseFu gEasyChatWordsByLetter_UnusedJapaneseHe gEasyChatWordsByLetter_UnusedJapaneseHo gEasyChatWordsByLetter_UnusedJapaneseMa gEasyChatWordsByLetter_UnusedJapaneseMi gEasyChatWordsByLetter_UnusedJapaneseMu gEasyChatWordsByLetter_UnusedJapaneseMe gEasyChatWordsByLetter_UnusedJapaneseMo gEasyChatWordsByLetter_UnusedJapaneseYa gEasyChatWordsByLetter_UnusedJapaneseYu gEasyChatWordsByLetter_UnusedJapaneseYo gEasyChatWordsByLetter_UnusedJapaneseRa gEasyChatWordsByLetter_UnusedJapaneseRi gEasyChatWordsByLetter_UnusedJapaneseRu gEasyChatWordsByLetter_UnusedJapaneseRe gEasyChatWordsByLetter_UnusedJapaneseRo gEasyChatWordsByLetter_UnusedJapaneseWa gEasyChatWordsByLetterPointers sEasyChatGroupNamePointers sDefaultProfileWords sDefaultBattleStartWords sDefaultBattleWonWords sDefaultBattleLostWords sRestrictedWordSpecies
#[allow(unused_imports)]
use crate::data::easy_chat::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEasyChatScreen: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScreenControl: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWordData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gEasyChatMode_Pal: u8;
    static mut gEasyChatWindow_Gfx: u8;
    static mut gEasyChatWindow_Tilemap: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gNumBardWords_Moves: u8;
    static mut gNumBardWords_Species: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AllTextBeingEditedWill: u8;
    static mut gText_BeDeletedThatOkay: u8;
    static mut gText_ChallengeQuestionMark: u8;
    static mut gText_CombineTwoWordsOrPhrases3: u8;
    static mut gText_CreateAQuiz: u8;
    static mut gText_F700sQuiz: u8;
    static mut gText_Lady: u8;
    static mut gText_LikeToQuitQuiz: u8;
    static mut gText_LyricsCantBeDeleted: u8;
    static mut gText_OnlyOnePhrase: u8;
    static mut gText_OriginalSongWillBeUsed: u8;
    static mut gText_QuitEditing: u8;
    static mut gText_SectionMustBeCompleted: u8;
    static mut gText_SelectTheAnswer: u8;
    static mut gText_StopGivingPkmnMail: u8;
    static mut gText_ThreeQuestionMarks: u8;
    static mut gText_YouCannotQuitHere: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScript();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CleanupOverworldWindowsAndTilemaps();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetQuestionnaireWordsPtr() -> *mut u16;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsOverworldLinkActive() -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheets(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn TransferPlttBuffer();
    fn TrySetTrendyPhrase(a0: *mut u16) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn WriteColorChangeControlCode(a0: *mut u8, a1: u32, a2: u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoEasyChatScreen(
    r#type: u8,
    words: *mut u16,
    exitCallback: Option<unsafe extern "C" fn()>,
    displayedPersonType: u8,
) {
    unsafe {
        let mut r#type = r#type;
        let mut words = words;
        let mut exitCallback = exitCallback;
        let mut displayedPersonType = displayedPersonType;
        let mut taskId: u8 = 0u8;
        ResetTasks();
        taskId = CreateTask(Some(Task_InitEasyChatScreen), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((r#type) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((displayedPersonType) as i16));
        SetWordTaskArg(taskId, 2u8, ((words) as usize as u32));
        SetWordTaskArg(
            taskId,
            4u8,
            (core::mem::transmute::<_, usize>(exitCallback) as u32),
        );
        SetMainCallback2(Some(CB2_EasyChatScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_EasyChatScreen() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_EasyChatScreen() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
    }
}
pub(crate) unsafe extern "C" fn StartEasyChatScreen(
    taskId: u8,
    taskFunc: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut taskFunc = taskFunc;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(taskFunc);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_InitEasyChatScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsOverworldLinkActive()) != 0) {
            'l1: loop {
                if !((InitEasyChatScreen(taskId)) != 0) {
                    break 'l1;
                }
            }
        } else {
            if ((InitEasyChatScreen(taskId)) as i32) == 1i32 {
                return;
            }
        }
        StartEasyChatScreen(taskId, Some(Task_EasyChatScreen));
    }
}
pub(crate) unsafe extern "C" fn Task_EasyChatScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut funcId: u16 = 0u16;
        let mut data: *mut i16 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(Some(VBlankCB_EasyChatScreen));
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                (data).write(5i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                funcId = HandleEasyChatInput();
                if (IsFuncIdForQuizLadyScreen(funcId)) != 0 {
                    BeginNormalPaletteFade(4294967295u32, (-2i8), 0u8, 16u8, 0u16);
                    (data).write(3i16);
                    ((data).wrapping_offset(6)).write(((funcId) as i16));
                } else {
                    if ((funcId) as i32) == 24i32 {
                        BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                        (data).write(4i16);
                    } else {
                        if ((funcId) as i32) != 0i32 {
                            PlaySE(5u16);
                            StartEasyChatFunction(funcId);
                            (data).write(((data).read()).wrapping_add(1));
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((RunEasyChatFunction()) != 0) {
                    (data).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    EnterQuizLadyScreen(((((data).wrapping_offset(6)).read()) as u16));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ExitEasyChatScreen(
                        (core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(
                            (GetWordTaskArg(taskId, 4u8)) as usize,
                        )),
                    );
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (data).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreen(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                ResetSpriteData();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((InitEasyChatScreenWordData()) != 0) {
                    ExitEasyChatScreen(
                        (core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(
                            (GetWordTaskArg(taskId, 4u8)) as usize,
                        )),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((InitEasyChatScreenStruct(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    ((GetWordTaskArg(taskId, 2u8)) as usize as *mut u16),
                    ((((data).wrapping_offset(7)).read()) as u8),
                )) != 0)
                {
                    ExitEasyChatScreen(
                        (core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(
                            (GetWordTaskArg(taskId, 4u8)) as usize,
                        )),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((InitEasyChatScreenControl()) != 0) {
                    ExitEasyChatScreen(
                        (core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(
                            (GetWordTaskArg(taskId, 4u8)) as usize,
                        )),
                    );
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (LoadEasyChatScreen()) != 0 {
                    return 1u8;
                }
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ExitEasyChatScreen(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        FreeEasyChatScreenControl();
        FreeEasyChatScreenStruct();
        FreeEasyChatScreenWordData();
        FreeAllWindowBuffers();
        SetMainCallback2(callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowEasyChatScreen() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut words: *mut u16 = core::ptr::null_mut();
        let mut bard: *mut u8 = core::ptr::null_mut();
        let mut displayedPersonType: u8 = 3u8;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 6i32
                || __sw1 == 5i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32;
            if __sw1 == 0i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11184))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 1i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11196))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 2i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11208))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 3i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11220))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 4i32 {
                words = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11232))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 6i32 {
                bard = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((bard).wrapping_add(14)).cast::<u16>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((((bard).wrapping_add(2)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                words = ((bard).wrapping_add(14)).cast::<u16>();
                break 'l1;
            }
            if __sw1 == 5i32 {
                words = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(4))
                .cast::<u16>();
                displayedPersonType =
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                words = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(28))
                .cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize,
                );
                displayedPersonType = 1u8;
                break 'l1;
            }
            if __sw1 == 8i32 {
                words = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(2))
                .cast::<u16>();
                displayedPersonType = 0u8;
                break 'l1;
            }
            if __sw1 == 9i32 {
                words = ((&raw mut gStringVar3).cast::<u8>()).cast::<u16>();
                (words).write(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11880))
                    .cast::<u8>())
                    .wrapping_add(4))
                    .cast::<u16>())
                    .read(),
                );
                ((words).wrapping_offset(1)).write(
                    ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11880))
                    .cast::<u8>())
                    .wrapping_add(4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 10i32 {
                words = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11172))
                .wrapping_add(6))
                .cast::<u16>();
                (words).write(65535u16);
                displayedPersonType = 1u8;
                break 'l1;
            }
            if __sw1 == 11i32 {
                words = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(4))
                .cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize,
                );
                displayedPersonType = 0u8;
                break 'l1;
            }
            if __sw1 == 12i32 {
                words = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(24))
                .cast::<u16>();
                displayedPersonType = 1u8;
                break 'l1;
            }
            if __sw1 == 13i32 {
                words = ((&raw mut gStringVar3).cast::<u8>()).cast::<u16>();
                InitializeEasyChatWordArray(words, 2u16);
                break 'l1;
            }
            if __sw1 == 14i32 {
                words = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .wrapping_add(20))
                .cast::<u16>();
                (words).write(65535u16);
                displayedPersonType = 2u8;
                break 'l1;
            }
            if __sw1 == 15i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(15192))
                .wrapping_add(22)
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 16i32 {
                return;
            }
            if __sw1 == 17i32 {
                words = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(15192))
                .wrapping_add(2))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 18i32 {
                words = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(15192))
                .wrapping_add(20)
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 19i32 {
                words = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(220))
                .cast::<u8>())
                .wrapping_add(40))
                .cast::<u16>();
                break 'l1;
            }
            if __sw1 == 20i32 {
                words = GetQuestionnaireWordsPtr();
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        CleanupOverworldWindowsAndTilemaps();
        DoEasyChatScreen(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
            words,
            Some(CB2_ReturnToFieldContinueScript),
            displayedPersonType,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_QuizLadyQuestion() {
    unsafe {
        let mut lilycoveLady: *mut u8 = core::ptr::null_mut();
        UpdatePaletteFade();
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                FadeScreen(1u8, 0i8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    lilycoveLady =
                        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192);
                    ((lilycoveLady).wrapping_add(22).cast::<u16>()).write(65535u16);
                    CleanupOverworldWindowsAndTilemaps();
                    DoQuizQuestionEasyChatScreen();
                }
                return;
            }
        }
        let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyShowQuizQuestion() {
    unsafe {
        SetMainCallback2(Some(CB2_QuizLadyQuestion));
    }
}
pub(crate) unsafe extern "C" fn GetQuizLadyScreenByFuncId(funcId: u16) -> i32 {
    unsafe {
        let mut funcId = funcId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(32u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((funcId) as i32)
                        == (((((((&raw const sQuizLadyEasyChatScreens)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
pub(crate) unsafe extern "C" fn IsFuncIdForQuizLadyScreen(funcId: u16) -> u32 {
    unsafe {
        let mut funcId = funcId;
        return ((if GetQuizLadyScreenByFuncId(funcId) == (-1i32) {
            0i32
        } else {
            1i32
        }) as u32);
    }
}
pub(crate) unsafe extern "C" fn EnterQuizLadyScreen(funcId: u16) {
    unsafe {
        let mut funcId = funcId;
        let mut i: i32 = 0i32;
        i = GetQuizLadyScreenByFuncId(funcId);
        ResetTasks();
        ExitEasyChatScreen(
            (((((&raw const sQuizLadyEasyChatScreens)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((i) as isize * 8))
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoQuizAnswerEasyChatScreen() {
    unsafe {
        DoEasyChatScreen(
            15u8,
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192))
                .wrapping_add(22)
                .cast::<u16>(),
            Some(CB2_ReturnToFieldContinueScript),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DoQuizQuestionEasyChatScreen() {
    unsafe {
        DoEasyChatScreen(
            16u8,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192))
                .wrapping_add(2))
            .cast::<u16>(),
            Some(CB2_ReturnToFieldContinueScript),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DoQuizSetAnswerEasyChatScreen() {
    unsafe {
        DoEasyChatScreen(
            18u8,
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192))
                .wrapping_add(20)
                .cast::<u16>(),
            Some(CB2_ReturnToFieldContinueScript),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DoQuizSetQuestionEasyChatScreen() {
    unsafe {
        DoEasyChatScreen(
            17u8,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192))
                .wrapping_add(2))
            .cast::<u16>(),
            Some(CB2_ReturnToFieldContinueScript),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenStruct(
    r#type: u8,
    words: *mut u16,
    displayedPersonType: u8,
) -> u8 {
    unsafe {
        let mut r#type = r#type;
        let mut words = words;
        let mut displayedPersonType = displayedPersonType;
        let mut templateId: u8 = 0u8;
        let mut i: i32 = 0i32;
        ((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).write(Alloc(80u32));
        if ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
        {
            return 0u8;
        }
        (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).write(r#type);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<*mut u16>())
        .write(words);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(displayedPersonType);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
            .write(0u8);
        templateId = GetEachChatScreenTemplateId(r#type);
        if ((r#type) as i32) == 16i32 {
            GetQuizTitle(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .cast::<u8>(),
            );
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<*mut u8>())
            .write(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .cast::<u8>(),
            );
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(7u8);
        } else {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<*mut u8>())
            .write(
                (((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((templateId) as i32) as isize * 24))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read(),
            );
        }
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(
                (((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((templateId) as i32) as isize * 24))
                .wrapping_add(1))
                .read(),
            );
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(
                (((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((templateId) as i32) as isize * 24))
                .wrapping_add(2))
                .read(),
            );
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
            .write(
                ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    .wrapping_mul(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read()) as i32),
                    )) as u8),
            );
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(templateId);
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
            .read()) as u32)
            > crate::c::div_u32(18u32, 2u32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
                .write(((crate::c::div_u32(18u32, 2u32)) as u8));
        }
        if ((words) as usize) != 0usize {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (words).cast::<u8>(),
                                (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(60))
                                .cast::<u16>())
                                .cast::<u8>(),
                                (0u32
                                    | (crate::c::div_u32(
                                        ((((((&raw mut sEasyChatScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(7))
                                        .read()) as u32)
                                            .wrapping_mul(2u32),
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        } else {
            {
                i = 0i32;
                'l5: loop {
                    if !(i
                        < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(7))
                        .read()) as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .write(65535u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(56)
                .cast::<*mut u16>())
            .write(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60))
                .cast::<u16>(),
            );
        }
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(
                (((crate::c::div_i32(
                    ((GetNumUnlockedEasyChatGroups()) as i32).wrapping_sub(1i32),
                    2i32,
                ))
                .wrapping_add(1i32)) as u8),
            );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenStruct() {
    unsafe {
        if ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
        {
            Free(((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32);
            if __sw1 == 0i32 {
                return HandleEasyChatInput_Phrase();
            }
            if __sw1 == 1i32 {
                return HandleEasyChatInput_MainScreenButtons();
            }
            if __sw1 == 2i32 {
                return HandleEasyChatInput_Keyboard();
            }
            if __sw1 == 3i32 {
                return HandleEasyChatInput_WordSelect();
            }
            if __sw1 == 4i32 {
                return HandleEasyChatInput_ExitPrompt();
            }
            if __sw1 == 5i32 {
                return HandleEasyChatInput_DeleteAllYesNo();
            }
            if __sw1 == 6i32 {
                return HandleEasyChatInput_ConfirmWordsYesNo();
            }
            if __sw1 == 7i32 {
                return HandleEasyChatInput_QuizQuestion();
            }
            if __sw1 == 8i32 {
                return HandleEasyChatInput_WaitForMsg();
            }
            if __sw1 == 9i32 {
                return HandleEasyChatInput_StartConfirmLyrics();
            }
            if __sw1 == 10i32 {
                return HandleEasyChatInput_ConfirmLyricsYesNo();
            }
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn IsCurrentFrame2x5() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((GetEasyChatScreenFrameId()) as i32);
            if __sw1 == 2i32 || __sw1 == 7i32 || __sw1 == 8i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_Phrase() -> u16 {
    unsafe {
        'l1: loop {
            'l2: {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    ClearUnusedField();
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(2u8);
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .write(0i8);
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .write(0u8);
                    return 9u16;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        return StartConfirmExitPrompt();
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 8i32)
                            != 0
                        {
                            return TryConfirmWords();
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 64i32)
                                != 0
                            {
                                let __p1 =
                                    (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<i8>();
                                (__p1).write(((__p1).read()).wrapping_sub(1));
                                break 'l1;
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 32i32)
                                    != 0
                                {
                                    let __p2 = (((&raw mut sEasyChatScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(5)
                                    .cast::<i8>();
                                    (__p2).write(((__p2).read()).wrapping_sub(1));
                                    break 'l1;
                                } else {
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(46)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 128i32)
                                        != 0
                                    {
                                        let __p3 = (((&raw mut sEasyChatScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(6)
                                        .cast::<i8>();
                                        (__p3).write(((__p3).read()).wrapping_add(1));
                                        break 'l1;
                                    } else {
                                        if ((((((&raw mut gMain).cast::<u8>())
                                            .wrapping_add(46)
                                            .cast::<u16>())
                                        .read())
                                            as i32)
                                            & 16i32)
                                            != 0
                                        {
                                            let __p4 = (((&raw mut sEasyChatScreen)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(5)
                                            .cast::<i8>();
                                            (__p4).write(((__p4).read()).wrapping_add(1));
                                            break 'l1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return 0u16;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as i32)
            < 0i32
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<i8>())
            .write(
                (((((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 24,
                ))
                .wrapping_add(2))
                .read()) as i8),
            );
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as i32)
            > (((((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(2))
            .read()) as i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<i8>())
            .write(0i8);
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as i32)
            == (((((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(2))
            .read()) as i32)
        {
            if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32)
                > 2i32
            {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .write(2i8);
            }
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            return 3u16;
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .read()) as i32)
            < 0i32
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .write(
                (((((((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 24,
                ))
                .wrapping_add(1))
                .read()) as i32)
                    .wrapping_sub(1i32)) as i8),
            );
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .read()) as i32)
            >= (((((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(1))
            .read()) as i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .write(0i8);
        }
        if (((IsCurrentFrame2x5()) != 0)
            && (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32)
                == 1i32))
            && (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<i8>())
            .read()) as i32)
                == 4i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .write(0i8);
        }
        return 2u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_MainScreenButtons() -> u16 {
    unsafe {
        'l1: loop {
            'l2: {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    'l3: {
                        let __sw1 =
                            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(5)
                            .cast::<i8>())
                            .read()) as i32);
                        if __sw1 == 0i32 {
                            return ((DoDeleteAllButton()) as u16);
                        }
                        if __sw1 == 1i32 {
                            return StartConfirmExitPrompt();
                        }
                        if __sw1 == 2i32 {
                            return TryConfirmWords();
                        }
                        if __sw1 == 3i32 {
                            return ((DoQuizButton()) as u16);
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
                    return StartConfirmExitPrompt();
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 8i32)
                        != 0
                    {
                        return TryConfirmWords();
                    } else {
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 64i32)
                            != 0
                        {
                            let __p2 =
                                (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(6)
                                .cast::<i8>();
                            (__p2).write(((__p2).read()).wrapping_sub(1));
                            break 'l1;
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 32i32)
                                != 0
                            {
                                let __p3 =
                                    (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(5)
                                    .cast::<i8>();
                                (__p3).write(((__p3).read()).wrapping_sub(1));
                                break 'l1;
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 128i32)
                                    != 0
                                {
                                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6)
                                    .cast::<i8>())
                                    .write(0i8);
                                    break 'l1;
                                } else {
                                    if ((((((&raw mut gMain).cast::<u8>())
                                        .wrapping_add(46)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        & 16i32)
                                        != 0
                                    {
                                        let __p4 = (((&raw mut sEasyChatScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(5)
                                        .cast::<i8>();
                                        (__p4).write(((__p4).read()).wrapping_add(1));
                                        break 'l1;
                                    }
                                }
                            }
                        }
                    }
                }
                return 0u16;
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as i32)
            == (((((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(2))
            .read()) as i32)
        {
            let mut numFooterColumns: i32 = (if (FooterHasFourOptions()) != 0 {
                4i32
            } else {
                3i32
            });
            if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32)
                < 0i32
            {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .write((((numFooterColumns).wrapping_sub(1i32)) as i8));
            }
            if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32)
                >= numFooterColumns
            {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .write(0i8);
            }
            return 3u16;
        }
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .read()) as i32)
            >= (((((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(1))
            .read()) as i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .write(
                (((((((((&raw const sEasyChatScreenTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1))
                    .read()) as i32) as isize
                        * 24,
                ))
                .wrapping_add(1))
                .read()) as i32)
                    .wrapping_sub(1i32)) as i8),
            );
        }
        if (((IsCurrentFrame2x5()) != 0)
            && (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32)
                == 1i32))
            && (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<i8>())
            .read()) as i32)
                == 4i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .write(0i8);
        }
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        return 2u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_Keyboard() -> u16 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            return ((ExitKeyboardToMainScreen()) as u16);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as i32)
                != (-1i32)
            {
                return ((SelectKeyboardGroup()) as u16);
            }
            'l1: {
                let __sw1 = ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(11)
                .cast::<i8>())
                .read()) as i32);
                if __sw1 == 0i32 {
                    return ((StartSwitchKeyboardMode()) as u16);
                }
                if __sw1 == 1i32 {
                    return ((DeleteSelectedWord()) as u16);
                }
                if __sw1 == 2i32 {
                    return ((ExitKeyboardToMainScreen()) as u16);
                }
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            return ((StartSwitchKeyboardMode()) as u16);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return MoveKeyboardCursor(2i32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return MoveKeyboardCursor(3i32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            return MoveKeyboardCursor(1i32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            return MoveKeyboardCursor(0i32);
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_WordSelect() -> u16 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(2u8);
            return 14u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            return ((SelectNewWord()) as u16);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 8i32)
            != 0
        {
            return MoveWordSelectCursor(4u32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            return MoveWordSelectCursor(5u32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return MoveWordSelectCursor(2u32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return MoveWordSelectCursor(3u32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            return MoveWordSelectCursor(1u32);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            return MoveWordSelectCursor(0u32);
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ExitPrompt() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(GetEasyChatBackupState());
                return 7u16;
            }
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    == 17i32)
                    || ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .read()) as i32)
                        == 18i32)
                {
                    SaveCurrentPhrase();
                }
                return 24u16;
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ConfirmWordsYesNo() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(GetEasyChatBackupState());
                return 7u16;
            }
            if __sw1 == 0i32 {
                SetSpecialEasyChatResult();
                ((&raw mut gSpecialVar_Result).cast::<u16>())
                    .write(((GetEasyChatCompleted()) as u16));
                SaveCurrentPhrase();
                return 24u16;
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_DeleteAllYesNo() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(1u8);
                return 7u16;
            }
            if __sw1 == 0i32 {
                ResetCurrentPhrase();
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(1u8);
                return 8u16;
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_QuizQuestion() -> u16 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            return 26u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            return StartConfirmExitPrompt();
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_WaitForMsg() -> u16 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(GetEasyChatBackupState());
            return 7u16;
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_StartConfirmLyrics() -> u16 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(10u8);
        return 6u16;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ConfirmLyricsYesNo() -> u16 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == 1i32 || __sw1 == 0i32;
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                ResetCurrentPhraseToSaved();
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .write(0u8);
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(8u8);
                return 31u16;
            }
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>())
                    .write(((GetEasyChatCompleted()) as u16));
                SaveCurrentPhrase();
                return 24u16;
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn StartConfirmExitPrompt() -> u16 {
    unsafe {
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 19i32)
            || ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 11i32)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .write(
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                );
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(8u8);
            return 34u16;
        } else {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .write(
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                );
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(4u8);
            return 5u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn DoDeleteAllButton() -> i32 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read(),
            );
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            != 6i32
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(5u8);
            return 4i32;
        } else {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .write(
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read(),
                );
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(8u8);
            return 32i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn TryConfirmWords() -> u16 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read(),
            );
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 17i32
        {
            if (IsQuizQuestionEmpty()) != 0 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(8u8);
                return 29u16;
            }
            if (IsQuizAnswerEmpty()) != 0 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(8u8);
                return 30u16;
            }
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(6u8);
            return 6u16;
        } else {
            if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 18i32
            {
                if (IsQuizAnswerEmpty()) != 0 {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(8u8);
                    return 30u16;
                }
                if (IsQuizQuestionEmpty()) != 0 {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(8u8);
                    return 29u16;
                }
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(6u8);
                return 6u16;
            } else {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    == 9i32)
                    || ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .read()) as i32)
                        == 13i32)
                {
                    if !((IsCurrentPhraseFull()) != 0) {
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .write(8u8);
                        return 33u16;
                    }
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(6u8);
                    return 6u16;
                } else {
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .read()) as i32)
                        == 19i32)
                        || ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .read()) as i32)
                            == 11i32)
                    {
                        if (IsCurrentPhraseEmpty()) != 0 {
                            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .write(8u8);
                            return 34u16;
                        }
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .write(6u8);
                        return 6u16;
                    } else {
                        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .read()) as i32)
                            == 20i32
                        {
                            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .write(6u8);
                            return 6u16;
                        } else {
                            if (IsCurrentPhraseEmpty() == 1u32)
                                || (!((GetEasyChatCompleted()) != 0))
                            {
                                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4))
                                .write(4u8);
                                return 5u16;
                            }
                            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .write(6u8);
                            return 6u16;
                        }
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
pub(crate) unsafe extern "C" fn DoQuizButton() -> i32 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read(),
            );
        'l1: {
            let __sw1 = (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .read()) as i32);
            let __matched = __sw1 == 15i32 || __sw1 == 17i32 || __sw1 == 18i32;
            if __sw1 == 15i32 {
                return 25i32;
            }
            if __sw1 == 17i32 {
                SaveCurrentPhrase();
                return 28i32;
            }
            if __sw1 == 18i32 {
                SaveCurrentPhrase();
                return 27i32;
            }
            if !__matched {
                return 0i32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatBackupState() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8))
        .read();
    }
}
pub(crate) unsafe extern "C" fn SelectKeyboardGroup() -> i32 {
    unsafe {
        let mut numWords: u16 = 0u16;
        if !((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read())
            != 0)
        {
            let mut groupId: u8 = GetUnlockedEasyChatGroupId(((GetSelectedGroupIndex()) as u8));
            SetSelectedWordGroup(0u32, ((groupId) as u16));
        } else {
            SetSelectedWordGroup(1u32, ((GetSelectedAlphabetGroupId()) as u16));
        }
        numWords = GetNumWordsInSelectedGroup();
        if ((numWords) as i32) == 0i32 {
            return 0i32;
        }
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15))
            .write(((crate::c::div_i32(((numWords) as i32).wrapping_sub(1i32), 2i32)) as u8));
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .write(0u8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(17)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(3u8);
        return 11i32;
    }
}
pub(crate) unsafe extern "C" fn ExitKeyboardToMainScreen() -> i32 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        return 10i32;
    }
}
pub(crate) unsafe extern "C" fn StartSwitchKeyboardMode() -> i32 {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(11)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .write(0u8);
        if !((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read())
            != 0)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .write(1u8);
        } else {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .write(0u8);
        }
        return 23i32;
    }
}
pub(crate) unsafe extern "C" fn DeleteSelectedWord() -> i32 {
    unsafe {
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 6i32
        {
            PlaySE(32u16);
            return 0i32;
        } else {
            SetSelectedWord(65535u16);
            return 1i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn SelectNewWord() -> i32 {
    unsafe {
        let mut easyChatWord: u16 = GetWordFromSelectedGroup(GetSelectedWordIndex());
        if (DummyWordCheck(((easyChatWord) as i32))) != 0 {
            PlaySE(32u16);
            return 0i32;
        } else {
            SetSelectedWord(easyChatWord);
            if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                != 6i32
            {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(0u8);
                return 12i32;
            } else {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .write(9u8);
                return 13i32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn SaveCurrentPhrase() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(56)
                        .cast::<*mut u16>())
                    .read())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetCurrentPhrase() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetCurrentPhraseToSaved() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWord(easyChatWord: u16) {
    unsafe {
        let mut easyChatWord = easyChatWord;
        let mut index: u16 = GetWordIndexToReplace();
        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(60))
            .cast::<u16>())
        .wrapping_offset(((index) as i32) as isize))
        .write(easyChatWord);
    }
}
pub(crate) unsafe extern "C" fn DidPhraseChange() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(56)
                        .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
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
pub(crate) unsafe extern "C" fn GetEasyChatCompleted() -> u32 {
    unsafe {
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 17i32)
            || ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 18i32)
        {
            if (IsQuizQuestionEmpty()) != 0 {
                return 0u32;
            }
            if (IsQuizAnswerEmpty()) != 0 {
                return 0u32;
            }
            return 1u32;
        } else {
            return ((DidPhraseChange()) as u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor(input: i32) -> u16 {
    unsafe {
        let mut input = input;
        if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<i8>())
        .read()) as i32)
            != (-1i32)
        {
            if !((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(9))
            .read())
                != 0)
            {
                return ((MoveKeyboardCursor_GroupNames(((input) as u32))) as u16);
            } else {
                return ((MoveKeyboardCursor_Alphabet(((input) as u32))) as u16);
            }
        } else {
            return ((MoveKeyboardCursor_ButtonWindow(((input) as u32))) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_GroupNames(input: u32) -> i32 {
    unsafe {
        let mut input = input;
        'l1: {
            let __sw1 = input;
            if __sw1 == 2u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read()) as i32)
                    != ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32)
                        .wrapping_neg()
                {
                    if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .read())
                        != 0
                    {
                        let __p2 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(11)
                        .cast::<i8>();
                        (__p2).write(((__p2).read()).wrapping_sub(1));
                        return 15i32;
                    } else {
                        let __p3 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12);
                        (__p3).write(((__p3).read()).wrapping_sub(1));
                        return 17i32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .read()) as i32),
                    )
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(13))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    let mut funcId: i32 = 0i32;
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .read()) as i32)
                        < 3i32
                    {
                        let __p4 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(11)
                        .cast::<i8>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        funcId = 15i32;
                    } else {
                        let __p5 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        funcId = 16i32;
                    }
                    ReduceToValidKeyboardColumn();
                    return funcId;
                }
                break 'l1;
            }
            if __sw1 == 1u32 {
                if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read())
                    != 0
                {
                    let __p6 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<i8>();
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                } else {
                    SetKeyboardCursorInButtonWindow();
                }
                return 15i32;
            }
            if __sw1 == 0u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as i32)
                    < 1i32
                {
                    let __p7 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10)
                        .cast::<i8>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    if (IsSelectedKeyboardIndexInvalid()) != 0 {
                        SetKeyboardCursorInButtonWindow();
                    }
                } else {
                    SetKeyboardCursorInButtonWindow();
                }
                return 15i32;
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_Alphabet(input: u32) -> i32 {
    unsafe {
        let mut input = input;
        'l1: {
            let __sw1 = input;
            if __sw1 == 2u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read()) as i32)
                    > 0i32
                {
                    let __p2 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .write(3i8);
                }
                ReduceToValidKeyboardColumn();
                return 15i32;
            }
            if __sw1 == 3u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read()) as i32)
                    < 3i32
                {
                    let __p3 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .write(0i8);
                }
                ReduceToValidKeyboardColumn();
                return 15i32;
            }
            if __sw1 == 0u32 {
                let __p4 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                if (IsSelectedKeyboardIndexInvalid()) != 0 {
                    SetKeyboardCursorInButtonWindow();
                }
                return 15i32;
            }
            if __sw1 == 1u32 {
                let __p5 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>();
                (__p5).write(((__p5).read()).wrapping_sub(1));
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as i32)
                    < 0i32
                {
                    SetKeyboardCursorInButtonWindow();
                }
                return 15i32;
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_ButtonWindow(input: u32) -> i32 {
    unsafe {
        let mut input = input;
        'l1: {
            let __sw1 = input;
            if __sw1 == 2u32 {
                if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read())
                    != 0
                {
                    let __p2 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>();
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .write(2i8);
                }
                return 15i32;
            }
            if __sw1 == 3u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>())
                .read()) as i32)
                    < 2i32
                {
                    let __p3 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .write(0i8);
                }
                return 15i32;
            }
            if __sw1 == 1u32 {
                let __p4 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                SetKeyboardCursorToLastColumn();
                return 15i32;
            }
            if __sw1 == 0u32 {
                ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>())
                .write(0i8);
                let __p5 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(11)
                    .cast::<i8>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                return 15i32;
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorInButtonWindow() {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(10)
            .cast::<i8>())
        .write((-1i8));
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(11)
            .cast::<i8>())
        .read())
            != 0
        {
            let __p1 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(11)
                .cast::<i8>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorToLastColumn() {
    unsafe {
        if !((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read())
            != 0)
        {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .write(1i8);
            ReduceToValidKeyboardColumn();
        } else {
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .write(
                ((GetLastAlphabetColumn(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .read()) as u8),
                )) as i8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MoveWordSelectCursor(input: u32) -> u16 {
    unsafe {
        let mut input = input;
        let mut funcId: u16 = 0u16;
        'l1: {
            let __sw1 = input;
            if __sw1 == 2u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17)
                    .cast::<i8>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14))
                        .read()) as i32),
                    )
                    > 0i32
                {
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(17)
                        .cast::<i8>())
                    .read()) as i32)
                        > 0i32
                    {
                        let __p2 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(17)
                        .cast::<i8>();
                        (__p2).write(((__p2).read()).wrapping_sub(1));
                        funcId = 18u16;
                    } else {
                        let __p3 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14);
                        (__p3).write(((__p3).read()).wrapping_sub(1));
                        funcId = 19u16;
                    }
                    ReduceToValidWordSelectColumn();
                    return funcId;
                }
                break 'l1;
            }
            if __sw1 == 3u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(17)
                    .cast::<i8>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14))
                        .read()) as i32),
                    )
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .read()) as i32)
                {
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(17)
                        .cast::<i8>())
                    .read()) as i32)
                        < 3i32
                    {
                        let __p4 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(17)
                        .cast::<i8>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        funcId = 18u16;
                    } else {
                        let __p5 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        funcId = 20u16;
                    }
                    ReduceToValidWordSelectColumn();
                    return funcId;
                }
                break 'l1;
            }
            if __sw1 == 1u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i8>())
                .read()) as i32)
                    > 0i32
                {
                    let __p6 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<i8>();
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<i8>())
                    .write(1i8);
                }
                ReduceToValidWordSelectColumn();
                return 18u16;
            }
            if __sw1 == 0u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i8>())
                .read()) as i32)
                    < 1i32
                {
                    let __p7 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<i8>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    if (IsSelectedWordIndexInvalid()) != 0 {
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16)
                            .cast::<i8>())
                        .write(0i8);
                    }
                } else {
                    ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16)
                        .cast::<i8>())
                    .write(0i8);
                }
                return 18u16;
            }
            if __sw1 == 4u32 {
                if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read())
                    != 0
                {
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .read()) as i32)
                        >= 4i32
                    {
                        let __p8 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14);
                        (__p8).write((((((__p8).read()) as i32).wrapping_sub(4i32)) as u8));
                    } else {
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14))
                        .write(0u8);
                    }
                    return 21u16;
                }
                break 'l1;
            }
            if __sw1 == 5u32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read()) as i32)
                    <= ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .read()) as i32)
                        .wrapping_sub(4i32)
                {
                    let __p9 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14);
                    (__p9).write((((((__p9).read()) as i32).wrapping_add(4i32)) as u8));
                    if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .read()) as i32)
                        > (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(15))
                        .read()) as i32)
                            .wrapping_sub(4i32))
                        .wrapping_add(1i32)
                    {
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14))
                        .write(
                            (((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(15))
                            .read()) as i32)
                                .wrapping_sub(4i32))
                            .wrapping_add(1i32)) as u8),
                        );
                    }
                    ReduceToValidWordSelectColumn();
                    return 22u16;
                }
                break 'l1;
            }
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn GetWordIndexToReplace() -> u16 {
    unsafe {
        return (((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as i32)
            .wrapping_mul(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32),
            ))
        .wrapping_add(
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5)
                .cast::<i8>())
            .read()) as i32),
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetSelectedGroupIndex() -> u16 {
    unsafe {
        return ((((2i32).wrapping_mul(
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(11)
                .cast::<i8>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32),
                ),
        ))
        .wrapping_add(
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as i32),
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetSelectedAlphabetGroupId() -> i32 {
    unsafe {
        let mut column: i32 =
            (if (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as u8) as i32)
                < 7i32
            {
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as i32)
            } else {
                0i32
            });
        let mut row: i32 = (if (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(11)
        .cast::<i8>())
        .read()) as u8) as i32)
            < 4i32
        {
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(11)
                .cast::<i8>())
            .read()) as i32)
        } else {
            0i32
        });
        return ((((((((&raw const sAlphabetGroupIdMap).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset((row) as isize * 7))
        .cast::<u8>())
        .wrapping_offset((column) as isize))
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetSelectedWordIndex() -> u16 {
    unsafe {
        return ((((2i32).wrapping_mul(
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(17)
                .cast::<i8>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .read()) as i32),
                ),
        ))
        .wrapping_add(
            ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<i8>())
            .read()) as i32),
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn GetLastAlphabetColumn(row: u8) -> u8 {
    unsafe {
        let mut row = row;
        'l1: {
            let __sw1 = ((row) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                return 6u8;
            }
            if __sw1 == 1i32 {
                return 5u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ReduceToValidKeyboardColumn() {
    unsafe {
        'l1: loop {
            if !((IsSelectedKeyboardIndexInvalid()) != 0) {
                break 'l1;
            }
            if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read())
                != 0
            {
                let __p1 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10)
                    .cast::<i8>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReduceToValidWordSelectColumn() {
    unsafe {
        'l1: loop {
            if !((IsSelectedWordIndexInvalid()) != 0) {
                break 'l1;
            }
            if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<i8>())
            .read())
                != 0
            {
                let __p1 = (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<i8>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsSelectedKeyboardIndexInvalid() -> u8 {
    unsafe {
        if !((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read())
            != 0)
        {
            return ((if ((GetSelectedGroupIndex()) as i32)
                >= ((GetNumUnlockedEasyChatGroups()) as i32)
            {
                1i32
            } else {
                0i32
            }) as u8);
        } else {
            return ((if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as i32)
                > ((GetLastAlphabetColumn(
                    ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11)
                        .cast::<i8>())
                    .read()) as u8),
                )) as i32)
            {
                1i32
            } else {
                0i32
            }) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsSelectedWordIndexInvalid() -> u8 {
    unsafe {
        return ((if ((GetSelectedWordIndex()) as i32) >= ((GetNumWordsInSelectedGroup()) as i32) {
            1i32
        } else {
            0i32
        }) as u8);
    }
}
pub(crate) unsafe extern "C" fn FooterHasFourOptions() -> i32 {
    unsafe {
        return ((crate::c::bf_read(
            ((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(3),
            7,
            1,
            false,
        ) as u8) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatScreenType() -> u8 {
    unsafe {
        return (((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read();
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatScreenFrameId() -> u8 {
    unsafe {
        return (crate::c::bf_read(
            ((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(3),
            0,
            7,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTitleText() -> *mut u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<*mut u8>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetCurrentPhrase() -> *mut u16 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(60))
        .cast::<u16>();
    }
}
pub(crate) unsafe extern "C" fn GetNumRows() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetNumColumns() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetMainCursorColumn() -> u8 {
    unsafe {
        return ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetMainCursorRow() -> u8 {
    unsafe {
        return ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<i8>())
        .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatInstructionsText(
    str1: *mut *mut u8,
    str2: *mut *mut u8,
) {
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        (str1).write(
            (((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(8)
            .cast::<*mut u8>())
            .read(),
        );
        (str2).write(
            (((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(12)
            .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmText(str1: *mut *mut u8, str2: *mut *mut u8) {
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        (str1).write(
            (((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(16)
            .cast::<*mut u8>())
            .read(),
        );
        (str2).write(
            (((((&raw const sEasyChatScreenTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .read()) as i32) as isize
                    * 24,
            ))
            .wrapping_add(20)
            .cast::<*mut u8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmExitText(str1: *mut *mut u8, str2: *mut *mut u8) {
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        'l1: {
            let __sw1 = (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .read()) as i32);
            let __matched = __sw1 == 4i32 || __sw1 == 15i32 || __sw1 == 16i32;
            if __sw1 == 4i32 {
                (str1).write((&raw mut gText_StopGivingPkmnMail).cast::<u8>());
                (str2).write(core::ptr::null_mut());
                break 'l1;
            }
            if __sw1 == 15i32 || __sw1 == 16i32 {
                (str1).write((&raw mut gText_LikeToQuitQuiz).cast::<u8>());
                (str2).write((&raw mut gText_ChallengeQuestionMark).cast::<u8>());
                break 'l1;
            }
            if !__matched {
                (str1).write((&raw mut gText_QuitEditing).cast::<u8>());
                (str2).write(core::ptr::null_mut());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmDeletionText(
    str1: *mut *mut u8,
    str2: *mut *mut u8,
) {
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        (str1).write((&raw mut gText_AllTextBeingEditedWill).cast::<u8>());
        (str2).write((&raw mut gText_BeDeletedThatOkay).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn GetKeyboardCursorColAndRow(column: *mut i8, row: *mut i8) {
    unsafe {
        let mut column = column;
        let mut row = row;
        (column).write(
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10)
                .cast::<i8>())
            .read(),
        );
        (row).write(
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(11)
                .cast::<i8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetInAlphabetMode() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(9))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetKeyboardScrollOffset() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetWordSelectColAndRow(column: *mut i8, row: *mut i8) {
    unsafe {
        let mut column = column;
        let mut row = row;
        (column).write(
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<i8>())
            .read(),
        );
        (row).write(
            ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(17)
                .cast::<i8>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetWordSelectScrollOffset() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetWordSelectLastRow() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(15))
        .read();
    }
}
pub(crate) unsafe extern "C" fn UnusedDummy() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CanScrollUp() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32);
            if __sw1 == 2i32 {
                if (!((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9))
                .read())
                    != 0))
                    && ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read())
                        != 0)
                {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read())
                    != 0
                {
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CanScrollDown() -> u32 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32);
            if __sw1 == 2i32 {
                if (!((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9))
                .read())
                    != 0))
                    && (((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .read()) as i32)
                        .wrapping_add(4i32)
                        <= ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(13))
                        .read()) as i32)
                            .wrapping_sub(1i32))
                {
                    return 1u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read()) as i32)
                    .wrapping_add(4i32)
                    <= ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .read()) as i32)
                {
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn FooterHasFourOptions_() -> i32 {
    unsafe {
        return FooterHasFourOptions();
    }
}
pub(crate) unsafe extern "C" fn IsPhraseDifferentThanPlayerInput(
    phrase: *mut u16,
    phraseLength: u8,
) -> u8 {
    unsafe {
        let mut phrase = phrase;
        let mut phraseLength = phraseLength;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((phraseLength) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((phrase).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        != ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(60))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
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
pub(crate) unsafe extern "C" fn GetDisplayedPersonType() -> u8 {
    unsafe {
        return ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetEachChatScreenTemplateId(r#type: u8) -> u8 {
    unsafe {
        let mut r#type = r#type;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(504u32, 24u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sEasyChatScreenTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                    .read()) as i32)
                        == ((r#type) as i32)
                    {
                        return ((i) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsCurrentPhraseEmpty() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn IsCurrentPhraseFull() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn IsQuizQuestionEmpty() -> i32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut saveBlock1: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 17i32
        {
            return ((IsCurrentPhraseEmpty()) as i32);
        }
        saveBlock1 = ((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((saveBlock1).wrapping_add(15192)).wrapping_add(2)).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32
                    {
                        return 0i32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1i32;
    }
}
pub(crate) unsafe extern "C" fn IsQuizAnswerEmpty() -> i32 {
    unsafe {
        let mut quiz: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 18i32
        {
            return ((IsCurrentPhraseEmpty()) as i32);
        }
        quiz = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192));
        return (if ((((quiz).wrapping_add(20).cast::<u16>()).read()) as i32) == 65535i32 {
            1i32
        } else {
            0i32
        });
    }
}
pub(crate) unsafe extern "C" fn GetQuizTitle(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut name = crate::ffi::Align4([0u8; 32]);
        let mut saveBlock1: *mut u8 = ((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read();
        DynamicPlaceholderTextUtil_Reset();
        if ((StringLength((((saveBlock1).wrapping_add(15192)).wrapping_add(24)).cast::<u8>()))
            as i32)
            != 0i32
        {
            TVShowConvertInternationalString(
                (&raw mut name).cast::<u8>(),
                (((saveBlock1).wrapping_add(15192)).wrapping_add(24)).cast::<u8>(),
                (((((saveBlock1).wrapping_add(15192)).wrapping_add(45)).read()) as i32),
            );
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut name).cast::<u8>());
        } else {
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0u8, (&raw mut gText_Lady).cast::<u8>());
        }
        DynamicPlaceholderTextUtil_ExpandPlaceholders(dst, (&raw mut gText_F700sQuiz).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn BufferCurrentPhraseToStringVar2() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut phrase: *mut u16 = core::ptr::null_mut();
        let mut str: *mut u8 = core::ptr::null_mut();
        phrase = ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(60))
        .cast::<u16>();
        str = (&raw mut gStringVar2).cast::<u8>();
        i = 0i32;
        'l1: loop {
            if !(i
                < ((((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .read()) as i32))
            {
                break 'l1;
            }
            str = CopyEasyChatWordPadded(str, (phrase).read(), 0u16);
            (str).write(0u8);
            str = (str).wrapping_offset(1);
            phrase = (phrase).wrapping_offset(1);
            i = (i).wrapping_add(1);
        }
        str = (str).wrapping_offset(-1);
        (str).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn SetSpecialEasyChatResult() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                .read()) as i32);
            if __sw1 == 0i32 {
                FlagSet(2149u16);
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (DidPlayerInputMysteryGiftPhrase()) != 0 {
                    ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(2u16);
                } else {
                    ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                BufferCurrentPhraseToStringVar2();
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
                    ((TrySetTrendyPhrase(
                        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(60))
                        .cast::<u16>(),
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 13i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>())
                    .write(DidPlayerInputABerryMasterWifePhrase());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DidPlayerInputMysteryGiftPhrase() -> i32 {
    unsafe {
        return ((!((IsPhraseDifferentThanPlayerInput(
            ((&raw const sMysteryGiftPhrase)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>(),
            ((crate::c::div_u32(8u32, 2u32)) as u8),
        )) != 0)) as i32);
    }
}
pub(crate) unsafe extern "C" fn DidPlayerInputABerryMasterWifePhrase() -> u16 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(20u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((IsPhraseDifferentThanPlayerInput(
                        ((((&raw const sBerryMasterWifePhrases).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .cast::<u16>(),
                        ((crate::c::div_u32(4u32, 2u32)) as u8),
                    )) != 0)
                    {
                        return (((i).wrapping_add(1i32)) as u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn ClearUnusedField() {
    unsafe {
        ((((&raw mut sEasyChatScreen).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(19))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DummyWordCheck(easyChatWord: i32) -> u32 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenControl() -> u8 {
    unsafe {
        if !((InitEasyChatScreenControl_()) != 0) {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn LoadEasyChatScreen() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sEasyChatBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    3u8,
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2816))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    1u8,
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(768))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                InitWindows(
                    ((&raw const sEasyChatWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                LoadEasyChatPalettes();
                InitEasyChatBgs();
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l2: loop {
                        'l3: {
                            CpuFastSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((117440512i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    3u8,
                    (((&raw mut gEasyChatWindow_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    (((&raw mut gEasyChatWindow_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                AdjustBgTilemapForFooter();
                BufferFrameTilemap(
                    ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(768))
                    .cast::<u16>(),
                );
                AddPhraseWindow();
                AddMainScreenButtonWindow();
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                DecompressAndLoadBgGfxUsingHeap(
                    1u8,
                    (((&raw const sTextInputFrame_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                CopyBgTilemapBufferToVram(1u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                PrintTitle();
                PrintInitialInstructions();
                PrintCurrentPhrase();
                DrawLowerWindow();
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadEasyChatGfx();
                if ((GetEasyChatScreenType()) as i32) != 16i32 {
                    CreateMainCursorSprite();
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 1u8;
                } else {
                    SetWindowDimensions(0u8, 0u8, 0u8, 0u8);
                    SetGpuReg(72u8, 63u16);
                    SetGpuReg(74u8, 59u16);
                    ShowBg(3u8);
                    ShowBg(1u8);
                    ShowBg(2u8);
                    ShowBg(0u8);
                    CreateScrollIndicatorSprites();
                    CreateStartSelectButtonSprites();
                    TryAddInterviewObjectEvents();
                }
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        let __p2 =
            (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenControl() {
    unsafe {
        if ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize
        {
            Free(((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn StartEasyChatFunction(funcId: u16) {
    unsafe {
        let mut funcId = funcId;
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(funcId);
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
            .write(0u16);
        RunEasyChatFunction();
    }
}
pub(crate) unsafe extern "C" fn RunEasyChatFunction() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
                || __sw1 == 28i32
                || __sw1 == 29i32
                || __sw1 == 30i32
                || __sw1 == 31i32
                || __sw1 == 32i32
                || __sw1 == 33i32
                || __sw1 == 34i32;
            if __sw1 == 0i32 {
                return 0u8;
            }
            if __sw1 == 1i32 {
                return ReprintPhrase();
            }
            if __sw1 == 2i32 {
                return UpdateMainCursor();
            }
            if __sw1 == 3i32 {
                return UpdateMainCursorOnButtons();
            }
            if __sw1 == 4i32 {
                return ShowConfirmDeleteAllPrompt();
            }
            if __sw1 == 5i32 {
                return ShowConfirmExitPrompt();
            }
            if __sw1 == 6i32 {
                return ShowConfirmPrompt();
            }
            if __sw1 == 7i32 {
                return ClosePrompt();
            }
            if __sw1 == 8i32 {
                return ClosePromptAfterDeleteAll();
            }
            if __sw1 == 9i32 {
                return OpenKeyboard();
            }
            if __sw1 == 10i32 {
                return CloseKeyboard();
            }
            if __sw1 == 11i32 {
                return OpenWordSelect();
            }
            if __sw1 == 12i32 {
                return CloseWordSelect();
            }
            if __sw1 == 13i32 {
                return ShowConfirmLyricsPrompt();
            }
            if __sw1 == 14i32 {
                return ReturnToKeyboard();
            }
            if __sw1 == 15i32 {
                return UpdateKeyboardCursor();
            }
            if __sw1 == 16i32 {
                return GroupNamesScrollDown();
            }
            if __sw1 == 17i32 {
                return GroupNamesScrollUp();
            }
            if __sw1 == 18i32 {
                return UpdateWordSelectCursor();
            }
            if __sw1 == 19i32 {
                return WordSelectScrollUp();
            }
            if __sw1 == 20i32 {
                return WordSelectScrollDown();
            }
            if __sw1 == 21i32 {
                return WordSelectPageScrollUp();
            }
            if __sw1 == 22i32 {
                return WordSelectPageScrollDown();
            }
            if __sw1 == 23i32 {
                return SwitchKeyboardMode();
            }
            if __sw1 == 24i32 {
                return 0u8;
            }
            if __sw1 == 25i32 {
                return 0u8;
            }
            if __sw1 == 26i32 {
                return 0u8;
            }
            if __sw1 == 27i32 {
                return 0u8;
            }
            if __sw1 == 28i32 {
                return 0u8;
            }
            if __sw1 == 29i32 {
                return ShowCreateQuizMsg();
            }
            if __sw1 == 30i32 {
                return ShowSelectAnswerMsg();
            }
            if __sw1 == 31i32 {
                return ShowSongTooShortMsg();
            }
            if __sw1 == 32i32 {
                return ShowCantDeleteLyricsMsg();
            }
            if __sw1 == 33i32 {
                return ShowCombineTwoWordsMsg();
            }
            if __sw1 == 34i32 {
                return ShowCantExitMsg();
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn ReprintPhrase() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintCurrentPhrase();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateMainCursor() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut currentPhrase: *mut u16 = core::ptr::null_mut();
        let mut ecWord: *mut u16 = core::ptr::null_mut();
        let mut frameId: u8 = 0u8;
        let mut cursorColumn: u8 = 0u8;
        let mut cursorRow: u8 = 0u8;
        let mut numColumns: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut stringWidth: i32 = 0i32;
        let mut trueStringWidth: i32 = 0i32;
        let mut y: u8 = 0u8;
        let mut str = crate::ffi::Align4([0u8; 64]);
        currentPhrase = GetCurrentPhrase();
        frameId = GetEasyChatScreenFrameId();
        cursorColumn = GetMainCursorColumn();
        cursorRow = GetMainCursorRow();
        numColumns = GetNumColumns();
        ecWord = (currentPhrase)
            .wrapping_offset((((cursorRow) as i32).wrapping_mul(((numColumns) as i32))) as isize);
        x = ((((8i32).wrapping_mul(
            ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32),
        ))
        .wrapping_add(13i32)) as i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((cursorColumn) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((ecWord).read()) as i32) == 65535i32 {
                        stringWidth = 72i32;
                    } else {
                        CopyEasyChatWord((&raw mut str).cast::<u8>(), (ecWord).read());
                        stringWidth = GetStringWidth(1u8, (&raw mut str).cast::<u8>(), 0i16);
                    }
                    trueStringWidth = (stringWidth).wrapping_add(17i32);
                    x = ((((x) as i32).wrapping_add(trueStringWidth)) as i16);
                    ecWord = (ecWord).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        y = (((8i32).wrapping_mul(
            ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                5,
                3,
                false,
            ) as u8) as i32)
                .wrapping_add(((cursorRow) as i32).wrapping_mul(2i32)),
        )) as u8);
        SetMainCursorPos(((x) as u8), ((((y) as i32).wrapping_add(8i32)) as u8));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateMainCursorOnButtons() -> u8 {
    unsafe {
        let mut xOffset: u8 = ((GetFooterOptionXOffset(((GetMainCursorColumn()) as i32))) as u8);
        SetMainCursorPos(xOffset, 96u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShowConfirmExitPrompt() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(2u8);
                CreateEasyChatYesNoMenu(1u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowConfirmPrompt() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(3u8);
                CreateEasyChatYesNoMenu(0u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowConfirmDeleteAllPrompt() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(1u8);
                CreateEasyChatYesNoMenu(1u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ClosePrompt() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StartMainCursorAnim();
                PrintEasyChatStdMessage(0u8);
                PrintCurrentPhrase();
                ShowBg(0u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ClosePromptAfterDeleteAll() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                StartMainCursorAnim();
                PrintEasyChatStdMessage(0u8);
                PrintCurrentPhrase();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn OpenKeyboard() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                HideBg(0u8);
                SetWindowDimensions(0u8, 0u8, 0u8, 0u8);
                PrintKeyboardText();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(0i32);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (!((IsDma3ManagerBusyWithBgCopy()) != 0)) && (!((UpdateLowerWindowAnim()) != 0))
                {
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateSideWindowSprites();
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((ShowSideWindow()) != 0) {
                    CreateRectangleCursorSprites();
                    SetScrollIndicatorXPos(0u32);
                    UpdateScrollIndicatorsVisibility();
                    let __p6 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CloseKeyboard() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                DestroyRectangleCursorSprites();
                HideModeWindow();
                HideScrollIndicators();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if ((DestroySideWindowSprites()) as i32) == 1i32 {
                    break 'l1;
                }
                InitLowerWindowAnim(1i32);
                let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if !((UpdateLowerWindowAnim()) != 0) {
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    StartMainCursorAnim();
                    ShowBg(0u8);
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SwitchKeyboardMode() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                DestroyRectangleCursorSprites();
                HideScrollIndicators();
                SetModeWindowToTransition();
                InitLowerWindowAnim(5i32);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((UpdateLowerWindowAnim()) != 0)) && (!((IsModeWindowAnimActive()) != 0)) {
                    PrintKeyboardText();
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(6i32);
                    UpdateModeWindowAnim();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (!((UpdateLowerWindowAnim()) != 0)) && (!((IsModeWindowAnimActive()) != 0)) {
                    UpdateScrollIndicatorsVisibility();
                    CreateRectangleCursorSprites();
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateKeyboardCursor() -> u8 {
    unsafe {
        UpdateRectangleCursorPos();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GroupNamesScrollDown() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                InitLowerWindowScroll(1i16, 4u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateRectangleCursorPos();
                    UpdateScrollIndicatorsVisibility();
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GroupNamesScrollUp() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                InitLowerWindowScroll((-1i16), 4u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateScrollIndicatorsVisibility();
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn OpenWordSelect() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                DestroyRectangleCursorSprites();
                HideModeWindow();
                HideScrollIndicators();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((DestroySideWindowSprites()) != 0) {
                    ClearWordSelectWindow();
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(2i32);
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((UpdateLowerWindowAnim()) != 0) {
                    InitLowerWindowText(2u32);
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateWordSelectCursorSprite();
                    SetScrollIndicatorXPos(1u32);
                    UpdateScrollIndicatorsVisibility();
                    UpdateStartSelectButtonsVisibility();
                    let __p6 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CloseWordSelect() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintCurrentPhrase();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                DestroyWordSelectCursorSprite();
                HideScrollIndicators();
                HideStartSelectButtons();
                ClearWordSelectWindow();
                let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(3i32);
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((UpdateLowerWindowAnim()) != 0) {
                    ShowBg(0u8);
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    StartMainCursorAnim();
                    let __p6 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowConfirmLyricsPrompt() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintCurrentPhrase();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                DestroyWordSelectCursorSprite();
                HideScrollIndicators();
                HideStartSelectButtons();
                ClearWordSelectWindow();
                let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(3i32);
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((UpdateLowerWindowAnim()) != 0) {
                    PrintEasyChatStdMessage(3u8);
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    ShowBg(0u8);
                    let __p6 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    StartMainCursorAnim();
                    let __p7 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ReturnToKeyboard() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                DestroyWordSelectCursorSprite();
                HideScrollIndicators();
                HideStartSelectButtons();
                ClearWordSelectWindow();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowAnim(4i32);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateLowerWindowAnim()) != 0) {
                    PrintKeyboardText();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    CreateSideWindowSprites();
                    let __p5 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((ShowSideWindow()) != 0) {
                    CreateRectangleCursorSprites();
                    SetScrollIndicatorXPos(0u32);
                    UpdateScrollIndicatorsVisibility();
                    let __p6 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateWordSelectCursor() -> u8 {
    unsafe {
        UpdateWordSelectCursorPos();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WordSelectScrollDown() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintWordSelectNextRowDown();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowScroll(1i16, 4u8);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateWordSelectCursorPos();
                    UpdateScrollIndicatorsVisibility();
                    UpdateStartSelectButtonsVisibility();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WordSelectScrollUp() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintWordSelectNextRowUp();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    InitLowerWindowScroll((-1i16), 4u8);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateScrollIndicatorsVisibility();
                    UpdateStartSelectButtonsVisibility();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WordSelectPageScrollDown() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintWordSelectRowsPageDown();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    let mut scrollChange: i16 = ((((GetWordSelectScrollOffset()) as i32)
                        .wrapping_sub(GetLowerWindowScrollOffset()))
                        as i16);
                    InitLowerWindowScroll(scrollChange, 8u8);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateWordSelectCursorPos();
                    UpdateScrollIndicatorsVisibility();
                    UpdateStartSelectButtonsVisibility();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WordSelectPageScrollUp() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrintWordSelectRowsPageUp();
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    let mut scrollChange: i16 = ((((GetWordSelectScrollOffset()) as i32)
                        .wrapping_sub(GetLowerWindowScrollOffset()))
                        as i16);
                    InitLowerWindowScroll(scrollChange, 8u8);
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((UpdateLowerWindowScroll()) != 0) {
                    UpdateScrollIndicatorsVisibility();
                    UpdateStartSelectButtonsVisibility();
                    let __p4 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowCreateQuizMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(4u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowSelectAnswerMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(5u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowSongTooShortMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(6u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowCantDeleteLyricsMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(7u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowCombineTwoWordsMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(8u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShowCantExitMsg() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                StopMainCursorAnim();
                PrintEasyChatStdMessage(9u8);
                let __p2 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                return IsDma3ManagerBusyWithBgCopy();
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenControl_() -> u8 {
    unsafe {
        ((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).write(Alloc(4864u32));
        if !(!(((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return 0u8;
        }
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
            .write(0u16);
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(736)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(740)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(744)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(748)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(752)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(756)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(760)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(764)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(((FooterHasFourOptions_()) as u8));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatBgs() {
    unsafe {
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        SetGpuReg(0u8, 12352u16);
    }
}
pub(crate) unsafe extern "C" fn LoadEasyChatPalettes() {
    unsafe {
        ResetPaletteFade();
        LoadPalette(
            (((&raw mut gEasyChatMode_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            0u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTextInputFrameOrange_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            16u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTextInputFrameGreen_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            64u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTitleText_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            160u16,
            8u16,
        );
        LoadPalette(
            (((&raw const sText_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .cast::<u8>(),
            176u16,
            12u16,
        );
        LoadPalette(
            (((&raw const sText_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .cast::<u8>(),
            240u16,
            12u16,
        );
        LoadPalette(
            (((&raw const sText_Pal).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                .cast::<u8>(),
            48u16,
            12u16,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintTitle() {
    unsafe {
        let mut xOffset: i32 = 0i32;
        let mut titleText: *mut u8 = GetTitleText();
        if !(!(titleText).is_null()) {
            return;
        }
        xOffset = GetStringCenterAlignXOffset(1i32, titleText, 144i32);
        FillWindowPixelBuffer(0u8, 0u8);
        PrintEasyChatTextWithColors(
            0u8,
            1u8,
            titleText,
            ((xOffset) as u8),
            1u8,
            255u8,
            0u8,
            2u8,
            3u8,
        );
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintEasyChatText(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    speed: u8,
    callback: Option<unsafe extern "C" fn(*mut u8, u16)>,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut speed = speed;
        let mut callback = callback;
        AddTextPrinterParameterized(windowId, fontId, str, x, y, speed, callback);
    }
}
pub(crate) unsafe extern "C" fn PrintEasyChatTextWithColors(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
    speed: u8,
    bg: u8,
    fg: u8,
    shadow: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut speed = speed;
        let mut bg = bg;
        let mut fg = fg;
        let mut shadow = shadow;
        let mut color = crate::ffi::Align4([0u8; 3]);
        ((&raw mut color).cast::<u8>()).write(bg);
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(fg);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(shadow);
        AddTextPrinterParameterized3(
            windowId,
            fontId,
            left,
            top,
            (&raw mut color).cast::<u8>(),
            ((speed) as i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintInitialInstructions() {
    unsafe {
        FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 20u8, 17u8);
        LoadUserWindowBorderGfx(1u8, 1u16, 224u8);
        DrawTextBorderOuter(1u8, 1u16, 14u8);
        PrintEasyChatStdMessage(0u8);
        PutWindowTilemap(1u8);
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintEasyChatStdMessage(msgId: u8) {
    unsafe {
        let mut msgId = msgId;
        let mut text2: *mut u8 = core::ptr::null_mut();
        let mut text1: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((msgId) as i32);
            if __sw1 == 0i32 {
                GetEasyChatInstructionsText(&raw mut text1, &raw mut text2);
                break 'l1;
            }
            if __sw1 == 2i32 {
                GetEasyChatConfirmExitText(&raw mut text1, &raw mut text2);
                break 'l1;
            }
            if __sw1 == 3i32 {
                GetEasyChatConfirmText(&raw mut text1, &raw mut text2);
                break 'l1;
            }
            if __sw1 == 1i32 {
                GetEasyChatConfirmDeletionText(&raw mut text1, &raw mut text2);
                break 'l1;
            }
            if __sw1 == 4i32 {
                text1 = (&raw mut gText_CreateAQuiz).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 5i32 {
                text1 = (&raw mut gText_SelectTheAnswer).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 6i32 {
                text1 = (&raw mut gText_OnlyOnePhrase).cast::<u8>();
                text2 = (&raw mut gText_OriginalSongWillBeUsed).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 7i32 {
                text1 = (&raw mut gText_LyricsCantBeDeleted).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 8i32 {
                text1 = (&raw mut gText_CombineTwoWordsOrPhrases3).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 9i32 {
                text1 = (&raw mut gText_YouCannotQuitHere).cast::<u8>();
                text2 = (&raw mut gText_SectionMustBeCompleted).cast::<u8>();
                break 'l1;
            }
        }
        FillWindowPixelBuffer(1u8, 17u8);
        if !(text1).is_null() {
            PrintEasyChatText(1u8, 1u8, text1, 0u8, 1u8, 255u8, None);
        }
        if !(text2).is_null() {
            PrintEasyChatText(1u8, 1u8, text2, 0u8, 17u8, 255u8, None);
        }
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn CreateEasyChatYesNoMenu(initialCursorPos: u8) {
    unsafe {
        let mut initialCursorPos = initialCursorPos;
        CreateYesNoMenu(
            (&raw const sEasyChatYesNoWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
            1u16,
            14u8,
            initialCursorPos,
        );
    }
}
pub(crate) unsafe extern "C" fn AddPhraseWindow() {
    unsafe {
        let mut frameId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 8]);
        frameId = GetEasyChatScreenFrameId();
        ((&raw mut template).cast::<u8>()).write(3u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(
            (crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8),
        );
        (((&raw mut template).cast::<u8>()).wrapping_add(2)).write(
            (crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                5,
                3,
                false,
            ) as u8),
        );
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(
            (((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(1))
            .read(),
        );
        (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(
            (((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(2))
            .read(),
        );
        (((&raw mut template).cast::<u8>()).wrapping_add(5)).write(11u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(108u16);
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(AddWindow((&raw mut template).cast::<u8>()));
        PutWindowTilemap(
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintCurrentPhrase() {
    unsafe {
        let mut strClear = crate::ffi::Align4([0u8; 4]);
        let mut currentPhrase: *mut u16 = core::ptr::null_mut();
        let mut numColumns: u8 = 0u8;
        let mut numRows: u8 = 0u8;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut frameId: i32 = 0i32;
        let mut isQuizQuestion: u32 = 0u32;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        currentPhrase = GetCurrentPhrase();
        numColumns = GetNumColumns();
        numRows = GetNumRows();
        frameId = ((GetEasyChatScreenFrameId()) as i32);
        isQuizQuestion = 0u32;
        if frameId == 7i32 {
            isQuizQuestion = 1u32;
        }
        FillWindowPixelBuffer(
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as u8),
            17u8,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numRows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::memcpy(
                        (&raw mut strClear).cast::<u8>(),
                        ((&raw const sText_Clear17).cast::<u8>().cast_mut()).cast::<u8>(),
                        4u32,
                    );
                    if (isQuizQuestion) != 0 {
                        (((&raw mut strClear).cast::<u8>()).wrapping_offset(2)).write(6u8);
                    }
                    str = ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11))
                    .cast::<u8>();
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(11))
                    .cast::<u8>())
                    .write(255u8);
                    str = StringAppend(str, (&raw mut strClear).cast::<u8>());
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < ((numColumns) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if (((currentPhrase).read()) as i32) != 65535i32 {
                                    str = CopyEasyChatWord(str, (currentPhrase).read());
                                    currentPhrase = (currentPhrase).wrapping_offset(1);
                                } else {
                                    currentPhrase = (currentPhrase).wrapping_offset(1);
                                    if !((isQuizQuestion) != 0) {
                                        str = WriteColorChangeControlCode(str, 0u32, 4u8);
                                        {
                                            k = 0i32;
                                            'l5: loop {
                                                if !(k < 12i32) {
                                                    break 'l5;
                                                }
                                                'l6: {
                                                    (str).write(174u8);
                                                    str = (str).wrapping_offset(1);
                                                }
                                                k = (k).wrapping_add(1);
                                            }
                                        }
                                        str = WriteColorChangeControlCode(str, 0u32, 2u8);
                                    }
                                }
                                if (isQuizQuestion) != 0 {
                                    (((&raw mut strClear).cast::<u8>()).wrapping_offset(2))
                                        .write(3u8);
                                }
                                str = StringAppend(str, (&raw mut strClear).cast::<u8>());
                                if ((frameId == 2i32) || (frameId == 7i32)) || (frameId == 8i32) {
                                    if (j == 0i32) && (i == 4i32) {
                                        break 'l3;
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    (str).write(255u8);
                    PrintEasyChatText(
                        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as u8),
                        1u8,
                        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(11))
                        .cast::<u8>(),
                        0u8,
                        ((((i).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as u8),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferFrameTilemap(tilemap: *mut u16) {
    unsafe {
        let mut tilemap = tilemap;
        let mut frameId: u8 = 0u8;
        let mut right: i32 = 0i32;
        let mut bottom: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        frameId = GetEasyChatScreenFrameId();
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (tilemap).cast::<u8>(),
                        ((16777216i32
                            | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        if (((frameId) as i32) == 2i32) || (((frameId) as i32) == 8i32) {
            right = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32)
                .wrapping_add(
                    (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                    .wrapping_add(1))
                    .read()) as i32),
                );
            bottom = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                5,
                3,
                false,
            ) as u8) as i32)
                .wrapping_add(
                    (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                    .wrapping_add(2))
                    .read()) as i32),
                );
            {
                y = ((crate::c::bf_read(
                    ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((frameId) as i32) as isize * 4))
                    .wrapping_add(0),
                    5,
                    3,
                    false,
                ) as u8) as i32);
                'l3: loop {
                    if !(y < bottom) {
                        break 'l3;
                    }
                    'l4: {
                        x = ((crate::c::bf_read(
                            ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((frameId) as i32) as isize * 4))
                            .wrapping_add(0),
                            0,
                            5,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32);
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4101u16);
                        x = (x).wrapping_add(1);
                        {
                            'l5: loop {
                                if !(x < right) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((tilemap).wrapping_offset(
                                        (((y).wrapping_mul(32i32)).wrapping_add(x)) as isize,
                                    ))
                                    .write(4096u16);
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4103u16);
                    }
                    y = (y).wrapping_add(1);
                }
            }
        } else {
            y = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                5,
                3,
                false,
            ) as u8) as i32)
                .wrapping_sub(1i32);
            x = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32)
                .wrapping_sub(1i32);
            right = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32)
                .wrapping_add(
                    (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                    .wrapping_add(1))
                    .read()) as i32),
                );
            bottom = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                5,
                3,
                false,
            ) as u8) as i32)
                .wrapping_add(
                    (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                    .wrapping_add(2))
                    .read()) as i32),
                );
            ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                .write(4097u16);
            x = (x).wrapping_add(1);
            {
                'l7: loop {
                    if !(x < right) {
                        break 'l7;
                    }
                    'l8: {
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4098u16);
                    }
                    x = (x).wrapping_add(1);
                }
            }
            ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                .write(4099u16);
            y = (y).wrapping_add(1);
            {
                'l9: loop {
                    if !(y < bottom) {
                        break 'l9;
                    }
                    'l10: {
                        x = ((crate::c::bf_read(
                            ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((frameId) as i32) as isize * 4))
                            .wrapping_add(0),
                            0,
                            5,
                            false,
                        ) as u8) as i32)
                            .wrapping_sub(1i32);
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4101u16);
                        x = (x).wrapping_add(1);
                        {
                            'l11: loop {
                                if !(x < right) {
                                    break 'l11;
                                }
                                'l12: {
                                    ((tilemap).wrapping_offset(
                                        (((y).wrapping_mul(32i32)).wrapping_add(x)) as isize,
                                    ))
                                    .write(4096u16);
                                }
                                x = (x).wrapping_add(1);
                            }
                        }
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4103u16);
                    }
                    y = (y).wrapping_add(1);
                }
            }
            x = ((crate::c::bf_read(
                ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((frameId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                5,
                false,
            ) as u8) as i32)
                .wrapping_sub(1i32);
            ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                .write(4105u16);
            x = (x).wrapping_add(1);
            {
                'l13: loop {
                    if !(x < right) {
                        break 'l13;
                    }
                    'l14: {
                        ((tilemap)
                            .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                        .write(4106u16);
                    }
                    x = (x).wrapping_add(1);
                }
            }
            ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                .write(4107u16);
        }
    }
}
pub(crate) unsafe extern "C" fn AdjustBgTilemapForFooter() {
    unsafe {
        let mut frameId: u8 = 0u8;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        tilemap = (GetBgTilemapBuffer(3u8)).cast::<u16>();
        frameId = GetEasyChatScreenFrameId();
        'l1: {
            let __sw1 = (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(3))
            .read()) as i32);
            if __sw1 == 2i32 {
                tilemap = (tilemap).wrapping_offset(672);
                CopyToBgTilemapBufferRect(3u8, (tilemap).cast::<u8>(), 0u8, 11u8, 32u8, 2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                tilemap = (tilemap).wrapping_offset(768);
                CopyToBgTilemapBufferRect(3u8, (tilemap).cast::<u8>(), 0u8, 11u8, 32u8, 2u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                CopyToBgTilemapBufferRect(3u8, (tilemap).cast::<u8>(), 0u8, 10u8, 32u8, 4u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawLowerWindow() {
    unsafe {
        PutWindowTilemap(2u8);
        CopyBgTilemapBufferToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn InitLowerWindowText(whichText: u32) {
    unsafe {
        let mut whichText = whichText;
        ResetLowerWindowScroll();
        FillWindowPixelBuffer(2u8, 17u8);
        'l1: {
            let __sw1 = whichText;
            if __sw1 == 0u32 {
                PrintKeyboardGroupNames();
                break 'l1;
            }
            if __sw1 == 1u32 {
                PrintKeyboardAlphabet();
                break 'l1;
            }
            if __sw1 == 2u32 {
                PrintInitialWordSelectText();
                break 'l1;
            }
        }
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardText() {
    unsafe {
        if !((GetInAlphabetMode()) != 0) {
            InitLowerWindowText(0u32);
        } else {
            InitLowerWindowText(1u32);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardGroupNames() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        i = 0i32;
        y = 97i32;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            {
                x = 0i32;
                'l2: loop {
                    if !(x < 2i32) {
                        break 'l2;
                    }
                    'l3: {
                        let mut groupId: u8 = GetUnlockedEasyChatGroupId(
                            (({
                                let __t1 = i;
                                i = (i).wrapping_add(1);
                                __t1
                            }) as u8),
                        );
                        if ((groupId) as i32) == 22i32 {
                            InitLowerWindowScroll(((GetKeyboardScrollOffset()) as i16), 0u8);
                            return;
                        }
                        PrintEasyChatText(
                            2u8,
                            1u8,
                            GetEasyChatWordGroupName(groupId),
                            ((((x).wrapping_mul(84i32)).wrapping_add(10i32)) as u8),
                            ((y) as u8),
                            255u8,
                            None,
                        );
                    }
                    x = (x).wrapping_add(1);
                }
            }
            y = (y).wrapping_add(16i32);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardAlphabet() {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(16u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    PrintEasyChatText(
                        2u8,
                        1u8,
                        ((((&raw const sEasyChatKeyboardAlphabet)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        10u8,
                        (((97u32).wrapping_add((i).wrapping_mul(16u32))) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintInitialWordSelectText() {
    unsafe {
        PrintWordSelectText(0u8, 4u8);
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectNextRowDown() {
    unsafe {
        let mut wordScroll: u8 = (((((GetWordSelectScrollOffset()) as i32).wrapping_add(4i32))
            .wrapping_sub(1i32)) as u8);
        EraseWordSelectRows(wordScroll, 1u8);
        PrintWordSelectText(wordScroll, 1u8);
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectNextRowUp() {
    unsafe {
        let mut wordScroll: u8 = GetWordSelectScrollOffset();
        EraseWordSelectRows(wordScroll, 1u8);
        PrintWordSelectText(wordScroll, 1u8);
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectRowsPageDown() {
    unsafe {
        let mut wordScroll: u8 = GetWordSelectScrollOffset();
        let mut maxScroll: u8 = ((((wordScroll) as i32).wrapping_add(4i32)) as u8);
        let mut maxRows: u8 = ((((GetWordSelectLastRow()) as i32).wrapping_add(1i32)) as u8);
        if ((maxScroll) as i32) > ((maxRows) as i32) {
            maxScroll = maxRows;
        }
        if ((wordScroll) as i32) < ((maxScroll) as i32) {
            let mut numRows: u8 =
                ((((maxScroll) as i32).wrapping_sub(((wordScroll) as i32))) as u8);
            EraseWordSelectRows(wordScroll, numRows);
            PrintWordSelectText(wordScroll, numRows);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectRowsPageUp() {
    unsafe {
        let mut wordScroll: u8 = GetWordSelectScrollOffset();
        let mut windowScroll: u8 = ((GetLowerWindowScrollOffset()) as u8);
        if ((wordScroll) as i32) < ((windowScroll) as i32) {
            let mut numRows: u8 =
                ((((windowScroll) as i32).wrapping_sub(((wordScroll) as i32))) as u8);
            EraseWordSelectRows(wordScroll, numRows);
            PrintWordSelectText(wordScroll, numRows);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectText(scrollOffset: u8, numRows: u8) {
    unsafe {
        let mut scrollOffset = scrollOffset;
        let mut numRows = numRows;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut easyChatWord: u16 = 0u16;
        let mut y: i32 = 0i32;
        let mut wordIndex: i32 = 0i32;
        wordIndex = ((scrollOffset) as i32).wrapping_mul(2i32);
        y = ((((scrollOffset) as i32).wrapping_mul(16i32)).wrapping_add(96i32) & 255i32);
        y = (y).wrapping_add(1);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((numRows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                easyChatWord = GetWordFromSelectedGroup(
                                    (({
                                        let __t1 = wordIndex;
                                        wordIndex = (wordIndex).wrapping_add(1);
                                        __t1
                                    }) as u16),
                                );
                                if ((easyChatWord) as i32) != 65535i32 {
                                    CopyEasyChatWordPadded(
                                        ((((&raw mut sScreenControl)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(204))
                                        .cast::<u8>(),
                                        easyChatWord,
                                        0u16,
                                    );
                                    if !((DummyWordCheck(((easyChatWord) as i32))) != 0) {
                                        PrintEasyChatText(
                                            2u8,
                                            1u8,
                                            ((((&raw mut sScreenControl)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(204))
                                            .cast::<u8>(),
                                            (((((j).wrapping_mul(13i32)).wrapping_add(3i32))
                                                .wrapping_mul(8i32))
                                                as u8),
                                            ((y) as u8),
                                            255u8,
                                            None,
                                        );
                                    } else {
                                        PrintEasyChatTextWithColors(
                                            2u8,
                                            1u8,
                                            ((((&raw mut sScreenControl)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(204))
                                            .cast::<u8>(),
                                            (((((j).wrapping_mul(13i32)).wrapping_add(3i32))
                                                .wrapping_mul(8i32))
                                                as u8),
                                            ((y) as u8),
                                            255u8,
                                            1u8,
                                            5u8,
                                            3u8,
                                        );
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    y = (y).wrapping_add(16i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn EraseWordSelectRows(scrollOffset: u8, numRows: u8) {
    unsafe {
        let mut scrollOffset = scrollOffset;
        let mut numRows = numRows;
        let mut y: i32 = 0i32;
        let mut var0: i32 = 0i32;
        let mut var1: i32 = 0i32;
        let mut var2: i32 = 0i32;
        y = ((((scrollOffset) as i32).wrapping_mul(16i32)).wrapping_add(96i32) & 255i32);
        var2 = ((numRows) as i32).wrapping_mul(16i32);
        var0 = (y).wrapping_add(var2);
        if var0 > 255i32 {
            var1 = (var0).wrapping_sub(256i32);
            var2 = (256i32).wrapping_sub(y);
        } else {
            var1 = 0i32;
        }
        FillWindowPixelRect(2u8, 17u8, 0u16, ((y) as u16), 224u16, ((var2) as u16));
        if (var1) != 0 {
            FillWindowPixelRect(2u8, 17u8, 0u16, 0u16, 224u16, ((var1) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn ClearWordSelectWindow() {
    unsafe {
        FillWindowPixelBuffer(2u8, 17u8);
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn InitLowerWindowAnim(winAnimType: i32) {
    unsafe {
        let mut winAnimType = winAnimType;
        'l1: {
            let __sw1 = winAnimType;
            if __sw1 == 0i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(0u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(10u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(9u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(11u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(17u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(17u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(0u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(17u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(10u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(18u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(22u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .write(22u8);
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .write(18u8);
                break 'l1;
            }
        }
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<i8>())
        .write(
            ((if ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6))
            .read()) as i32)
                < ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(7))
                .read()) as i32)
            {
                1i32
            } else {
                (-1i32)
            }) as i8),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateLowerWindowAnim() -> u8 {
    unsafe {
        let mut curState: u8 = 0u8;
        let mut destState: u8 = 0u8;
        if ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .read()) as i32)
            == ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(7))
            .read()) as i32)
        {
            return 0u8;
        }
        let __p1 =
            (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<i8>())
                .read()) as i32),
            )) as u8),
        );
        DrawLowerWindowFrame(
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .read(),
        );
        curState = ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6))
        .read();
        destState = ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(7))
        .read();
        return (((((curState) as i32) ^ ((destState) as i32)) > 0i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn DrawLowerWindowFrame(r#type: u8) {
    unsafe {
        let mut r#type = r#type;
        FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 10u8, 30u8, 10u8);
        'l1: {
            let __sw1 = ((r#type) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                BufferLowerWindowFrame(11i32, 14i32, 3i32, 2i32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                BufferLowerWindowFrame(9i32, 14i32, 7i32, 2i32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                BufferLowerWindowFrame(7i32, 14i32, 11i32, 2i32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                BufferLowerWindowFrame(5i32, 14i32, 15i32, 2i32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                BufferLowerWindowFrame(3i32, 14i32, 19i32, 2i32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                BufferLowerWindowFrame(1i32, 14i32, 23i32, 2i32);
                break 'l1;
            }
            if __sw1 == 7i32 {
                BufferLowerWindowFrame(1i32, 13i32, 23i32, 4i32);
                break 'l1;
            }
            if __sw1 == 8i32 {
                BufferLowerWindowFrame(1i32, 12i32, 23i32, 6i32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                BufferLowerWindowFrame(1i32, 11i32, 23i32, 8i32);
                break 'l1;
            }
            if __sw1 == 10i32 {
                BufferLowerWindowFrame(1i32, 10i32, 23i32, 10i32);
                break 'l1;
            }
            if __sw1 == 11i32 {
                BufferLowerWindowFrame(1i32, 10i32, 24i32, 10i32);
                break 'l1;
            }
            if __sw1 == 12i32 {
                BufferLowerWindowFrame(1i32, 10i32, 25i32, 10i32);
                break 'l1;
            }
            if __sw1 == 13i32 {
                BufferLowerWindowFrame(1i32, 10i32, 26i32, 10i32);
                break 'l1;
            }
            if __sw1 == 14i32 {
                BufferLowerWindowFrame(1i32, 10i32, 27i32, 10i32);
                break 'l1;
            }
            if __sw1 == 15i32 {
                BufferLowerWindowFrame(1i32, 10i32, 28i32, 10i32);
                break 'l1;
            }
            if __sw1 == 16i32 {
                BufferLowerWindowFrame(1i32, 10i32, 29i32, 10i32);
                break 'l1;
            }
            if __sw1 == 17i32 {
                BufferLowerWindowFrame(0i32, 10i32, 30i32, 10i32);
                break 'l1;
            }
            if __sw1 == 18i32 {
                BufferLowerWindowFrame(1i32, 10i32, 23i32, 10i32);
                break 'l1;
            }
            if __sw1 == 19i32 {
                BufferLowerWindowFrame(1i32, 11i32, 23i32, 8i32);
                break 'l1;
            }
            if __sw1 == 20i32 {
                BufferLowerWindowFrame(1i32, 12i32, 23i32, 6i32);
                break 'l1;
            }
            if __sw1 == 21i32 {
                BufferLowerWindowFrame(1i32, 13i32, 23i32, 4i32);
                break 'l1;
            }
            if __sw1 == 22i32 {
                BufferLowerWindowFrame(1i32, 14i32, 23i32, 2i32);
                break 'l1;
            }
        }
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn BufferLowerWindowFrame(
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut right: i32 = 0i32;
        let mut bottom: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        tilemap = ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(768))
        .cast::<u16>();
        right = ((left).wrapping_add(width)).wrapping_sub(1i32);
        bottom = ((top).wrapping_add(height)).wrapping_sub(1i32);
        x = left;
        y = top;
        ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
            .write(16385u16);
        x = (x).wrapping_add(1);
        {
            'l1: loop {
                if !(x < right) {
                    break 'l1;
                }
                'l2: {
                    ((tilemap)
                        .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                    .write(16386u16);
                }
                x = (x).wrapping_add(1);
            }
        }
        ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
            .write(16387u16);
        y = (y).wrapping_add(1);
        {
            'l3: loop {
                if !(y < bottom) {
                    break 'l3;
                }
                'l4: {
                    ((tilemap)
                        .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(left)) as isize))
                    .write(16389u16);
                    x = (left).wrapping_add(1i32);
                    {
                        'l5: loop {
                            if !(x < right) {
                                break 'l5;
                            }
                            'l6: {
                                ((tilemap).wrapping_offset(
                                    (((y).wrapping_mul(32i32)).wrapping_add(x)) as isize,
                                ))
                                .write(16384u16);
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                    ((tilemap)
                        .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                    .write(16391u16);
                }
                y = (y).wrapping_add(1);
            }
        }
        ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(left)) as isize))
            .write(16393u16);
        x = (left).wrapping_add(1i32);
        {
            'l7: loop {
                if !(x < right) {
                    break 'l7;
                }
                'l8: {
                    ((tilemap)
                        .wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
                    .write(16394u16);
                }
                x = (x).wrapping_add(1);
            }
        }
        ((tilemap).wrapping_offset((((y).wrapping_mul(32i32)).wrapping_add(x)) as isize))
            .write(16395u16);
        SetWindowDimensions(
            ((((left).wrapping_add(1i32)).wrapping_mul(8i32)) as u8),
            ((((top).wrapping_add(1i32)).wrapping_mul(8i32)) as u8),
            ((((width).wrapping_sub(2i32)).wrapping_mul(8i32)) as u8),
            ((((height).wrapping_sub(2i32)).wrapping_mul(8i32)) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn ResetLowerWindowScroll() {
    unsafe {
        ChangeBgY(2u8, 2048i32, 0u8);
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(718)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn InitLowerWindowScroll(scrollChange: i16, speed: u8) {
    unsafe {
        let mut scrollChange = scrollChange;
        let mut speed = speed;
        let mut bgY: i32 = 0i32;
        let mut yChange: i16 = 0i16;
        bgY = GetBgY(2u8);
        let __p1 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(718)
            .cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((scrollChange) as i32))) as u16));
        yChange = ((((scrollChange) as i32).wrapping_mul(16i32)) as i16);
        bgY = (bgY).wrapping_add(((yChange) as i32).wrapping_mul(256i32));
        if (speed) != 0 {
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(720)
                .cast::<i32>())
            .write(bgY);
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(724)
                .cast::<i32>())
            .write(((speed) as i32).wrapping_mul(256i32));
            if ((yChange) as i32) < 0i32 {
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(724)
                    .cast::<i32>())
                .write(
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(724)
                        .cast::<i32>())
                    .read())
                    .wrapping_neg(),
                );
            }
        } else {
            ChangeBgY(2u8, bgY, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateLowerWindowScroll() -> u8 {
    unsafe {
        let mut bgY: i32 = 0i32;
        bgY = GetBgY(2u8);
        if bgY
            == ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(720)
                .cast::<i32>())
            .read()
        {
            return 0u8;
        } else {
            ChangeBgY(
                2u8,
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(724)
                    .cast::<i32>())
                .read(),
                1u8,
            );
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetLowerWindowScrollOffset() -> i32 {
    unsafe {
        return ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(718)
            .cast::<u16>())
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn SetWindowDimensions(left: u8, top: u8, width: u8, height: u8) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut horizontalDimensions: u16 =
            (((((left) as i32) << 8) | ((left) as i32).wrapping_add(((width) as i32))) as u16);
        let mut verticalDimensions: u16 =
            (((((top) as i32) << 8) | ((top) as i32).wrapping_add(((height) as i32))) as u16);
        SetGpuReg(64u8, horizontalDimensions);
        SetGpuReg(68u8, verticalDimensions);
    }
}
pub(crate) unsafe extern "C" fn LoadEasyChatGfx() {
    unsafe {
        let mut i: u32 = 0u32;
        LoadSpriteSheets(((&raw const sSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>());
        LoadSpritePalettes(((&raw const sSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>());
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(32u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sCompressedSpriteSheets).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMainCursorSprite() {
    unsafe {
        let mut frameId: u8 = GetEasyChatScreenFrameId();
        let mut x: i32 = (((crate::c::bf_read(
            ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(0),
            0,
            5,
            false,
        ) as u8) as i32)
            .wrapping_mul(8i32))
        .wrapping_add(13i32);
        let mut y: i32 = (((crate::c::bf_read(
            ((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(0),
            5,
            3,
            false,
        ) as u8) as i32)
            .wrapping_mul(8i32))
        .wrapping_add(8i32);
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_TriangleCursor)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            2u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 2i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                if (({
                    let __p3 = (sprite).wrapping_add(36).cast::<i16>();
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 0i32
                {
                    ((sprite).wrapping_add(36).cast::<i16>()).write((-6i16));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMainCursorPos(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(32)
        .cast::<i16>())
        .write(((x) as i16));
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(((y) as i16));
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(0i16);
        (((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn StopMainCursorAnim() {
    unsafe {
        (((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        ((((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn StartMainCursorAnim() {
    unsafe {
        ((((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(728)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn CreateRectangleCursorSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_RectangleCursor)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            3u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write(32i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_RectangleCursor)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            3u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(736)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(736)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write((-32i16));
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(63),
            0,
            1,
            (1u16) as i32,
        );
        UpdateRectangleCursorPos();
    }
}
pub(crate) unsafe extern "C" fn DestroyRectangleCursorSprites() {
    unsafe {
        DestroySprite(
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        DestroySprite(
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read(),
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(736)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn UpdateRectangleCursorPos() {
    unsafe {
        let mut column: i8 = 0i8;
        let mut row: i8 = 0i8;
        if (!(((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(732)
            .cast::<*mut u8>())
        .read())
        .is_null())
            && (!(((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .is_null())
        {
            GetKeyboardCursorColAndRow(&raw mut column, &raw mut row);
            if !((GetInAlphabetMode()) != 0) {
                SetRectangleCursorPos_GroupMode(column, row);
            } else {
                SetRectangleCursorPos_AlphabetMode(column, row);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetRectangleCursorPos_GroupMode(column: i8, row: i8) {
    unsafe {
        let mut column = column;
        let mut row = row;
        if ((column) as i32) != (-1i32) {
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(732)
                    .cast::<*mut u8>())
                .read(),
                0u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write((((((column) as i32).wrapping_mul(84i32)).wrapping_add(58i32)) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(96i32)) as i16));
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(736)
                    .cast::<*mut u8>())
                .read(),
                0u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write((((((column) as i32).wrapping_mul(84i32)).wrapping_add(58i32)) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(96i32)) as i16));
        } else {
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(732)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(216i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(112i32)) as i16));
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(736)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(216i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(112i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn SetRectangleCursorPos_AlphabetMode(column: i8, row: i8) {
    unsafe {
        let mut column = column;
        let mut row = row;
        let mut anim: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        if ((column) as i32) != (-1i32) {
            y = (((row) as i32).wrapping_mul(16i32)).wrapping_add(96i32);
            x = 32i32;
            if (((column) as i32) == 6i32) && (((row) as i32) == 0i32) {
                x = 158i32;
                anim = 2i32;
            } else {
                x = (x).wrapping_add(
                    ((((((&raw const sAlphabetKeyboardColumnOffsets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (if (((column) as u8) as i32) < 7i32 {
                            ((column) as i32)
                        } else {
                            0i32
                        }) as isize,
                    ))
                    .read()) as i32),
                );
                anim = 3i32;
            }
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(732)
                    .cast::<*mut u8>())
                .read(),
                ((anim) as u8),
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(((x) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(((y) as i16));
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(736)
                    .cast::<*mut u8>())
                .read(),
                ((anim) as u8),
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(((x) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(((y) as i16));
        } else {
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(732)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(216i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(732)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(112i32)) as i16));
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(736)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(216i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(736)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write((((((row) as i32).wrapping_mul(16i32)).wrapping_add(112i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWordSelectCursorSprite() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_TriangleCursor)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            4u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(740)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(740)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_WordSelectCursor));
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (2u16) as i32,
        );
        UpdateWordSelectCursorPos();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WordSelectCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if (({
                let __p3 = (sprite).wrapping_add(36).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 0i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write((-6i16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateWordSelectCursorPos() {
    unsafe {
        let mut column: i8 = 0i8;
        let mut row: i8 = 0i8;
        let mut x: i8 = 0i8;
        let mut y: i8 = 0i8;
        GetWordSelectColAndRow(&raw mut column, &raw mut row);
        x = ((((column) as i32).wrapping_mul(13i32)) as i8);
        x = (((((x) as i32).wrapping_mul(8i32)).wrapping_add(28i32)) as i8);
        y = (((((row) as i32).wrapping_mul(16i32)).wrapping_add(96i32)) as i8);
        SetWordSelectCursorPos(((x) as u8), ((y) as u8));
    }
}
pub(crate) unsafe extern "C" fn SetWordSelectCursorPos(x: u8, y: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if !(((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(740)
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(((x) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(((y) as i16));
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            (((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyWordSelectCursorSprite() {
    unsafe {
        if !(((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(740)
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            DestroySprite(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(740)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(740)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSideWindowSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ButtonWindow)
                .cast::<u8>()
                .cast_mut(),
            208i16,
            128i16,
            6u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(744)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(744)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(36)
        .cast::<i16>())
        .write((-64i16));
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ModeWindow)
                .cast::<u8>()
                .cast_mut(),
            208i16,
            80i16,
            5u8,
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(748)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn ShowSideWindow() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(9))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                return 0u8;
            }
            if __sw1 == 0i32 {
                let __p2 = (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(744)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(36)
                .cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(744)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(36)
                .cast::<i16>())
                .read()) as i32)
                    >= 0i32
                {
                    ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(744)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    if !((GetInAlphabetMode()) != 0) {
                        StartSpriteAnim(
                            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(748)
                                .cast::<*mut u8>())
                            .read(),
                            1u8,
                        );
                    } else {
                        StartSpriteAnim(
                            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(748)
                                .cast::<*mut u8>())
                            .read(),
                            2u8,
                        );
                    }
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (crate::c::bf_read(
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(748)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(2u8);
                    return 0u8;
                }
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn HideModeWindow() {
    unsafe {
        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
        StartSpriteAnim(
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(748)
                .cast::<*mut u8>())
            .read(),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DestroySideWindowSprites() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(9))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                return 0u8;
            }
            if __sw1 == 0i32 {
                if (crate::c::bf_read(
                    (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(748)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p2 = (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(744)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(36)
                .cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(8i32)) as i16));
                if ((((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(744)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(36)
                .cast::<i16>())
                .read()) as i32)
                    <= (-64i32)
                {
                    DestroySprite(
                        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(748)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    DestroySprite(
                        ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(744)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(748)
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                    ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(744)
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                    let __p3 = (((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    return 0u8;
                }
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SetModeWindowToTransition() {
    unsafe {
        StartSpriteAnim(
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(748)
                .cast::<*mut u8>())
            .read(),
            4u8,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateModeWindowAnim() {
    unsafe {
        if !((GetInAlphabetMode()) != 0) {
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(748)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
        } else {
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(748)
                    .cast::<*mut u8>())
                .read(),
                2u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsModeWindowAnimActive() -> u8 {
    unsafe {
        return ((!((crate::c::bf_read(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(748)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(63),
            4,
            1,
            false,
        ) as u16)
            != 0)) as u8);
    }
}
pub(crate) unsafe extern "C" fn CreateScrollIndicatorSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_ScrollIndicator)
                .cast::<u8>()
                .cast_mut(),
            96i16,
            80i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(752)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        }
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ScrollIndicator)
                .cast::<u8>()
                .cast_mut(),
            96i16,
            156i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(756)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            crate::c::bf_write(
                (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(756)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(63),
                1,
                1,
                (1u16) as i32,
            );
        }
        HideScrollIndicators();
    }
}
pub(crate) unsafe extern "C" fn UpdateScrollIndicatorsVisibility() {
    unsafe {
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(752)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            ((!((CanScrollUp()) != 0)) as u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(756)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            ((!((CanScrollDown()) != 0)) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn HideScrollIndicators() {
    unsafe {
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(752)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(756)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SetScrollIndicatorXPos(inWordSelect: u32) {
    unsafe {
        let mut inWordSelect = inWordSelect;
        if !((inWordSelect) != 0) {
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(752)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(96i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(756)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(96i16);
        } else {
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(752)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(120i16);
            ((((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(756)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(120i16);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStartSelectButtonSprites() {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_StartSelectButton)
                .cast::<u8>()
                .cast_mut(),
            220i16,
            84i16,
            1u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(760)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        }
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_StartSelectButton)
                .cast::<u8>()
                .cast_mut(),
            220i16,
            156i16,
            1u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(764)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            StartSpriteAnim(
                ((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(764)
                    .cast::<*mut u8>())
                .read(),
                1u8,
            );
        }
        HideStartSelectButtons();
    }
}
pub(crate) unsafe extern "C" fn UpdateStartSelectButtonsVisibility() {
    unsafe {
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(760)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            ((!((CanScrollUp()) != 0)) as u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(764)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            ((!((CanScrollDown()) != 0)) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn HideStartSelectButtons() {
    unsafe {
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(760)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut sScreenControl).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(764)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn TryAddInterviewObjectEvents() {
    unsafe {
        let mut graphicsId: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((GetDisplayedPersonType()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                graphicsId = 67i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                graphicsId = 68i32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                graphicsId = 7i32;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        if ((GetEasyChatScreenFrameId()) as i32) != 4i32 {
            return;
        }
        spriteId = CreateObjectGraphicsSprite(
            ((graphicsId) as u16),
            Some(SpriteCallbackDummy),
            76i16,
            40i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                2u8,
            );
        }
        spriteId = CreateObjectGraphicsSprite(
            ((if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                == 0i32
            {
                100i32
            } else {
                105i32
            }) as u16),
            Some(SpriteCallbackDummy),
            52i16,
            40i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                3u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFooterIndex() -> i32 {
    unsafe {
        let mut frameId: u8 = GetEasyChatScreenFrameId();
        'l1: {
            let __sw1 = (((((((&raw const sPhraseFrameDimensions).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((frameId) as i32) as isize * 4))
            .wrapping_add(3))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                return 1i32;
            }
            if __sw1 == 2i32 {
                return 2i32;
            }
            if __sw1 == 0i32 {
                return 0i32;
            }
            if !__matched {
                return 3i32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetFooterOptionXOffset(option: i32) -> i32 {
    unsafe {
        let mut option = option;
        let mut footerIndex: i32 = GetFooterIndex();
        if footerIndex < 3i32 {
            return ((((((((&raw const sFooterOptionXOffsets).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset((footerIndex) as isize * 4))
            .cast::<u8>())
            .wrapping_offset((option) as isize))
            .read()) as i32)
                .wrapping_add(4i32);
        } else {
            return 0i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn AddMainScreenButtonWindow() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut windowId: u16 = 0u16;
        let mut template = crate::ffi::Align4([0u8; 8]);
        let mut footerIndex: i32 = GetFooterIndex();
        if footerIndex == 3i32 {
            return;
        }
        ((&raw mut template).cast::<u8>()).write(3u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(1)).write(1u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(2)).write(11u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(28u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(2u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(5)).write(11u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(52u16);
        windowId = AddWindow((&raw mut template).cast::<u8>());
        FillWindowPixelBuffer(((windowId) as u8), 17u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(16u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut str: *mut u8 =
                        ((((((&raw const sFooterTextOptions).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((footerIndex) as isize * 16))
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read();
                    if !(str).is_null() {
                        let mut x: i32 =
                            ((((((((&raw const sFooterOptionXOffsets).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset((footerIndex) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32);
                        PrintEasyChatText(
                            ((windowId) as u8),
                            1u8,
                            str,
                            ((x) as u8),
                            1u8,
                            0u8,
                            None,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(((windowId) as u8));
    }
}
pub(crate) unsafe extern "C" fn IsEasyChatGroupUnlocked(groupId: u8) -> u8 {
    unsafe {
        let mut groupId = groupId;
        'l1: {
            let __sw1 = ((groupId) as i32);
            let __matched = __sw1 == 20i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 21i32;
            if __sw1 == 20i32 {
                return FlagGet(2150u16);
            }
            if __sw1 == 17i32 || __sw1 == 18i32 || __sw1 == 19i32 {
                return FlagGet(2148u16);
            }
            if __sw1 == 21i32 {
                return EasyChatIsNationalPokedexEnabled();
            }
            if !__matched {
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EasyChat_GetNumWordsInGroup(groupId: u8) -> u16 {
    unsafe {
        let mut groupId = groupId;
        if ((groupId) as i32) == 0i32 {
            return GetNationalPokedexCount(0u8);
        }
        if (IsEasyChatGroupUnlocked(groupId)) != 0 {
            return (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read();
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn IsEasyChatWordInvalid(easyChatWord: u16) -> u8 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        let mut i: u16 = 0u16;
        let mut groupId: u8 = 0u8;
        let mut index: u32 = 0u32;
        let mut numWords: u16 = 0u16;
        let mut list: *mut u16 = core::ptr::null_mut();
        if ((easyChatWord) as i32) == 65535i32 {
            return 0u8;
        }
        groupId = ((((easyChatWord) as i32) >> 9) as u8);
        index = ((((easyChatWord) as i32) & 511i32) as u32);
        if ((groupId) as i32) >= 22i32 {
            return 1u8;
        }
        numWords = (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((groupId) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<u16>())
        .read();
        'l1: {
            let __sw1 = ((groupId) as i32);
            if __sw1 == 0i32 || __sw1 == 21i32 || __sw1 == 18i32 || __sw1 == 19i32 {
                list = (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((groupId) as i32) as isize * 8))
                .cast::<*mut u16>())
                .read();
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < ((numWords) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            if index
                                == ((((list).wrapping_offset(((i) as i32) as isize)).read()) as u32)
                            {
                                return 0u8;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return 1u8;
            }
        }
        if index >= ((numWords) as u32) {
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
pub unsafe extern "C" fn IsBardWordInvalid(easyChatWord: u16) -> u8 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        let mut numWordsInGroup: i32 = 0i32;
        let mut groupId: u8 = ((((easyChatWord) as i32) >> 9) as u8);
        let mut index: u32 = ((((easyChatWord) as i32) & 511i32) as u32);
        if ((groupId) as i32) >= 22i32 {
            return 1u8;
        }
        'l1: {
            let __sw1 = ((groupId) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 21i32 || __sw1 == 18i32 || __sw1 == 19i32;
            if __sw1 == 0i32 || __sw1 == 21i32 {
                numWordsInGroup =
                    ((((&raw mut gNumBardWords_Species).cast::<u16>()).read()) as i32);
                break 'l1;
            }
            if __sw1 == 18i32 || __sw1 == 19i32 {
                numWordsInGroup = ((((&raw mut gNumBardWords_Moves).cast::<u16>()).read()) as i32);
                break 'l1;
            }
            if !__matched {
                numWordsInGroup = (((((((&raw const gEasyChatGroups).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as i32);
                break 'l1;
            }
        }
        if ((numWordsInGroup) as u32) <= index {
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
pub(crate) unsafe extern "C" fn GetEasyChatWord(groupId: u8, index: u16) -> *mut u8 {
    unsafe {
        let mut groupId = groupId;
        let mut index = index;
        'l1: {
            let __sw1 = ((groupId) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 21i32 || __sw1 == 18i32 || __sw1 == 19i32;
            if __sw1 == 0i32 || __sw1 == 21i32 {
                return (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 11))
                .cast::<u8>();
            }
            if __sw1 == 18i32 || __sw1 == 19i32 {
                return (((&raw mut gMoveNames).cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 13))
                .cast::<u8>();
            }
            if !__matched {
                return ((((((((&raw const gEasyChatGroups).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((index) as i32) as isize * 12))
                .cast::<*mut u8>())
                .read();
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyEasyChatWord(dest: *mut u8, easyChatWord: u16) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut easyChatWord = easyChatWord;
        let mut resultStr: *mut u8 = core::ptr::null_mut();
        if (IsEasyChatWordInvalid(easyChatWord)) != 0 {
            resultStr = StringCopy(dest, (&raw mut gText_ThreeQuestionMarks).cast::<u8>());
        } else {
            if ((easyChatWord) as i32) != 65535i32 {
                let mut index: u16 = ((((easyChatWord) as i32) & 511i32) as u16);
                let mut groupId: u8 = ((((easyChatWord) as i32) >> 9) as u8);
                resultStr = StringCopy(dest, GetEasyChatWord(groupId, index));
            } else {
                (dest).write(255u8);
                resultStr = dest;
            }
        }
        return resultStr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertEasyChatWordsToString(
    dest: *mut u8,
    src: *mut u16,
    columns: u16,
    rows: u16,
) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut columns = columns;
        let mut rows = rows;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut numColumns: u16 = ((((columns) as i32).wrapping_sub(1i32)) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((rows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < ((numColumns) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                dest = CopyEasyChatWord(dest, (src).read());
                                if (((src).read()) as i32) != 65535i32 {
                                    (dest).write(0u8);
                                    dest = (dest).wrapping_offset(1);
                                }
                                src = (src).wrapping_offset(1);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    dest = CopyEasyChatWord(
                        dest,
                        ({
                            let __t2 = src;
                            src = (src).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                    (dest).write(254u8);
                    dest = (dest).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        dest = (dest).wrapping_offset(-1);
        (dest).write(255u8);
        return dest;
    }
}
pub(crate) unsafe extern "C" fn UnusedConvertEasyChatWordsToString(
    dest: *mut u8,
    src: *mut u16,
    columns: u16,
    rows: u16,
) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut columns = columns;
        let mut rows = rows;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut k: u16 = 0u16;
        let mut numColumns: u16 = 0u16;
        let mut notEmpty: i32 = 0i32;
        let mut lineNumber: i32 = 0i32;
        numColumns = columns;
        lineNumber = 0i32;
        columns = (columns).wrapping_sub(1);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((rows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut str: *mut u16 = src;
                    notEmpty = 0i32;
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < ((numColumns) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((str).wrapping_offset(((j) as i32) as isize)).read()) as i32)
                                    != 65535i32
                                {
                                    notEmpty = 1i32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if !((notEmpty) != 0) {
                        src = (src).wrapping_offset(((numColumns) as i32) as isize);
                        break 'l2;
                    }
                    {
                        k = 0u16;
                        'l5: loop {
                            if !(((k) as i32) < ((columns) as i32)) {
                                break 'l5;
                            }
                            'l6: {
                                dest = CopyEasyChatWord(dest, (src).read());
                                if (((src).read()) as i32) != 65535i32 {
                                    (dest).write(0u8);
                                    dest = (dest).wrapping_offset(1);
                                }
                                src = (src).wrapping_offset(1);
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                    dest = CopyEasyChatWord(
                        dest,
                        ({
                            let __t2 = src;
                            src = (src).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                    if lineNumber == 0i32 {
                        (dest).write(254u8);
                    } else {
                        (dest).write(250u8);
                    }
                    dest = (dest).wrapping_offset(1);
                    lineNumber = (lineNumber).wrapping_add(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        dest = (dest).wrapping_offset(-1);
        (dest).write(255u8);
        return dest;
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatWordStringLength(easyChatWord: u16) -> u16 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        if ((easyChatWord) as i32) == 65535i32 {
            return 0u16;
        }
        if (IsEasyChatWordInvalid(easyChatWord)) != 0 {
            return StringLength((&raw mut gText_ThreeQuestionMarks).cast::<u8>());
        } else {
            let mut index: u16 = ((((easyChatWord) as i32) & 511i32) as u16);
            let mut groupId: u8 = ((((easyChatWord) as i32) >> 9) as u8);
            return StringLength(GetEasyChatWord(groupId, index));
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn CanPhraseFitInXRowsYCols(
    easyChatWords: *mut u16,
    numRows: u8,
    numColumns: u8,
    maxLength: u16,
) -> u8 {
    unsafe {
        let mut easyChatWords = easyChatWords;
        let mut numRows = numRows;
        let mut numColumns = numColumns;
        let mut maxLength = maxLength;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numColumns) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut totalLength: u16 = ((((numRows) as i32).wrapping_sub(1i32)) as u16);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((numRows) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                totalLength = ((((totalLength) as i32).wrapping_add(
                                    ((GetEasyChatWordStringLength(
                                        ({
                                            let __t2 = easyChatWords;
                                            easyChatWords = (easyChatWords).wrapping_offset(1);
                                            __t2
                                        })
                                        .read(),
                                    )) as i32),
                                )) as u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((totalLength) as i32) > ((maxLength) as i32) {
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
pub unsafe extern "C" fn GetRandomEasyChatWordFromGroup(groupId: u16) -> u16 {
    unsafe {
        let mut groupId = groupId;
        let mut index: u16 = ((crate::c::rem_i32(
            ((Random()) as i32),
            (((((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read()) as i32),
        )) as u16);
        if (((((groupId) as i32) == 0i32) || (((groupId) as i32) == 21i32))
            || (((groupId) as i32) == 18i32))
            || (((groupId) as i32) == 19i32)
        {
            index = (((((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
            .cast::<*mut u16>())
            .read())
            .wrapping_offset(((index) as i32) as isize))
            .read();
        }
        return ((((((groupId) as i32) & 127i32) << 9) | (((index) as i32) & 511i32)) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomEasyChatWordFromUnlockedGroup(groupId: u16) -> u16 {
    unsafe {
        let mut groupId = groupId;
        if !((IsEasyChatGroupUnlocked(((groupId) as u8))) != 0) {
            return 65535u16;
        }
        if ((groupId) as i32) == 0i32 {
            return GetRandomUnlockedEasyChatPokemon();
        }
        return GetRandomEasyChatWordFromGroup(groupId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowEasyChatProfile() {
    unsafe {
        let mut easyChatWords: *mut u16 = core::ptr::null_mut();
        let mut columns: i32 = 0i32;
        let mut rows: i32 = 0i32;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                easyChatWords = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11184))
                .cast::<u16>();
                columns = 2i32;
                rows = 2i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                easyChatWords = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11196))
                .cast::<u16>();
                if (CanPhraseFitInXRowsYCols(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11196))
                        .cast::<u16>(),
                    3u8,
                    2u8,
                    18u16,
                )) != 0
                {
                    columns = 2i32;
                    rows = 3i32;
                } else {
                    columns = 3i32;
                    rows = 2i32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                easyChatWords = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11208))
                .cast::<u16>();
                columns = 3i32;
                rows = 2i32;
                break 'l1;
            }
            if __sw1 == 3i32 {
                easyChatWords = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(11220))
                .cast::<u16>();
                columns = 3i32;
                rows = 2i32;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        ConvertEasyChatWordsToString(
            (&raw mut gStringVar4).cast::<u8>(),
            easyChatWords,
            ((columns) as u16),
            ((rows) as u16),
        );
        ShowFieldAutoScrollMessage((&raw mut gStringVar4).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferDeepLinkPhrase() {
    unsafe {
        let mut groupId: i32 = (if (((Random()) as i32) & 1i32) != 0 {
            13i32
        } else {
            12i32
        });
        let mut easyChatWord: u16 = GetRandomEasyChatWordFromUnlockedGroup(((groupId) as u16));
        CopyEasyChatWord((&raw mut gStringVar2).cast::<u8>(), easyChatWord);
    }
}
pub(crate) unsafe extern "C" fn IsTrendySayingUnlocked(wordIndex: u8) -> u8 {
    unsafe {
        let mut wordIndex = wordIndex;
        let mut byteOffset: i32 = crate::c::div_i32(((wordIndex) as i32), 8i32);
        let mut shift: i32 = crate::c::rem_i32(((wordIndex) as i32), 8i32);
        return ((crate::c::shr_i32(
            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11808))
                .cast::<u8>())
            .wrapping_offset((byteOffset) as isize))
            .read()) as i32),
            ((shift) as u32),
        ) & 1i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnlockTrendySaying(wordIndex: u8) {
    unsafe {
        let mut wordIndex = wordIndex;
        if ((wordIndex) as i32) < 33i32 {
            let mut byteOffset: i32 = crate::c::div_i32(((wordIndex) as i32), 8i32);
            let mut shift: i32 = crate::c::rem_i32(((wordIndex) as i32), 8i32);
            let __p1 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(11808))
            .cast::<u8>())
            .wrapping_offset((byteOffset) as isize);
            (__p1).write(
                (((((__p1).read()) as i32) | crate::c::shl_i32(1i32, ((shift) as u32))) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetNumTrendySayingsUnlocked() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numUnlocked: u8 = 0u8;
        {
            i = 0u8;
            numUnlocked = 0u8;
            'l1: loop {
                if !(((i) as i32) < 33i32) {
                    break 'l1;
                }
                'l2: {
                    if (IsTrendySayingUnlocked(i)) != 0 {
                        numUnlocked = (numUnlocked).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return numUnlocked;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnlockRandomTrendySaying() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut numToSkip: u16 = 0u16;
        let mut numUnlocked: u8 = GetNumTrendySayingsUnlocked();
        if ((numUnlocked) as i32) == 33i32 {
            return 65535u16;
        }
        numToSkip = ((crate::c::rem_i32(
            ((Random()) as i32),
            (33i32).wrapping_sub(((numUnlocked) as i32)),
        )) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 33i32) {
                    break 'l1;
                }
                'l2: {
                    if !((IsTrendySayingUnlocked(((i) as u8))) != 0) {
                        if (numToSkip) != 0 {
                            numToSkip = (numToSkip).wrapping_sub(1);
                        } else {
                            UnlockTrendySaying(((i) as u8));
                            return ((10240i32 | (((i) as i32) & 511i32)) as u16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
pub(crate) unsafe extern "C" fn GetRandomUnlockedTrendySaying() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut n: u16 = ((GetNumTrendySayingsUnlocked()) as u16);
        if ((n) as i32) == 0i32 {
            return 65535u16;
        }
        n = ((crate::c::rem_i32(((Random()) as i32), ((n) as i32))) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 33i32) {
                    break 'l1;
                }
                'l2: {
                    if (IsTrendySayingUnlocked(((i) as u8))) != 0 {
                        if (n) != 0 {
                            n = (n).wrapping_sub(1);
                        } else {
                            return ((10240i32 | (((i) as i32) & 511i32)) as u16);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
pub(crate) unsafe extern "C" fn EasyChatIsNationalPokedexEnabled() -> u8 {
    unsafe {
        return ((IsNationalPokedexEnabled()) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetRandomUnlockedEasyChatPokemon() -> u16 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut numWords: u16 = 0u16;
        let mut species: *mut u16 = core::ptr::null_mut();
        let mut index: u16 = EasyChat_GetNumWordsInGroup(0u8);
        if ((index) as i32) == 0i32 {
            return 65535u16;
        }
        index = ((crate::c::rem_i32(((Random()) as i32), ((index) as i32))) as u16);
        species = ((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
            .cast::<*mut u16>())
        .read();
        numWords = ((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .read();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((numWords) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut dexNum: u16 = SpeciesToNationalPokedexNum((species).read());
                    if (GetSetPokedexFlag(dexNum, 0u8)) != 0 {
                        if (index) != 0 {
                            index = (index).wrapping_sub(1);
                        } else {
                            return ((0i32 | ((((species).read()) as i32) & 511i32)) as u16);
                        }
                    }
                    species = (species).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        return 65535u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitEasyChatPhrases() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11184))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sDefaultProfileWords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11196))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sDefaultBattleStartWords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11208))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sDefaultBattleWonWords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l7;
                }
                'l8: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11220))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sDefaultBattleLostWords)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l9: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l9;
                }
                'l10: {
                    {
                        j = 0u16;
                        'l11: loop {
                            if !(((j) as i32) < 9i32) {
                                break 'l11;
                            }
                            'l12: {
                                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(11232))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(65535u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l13: loop {
                if !(((i) as u32) < crate::c::div_u32(5u32, 1u32)) {
                    break 'l13;
                }
                'l14: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11808))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenWordData() -> u8 {
    unsafe {
        ((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).write(Alloc(15268u32));
        if !(!(((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return 0u8;
        }
        SetUnlockedEasyChatGroups();
        SetUnlockedWordsByAlphabet();
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenWordData() {
    unsafe {
        if ((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            Free(((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SetUnlockedEasyChatGroups() {
    unsafe {
        let mut i: i32 = 0i32;
        ((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>()).write(0u16);
        if (GetNationalPokedexCount(0u8)) != 0 {
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p1 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t2 = (__p1).read();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                    __t2
                }) as i32) as isize,
            ))
            .write(0u16);
        }
        {
            i = 1i32;
            'l1: loop {
                if !(i <= 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset(
                        (({
                            let __p3 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .cast::<u16>();
                            let __t4 = (__p3).read();
                            (__p3).write(((__p3).read()).wrapping_add(1));
                            __t4
                        }) as i32) as isize,
                    ))
                    .write(((i) as u16));
                }
                i = (i).wrapping_add(1);
            }
        }
        if (FlagGet(2148u16)) != 0 {
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p5 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t6 = (__p5).read();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    __t6
                }) as i32) as isize,
            ))
            .write(17u16);
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p7 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t8 = (__p7).read();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    __t8
                }) as i32) as isize,
            ))
            .write(18u16);
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p9 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t10 = (__p9).read();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    __t10
                }) as i32) as isize,
            ))
            .write(19u16);
        }
        if (FlagGet(2150u16)) != 0 {
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p11 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t12 = (__p11).read();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                    __t12
                }) as i32) as isize,
            ))
            .write(20u16);
        }
        if (IsNationalPokedexEnabled()) != 0 {
            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .cast::<u16>())
            .wrapping_offset(
                (({
                    let __p13 = (((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>();
                    let __t14 = (__p13).read();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                    __t14
                }) as i32) as isize,
            ))
            .write(21u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetNumUnlockedEasyChatGroups() -> u8 {
    unsafe {
        return ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
            .read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetUnlockedEasyChatGroupId(index: u8) -> u8 {
    unsafe {
        let mut index = index;
        if ((index) as i32)
            >= ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .read()) as i32)
        {
            return 22u8;
        } else {
            return ((((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .cast::<u16>())
            .wrapping_offset(((index) as i32) as isize))
            .read()) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BufferEasyChatWordGroupName(
    dest: *mut u8,
    groupId: u8,
    totalChars: u16,
) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut groupId = groupId;
        let mut totalChars = totalChars;
        let mut i: u16 = 0u16;
        let mut str: *mut u8 = StringCopy(
            dest,
            ((((&raw const sEasyChatGroupNamePointers)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((groupId) as i32) as isize))
            .read(),
        );
        {
            i = ((((str) as usize).wrapping_sub((dest) as usize) as i32 / 1) as u16);
            'l1: loop {
                if !(((i) as i32) < ((totalChars) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (str).write(0u8);
                    str = (str).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        (str).write(255u8);
        return str;
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatWordGroupName(groupId: u8) -> *mut u8 {
    unsafe {
        let mut groupId = groupId;
        return ((((&raw const sEasyChatGroupNamePointers)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((groupId) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn CopyEasyChatWordPadded(
    dest: *mut u8,
    easyChatWord: u16,
    totalChars: u16,
) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut easyChatWord = easyChatWord;
        let mut totalChars = totalChars;
        let mut i: u16 = 0u16;
        let mut str: *mut u8 = CopyEasyChatWord(dest, easyChatWord);
        {
            i = ((((str) as usize).wrapping_sub((dest) as usize) as i32 / 1) as u16);
            'l1: loop {
                if !(((i) as i32) < ((totalChars) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (str).write(0u8);
                    str = (str).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        (str).write(255u8);
        return str;
    }
}
pub(crate) unsafe extern "C" fn SetUnlockedWordsByAlphabet() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut numWords: i32 = 0i32;
        let mut words: *mut u16 = core::ptr::null_mut();
        let mut numToProcess: u16 = 0u16;
        let mut index: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 27i32) {
                    break 'l1;
                }
                'l2: {
                    numWords = (((((&raw const gEasyChatWordsByLetterPointers)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read();
                    words = (((((&raw const gEasyChatWordsByLetterPointers)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<*mut u16>())
                    .read();
                    ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                    index = 0i32;
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < numWords) {
                                break 'l3;
                            }
                            'l4: {
                                if (((words).read()) as i32) == 65535i32 {
                                    words = (words).wrapping_offset(1);
                                    numToProcess = (words).read();
                                    words = (words).wrapping_offset(1);
                                    j = (j)
                                        .wrapping_add((1i32).wrapping_add(((numToProcess) as i32)));
                                } else {
                                    numToProcess = 1u16;
                                }
                                {
                                    k = 0i32;
                                    'l5: loop {
                                        if !(k < ((numToProcess) as i32)) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            if (IsEasyChatWordUnlocked(
                                                ((words).wrapping_offset((k) as isize)).read(),
                                            )) != 0
                                            {
                                                ((((((((&raw mut sWordData)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(100))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 540))
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ({
                                                        let __t1 = index;
                                                        index = (index).wrapping_add(1);
                                                        __t1
                                                    })
                                                        as isize,
                                                ))
                                                .write(
                                                    ((words).wrapping_offset((k) as isize)).read(),
                                                );
                                                let __p2 = (((((&raw mut sWordData)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(46))
                                                .cast::<u16>())
                                                .wrapping_offset((i) as isize);
                                                (__p2).write(((__p2).read()).wrapping_add(1));
                                                break 'l5;
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                                words = (words).wrapping_offset(((numToProcess) as i32) as isize);
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
pub(crate) unsafe extern "C" fn SetSelectedWordGroup(inAlphabetMode: u32, groupId: u16) {
    unsafe {
        let mut inAlphabetMode = inAlphabetMode;
        let mut groupId = groupId;
        if !((inAlphabetMode) != 0) {
            ((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15264)
                .cast::<u16>())
            .write(SetSelectedWordGroup_GroupMode(groupId));
        } else {
            ((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15264)
                .cast::<u16>())
            .write(SetSelectedWordGroup_AlphabetMode(groupId));
        }
    }
}
pub(crate) unsafe extern "C" fn GetWordFromSelectedGroup(index: u16) -> u16 {
    unsafe {
        let mut index = index;
        if ((index) as i32)
            >= ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15264)
                .cast::<u16>())
            .read()) as i32)
        {
            return 65535u16;
        } else {
            return ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14724))
            .cast::<u16>())
            .wrapping_offset(((index) as i32) as isize))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn GetNumWordsInSelectedGroup() -> u16 {
    unsafe {
        return ((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(15264)
            .cast::<u16>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWordGroup_GroupMode(groupId: u16) -> u16 {
    unsafe {
        let mut groupId = groupId;
        let mut i: u32 = 0u32;
        let mut totalWords: i32 = 0i32;
        let mut list: *mut u16 = core::ptr::null_mut();
        let mut wordInfo: *mut u8 = core::ptr::null_mut();
        let mut numWords: u16 = (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((groupId) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<u16>())
        .read();
        if (((((groupId) as i32) == 0i32) || (((groupId) as i32) == 21i32))
            || (((groupId) as i32) == 18i32))
            || (((groupId) as i32) == 19i32)
        {
            list = (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
            .cast::<*mut u16>())
            .read();
            {
                i = 0u32;
                totalWords = 0i32;
                'l1: loop {
                    if !(i < ((numWords) as u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (IsEasyChatIndexAndGroupUnlocked(
                            ((list).wrapping_offset(((i) as i32) as isize)).read(),
                            ((groupId) as u8),
                        )) != 0
                        {
                            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14724))
                            .cast::<u16>())
                            .wrapping_offset(
                                ({
                                    let __t1 = totalWords;
                                    totalWords = (totalWords).wrapping_add(1);
                                    __t1
                                }) as isize,
                            ))
                            .write(
                                ((((((groupId) as i32) & 127i32) << 9)
                                    | (((((list).wrapping_offset(((i) as i32) as isize)).read())
                                        as i32)
                                        & 511i32)) as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return ((totalWords) as u16);
        } else {
            wordInfo = (((((&raw const gEasyChatGroups).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
            .cast::<*mut u8>())
            .read();
            {
                i = 0u32;
                totalWords = 0i32;
                'l3: loop {
                    if !(i < ((numWords) as u32)) {
                        break 'l3;
                    }
                    'l4: {
                        let mut alphabeticalOrder: u16 = (((((wordInfo)
                            .wrapping_offset(((i) as i32) as isize * 12))
                        .wrapping_add(4)
                        .cast::<i32>())
                        .read()) as u16);
                        if (IsEasyChatIndexAndGroupUnlocked(alphabeticalOrder, ((groupId) as u8)))
                            != 0
                        {
                            ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(14724))
                            .cast::<u16>())
                            .wrapping_offset(
                                ({
                                    let __t2 = totalWords;
                                    totalWords = (totalWords).wrapping_add(1);
                                    __t2
                                }) as isize,
                            ))
                            .write(
                                ((((((groupId) as i32) & 127i32) << 9)
                                    | (((alphabeticalOrder) as i32) & 511i32))
                                    as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            return ((totalWords) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWordGroup_AlphabetMode(groupId: u16) -> u16 {
    unsafe {
        let mut groupId = groupId;
        let mut i: u16 = 0u16;
        let mut totalWords: u16 = 0u16;
        {
            i = 0u16;
            totalWords = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(46))
                    .cast::<u16>())
                    .wrapping_offset(((groupId) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14724))
                    .cast::<u16>())
                    .wrapping_offset(
                        (({
                            let __t1 = totalWords;
                            totalWords = (totalWords).wrapping_add(1);
                            __t1
                        }) as i32) as isize,
                    ))
                    .write(
                        ((((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100))
                        .cast::<u8>())
                        .wrapping_offset(((groupId) as i32) as isize * 540))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return totalWords;
    }
}
pub(crate) unsafe extern "C" fn IsEasyChatGroupUnlocked2(groupId: u8) -> u8 {
    unsafe {
        let mut groupId = groupId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sWordData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((groupId) as i32)
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
pub(crate) unsafe extern "C" fn IsEasyChatIndexAndGroupUnlocked(wordIndex: u16, groupId: u8) -> u8 {
    unsafe {
        let mut wordIndex = wordIndex;
        let mut groupId = groupId;
        'l1: {
            let __sw1 = ((groupId) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 21i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32;
            if __sw1 == 0i32 {
                return ((GetSetPokedexFlag(SpeciesToNationalPokedexNum(wordIndex), 0u8)) as u8);
            }
            if __sw1 == 21i32 {
                if (IsRestrictedWordSpecies(wordIndex)) != 0 {
                    GetSetPokedexFlag(SpeciesToNationalPokedexNum(wordIndex), 0u8);
                }
                return 1u8;
            }
            if __sw1 == 18i32 || __sw1 == 19i32 {
                return 1u8;
            }
            if __sw1 == 20i32 {
                return IsTrendySayingUnlocked(((wordIndex) as u8));
            }
            if !__matched {
                return ((((((((((&raw const gEasyChatGroups).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((groupId) as i32) as isize * 8))
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(((wordIndex) as i32) as isize * 12))
                .wrapping_add(8)
                .cast::<i32>())
                .read()) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsRestrictedWordSpecies(species: u16) -> i32 {
    unsafe {
        let mut species = species;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(2u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sRestrictedWordSpecies)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((species) as i32)
                    {
                        return 1i32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn IsEasyChatWordUnlocked(easyChatWord: u16) -> u8 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        let mut groupId: u8 = ((((easyChatWord) as i32) >> 9) as u8);
        let mut index: u32 = ((((easyChatWord) as i32) & 511i32) as u32);
        if !((IsEasyChatGroupUnlocked2(groupId)) != 0) {
            return 0u8;
        } else {
            return IsEasyChatIndexAndGroupUnlocked(((index) as u16), groupId);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitializeEasyChatWordArray(words: *mut u16, length: u16) {
    unsafe {
        let mut words = words;
        let mut length = length;
        let mut i: u16 = 0u16;
        {
            i = ((((length) as i32).wrapping_sub(1i32)) as u16);
            'l1: loop {
                if !(((i) as i32) != 65535i32) {
                    break 'l1;
                }
                'l2: {
                    ({
                        let __t1 = words;
                        words = (words).wrapping_offset(1);
                        __t1
                    })
                    .write(65535u16);
                }
                i = (i).wrapping_sub(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitQuestionnaireWords() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut words: *mut u16 = GetQuestionnaireWordsPtr();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((words).wrapping_offset((i) as isize)).write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEasyChatAnswerUnlocked(easyChatWord: i32) -> u32 {
    unsafe {
        let mut easyChatWord = easyChatWord;
        let mut groupId: i32 = (easyChatWord >> 9);
        let mut mask: i32 = 127i32;
        let mut index: i32 = (easyChatWord & 511i32);
        if !((IsEasyChatGroupUnlocked(((groupId & mask) as u8))) != 0) {
            return 0u32;
        } else {
            return ((IsEasyChatIndexAndGroupUnlocked(((index) as u16), ((groupId & mask) as u8)))
                as u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
