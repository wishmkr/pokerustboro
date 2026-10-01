//! Translated from `src/pokenav_conditions.c` by tools/rustport/c2rs.py.
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

/// `struct Pokenav_ConditionMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_ConditionMenu {
    pub monPal: CArray<CArray<u32, 32>, 3>,
    pub fill: CArray<u8, 384>,
    pub monPicGfx: CArray<CArray<u32, 2048>, 3>,
    pub inSearchMode: u8,
    pub toLoadListIndex: i16,
    pub callback: Option<unsafe extern "C" fn(*mut Pokenav_ConditionMenu) -> u32>,
    pub fill2: CArray<u8, 24>,
    pub locationText: CArray<CArray<u8, 24>, 3>,
    pub nameText: CArray<CArray<u8, 64>, 3>,
    pub graph: ConditionGraph,
    pub numSparkles: CArray<u8, 3>,
    pub monMarks: CArray<u8, 3>,
    pub loadId: i8,
    pub nextLoadIdDown: i8,
    pub nextLoadIdUp: i8,
    pub toLoadId: i8,
    pub state: u8,
}

unsafe impl Sync for Pokenav_ConditionMenu {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_ConditionMenu>() == 26508);
    assert!(offset_of!(Pokenav_ConditionMenu, monPal) == 0);
    assert!(offset_of!(Pokenav_ConditionMenu, fill) == 384);
    assert!(offset_of!(Pokenav_ConditionMenu, monPicGfx) == 768);
    assert!(offset_of!(Pokenav_ConditionMenu, inSearchMode) == 25344);
    assert!(offset_of!(Pokenav_ConditionMenu, toLoadListIndex) == 25346);
    assert!(offset_of!(Pokenav_ConditionMenu, callback) == 25348);
    assert!(offset_of!(Pokenav_ConditionMenu, fill2) == 25352);
    assert!(offset_of!(Pokenav_ConditionMenu, locationText) == 25376);
    assert!(offset_of!(Pokenav_ConditionMenu, nameText) == 25448);
    assert!(offset_of!(Pokenav_ConditionMenu, graph) == 25640);
    assert!(offset_of!(Pokenav_ConditionMenu, numSparkles) == 26496);
    assert!(offset_of!(Pokenav_ConditionMenu, monMarks) == 26499);
    assert!(offset_of!(Pokenav_ConditionMenu, loadId) == 26502);
    assert!(offset_of!(Pokenav_ConditionMenu, nextLoadIdDown) == 26503);
    assert!(offset_of!(Pokenav_ConditionMenu, nextLoadIdUp) == 26504);
    assert!(offset_of!(Pokenav_ConditionMenu, toLoadId) == 26505);
    assert!(offset_of!(Pokenav_ConditionMenu, state) == 26506);
};

unsafe extern "C" {
    static mut gKeyRepeatStartDelay: u16;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static gText_EggNickname: CArray<u8, 0>;
    static gText_InParty: CArray<u8, 0>;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn CalculatePlayerPartyCount() -> u8;
    fn ConditionGraph_CalcPositions(a0: *mut u8, a1: *mut UCoords16);
    fn ConditionGraph_Init(a0: *mut ConditionGraph);
    fn ConditionGraph_SetNewPositions(
        a0: *mut ConditionGraph,
        a1: *mut UCoords16,
        a2: *mut UCoords16,
    );
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn FreePokenavSubstruct(a0: u32);
    fn GetBoxMonGender(a0: *mut BoxPokemon) -> u8;
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetBoxOrPartyMonData(a0: u16, a1: u16, a2: i32, a3: *mut u8) -> i32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
    fn GetLevelFromBoxMonExp(a0: *mut BoxPokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonMarkingsData() -> u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn HandleMonMarkingsMenuInput() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadSpecialPokePic(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
        a4: u8,
    );
    fn PlaySE(a0: u16);
    fn SetBoxMonDataAt(a0: u8, a1: u8, a2: i32, a3: *mut c_void);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionGraph_Party() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU, 26508) as *mut Pokenav_ConditionMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    ConditionGraph_Init(&raw mut (*menu).graph);
    InitPartyConditionListParameters();
    gKeyRepeatStartDelay = 20;
    (*menu).callback = Some(HandleConditionMenuInput);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionGraph_Search() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU, 26508) as *mut Pokenav_ConditionMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    ConditionGraph_Init(&raw mut (*menu).graph);
    InitSearchResultsConditionList();
    gKeyRepeatStartDelay = 20;
    (*menu).callback = Some(HandleConditionMenuInput);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuCallback() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).callback.unwrap_unchecked()(menu);
}
pub(crate) unsafe extern "C" fn HandleConditionMenuInput(menu: *mut Pokenav_ConditionMenu) -> u32 {
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    let mut ret: u32 = ConditionGraphHandleDpadInput(menu) as u32;
    if ret == CONDITION_FUNC_NONE {
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            (*menu).callback = Some(GetConditionReturnCallback);
            ret = CONDITION_FUNC_RETURN;
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            if (*menu).inSearchMode == 0 {
                if (*monListPtr).currIndex as i32 == (*monListPtr).listCount as i32 - 1 {
                    PlaySE(SE_SELECT);
                    (*menu).callback = Some(GetConditionReturnCallback);
                    ret = CONDITION_FUNC_RETURN;
                }
            } else {
                PlaySE(SE_SELECT);
                ret = CONDITION_FUNC_ADD_MARKINGS;
                (*menu).callback = Some(OpenMarkingsMenu);
            }
        }
    }
    return ret;
}
pub(crate) unsafe extern "C" fn OpenMarkingsMenu(menu: *mut Pokenav_ConditionMenu) -> u32 {
    let mut monListPtr: *mut PokenavMonList = null_mut();
    let mut markings: u8 = 0;
    let mut ret: u32 = CONDITION_FUNC_NONE;
    let mut boxId: u32 = 0;
    let mut monId: u32 = 0;
    if HandleMonMarkingsMenuInput() == 0 {
        (*menu).monMarks[(*menu).loadId] = GetMonMarkingsData();
        monListPtr = GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
        boxId = (*monListPtr).monData[(*monListPtr).currIndex].boxId as u32;
        monId = (*monListPtr).monData[(*monListPtr).currIndex].monId as u32;
        markings = (*menu).monMarks[(*menu).loadId];
        if boxId == TOTAL_BOXES_COUNT as u32 {
            SetMonData(
                &raw mut gPlayerParty[monId],
                MON_DATA_MARKINGS,
                &raw mut markings as *mut c_void,
            );
        } else {
            SetBoxMonDataAt(
                boxId as u8,
                monId as u8,
                MON_DATA_MARKINGS,
                &raw mut markings as *mut c_void,
            );
        }
        (*menu).callback = Some(HandleConditionMenuInput);
        ret = CONDITION_FUNC_CLOSE_MARKINGS;
    }
    return ret;
}
pub(crate) unsafe extern "C" fn GetConditionReturnCallback(
    menu: *mut Pokenav_ConditionMenu,
) -> u32 {
    if (*menu).inSearchMode == 0 {
        return POKENAV_CONDITION_MENU;
    } else {
        return POKENAV_RETURN_CONDITION_SEARCH;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeConditionGraphMenuSubstruct1() {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == 0 {
        FreePokenavSubstruct(POKENAV_SUBSTRUCT_MON_LIST);
    }
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU);
}
pub(crate) unsafe extern "C" fn ConditionGraphHandleDpadInput(
    menu: *mut Pokenav_ConditionMenu,
) -> u8 {
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    let mut ret: u8 = CONDITION_FUNC_NONE as u8;
    if gMain.heldKeys as i32 & DPAD_UP != 0 {
        if (*menu).inSearchMode == 0 || (*monListPtr).currIndex != 0 {
            PlaySE(SE_SELECT);
            ret = SwitchConditionSummaryIndex(TRUE);
        }
    } else if gMain.heldKeys as i32 & DPAD_DOWN != 0 {
        if (*menu).inSearchMode == 0
            || ((*monListPtr).currIndex as i32) < (*monListPtr).listCount as i32 - 1
        {
            PlaySE(SE_SELECT);
            ret = SwitchConditionSummaryIndex(FALSE);
        }
    }
    return ret;
}
pub(crate) unsafe extern "C" fn SwitchConditionSummaryIndex(moveUp: u8) -> u8 {
    let mut newLoadId: u16 = 0;
    let mut wasNotLastMon: u8 = 0;
    let mut isNotLastMon: u8 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    newLoadId = (if moveUp != 0 {
        (*menu).nextLoadIdUp
    } else {
        (*menu).nextLoadIdDown
    }) as u16;
    ConditionGraph_SetNewPositions(
        &raw mut (*menu).graph,
        (*menu).graph.savedPositions[(*menu).loadId].as_mut_ptr(),
        (*menu).graph.savedPositions[newLoadId].as_mut_ptr(),
    );
    wasNotLastMon = ((*monListPtr).currIndex as i32
        != (if IsConditionMenuSearchMode() != 0 {
            (*monListPtr).listCount as i32
        } else {
            (*monListPtr).listCount as i32 - 1
        })) as u8;
    if moveUp != 0 {
        (*menu).nextLoadIdUp = (*menu).nextLoadIdDown;
        (*menu).nextLoadIdDown = (*menu).loadId;
        (*menu).loadId = newLoadId as i8;
        (*menu).toLoadId = (*menu).nextLoadIdUp;
        (*monListPtr).currIndex = (if (*monListPtr).currIndex == 0 {
            (*monListPtr).listCount as i32 - 1
        } else {
            (*monListPtr).currIndex as i32 - 1
        }) as u16;
        (*menu).toLoadListIndex = (if (*monListPtr).currIndex != 0 {
            (*monListPtr).currIndex as i32 - 1
        } else {
            (*monListPtr).listCount as i32 - 1
        }) as i16;
    } else {
        (*menu).nextLoadIdDown = (*menu).nextLoadIdUp;
        (*menu).nextLoadIdUp = (*menu).loadId;
        (*menu).loadId = newLoadId as i8;
        (*menu).toLoadId = (*menu).nextLoadIdDown;
        (*monListPtr).currIndex =
            (if ((*monListPtr).currIndex as i32) < (*monListPtr).listCount as i32 - 1 {
                (*monListPtr).currIndex as i32 + 1
            } else {
                0
            }) as u16;
        (*menu).toLoadListIndex =
            (if ((*monListPtr).currIndex as i32) < (*monListPtr).listCount as i32 - 1 {
                (*monListPtr).currIndex as i32 + 1
            } else {
                0
            }) as i16;
    }
    isNotLastMon = ((*monListPtr).currIndex as i32
        != (if IsConditionMenuSearchMode() != 0 {
            (*monListPtr).listCount as i32
        } else {
            (*monListPtr).listCount as i32 - 1
        })) as u8;
    if wasNotLastMon == 0 {
        return CONDITION_FUNC_NO_TRANSITION;
    } else if isNotLastMon == 0 {
        return CONDITION_FUNC_SLIDE_MON_OUT;
    } else {
        return CONDITION_FUNC_SLIDE_MON_IN;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadConditionGraphMenuGfx() -> u32 {
    let mut var: i32 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    match (*menu).state {
        0 => {
            CopyMonNameGenderLocation((*monListPtr).currIndex as i16, 0);
        }
        1 => {
            GetMonConditionGraphData((*monListPtr).currIndex as i16, 0);
        }
        2 => {
            ConditionGraphDrawMonPic((*monListPtr).currIndex as i16, 0);
        }
        3 => {
            if (*monListPtr).listCount == 1 {
                (*menu).loadId = 0;
                (*menu).nextLoadIdDown = 0;
                (*menu).nextLoadIdUp = 0;
                (*menu).state = 0;
                return TRUE as u32;
            } else {
                (*menu).loadId = 0;
                (*menu).nextLoadIdDown = 1;
                (*menu).nextLoadIdUp = 2;
            }
        }
        4 => {
            var = (*monListPtr).currIndex as i32 + 1;
            if var >= (*monListPtr).listCount as i32 {
                var = 0;
            }
            CopyMonNameGenderLocation(var as i16, 1);
        }
        5 => {
            var = (*monListPtr).currIndex as i32 + 1;
            if var >= (*monListPtr).listCount as i32 {
                var = 0;
            }
            GetMonConditionGraphData(var as i16, 1);
        }
        6 => {
            var = (*monListPtr).currIndex as i32 + 1;
            if var >= (*monListPtr).listCount as i32 {
                var = 0;
            }
            ConditionGraphDrawMonPic(var as i16, 1);
        }
        7 => {
            CopyMonNameGenderLocation(
                (if (*monListPtr).currIndex as i32 - 1 >= 0 {
                    (*monListPtr).currIndex as i32 - 1
                } else {
                    (*monListPtr).listCount as i32 - 1
                }) as i16,
                2,
            );
        }
        8 => {
            GetMonConditionGraphData(
                (if (*monListPtr).currIndex as i32 - 1 >= 0 {
                    (*monListPtr).currIndex as i32 - 1
                } else {
                    (*monListPtr).listCount as i32 - 1
                }) as i16,
                2,
            );
        }
        9 => {
            ConditionGraphDrawMonPic(
                (if (*monListPtr).currIndex as i32 - 1 >= 0 {
                    (*monListPtr).currIndex as i32 - 1
                } else {
                    (*monListPtr).listCount as i32 - 1
                }) as i16,
                2,
            );
            (*menu).state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    (*menu).state += 1;
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadNextConditionMenuMonData(mode: u8) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    match mode {
        CONDITION_LOAD_MON_INFO => {
            CopyMonNameGenderLocation((*menu).toLoadListIndex, (*menu).toLoadId as u8);
        }
        CONDITION_LOAD_GRAPH => {
            GetMonConditionGraphData((*menu).toLoadListIndex, (*menu).toLoadId as u8);
        }
        CONDITION_LOAD_MON_PIC => {
            ConditionGraphDrawMonPic((*menu).toLoadListIndex, (*menu).toLoadId as u8);
            return TRUE as u32;
        }
        _ => {}
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyStringLeftAlignedToConditionData(
    mut dst: *mut u8,
    mut src: *mut u8,
    mut n: i16,
) -> *mut u8 {
    while *src != EOS {
        *({
            let t1 = dst;
            dst = dst.at(1);
            t1
        }) = *({
            let t3 = src;
            src = src.at(1);
            t3
        });
        n -= 1;
    }
    while ({
        let t4 = n;
        n -= 1;
        t4
    }) > 0
    {
        *({
            let t5 = dst;
            dst = dst.at(1);
            t5
        }) = CHAR_SPACE;
    }
    *dst = EOS;
    return dst;
}
pub(crate) unsafe extern "C" fn CopyConditionMonNameGender(
    mut str: *mut u8,
    listId: u16,
    skipPadding: u8,
) -> *mut u8 {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let mut gender: u16 = 0;
    let mut species: u16 = 0;
    let mut level: u16 = 0;
    let mut lvlDigits: u16 = 0;
    let mut boxMon: *mut BoxPokemon = null_mut();
    let mut txtPtr: *mut u8 = null_mut();
    let mut str_: *mut u8 = null_mut();
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    boxId = (*monListPtr).monData[listId].boxId as u16;
    monId = (*monListPtr).monData[listId].monId as u16;
    *({
        let t1 = str;
        str = str.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = str;
        str = str.at(1);
        t2
    }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
    *({
        let t3 = str;
        str = str.at(1);
        t3
    }) = TEXT_COLOR_BLUE;
    *({
        let t4 = str;
        str = str.at(1);
        t4
    }) = TEXT_COLOR_TRANSPARENT;
    *({
        let t5 = str;
        str = str.at(1);
        t5
    }) = TEXT_COLOR_LIGHT_BLUE;
    if GetBoxOrPartyMonData(boxId, monId, MON_DATA_IS_EGG, null_mut()) != 0 {
        return StringCopyPadded(str, gText_EggNickname.as_ptr().cast_mut(), CHAR_SPACE, 12);
    }
    GetBoxOrPartyMonData(boxId, monId, MON_DATA_NICKNAME, str);
    StringGet_Nickname(str);
    species = GetBoxOrPartyMonData(boxId, monId, MON_DATA_SPECIES, null_mut()) as u16;
    if boxId == TOTAL_BOXES_COUNT as u16 {
        level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u16;
        gender = GetMonGender(&raw mut gPlayerParty[monId]) as u16;
    } else {
        boxMon = GetBoxedMonPtr(boxId as u8, monId as u8);
        gender = GetBoxMonGender(boxMon) as u16;
        level = GetLevelFromBoxMonExp(boxMon) as u16;
    }
    if (species == SPECIES_NIDORAN_F || species == SPECIES_NIDORAN_M)
        && StringCompare(str, gSpeciesNames[species].as_ptr().cast_mut()) == 0
    {
        gender = MON_GENDERLESS as u16;
    }
    str_ = str;
    while *str_ != EOS {
        str_ = str_.at(1);
    }
    *({
        let t6 = str_;
        str_ = str_.at(1);
        t6
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t7 = str_;
        str_ = str_.at(1);
        t7
    }) = EXT_CTRL_CODE_SKIP_TO;
    *({
        let t8 = str_;
        str_ = str_.at(1);
        t8
    }) = 60;
    match gender {
        0 => {
            *({
                let t10 = str_;
                str_ = str_.at(1);
                t10
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t11 = str_;
                str_ = str_.at(1);
                t11
            }) = EXT_CTRL_CODE_COLOR;
            *({
                let t12 = str_;
                str_ = str_.at(1);
                t12
            }) = TEXT_COLOR_RED;
            *({
                let t13 = str_;
                str_ = str_.at(1);
                t13
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t14 = str_;
                str_ = str_.at(1);
                t14
            }) = EXT_CTRL_CODE_SHADOW;
            *({
                let t15 = str_;
                str_ = str_.at(1);
                t15
            }) = TEXT_COLOR_LIGHT_RED;
            *({
                let t16 = str_;
                str_ = str_.at(1);
                t16
            }) = CHAR_MALE;
        }
        254 => {
            *({
                let t17 = str_;
                str_ = str_.at(1);
                t17
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t18 = str_;
                str_ = str_.at(1);
                t18
            }) = EXT_CTRL_CODE_COLOR;
            *({
                let t19 = str_;
                str_ = str_.at(1);
                t19
            }) = TEXT_COLOR_GREEN;
            *({
                let t20 = str_;
                str_ = str_.at(1);
                t20
            }) = EXT_CTRL_CODE_BEGIN;
            *({
                let t21 = str_;
                str_ = str_.at(1);
                t21
            }) = EXT_CTRL_CODE_SHADOW;
            *({
                let t22 = str_;
                str_ = str_.at(1);
                t22
            }) = TEXT_COLOR_LIGHT_GREEN;
            *({
                let t23 = str_;
                str_ = str_.at(1);
                t23
            }) = CHAR_FEMALE;
        }
        _ => {
            *({
                let t9 = str_;
                str_ = str_.at(1);
                t9
            }) = CHAR_SPACER;
        }
    }
    *({
        let t24 = str_;
        str_ = str_.at(1);
        t24
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t25 = str_;
        str_ = str_.at(1);
        t25
    }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
    *({
        let t26 = str_;
        str_ = str_.at(1);
        t26
    }) = TEXT_COLOR_BLUE;
    *({
        let t27 = str_;
        str_ = str_.at(1);
        t27
    }) = TEXT_COLOR_TRANSPARENT;
    *({
        let t28 = str_;
        str_ = str_.at(1);
        t28
    }) = TEXT_COLOR_LIGHT_BLUE;
    *({
        let t29 = str_;
        str_ = str_.at(1);
        t29
    }) = CHAR_SLASH;
    *({
        let t30 = str_;
        str_ = str_.at(1);
        t30
    }) = CHAR_EXTRA_SYMBOL;
    *({
        let t31 = str_;
        str_ = str_.at(1);
        t31
    }) = CHAR_LV_2;
    txtPtr = str_;
    str_ = ConvertIntToDecimalStringN(str_, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    lvlDigits = (str_ as usize).wrapping_sub(txtPtr as usize) as i32 as u16;
    *({
        let t32 = str_;
        str_ = str_.at(1);
        t32
    }) = CHAR_SPACE;
    if skipPadding == 0 {
        lvlDigits = 3 - lvlDigits;
        while ({
            let t33 = lvlDigits;
            lvlDigits -= 1;
            t33
        }) != 0
        {
            *({
                let t34 = str_;
                str_ = str_.at(1);
                t34
            }) = CHAR_SPACE;
        }
    }
    *str_ = EOS;
    return str_;
}
pub(crate) unsafe extern "C" fn CopyMonNameGenderLocation(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut i: u16 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    if listId as i32
        != (if IsConditionMenuSearchMode() != 0 {
            (*monListPtr).listCount as i32
        } else {
            (*monListPtr).listCount as i32 - 1
        })
    {
        CopyConditionMonNameGender((*menu).nameText[loadId].as_mut_ptr(), listId as u16, FALSE);
        boxId = (*monListPtr).monData[listId].boxId as u16;
        (*menu).locationText[loadId][0] = EXT_CTRL_CODE_BEGIN;
        (*menu).locationText[loadId][1] = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
        (*menu).locationText[loadId][2] = TEXT_COLOR_BLUE;
        (*menu).locationText[loadId][3] = TEXT_COLOR_TRANSPARENT;
        (*menu).locationText[loadId][4] = TEXT_COLOR_LIGHT_BLUE;
        if boxId == TOTAL_BOXES_COUNT as u16 {
            CopyStringLeftAlignedToConditionData(
                &raw mut (*menu).locationText[loadId][5],
                gText_InParty.as_ptr().cast_mut(),
                BOX_NAME_LENGTH,
            );
        } else {
            CopyStringLeftAlignedToConditionData(
                &raw mut (*menu).locationText[loadId][5],
                GetBoxNamePtr(boxId as u8),
                BOX_NAME_LENGTH,
            );
        }
    } else {
        i = 0;
        while i < 12 {
            (*menu).nameText[loadId][i] = CHAR_SPACE;
            i += 1;
        }
        (*menu).nameText[loadId][i] = EOS;
        i = 0;
        while i < BOX_NAME_LENGTH as u16 {
            (*menu).locationText[loadId][i] = CHAR_SPACE;
            i += 1;
        }
        (*menu).locationText[loadId][i] = EOS;
    }
}
pub(crate) unsafe extern "C" fn InitPartyConditionListParameters() {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        AllocSubstruct(POKENAV_SUBSTRUCT_MON_LIST, 1708) as *mut PokenavMonList;
    (*menu).inSearchMode = FALSE;
    i = 0;
    count = 0;
    while i < CalculatePlayerPartyCount() as u16 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0 {
            (*monListPtr).monData[count].boxId = TOTAL_BOXES_COUNT;
            (*monListPtr).monData[count].monId = i as u8;
            (*monListPtr).monData[count].data = 0;
            count += 1;
        }
        i += 1;
    }
    (*monListPtr).monData[count].boxId = 0;
    (*monListPtr).monData[count].monId = 0;
    (*monListPtr).monData[count].data = 0;
    (*monListPtr).currIndex = 0;
    (*monListPtr).listCount = count + 1;
    (*menu).state = 0;
}
pub(crate) unsafe extern "C" fn InitSearchResultsConditionList() {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).inSearchMode = TRUE;
    (*menu).state = 0;
}
pub(crate) unsafe extern "C" fn GetMonConditionGraphData(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let mut i: u16 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    if listId as i32
        != (if IsConditionMenuSearchMode() != 0 {
            (*monListPtr).listCount as i32
        } else {
            (*monListPtr).listCount as i32 - 1
        })
    {
        boxId = (*monListPtr).monData[listId].boxId as u16;
        monId = (*monListPtr).monData[listId].monId as u16;
        (*menu).graph.conditions[loadId][0] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_COOL, null_mut()) as u8;
        (*menu).graph.conditions[loadId][1] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_TOUGH, null_mut()) as u8;
        (*menu).graph.conditions[loadId][2] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_SMART, null_mut()) as u8;
        (*menu).graph.conditions[loadId][3] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_CUTE, null_mut()) as u8;
        (*menu).graph.conditions[loadId][4] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_BEAUTY, null_mut()) as u8;
        (*menu).numSparkles[loadId] =
            (if GetBoxOrPartyMonData(boxId, monId, MON_DATA_SHEEN, null_mut()) != 255 {
                GetBoxOrPartyMonData(boxId, monId, MON_DATA_SHEEN, null_mut()) / 29
            } else {
                9
            }) as u8;
        (*menu).monMarks[loadId] =
            GetBoxOrPartyMonData(boxId, monId, MON_DATA_MARKINGS, null_mut()) as u8;
        ConditionGraph_CalcPositions(
            (*menu).graph.conditions[loadId].as_mut_ptr(),
            (*menu).graph.savedPositions[loadId].as_mut_ptr(),
        );
    } else {
        i = 0;
        while i < CONDITION_COUNT {
            (*menu).graph.conditions[loadId][i] = 0;
            (*menu).graph.savedPositions[loadId][i].x = CONDITION_GRAPH_CENTER_X;
            (*menu).graph.savedPositions[loadId][i].y = CONDITION_GRAPH_CENTER_Y;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn ConditionGraphDrawMonPic(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut tid: u32 = 0;
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    if listId as i32
        == (if IsConditionMenuSearchMode() != 0 {
            (*monListPtr).listCount as i32
        } else {
            (*monListPtr).listCount as i32 - 1
        })
    {
        return;
    }
    boxId = (*monListPtr).monData[listId].boxId as u16;
    monId = (*monListPtr).monData[listId].monId as u16;
    species = GetBoxOrPartyMonData(boxId, monId, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    tid = GetBoxOrPartyMonData(boxId, monId, MON_DATA_OT_ID, null_mut()) as u32;
    personality = GetBoxOrPartyMonData(boxId, monId, MON_DATA_PERSONALITY, null_mut()) as u32;
    LoadSpecialPokePic(
        (&raw const gMonFrontPicTable[species]).cast_mut(),
        (*menu).monPicGfx[loadId].as_mut_ptr() as *mut c_void,
        species as i32,
        personality,
        TRUE,
    );
    LZ77UnCompWram(
        GetMonSpritePalFromSpeciesAndPersonality(species, tid, personality),
        (*menu).monPal[loadId].as_mut_ptr() as *mut c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonListCount() -> u16 {
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    return (*monListPtr).listCount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphCurrentListIndex() -> u16 {
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    return (*monListPtr).currIndex;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphPtr() -> *mut ConditionGraph {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return &raw mut (*menu).graph;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuCurrentLoadIndex() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).loadId as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuToLoadListIndex() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).toLoadListIndex as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonPicGfx(loadId: u8) -> *mut c_void {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).monPicGfx[loadId].as_mut_ptr() as *mut c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonPal(loadId: u8) -> *mut c_void {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).monPal[loadId].as_mut_ptr() as *mut c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuToLoadId() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).toLoadId as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonNameText(loadId: u8) -> *mut u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).nameText[loadId].as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonLocationText(loadId: u8) -> *mut u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).locationText[loadId].as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonDataBuffer() -> u16 {
    let mut monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    return (*monListPtr).monData[(*monListPtr).currIndex].data;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsConditionMenuSearchMode() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == TRUE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGetMonMarkId() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == TRUE {
        return (*menu).monMarks[(*menu).loadId];
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumConditionMonSparkles() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    return (*menu).numSparkles[(*menu).loadId];
}
