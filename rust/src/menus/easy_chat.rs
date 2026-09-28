//! Translated from `src/easy_chat.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sQuizLadyEasyChatScreens sEasyChatScreenTemplates sAlphabetGroupIdMap sMysteryGiftPhrase sBerryMasterWifePhrases sTriangleCursor_Pal sTriangleCursor_Gfx sScrollIndicator_Gfx sStartSelectButtons_Gfx sRSInterviewFrame_Pal sRSInterviewFrame_Gfx sTextInputFrameOrange_Pal sTextInputFrameGreen_Pal sTextInputFrame_Gfx sTitleText_Pal sText_Pal sPhraseFrameDimensions sEasyChatBgTemplates sEasyChatWindowTemplates sEasyChatYesNoWindowTemplate sText_Clear17 sEasyChatKeyboardAlphabet sSpriteSheets sSpritePalettes sCompressedSpriteSheets sAlphabetKeyboardColumnOffsets sOamData_TriangleCursor sSpriteTemplate_TriangleCursor sOamData_RectangleCursor sAnim_RectangleCursor_OnGroup sAnim_RectangleCursor_OnButton sAnim_RectangleCursor_OnOthers sAnim_RectangleCursor_OnLetter sAnims_RectangleCursor sSpriteTemplate_RectangleCursor sOamData_ModeWindow sAnim_ModeWindow_Hidden sAnim_ModeWindow_ToGroup sAnim_ModeWindow_ToAlphabet sAnim_ModeWindow_ToHidden sAnim_ModeWindow_Transition sAnims_ModeWindow sSpriteTemplate_ModeWindow sOamData_ButtonWindow sSpriteTemplate_ButtonWindow sOamData_StartSelectButton sOamData_ScrollIndicator sAnim_Frame0 sAnim_Frame1 sAnims_TwoFrame sSpriteTemplate_StartSelectButton sSpriteTemplate_ScrollIndicator sFooterOptionXOffsets sFooterTextOptions gEasyChatGroup_Pokemon gEasyChatWord_IChooseYou gEasyChatWord_Gotcha gEasyChatWord_Trade gEasyChatWord_Sapphire gEasyChatWord_Evolve gEasyChatWord_Encyclopedia gEasyChatWord_Nature gEasyChatWord_Center gEasyChatWord_Egg gEasyChatWord_Link gEasyChatWord_SpAbility gEasyChatWord_Trainer gEasyChatWord_Version gEasyChatWord_Pokenav gEasyChatWord_Pokemon gEasyChatWord_Get gEasyChatWord_Pokedex gEasyChatWord_Ruby gEasyChatWord_Level gEasyChatWord_Red gEasyChatWord_Green gEasyChatWord_Bag gEasyChatWord_Flame gEasyChatWord_Gold gEasyChatWord_Leaf gEasyChatWord_Silver gEasyChatWord_Emerald gEasyChatGroup_Trainer gEasyChatWord_Dark gEasyChatWord_Stench gEasyChatWord_ThickFat gEasyChatWord_RainDish gEasyChatWord_Drizzle gEasyChatWord_ArenaTrap gEasyChatWord_Intimidate gEasyChatWord_RockHead gEasyChatWord_Color gEasyChatWord_AltColor gEasyChatWord_Rock gEasyChatWord_Beautiful gEasyChatWord_Beauty gEasyChatWord_AirLock gEasyChatWord_Psychic gEasyChatWord_HyperCutter gEasyChatWord_Fighting gEasyChatWord_ShadowTag gEasyChatWord_Smart gEasyChatWord_Smartness gEasyChatWord_SpeedBoost gEasyChatWord_Cool gEasyChatWord_Coolness gEasyChatWord_BattleArmor gEasyChatWord_Cute gEasyChatWord_Cuteness gEasyChatWord_Sturdy gEasyChatWord_SuctionCups gEasyChatWord_Grass gEasyChatWord_ClearBody gEasyChatWord_Torrent gEasyChatWord_Ghost gEasyChatWord_Ice gEasyChatWord_Guts gEasyChatWord_RoughSkin gEasyChatWord_ShellArmor gEasyChatWord_NaturalCure gEasyChatWord_Damp gEasyChatWord_Ground gEasyChatWord_Limber gEasyChatWord_MagnetPull gEasyChatWord_WhiteSmoke gEasyChatWord_Synchronize gEasyChatWord_Overgrow gEasyChatWord_SwiftSwim gEasyChatWord_SandStream gEasyChatWord_SandVeil gEasyChatWord_KeenEye gEasyChatWord_InnerFocus gEasyChatWord_Static gEasyChatWord_Type gEasyChatWord_Tough gEasyChatWord_Toughness gEasyChatWord_ShedSkin gEasyChatWord_HugePower gEasyChatWord_VoltAbsorb gEasyChatWord_WaterAbsorb gEasyChatWord_Electric gEasyChatWord_Forecast gEasyChatWord_SereneGrace gEasyChatWord_Poison gEasyChatWord_PoisonPoint gEasyChatWord_Dragon gEasyChatWord_Trace gEasyChatWord_Oblivious gEasyChatWord_Truant gEasyChatWord_RunAway gEasyChatWord_StickyHold gEasyChatWord_CloudNine gEasyChatWord_Normal gEasyChatWord_Steel gEasyChatWord_Illuminate gEasyChatWord_EarlyBird gEasyChatWord_Hustle gEasyChatWord_Shine gEasyChatWord_Flying gEasyChatWord_Drought gEasyChatWord_Lightningrod gEasyChatWord_Compoundeyes gEasyChatWord_MarvelScale gEasyChatWord_WonderGuard gEasyChatWord_Insomnia gEasyChatWord_Levitate gEasyChatWord_Plus gEasyChatWord_Pressure gEasyChatWord_LiquidOoze gEasyChatWord_ColorChange gEasyChatWord_Soundproof gEasyChatWord_EffectSpore gEasyChatWord_Pkrs gEasyChatWord_Fire gEasyChatWord_FlameBody gEasyChatWord_Minus gEasyChatWord_OwnTempo gEasyChatWord_MagmaArmor gEasyChatWord_Water gEasyChatWord_WaterVeil gEasyChatWord_Bug gEasyChatWord_Swarm gEasyChatWord_CuteCharm gEasyChatWord_Immunity gEasyChatWord_Blaze gEasyChatWord_Pickup gEasyChatWord_Pattern gEasyChatWord_FlashFire gEasyChatWord_VitalSpirit gEasyChatWord_Chlorophyll gEasyChatWord_PurePower gEasyChatWord_ShieldDust gEasyChatGroup_Status gEasyChatWord_MatchUp gEasyChatWord_Go gEasyChatWord_No1 gEasyChatWord_Decide gEasyChatWord_LetMeWin gEasyChatWord_Wins gEasyChatWord_Win gEasyChatWord_Won gEasyChatWord_IfIWin gEasyChatWord_WhenIWin gEasyChatWord_CantWin gEasyChatWord_CanWin gEasyChatWord_NoMatch gEasyChatWord_Spirit gEasyChatWord_Decided gEasyChatWord_TrumpCard gEasyChatWord_TakeThat gEasyChatWord_ComeOn gEasyChatWord_Attack gEasyChatWord_Surrender gEasyChatWord_Gutsy gEasyChatWord_Talent gEasyChatWord_Strategy gEasyChatWord_Smite gEasyChatWord_Match gEasyChatWord_Victory gEasyChatWord_Offensive gEasyChatWord_Sense gEasyChatWord_Versus gEasyChatWord_Fights gEasyChatWord_Power gEasyChatWord_Challenge gEasyChatWord_Strong gEasyChatWord_TooStrong gEasyChatWord_GoEasy gEasyChatWord_Foe gEasyChatWord_Genius gEasyChatWord_Legend gEasyChatWord_Escape gEasyChatWord_Aim gEasyChatWord_Battle gEasyChatWord_Fight gEasyChatWord_Resuscitate gEasyChatWord_Points gEasyChatWord_Serious gEasyChatWord_GiveUp gEasyChatWord_Loss gEasyChatWord_IfILose gEasyChatWord_Lost gEasyChatWord_Lose gEasyChatWord_Guard gEasyChatWord_Partner gEasyChatWord_Reject gEasyChatWord_Accept gEasyChatWord_Invincible gEasyChatWord_Received gEasyChatWord_Easy gEasyChatWord_Weak gEasyChatWord_TooWeak gEasyChatWord_Pushover gEasyChatWord_Leader gEasyChatWord_Rule gEasyChatWord_Move gEasyChatGroup_Battle gEasyChatWord_Thanks gEasyChatWord_Yes gEasyChatWord_HereGoes gEasyChatWord_HereICome gEasyChatWord_HereItIs gEasyChatWord_Yeah gEasyChatWord_Welcome gEasyChatWord_Oi gEasyChatWord_HowDo gEasyChatWord_Congrats gEasyChatWord_GiveMe gEasyChatWord_Sorry gEasyChatWord_Apologize gEasyChatWord_Forgive gEasyChatWord_HeyThere gEasyChatWord_Hello gEasyChatWord_GoodBye gEasyChatWord_ThankYou gEasyChatWord_IveArrived gEasyChatWord_Pardon gEasyChatWord_Excuse gEasyChatWord_SeeYa gEasyChatWord_ExcuseMe gEasyChatWord_WellThen gEasyChatWord_GoAhead gEasyChatWord_Appreciate gEasyChatWord_HeyQues gEasyChatWord_WhatsUpQues gEasyChatWord_HuhQues gEasyChatWord_No gEasyChatWord_Hi gEasyChatWord_YeahYeah gEasyChatWord_ByeBye gEasyChatWord_MeetYou gEasyChatWord_Hey gEasyChatWord_Smell gEasyChatWord_Listening gEasyChatWord_HooHah gEasyChatWord_Yahoo gEasyChatWord_Yo gEasyChatWord_ComeOver gEasyChatWord_CountOn gEasyChatGroup_Greetings gEasyChatWord_Opponent gEasyChatWord_I gEasyChatWord_You gEasyChatWord_Yours gEasyChatWord_Son gEasyChatWord_Your gEasyChatWord_Youre gEasyChatWord_Youve gEasyChatWord_Mother gEasyChatWord_Grandfather gEasyChatWord_Uncle gEasyChatWord_Father gEasyChatWord_Boy gEasyChatWord_Adult gEasyChatWord_Brother gEasyChatWord_Sister gEasyChatWord_Grandmother gEasyChatWord_Aunt gEasyChatWord_Parent gEasyChatWord_Man gEasyChatWord_Me gEasyChatWord_Girl gEasyChatWord_Babe gEasyChatWord_Family gEasyChatWord_Her gEasyChatWord_Him gEasyChatWord_He gEasyChatWord_Place gEasyChatWord_Daughter gEasyChatWord_His gEasyChatWord_Hes gEasyChatWord_Arent gEasyChatWord_Siblings gEasyChatWord_Kid gEasyChatWord_Children gEasyChatWord_Mr gEasyChatWord_Mrs gEasyChatWord_Myself gEasyChatWord_IWas gEasyChatWord_ToMe gEasyChatWord_My gEasyChatWord_IAm gEasyChatWord_Ive gEasyChatWord_Who gEasyChatWord_Someone gEasyChatWord_WhoWas gEasyChatWord_ToWhom gEasyChatWord_Whose gEasyChatWord_WhoIs gEasyChatWord_Its gEasyChatWord_Lady gEasyChatWord_Friend gEasyChatWord_Ally gEasyChatWord_Person gEasyChatWord_Dude gEasyChatWord_They gEasyChatWord_TheyWere gEasyChatWord_ToThem gEasyChatWord_Their gEasyChatWord_Theyre gEasyChatWord_Theyve gEasyChatWord_We gEasyChatWord_Been gEasyChatWord_ToUs gEasyChatWord_Our gEasyChatWord_WeRe gEasyChatWord_Rival gEasyChatWord_Weve gEasyChatWord_Woman gEasyChatWord_She gEasyChatWord_SheWas gEasyChatWord_ToHer gEasyChatWord_Hers gEasyChatWord_SheIs gEasyChatWord_Some gEasyChatGroup_People gEasyChatWord_Excl gEasyChatWord_ExclExcl gEasyChatWord_QuesExcl gEasyChatWord_Ques gEasyChatWord_Ellipsis gEasyChatWord_EllipsisExcl gEasyChatWord_EllipsisEllipsisEllipsis gEasyChatWord_Dash gEasyChatWord_DashDashDash gEasyChatWord_UhOh gEasyChatWord_Waaah gEasyChatWord_Ahaha gEasyChatWord_OhQues gEasyChatWord_Nope gEasyChatWord_Urgh gEasyChatWord_Hmm gEasyChatWord_Whoah gEasyChatWord_WroooaarExcl gEasyChatWord_Wow gEasyChatWord_Giggle gEasyChatWord_Sigh gEasyChatWord_Unbelievable gEasyChatWord_Cries gEasyChatWord_Agree gEasyChatWord_EhQues gEasyChatWord_Cry gEasyChatWord_Ehehe gEasyChatWord_OiOiOi gEasyChatWord_OhYeah gEasyChatWord_Oh gEasyChatWord_Oops gEasyChatWord_Shocked gEasyChatWord_Eek gEasyChatWord_Graaah gEasyChatWord_Gwahahaha gEasyChatWord_Way gEasyChatWord_Tch gEasyChatWord_Hehe gEasyChatWord_Hah gEasyChatWord_Yup gEasyChatWord_Hahaha gEasyChatWord_Aiyeeh gEasyChatWord_Hiyah gEasyChatWord_Fufufu gEasyChatWord_Lol gEasyChatWord_Snort gEasyChatWord_Humph gEasyChatWord_Hehehe gEasyChatWord_Heh gEasyChatWord_Hohoho gEasyChatWord_UhHuh gEasyChatWord_OhDear gEasyChatWord_Arrgh gEasyChatWord_Mufufu gEasyChatWord_Mmm gEasyChatWord_OhKay gEasyChatWord_Okay gEasyChatWord_Lalala gEasyChatWord_Yay gEasyChatWord_Aww gEasyChatWord_Wowee gEasyChatWord_Gwah gEasyChatWord_Wahahaha gEasyChatGroup_Voices gEasyChatWord_Listen gEasyChatWord_NotVery gEasyChatWord_Mean gEasyChatWord_Lie gEasyChatWord_Lay gEasyChatWord_Recommend gEasyChatWord_Nitwit gEasyChatWord_Quite gEasyChatWord_From gEasyChatWord_Feeling gEasyChatWord_But gEasyChatWord_However gEasyChatWord_Case gEasyChatWord_The gEasyChatWord_Miss gEasyChatWord_How gEasyChatWord_Hit gEasyChatWord_Enough gEasyChatWord_ALot gEasyChatWord_ALittle gEasyChatWord_Absolutely gEasyChatWord_And gEasyChatWord_Only gEasyChatWord_Around gEasyChatWord_Probably gEasyChatWord_If gEasyChatWord_Very gEasyChatWord_ATinyBit gEasyChatWord_Wild gEasyChatWord_Thats gEasyChatWord_Just gEasyChatWord_EvenSo gEasyChatWord_MustBe gEasyChatWord_Naturally gEasyChatWord_ForNow gEasyChatWord_Understood gEasyChatWord_Joking gEasyChatWord_Ready gEasyChatWord_Something gEasyChatWord_Somehow gEasyChatWord_Although gEasyChatWord_Also gEasyChatWord_Perfect gEasyChatWord_AsMuchAs gEasyChatWord_Really gEasyChatWord_Truly gEasyChatWord_Seriously gEasyChatWord_Totally gEasyChatWord_Until gEasyChatWord_AsIf gEasyChatWord_Mood gEasyChatWord_Rather gEasyChatWord_Awfully gEasyChatWord_Mode gEasyChatWord_More gEasyChatWord_TooLate gEasyChatWord_Finally gEasyChatWord_Any gEasyChatWord_Instead gEasyChatWord_Fantastic gEasyChatGroup_Speech gEasyChatWord_Will gEasyChatWord_WillBeHere gEasyChatWord_Or gEasyChatWord_Times gEasyChatWord_Wonder gEasyChatWord_IsItQues gEasyChatWord_Be gEasyChatWord_Gimme gEasyChatWord_Could gEasyChatWord_LikelyTo gEasyChatWord_Would gEasyChatWord_Is gEasyChatWord_IsntItQues gEasyChatWord_Lets gEasyChatWord_Other gEasyChatWord_Are gEasyChatWord_Was gEasyChatWord_Were gEasyChatWord_Those gEasyChatWord_Isnt gEasyChatWord_Wont gEasyChatWord_Cant gEasyChatWord_Can gEasyChatWord_Dont gEasyChatWord_Do gEasyChatWord_Does gEasyChatWord_Whom gEasyChatWord_Which gEasyChatWord_Wasnt gEasyChatWord_Werent gEasyChatWord_Have gEasyChatWord_Havent gEasyChatWord_A gEasyChatWord_An gEasyChatWord_Not gEasyChatWord_There gEasyChatWord_OkQues gEasyChatWord_So gEasyChatWord_Maybe gEasyChatWord_About gEasyChatWord_Over gEasyChatWord_It gEasyChatWord_All gEasyChatWord_For gEasyChatWord_On gEasyChatWord_Off gEasyChatWord_As gEasyChatWord_To gEasyChatWord_With gEasyChatWord_Better gEasyChatWord_Ever gEasyChatWord_Since gEasyChatWord_Of gEasyChatWord_BelongsTo gEasyChatWord_At gEasyChatWord_In gEasyChatWord_Out gEasyChatWord_Too gEasyChatWord_Like gEasyChatWord_Did gEasyChatWord_Didnt gEasyChatWord_Doesnt gEasyChatWord_Without gEasyChatWord_After gEasyChatWord_Before gEasyChatWord_While gEasyChatWord_Than gEasyChatWord_Once gEasyChatWord_Anywhere gEasyChatGroup_Endings gEasyChatWord_Meet gEasyChatWord_Play gEasyChatWord_Hurried gEasyChatWord_Goes gEasyChatWord_Giddy gEasyChatWord_Happy gEasyChatWord_Happiness gEasyChatWord_Excite gEasyChatWord_Important gEasyChatWord_Funny gEasyChatWord_Got gEasyChatWord_GoHome gEasyChatWord_Disappointed gEasyChatWord_Disappoints gEasyChatWord_Sad gEasyChatWord_Try gEasyChatWord_Tries gEasyChatWord_Hears gEasyChatWord_Think gEasyChatWord_Hear gEasyChatWord_Wants gEasyChatWord_Misheard gEasyChatWord_Dislike gEasyChatWord_Angry gEasyChatWord_Anger gEasyChatWord_Scary gEasyChatWord_Lonesome gEasyChatWord_Disappoint gEasyChatWord_Joy gEasyChatWord_Gets gEasyChatWord_Never gEasyChatWord_Darn gEasyChatWord_Downcast gEasyChatWord_Incredible gEasyChatWord_Likes gEasyChatWord_Dislikes gEasyChatWord_Boring gEasyChatWord_Care gEasyChatWord_Cares gEasyChatWord_AllRight gEasyChatWord_Adore gEasyChatWord_Disaster gEasyChatWord_Enjoy gEasyChatWord_Enjoys gEasyChatWord_Eat gEasyChatWord_Lacking gEasyChatWord_Bad gEasyChatWord_Hard gEasyChatWord_Terrible gEasyChatWord_Should gEasyChatWord_Nice gEasyChatWord_Drink gEasyChatWord_Surprise gEasyChatWord_Fear gEasyChatWord_Want gEasyChatWord_Wait gEasyChatWord_Satisfied gEasyChatWord_See gEasyChatWord_Rare gEasyChatWord_Negative gEasyChatWord_Done gEasyChatWord_Danger gEasyChatWord_Defeated gEasyChatWord_Beat gEasyChatWord_Great gEasyChatWord_Romantic gEasyChatWord_Question gEasyChatWord_Understand gEasyChatWord_Understands gEasyChatGroup_Feelings gEasyChatWord_Hot gEasyChatWord_Exists gEasyChatWord_Excess gEasyChatWord_Approved gEasyChatWord_Has gEasyChatWord_Good gEasyChatWord_Less gEasyChatWord_Momentum gEasyChatWord_Going gEasyChatWord_Weird gEasyChatWord_Busy gEasyChatWord_Together gEasyChatWord_Full gEasyChatWord_Absent gEasyChatWord_Being gEasyChatWord_Need gEasyChatWord_Tasty gEasyChatWord_Skilled gEasyChatWord_Noisy gEasyChatWord_Big gEasyChatWord_Late gEasyChatWord_Close gEasyChatWord_Docile gEasyChatWord_Amusing gEasyChatWord_Entertaining gEasyChatWord_Perfection gEasyChatWord_Pretty gEasyChatWord_Healthy gEasyChatWord_Excellent gEasyChatWord_UpsideDown gEasyChatWord_Cold gEasyChatWord_Refreshing gEasyChatWord_Unavoidable gEasyChatWord_Much gEasyChatWord_Overwhelming gEasyChatWord_Fabulous gEasyChatWord_Else gEasyChatWord_Expensive gEasyChatWord_Correct gEasyChatWord_Impossible gEasyChatWord_Small gEasyChatWord_Different gEasyChatWord_Tired gEasyChatWord_Skill gEasyChatWord_Top gEasyChatWord_NonStop gEasyChatWord_Preposterous gEasyChatWord_None gEasyChatWord_Nothing gEasyChatWord_Natural gEasyChatWord_Becomes gEasyChatWord_Lukewarm gEasyChatWord_Fast gEasyChatWord_Low gEasyChatWord_Awful gEasyChatWord_Alone gEasyChatWord_Bored gEasyChatWord_Secret gEasyChatWord_Mystery gEasyChatWord_Lacks gEasyChatWord_Best gEasyChatWord_Lousy gEasyChatWord_Mistake gEasyChatWord_Kind gEasyChatWord_Well gEasyChatWord_Weakened gEasyChatWord_Simple gEasyChatWord_Seems gEasyChatWord_Badly gEasyChatGroup_Conditions gEasyChatWord_Meets gEasyChatWord_Concede gEasyChatWord_Give gEasyChatWord_Gives gEasyChatWord_Played gEasyChatWord_Plays gEasyChatWord_Collect gEasyChatWord_Walking gEasyChatWord_Walks gEasyChatWord_Says gEasyChatWord_Went gEasyChatWord_Said gEasyChatWord_WakeUp gEasyChatWord_WakesUp gEasyChatWord_Angers gEasyChatWord_Teach gEasyChatWord_Teaches gEasyChatWord_Please gEasyChatWord_Learn gEasyChatWord_Change gEasyChatWord_Story gEasyChatWord_Trust gEasyChatWord_Lavish gEasyChatWord_Listens gEasyChatWord_Hearing gEasyChatWord_Trains gEasyChatWord_Choose gEasyChatWord_Come gEasyChatWord_Came gEasyChatWord_Search gEasyChatWord_Make gEasyChatWord_Cause gEasyChatWord_Know gEasyChatWord_Knows gEasyChatWord_Refuse gEasyChatWord_Stores gEasyChatWord_Brag gEasyChatWord_Ignorant gEasyChatWord_Thinks gEasyChatWord_Believe gEasyChatWord_Slide gEasyChatWord_Eats gEasyChatWord_Use gEasyChatWord_Uses gEasyChatWord_Using gEasyChatWord_Couldnt gEasyChatWord_Capable gEasyChatWord_Disappear gEasyChatWord_Appear gEasyChatWord_Throw gEasyChatWord_Worry gEasyChatWord_Slept gEasyChatWord_Sleep gEasyChatWord_Release gEasyChatWord_Drinks gEasyChatWord_Runs gEasyChatWord_Run gEasyChatWord_Works gEasyChatWord_Working gEasyChatWord_Talking gEasyChatWord_Talk gEasyChatWord_Sink gEasyChatWord_Smack gEasyChatWord_Pretend gEasyChatWord_Praise gEasyChatWord_Overdo gEasyChatWord_Show gEasyChatWord_Looks gEasyChatWord_Sees gEasyChatWord_Seek gEasyChatWord_Own gEasyChatWord_Take gEasyChatWord_Allow gEasyChatWord_Forget gEasyChatWord_Forgets gEasyChatWord_Appears gEasyChatWord_Faint gEasyChatWord_Fainted gEasyChatGroup_Actions gEasyChatWord_Chores gEasyChatWord_Home gEasyChatWord_Money gEasyChatWord_Allowance gEasyChatWord_Bath gEasyChatWord_Conversation gEasyChatWord_School gEasyChatWord_Commemorate gEasyChatWord_Habit gEasyChatWord_Group gEasyChatWord_Word gEasyChatWord_Store gEasyChatWord_Service gEasyChatWord_Work gEasyChatWord_System gEasyChatWord_Train gEasyChatWord_Class gEasyChatWord_Lessons gEasyChatWord_Information gEasyChatWord_Living gEasyChatWord_Teacher gEasyChatWord_Tournament gEasyChatWord_Letter gEasyChatWord_Event gEasyChatWord_Digital gEasyChatWord_Test gEasyChatWord_DeptStore gEasyChatWord_Television gEasyChatWord_Phone gEasyChatWord_Item gEasyChatWord_Name gEasyChatWord_News gEasyChatWord_Popular gEasyChatWord_Party gEasyChatWord_Study gEasyChatWord_Machine gEasyChatWord_Mail gEasyChatWord_Message gEasyChatWord_Promise gEasyChatWord_Dream gEasyChatWord_Kindergarten gEasyChatWord_Life gEasyChatWord_Radio gEasyChatWord_Rental gEasyChatWord_World gEasyChatGroup_Lifestyle gEasyChatWord_Idol gEasyChatWord_Anime gEasyChatWord_Song gEasyChatWord_Movie gEasyChatWord_Sweets gEasyChatWord_Chat gEasyChatWord_ChildsPlay gEasyChatWord_Toys gEasyChatWord_Music gEasyChatWord_Cards gEasyChatWord_Shopping gEasyChatWord_Camera gEasyChatWord_Viewing gEasyChatWord_Spectator gEasyChatWord_Gourmet gEasyChatWord_Game gEasyChatWord_Rpg gEasyChatWord_Collection gEasyChatWord_Complete gEasyChatWord_Magazine gEasyChatWord_Walk gEasyChatWord_Bike gEasyChatWord_Hobby gEasyChatWord_Sports gEasyChatWord_Software gEasyChatWord_Songs gEasyChatWord_Diet gEasyChatWord_Treasure gEasyChatWord_Travel gEasyChatWord_Dance gEasyChatWord_Channel gEasyChatWord_Making gEasyChatWord_Fishing gEasyChatWord_Date gEasyChatWord_Design gEasyChatWord_Locomotive gEasyChatWord_PlushDoll gEasyChatWord_Pc gEasyChatWord_Flowers gEasyChatWord_Hero gEasyChatWord_Nap gEasyChatWord_Heroine gEasyChatWord_Fashion gEasyChatWord_Adventure gEasyChatWord_Board gEasyChatWord_Ball gEasyChatWord_Book gEasyChatWord_Festival gEasyChatWord_Comics gEasyChatWord_Holiday gEasyChatWord_Plans gEasyChatWord_Trendy gEasyChatWord_Vacation gEasyChatWord_Look gEasyChatGroup_Hobbies gEasyChatWord_Fall gEasyChatWord_Morning gEasyChatWord_Tomorrow gEasyChatWord_Last gEasyChatWord_Day gEasyChatWord_Sometime gEasyChatWord_Always gEasyChatWord_Current gEasyChatWord_Forever gEasyChatWord_Days gEasyChatWord_End gEasyChatWord_Tuesday gEasyChatWord_Yesterday gEasyChatWord_Today gEasyChatWord_Friday gEasyChatWord_Monday gEasyChatWord_Later gEasyChatWord_Earlier gEasyChatWord_Another gEasyChatWord_Time gEasyChatWord_Finish gEasyChatWord_Wednesday gEasyChatWord_Soon gEasyChatWord_Start gEasyChatWord_Month gEasyChatWord_Stop gEasyChatWord_Now gEasyChatWord_Final gEasyChatWord_Next gEasyChatWord_Age gEasyChatWord_Saturday gEasyChatWord_Summer gEasyChatWord_Sunday gEasyChatWord_Beginning gEasyChatWord_Spring gEasyChatWord_Daytime gEasyChatWord_Winter gEasyChatWord_Daily gEasyChatWord_Olden gEasyChatWord_Almost gEasyChatWord_Nearly gEasyChatWord_Thursday gEasyChatWord_Nighttime gEasyChatWord_Night gEasyChatWord_Week gEasyChatGroup_Time gEasyChatWord_Highs gEasyChatWord_Lows gEasyChatWord_Um gEasyChatWord_Rear gEasyChatWord_Things gEasyChatWord_Thing gEasyChatWord_Below gEasyChatWord_Above gEasyChatWord_Back gEasyChatWord_High gEasyChatWord_Here gEasyChatWord_Inside gEasyChatWord_Outside gEasyChatWord_Beside gEasyChatWord_ThisIsItExcl gEasyChatWord_This gEasyChatWord_Every gEasyChatWord_These gEasyChatWord_TheseWere gEasyChatWord_Down gEasyChatWord_That gEasyChatWord_ThoseAre gEasyChatWord_ThoseWere gEasyChatWord_ThatsItExcl gEasyChatWord_Am gEasyChatWord_ThatWas gEasyChatWord_Front gEasyChatWord_Up gEasyChatWord_Choice gEasyChatWord_Far gEasyChatWord_Away gEasyChatWord_Near gEasyChatWord_Where gEasyChatWord_When gEasyChatWord_What gEasyChatWord_Deep gEasyChatWord_Shallow gEasyChatWord_Why gEasyChatWord_Confused gEasyChatWord_Opposite gEasyChatWord_Left gEasyChatWord_Right gEasyChatGroup_Misc gEasyChatWord_Wandering gEasyChatWord_Rickety gEasyChatWord_RockSolid gEasyChatWord_Hungry gEasyChatWord_Tight gEasyChatWord_Ticklish gEasyChatWord_Twirling gEasyChatWord_Spiraling gEasyChatWord_Thirsty gEasyChatWord_Lolling gEasyChatWord_Silky gEasyChatWord_Sadly gEasyChatWord_Hopeless gEasyChatWord_Useless gEasyChatWord_Drooling gEasyChatWord_Exciting gEasyChatWord_Thick gEasyChatWord_Smooth gEasyChatWord_Slimy gEasyChatWord_Thin gEasyChatWord_Break gEasyChatWord_Voracious gEasyChatWord_Scatter gEasyChatWord_Awesome gEasyChatWord_Wimpy gEasyChatWord_Wobbly gEasyChatWord_Shaky gEasyChatWord_Ripped gEasyChatWord_Shredded gEasyChatWord_Increasing gEasyChatWord_Yet gEasyChatWord_Destroyed gEasyChatWord_Fiery gEasyChatWord_LoveyDovey gEasyChatWord_Happily gEasyChatWord_Anticipation gEasyChatGroup_Adjectives gEasyChatWord_Appeal gEasyChatWord_Events gEasyChatWord_StayAtHome gEasyChatWord_Berry gEasyChatWord_Contest gEasyChatWord_Mc gEasyChatWord_Judge gEasyChatWord_Super gEasyChatWord_Stage gEasyChatWord_HallOfFame gEasyChatWord_Evolution gEasyChatWord_Hyper gEasyChatWord_BattleTower gEasyChatWord_Leaders gEasyChatWord_BattleRoom gEasyChatWord_Hidden gEasyChatWord_SecretBase gEasyChatWord_Blend gEasyChatWord_POKEBLOCK gEasyChatWord_Master gEasyChatWord_Rank gEasyChatWord_Ribbon gEasyChatWord_Crush gEasyChatWord_Direct gEasyChatWord_Tower gEasyChatWord_Union gEasyChatWord_Room gEasyChatWord_Wireless gEasyChatWord_Frontier gEasyChatGroup_Events gEasyChatGroup_Move1 gEasyChatGroup_Move2 gEasyChatWord_KthxBye gEasyChatWord_YesSirExcl gEasyChatWord_AvantGarde gEasyChatWord_Couple gEasyChatWord_MuchObliged gEasyChatWord_YeehawExcl gEasyChatWord_Mega gEasyChatWord_1HitKOExcl gEasyChatWord_Destiny gEasyChatWord_Cancel gEasyChatWord_New gEasyChatWord_Flatten gEasyChatWord_Kidding gEasyChatWord_Loser gEasyChatWord_Losing gEasyChatWord_Happening gEasyChatWord_HipAnd gEasyChatWord_Shake gEasyChatWord_Shady gEasyChatWord_Upbeat gEasyChatWord_Modern gEasyChatWord_SmellYa gEasyChatWord_Bang gEasyChatWord_Knockout gEasyChatWord_Hassle gEasyChatWord_Winner gEasyChatWord_Fever gEasyChatWord_Wannabe gEasyChatWord_Baby gEasyChatWord_Heart gEasyChatWord_Old gEasyChatWord_Young gEasyChatWord_Ugly gEasyChatGroup_TrendySaying gEasyChatGroup_Pokemon2 gEasyChatGroups gEasyChatWordsByLetter_Others gEasyChatWordsByLetter_A gEasyChatWordsByLetter_B gEasyChatWordsByLetter_C gEasyChatWordsByLetter_D gEasyChatWordsByLetter_E gEasyChatWordsByLetter_F gEasyChatWordsByLetter_G gEasyChatWordsByLetter_H gEasyChatWordsByLetter_I gEasyChatWordsByLetter_J gEasyChatWordsByLetter_K gEasyChatWordsByLetter_L gEasyChatWordsByLetter_M gEasyChatWordsByLetter_N gEasyChatWordsByLetter_O gEasyChatWordsByLetter_P gEasyChatWordsByLetter_Q gEasyChatWordsByLetter_R gEasyChatWordsByLetter_S gEasyChatWordsByLetter_T gEasyChatWordsByLetter_U gEasyChatWordsByLetter_V gEasyChatWordsByLetter_W gEasyChatWordsByLetter_X gEasyChatWordsByLetter_Y gEasyChatWordsByLetter_Z gEasyChatWordsByLetter_UnusedJapaneseHi gEasyChatWordsByLetter_UnusedJapaneseFu gEasyChatWordsByLetter_UnusedJapaneseHe gEasyChatWordsByLetter_UnusedJapaneseHo gEasyChatWordsByLetter_UnusedJapaneseMa gEasyChatWordsByLetter_UnusedJapaneseMi gEasyChatWordsByLetter_UnusedJapaneseMu gEasyChatWordsByLetter_UnusedJapaneseMe gEasyChatWordsByLetter_UnusedJapaneseMo gEasyChatWordsByLetter_UnusedJapaneseYa gEasyChatWordsByLetter_UnusedJapaneseYu gEasyChatWordsByLetter_UnusedJapaneseYo gEasyChatWordsByLetter_UnusedJapaneseRa gEasyChatWordsByLetter_UnusedJapaneseRi gEasyChatWordsByLetter_UnusedJapaneseRu gEasyChatWordsByLetter_UnusedJapaneseRe gEasyChatWordsByLetter_UnusedJapaneseRo gEasyChatWordsByLetter_UnusedJapaneseWa gEasyChatWordsByLetterPointers sEasyChatGroupNamePointers sDefaultProfileWords sDefaultBattleStartWords sDefaultBattleWonWords sDefaultBattleLostWords sRestrictedWordSpecies

/// `__typeof__(sQuizLadyEasyChatScreens[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sQuizLadyEasyChatScreens_0_t {
    pub funcId: u16,
    pub callback: Option<unsafe extern "C" fn()>,
}

unsafe impl Sync for sQuizLadyEasyChatScreens_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sQuizLadyEasyChatScreens_0_t>() == 8);
    assert!(offset_of!(sQuizLadyEasyChatScreens_0_t, funcId) == 0);
    assert!(offset_of!(sQuizLadyEasyChatScreens_0_t, callback) == 4);
};

const ECFUNC_CLOSE_KEYBOARD: i32 = 10;
const ECFUNC_CLOSE_PROMPT: u16 = 7;
const ECFUNC_CLOSE_PROMPT_AFTER_DELETE: u16 = 8;
const ECFUNC_CLOSE_WORD_SELECT: i32 = 12;
const ECFUNC_EXIT: u16 = 24;
const ECFUNC_GROUP_NAMES_SCROLL_DOWN: i32 = 16;
const ECFUNC_GROUP_NAMES_SCROLL_UP: i32 = 17;
const ECFUNC_MSG_CANT_DELETE_LYRICS: i32 = 32;
const ECFUNC_MSG_CANT_EXIT: u16 = 34;
const ECFUNC_MSG_COMBINE_TWO_WORDS: u16 = 33;
const ECFUNC_MSG_CREATE_QUIZ: u16 = 29;
const ECFUNC_MSG_SELECT_ANSWER: u16 = 30;
const ECFUNC_MSG_SONG_TOO_SHORT: u16 = 31;
const ECFUNC_NONE: u16 = 0;
const ECFUNC_OPEN_KEYBOARD: u16 = 9;
const ECFUNC_OPEN_WORD_SELECT: i32 = 11;
const ECFUNC_PROMPT_CONFIRM: u16 = 6;
const ECFUNC_PROMPT_CONFIRM_LYRICS: i32 = 13;
const ECFUNC_PROMPT_DELETE_ALL: i32 = 4;
const ECFUNC_PROMPT_EXIT: u16 = 5;
const ECFUNC_QUIZ_ANSWER: u16 = 26;
const ECFUNC_QUIZ_QUESTION: i32 = 25;
const ECFUNC_REPRINT_PHRASE: i32 = 1;
const ECFUNC_RETURN_TO_KEYBOARD: u16 = 14;
const ECFUNC_SET_QUIZ_ANSWER: i32 = 28;
const ECFUNC_SET_QUIZ_QUESTION: i32 = 27;
const ECFUNC_SWITCH_KEYBOARD_MODE: i32 = 23;
const ECFUNC_UPDATE_KEYBOARD_CURSOR: i32 = 15;
const ECFUNC_UPDATE_MAIN_CURSOR: u16 = 2;
const ECFUNC_UPDATE_MAIN_CURSOR_ON_BUTTONS: u16 = 3;
const ECFUNC_UPDATE_WORD_SELECT_CURSOR: u16 = 18;
const ECFUNC_WORD_SELECT_PAGE_DOWN: u16 = 22;
const ECFUNC_WORD_SELECT_PAGE_UP: u16 = 21;
const ECFUNC_WORD_SELECT_SCROLL_DOWN: u16 = 20;
const ECFUNC_WORD_SELECT_SCROLL_UP: u16 = 19;
const FOOTER_ANSWER: u8 = 2;
const FOOTER_NORMAL: i32 = 0;
const FOOTER_QUIZ: u8 = 1;
const FRAMEID_INTERVIEW_SHOW_PERSON: u8 = 4;
const FRAMEID_MAIL: u8 = 2;
const FRAMEID_QUIZ_QUESTION: i32 = 7;
const FRAMEID_QUIZ_SET_QUESTION: u8 = 8;
const FRAME_OFFSET_GREEN: u16 = 16384;
const FRAME_OFFSET_ORANGE: u16 = 4096;
const FRAME_TILE_BOTTOM_EDGE: i32 = 10;
const FRAME_TILE_BOTTOM_L_CORNER: i32 = 9;
const FRAME_TILE_BOTTOM_R_CORNER: i32 = 11;
const FRAME_TILE_L_EDGE: i32 = 5;
const FRAME_TILE_R_EDGE: i32 = 7;
const FRAME_TILE_TOP_EDGE: i32 = 2;
const FRAME_TILE_TOP_L_CORNER: i32 = 1;
const FRAME_TILE_TOP_R_CORNER: i32 = 3;
const FRAME_TILE_TRANSPARENT: i32 = 0;
const INPUTSTATE_CONFIRM_LYRICS_YES_NO: u8 = 10;
const INPUTSTATE_CONFIRM_WORDS_YES_NO: u8 = 6;
const INPUTSTATE_DELETE_ALL_YES_NO: u8 = 5;
const INPUTSTATE_EXIT_PROMPT: u8 = 4;
const INPUTSTATE_KEYBOARD: u8 = 2;
const INPUTSTATE_MAIN_SCREEN_BUTTONS: u8 = 1;
const INPUTSTATE_PHRASE: u8 = 0;
const INPUTSTATE_QUIZ_QUESTION: u8 = 7;
const INPUTSTATE_START_CONFIRM_LYRICS: u8 = 9;
const INPUTSTATE_WAIT_FOR_MSG: u8 = 8;
const INPUTSTATE_WORD_SELECT: u8 = 3;
const INPUT_DOWN: u32 = 3;
const INPUT_LEFT: u32 = 1;
const INPUT_RIGHT: u32 = 0;
const INPUT_SELECT: u32 = 5;
const INPUT_START: u32 = 4;
const INPUT_UP: u32 = 2;
const MAINSTATE_EXIT: i16 = 4;
const MAINSTATE_FADE_IN: i16 = 0;
const MAINSTATE_HANDLE_INPUT: i16 = 1;
const MAINSTATE_RUN_FUNC: i16 = 2;
const MAINSTATE_TO_QUIZ_LADY: i16 = 3;
const MAINSTATE_WAIT_FADE_IN: i16 = 5;
const MODEWINDOW_ANIM_TO_ALPHABET: u8 = 2;
const MODEWINDOW_ANIM_TO_GROUP: u8 = 1;
const MODEWINDOW_ANIM_TO_HIDDEN: u8 = 3;
const MODEWINDOW_ANIM_TRANSITION: u8 = 4;
const MSG_CANT_DELETE_LYRICS: u8 = 7;
const MSG_CANT_QUIT: u8 = 9;
const MSG_COMBINE_TWO_WORDS: u8 = 8;
const MSG_CONFIRM: u8 = 3;
const MSG_CONFIRM_DELETE: u8 = 1;
const MSG_CONFIRM_EXIT: u8 = 2;
const MSG_CREATE_QUIZ: u8 = 4;
const MSG_INSTRUCTIONS: u8 = 0;
const MSG_SELECT_ANSWER: u8 = 5;
const MSG_SONG_TOO_SHORT: u8 = 6;
const NUM_ALPHABET_COLUMNS: u8 = 7;
const NUM_ALPHABET_ROWS: u8 = 4;
const NUM_BUTTON_ROWS: i32 = 3;
const NUM_FOOTER_TYPES: i32 = 3;
const NUM_GROUP_NAME_COLUMNS: u16 = 2;
const NUM_GROUP_NAME_ROWS: i32 = 4;
const NUM_WORD_SELECT_COLUMNS: u16 = 2;
const NUM_WORD_SELECT_ROWS: u8 = 4;
const RECTCURSOR_ANIM_ON_BUTTON: u8 = 1;
const RECTCURSOR_ANIM_ON_GROUP: u8 = 0;
const RECTCURSOR_ANIM_ON_LETTER: i32 = 3;
const RECTCURSOR_ANIM_ON_OTHERS: i32 = 2;
const TASKIDX_EXIT_CALLBACK: u8 = 4;
const TASKIDX_WORDS: u8 = 2;
const TEXT_ALPHABET: u32 = 1;
const TEXT_GROUPS: u32 = 0;
const TEXT_WORD_SELECT: u32 = 2;
const WINANIM_CLOSE_KEYBOARD: i32 = 1;
const WINANIM_CLOSE_WORD_SELECT: i32 = 3;
const WINANIM_KEYBOARD_SWITCH_IN: i32 = 6;
const WINANIM_KEYBOARD_SWITCH_OUT: i32 = 5;
const WINANIM_OPEN_KEYBOARD: i32 = 0;
const WINANIM_OPEN_WORD_SELECT: i32 = 2;
const WINANIM_RETURN_TO_KEYBOARD: i32 = 4;
const WIN_INPUT_SELECT: u8 = 2;
const WIN_MSG: u8 = 1;
const WIN_TITLE: u8 = 0;

static gEasyChatGroups: Table<CArray<EasyChatGroup, 22>> =
    Table((&raw const crate::data::easy_chat::gEasyChatGroups).cast());
static gEasyChatWordsByLetterPointers: Table<CArray<EasyChatWordsByLetter, 45>> =
    Table((&raw const crate::data::easy_chat::gEasyChatWordsByLetterPointers).cast());
static sAlphabetGroupIdMap: Table<CArray<CArray<u8, 7>, 4>> =
    Table((&raw const crate::data::easy_chat::sAlphabetGroupIdMap).cast());
static sAlphabetKeyboardColumnOffsets: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::easy_chat::sAlphabetKeyboardColumnOffsets).cast());
static sBerryMasterWifePhrases: Table<CArray<CArray<u16, 2>, 5>> =
    Table((&raw const crate::data::easy_chat::sBerryMasterWifePhrases).cast());
static sCompressedSpriteSheets: Table<CArray<CompressedSpriteSheet, 4>> =
    Table((&raw const crate::data::easy_chat::sCompressedSpriteSheets).cast());
static sDefaultBattleLostWords: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::easy_chat::sDefaultBattleLostWords).cast());
static sDefaultBattleStartWords: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::easy_chat::sDefaultBattleStartWords).cast());
static sDefaultBattleWonWords: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::easy_chat::sDefaultBattleWonWords).cast());
static sDefaultProfileWords: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::easy_chat::sDefaultProfileWords).cast());
static sEasyChatBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::easy_chat::sEasyChatBgTemplates).cast());
static sEasyChatGroupNamePointers: Table<CArray<*mut u8, 22>> =
    Table((&raw const crate::data::easy_chat::sEasyChatGroupNamePointers).cast());
static sEasyChatKeyboardAlphabet: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::easy_chat::sEasyChatKeyboardAlphabet).cast());
static sEasyChatScreenTemplates: Table<CArray<EasyChatScreenTemplate, 21>> =
    Table((&raw const crate::data::easy_chat::sEasyChatScreenTemplates).cast());
static sEasyChatWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::easy_chat::sEasyChatWindowTemplates).cast());
static sEasyChatYesNoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::easy_chat::sEasyChatYesNoWindowTemplate).cast());
static sFooterOptionXOffsets: Table<CArray<CArray<u8, 4>, 3>> =
    Table((&raw const crate::data::easy_chat::sFooterOptionXOffsets).cast());
static sFooterTextOptions: Table<CArray<CArray<*mut u8, 4>, 3>> =
    Table((&raw const crate::data::easy_chat::sFooterTextOptions).cast());
static sMysteryGiftPhrase: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::easy_chat::sMysteryGiftPhrase).cast());
static sPhraseFrameDimensions: Table<CArray<EasyChatPhraseFrameDimensions, 9>> =
    Table((&raw const crate::data::easy_chat::sPhraseFrameDimensions).cast());
static sQuizLadyEasyChatScreens: Table<CArray<sQuizLadyEasyChatScreens_0_t, 4>> =
    Table((&raw const crate::data::easy_chat::sQuizLadyEasyChatScreens).cast());
static sRestrictedWordSpecies: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::easy_chat::sRestrictedWordSpecies).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 5>> =
    Table((&raw const crate::data::easy_chat::sSpritePalettes).cast());
static sSpriteSheets: Table<CArray<SpriteSheet, 4>> =
    Table((&raw const crate::data::easy_chat::sSpriteSheets).cast());
static sSpriteTemplate_ButtonWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_ButtonWindow).cast());
static sSpriteTemplate_ModeWindow: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_ModeWindow).cast());
static sSpriteTemplate_RectangleCursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_RectangleCursor).cast());
static sSpriteTemplate_ScrollIndicator: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_ScrollIndicator).cast());
static sSpriteTemplate_StartSelectButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_StartSelectButton).cast());
static sSpriteTemplate_TriangleCursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::easy_chat::sSpriteTemplate_TriangleCursor).cast());
static sTextInputFrameGreen_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::easy_chat::sTextInputFrameGreen_Pal).cast());
static sTextInputFrameOrange_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::easy_chat::sTextInputFrameOrange_Pal).cast());
static sTextInputFrame_Gfx: Table<CArray<u32, 50>> =
    Table((&raw const crate::data::easy_chat::sTextInputFrame_Gfx).cast());
static sText_Clear17: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::easy_chat::sText_Clear17).cast());
static sText_Pal: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::easy_chat::sText_Pal).cast());
static sTitleText_Pal: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::easy_chat::sTitleText_Pal).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sEasyChatScreen: *mut EasyChatScreen = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScreenControl: *mut EasyChatScreenControl = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWordData: *mut EasyChatScreenWordData = null_mut();

unsafe extern "C" {
    static gEasyChatMode_Pal: CArray<u16, 0>;
    static gEasyChatWindow_Gfx: CArray<u32, 0>;
    static gEasyChatWindow_Tilemap: CArray<u32, 0>;
    static mut gMain: Main;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static gNumBardWords_Moves: u16;
    static gNumBardWords_Species: u16;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AllTextBeingEditedWill: CArray<u8, 0>;
    static gText_BeDeletedThatOkay: CArray<u8, 0>;
    static gText_ChallengeQuestionMark: CArray<u8, 0>;
    static gText_CombineTwoWordsOrPhrases3: CArray<u8, 0>;
    static gText_CreateAQuiz: CArray<u8, 0>;
    static gText_F700sQuiz: CArray<u8, 0>;
    static gText_Lady: CArray<u8, 0>;
    static gText_LikeToQuitQuiz: CArray<u8, 0>;
    static gText_LyricsCantBeDeleted: CArray<u8, 0>;
    static gText_OnlyOnePhrase: CArray<u8, 0>;
    static gText_OriginalSongWillBeUsed: CArray<u8, 0>;
    static gText_QuitEditing: CArray<u8, 0>;
    static gText_SectionMustBeCompleted: CArray<u8, 0>;
    static gText_SelectTheAnswer: CArray<u8, 0>;
    static gText_StopGivingPkmnMail: CArray<u8, 0>;
    static gText_ThreeQuestionMarks: CArray<u8, 0>;
    static gText_YouCannotQuitHere: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScript();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CleanupOverworldWindowsAndTilemaps();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut Sprite);
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
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetBgY(a0: u8) -> i32;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetQuestionnaireWordsPtr() -> *mut u16;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsOverworldLinkActive() -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
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
    let mut taskId: u8 = 0;
    ResetTasks();
    taskId = CreateTask(Some(Task_InitEasyChatScreen), 0);
    gTasks[taskId].data[1] = r#type as i16;
    gTasks[taskId].data[7] = displayedPersonType as i16;
    SetWordTaskArg(taskId, TASKIDX_WORDS, words as usize as u32);
    SetWordTaskArg(
        taskId,
        TASKIDX_EXIT_CALLBACK,
        core::mem::transmute::<_, usize>(exitCallback) as u32,
    );
    SetMainCallback2(Some(CB2_EasyChatScreen));
}
pub(crate) unsafe extern "C" fn CB2_EasyChatScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_EasyChatScreen() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
}
pub(crate) unsafe extern "C" fn StartEasyChatScreen(
    taskId: u8,
    taskFunc: Option<unsafe extern "C" fn(u8)>,
) {
    gTasks[taskId].func = taskFunc;
    gTasks[taskId].data[0] = MAINSTATE_FADE_IN;
}
pub(crate) unsafe extern "C" fn Task_InitEasyChatScreen(taskId: u8) {
    if IsOverworldLinkActive() == 0 {
        while InitEasyChatScreen(taskId) != 0 {}
    } else {
        if InitEasyChatScreen(taskId) == TRUE {
            return;
        }
    }
    StartEasyChatScreen(taskId, Some(Task_EasyChatScreen));
}
pub(crate) unsafe extern "C" fn Task_EasyChatScreen(taskId: u8) {
    let mut funcId: u16 = 0;
    let mut data: *mut i16 = null_mut();
    data = gTasks[taskId].data.as_mut_ptr();
    match *data {
        MAINSTATE_FADE_IN => {
            SetVBlankCallback(Some(VBlankCB_EasyChatScreen));
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            *data = MAINSTATE_WAIT_FADE_IN;
        }
        MAINSTATE_HANDLE_INPUT => {
            funcId = HandleEasyChatInput();
            if IsFuncIdForQuizLadyScreen(funcId) != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, -2, 0, 16, 0);
                *data = MAINSTATE_TO_QUIZ_LADY;
                *data.at(6) = funcId as i16;
            } else if funcId == ECFUNC_EXIT {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                *data = MAINSTATE_EXIT;
            } else if funcId != ECFUNC_NONE {
                PlaySE(SE_SELECT);
                StartEasyChatFunction(funcId);
                *data += 1;
            }
        }
        MAINSTATE_RUN_FUNC => {
            if RunEasyChatFunction() == 0 {
                *data = MAINSTATE_HANDLE_INPUT;
            }
        }
        MAINSTATE_TO_QUIZ_LADY => {
            if gPaletteFade.active() == 0 {
                EnterQuizLadyScreen(*data.at(6) as u16);
            }
        }
        MAINSTATE_EXIT => {
            if gPaletteFade.active() == 0 {
                ExitEasyChatScreen(
                    core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(GetWordTaskArg(
                        taskId,
                        TASKIDX_EXIT_CALLBACK,
                    )
                        as usize),
                );
            }
        }
        MAINSTATE_WAIT_FADE_IN => {
            if gPaletteFade.active() == 0 {
                *data = MAINSTATE_HANDLE_INPUT;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreen(taskId: u8) -> u8 {
    let mut data: *mut i16 = null_mut();
    data = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            ResetPaletteFade();
        }
        1 => {
            if InitEasyChatScreenWordData() == 0 {
                ExitEasyChatScreen(
                    core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(GetWordTaskArg(
                        taskId,
                        TASKIDX_EXIT_CALLBACK,
                    )
                        as usize),
                );
            }
        }
        2 => {
            if InitEasyChatScreenStruct(
                *data.at(1) as u8,
                GetWordTaskArg(taskId, TASKIDX_WORDS) as usize as *mut u16,
                *data.at(7) as u8,
            ) == 0
            {
                ExitEasyChatScreen(
                    core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(GetWordTaskArg(
                        taskId,
                        TASKIDX_EXIT_CALLBACK,
                    )
                        as usize),
                );
            }
        }
        3 => {
            if InitEasyChatScreenControl() == 0 {
                ExitEasyChatScreen(
                    core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(GetWordTaskArg(
                        taskId,
                        TASKIDX_EXIT_CALLBACK,
                    )
                        as usize),
                );
            }
        }
        4 => {
            if LoadEasyChatScreen() != 0 {
                return TRUE;
            }
        }
        _ => {
            return FALSE;
        }
    }
    *data += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn ExitEasyChatScreen(callback: Option<unsafe extern "C" fn()>) {
    FreeEasyChatScreenControl();
    FreeEasyChatScreenStruct();
    FreeEasyChatScreenWordData();
    FreeAllWindowBuffers();
    SetMainCallback2(callback);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowEasyChatScreen() {
    let mut i: i32 = 0;
    let mut words: *mut u16 = null_mut();
    let mut bard: *mut MauvilleManBard = null_mut();
    let mut displayedPersonType: u8 = EASY_CHAT_PERSON_DISPLAY_NONE;
    match gSpecialVar_0x8004 {
        EASY_CHAT_TYPE_PROFILE => {
            words = (*gSaveBlock1Ptr).easyChatProfile.as_mut_ptr();
        }
        EASY_CHAT_TYPE_BATTLE_START => {
            words = (*gSaveBlock1Ptr).easyChatBattleStart.as_mut_ptr();
        }
        EASY_CHAT_TYPE_BATTLE_WON => {
            words = (*gSaveBlock1Ptr).easyChatBattleWon.as_mut_ptr();
        }
        EASY_CHAT_TYPE_BATTLE_LOST => {
            words = (*gSaveBlock1Ptr).easyChatBattleLost.as_mut_ptr();
        }
        4 => {
            words = (*gSaveBlock1Ptr).mail[gSpecialVar_0x8005]
                .words
                .as_mut_ptr();
        }
        6 => {
            bard = &raw mut (*gSaveBlock1Ptr).oldMan.bard;
            i = 0;
            while i < NUM_BARD_SONG_WORDS {
                (*bard).newSongLyrics[i] = (*bard).songLyrics[i];
                i += 1;
            }
            words = (*bard).newSongLyrics.as_mut_ptr();
        }
        EASY_CHAT_TYPE_INTERVIEW => {
            words = (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .bravoTrainer
                .words
                .as_mut_ptr();
            displayedPersonType = gSpecialVar_0x8006 as u8;
        }
        EASY_CHAT_TYPE_FAN_CLUB => {
            words = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .fanclubOpinions
                .words[gSpecialVar_0x8006];
            displayedPersonType = EASY_CHAT_PERSON_REPORTER_FEMALE;
        }
        EASY_CHAT_TYPE_DUMMY_SHOW => {
            words = (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .dummy
                .words
                .as_mut_ptr();
            displayedPersonType = EASY_CHAT_PERSON_REPORTER_MALE;
        }
        9 => {
            words = gStringVar3.as_mut_ptr() as *mut u16;
            *words = (*gSaveBlock1Ptr).dewfordTrends[0].words[0];
            *words.at(1) = (*gSaveBlock1Ptr).dewfordTrends[0].words[1];
        }
        EASY_CHAT_TYPE_GABBY_AND_TY => {
            words = (*gSaveBlock1Ptr).gabbyAndTyData.quote.as_mut_ptr();
            *words = EC_EMPTY_WORD;
            displayedPersonType = EASY_CHAT_PERSON_REPORTER_FEMALE;
        }
        11 => {
            words = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .bravoTrainer
                .words[gSpecialVar_0x8006];
            displayedPersonType = EASY_CHAT_PERSON_REPORTER_MALE;
        }
        EASY_CHAT_TYPE_BATTLE_TOWER_INTERVIEW => {
            words = (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .bravoTrainerTower
                .words
                .as_mut_ptr();
            displayedPersonType = EASY_CHAT_PERSON_REPORTER_FEMALE;
        }
        13 => {
            words = gStringVar3.as_mut_ptr() as *mut u16;
            InitializeEasyChatWordArray(words, 2);
        }
        EASY_CHAT_TYPE_FAN_QUESTION => {
            words = (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8005]
                .fanClubSpecial
                .words
                .as_mut_ptr();
            *words = EC_EMPTY_WORD;
            displayedPersonType = EASY_CHAT_PERSON_BOY;
        }
        15 => {
            words = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz.playerAnswer;
        }
        16 => {
            return;
        }
        17 => {
            words = (*gSaveBlock1Ptr).lilycoveLady.quiz.question.as_mut_ptr();
        }
        18 => {
            words = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz.correctAnswer;
        }
        19 => {
            words = (*gSaveBlock2Ptr).apprentices[0].speechWon.as_mut_ptr();
        }
        20 => {
            words = GetQuestionnaireWordsPtr();
        }
        _ => {
            return;
        }
    }
    CleanupOverworldWindowsAndTilemaps();
    DoEasyChatScreen(
        gSpecialVar_0x8004 as u8,
        words,
        Some(CB2_ReturnToFieldContinueScript),
        displayedPersonType,
    );
}
pub(crate) unsafe extern "C" fn CB2_QuizLadyQuestion() {
    let mut lilycoveLady: *mut LilycoveLady = null_mut();
    UpdatePaletteFade();
    match gMain.state {
        0 => {
            FadeScreen(FADE_TO_BLACK, 0);
        }
        1 => {
            if gPaletteFade.active() == 0 {
                lilycoveLady = &raw mut (*gSaveBlock1Ptr).lilycoveLady;
                (*lilycoveLady).quiz.playerAnswer = EC_EMPTY_WORD;
                CleanupOverworldWindowsAndTilemaps();
                DoQuizQuestionEasyChatScreen();
            }
            return;
        }
        _ => {}
    }
    gMain.state += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyShowQuizQuestion() {
    SetMainCallback2(Some(CB2_QuizLadyQuestion));
}
pub(crate) unsafe extern "C" fn GetQuizLadyScreenByFuncId(funcId: u16) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        if funcId == sQuizLadyEasyChatScreens[i].funcId {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn IsFuncIdForQuizLadyScreen(funcId: u16) -> u32 {
    return (if GetQuizLadyScreenByFuncId(funcId) == -1 {
        FALSE as i32
    } else {
        1
    }) as u32;
}
pub(crate) unsafe extern "C" fn EnterQuizLadyScreen(funcId: u16) {
    let mut i: i32 = 0;
    i = GetQuizLadyScreenByFuncId(funcId);
    ResetTasks();
    ExitEasyChatScreen(sQuizLadyEasyChatScreens[i].callback);
}
pub(crate) unsafe extern "C" fn DoQuizAnswerEasyChatScreen() {
    DoEasyChatScreen(
        EASY_CHAT_TYPE_QUIZ_ANSWER,
        &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz.playerAnswer,
        Some(CB2_ReturnToFieldContinueScript),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn DoQuizQuestionEasyChatScreen() {
    DoEasyChatScreen(
        EASY_CHAT_TYPE_QUIZ_QUESTION,
        (*gSaveBlock1Ptr).lilycoveLady.quiz.question.as_mut_ptr(),
        Some(CB2_ReturnToFieldContinueScript),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn DoQuizSetAnswerEasyChatScreen() {
    DoEasyChatScreen(
        EASY_CHAT_TYPE_QUIZ_SET_ANSWER,
        &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz.correctAnswer,
        Some(CB2_ReturnToFieldContinueScript),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn DoQuizSetQuestionEasyChatScreen() {
    DoEasyChatScreen(
        EASY_CHAT_TYPE_QUIZ_SET_QUESTION,
        (*gSaveBlock1Ptr).lilycoveLady.quiz.question.as_mut_ptr(),
        Some(CB2_ReturnToFieldContinueScript),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenStruct(
    r#type: u8,
    words: *mut u16,
    displayedPersonType: u8,
) -> u8 {
    let mut templateId: u8 = 0;
    let mut i: i32 = 0;
    sEasyChatScreen = Alloc(80) as *mut EasyChatScreen;
    if sEasyChatScreen.is_null() {
        return FALSE;
    }
    (*sEasyChatScreen).r#type = r#type;
    (*sEasyChatScreen).savedPhrase = words;
    (*sEasyChatScreen).mainCursorColumn = 0;
    (*sEasyChatScreen).mainCursorRow = 0;
    (*sEasyChatScreen).inAlphabetMode = FALSE;
    (*sEasyChatScreen).displayedPersonType = displayedPersonType;
    (*sEasyChatScreen).unused = 0;
    templateId = GetEachChatScreenTemplateId(r#type);
    if r#type == EASY_CHAT_TYPE_QUIZ_QUESTION {
        GetQuizTitle((*sEasyChatScreen).quizTitle.as_mut_ptr());
        (*sEasyChatScreen).titleText = (*sEasyChatScreen).quizTitle.as_mut_ptr();
        (*sEasyChatScreen).inputState = INPUTSTATE_QUIZ_QUESTION;
    } else {
        (*sEasyChatScreen).inputState = INPUTSTATE_PHRASE;
        (*sEasyChatScreen).titleText = sEasyChatScreenTemplates[templateId].titleText;
    }
    (*sEasyChatScreen).numColumns = sEasyChatScreenTemplates[templateId].numColumns;
    (*sEasyChatScreen).numRows = sEasyChatScreenTemplates[templateId].numRows;
    (*sEasyChatScreen).maxWords = (*sEasyChatScreen).numColumns * (*sEasyChatScreen).numRows;
    (*sEasyChatScreen).templateId = templateId;
    if (*sEasyChatScreen).maxWords > 9 {
        (*sEasyChatScreen).maxWords = 9;
    }
    if !words.is_null() {
        CpuSet(
            words as *mut c_void,
            (*sEasyChatScreen).currentPhrase.as_mut_ptr() as *mut c_void,
            0x00000000 | (*sEasyChatScreen).maxWords as u32 * 2 / 2 & 0x1FFFFF,
        );
    } else {
        i = 0;
        while i < (*sEasyChatScreen).maxWords as i32 {
            (*sEasyChatScreen).currentPhrase[i] = EC_EMPTY_WORD;
            i += 1;
        }
        (*sEasyChatScreen).savedPhrase = (*sEasyChatScreen).currentPhrase.as_mut_ptr();
    }
    (*sEasyChatScreen).keyboardLastRow =
        ((GetNumUnlockedEasyChatGroups() as i32 - 1) / 2) as u8 + 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenStruct() {
    if !sEasyChatScreen.is_null() {
        Free(sEasyChatScreen as *mut c_void);
        sEasyChatScreen = null_mut();
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput() -> u16 {
    match (*sEasyChatScreen).inputState {
        INPUTSTATE_PHRASE => {
            return HandleEasyChatInput_Phrase();
        }
        INPUTSTATE_MAIN_SCREEN_BUTTONS => {
            return HandleEasyChatInput_MainScreenButtons();
        }
        INPUTSTATE_KEYBOARD => {
            return HandleEasyChatInput_Keyboard();
        }
        INPUTSTATE_WORD_SELECT => {
            return HandleEasyChatInput_WordSelect();
        }
        INPUTSTATE_EXIT_PROMPT => {
            return HandleEasyChatInput_ExitPrompt();
        }
        INPUTSTATE_DELETE_ALL_YES_NO => {
            return HandleEasyChatInput_DeleteAllYesNo();
        }
        INPUTSTATE_CONFIRM_WORDS_YES_NO => {
            return HandleEasyChatInput_ConfirmWordsYesNo();
        }
        INPUTSTATE_QUIZ_QUESTION => {
            return HandleEasyChatInput_QuizQuestion();
        }
        INPUTSTATE_WAIT_FOR_MSG => {
            return HandleEasyChatInput_WaitForMsg();
        }
        INPUTSTATE_START_CONFIRM_LYRICS => {
            return HandleEasyChatInput_StartConfirmLyrics();
        }
        INPUTSTATE_CONFIRM_LYRICS_YES_NO => {
            return HandleEasyChatInput_ConfirmLyricsYesNo();
        }
        _ => {}
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn IsCurrentFrame2x5() -> u32 {
    match GetEasyChatScreenFrameId() {
        FRAMEID_MAIL | 7 | FRAMEID_QUIZ_SET_QUESTION => {
            return TRUE as u32;
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_Phrase() -> u16 {
    'l2: {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            ClearUnusedField();
            (*sEasyChatScreen).inputState = INPUTSTATE_KEYBOARD;
            (*sEasyChatScreen).keyboardColumn = 0;
            (*sEasyChatScreen).keyboardRow = 0;
            (*sEasyChatScreen).keyboardScrollOffset = 0;
            return ECFUNC_OPEN_KEYBOARD;
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            return StartConfirmExitPrompt();
        } else if gMain.newKeys as i32 & START_BUTTON != 0 {
            return TryConfirmWords();
        } else if gMain.newKeys as i32 & DPAD_UP != 0 {
            (*sEasyChatScreen).mainCursorRow -= 1;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
            (*sEasyChatScreen).mainCursorColumn -= 1;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            (*sEasyChatScreen).mainCursorRow += 1;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
            (*sEasyChatScreen).mainCursorColumn += 1;
            break 'l2;
        }
        return ECFUNC_NONE;
    }
    if (*sEasyChatScreen).mainCursorRow < 0 {
        (*sEasyChatScreen).mainCursorRow =
            sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numRows as i8;
    }
    if (*sEasyChatScreen).mainCursorRow as i32
        > sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numRows as i32
    {
        (*sEasyChatScreen).mainCursorRow = 0;
    }
    if (*sEasyChatScreen).mainCursorRow as i32
        == sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numRows as i32
    {
        if (*sEasyChatScreen).mainCursorColumn > 2 {
            (*sEasyChatScreen).mainCursorColumn = 2;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_MAIN_SCREEN_BUTTONS;
        return ECFUNC_UPDATE_MAIN_CURSOR_ON_BUTTONS;
    }
    if (*sEasyChatScreen).mainCursorColumn < 0 {
        (*sEasyChatScreen).mainCursorColumn =
            sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numColumns as i8 - 1;
    }
    if (*sEasyChatScreen).mainCursorColumn as i32
        >= sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numColumns as i32
    {
        (*sEasyChatScreen).mainCursorColumn = 0;
    }
    if IsCurrentFrame2x5() != 0
        && (*sEasyChatScreen).mainCursorColumn == 1
        && (*sEasyChatScreen).mainCursorRow == 4
    {
        (*sEasyChatScreen).mainCursorColumn = 0;
    }
    return ECFUNC_UPDATE_MAIN_CURSOR;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_MainScreenButtons() -> u16 {
    'l2: {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            match (*sEasyChatScreen).mainCursorColumn {
                0 => {
                    return DoDeleteAllButton() as u16;
                }
                1 => {
                    return StartConfirmExitPrompt();
                }
                2 => {
                    return TryConfirmWords();
                }
                3 => {
                    return DoQuizButton() as u16;
                }
                _ => {}
            }
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            return StartConfirmExitPrompt();
        } else if gMain.newKeys as i32 & START_BUTTON != 0 {
            return TryConfirmWords();
        } else if gMain.newKeys as i32 & DPAD_UP != 0 {
            (*sEasyChatScreen).mainCursorRow -= 1;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 {
            (*sEasyChatScreen).mainCursorColumn -= 1;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            (*sEasyChatScreen).mainCursorRow = 0;
            break 'l2;
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
            (*sEasyChatScreen).mainCursorColumn += 1;
            break 'l2;
        }
        return ECFUNC_NONE;
    }
    if (*sEasyChatScreen).mainCursorRow as i32
        == sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numRows as i32
    {
        let mut numFooterColumns: i32 = if FooterHasFourOptions() != 0 { 4 } else { 3 };
        if (*sEasyChatScreen).mainCursorColumn < 0 {
            (*sEasyChatScreen).mainCursorColumn = numFooterColumns as i8 - 1;
        }
        if (*sEasyChatScreen).mainCursorColumn as i32 >= numFooterColumns {
            (*sEasyChatScreen).mainCursorColumn = 0;
        }
        return ECFUNC_UPDATE_MAIN_CURSOR_ON_BUTTONS;
    }
    if (*sEasyChatScreen).mainCursorColumn as i32
        >= sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numColumns as i32
    {
        (*sEasyChatScreen).mainCursorColumn =
            sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].numColumns as i8 - 1;
    }
    if IsCurrentFrame2x5() != 0
        && (*sEasyChatScreen).mainCursorColumn == 1
        && (*sEasyChatScreen).mainCursorRow == 4
    {
        (*sEasyChatScreen).mainCursorColumn = 0;
    }
    (*sEasyChatScreen).inputState = INPUTSTATE_PHRASE;
    return ECFUNC_UPDATE_MAIN_CURSOR;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_Keyboard() -> u16 {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return ExitKeyboardToMainScreen() as u16;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if (*sEasyChatScreen).keyboardColumn != -1 {
            return SelectKeyboardGroup() as u16;
        }
        match (*sEasyChatScreen).keyboardRow {
            0 => {
                return StartSwitchKeyboardMode() as u16;
            }
            1 => {
                return DeleteSelectedWord() as u16;
            }
            2 => {
                return ExitKeyboardToMainScreen() as u16;
            }
            _ => {}
        }
    }
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        return StartSwitchKeyboardMode() as u16;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return MoveKeyboardCursor(INPUT_UP as i32);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return MoveKeyboardCursor(INPUT_DOWN as i32);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        return MoveKeyboardCursor(INPUT_LEFT as i32);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        return MoveKeyboardCursor(INPUT_RIGHT as i32);
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_WordSelect() -> u16 {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*sEasyChatScreen).inputState = INPUTSTATE_KEYBOARD;
        return ECFUNC_RETURN_TO_KEYBOARD;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return SelectNewWord() as u16;
    }
    if gMain.newKeys as i32 & START_BUTTON != 0 {
        return MoveWordSelectCursor(INPUT_START);
    }
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        return MoveWordSelectCursor(INPUT_SELECT);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        return MoveWordSelectCursor(INPUT_UP);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        return MoveWordSelectCursor(INPUT_DOWN);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        return MoveWordSelectCursor(INPUT_LEFT);
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        return MoveWordSelectCursor(INPUT_RIGHT);
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ExitPrompt() -> u16 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        MENU_B_PRESSED | 1 => {
            (*sEasyChatScreen).inputState = GetEasyChatBackupState();
            return ECFUNC_CLOSE_PROMPT;
        }
        0 => {
            gSpecialVar_Result = 0;
            if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_QUESTION
                || (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_ANSWER
            {
                SaveCurrentPhrase();
            }
            return ECFUNC_EXIT;
        }
        _ => {
            return ECFUNC_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ConfirmWordsYesNo() -> u16 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        MENU_B_PRESSED | 1 => {
            (*sEasyChatScreen).inputState = GetEasyChatBackupState();
            return ECFUNC_CLOSE_PROMPT;
        }
        0 => {
            SetSpecialEasyChatResult();
            gSpecialVar_Result = GetEasyChatCompleted() as u16;
            SaveCurrentPhrase();
            return ECFUNC_EXIT;
        }
        _ => {
            return ECFUNC_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_DeleteAllYesNo() -> u16 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        MENU_B_PRESSED | 1 => {
            (*sEasyChatScreen).inputState = INPUTSTATE_MAIN_SCREEN_BUTTONS;
            return ECFUNC_CLOSE_PROMPT;
        }
        0 => {
            ResetCurrentPhrase();
            (*sEasyChatScreen).inputState = INPUTSTATE_MAIN_SCREEN_BUTTONS;
            return ECFUNC_CLOSE_PROMPT_AFTER_DELETE;
        }
        _ => {
            return ECFUNC_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_QuizQuestion() -> u16 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return ECFUNC_QUIZ_ANSWER;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        return StartConfirmExitPrompt();
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_WaitForMsg() -> u16 {
    if gMain.newKeys as i32 & 3 != 0 {
        (*sEasyChatScreen).inputState = GetEasyChatBackupState();
        return ECFUNC_CLOSE_PROMPT;
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_StartConfirmLyrics() -> u16 {
    (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_LYRICS_YES_NO;
    return ECFUNC_PROMPT_CONFIRM;
}
pub(crate) unsafe extern "C" fn HandleEasyChatInput_ConfirmLyricsYesNo() -> u16 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        MENU_B_PRESSED | 1 => {
            ResetCurrentPhraseToSaved();
            (*sEasyChatScreen).inputStateBackup = INPUTSTATE_PHRASE;
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_SONG_TOO_SHORT;
        }
        0 => {
            gSpecialVar_Result = GetEasyChatCompleted() as u16;
            SaveCurrentPhrase();
            return ECFUNC_EXIT;
        }
        _ => {
            return ECFUNC_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn StartConfirmExitPrompt() -> u16 {
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_APPRENTICE
        || (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_CONTEST_INTERVIEW
    {
        (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
        (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
        return ECFUNC_MSG_CANT_EXIT;
    } else {
        (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
        (*sEasyChatScreen).inputState = INPUTSTATE_EXIT_PROMPT;
        return ECFUNC_PROMPT_EXIT;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DoDeleteAllButton() -> i32 {
    (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
    if (*sEasyChatScreen).r#type != EASY_CHAT_TYPE_BARD_SONG {
        (*sEasyChatScreen).inputState = INPUTSTATE_DELETE_ALL_YES_NO;
        return ECFUNC_PROMPT_DELETE_ALL;
    } else {
        (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
        (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
        return ECFUNC_MSG_CANT_DELETE_LYRICS;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TryConfirmWords() -> u16 {
    (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_QUESTION {
        if IsQuizQuestionEmpty() != 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_CREATE_QUIZ;
        }
        if IsQuizAnswerEmpty() != 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_SELECT_ANSWER;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    } else if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_ANSWER {
        if IsQuizAnswerEmpty() != 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_SELECT_ANSWER;
        }
        if IsQuizQuestionEmpty() != 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_CREATE_QUIZ;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    } else if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_TRENDY_PHRASE
        || (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_GOOD_SAYING
    {
        if IsCurrentPhraseFull() == 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_COMBINE_TWO_WORDS;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    } else if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_APPRENTICE
        || (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_CONTEST_INTERVIEW
    {
        if IsCurrentPhraseEmpty() != 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_WAIT_FOR_MSG;
            return ECFUNC_MSG_CANT_EXIT;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    } else if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUESTIONNAIRE {
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    } else {
        if IsCurrentPhraseEmpty() == TRUE as u32 || GetEasyChatCompleted() == 0 {
            (*sEasyChatScreen).inputState = INPUTSTATE_EXIT_PROMPT;
            return ECFUNC_PROMPT_EXIT;
        }
        (*sEasyChatScreen).inputState = INPUTSTATE_CONFIRM_WORDS_YES_NO;
        return ECFUNC_PROMPT_CONFIRM;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DoQuizButton() -> i32 {
    (*sEasyChatScreen).inputStateBackup = (*sEasyChatScreen).inputState;
    match (*sEasyChatScreen).r#type {
        EASY_CHAT_TYPE_QUIZ_ANSWER => {
            return ECFUNC_QUIZ_QUESTION;
        }
        EASY_CHAT_TYPE_QUIZ_SET_QUESTION => {
            SaveCurrentPhrase();
            return ECFUNC_SET_QUIZ_ANSWER;
        }
        EASY_CHAT_TYPE_QUIZ_SET_ANSWER => {
            SaveCurrentPhrase();
            return ECFUNC_SET_QUIZ_QUESTION;
        }
        _ => {
            return ECFUNC_NONE as i32;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatBackupState() -> u8 {
    return (*sEasyChatScreen).inputStateBackup;
}
pub(crate) unsafe extern "C" fn SelectKeyboardGroup() -> i32 {
    let mut numWords: u16 = 0;
    if (*sEasyChatScreen).inAlphabetMode == 0 {
        let mut groupId: u8 = GetUnlockedEasyChatGroupId(GetSelectedGroupIndex() as u8);
        SetSelectedWordGroup(FALSE as u32, groupId as u16);
    } else {
        SetSelectedWordGroup(TRUE as u32, GetSelectedAlphabetGroupId() as u16);
    }
    numWords = GetNumWordsInSelectedGroup();
    if numWords == 0 {
        return ECFUNC_NONE as i32;
    }
    (*sEasyChatScreen).wordSelectLastRow = ((numWords as i32 - 1) / 2) as u8;
    (*sEasyChatScreen).wordSelectScrollOffset = 0;
    (*sEasyChatScreen).wordSelectColumn = 0;
    (*sEasyChatScreen).wordSelectRow = 0;
    (*sEasyChatScreen).inputState = INPUTSTATE_WORD_SELECT;
    return ECFUNC_OPEN_WORD_SELECT;
}
pub(crate) unsafe extern "C" fn ExitKeyboardToMainScreen() -> i32 {
    (*sEasyChatScreen).inputState = INPUTSTATE_PHRASE;
    return ECFUNC_CLOSE_KEYBOARD;
}
pub(crate) unsafe extern "C" fn StartSwitchKeyboardMode() -> i32 {
    (*sEasyChatScreen).keyboardColumn = 0;
    (*sEasyChatScreen).keyboardRow = 0;
    (*sEasyChatScreen).keyboardScrollOffset = 0;
    if (*sEasyChatScreen).inAlphabetMode == 0 {
        (*sEasyChatScreen).inAlphabetMode = TRUE;
    } else {
        (*sEasyChatScreen).inAlphabetMode = FALSE;
    }
    return ECFUNC_SWITCH_KEYBOARD_MODE;
}
pub(crate) unsafe extern "C" fn DeleteSelectedWord() -> i32 {
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_BARD_SONG {
        PlaySE(SE_FAILURE);
        return ECFUNC_NONE as i32;
    } else {
        SetSelectedWord(EC_EMPTY_WORD);
        return ECFUNC_REPRINT_PHRASE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SelectNewWord() -> i32 {
    let mut easyChatWord: u16 = GetWordFromSelectedGroup(GetSelectedWordIndex());
    if DummyWordCheck(easyChatWord as i32) != 0 {
        PlaySE(SE_FAILURE);
        return ECFUNC_NONE as i32;
    } else {
        SetSelectedWord(easyChatWord);
        if (*sEasyChatScreen).r#type != EASY_CHAT_TYPE_BARD_SONG {
            (*sEasyChatScreen).inputState = INPUTSTATE_PHRASE;
            return ECFUNC_CLOSE_WORD_SELECT;
        } else {
            (*sEasyChatScreen).inputState = INPUTSTATE_START_CONFIRM_LYRICS;
            return ECFUNC_PROMPT_CONFIRM_LYRICS;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SaveCurrentPhrase() {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        *(*sEasyChatScreen).savedPhrase.at(i) = (*sEasyChatScreen).currentPhrase[i];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ResetCurrentPhrase() {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        (*sEasyChatScreen).currentPhrase[i] = EC_EMPTY_WORD;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ResetCurrentPhraseToSaved() {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        (*sEasyChatScreen).currentPhrase[i] = *(*sEasyChatScreen).savedPhrase.at(i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWord(easyChatWord: u16) {
    let mut index: u16 = GetWordIndexToReplace();
    (*sEasyChatScreen).currentPhrase[index] = easyChatWord;
}
pub(crate) unsafe extern "C" fn DidPhraseChange() -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as u16 {
        if (*sEasyChatScreen).currentPhrase[i] != *(*sEasyChatScreen).savedPhrase.at(i) {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetEasyChatCompleted() -> u32 {
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_QUESTION
        || (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_ANSWER
    {
        if IsQuizQuestionEmpty() != 0 {
            return FALSE as u32;
        }
        if IsQuizAnswerEmpty() != 0 {
            return FALSE as u32;
        }
        return TRUE as u32;
    } else {
        return DidPhraseChange() as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor(input: i32) -> u16 {
    if (*sEasyChatScreen).keyboardColumn != -1 {
        if (*sEasyChatScreen).inAlphabetMode == 0 {
            return MoveKeyboardCursor_GroupNames(input as u32) as u16;
        } else {
            return MoveKeyboardCursor_Alphabet(input as u32) as u16;
        }
    } else {
        return MoveKeyboardCursor_ButtonWindow(input as u32) as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_GroupNames(input: u32) -> i32 {
    match input {
        INPUT_UP => {
            if (*sEasyChatScreen).keyboardRow as i32
                != -((*sEasyChatScreen).keyboardScrollOffset as i32)
            {
                if (*sEasyChatScreen).keyboardRow != 0 {
                    (*sEasyChatScreen).keyboardRow -= 1;
                    return ECFUNC_UPDATE_KEYBOARD_CURSOR;
                } else {
                    (*sEasyChatScreen).keyboardScrollOffset -= 1;
                    return ECFUNC_GROUP_NAMES_SCROLL_UP;
                }
            }
        }
        INPUT_DOWN => {
            if ((*sEasyChatScreen).keyboardRow as i32
                + (*sEasyChatScreen).keyboardScrollOffset as i32)
                < (*sEasyChatScreen).keyboardLastRow as i32 - 1
            {
                let mut funcId: i32 = 0;
                if (*sEasyChatScreen).keyboardRow < 3 {
                    (*sEasyChatScreen).keyboardRow += 1;
                    funcId = ECFUNC_UPDATE_KEYBOARD_CURSOR;
                } else {
                    (*sEasyChatScreen).keyboardScrollOffset += 1;
                    funcId = ECFUNC_GROUP_NAMES_SCROLL_DOWN;
                }
                ReduceToValidKeyboardColumn();
                return funcId;
            }
        }
        INPUT_LEFT => {
            if (*sEasyChatScreen).keyboardColumn != 0 {
                (*sEasyChatScreen).keyboardColumn -= 1;
            } else {
                SetKeyboardCursorInButtonWindow();
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_RIGHT => {
            if (*sEasyChatScreen).keyboardColumn < 1 {
                (*sEasyChatScreen).keyboardColumn += 1;
                if IsSelectedKeyboardIndexInvalid() != 0 {
                    SetKeyboardCursorInButtonWindow();
                }
            } else {
                SetKeyboardCursorInButtonWindow();
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        _ => {}
    }
    return ECFUNC_NONE as i32;
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_Alphabet(input: u32) -> i32 {
    match input {
        INPUT_UP => {
            if (*sEasyChatScreen).keyboardRow > 0 {
                (*sEasyChatScreen).keyboardRow -= 1;
            } else {
                (*sEasyChatScreen).keyboardRow = 3;
            }
            ReduceToValidKeyboardColumn();
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_DOWN => {
            if (*sEasyChatScreen).keyboardRow < 3 {
                (*sEasyChatScreen).keyboardRow += 1;
            } else {
                (*sEasyChatScreen).keyboardRow = 0;
            }
            ReduceToValidKeyboardColumn();
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_RIGHT => {
            (*sEasyChatScreen).keyboardColumn += 1;
            if IsSelectedKeyboardIndexInvalid() != 0 {
                SetKeyboardCursorInButtonWindow();
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_LEFT => {
            (*sEasyChatScreen).keyboardColumn -= 1;
            if (*sEasyChatScreen).keyboardColumn < 0 {
                SetKeyboardCursorInButtonWindow();
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        _ => {}
    }
    return ECFUNC_NONE as i32;
}
pub(crate) unsafe extern "C" fn MoveKeyboardCursor_ButtonWindow(input: u32) -> i32 {
    match input {
        INPUT_UP => {
            if (*sEasyChatScreen).keyboardRow != 0 {
                (*sEasyChatScreen).keyboardRow -= 1;
            } else {
                (*sEasyChatScreen).keyboardRow = 2;
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_DOWN => {
            if (*sEasyChatScreen).keyboardRow < 2 {
                (*sEasyChatScreen).keyboardRow += 1;
            } else {
                (*sEasyChatScreen).keyboardRow = 0;
            }
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_LEFT => {
            (*sEasyChatScreen).keyboardRow += 1;
            SetKeyboardCursorToLastColumn();
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        INPUT_RIGHT => {
            (*sEasyChatScreen).keyboardColumn = 0;
            (*sEasyChatScreen).keyboardRow += 1;
            return ECFUNC_UPDATE_KEYBOARD_CURSOR;
        }
        _ => {}
    }
    return ECFUNC_NONE as i32;
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorInButtonWindow() {
    (*sEasyChatScreen).keyboardColumn = -1;
    if (*sEasyChatScreen).keyboardRow != 0 {
        (*sEasyChatScreen).keyboardRow -= 1;
    }
}
pub(crate) unsafe extern "C" fn SetKeyboardCursorToLastColumn() {
    if (*sEasyChatScreen).inAlphabetMode == 0 {
        (*sEasyChatScreen).keyboardColumn = 1;
        ReduceToValidKeyboardColumn();
    } else {
        (*sEasyChatScreen).keyboardColumn =
            GetLastAlphabetColumn((*sEasyChatScreen).keyboardRow as u8) as i8;
    }
}
pub(crate) unsafe extern "C" fn MoveWordSelectCursor(input: u32) -> u16 {
    let mut funcId: u16 = 0;
    match input {
        INPUT_UP => {
            if (*sEasyChatScreen).wordSelectRow as i32
                + (*sEasyChatScreen).wordSelectScrollOffset as i32
                > 0
            {
                if (*sEasyChatScreen).wordSelectRow > 0 {
                    (*sEasyChatScreen).wordSelectRow -= 1;
                    funcId = ECFUNC_UPDATE_WORD_SELECT_CURSOR;
                } else {
                    (*sEasyChatScreen).wordSelectScrollOffset -= 1;
                    funcId = ECFUNC_WORD_SELECT_SCROLL_UP;
                }
                ReduceToValidWordSelectColumn();
                return funcId;
            }
        }
        INPUT_DOWN => {
            if ((*sEasyChatScreen).wordSelectRow as i32
                + (*sEasyChatScreen).wordSelectScrollOffset as i32)
                < (*sEasyChatScreen).wordSelectLastRow as i32
            {
                if (*sEasyChatScreen).wordSelectRow < 3 {
                    (*sEasyChatScreen).wordSelectRow += 1;
                    funcId = ECFUNC_UPDATE_WORD_SELECT_CURSOR;
                } else {
                    (*sEasyChatScreen).wordSelectScrollOffset += 1;
                    funcId = ECFUNC_WORD_SELECT_SCROLL_DOWN;
                }
                ReduceToValidWordSelectColumn();
                return funcId;
            }
        }
        INPUT_LEFT => {
            if (*sEasyChatScreen).wordSelectColumn > 0 {
                (*sEasyChatScreen).wordSelectColumn -= 1;
            } else {
                (*sEasyChatScreen).wordSelectColumn = 1;
            }
            ReduceToValidWordSelectColumn();
            return ECFUNC_UPDATE_WORD_SELECT_CURSOR;
        }
        INPUT_RIGHT => {
            if (*sEasyChatScreen).wordSelectColumn < 1 {
                (*sEasyChatScreen).wordSelectColumn += 1;
                if IsSelectedWordIndexInvalid() != 0 {
                    (*sEasyChatScreen).wordSelectColumn = 0;
                }
            } else {
                (*sEasyChatScreen).wordSelectColumn = 0;
            }
            return ECFUNC_UPDATE_WORD_SELECT_CURSOR;
        }
        INPUT_START => {
            if (*sEasyChatScreen).wordSelectScrollOffset != 0 {
                if (*sEasyChatScreen).wordSelectScrollOffset >= NUM_WORD_SELECT_ROWS {
                    (*sEasyChatScreen).wordSelectScrollOffset -= NUM_WORD_SELECT_ROWS;
                } else {
                    (*sEasyChatScreen).wordSelectScrollOffset = 0;
                }
                return ECFUNC_WORD_SELECT_PAGE_UP;
            }
        }
        INPUT_SELECT => {
            if (*sEasyChatScreen).wordSelectScrollOffset as i32
                <= (*sEasyChatScreen).wordSelectLastRow as i32 - NUM_WORD_SELECT_ROWS as i32
            {
                (*sEasyChatScreen).wordSelectScrollOffset += NUM_WORD_SELECT_ROWS;
                if (*sEasyChatScreen).wordSelectScrollOffset as i32
                    > (*sEasyChatScreen).wordSelectLastRow as i32 - NUM_WORD_SELECT_ROWS as i32 + 1
                {
                    (*sEasyChatScreen).wordSelectScrollOffset =
                        (*sEasyChatScreen).wordSelectLastRow - NUM_WORD_SELECT_ROWS + 1;
                }
                ReduceToValidWordSelectColumn();
                return ECFUNC_WORD_SELECT_PAGE_DOWN;
            }
        }
        _ => {}
    }
    return ECFUNC_NONE;
}
pub(crate) unsafe extern "C" fn GetWordIndexToReplace() -> u16 {
    return (*sEasyChatScreen).mainCursorRow as u16 * (*sEasyChatScreen).numColumns as u16
        + (*sEasyChatScreen).mainCursorColumn as u16;
}
pub(crate) unsafe extern "C" fn GetSelectedGroupIndex() -> u16 {
    return NUM_GROUP_NAME_COLUMNS
        * ((*sEasyChatScreen).keyboardRow as u16 + (*sEasyChatScreen).keyboardScrollOffset as u16)
        + (*sEasyChatScreen).keyboardColumn as u16;
}
pub(crate) unsafe extern "C" fn GetSelectedAlphabetGroupId() -> i32 {
    let mut column: i32 = if ((*sEasyChatScreen).keyboardColumn as u8) < NUM_ALPHABET_COLUMNS {
        (*sEasyChatScreen).keyboardColumn as i32
    } else {
        0
    };
    let mut row: i32 = if ((*sEasyChatScreen).keyboardRow as u8) < NUM_ALPHABET_ROWS {
        (*sEasyChatScreen).keyboardRow as i32
    } else {
        0
    };
    return sAlphabetGroupIdMap[row][column] as i32;
}
pub(crate) unsafe extern "C" fn GetSelectedWordIndex() -> u16 {
    return NUM_WORD_SELECT_COLUMNS
        * ((*sEasyChatScreen).wordSelectRow as u16
            + (*sEasyChatScreen).wordSelectScrollOffset as u16)
        + (*sEasyChatScreen).wordSelectColumn as u16;
}
pub(crate) unsafe extern "C" fn GetLastAlphabetColumn(row: u8) -> u8 {
    match row {
        1 => {
            return 5;
        }
        _ => {
            return 6;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ReduceToValidKeyboardColumn() {
    while IsSelectedKeyboardIndexInvalid() != 0 {
        if (*sEasyChatScreen).keyboardColumn != 0 {
            (*sEasyChatScreen).keyboardColumn -= 1;
        } else {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn ReduceToValidWordSelectColumn() {
    while IsSelectedWordIndexInvalid() != 0 {
        if (*sEasyChatScreen).wordSelectColumn != 0 {
            (*sEasyChatScreen).wordSelectColumn -= 1;
        } else {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn IsSelectedKeyboardIndexInvalid() -> u8 {
    if (*sEasyChatScreen).inAlphabetMode == 0 {
        return (if GetSelectedGroupIndex() >= GetNumUnlockedEasyChatGroups() as u16 {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u8;
    } else {
        return (if (*sEasyChatScreen).keyboardColumn as i32
            > GetLastAlphabetColumn((*sEasyChatScreen).keyboardRow as u8) as i32
        {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsSelectedWordIndexInvalid() -> u8 {
    return (if GetSelectedWordIndex() >= GetNumWordsInSelectedGroup() {
        TRUE as i32
    } else {
        FALSE as i32
    }) as u8;
}
pub(crate) unsafe extern "C" fn FooterHasFourOptions() -> i32 {
    return sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].fourFooterOptions() as i32;
}
pub(crate) unsafe extern "C" fn GetEasyChatScreenType() -> u8 {
    return (*sEasyChatScreen).r#type;
}
pub(crate) unsafe extern "C" fn GetEasyChatScreenFrameId() -> u8 {
    return sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].frameId();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTitleText() -> *mut u8 {
    return (*sEasyChatScreen).titleText;
}
pub(crate) unsafe extern "C" fn GetCurrentPhrase() -> *mut u16 {
    return (*sEasyChatScreen).currentPhrase.as_mut_ptr();
}
pub(crate) unsafe extern "C" fn GetNumRows() -> u8 {
    return (*sEasyChatScreen).numRows;
}
pub(crate) unsafe extern "C" fn GetNumColumns() -> u8 {
    return (*sEasyChatScreen).numColumns;
}
pub(crate) unsafe extern "C" fn GetMainCursorColumn() -> u8 {
    return (*sEasyChatScreen).mainCursorColumn as u8;
}
pub(crate) unsafe extern "C" fn GetMainCursorRow() -> u8 {
    return (*sEasyChatScreen).mainCursorRow as u8;
}
pub(crate) unsafe extern "C" fn GetEasyChatInstructionsText(
    str1: *mut *mut u8,
    str2: *mut *mut u8,
) {
    *str1 = sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].instructionsText1;
    *str2 = sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].instructionsText2;
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmText(str1: *mut *mut u8, str2: *mut *mut u8) {
    *str1 = sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].confirmText1;
    *str2 = sEasyChatScreenTemplates[(*sEasyChatScreen).templateId].confirmText2;
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmExitText(str1: *mut *mut u8, str2: *mut *mut u8) {
    match (*sEasyChatScreen).r#type {
        EASY_CHAT_TYPE_MAIL => {
            *str1 = gText_StopGivingPkmnMail.as_ptr().cast_mut();
            *str2 = null_mut();
        }
        EASY_CHAT_TYPE_QUIZ_ANSWER | EASY_CHAT_TYPE_QUIZ_QUESTION => {
            *str1 = gText_LikeToQuitQuiz.as_ptr().cast_mut();
            *str2 = gText_ChallengeQuestionMark.as_ptr().cast_mut();
        }
        _ => {
            *str1 = gText_QuitEditing.as_ptr().cast_mut();
            *str2 = null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatConfirmDeletionText(
    str1: *mut *mut u8,
    str2: *mut *mut u8,
) {
    *str1 = gText_AllTextBeingEditedWill.as_ptr().cast_mut();
    *str2 = gText_BeDeletedThatOkay.as_ptr().cast_mut();
}
pub(crate) unsafe extern "C" fn GetKeyboardCursorColAndRow(column: *mut i8, row: *mut i8) {
    *column = (*sEasyChatScreen).keyboardColumn;
    *row = (*sEasyChatScreen).keyboardRow;
}
pub(crate) unsafe extern "C" fn GetInAlphabetMode() -> u8 {
    return (*sEasyChatScreen).inAlphabetMode;
}
pub(crate) unsafe extern "C" fn GetKeyboardScrollOffset() -> u8 {
    return (*sEasyChatScreen).keyboardScrollOffset;
}
pub(crate) unsafe extern "C" fn GetWordSelectColAndRow(column: *mut i8, row: *mut i8) {
    *column = (*sEasyChatScreen).wordSelectColumn;
    *row = (*sEasyChatScreen).wordSelectRow;
}
pub(crate) unsafe extern "C" fn GetWordSelectScrollOffset() -> u8 {
    return (*sEasyChatScreen).wordSelectScrollOffset;
}
pub(crate) unsafe extern "C" fn GetWordSelectLastRow() -> u8 {
    return (*sEasyChatScreen).wordSelectLastRow;
}
pub(crate) unsafe extern "C" fn UnusedDummy() -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn CanScrollUp() -> u32 {
    match (*sEasyChatScreen).inputState {
        INPUTSTATE_KEYBOARD => {
            if (*sEasyChatScreen).inAlphabetMode == 0
                && (*sEasyChatScreen).keyboardScrollOffset != 0
            {
                return TRUE as u32;
            }
        }
        INPUTSTATE_WORD_SELECT => {
            if (*sEasyChatScreen).wordSelectScrollOffset != 0 {
                return TRUE as u32;
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CanScrollDown() -> u32 {
    match (*sEasyChatScreen).inputState {
        INPUTSTATE_KEYBOARD => {
            if (*sEasyChatScreen).inAlphabetMode == 0
                && (*sEasyChatScreen).keyboardScrollOffset as i32 + NUM_GROUP_NAME_ROWS
                    <= (*sEasyChatScreen).keyboardLastRow as i32 - 1
            {
                return TRUE as u32;
            }
        }
        INPUTSTATE_WORD_SELECT => {
            if (*sEasyChatScreen).wordSelectScrollOffset as i32 + NUM_WORD_SELECT_ROWS as i32
                <= (*sEasyChatScreen).wordSelectLastRow as i32
            {
                return TRUE as u32;
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn FooterHasFourOptions_() -> i32 {
    return FooterHasFourOptions();
}
pub(crate) unsafe extern "C" fn IsPhraseDifferentThanPlayerInput(
    phrase: *mut u16,
    phraseLength: u8,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < phraseLength {
        if *phrase.at(i) != (*sEasyChatScreen).currentPhrase[i] {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetDisplayedPersonType() -> u8 {
    return (*sEasyChatScreen).displayedPersonType;
}
pub(crate) unsafe extern "C" fn GetEachChatScreenTemplateId(r#type: u8) -> u8 {
    let mut i: u32 = 0;
    i = 0;
    while i < 21 {
        if sEasyChatScreenTemplates[i].r#type == r#type {
            return i as u8;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn IsCurrentPhraseEmpty() -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        if (*sEasyChatScreen).currentPhrase[i] != EC_EMPTY_WORD {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn IsCurrentPhraseFull() -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        if (*sEasyChatScreen).currentPhrase[i] == EC_EMPTY_WORD {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn IsQuizQuestionEmpty() -> i32 {
    let mut i: i32 = 0;
    let mut saveBlock1: *mut SaveBlock1 = null_mut();
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_QUESTION {
        return IsCurrentPhraseEmpty() as i32;
    }
    saveBlock1 = gSaveBlock1Ptr;
    i = 0;
    while i < QUIZ_QUESTION_LEN as i32 {
        if (*saveBlock1).lilycoveLady.quiz.question[i] != EC_EMPTY_WORD {
            return FALSE as i32;
        }
        i += 1;
    }
    return TRUE as i32;
}
pub(crate) unsafe extern "C" fn IsQuizAnswerEmpty() -> i32 {
    let mut quiz: *mut LilycoveLadyQuiz = null_mut();
    if (*sEasyChatScreen).r#type == EASY_CHAT_TYPE_QUIZ_SET_ANSWER {
        return IsCurrentPhraseEmpty() as i32;
    }
    quiz = &raw mut (*gSaveBlock1Ptr).lilycoveLady.quiz;
    return if (*quiz).correctAnswer == EC_EMPTY_WORD {
        TRUE as i32
    } else {
        FALSE as i32
    };
}
pub(crate) unsafe extern "C" fn GetQuizTitle(dst: *mut u8) {
    let mut name: CArray<u8, 32> = zeroed();
    let mut saveBlock1: *mut SaveBlock1 = gSaveBlock1Ptr;
    DynamicPlaceholderTextUtil_Reset();
    if StringLength((*saveBlock1).lilycoveLady.quiz.playerName.as_mut_ptr()) != 0 {
        TVShowConvertInternationalString(
            name.as_mut_ptr(),
            (*saveBlock1).lilycoveLady.quiz.playerName.as_mut_ptr(),
            (*saveBlock1).lilycoveLady.quiz.language as i32,
        );
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, name.as_mut_ptr());
    } else {
        DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, gText_Lady.as_ptr().cast_mut());
    }
    DynamicPlaceholderTextUtil_ExpandPlaceholders(dst, gText_F700sQuiz.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn BufferCurrentPhraseToStringVar2() {
    let mut i: i32 = 0;
    let mut phrase: *mut u16 = null_mut();
    let mut str: *mut u8 = null_mut();
    phrase = (*sEasyChatScreen).currentPhrase.as_mut_ptr();
    str = gStringVar2.as_mut_ptr();
    i = 0;
    while i < (*sEasyChatScreen).maxWords as i32 {
        str = CopyEasyChatWordPadded(str, *phrase, 0);
        *str = 0;
        str = str.at(1);
        phrase = phrase.at(1);
        i += 1;
    }
    str = str.at(-1);
    *str = EOS;
}
pub(crate) unsafe extern "C" fn SetSpecialEasyChatResult() {
    match (*sEasyChatScreen).r#type {
        0 => {
            FlagSet(FLAG_SYS_CHAT_USED);
        }
        EASY_CHAT_TYPE_QUESTIONNAIRE => {
            if DidPlayerInputMysteryGiftPhrase() != 0 {
                gSpecialVar_0x8004 = 2;
            } else {
                gSpecialVar_0x8004 = 0;
            }
        }
        EASY_CHAT_TYPE_TRENDY_PHRASE => {
            BufferCurrentPhraseToStringVar2();
            gSpecialVar_0x8004 =
                TrySetTrendyPhrase((*sEasyChatScreen).currentPhrase.as_mut_ptr()) as u16;
        }
        EASY_CHAT_TYPE_GOOD_SAYING => {
            gSpecialVar_0x8004 = DidPlayerInputABerryMasterWifePhrase();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DidPlayerInputMysteryGiftPhrase() -> i32 {
    return (IsPhraseDifferentThanPlayerInput(sMysteryGiftPhrase.as_ptr().cast_mut(), 4) == 0)
        as i32;
}
pub(crate) unsafe extern "C" fn DidPlayerInputABerryMasterWifePhrase() -> u16 {
    let mut i: i32 = 0;
    i = 0;
    while i < 5 {
        if IsPhraseDifferentThanPlayerInput(sBerryMasterWifePhrases[i].as_ptr().cast_mut(), 2) == 0
        {
            return i as u16 + 1;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn ClearUnusedField() {
    (*sEasyChatScreen).unused = 0;
}
pub(crate) unsafe extern "C" fn DummyWordCheck(easyChatWord: i32) -> u32 {
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenControl() -> u8 {
    if InitEasyChatScreenControl_() == 0 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn LoadEasyChatScreen() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sEasyChatBgTemplates.as_ptr().cast_mut(), 4);
            SetBgTilemapBuffer(
                3,
                (*sScreenControl).bg3TilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(
                1,
                (*sScreenControl).bg1TilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            InitWindows(sEasyChatWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            LoadEasyChatPalettes();
            InitEasyChatBgs();
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuFastSet(
                    &raw mut tmp as *mut c_void,
                    OAM as i32 as usize as *mut c_void,
                    0x1000100,
                );
            }
        }
        1 => {
            DecompressAndLoadBgGfxUsingHeap(
                3,
                gEasyChatWindow_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            CopyToBgTilemapBuffer(
                3,
                gEasyChatWindow_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            AdjustBgTilemapForFooter();
            BufferFrameTilemap((*sScreenControl).bg1TilemapBuffer.as_mut_ptr());
            AddPhraseWindow();
            AddMainScreenButtonWindow();
            CopyBgTilemapBufferToVram(3);
        }
        2 => {
            DecompressAndLoadBgGfxUsingHeap(
                1,
                sTextInputFrame_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
        }
        3 => {
            PrintTitle();
            PrintInitialInstructions();
            PrintCurrentPhrase();
            DrawLowerWindow();
        }
        4 => {
            LoadEasyChatGfx();
            if GetEasyChatScreenType() != EASY_CHAT_TYPE_QUIZ_QUESTION {
                CreateMainCursorSprite();
            }
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return TRUE;
            } else {
                SetWindowDimensions(0, 0, 0, 0);
                SetGpuReg(REG_OFFSET_WININ, 63);
                SetGpuReg(REG_OFFSET_WINOUT, 59);
                ShowBg(3);
                ShowBg(1);
                ShowBg(2);
                ShowBg(0);
                CreateScrollIndicatorSprites();
                CreateStartSelectButtonSprites();
                TryAddInterviewObjectEvents();
            }
        }
        _ => {
            return FALSE;
        }
    }
    (*sScreenControl).funcState += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenControl() {
    if !sScreenControl.is_null() {
        Free(sScreenControl as *mut c_void);
        sScreenControl = null_mut();
    }
}
pub(crate) unsafe extern "C" fn StartEasyChatFunction(funcId: u16) {
    (*sScreenControl).currentFuncId = funcId;
    (*sScreenControl).funcState = 0;
    RunEasyChatFunction();
}
pub(crate) unsafe extern "C" fn RunEasyChatFunction() -> u8 {
    match (*sScreenControl).currentFuncId {
        ECFUNC_NONE => {
            return FALSE;
        }
        1 => {
            return ReprintPhrase();
        }
        ECFUNC_UPDATE_MAIN_CURSOR => {
            return UpdateMainCursor();
        }
        ECFUNC_UPDATE_MAIN_CURSOR_ON_BUTTONS => {
            return UpdateMainCursorOnButtons();
        }
        4 => {
            return ShowConfirmDeleteAllPrompt();
        }
        ECFUNC_PROMPT_EXIT => {
            return ShowConfirmExitPrompt();
        }
        ECFUNC_PROMPT_CONFIRM => {
            return ShowConfirmPrompt();
        }
        ECFUNC_CLOSE_PROMPT => {
            return ClosePrompt();
        }
        ECFUNC_CLOSE_PROMPT_AFTER_DELETE => {
            return ClosePromptAfterDeleteAll();
        }
        ECFUNC_OPEN_KEYBOARD => {
            return OpenKeyboard();
        }
        10 => {
            return CloseKeyboard();
        }
        11 => {
            return OpenWordSelect();
        }
        12 => {
            return CloseWordSelect();
        }
        13 => {
            return ShowConfirmLyricsPrompt();
        }
        ECFUNC_RETURN_TO_KEYBOARD => {
            return ReturnToKeyboard();
        }
        15 => {
            return UpdateKeyboardCursor();
        }
        16 => {
            return GroupNamesScrollDown();
        }
        17 => {
            return GroupNamesScrollUp();
        }
        ECFUNC_UPDATE_WORD_SELECT_CURSOR => {
            return UpdateWordSelectCursor();
        }
        ECFUNC_WORD_SELECT_SCROLL_UP => {
            return WordSelectScrollUp();
        }
        ECFUNC_WORD_SELECT_SCROLL_DOWN => {
            return WordSelectScrollDown();
        }
        ECFUNC_WORD_SELECT_PAGE_UP => {
            return WordSelectPageScrollUp();
        }
        ECFUNC_WORD_SELECT_PAGE_DOWN => {
            return WordSelectPageScrollDown();
        }
        23 => {
            return SwitchKeyboardMode();
        }
        ECFUNC_EXIT => {
            return FALSE;
        }
        25 => {
            return FALSE;
        }
        ECFUNC_QUIZ_ANSWER => {
            return FALSE;
        }
        27 => {
            return FALSE;
        }
        28 => {
            return FALSE;
        }
        ECFUNC_MSG_CREATE_QUIZ => {
            return ShowCreateQuizMsg();
        }
        ECFUNC_MSG_SELECT_ANSWER => {
            return ShowSelectAnswerMsg();
        }
        ECFUNC_MSG_SONG_TOO_SHORT => {
            return ShowSongTooShortMsg();
        }
        32 => {
            return ShowCantDeleteLyricsMsg();
        }
        ECFUNC_MSG_COMBINE_TWO_WORDS => {
            return ShowCombineTwoWordsMsg();
        }
        ECFUNC_MSG_CANT_EXIT => {
            return ShowCantExitMsg();
        }
        _ => {
            return FALSE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ReprintPhrase() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintCurrentPhrase();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateMainCursor() -> u8 {
    let mut i: u8 = 0;
    let mut currentPhrase: *mut u16 = null_mut();
    let mut ecWord: *mut u16 = null_mut();
    let mut frameId: u8 = 0;
    let mut cursorColumn: u8 = 0;
    let mut cursorRow: u8 = 0;
    let mut numColumns: u8 = 0;
    let mut x: i16 = 0;
    let mut stringWidth: i32 = 0;
    let mut trueStringWidth: i32 = 0;
    let mut y: u8 = 0;
    let mut str: CArray<u8, 64> = zeroed();
    currentPhrase = GetCurrentPhrase();
    frameId = GetEasyChatScreenFrameId();
    cursorColumn = GetMainCursorColumn();
    cursorRow = GetMainCursorRow();
    numColumns = GetNumColumns();
    ecWord = currentPhrase.at(cursorRow as i32 * numColumns as i32);
    x = 8 * sPhraseFrameDimensions[frameId].left() as i16 + 13;
    i = 0;
    while i < cursorColumn {
        if *ecWord == EC_EMPTY_WORD {
            stringWidth = 72;
        } else {
            CopyEasyChatWord(str.as_mut_ptr(), *ecWord);
            stringWidth = GetStringWidth(FONT_NORMAL, str.as_mut_ptr(), 0);
        }
        trueStringWidth = stringWidth + 17;
        x += trueStringWidth as i16;
        ecWord = ecWord.at(1);
        i += 1;
    }
    y = 8 * (sPhraseFrameDimensions[frameId].top() + cursorRow * 2);
    SetMainCursorPos(x as u8, y + 8);
    return FALSE;
}
pub(crate) unsafe extern "C" fn UpdateMainCursorOnButtons() -> u8 {
    let mut xOffset: u8 = GetFooterOptionXOffset(GetMainCursorColumn() as i32) as u8;
    SetMainCursorPos(xOffset, 96);
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShowConfirmExitPrompt() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CONFIRM_EXIT);
            CreateEasyChatYesNoMenu(1);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowConfirmPrompt() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CONFIRM);
            CreateEasyChatYesNoMenu(0);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowConfirmDeleteAllPrompt() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CONFIRM_DELETE);
            CreateEasyChatYesNoMenu(1);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ClosePrompt() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StartMainCursorAnim();
            PrintEasyChatStdMessage(MSG_INSTRUCTIONS);
            PrintCurrentPhrase();
            ShowBg(0);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ClosePromptAfterDeleteAll() -> u8 {
    'l1: {
        let sw1: u16 = (*sScreenControl).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            StartMainCursorAnim();
            PrintEasyChatStdMessage(MSG_INSTRUCTIONS);
            PrintCurrentPhrase();
            (*sScreenControl).funcState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            return IsDma3ManagerBusyWithBgCopy();
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn OpenKeyboard() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            HideBg(0);
            SetWindowDimensions(0, 0, 0, 0);
            PrintKeyboardText();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_OPEN_KEYBOARD);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 && UpdateLowerWindowAnim() == 0 {
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateSideWindowSprites();
                (*sScreenControl).funcState += 1;
            }
        }
        4 => {
            if ShowSideWindow() == 0 {
                CreateRectangleCursorSprites();
                SetScrollIndicatorXPos(FALSE as u32);
                UpdateScrollIndicatorsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn CloseKeyboard() -> u8 {
    'l1: {
        let sw1: u16 = (*sScreenControl).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            DestroyRectangleCursorSprites();
            HideModeWindow();
            HideScrollIndicators();
            (*sScreenControl).funcState += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if DestroySideWindowSprites() == TRUE {
                break 'l1;
            }
            InitLowerWindowAnim(WINANIM_CLOSE_KEYBOARD);
            (*sScreenControl).funcState += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            if UpdateLowerWindowAnim() == 0 {
                (*sScreenControl).funcState += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                StartMainCursorAnim();
                ShowBg(0);
                (*sScreenControl).funcState += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SwitchKeyboardMode() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            DestroyRectangleCursorSprites();
            HideScrollIndicators();
            SetModeWindowToTransition();
            InitLowerWindowAnim(WINANIM_KEYBOARD_SWITCH_OUT);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if UpdateLowerWindowAnim() == 0 && IsModeWindowAnimActive() == 0 {
                PrintKeyboardText();
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_KEYBOARD_SWITCH_IN);
                UpdateModeWindowAnim();
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if UpdateLowerWindowAnim() == 0 && IsModeWindowAnimActive() == 0 {
                UpdateScrollIndicatorsVisibility();
                CreateRectangleCursorSprites();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        4 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateKeyboardCursor() -> u8 {
    UpdateRectangleCursorPos();
    return FALSE;
}
pub(crate) unsafe extern "C" fn GroupNamesScrollDown() -> u8 {
    'l1: {
        let sw1: u16 = (*sScreenControl).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            InitLowerWindowScroll(1, 4);
            (*sScreenControl).funcState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if UpdateLowerWindowScroll() == 0 {
                UpdateRectangleCursorPos();
                UpdateScrollIndicatorsVisibility();
                return FALSE;
            }
            break 'l1;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn GroupNamesScrollUp() -> u8 {
    'l1: {
        let sw1: u16 = (*sScreenControl).funcState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            InitLowerWindowScroll(-1, 4);
            (*sScreenControl).funcState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if UpdateLowerWindowScroll() == 0 {
                UpdateScrollIndicatorsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn OpenWordSelect() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            DestroyRectangleCursorSprites();
            HideModeWindow();
            HideScrollIndicators();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if DestroySideWindowSprites() == 0 {
                ClearWordSelectWindow();
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_OPEN_WORD_SELECT);
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if UpdateLowerWindowAnim() == 0 {
                InitLowerWindowText(TEXT_WORD_SELECT);
                (*sScreenControl).funcState += 1;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateWordSelectCursorSprite();
                SetScrollIndicatorXPos(TRUE as u32);
                UpdateScrollIndicatorsVisibility();
                UpdateStartSelectButtonsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        5 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn CloseWordSelect() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintCurrentPhrase();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            DestroyWordSelectCursorSprite();
            HideScrollIndicators();
            HideStartSelectButtons();
            ClearWordSelectWindow();
            (*sScreenControl).funcState += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_CLOSE_WORD_SELECT);
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if UpdateLowerWindowAnim() == 0 {
                ShowBg(0);
                (*sScreenControl).funcState += 1;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                StartMainCursorAnim();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        5 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowConfirmLyricsPrompt() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintCurrentPhrase();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            DestroyWordSelectCursorSprite();
            HideScrollIndicators();
            HideStartSelectButtons();
            ClearWordSelectWindow();
            (*sScreenControl).funcState += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_CLOSE_WORD_SELECT);
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if UpdateLowerWindowAnim() == 0 {
                PrintEasyChatStdMessage(MSG_CONFIRM);
                (*sScreenControl).funcState += 1;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                (*sScreenControl).funcState += 1;
            }
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                StartMainCursorAnim();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        6 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ReturnToKeyboard() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            DestroyWordSelectCursorSprite();
            HideScrollIndicators();
            HideStartSelectButtons();
            ClearWordSelectWindow();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowAnim(WINANIM_RETURN_TO_KEYBOARD);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if UpdateLowerWindowAnim() == 0 {
                PrintKeyboardText();
                (*sScreenControl).funcState += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                CreateSideWindowSprites();
                (*sScreenControl).funcState += 1;
            }
        }
        4 => {
            if ShowSideWindow() == 0 {
                CreateRectangleCursorSprites();
                SetScrollIndicatorXPos(FALSE as u32);
                UpdateScrollIndicatorsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateWordSelectCursor() -> u8 {
    UpdateWordSelectCursorPos();
    return FALSE;
}
pub(crate) unsafe extern "C" fn WordSelectScrollDown() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintWordSelectNextRowDown();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowScroll(1, 4);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if UpdateLowerWindowScroll() == 0 {
                UpdateWordSelectCursorPos();
                UpdateScrollIndicatorsVisibility();
                UpdateStartSelectButtonsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        3 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn WordSelectScrollUp() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintWordSelectNextRowUp();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                InitLowerWindowScroll(-1, 4);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if UpdateLowerWindowScroll() == 0 {
                UpdateScrollIndicatorsVisibility();
                UpdateStartSelectButtonsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        3 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn WordSelectPageScrollDown() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintWordSelectRowsPageDown();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                let mut scrollChange: i16 =
                    GetWordSelectScrollOffset() as i16 - GetLowerWindowScrollOffset() as i16;
                InitLowerWindowScroll(scrollChange, 8);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if UpdateLowerWindowScroll() == 0 {
                UpdateWordSelectCursorPos();
                UpdateScrollIndicatorsVisibility();
                UpdateStartSelectButtonsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        3 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn WordSelectPageScrollUp() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            PrintWordSelectRowsPageUp();
            (*sScreenControl).funcState += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                let mut scrollChange: i16 =
                    GetWordSelectScrollOffset() as i16 - GetLowerWindowScrollOffset() as i16;
                InitLowerWindowScroll(scrollChange, 8);
                (*sScreenControl).funcState += 1;
            }
        }
        2 => {
            if UpdateLowerWindowScroll() == 0 {
                UpdateScrollIndicatorsVisibility();
                UpdateStartSelectButtonsVisibility();
                (*sScreenControl).funcState += 1;
                return FALSE;
            }
        }
        3 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowCreateQuizMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CREATE_QUIZ);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowSelectAnswerMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_SELECT_ANSWER);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowSongTooShortMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_SONG_TOO_SHORT);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowCantDeleteLyricsMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CANT_DELETE_LYRICS);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowCombineTwoWordsMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_COMBINE_TWO_WORDS);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ShowCantExitMsg() -> u8 {
    match (*sScreenControl).funcState {
        0 => {
            StopMainCursorAnim();
            PrintEasyChatStdMessage(MSG_CANT_QUIT);
            (*sScreenControl).funcState += 1;
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenControl_() -> u8 {
    sScreenControl = Alloc(4864) as *mut EasyChatScreenControl;
    if sScreenControl.is_null() {
        return FALSE;
    }
    (*sScreenControl).funcState = 0;
    (*sScreenControl).mainCursorSprite = null_mut();
    (*sScreenControl).rectangleCursorSpriteRight = null_mut();
    (*sScreenControl).rectangleCursorSpriteLeft = null_mut();
    (*sScreenControl).wordSelectCursorSprite = null_mut();
    (*sScreenControl).buttonWindowSprite = null_mut();
    (*sScreenControl).modeWindowSprite = null_mut();
    (*sScreenControl).scrollIndicatorUpSprite = null_mut();
    (*sScreenControl).scrollIndicatorDownSprite = null_mut();
    (*sScreenControl).startButtonSprite = null_mut();
    (*sScreenControl).selectButtonSprite = null_mut();
    (*sScreenControl).fourFooterOptions = FooterHasFourOptions_() as u8;
    return TRUE;
}
pub(crate) unsafe extern "C" fn InitEasyChatBgs() {
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    SetGpuReg(0x0, 12352);
}
pub(crate) unsafe extern "C" fn LoadEasyChatPalettes() {
    ResetPaletteFade();
    LoadPalette(gEasyChatMode_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
    LoadPalette(
        sTextInputFrameOrange_Pal.as_ptr().cast_mut() as *mut c_void,
        16,
        32,
    );
    LoadPalette(
        sTextInputFrameGreen_Pal.as_ptr().cast_mut() as *mut c_void,
        64,
        32,
    );
    LoadPalette(sTitleText_Pal.as_ptr().cast_mut() as *mut c_void, 160, 8);
    LoadPalette(sText_Pal.as_ptr().cast_mut() as *mut c_void, 176, 12);
    LoadPalette(sText_Pal.as_ptr().cast_mut() as *mut c_void, 240, 12);
    LoadPalette(sText_Pal.as_ptr().cast_mut() as *mut c_void, 48, 12);
}
pub(crate) unsafe extern "C" fn PrintTitle() {
    let mut xOffset: i32 = 0;
    let mut titleText: *mut u8 = GetTitleText();
    if titleText.is_null() {
        return;
    }
    xOffset = GetStringCenterAlignXOffset(FONT_NORMAL as i32, titleText, 144);
    FillWindowPixelBuffer(WIN_TITLE, 0);
    PrintEasyChatTextWithColors(
        WIN_TITLE,
        FONT_NORMAL,
        titleText,
        xOffset as u8,
        1,
        TEXT_SKIP_DRAW,
        TEXT_COLOR_TRANSPARENT,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_LIGHT_GRAY,
    );
    PutWindowTilemap(WIN_TITLE);
    CopyWindowToVram(WIN_TITLE, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PrintEasyChatText(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    speed: u8,
    callback: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
) {
    AddTextPrinterParameterized(windowId, fontId, str, x, y, speed, callback);
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
    let mut color: CArray<u8, 3> = zeroed();
    color[0] = bg;
    color[1] = fg;
    color[2] = shadow;
    AddTextPrinterParameterized3(
        windowId,
        fontId,
        left,
        top,
        color.as_mut_ptr(),
        speed as i8,
        str,
    );
}
pub(crate) unsafe extern "C" fn PrintInitialInstructions() {
    FillBgTilemapBufferRect(0, 0, 0, 0, 32, 20, 17);
    LoadUserWindowBorderGfx(WIN_MSG, 1, 224);
    DrawTextBorderOuter(WIN_MSG, 1, 14);
    PrintEasyChatStdMessage(MSG_INSTRUCTIONS);
    PutWindowTilemap(WIN_MSG);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn PrintEasyChatStdMessage(msgId: u8) {
    let mut text2: *mut u8 = null_mut();
    let mut text1: *mut u8 = null_mut();
    match msgId {
        MSG_INSTRUCTIONS => {
            GetEasyChatInstructionsText(&raw mut text1, &raw mut text2);
        }
        MSG_CONFIRM_EXIT => {
            GetEasyChatConfirmExitText(&raw mut text1, &raw mut text2);
        }
        MSG_CONFIRM => {
            GetEasyChatConfirmText(&raw mut text1, &raw mut text2);
        }
        MSG_CONFIRM_DELETE => {
            GetEasyChatConfirmDeletionText(&raw mut text1, &raw mut text2);
        }
        MSG_CREATE_QUIZ => {
            text1 = gText_CreateAQuiz.as_ptr().cast_mut();
        }
        MSG_SELECT_ANSWER => {
            text1 = gText_SelectTheAnswer.as_ptr().cast_mut();
        }
        MSG_SONG_TOO_SHORT => {
            text1 = gText_OnlyOnePhrase.as_ptr().cast_mut();
            text2 = gText_OriginalSongWillBeUsed.as_ptr().cast_mut();
        }
        MSG_CANT_DELETE_LYRICS => {
            text1 = gText_LyricsCantBeDeleted.as_ptr().cast_mut();
        }
        MSG_COMBINE_TWO_WORDS => {
            text1 = gText_CombineTwoWordsOrPhrases3.as_ptr().cast_mut();
        }
        MSG_CANT_QUIT => {
            text1 = gText_YouCannotQuitHere.as_ptr().cast_mut();
            text2 = gText_SectionMustBeCompleted.as_ptr().cast_mut();
        }
        _ => {}
    }
    FillWindowPixelBuffer(WIN_MSG, 17);
    if !text1.is_null() {
        PrintEasyChatText(WIN_MSG, FONT_NORMAL, text1, 0, 1, TEXT_SKIP_DRAW, None);
    }
    if !text2.is_null() {
        PrintEasyChatText(WIN_MSG, FONT_NORMAL, text2, 0, 17, TEXT_SKIP_DRAW, None);
    }
    CopyWindowToVram(WIN_MSG, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn CreateEasyChatYesNoMenu(initialCursorPos: u8) {
    CreateYesNoMenu(
        (&raw const *sEasyChatYesNoWindowTemplate).cast_mut(),
        1,
        14,
        initialCursorPos,
    );
}
pub(crate) unsafe extern "C" fn AddPhraseWindow() {
    let mut frameId: u8 = 0;
    let mut template: WindowTemplate = zeroed();
    frameId = GetEasyChatScreenFrameId();
    template.bg = 3;
    template.tilemapLeft = sPhraseFrameDimensions[frameId].left();
    template.tilemapTop = sPhraseFrameDimensions[frameId].top();
    template.width = sPhraseFrameDimensions[frameId].width;
    template.height = sPhraseFrameDimensions[frameId].height;
    template.paletteNum = 11;
    template.baseBlock = 0x6C;
    (*sScreenControl).windowId = AddWindow(&raw mut template);
    PutWindowTilemap((*sScreenControl).windowId as u8);
}
pub(crate) unsafe extern "C" fn PrintCurrentPhrase() {
    let mut strClear: CArray<u8, 4> = zeroed();
    let mut currentPhrase: *mut u16 = null_mut();
    let mut numColumns: u8 = 0;
    let mut numRows: u8 = 0;
    let mut str: *mut u8 = null_mut();
    let mut frameId: i32 = 0;
    let mut isQuizQuestion: u32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    currentPhrase = GetCurrentPhrase();
    numColumns = GetNumColumns();
    numRows = GetNumRows();
    frameId = GetEasyChatScreenFrameId() as i32;
    isQuizQuestion = FALSE as u32;
    if frameId == FRAMEID_QUIZ_QUESTION {
        isQuizQuestion = TRUE as u32;
    }
    FillWindowPixelBuffer((*sScreenControl).windowId as u8, 17);
    i = 0;
    while i < numRows as i32 {
        memcpy(strClear.as_mut_ptr(), sText_Clear17.as_ptr().cast_mut(), 4);
        if isQuizQuestion != 0 {
            strClear[2] = 6;
        }
        str = (*sScreenControl).phrasePrintBuffer.as_mut_ptr();
        (*sScreenControl).phrasePrintBuffer[0] = EOS;
        str = StringAppend(str, strClear.as_mut_ptr());
        j = 0;
        while j < numColumns as i32 {
            if *currentPhrase != EC_EMPTY_WORD {
                str = CopyEasyChatWord(str, *currentPhrase);
                currentPhrase = currentPhrase.at(1);
            } else {
                currentPhrase = currentPhrase.at(1);
                if isQuizQuestion == 0 {
                    str = WriteColorChangeControlCode(str, 0, 4);
                    k = 0;
                    while k < 12 {
                        *str = CHAR_HYPHEN;
                        str = str.at(1);
                        k += 1;
                    }
                    str = WriteColorChangeControlCode(str, 0, 2);
                }
            }
            if isQuizQuestion != 0 {
                strClear[2] = 3;
            }
            str = StringAppend(str, strClear.as_mut_ptr());
            if frameId == FRAMEID_MAIL as i32
                || frameId == FRAMEID_QUIZ_QUESTION
                || frameId == FRAMEID_QUIZ_SET_QUESTION as i32
            {
                if j == 0 && i == 4 {
                    break;
                }
            }
            j += 1;
        }
        *str = EOS;
        PrintEasyChatText(
            (*sScreenControl).windowId as u8,
            FONT_NORMAL,
            (*sScreenControl).phrasePrintBuffer.as_mut_ptr(),
            0,
            i as u8 * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    CopyWindowToVram((*sScreenControl).windowId as u8, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn BufferFrameTilemap(mut tilemap: *mut u16) {
    let mut frameId: u8 = 0;
    let mut right: i32 = 0;
    let mut bottom: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    frameId = GetEasyChatScreenFrameId();
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            tilemap as *mut c_void,
            0x1000200,
        );
    }
    if frameId == FRAMEID_MAIL || frameId == FRAMEID_QUIZ_SET_QUESTION {
        right = sPhraseFrameDimensions[frameId].left() as i32
            + sPhraseFrameDimensions[frameId].width as i32;
        bottom = sPhraseFrameDimensions[frameId].top() as i32
            + sPhraseFrameDimensions[frameId].height as i32;
        y = sPhraseFrameDimensions[frameId].top() as i32;
        while y < bottom {
            x = sPhraseFrameDimensions[frameId].left() as i32 - 1;
            *tilemap.at(y * 32 + x) = 4101;
            x += 1;
            while x < right {
                *tilemap.at(y * 32 + x) = FRAME_OFFSET_ORANGE;
                x += 1;
            }
            *tilemap.at(y * 32 + x) = 4103;
            y += 1;
        }
    } else {
        y = sPhraseFrameDimensions[frameId].top() as i32 - 1;
        x = sPhraseFrameDimensions[frameId].left() as i32 - 1;
        right = sPhraseFrameDimensions[frameId].left() as i32
            + sPhraseFrameDimensions[frameId].width as i32;
        bottom = sPhraseFrameDimensions[frameId].top() as i32
            + sPhraseFrameDimensions[frameId].height as i32;
        *tilemap.at(y * 32 + x) = 4097;
        x += 1;
        while x < right {
            *tilemap.at(y * 32 + x) = 4098;
            x += 1;
        }
        *tilemap.at(y * 32 + x) = 4099;
        y += 1;
        while y < bottom {
            x = sPhraseFrameDimensions[frameId].left() as i32 - 1;
            *tilemap.at(y * 32 + x) = 4101;
            x += 1;
            while x < right {
                *tilemap.at(y * 32 + x) = FRAME_OFFSET_ORANGE;
                x += 1;
            }
            *tilemap.at(y * 32 + x) = 4103;
            y += 1;
        }
        x = sPhraseFrameDimensions[frameId].left() as i32 - 1;
        *tilemap.at(y * 32 + x) = 4105;
        x += 1;
        while x < right {
            *tilemap.at(y * 32 + x) = 4106;
            x += 1;
        }
        *tilemap.at(y * 32 + x) = 4107;
    }
}
pub(crate) unsafe extern "C" fn AdjustBgTilemapForFooter() {
    let mut frameId: u8 = 0;
    let mut tilemap: *mut u16 = null_mut();
    tilemap = GetBgTilemapBuffer(3) as *mut u16;
    frameId = GetEasyChatScreenFrameId();
    match sPhraseFrameDimensions[frameId].footerId {
        FOOTER_ANSWER => {
            tilemap = tilemap.at(672);
            CopyToBgTilemapBufferRect(3, tilemap as *mut c_void, 0, 11, 32, 2);
        }
        FOOTER_QUIZ => {
            tilemap = tilemap.at(768);
            CopyToBgTilemapBufferRect(3, tilemap as *mut c_void, 0, 11, 32, 2);
        }
        3 => {
            CopyToBgTilemapBufferRect(3, tilemap as *mut c_void, 0, 10, 32, 4);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DrawLowerWindow() {
    PutWindowTilemap(WIN_INPUT_SELECT);
    CopyBgTilemapBufferToVram(WIN_INPUT_SELECT);
}
pub(crate) unsafe extern "C" fn InitLowerWindowText(whichText: u32) {
    ResetLowerWindowScroll();
    FillWindowPixelBuffer(WIN_INPUT_SELECT, 17);
    match whichText {
        TEXT_GROUPS => {
            PrintKeyboardGroupNames();
        }
        TEXT_ALPHABET => {
            PrintKeyboardAlphabet();
        }
        TEXT_WORD_SELECT => {
            PrintInitialWordSelectText();
        }
        _ => {}
    }
    CopyWindowToVram(WIN_INPUT_SELECT, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn PrintKeyboardText() {
    if GetInAlphabetMode() == 0 {
        InitLowerWindowText(TEXT_GROUPS);
    } else {
        InitLowerWindowText(TEXT_ALPHABET);
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardGroupNames() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    i = 0;
    y = 97;
    loop {
        x = 0;
        while x < 2 {
            let mut groupId: u8 = GetUnlockedEasyChatGroupId(
                ({
                    let t1 = i;
                    i += 1;
                    t1
                }) as u8,
            );
            if groupId == EC_NUM_GROUPS {
                InitLowerWindowScroll(GetKeyboardScrollOffset() as i16, 0);
                return;
            }
            PrintEasyChatText(
                WIN_INPUT_SELECT,
                FONT_NORMAL,
                GetEasyChatWordGroupName(groupId),
                x as u8 * 84 + 10,
                y as u8,
                TEXT_SKIP_DRAW,
                None,
            );
            x += 1;
        }
        y += 16;
    }
}
pub(crate) unsafe extern "C" fn PrintKeyboardAlphabet() {
    let mut i: u32 = 0;
    i = 0;
    while i < 4 {
        PrintEasyChatText(
            WIN_INPUT_SELECT,
            FONT_NORMAL,
            sEasyChatKeyboardAlphabet[i],
            10,
            97 + i as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintInitialWordSelectText() {
    PrintWordSelectText(0, NUM_WORD_SELECT_ROWS);
}
pub(crate) unsafe extern "C" fn PrintWordSelectNextRowDown() {
    let mut wordScroll: u8 = GetWordSelectScrollOffset() + NUM_WORD_SELECT_ROWS - 1;
    EraseWordSelectRows(wordScroll, 1);
    PrintWordSelectText(wordScroll, 1);
}
pub(crate) unsafe extern "C" fn PrintWordSelectNextRowUp() {
    let mut wordScroll: u8 = GetWordSelectScrollOffset();
    EraseWordSelectRows(wordScroll, 1);
    PrintWordSelectText(wordScroll, 1);
}
pub(crate) unsafe extern "C" fn PrintWordSelectRowsPageDown() {
    let mut wordScroll: u8 = GetWordSelectScrollOffset();
    let mut maxScroll: u8 = wordScroll + NUM_WORD_SELECT_ROWS;
    let mut maxRows: u8 = GetWordSelectLastRow() + 1;
    if maxScroll > maxRows {
        maxScroll = maxRows;
    }
    if wordScroll < maxScroll {
        let mut numRows: u8 = maxScroll - wordScroll;
        EraseWordSelectRows(wordScroll, numRows);
        PrintWordSelectText(wordScroll, numRows);
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectRowsPageUp() {
    let mut wordScroll: u8 = GetWordSelectScrollOffset();
    let mut windowScroll: u8 = GetLowerWindowScrollOffset() as u8;
    if wordScroll < windowScroll {
        let mut numRows: u8 = windowScroll - wordScroll;
        EraseWordSelectRows(wordScroll, numRows);
        PrintWordSelectText(wordScroll, numRows);
    }
}
pub(crate) unsafe extern "C" fn PrintWordSelectText(scrollOffset: u8, numRows: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut easyChatWord: u16 = 0;
    let mut y: i32 = 0;
    let mut wordIndex: i32 = 0;
    wordIndex = scrollOffset as i32 * NUM_WORD_SELECT_COLUMNS as i32;
    y = scrollOffset as i32 * 16 + 96 & 0xFF;
    y += 1;
    i = 0;
    while i < numRows as i32 {
        j = 0;
        while j < 2 {
            easyChatWord = GetWordFromSelectedGroup(
                ({
                    let t1 = wordIndex;
                    wordIndex += 1;
                    t1
                }) as u16,
            );
            if easyChatWord != EC_EMPTY_WORD {
                CopyEasyChatWordPadded(
                    (*sScreenControl).wordSelectPrintBuffer.as_mut_ptr(),
                    easyChatWord,
                    0,
                );
                if DummyWordCheck(easyChatWord as i32) == 0 {
                    PrintEasyChatText(
                        WIN_INPUT_SELECT,
                        FONT_NORMAL,
                        (*sScreenControl).wordSelectPrintBuffer.as_mut_ptr(),
                        (j as u8 * 13 + 3) * 8,
                        y as u8,
                        TEXT_SKIP_DRAW,
                        None,
                    );
                } else {
                    PrintEasyChatTextWithColors(
                        WIN_INPUT_SELECT,
                        FONT_NORMAL,
                        (*sScreenControl).wordSelectPrintBuffer.as_mut_ptr(),
                        (j as u8 * 13 + 3) * 8,
                        y as u8,
                        TEXT_SKIP_DRAW,
                        TEXT_COLOR_WHITE,
                        TEXT_COLOR_LIGHT_RED,
                        0x3,
                    );
                }
            }
            j += 1;
        }
        y += 16;
        i += 1;
    }
    CopyWindowToVram(WIN_INPUT_SELECT, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn EraseWordSelectRows(scrollOffset: u8, numRows: u8) {
    let mut y: i32 = 0;
    let mut var0: i32 = 0;
    let mut var1: i32 = 0;
    let mut var2: i32 = 0;
    y = scrollOffset as i32 * 16 + 96 & 0xFF;
    var2 = numRows as i32 * 16;
    var0 = y + var2;
    if var0 > 255 {
        var1 = var0 - 256;
        var2 = 256 - y;
    } else {
        var1 = 0;
    }
    FillWindowPixelRect(WIN_INPUT_SELECT, 17, 0, y as u16, 224, var2 as u16);
    if var1 != 0 {
        FillWindowPixelRect(WIN_INPUT_SELECT, 17, 0, 0, 224, var1 as u16);
    }
}
pub(crate) unsafe extern "C" fn ClearWordSelectWindow() {
    FillWindowPixelBuffer(WIN_INPUT_SELECT, 17);
    CopyWindowToVram(WIN_INPUT_SELECT, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn InitLowerWindowAnim(winAnimType: i32) {
    match winAnimType {
        WINANIM_OPEN_KEYBOARD => {
            (*sScreenControl).curWindowAnimState = 0;
            (*sScreenControl).destWindowAnimState = 10;
        }
        WINANIM_CLOSE_KEYBOARD => {
            (*sScreenControl).curWindowAnimState = 9;
            (*sScreenControl).destWindowAnimState = 0;
        }
        WINANIM_OPEN_WORD_SELECT => {
            (*sScreenControl).curWindowAnimState = 11;
            (*sScreenControl).destWindowAnimState = 17;
        }
        WINANIM_CLOSE_WORD_SELECT => {
            (*sScreenControl).curWindowAnimState = 17;
            (*sScreenControl).destWindowAnimState = 0;
        }
        WINANIM_RETURN_TO_KEYBOARD => {
            (*sScreenControl).curWindowAnimState = 17;
            (*sScreenControl).destWindowAnimState = 10;
        }
        WINANIM_KEYBOARD_SWITCH_OUT => {
            (*sScreenControl).curWindowAnimState = 18;
            (*sScreenControl).destWindowAnimState = 22;
        }
        WINANIM_KEYBOARD_SWITCH_IN => {
            (*sScreenControl).curWindowAnimState = 22;
            (*sScreenControl).destWindowAnimState = 18;
        }
        _ => {}
    }
    (*sScreenControl).windowAnimStateDir =
        (if (*sScreenControl).curWindowAnimState < (*sScreenControl).destWindowAnimState {
            1
        } else {
            -1
        }) as i8;
}
pub(crate) unsafe extern "C" fn UpdateLowerWindowAnim() -> u8 {
    let mut curState: u8 = 0;
    let mut destState: u8 = 0;
    if (*sScreenControl).curWindowAnimState == (*sScreenControl).destWindowAnimState {
        return FALSE;
    }
    (*sScreenControl).curWindowAnimState += (*sScreenControl).windowAnimStateDir as u8;
    DrawLowerWindowFrame((*sScreenControl).curWindowAnimState);
    curState = (*sScreenControl).curWindowAnimState;
    destState = (*sScreenControl).destWindowAnimState;
    return (curState as i32 ^ destState as i32 > 0) as u8;
}
pub(crate) unsafe extern "C" fn DrawLowerWindowFrame(r#type: u8) {
    FillBgTilemapBufferRect_Palette0(1, 0, 0, 10, 30, 10);
    match r#type {
        0 => {}
        1 => {
            BufferLowerWindowFrame(11, 14, 3, 2);
        }
        2 => {
            BufferLowerWindowFrame(9, 14, 7, 2);
        }
        3 => {
            BufferLowerWindowFrame(7, 14, 11, 2);
        }
        4 => {
            BufferLowerWindowFrame(5, 14, 15, 2);
        }
        5 => {
            BufferLowerWindowFrame(3, 14, 19, 2);
        }
        6 => {
            BufferLowerWindowFrame(1, 14, 23, 2);
        }
        7 => {
            BufferLowerWindowFrame(1, 13, 23, 4);
        }
        8 => {
            BufferLowerWindowFrame(1, 12, 23, 6);
        }
        9 => {
            BufferLowerWindowFrame(1, 11, 23, 8);
        }
        10 => {
            BufferLowerWindowFrame(1, 10, 23, 10);
        }
        11 => {
            BufferLowerWindowFrame(1, 10, 24, 10);
        }
        12 => {
            BufferLowerWindowFrame(1, 10, 25, 10);
        }
        13 => {
            BufferLowerWindowFrame(1, 10, 26, 10);
        }
        14 => {
            BufferLowerWindowFrame(1, 10, 27, 10);
        }
        15 => {
            BufferLowerWindowFrame(1, 10, 28, 10);
        }
        16 => {
            BufferLowerWindowFrame(1, 10, 29, 10);
        }
        17 => {
            BufferLowerWindowFrame(0, 10, 30, 10);
        }
        18 => {
            BufferLowerWindowFrame(1, 10, 23, 10);
        }
        19 => {
            BufferLowerWindowFrame(1, 11, 23, 8);
        }
        20 => {
            BufferLowerWindowFrame(1, 12, 23, 6);
        }
        21 => {
            BufferLowerWindowFrame(1, 13, 23, 4);
        }
        22 => {
            BufferLowerWindowFrame(1, 14, 23, 2);
        }
        _ => {}
    }
    CopyBgTilemapBufferToVram(1);
}
pub(crate) unsafe extern "C" fn BufferLowerWindowFrame(
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) {
    let mut tilemap: *mut u16 = null_mut();
    let mut right: i32 = 0;
    let mut bottom: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    tilemap = (*sScreenControl).bg1TilemapBuffer.as_mut_ptr();
    right = left + width - 1;
    bottom = top + height - 1;
    x = left;
    y = top;
    *tilemap.at(y * 32 + x) = 16385;
    x += 1;
    while x < right {
        *tilemap.at(y * 32 + x) = 16386;
        x += 1;
    }
    *tilemap.at(y * 32 + x) = 16387;
    y += 1;
    while y < bottom {
        *tilemap.at(y * 32 + left) = 16389;
        x = left + 1;
        while x < right {
            *tilemap.at(y * 32 + x) = FRAME_OFFSET_GREEN;
            x += 1;
        }
        *tilemap.at(y * 32 + x) = 16391;
        y += 1;
    }
    *tilemap.at(y * 32 + left) = 16393;
    x = left + 1;
    while x < right {
        *tilemap.at(y * 32 + x) = 16394;
        x += 1;
    }
    *tilemap.at(y * 32 + x) = 16395;
    SetWindowDimensions(
        (left as u8 + 1) * 8,
        (top as u8 + 1) * 8,
        (width as u8 - 2) * 8,
        (height as u8 - 2) * 8,
    );
}
pub(crate) unsafe extern "C" fn ResetLowerWindowScroll() {
    ChangeBgY(2, 0x800, BG_COORD_SET);
    (*sScreenControl).scrollOffset = 0;
}
pub(crate) unsafe extern "C" fn InitLowerWindowScroll(scrollChange: i16, speed: u8) {
    let mut bgY: i32 = 0;
    let mut yChange: i16 = 0;
    bgY = GetBgY(2);
    (*sScreenControl).scrollOffset += scrollChange as u16;
    yChange = scrollChange * 16;
    bgY += yChange as i32 * 256;
    if speed != 0 {
        (*sScreenControl).scrollDest = bgY;
        (*sScreenControl).scrollSpeed = speed as i32 * 256;
        if yChange < 0 {
            (*sScreenControl).scrollSpeed = -(*sScreenControl).scrollSpeed;
        }
    } else {
        ChangeBgY(2, bgY, BG_COORD_SET);
    }
}
pub(crate) unsafe extern "C" fn UpdateLowerWindowScroll() -> u8 {
    let mut bgY: i32 = 0;
    bgY = GetBgY(2);
    if bgY == (*sScreenControl).scrollDest {
        return FALSE;
    } else {
        ChangeBgY(2, (*sScreenControl).scrollSpeed, BG_COORD_ADD);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetLowerWindowScrollOffset() -> i32 {
    return (*sScreenControl).scrollOffset as i32;
}
pub(crate) unsafe extern "C" fn SetWindowDimensions(left: u8, top: u8, width: u8, height: u8) {
    let mut horizontalDimensions: u16 = (left as u16) << 8 | left as u16 + width as u16;
    let mut verticalDimensions: u16 = (top as u16) << 8 | top as u16 + height as u16;
    SetGpuReg(REG_OFFSET_WIN0H, horizontalDimensions);
    SetGpuReg(REG_OFFSET_WIN0V, verticalDimensions);
}
pub(crate) unsafe extern "C" fn LoadEasyChatGfx() {
    let mut i: u32 = 0;
    LoadSpriteSheets(sSpriteSheets.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
    i = 0;
    while i < 4 {
        LoadCompressedSpriteSheet((&raw const sCompressedSpriteSheets[i]).cast_mut());
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateMainCursorSprite() {
    let mut frameId: u8 = GetEasyChatScreenFrameId();
    let mut x: i32 = sPhraseFrameDimensions[frameId].left() as i32 * 8 + 13;
    let mut y: i32 = sPhraseFrameDimensions[frameId].top() as i32 * 8 + 8;
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_TriangleCursor).cast_mut(),
        x as i16,
        y as i16,
        2,
    );
    (*sScreenControl).mainCursorSprite = &raw mut gSprites[spriteId];
    gSprites[spriteId].data[1] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor(sprite: *mut Sprite) {
    if (*sprite).data[1] != 0 {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) > 2
        {
            (*sprite).data[0] = 0;
            if ({
                (*sprite).x2 += 1;
                (*sprite).x2
            }) > 0
            {
                (*sprite).x2 = -6;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMainCursorPos(x: u8, y: u8) {
    (*(*sScreenControl).mainCursorSprite).x = x as i16;
    (*(*sScreenControl).mainCursorSprite).y = y as i16;
    (*(*sScreenControl).mainCursorSprite).x2 = 0;
    (*(*sScreenControl).mainCursorSprite).data[0] = 0;
}
pub(crate) unsafe extern "C" fn StopMainCursorAnim() {
    (*(*sScreenControl).mainCursorSprite).data[0] = 0;
    (*(*sScreenControl).mainCursorSprite).data[1] = FALSE as i16;
    (*(*sScreenControl).mainCursorSprite).x2 = 0;
}
pub(crate) unsafe extern "C" fn StartMainCursorAnim() {
    (*(*sScreenControl).mainCursorSprite).data[1] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn CreateRectangleCursorSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_RectangleCursor).cast_mut(),
        0,
        0,
        3,
    );
    (*sScreenControl).rectangleCursorSpriteRight = &raw mut gSprites[spriteId];
    (*(*sScreenControl).rectangleCursorSpriteRight).x2 = 32;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_RectangleCursor).cast_mut(),
        0,
        0,
        3,
    );
    (*sScreenControl).rectangleCursorSpriteLeft = &raw mut gSprites[spriteId];
    (*(*sScreenControl).rectangleCursorSpriteLeft).x2 = -32;
    (*(*sScreenControl).rectangleCursorSpriteRight).set_hFlip(TRUE as u16);
    UpdateRectangleCursorPos();
}
pub(crate) unsafe extern "C" fn DestroyRectangleCursorSprites() {
    DestroySprite((*sScreenControl).rectangleCursorSpriteRight);
    (*sScreenControl).rectangleCursorSpriteRight = null_mut();
    DestroySprite((*sScreenControl).rectangleCursorSpriteLeft);
    (*sScreenControl).rectangleCursorSpriteLeft = null_mut();
}
pub(crate) unsafe extern "C" fn UpdateRectangleCursorPos() {
    let mut column: i8 = 0;
    let mut row: i8 = 0;
    if !(*sScreenControl).rectangleCursorSpriteRight.is_null()
        && !(*sScreenControl).rectangleCursorSpriteLeft.is_null()
    {
        GetKeyboardCursorColAndRow(&raw mut column, &raw mut row);
        if GetInAlphabetMode() == 0 {
            SetRectangleCursorPos_GroupMode(column, row);
        } else {
            SetRectangleCursorPos_AlphabetMode(column, row);
        }
    }
}
pub(crate) unsafe extern "C" fn SetRectangleCursorPos_GroupMode(column: i8, row: i8) {
    if column != -1 {
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteRight,
            RECTCURSOR_ANIM_ON_GROUP,
        );
        (*(*sScreenControl).rectangleCursorSpriteRight).x = column as i16 * 84 + 58;
        (*(*sScreenControl).rectangleCursorSpriteRight).y = row as i16 * 16 + 96;
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteLeft,
            RECTCURSOR_ANIM_ON_GROUP,
        );
        (*(*sScreenControl).rectangleCursorSpriteLeft).x = column as i16 * 84 + 58;
        (*(*sScreenControl).rectangleCursorSpriteLeft).y = row as i16 * 16 + 96;
    } else {
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteRight,
            RECTCURSOR_ANIM_ON_BUTTON,
        );
        (*(*sScreenControl).rectangleCursorSpriteRight).x = 216;
        (*(*sScreenControl).rectangleCursorSpriteRight).y = row as i16 * 16 + 112;
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteLeft,
            RECTCURSOR_ANIM_ON_BUTTON,
        );
        (*(*sScreenControl).rectangleCursorSpriteLeft).x = 216;
        (*(*sScreenControl).rectangleCursorSpriteLeft).y = row as i16 * 16 + 112;
    }
}
pub(crate) unsafe extern "C" fn SetRectangleCursorPos_AlphabetMode(column: i8, row: i8) {
    let mut anim: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if column != -1 {
        y = row as i32 * 16 + 96;
        x = 32;
        if column == 6 && row == 0 {
            x = 158;
            anim = RECTCURSOR_ANIM_ON_OTHERS;
        } else {
            x += sAlphabetKeyboardColumnOffsets[if (column as u8) < NUM_ALPHABET_COLUMNS {
                column as i32
            } else {
                0
            }] as i32;
            anim = RECTCURSOR_ANIM_ON_LETTER;
        }
        StartSpriteAnim((*sScreenControl).rectangleCursorSpriteRight, anim as u8);
        (*(*sScreenControl).rectangleCursorSpriteRight).x = x as i16;
        (*(*sScreenControl).rectangleCursorSpriteRight).y = y as i16;
        StartSpriteAnim((*sScreenControl).rectangleCursorSpriteLeft, anim as u8);
        (*(*sScreenControl).rectangleCursorSpriteLeft).x = x as i16;
        (*(*sScreenControl).rectangleCursorSpriteLeft).y = y as i16;
    } else {
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteRight,
            RECTCURSOR_ANIM_ON_BUTTON,
        );
        (*(*sScreenControl).rectangleCursorSpriteRight).x = 216;
        (*(*sScreenControl).rectangleCursorSpriteRight).y = row as i16 * 16 + 112;
        StartSpriteAnim(
            (*sScreenControl).rectangleCursorSpriteLeft,
            RECTCURSOR_ANIM_ON_BUTTON,
        );
        (*(*sScreenControl).rectangleCursorSpriteLeft).x = 216;
        (*(*sScreenControl).rectangleCursorSpriteLeft).y = row as i16 * 16 + 112;
    }
}
pub(crate) unsafe extern "C" fn CreateWordSelectCursorSprite() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_TriangleCursor).cast_mut(),
        0,
        0,
        4,
    );
    (*sScreenControl).wordSelectCursorSprite = &raw mut gSprites[spriteId];
    (*(*sScreenControl).wordSelectCursorSprite).callback = Some(SpriteCB_WordSelectCursor);
    (*(*sScreenControl).wordSelectCursorSprite)
        .oam
        .set_priority(2);
    UpdateWordSelectCursorPos();
}
pub(crate) unsafe extern "C" fn SpriteCB_WordSelectCursor(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 2
    {
        (*sprite).data[0] = 0;
        if ({
            (*sprite).x2 += 1;
            (*sprite).x2
        }) > 0
        {
            (*sprite).x2 = -6;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateWordSelectCursorPos() {
    let mut column: i8 = 0;
    let mut row: i8 = 0;
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    GetWordSelectColAndRow(&raw mut column, &raw mut row);
    x = column * 13;
    x = x * 8 + 28;
    y = row * 16 + 96;
    SetWordSelectCursorPos(x as u8, y as u8);
}
pub(crate) unsafe extern "C" fn SetWordSelectCursorPos(x: u8, y: u8) {
    if !(*sScreenControl).wordSelectCursorSprite.is_null() {
        (*(*sScreenControl).wordSelectCursorSprite).x = x as i16;
        (*(*sScreenControl).wordSelectCursorSprite).y = y as i16;
        (*(*sScreenControl).wordSelectCursorSprite).x2 = 0;
        (*(*sScreenControl).wordSelectCursorSprite).data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn DestroyWordSelectCursorSprite() {
    if !(*sScreenControl).wordSelectCursorSprite.is_null() {
        DestroySprite((*sScreenControl).wordSelectCursorSprite);
        (*sScreenControl).wordSelectCursorSprite = null_mut();
    }
}
pub(crate) unsafe extern "C" fn CreateSideWindowSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ButtonWindow).cast_mut(),
        208,
        128,
        6,
    );
    (*sScreenControl).buttonWindowSprite = &raw mut gSprites[spriteId];
    (*(*sScreenControl).buttonWindowSprite).x2 = -64;
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ModeWindow).cast_mut(),
        208,
        80,
        5,
    );
    (*sScreenControl).modeWindowSprite = &raw mut gSprites[spriteId];
    (*sScreenControl).modeWindowState = 0;
}
pub(crate) unsafe extern "C" fn ShowSideWindow() -> u8 {
    match (*sScreenControl).modeWindowState {
        0 => {
            (*(*sScreenControl).buttonWindowSprite).x2 += 8;
            if (*(*sScreenControl).buttonWindowSprite).x2 >= 0 {
                (*(*sScreenControl).buttonWindowSprite).x2 = 0;
                if GetInAlphabetMode() == 0 {
                    StartSpriteAnim((*sScreenControl).modeWindowSprite, MODEWINDOW_ANIM_TO_GROUP);
                } else {
                    StartSpriteAnim(
                        (*sScreenControl).modeWindowSprite,
                        MODEWINDOW_ANIM_TO_ALPHABET,
                    );
                }
                (*sScreenControl).modeWindowState += 1;
            }
        }
        1 => {
            if (*(*sScreenControl).modeWindowSprite).animEnded() != 0 {
                (*sScreenControl).modeWindowState = 2;
                return FALSE;
            }
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn HideModeWindow() {
    (*sScreenControl).modeWindowState = 0;
    StartSpriteAnim(
        (*sScreenControl).modeWindowSprite,
        MODEWINDOW_ANIM_TO_HIDDEN,
    );
}
pub(crate) unsafe extern "C" fn DestroySideWindowSprites() -> u8 {
    match (*sScreenControl).modeWindowState {
        0 => {
            if (*(*sScreenControl).modeWindowSprite).animEnded() != 0 {
                (*sScreenControl).modeWindowState = 1;
            }
        }
        1 => {
            (*(*sScreenControl).buttonWindowSprite).x2 -= 8;
            if (*(*sScreenControl).buttonWindowSprite).x2 <= -64 {
                DestroySprite((*sScreenControl).modeWindowSprite);
                DestroySprite((*sScreenControl).buttonWindowSprite);
                (*sScreenControl).modeWindowSprite = null_mut();
                (*sScreenControl).buttonWindowSprite = null_mut();
                (*sScreenControl).modeWindowState += 1;
                return FALSE;
            }
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SetModeWindowToTransition() {
    StartSpriteAnim(
        (*sScreenControl).modeWindowSprite,
        MODEWINDOW_ANIM_TRANSITION,
    );
}
pub(crate) unsafe extern "C" fn UpdateModeWindowAnim() {
    if GetInAlphabetMode() == 0 {
        StartSpriteAnim((*sScreenControl).modeWindowSprite, MODEWINDOW_ANIM_TO_GROUP);
    } else {
        StartSpriteAnim(
            (*sScreenControl).modeWindowSprite,
            MODEWINDOW_ANIM_TO_ALPHABET,
        );
    }
}
pub(crate) unsafe extern "C" fn IsModeWindowAnimActive() -> u8 {
    return ((*(*sScreenControl).modeWindowSprite).animEnded() == 0) as u8;
}
pub(crate) unsafe extern "C" fn CreateScrollIndicatorSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_ScrollIndicator).cast_mut(),
        96,
        80,
        0,
    );
    if spriteId != MAX_SPRITES {
        (*sScreenControl).scrollIndicatorUpSprite = &raw mut gSprites[spriteId];
    }
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_ScrollIndicator).cast_mut(),
        96,
        156,
        0,
    );
    if spriteId != MAX_SPRITES {
        (*sScreenControl).scrollIndicatorDownSprite = &raw mut gSprites[spriteId];
        (*(*sScreenControl).scrollIndicatorDownSprite).set_vFlip(TRUE as u16);
    }
    HideScrollIndicators();
}
pub(crate) unsafe extern "C" fn UpdateScrollIndicatorsVisibility() {
    (*(*sScreenControl).scrollIndicatorUpSprite).set_invisible((CanScrollUp() == 0) as u16);
    (*(*sScreenControl).scrollIndicatorDownSprite).set_invisible((CanScrollDown() == 0) as u16);
}
pub(crate) unsafe extern "C" fn HideScrollIndicators() {
    (*(*sScreenControl).scrollIndicatorUpSprite).set_invisible(TRUE as u16);
    (*(*sScreenControl).scrollIndicatorDownSprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn SetScrollIndicatorXPos(inWordSelect: u32) {
    if inWordSelect == 0 {
        (*(*sScreenControl).scrollIndicatorUpSprite).x = 96;
        (*(*sScreenControl).scrollIndicatorDownSprite).x = 96;
    } else {
        (*(*sScreenControl).scrollIndicatorUpSprite).x = 120;
        (*(*sScreenControl).scrollIndicatorDownSprite).x = 120;
    }
}
pub(crate) unsafe extern "C" fn CreateStartSelectButtonSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_StartSelectButton).cast_mut(),
        220,
        84,
        1,
    );
    if spriteId != MAX_SPRITES {
        (*sScreenControl).startButtonSprite = &raw mut gSprites[spriteId];
    }
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_StartSelectButton).cast_mut(),
        220,
        156,
        1,
    );
    if spriteId != MAX_SPRITES {
        (*sScreenControl).selectButtonSprite = &raw mut gSprites[spriteId];
        StartSpriteAnim((*sScreenControl).selectButtonSprite, 1);
    }
    HideStartSelectButtons();
}
pub(crate) unsafe extern "C" fn UpdateStartSelectButtonsVisibility() {
    (*(*sScreenControl).startButtonSprite).set_invisible((CanScrollUp() == 0) as u16);
    (*(*sScreenControl).selectButtonSprite).set_invisible((CanScrollDown() == 0) as u16);
}
pub(crate) unsafe extern "C" fn HideStartSelectButtons() {
    (*(*sScreenControl).startButtonSprite).set_invisible(TRUE as u16);
    (*(*sScreenControl).selectButtonSprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn TryAddInterviewObjectEvents() {
    let mut graphicsId: i32 = 0;
    let mut spriteId: u8 = 0;
    match GetDisplayedPersonType() {
        EASY_CHAT_PERSON_REPORTER_MALE => {
            graphicsId = OBJ_EVENT_GFX_REPORTER_M;
        }
        EASY_CHAT_PERSON_REPORTER_FEMALE => {
            graphicsId = OBJ_EVENT_GFX_REPORTER_F;
        }
        EASY_CHAT_PERSON_BOY => {
            graphicsId = OBJ_EVENT_GFX_BOY_1 as i32;
        }
        _ => {
            return;
        }
    }
    if GetEasyChatScreenFrameId() != FRAMEID_INTERVIEW_SHOW_PERSON {
        return;
    }
    spriteId = CreateObjectGraphicsSprite(graphicsId as u16, Some(SpriteCallbackDummy), 76, 40, 0);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].oam.set_priority(0);
        StartSpriteAnim(&raw mut gSprites[spriteId], 2);
    }
    spriteId = CreateObjectGraphicsSprite(
        (if (*gSaveBlock2Ptr).playerGender == MALE {
            OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL as i32
        } else {
            OBJ_EVENT_GFX_RIVAL_MAY_NORMAL as i32
        }) as u16,
        Some(SpriteCallbackDummy),
        52,
        40,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].oam.set_priority(0);
        StartSpriteAnim(&raw mut gSprites[spriteId], 3);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFooterIndex() -> i32 {
    let mut frameId: u8 = GetEasyChatScreenFrameId();
    match sPhraseFrameDimensions[frameId].footerId {
        FOOTER_QUIZ => {
            return FOOTER_QUIZ as i32;
        }
        FOOTER_ANSWER => {
            return FOOTER_ANSWER as i32;
        }
        0 => {
            return FOOTER_NORMAL;
        }
        _ => {
            return NUM_FOOTER_TYPES;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetFooterOptionXOffset(option: i32) -> i32 {
    let mut footerIndex: i32 = GetFooterIndex();
    if footerIndex < NUM_FOOTER_TYPES {
        return sFooterOptionXOffsets[footerIndex][option] as i32 + 4;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AddMainScreenButtonWindow() {
    let mut i: i32 = 0;
    let mut windowId: u16 = 0;
    let mut template: WindowTemplate = zeroed();
    let mut footerIndex: i32 = GetFooterIndex();
    if footerIndex == NUM_FOOTER_TYPES {
        return;
    }
    template.bg = 3;
    template.tilemapLeft = 1;
    template.tilemapTop = 11;
    template.width = 28;
    template.height = 2;
    template.paletteNum = 11;
    template.baseBlock = 0x34;
    windowId = AddWindow(&raw mut template);
    FillWindowPixelBuffer(windowId as u8, 17);
    i = 0;
    while i < 4 {
        let mut str: *mut u8 = sFooterTextOptions[footerIndex][i];
        if !str.is_null() {
            let mut x: i32 = sFooterOptionXOffsets[footerIndex][i] as i32;
            PrintEasyChatText(windowId as u8, FONT_NORMAL, str, x as u8, 1, 0, None);
        }
        i += 1;
    }
    PutWindowTilemap(windowId as u8);
}
pub(crate) unsafe extern "C" fn IsEasyChatGroupUnlocked(groupId: u8) -> u8 {
    match groupId {
        EC_GROUP_TRENDY_SAYING => {
            return FlagGet(FLAG_UNLOCKED_TRENDY_SAYINGS);
        }
        EC_GROUP_EVENTS | EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => {
            return FlagGet(FLAG_SYS_GAME_CLEAR);
        }
        EC_GROUP_POKEMON_NATIONAL => {
            return EasyChatIsNationalPokedexEnabled();
        }
        _ => {
            return TRUE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EasyChat_GetNumWordsInGroup(groupId: u8) -> u16 {
    if groupId == EC_GROUP_POKEMON {
        return GetNationalPokedexCount(FLAG_GET_SEEN);
    }
    if IsEasyChatGroupUnlocked(groupId) != 0 {
        return gEasyChatGroups[groupId].numEnabledWords;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn IsEasyChatWordInvalid(easyChatWord: u16) -> u8 {
    let mut i: u16 = 0;
    let mut groupId: u8 = 0;
    let mut index: u32 = 0;
    let mut numWords: u16 = 0;
    let mut list: *mut u16 = null_mut();
    if easyChatWord == EC_EMPTY_WORD {
        return FALSE;
    }
    groupId = (easyChatWord >> 9) as u8;
    index = easyChatWord as u32 & 511;
    if groupId >= EC_NUM_GROUPS {
        return TRUE;
    }
    numWords = gEasyChatGroups[groupId].numWords;
    match groupId {
        EC_GROUP_POKEMON | EC_GROUP_POKEMON_NATIONAL | EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => {
            list = gEasyChatGroups[groupId].wordData.valueList;
            i = 0;
            while i < numWords {
                if index == *list.at(i) as u32 {
                    return FALSE;
                }
                i += 1;
            }
            return TRUE;
        }
        _ => {}
    }
    if index >= numWords as u32 {
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
pub unsafe extern "C" fn IsBardWordInvalid(easyChatWord: u16) -> u8 {
    let mut numWordsInGroup: i32 = 0;
    let mut groupId: u8 = (easyChatWord >> 9) as u8;
    let mut index: u32 = easyChatWord as u32 & 511;
    if groupId >= EC_NUM_GROUPS {
        return TRUE;
    }
    match groupId {
        EC_GROUP_POKEMON | EC_GROUP_POKEMON_NATIONAL => {
            numWordsInGroup = gNumBardWords_Species as i32;
        }
        EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => {
            numWordsInGroup = gNumBardWords_Moves as i32;
        }
        _ => {
            numWordsInGroup = gEasyChatGroups[groupId].numWords as i32;
        }
    }
    if numWordsInGroup as u32 <= index {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetEasyChatWord(groupId: u8, index: u16) -> *mut u8 {
    match groupId {
        EC_GROUP_POKEMON | EC_GROUP_POKEMON_NATIONAL => {
            return gSpeciesNames[index].as_ptr().cast_mut();
        }
        EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => {
            return gMoveNames[index].as_ptr().cast_mut();
        }
        _ => {
            return (*gEasyChatGroups[groupId].wordData.words.at(index)).text;
        }
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyEasyChatWord(dest: *mut u8, easyChatWord: u16) -> *mut u8 {
    let mut resultStr: *mut u8 = null_mut();
    if IsEasyChatWordInvalid(easyChatWord) != 0 {
        resultStr = StringCopy(dest, gText_ThreeQuestionMarks.as_ptr().cast_mut());
    } else if easyChatWord != EC_EMPTY_WORD {
        let mut index: u16 = easyChatWord & 511;
        let mut groupId: u8 = (easyChatWord >> 9) as u8;
        resultStr = StringCopy(dest, GetEasyChatWord(groupId, index));
    } else {
        *dest = EOS;
        resultStr = dest;
    }
    return resultStr;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertEasyChatWordsToString(
    mut dest: *mut u8,
    mut src: *mut u16,
    columns: u16,
    rows: u16,
) -> *mut u8 {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut numColumns: u16 = columns - 1;
    i = 0;
    while i < rows {
        j = 0;
        while j < numColumns {
            dest = CopyEasyChatWord(dest, *src);
            if *src != EC_EMPTY_WORD {
                *dest = CHAR_SPACE;
                dest = dest.at(1);
            }
            src = src.at(1);
            j += 1;
        }
        dest = CopyEasyChatWord(
            dest,
            *({
                let t2 = src;
                src = src.at(1);
                t2
            }),
        );
        *dest = CHAR_NEWLINE;
        dest = dest.at(1);
        i += 1;
    }
    dest = dest.at(-1);
    *dest = EOS;
    return dest;
}
pub(crate) unsafe extern "C" fn UnusedConvertEasyChatWordsToString(
    mut dest: *mut u8,
    mut src: *mut u16,
    mut columns: u16,
    rows: u16,
) -> *mut u8 {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut k: u16 = 0;
    let mut numColumns: u16 = 0;
    let mut notEmpty: i32 = 0;
    let mut lineNumber: i32 = 0;
    numColumns = columns;
    lineNumber = 0;
    columns -= 1;
    i = 0;
    while i < rows {
        'l1: {
            let mut str: *mut u16 = src;
            notEmpty = FALSE as i32;
            j = 0;
            while j < numColumns {
                if *str.at(j) != EC_EMPTY_WORD {
                    notEmpty = TRUE as i32;
                }
                j += 1;
            }
            if notEmpty == 0 {
                src = src.at(numColumns);
                break 'l1;
            }
            k = 0;
            while k < columns {
                dest = CopyEasyChatWord(dest, *src);
                if *src != EC_EMPTY_WORD {
                    *dest = CHAR_SPACE;
                    dest = dest.at(1);
                }
                src = src.at(1);
                k += 1;
            }
            dest = CopyEasyChatWord(
                dest,
                *({
                    let t2 = src;
                    src = src.at(1);
                    t2
                }),
            );
            if lineNumber == 0 {
                *dest = CHAR_NEWLINE;
            } else {
                *dest = CHAR_PROMPT_SCROLL;
            }
            dest = dest.at(1);
            lineNumber += 1;
        }
        i += 1;
    }
    dest = dest.at(-1);
    *dest = EOS;
    return dest;
}
pub(crate) unsafe extern "C" fn GetEasyChatWordStringLength(easyChatWord: u16) -> u16 {
    if easyChatWord == EC_EMPTY_WORD {
        return 0;
    }
    if IsEasyChatWordInvalid(easyChatWord) != 0 {
        return StringLength(gText_ThreeQuestionMarks.as_ptr().cast_mut());
    } else {
        let mut index: u16 = easyChatWord & 511;
        let mut groupId: u8 = (easyChatWord >> 9) as u8;
        return StringLength(GetEasyChatWord(groupId, index));
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CanPhraseFitInXRowsYCols(
    mut easyChatWords: *mut u16,
    numRows: u8,
    numColumns: u8,
    maxLength: u16,
) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < numColumns {
        let mut totalLength: u16 = numRows as u16 - 1;
        j = 0;
        while j < numRows {
            totalLength += GetEasyChatWordStringLength(
                *({
                    let t2 = easyChatWords;
                    easyChatWords = easyChatWords.at(1);
                    t2
                }),
            );
            j += 1;
        }
        if totalLength > maxLength {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomEasyChatWordFromGroup(groupId: u16) -> u16 {
    let mut index: u16 = rem_i32(Random() as i32, gEasyChatGroups[groupId].numWords as i32) as u16;
    if groupId == EC_GROUP_POKEMON as u16
        || groupId == EC_GROUP_POKEMON_NATIONAL as u16
        || groupId == EC_GROUP_MOVE_1 as u16
        || groupId == EC_GROUP_MOVE_2 as u16
    {
        index = *gEasyChatGroups[groupId].wordData.valueList.at(index);
    }
    return (groupId & 127) << 9 | index & 511;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomEasyChatWordFromUnlockedGroup(groupId: u16) -> u16 {
    if IsEasyChatGroupUnlocked(groupId as u8) == 0 {
        return EC_EMPTY_WORD;
    }
    if groupId == EC_GROUP_POKEMON as u16 {
        return GetRandomUnlockedEasyChatPokemon();
    }
    return GetRandomEasyChatWordFromGroup(groupId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowEasyChatProfile() {
    let mut easyChatWords: *mut u16 = null_mut();
    let mut columns: i32 = 0;
    let mut rows: i32 = 0;
    match gSpecialVar_0x8004 {
        0 => {
            easyChatWords = (*gSaveBlock1Ptr).easyChatProfile.as_mut_ptr();
            columns = 2;
            rows = 2;
        }
        1 => {
            easyChatWords = (*gSaveBlock1Ptr).easyChatBattleStart.as_mut_ptr();
            if CanPhraseFitInXRowsYCols(
                (*gSaveBlock1Ptr).easyChatBattleStart.as_mut_ptr(),
                3,
                2,
                18,
            ) != 0
            {
                columns = 2;
                rows = 3;
            } else {
                columns = 3;
                rows = 2;
            }
        }
        2 => {
            easyChatWords = (*gSaveBlock1Ptr).easyChatBattleWon.as_mut_ptr();
            columns = 3;
            rows = 2;
        }
        3 => {
            easyChatWords = (*gSaveBlock1Ptr).easyChatBattleLost.as_mut_ptr();
            columns = 3;
            rows = 2;
        }
        _ => {
            return;
        }
    }
    ConvertEasyChatWordsToString(
        gStringVar4.as_mut_ptr(),
        easyChatWords,
        columns as u16,
        rows as u16,
    );
    ShowFieldAutoScrollMessage(gStringVar4.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferDeepLinkPhrase() {
    let mut groupId: i32 = if Random() as i32 & 1 != 0 {
        EC_GROUP_HOBBIES
    } else {
        EC_GROUP_LIFESTYLE
    };
    let mut easyChatWord: u16 = GetRandomEasyChatWordFromUnlockedGroup(groupId as u16);
    CopyEasyChatWord(gStringVar2.as_mut_ptr(), easyChatWord);
}
pub(crate) unsafe extern "C" fn IsTrendySayingUnlocked(wordIndex: u8) -> u8 {
    let mut byteOffset: i32 = wordIndex as i32 / 8;
    let mut shift: i32 = wordIndex as i32 % 8;
    return shr_i32(
        (*gSaveBlock1Ptr).unlockedTrendySayings[byteOffset] as i32,
        shift as u32,
    ) as u8
        & 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnlockTrendySaying(wordIndex: u8) {
    if wordIndex < NUM_TRENDY_SAYINGS {
        let mut byteOffset: i32 = wordIndex as i32 / 8;
        let mut shift: i32 = wordIndex as i32 % 8;
        (*gSaveBlock1Ptr).unlockedTrendySayings[byteOffset] |= shl_i32(1, shift as u32) as u8;
    }
}
pub(crate) unsafe extern "C" fn GetNumTrendySayingsUnlocked() -> u8 {
    let mut i: u8 = 0;
    let mut numUnlocked: u8 = 0;
    i = 0;
    numUnlocked = 0;
    while i < NUM_TRENDY_SAYINGS {
        if IsTrendySayingUnlocked(i) != 0 {
            numUnlocked += 1;
        }
        i += 1;
    }
    return numUnlocked;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnlockRandomTrendySaying() -> u16 {
    let mut i: u16 = 0;
    let mut numToSkip: u16 = 0;
    let mut numUnlocked: u8 = GetNumTrendySayingsUnlocked();
    if numUnlocked == NUM_TRENDY_SAYINGS {
        return EC_EMPTY_WORD;
    }
    numToSkip = rem_i32(
        Random() as i32,
        NUM_TRENDY_SAYINGS as i32 - numUnlocked as i32,
    ) as u16;
    i = 0;
    while i < NUM_TRENDY_SAYINGS as u16 {
        if IsTrendySayingUnlocked(i as u8) == 0 {
            if numToSkip != 0 {
                numToSkip -= 1;
            } else {
                UnlockTrendySaying(i as u8);
                return 10240 | i & 511;
            }
        }
        i += 1;
    }
    return EC_EMPTY_WORD;
}
pub(crate) unsafe extern "C" fn GetRandomUnlockedTrendySaying() -> u16 {
    let mut i: u16 = 0;
    let mut n: u16 = GetNumTrendySayingsUnlocked() as u16;
    if n == 0 {
        return EC_EMPTY_WORD;
    }
    n = rem_i32(Random() as i32, n as i32) as u16;
    i = 0;
    while i < NUM_TRENDY_SAYINGS as u16 {
        if IsTrendySayingUnlocked(i as u8) != 0 {
            if n != 0 {
                n -= 1;
            } else {
                return 10240 | i & 511;
            }
        }
        i += 1;
    }
    return EC_EMPTY_WORD;
}
pub(crate) unsafe extern "C" fn EasyChatIsNationalPokedexEnabled() -> u8 {
    return IsNationalPokedexEnabled() as u8;
}
pub(crate) unsafe extern "C" fn GetRandomUnlockedEasyChatPokemon() -> u16 {
    let mut i: u16 = 0;
    let mut numWords: u16 = 0;
    let mut species: *mut u16 = null_mut();
    let mut index: u16 = EasyChat_GetNumWordsInGroup(EC_GROUP_POKEMON);
    if index == 0 {
        return EC_EMPTY_WORD;
    }
    index = rem_i32(Random() as i32, index as i32) as u16;
    species = gEasyChatGroups[0].wordData.valueList;
    numWords = gEasyChatGroups[0].numWords;
    i = 0;
    while i < numWords {
        let mut dexNum: u16 = SpeciesToNationalPokedexNum(*species);
        if GetSetPokedexFlag(dexNum, FLAG_GET_SEEN) != 0 {
            if index != 0 {
                index -= 1;
            } else {
                return EC_GROUP_POKEMON as u16 | *species & 511;
            }
        }
        species = species.at(1);
        i += 1;
    }
    return EC_EMPTY_WORD;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitEasyChatPhrases() {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while i < 4 {
        (*gSaveBlock1Ptr).easyChatProfile[i] = sDefaultProfileWords[i];
        i += 1;
    }
    i = 0;
    while i < EASY_CHAT_BATTLE_WORDS_COUNT as u16 {
        (*gSaveBlock1Ptr).easyChatBattleStart[i] = sDefaultBattleStartWords[i];
        i += 1;
    }
    i = 0;
    while i < EASY_CHAT_BATTLE_WORDS_COUNT as u16 {
        (*gSaveBlock1Ptr).easyChatBattleWon[i] = sDefaultBattleWonWords[i];
        i += 1;
    }
    i = 0;
    while i < EASY_CHAT_BATTLE_WORDS_COUNT as u16 {
        (*gSaveBlock1Ptr).easyChatBattleLost[i] = sDefaultBattleLostWords[i];
        i += 1;
    }
    i = 0;
    while i < MAIL_COUNT as u16 {
        j = 0;
        while j < MAIL_WORDS_COUNT {
            (*gSaveBlock1Ptr).mail[i].words[j] = EC_EMPTY_WORD;
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < 5 {
        (*gSaveBlock1Ptr).unlockedTrendySayings[i] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitEasyChatScreenWordData() -> u8 {
    sWordData = Alloc(15268) as *mut EasyChatScreenWordData;
    if sWordData.is_null() {
        return FALSE;
    }
    SetUnlockedEasyChatGroups();
    SetUnlockedWordsByAlphabet();
    return TRUE;
}
pub(crate) unsafe extern "C" fn FreeEasyChatScreenWordData() {
    if !sWordData.is_null() {
        Free(sWordData as *mut c_void);
        sWordData = null_mut();
    }
}
pub(crate) unsafe extern "C" fn SetUnlockedEasyChatGroups() {
    let mut i: i32 = 0;
    (*sWordData).numUnlockedGroups = 0;
    if GetNationalPokedexCount(FLAG_GET_SEEN) != 0 {
        (*sWordData).unlockedGroupIds[{
            let t1 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t1
        }] = EC_GROUP_POKEMON as u16;
    }
    i = EC_GROUP_TRAINER;
    while i <= EC_GROUP_ADJECTIVES {
        (*sWordData).unlockedGroupIds[{
            let t2 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t2
        }] = i as u16;
        i += 1;
    }
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        (*sWordData).unlockedGroupIds[{
            let t3 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t3
        }] = EC_GROUP_EVENTS as u16;
        (*sWordData).unlockedGroupIds[{
            let t4 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t4
        }] = EC_GROUP_MOVE_1 as u16;
        (*sWordData).unlockedGroupIds[{
            let t5 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t5
        }] = EC_GROUP_MOVE_2 as u16;
    }
    if FlagGet(FLAG_UNLOCKED_TRENDY_SAYINGS) != 0 {
        (*sWordData).unlockedGroupIds[{
            let t6 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t6
        }] = EC_GROUP_TRENDY_SAYING as u16;
    }
    if IsNationalPokedexEnabled() != 0 {
        (*sWordData).unlockedGroupIds[{
            let t7 = (*sWordData).numUnlockedGroups;
            (*sWordData).numUnlockedGroups += 1;
            t7
        }] = EC_GROUP_POKEMON_NATIONAL as u16;
    }
}
pub(crate) unsafe extern "C" fn GetNumUnlockedEasyChatGroups() -> u8 {
    return (*sWordData).numUnlockedGroups as u8;
}
pub(crate) unsafe extern "C" fn GetUnlockedEasyChatGroupId(index: u8) -> u8 {
    if index as u16 >= (*sWordData).numUnlockedGroups {
        return EC_NUM_GROUPS;
    } else {
        return (*sWordData).unlockedGroupIds[index] as u8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn BufferEasyChatWordGroupName(
    dest: *mut u8,
    groupId: u8,
    totalChars: u16,
) -> *mut u8 {
    let mut i: u16 = 0;
    let mut str: *mut u8 = StringCopy(dest, sEasyChatGroupNamePointers[groupId]);
    i = (str as usize).wrapping_sub(dest as usize) as i32 as u16;
    while i < totalChars {
        *str = CHAR_SPACE;
        str = str.at(1);
        i += 1;
    }
    *str = EOS;
    return str;
}
pub(crate) unsafe extern "C" fn GetEasyChatWordGroupName(groupId: u8) -> *mut u8 {
    return sEasyChatGroupNamePointers[groupId];
}
pub(crate) unsafe extern "C" fn CopyEasyChatWordPadded(
    dest: *mut u8,
    easyChatWord: u16,
    totalChars: u16,
) -> *mut u8 {
    let mut i: u16 = 0;
    let mut str: *mut u8 = CopyEasyChatWord(dest, easyChatWord);
    i = (str as usize).wrapping_sub(dest as usize) as i32 as u16;
    while i < totalChars {
        *str = CHAR_SPACE;
        str = str.at(1);
        i += 1;
    }
    *str = EOS;
    return str;
}
pub(crate) unsafe extern "C" fn SetUnlockedWordsByAlphabet() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut numWords: i32 = 0;
    let mut words: *mut u16 = null_mut();
    let mut numToProcess: u16 = 0;
    let mut index: i32 = 0;
    i = 0;
    while i < EC_NUM_ALPHABET_GROUPS {
        numWords = gEasyChatWordsByLetterPointers[i].numWords;
        words = gEasyChatWordsByLetterPointers[i].words;
        (*sWordData).numUnlockedAlphabetWords[i] = 0;
        index = 0;
        j = 0;
        while j < numWords {
            if *words == EC_EMPTY_WORD {
                words = words.at(1);
                numToProcess = *words;
                words = words.at(1);
                j += 1 + numToProcess as i32;
            } else {
                numToProcess = 1;
            }
            k = 0;
            while k < numToProcess as i32 {
                if IsEasyChatWordUnlocked(*words.at(k)) != 0 {
                    (*sWordData).unlockedAlphabetWords[i][{
                        let t1 = index;
                        index += 1;
                        t1
                    }] = *words.at(k);
                    (*sWordData).numUnlockedAlphabetWords[i] += 1;
                    break;
                }
                k += 1;
            }
            words = words.at(numToProcess);
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWordGroup(inAlphabetMode: u32, groupId: u16) {
    if inAlphabetMode == 0 {
        (*sWordData).numSelectedGroupWords = SetSelectedWordGroup_GroupMode(groupId);
    } else {
        (*sWordData).numSelectedGroupWords = SetSelectedWordGroup_AlphabetMode(groupId);
    }
}
pub(crate) unsafe extern "C" fn GetWordFromSelectedGroup(index: u16) -> u16 {
    if index >= (*sWordData).numSelectedGroupWords {
        return EC_EMPTY_WORD;
    } else {
        return (*sWordData).selectedGroupWords[index];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetNumWordsInSelectedGroup() -> u16 {
    return (*sWordData).numSelectedGroupWords;
}
pub(crate) unsafe extern "C" fn SetSelectedWordGroup_GroupMode(groupId: u16) -> u16 {
    let mut i: u32 = 0;
    let mut totalWords: i32 = 0;
    let mut list: *mut u16 = null_mut();
    let mut wordInfo: *mut EasyChatWordInfo = null_mut();
    let mut numWords: u16 = gEasyChatGroups[groupId].numWords;
    if groupId == EC_GROUP_POKEMON as u16
        || groupId == EC_GROUP_POKEMON_NATIONAL as u16
        || groupId == EC_GROUP_MOVE_1 as u16
        || groupId == EC_GROUP_MOVE_2 as u16
    {
        list = gEasyChatGroups[groupId].wordData.valueList;
        i = 0;
        totalWords = 0;
        while i < numWords as u32 {
            if IsEasyChatIndexAndGroupUnlocked(*list.at(i), groupId as u8) != 0 {
                (*sWordData).selectedGroupWords[{
                    let t1 = totalWords;
                    totalWords += 1;
                    t1
                }] = (groupId & 127) << 9 | *list.at(i) & 511;
            }
            i += 1;
        }
        return totalWords as u16;
    } else {
        wordInfo = gEasyChatGroups[groupId].wordData.words;
        i = 0;
        totalWords = 0;
        while i < numWords as u32 {
            let mut alphabeticalOrder: u16 = (*wordInfo.at(i)).alphabeticalOrder as u16;
            if IsEasyChatIndexAndGroupUnlocked(alphabeticalOrder, groupId as u8) != 0 {
                (*sWordData).selectedGroupWords[{
                    let t2 = totalWords;
                    totalWords += 1;
                    t2
                }] = (groupId & 127) << 9 | alphabeticalOrder & 511;
            }
            i += 1;
        }
        return totalWords as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetSelectedWordGroup_AlphabetMode(groupId: u16) -> u16 {
    let mut i: u16 = 0;
    let mut totalWords: u16 = 0;
    i = 0;
    totalWords = 0;
    while i < (*sWordData).numUnlockedAlphabetWords[groupId] {
        (*sWordData).selectedGroupWords[{
            let t1 = totalWords;
            totalWords += 1;
            t1
        }] = (*sWordData).unlockedAlphabetWords[groupId][i];
        i += 1;
    }
    return totalWords;
}
pub(crate) unsafe extern "C" fn IsEasyChatGroupUnlocked2(groupId: u8) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sWordData).numUnlockedGroups as i32 {
        if (*sWordData).unlockedGroupIds[i] == groupId as u16 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsEasyChatIndexAndGroupUnlocked(wordIndex: u16, groupId: u8) -> u8 {
    match groupId {
        EC_GROUP_POKEMON => {
            return GetSetPokedexFlag(SpeciesToNationalPokedexNum(wordIndex), FLAG_GET_SEEN) as u8;
        }
        EC_GROUP_POKEMON_NATIONAL => {
            if IsRestrictedWordSpecies(wordIndex) != 0 {
                GetSetPokedexFlag(SpeciesToNationalPokedexNum(wordIndex), FLAG_GET_SEEN);
            }
            return TRUE;
        }
        EC_GROUP_MOVE_1 | EC_GROUP_MOVE_2 => {
            return TRUE;
        }
        EC_GROUP_TRENDY_SAYING => {
            return IsTrendySayingUnlocked(wordIndex as u8);
        }
        _ => {
            return (*gEasyChatGroups[groupId].wordData.words.at(wordIndex)).enabled as u8;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsRestrictedWordSpecies(species: u16) -> i32 {
    let mut i: u32 = 0;
    i = 0;
    while i < 1 {
        if sRestrictedWordSpecies[i] == species {
            return TRUE as i32;
        }
        i += 1;
    }
    return FALSE as i32;
}
pub(crate) unsafe extern "C" fn IsEasyChatWordUnlocked(easyChatWord: u16) -> u8 {
    let mut groupId: u8 = (easyChatWord >> 9) as u8;
    let mut index: u32 = easyChatWord as u32 & 511;
    if IsEasyChatGroupUnlocked2(groupId) == 0 {
        return FALSE;
    } else {
        return IsEasyChatIndexAndGroupUnlocked(index as u16, groupId);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitializeEasyChatWordArray(mut words: *mut u16, length: u16) {
    let mut i: u16 = 0;
    i = length - 1;
    while i != EC_EMPTY_WORD {
        *({
            let t1 = words;
            words = words.at(1);
            t1
        }) = EC_EMPTY_WORD;
        i -= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitQuestionnaireWords() {
    let mut i: i32 = 0;
    let mut words: *mut u16 = GetQuestionnaireWordsPtr();
    i = 0;
    while i < NUM_QUESTIONNAIRE_WORDS {
        *words.at(i) = EC_EMPTY_WORD;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEasyChatAnswerUnlocked(easyChatWord: i32) -> u32 {
    let mut groupId: i32 = easyChatWord >> 9;
    let mut mask: i32 = EC_MASK_GROUP;
    let mut index: i32 = easyChatWord & 511;
    if IsEasyChatGroupUnlocked(groupId as u8 & mask as u8) == 0 {
        return FALSE as u32;
    } else {
        return IsEasyChatIndexAndGroupUnlocked(index as u16, groupId as u8 & mask as u8) as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
