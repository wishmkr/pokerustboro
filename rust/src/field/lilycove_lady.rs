//! Translated from `src/lilycove_lady.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sContestLadyMonGfxId sLilycoveLadyGfxId sQuizLadyQuestion1 sQuizLadyQuestion2 sQuizLadyQuestion3 sQuizLadyQuestion4 sQuizLadyQuestion5 sQuizLadyQuestion6 sQuizLadyQuestion7 sQuizLadyQuestion8 sQuizLadyQuestion9 sQuizLadyQuestion10 sQuizLadyQuestion11 sQuizLadyQuestion12 sQuizLadyQuestion13 sQuizLadyQuestion14 sQuizLadyQuestion15 sQuizLadyQuestion16 sQuizLadyQuizQuestions sQuizLadyQuizAnswers sQuizLadyPrizes sFavorLadyRequests sFavorLadyAcceptedItems_Slippery sFavorLadyAcceptedItems_Roundish sFavorLadyAcceptedItems_Whamish sFavorLadyAcceptedItems_Shiny sFavorLadyAcceptedItems_Sticky sFavorLadyAcceptedItems_Pointy sFavorLadyAcceptedItemLists sFavorLadyPrizes sContestLadyMonNames sContestLadyCategoryNames sContestNames sContestLadyMonSpecies
#[allow(unused_imports)]
use crate::data::lilycove_lady::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFavorLadyPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sQuizLadyPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sContestLadyPtr: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gGameLanguage: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gText_QuizLady_Lady: u8;
    fn CB2_ReturnToField();
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn FavorLadyOpenBagMenu();
    fn GetItemName(a0: u16) -> *mut u8;
    fn IsEasyChatAnswerUnlocked(a0: i32) -> u32;
    fn OpenPokeblockCase(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn QuizLadyOpenBagMenu();
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn ScriptContext_Enable();
    fn ShowEasyChatScreen();
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_PlayerName(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLilycoveLadyId() -> u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLilycoveLadyGfx() {
    unsafe {
        let mut lilycoveLady: *mut u8 = core::ptr::null_mut();
        VarSet(
            16400u16,
            ((((&raw const sLilycoveLadyGfxId)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((GetLilycoveLadyId()) as i32) as isize))
            .read(),
        );
        if ((GetLilycoveLadyId()) as i32) == 2i32 {
            lilycoveLady =
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192);
            VarSet(
                16401u16,
                ((((&raw const sContestLadyMonGfxId)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((((lilycoveLady).wrapping_add(13)).read()) as i32) as isize))
                .read(),
            );
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitLilycoveLady() {
    unsafe {
        let mut id: u16 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            << 8)
            | (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .read()) as i32)) as u16);
        id = ((crate::c::rem_i32(((id) as i32), 6i32)) as u16);
        id = ((((id) as i32) >> 1) as u16);
        'l1: {
            let __sw1 = ((id) as i32);
            if __sw1 == 0i32 {
                InitLilycoveQuizLady();
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitLilycoveFavorLady();
                break 'l1;
            }
            if __sw1 == 2i32 {
                InitLilycoveContestLady();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLilycoveLadyForRecordMix() {
    unsafe {
        'l1: {
            let __sw1 = ((GetLilycoveLadyId()) as i32);
            if __sw1 == 0i32 {
                ResetQuizLadyForRecordMix();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetFavorLadyForRecordMix();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetContestLadyForRecordMix();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitLilycoveLadyRandomly() {
    unsafe {
        let mut lady: u8 = ((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8);
        'l1: {
            let __sw1 = ((lady) as i32);
            if __sw1 == 0i32 {
                InitLilycoveQuizLady();
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitLilycoveFavorLady();
                break 'l1;
            }
            if __sw1 == 2i32 {
                InitLilycoveContestLady();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_GetLilycoveLadyId() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetLilycoveLadyId()) as u16));
    }
}
pub(crate) unsafe extern "C" fn GetNumAcceptedItems(itemsArray: *mut u16) -> u8 {
    unsafe {
        let mut itemsArray = itemsArray;
        let mut numItems: u8 = 0u8;
        {
            numItems = 0u8;
            'l1: loop {
                if !((((itemsArray).read()) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {}
                numItems = (numItems).wrapping_add(1);
                itemsArray = (itemsArray).wrapping_offset(1);
            }
        }
        return numItems;
    }
}
pub(crate) unsafe extern "C" fn FavorLadyPickFavorAndBestItem() {
    unsafe {
        let mut numItems: u8 = 0u8;
        let mut bestItem: u8 = 0u8;
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .write(
                ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(24u32, 4u32))) as u8),
            );
        numItems = GetNumAcceptedItems(
            ((((&raw const sFavorLadyAcceptedItemLists)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        bestItem = ((crate::c::rem_i32(((Random()) as i32), ((numItems) as i32))) as u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            ((((((&raw const sFavorLadyAcceptedItemLists)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read()) as i32) as isize,
            ))
            .read())
            .wrapping_offset(((bestItem) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitLilycoveFavorLady() {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        (((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<u8>())
        .write(255u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(0u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(0u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
        FavorLadyPickFavorAndBestItem();
    }
}
pub(crate) unsafe extern "C" fn ResetFavorLadyForRecordMix() {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(1u8);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFavorLadyState() -> u8 {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 2i32
        {
            return 2u8;
        } else {
            if ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32)
                == 1i32
            {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetFavorLadyRequest(idx: u8) -> *mut u8 {
    unsafe {
        let mut idx = idx;
        return ((((&raw const sFavorLadyRequests)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(((idx) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferFavorLadyRequest() {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            GetFavorLadyRequest(
                ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read(),
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasAnotherPlayerGivenFavorLadyItem() -> u8 {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if (((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<u8>())
        .read()) as i32)
            != 255i32
        {
            StringCopy_PlayerName(
                (&raw mut gStringVar3).cast::<u8>(),
                ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .cast::<u8>(),
            );
            ConvertInternationalString(
                (&raw mut gStringVar3).cast::<u8>(),
                ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18))
                .read(),
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BufferItemName(dest: *mut u8, itemId: u16) {
    unsafe {
        let mut dest = dest;
        let mut itemId = itemId;
        StringCopy(dest, GetItemName(itemId));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferFavorLadyItemName() {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        BufferItemName(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetFavorLadyPlayerName(src: *mut u8, dest: *mut u8) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        crate::c::memset(dest, 255i32, 8u32);
        StringCopy_PlayerName(dest, src);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferFavorLadyPlayerName() {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        SetFavorLadyPlayerName(
            ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
            (&raw mut gStringVar3).cast::<u8>(),
        );
        ConvertInternationalString(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
                .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DidFavorLadyLikeItem() -> u8 {
    unsafe {
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        return ((if (((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read())
            != 0
        {
            1i32
        } else {
            0i32
        }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_FavorLadyOpenBagMenu() {
    unsafe {
        FavorLadyOpenBagMenu();
    }
}
pub(crate) unsafe extern "C" fn DoesFavorLadyLikeItem(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        let mut numItems: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut likedItem: u8 = 0u8;
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        numItems = GetNumAcceptedItems(
            ((((&raw const sFavorLadyAcceptedItemLists)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(1u8);
        BufferItemName((&raw mut gStringVar2).cast::<u8>(), itemId);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(itemId);
        SetFavorLadyPlayerName(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
        );
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(18))
            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
        likedItem = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numItems) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const sFavorLadyAcceptedItemLists)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u16>())
                    .cast::<*mut u16>())
                    .wrapping_offset(
                        ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .read()) as i32) as isize,
                    ))
                    .read())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((itemId) as i32)
                    {
                        likedItem = 1u8;
                        let __p1 = (((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(3);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .write(1u8);
                        if ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((itemId) as i32)
                        {
                            ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3))
                            .write(5u8);
                        }
                        break 'l1;
                    }
                    ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return likedItem;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_DoesFavorLadyLikeItem() -> u8 {
    unsafe {
        return DoesFavorLadyLikeItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFavorLadyThresholdMet() -> u8 {
    unsafe {
        let mut numItemsGiven: u8 = 0u8;
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        numItemsGiven = ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3))
        .read();
        return ((if ((numItemsGiven) as i32) < 5i32 {
            0i32
        } else {
            1i32
        }) as u8);
    }
}
pub(crate) unsafe extern "C" fn FavorLadyBufferPrizeName(prize: u16) {
    unsafe {
        let mut prize = prize;
        BufferItemName((&raw mut gStringVar2).cast::<u8>(), prize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FavorLadyGetPrize() -> u16 {
    unsafe {
        let mut prize: u16 = 0u16;
        ((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        prize = ((((&raw const sFavorLadyPrizes)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(
            ((((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .read()) as i32) as isize,
        ))
        .read();
        FavorLadyBufferPrizeName(prize);
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(2u8);
        return prize;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFavorLadyState_Complete() {
    unsafe {
        InitLilycoveFavorLady();
        ((((&raw mut sFavorLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCallback_FavorLadyEnableScriptContexts() {
    unsafe {
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn QuizLadyPickQuestion() {
    unsafe {
        let mut questionId: u8 = 0u8;
        let mut i: u8 = 0u8;
        questionId =
            ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(64u32, 4u32))) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw const sQuizLadyQuizQuestions)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u16>())
                        .cast::<*mut u16>())
                        .wrapping_offset(((questionId) as i32) as isize))
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(
            ((((&raw const sQuizLadyQuizAnswers)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((questionId) as i32) as isize))
            .read(),
        );
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<u16>())
        .write(
            ((((&raw const sQuizLadyPrizes)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((questionId) as i32) as isize))
            .read(),
        );
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(43))
            .write(questionId);
        (((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
            .cast::<u8>())
        .write(255u8);
    }
}
pub(crate) unsafe extern "C" fn InitLilycoveQuizLady() {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(65535u16);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<u16>())
        .write(65535u16);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(42))
            .write(0u8);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
            .write(((crate::c::div_u32(64u32, 4u32)) as u8));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(45))
            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
        QuizLadyPickQuestion();
    }
}
pub(crate) unsafe extern "C" fn ResetQuizLadyForRecordMix() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(42))
            .write(0u8);
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<u16>())
        .write(65535u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetQuizLadyState() -> u8 {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 2i32
        {
            return 2u8;
        } else {
            if ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32)
                == 1i32
            {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetQuizAuthor() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut authorNameId: u8 = 0u8;
        let mut quiz: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192));
        if IsEasyChatAnswerUnlocked(((((quiz).wrapping_add(20).cast::<u16>()).read()) as i32))
            == 0u32
        {
            i = ((((quiz).wrapping_add(43)).read()) as i32);
            'l1: loop {
                'l2: {
                    if {
                        let __t1 = (i).wrapping_add(1);
                        i = __t1;
                        __t1
                    } >= ((crate::c::div_u32(64u32, 4u32)) as i32)
                    {
                        i = 0i32;
                    }
                }
                if !(IsEasyChatAnswerUnlocked(
                    ((((((&raw const sQuizLadyQuizAnswers)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32),
                ) == 0u32)
                {
                    break 'l1;
                }
            }
            {
                j = 0i32;
                'l3: loop {
                    if !(j < 9i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((quiz).wrapping_add(2)).cast::<u16>()).wrapping_offset((j) as isize))
                            .write(
                                ((((((&raw const sQuizLadyQuizQuestions)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u16>())
                                .cast::<*mut u16>())
                                .wrapping_offset((i) as isize))
                                .read())
                                .wrapping_offset((j) as isize))
                                .read(),
                            );
                    }
                    j = (j).wrapping_add(1);
                }
            }
            ((quiz).wrapping_add(20).cast::<u16>()).write(
                ((((&raw const sQuizLadyQuizAnswers)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read(),
            );
            ((quiz).wrapping_add(40).cast::<u16>()).write(
                ((((&raw const sQuizLadyPrizes)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read(),
            );
            ((quiz).wrapping_add(43)).write(((i) as u8));
            (((quiz).wrapping_add(24)).cast::<u8>()).write(255u8);
        }
        authorNameId = BufferQuizAuthorName();
        if ((authorNameId) as i32) == 0i32 {
            return 2u8;
        } else {
            if (((authorNameId) as i32) == 2i32) || ((IsQuizTrainerIdNotPlayer()) != 0) {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BufferQuizAuthorName() -> u8 {
    unsafe {
        let mut authorNameId: u8 = 0u8;
        let mut nameLen: u8 = 0u8;
        let mut i: u8 = 0u8;
        authorNameId = 1u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if (((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
            .cast::<u8>())
        .read()) as i32)
            == 255i32
        {
            StringCopy_PlayerName(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_QuizLady_Lady).cast::<u8>(),
            );
            authorNameId = 0u8;
        } else {
            StringCopy_PlayerName(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .cast::<u8>(),
            );
            ConvertInternationalString(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(45))
                .read(),
            );
            nameLen = GetPlayerNameLength(
                ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24))
                .cast::<u8>(),
            );
            if ((nameLen) as i32)
                == ((GetPlayerNameLength(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                )) as i32)
            {
                let mut name: *mut u8 =
                    ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24))
                    .cast::<u8>();
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < ((nameLen) as i32)) {
                            break 'l1;
                        }
                        'l2: {
                            name = ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(24))
                            .cast::<u8>();
                            if ((((name).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                            {
                                authorNameId = 2u8;
                                break 'l1;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        return authorNameId;
    }
}
pub(crate) unsafe extern "C" fn IsQuizTrainerIdNotPlayer() -> u8 {
    unsafe {
        let mut notPlayer: u8 = 0u8;
        let mut i: u8 = 0u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        notPlayer = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        notPlayer = 1u8;
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return notPlayer;
    }
}
pub(crate) unsafe extern "C" fn GetPlayerNameLength(playerName: *mut u8) -> u8 {
    unsafe {
        let mut playerName = playerName;
        let mut len: u8 = 0u8;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        {
            len = 0u8;
            ptr = playerName;
            'l1: loop {
                if !((((ptr).read()) as i32) != 255i32) {
                    break 'l1;
                }
                'l2: {}
                len = (len).wrapping_add(1);
                ptr = (ptr).wrapping_offset(1);
            }
        }
        return len;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferQuizPrizeName() {
    unsafe {
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            GetItemName(
                ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<u16>())
                .read(),
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferQuizAuthorNameAndCheckIfLady() -> u8 {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((BufferQuizAuthorName()) as i32) == 0i32 {
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(45))
                .write(((&raw mut gGameLanguage).cast::<u8>()).read());
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsQuizLadyWaitingForChallenger() -> u8 {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        return ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(42))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyGetPlayerAnswer() {
    unsafe {
        ShowEasyChatScreen();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsQuizAnswerCorrect() -> u8 {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        CopyEasyChatWord(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<u16>())
            .read(),
        );
        CopyEasyChatWord(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22)
                .cast::<u16>())
            .read(),
        );
        return ((if (StringCompare(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gStringVar2).cast::<u8>(),
        )) != 0
        {
            0i32
        } else {
            1i32
        }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferQuizPrizeItem() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetQuizLadyState_Complete() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetQuizLadyState_GivePrize() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearQuizLadyPlayerAnswer() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<u16>())
        .write(65535u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_QuizLadyOpenBagMenu() {
    unsafe {
        QuizLadyOpenBagMenu();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyPickNewQuestion() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if (BufferQuizAuthorNameAndCheckIfLady()) != 0 {
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .write(
                    ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(43))
                    .read(),
                );
        } else {
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .write(((crate::c::div_u32(64u32, 4u32)) as u8));
        }
        QuizLadyPickQuestion();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearQuizLadyQuestionAndAnswer() {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(20)
            .cast::<u16>())
        .write(65535u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadySetCustomQuestion() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(17u16);
        ShowEasyChatScreen();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyTakePrizeForCustomQuiz() {
    unsafe {
        RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyRecordCustomQuizData() {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy_PlayerName(
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(24))
                .cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(45))
            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadySetWaitingForChallenger() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(42))
            .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferQuizCorrectAnswer() {
    unsafe {
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        CopyEasyChatWord(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCallback_QuizLadyEnableScriptContexts() {
    unsafe {
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyClearQuestionForRecordMix(lilycoveLady: *mut u8) {
    unsafe {
        let mut lilycoveLady = lilycoveLady;
        let mut i: u8 = 0u8;
        ((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if (((((lilycoveLady).wrapping_add(44)).read()) as u32) < crate::c::div_u32(64u32, 4u32))
            && ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                == 0i32)
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((lilycoveLady).wrapping_add(44)).read()) as i32)
                            != ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(43))
                            .read()) as i32)
                        {
                            break 'l1;
                        }
                        ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(43))
                        .write(
                            ((crate::c::rem_u32(
                                ((Random()) as u32),
                                crate::c::div_u32(64u32, 4u32),
                            )) as u8),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((((lilycoveLady).wrapping_add(44)).read()) as i32)
                == ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(43))
                .read()) as i32)
            {
                ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(43))
                .write(
                    ((crate::c::rem_i32(
                        ((((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(43))
                        .read()) as i32)
                            .wrapping_add(1i32),
                        ((crate::c::div_u32(64u32, 4u32)) as i32),
                    )) as u8),
                );
            }
            ((((&raw mut sQuizLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44))
                .write(((lilycoveLady).wrapping_add(44)).read());
        }
    }
}
pub(crate) unsafe extern "C" fn ResetContestLadyContestData() {
    unsafe {
        (((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<u8>())
        .write(255u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(0u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(0u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .write(0u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
            .write(((crate::c::rem_i32(((Random()) as i32), 5i32)) as u8));
    }
}
pub(crate) unsafe extern "C" fn InitLilycoveContestLady() {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        ResetContestLadyContestData();
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn ResetContestLadyForRecordMix() {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).write(2u8);
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        if (((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as i32)
            == 5i32)
            || (((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                == 5i32)
        {
            ResetContestLadyContestData();
        }
    }
}
pub(crate) unsafe extern "C" fn ContestLadySavePlayerNameIfHighSheen(sheen: u8) {
    unsafe {
        let mut sheen = sheen;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .read()) as i32)
            <= ((sheen) as i32)
        {
            ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
                .write(sheen);
            crate::c::memset(
                ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .cast::<u8>(),
                255i32,
                8u32,
            );
            crate::c::memcpy(
                ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                8u32,
            );
            ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
                .write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GivePokeblockToContestLady(pokeblock: *mut u8) -> u8 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut sheen: u8 = 0u8;
        let mut correctFlavor: u8 = 0u8;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        'l1: {
            let __sw1 = ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(13))
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((pokeblock).wrapping_add(1)).read()) as i32) != 0i32 {
                    sheen = ((pokeblock).wrapping_add(1)).read();
                    correctFlavor = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((pokeblock).wrapping_add(2)).read()) as i32) != 0i32 {
                    sheen = ((pokeblock).wrapping_add(2)).read();
                    correctFlavor = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((pokeblock).wrapping_add(3)).read()) as i32) != 0i32 {
                    sheen = ((pokeblock).wrapping_add(3)).read();
                    correctFlavor = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((pokeblock).wrapping_add(4)).read()) as i32) != 0i32 {
                    sheen = ((pokeblock).wrapping_add(4)).read();
                    correctFlavor = 1u8;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((pokeblock).wrapping_add(5)).read()) as i32) != 0i32 {
                    sheen = ((pokeblock).wrapping_add(5)).read();
                    correctFlavor = 1u8;
                }
                break 'l1;
            }
        }
        if ((correctFlavor) as i32) == 1i32 {
            ContestLadySavePlayerNameIfHighSheen(sheen);
            let __p2 = (((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            let __p3 = (((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return correctFlavor;
    }
}
pub(crate) unsafe extern "C" fn BufferContestLadyCategoryAndMonName(
    category: *mut u8,
    nickname: *mut u8,
) {
    unsafe {
        let mut category = category;
        let mut nickname = nickname;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        StringCopy(
            category,
            ((((&raw const sContestLadyCategoryNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        StringCopy_Nickname(
            nickname,
            ((((&raw const sContestLadyMonNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13))
                .read()) as i32) as isize,
            ))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestLadyMonName(category: *mut u8, nickname: *mut u8) {
    unsafe {
        let mut category = category;
        let mut nickname = nickname;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (category).write(
            ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(13))
                .read(),
        );
        StringCopy(
            nickname,
            ((((&raw const sContestLadyMonNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13))
                .read()) as i32) as isize,
            ))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestLadyPlayerName(dest: *mut u8) {
    unsafe {
        let mut dest = dest;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        StringCopy(
            dest,
            ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestLadyLanguage(dest: *mut u8) {
    unsafe {
        let mut dest = dest;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        (dest).write(
            ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
                .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferContestName(dest: *mut u8, category: u8) {
    unsafe {
        let mut dest = dest;
        let mut category = category;
        StringCopy(
            dest,
            ((((&raw const sContestNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((category) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestLadyPokeblockState() -> u8 {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .read()) as i32)
            >= 5i32
        {
            return 1u8;
        } else {
            if ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                == 0i32
            {
                return 2u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasPlayerGivenContestLadyPokeblock() -> u8 {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 1i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldContestLadyShowGoOnAir() -> u8 {
    unsafe {
        let mut putOnAir: u8 = 0u8;
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        if (((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as i32)
            >= 5i32)
            || (((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32)
                >= 5i32)
        {
            putOnAir = 1u8;
        }
        return putOnAir;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_BufferContestLadyCategoryAndMonName() {
    unsafe {
        BufferContestLadyCategoryAndMonName(
            (&raw mut gStringVar2).cast::<u8>(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCaseForContestLady() {
    unsafe {
        OpenPokeblockCase(3u8, Some(CB2_ReturnToField));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestLadyGivenPokeblock() {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestLadyMonSpecies() {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
            ((((&raw const sContestLadyMonSpecies)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(
                ((((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(13))
                .read()) as i32) as isize,
            ))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetContestLadyCategory() -> u8 {
    unsafe {
        ((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(15192)));
        return ((((&raw mut sContestLadyPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(13))
        .read();
    }
}
