//! Translated from `src/pokenav_conditions.c` by tools/rustport/c2rs.py.
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
    clippy::int_plus_one,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gKeyRepeatStartDelay;
use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::menu_specialized::{
    ConditionGraph_CalcPositions, ConditionGraph_Init, ConditionGraph_SetNewPositions,
    GetBoxOrPartyMonData,
};
use crate::mon_markings::HandleMonMarkingsMenuInput;
use crate::pokemon::{
    CalculatePlayerPartyCount, GetBoxMonGender, GetLevelFromBoxMonExp, GetMonData2, GetMonGender,
    GetMonSpritePalFromSpeciesAndPersonality, SetMonData, gPlayerParty,
};
use crate::pokemon_storage_system::{GetBoxNamePtr, GetBoxedMonPtr, SetBoxMonDataAt};
use crate::pokenav::{AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr};
use crate::pokenav_conditions_gfx::GetMonMarkingsData;
use crate::sound::PlaySE;
use crate::string_util::StringGet_Nickname;
use crate::string_util::{ConvertIntToDecimalStringN, StringCompare, StringCopyPadded};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `LoadSpecialPokePic` with this module's view of its types.
#[inline]
unsafe fn LoadSpecialPokePic(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
    a4: u8,
) {
    unsafe {
        crate::decompress::LoadSpecialPokePic(a0 as _, a1 as _, a2, a3, a4);
    }
}

/// `struct Pokenav_ConditionMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_ConditionMenu {
    pub monPal: CArray<CArray<u32, 32>, 3>,
    pub fill: CArray<u8, 384>,
    pub monPicGfx: CArray<CArray<u32, 2048>, 3>,
    pub inSearchMode: u8,
    pub toLoadListIndex: i16,
    pub callback: Option<unsafe fn(*mut Pokenav_ConditionMenu) -> u32>,
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

/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn PokenavCallback_Init_ConditionGraph_Party() -> u32 {
    let menu: *mut Pokenav_ConditionMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU, 26508) as *mut Pokenav_ConditionMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    ConditionGraph_Init(&raw mut (*menu).graph);
    InitPartyConditionListParameters();
    gKeyRepeatStartDelay = 20;
    (*menu).callback = Some(HandleConditionMenuInput);
    TRUE as u32
}
pub unsafe fn PokenavCallback_Init_ConditionGraph_Search() -> u32 {
    let menu: *mut Pokenav_ConditionMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU, 26508) as *mut Pokenav_ConditionMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    ConditionGraph_Init(&raw mut (*menu).graph);
    InitSearchResultsConditionList();
    gKeyRepeatStartDelay = 20;
    (*menu).callback = Some(HandleConditionMenuInput);
    TRUE as u32
}
pub unsafe fn GetConditionGraphMenuCallback() -> u32 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).callback.unwrap_unchecked()(menu)
}
pub(crate) unsafe fn HandleConditionMenuInput(menu: *mut Pokenav_ConditionMenu) -> u32 {
    let monListPtr: *mut PokenavMonList =
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
    ret
}
pub(crate) unsafe fn OpenMarkingsMenu(menu: *mut Pokenav_ConditionMenu) -> u32 {
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
    ret
}
pub(crate) unsafe fn GetConditionReturnCallback(menu: *mut Pokenav_ConditionMenu) -> u32 {
    if (*menu).inSearchMode == 0 {
        return POKENAV_CONDITION_MENU;
    } else {
        return POKENAV_RETURN_CONDITION_SEARCH;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn FreeConditionGraphMenuSubstruct1() {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == 0 {
        FreePokenavSubstruct(POKENAV_SUBSTRUCT_MON_LIST);
    }
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU);
}
unsafe fn ConditionGraphHandleDpadInput(menu: *mut Pokenav_ConditionMenu) -> u8 {
    let monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    let mut ret: u8 = CONDITION_FUNC_NONE as u8;
    if gMain.heldKeys as i32 & DPAD_UP != 0 {
        if (*menu).inSearchMode == 0 || (*monListPtr).currIndex != 0 {
            PlaySE(SE_SELECT);
            ret = SwitchConditionSummaryIndex(TRUE);
        }
    } else if gMain.heldKeys as i32 & DPAD_DOWN != 0
        && ((*menu).inSearchMode == 0
            || ((*monListPtr).currIndex as i32) < (*monListPtr).listCount as i32 - 1)
    {
        PlaySE(SE_SELECT);
        ret = SwitchConditionSummaryIndex(FALSE);
    }
    ret
}
unsafe fn SwitchConditionSummaryIndex(moveUp: u8) -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    let newLoadId: u16 = (if moveUp != 0 {
        (*menu).nextLoadIdUp
    } else {
        (*menu).nextLoadIdDown
    }) as u16;
    ConditionGraph_SetNewPositions(
        &raw mut (*menu).graph,
        (*menu).graph.savedPositions[(*menu).loadId].as_mut_ptr(),
        (*menu).graph.savedPositions[newLoadId].as_mut_ptr(),
    );
    let wasNotLastMon: u8 = ((*monListPtr).currIndex as i32
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
    let isNotLastMon: u8 = ((*monListPtr).currIndex as i32
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
        0
    }
}
pub unsafe fn LoadConditionGraphMenuGfx() -> u32 {
    let mut var: i32 = 0;
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
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
    FALSE as u32
}
pub unsafe fn LoadNextConditionMenuMonData(mode: u8) -> u32 {
    let menu: *mut Pokenav_ConditionMenu =
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
    FALSE as u32
}
pub unsafe fn CopyStringLeftAlignedToConditionData(
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
    dst
}
unsafe fn CopyConditionMonNameGender(mut str: *mut u8, listId: u16, skipPadding: u8) -> *mut u8 {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let mut gender: u16 = 0;
    let mut level: u16 = 0;
    let mut boxMon: *mut BoxPokemon = null_mut();
    let monListPtr: *mut PokenavMonList =
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
        return StringCopyPadded(
            str,
            (*(&raw const crate::data::strings::gText_EggNickname).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            CHAR_SPACE,
            12,
        );
    }
    GetBoxOrPartyMonData(boxId, monId, MON_DATA_NICKNAME, str);
    StringGet_Nickname(str);
    let species: u16 = GetBoxOrPartyMonData(boxId, monId, MON_DATA_SPECIES, null_mut()) as u16;
    if boxId == TOTAL_BOXES_COUNT as u16 {
        level = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u16;
        gender = GetMonGender(&raw mut gPlayerParty[monId]) as u16;
    } else {
        boxMon = GetBoxedMonPtr(boxId as u8, monId as u8);
        gender = GetBoxMonGender(boxMon) as u16;
        level = GetLevelFromBoxMonExp(boxMon) as u16;
    }
    if (species == SPECIES_NIDORAN_F || species == SPECIES_NIDORAN_M)
        && StringCompare(
            str,
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        ) == 0
    {
        gender = MON_GENDERLESS as u16;
    }
    let mut str_: *mut u8 = str;
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
    let txtPtr: *mut u8 = str_;
    str_ = ConvertIntToDecimalStringN(str_, level as i32, STR_CONV_MODE_LEFT_ALIGN, 3);
    let mut lvlDigits: u16 = (str_ as usize).wrapping_sub(txtPtr as usize) as i32 as u16;
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
    str_
}
unsafe fn CopyMonNameGenderLocation(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut i: u16 = 0;
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
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
                (*(&raw const crate::data::strings::gText_InParty).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
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
unsafe fn InitPartyConditionListParameters() {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
        AllocSubstruct(POKENAV_SUBSTRUCT_MON_LIST, 1708) as *mut PokenavMonList;
    (*menu).inSearchMode = FALSE;
    let mut i: u16 = 0;
    let mut count: u16 = 0;
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
unsafe fn InitSearchResultsConditionList() {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).inSearchMode = TRUE;
    (*menu).state = 0;
}
unsafe fn GetMonConditionGraphData(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
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
        for i in 0..CONDITION_COUNT {
            (*menu).graph.conditions[loadId][i] = 0;
            (*menu).graph.savedPositions[loadId][i].x = CONDITION_GRAPH_CENTER_X;
            (*menu).graph.savedPositions[loadId][i].y = CONDITION_GRAPH_CENTER_Y;
        }
    }
}
unsafe fn ConditionGraphDrawMonPic(listId: i16, loadId: u8) {
    let mut boxId: u16 = 0;
    let mut monId: u16 = 0;
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    let monListPtr: *mut PokenavMonList =
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
    let species: u16 =
        GetBoxOrPartyMonData(boxId, monId, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16;
    let tid: u32 = GetBoxOrPartyMonData(boxId, monId, MON_DATA_OT_ID, null_mut()) as u32;
    let personality: u32 =
        GetBoxOrPartyMonData(boxId, monId, MON_DATA_PERSONALITY, null_mut()) as u32;
    LoadSpecialPokePic(
        (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
            .cast_mut(),
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
pub unsafe fn GetMonListCount() -> u16 {
    let monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*monListPtr).listCount
}
pub unsafe fn GetConditionGraphCurrentListIndex() -> u16 {
    let monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*monListPtr).currIndex
}
pub unsafe fn GetConditionGraphPtr() -> *mut ConditionGraph {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    &raw mut (*menu).graph
}
#[unsafe(no_mangle)]
pub unsafe fn GetConditionGraphMenuCurrentLoadIndex() -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).loadId as u8
}
pub unsafe fn GetConditionGraphMenuToLoadListIndex() -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).toLoadListIndex as u8
}
pub unsafe fn GetConditionMonPicGfx(loadId: u8) -> *mut c_void {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).monPicGfx[loadId].as_mut_ptr() as *mut c_void
}
pub unsafe fn GetConditionMonPal(loadId: u8) -> *mut c_void {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).monPal[loadId].as_mut_ptr() as *mut c_void
}
pub unsafe fn GetConditionGraphMenuToLoadId() -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).toLoadId as u8
}
pub unsafe fn GetConditionMonNameText(loadId: u8) -> *mut u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).nameText[loadId].as_mut_ptr()
}
pub unsafe fn GetConditionMonLocationText(loadId: u8) -> *mut u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).locationText[loadId].as_mut_ptr()
}
pub unsafe fn GetConditionMonDataBuffer() -> u16 {
    let monListPtr: *mut PokenavMonList =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MON_LIST) as *mut PokenavMonList;
    (*monListPtr).monData[(*monListPtr).currIndex].data
}
pub unsafe fn IsConditionMenuSearchMode() -> u32 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == TRUE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn TryGetMonMarkId() -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    if (*menu).inSearchMode == TRUE {
        return (*menu).monMarks[(*menu).loadId];
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetNumConditionMonSparkles() -> u8 {
    let menu: *mut Pokenav_ConditionMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU) as *mut Pokenav_ConditionMenu;
    (*menu).numSparkles[(*menu).loadId]
}
