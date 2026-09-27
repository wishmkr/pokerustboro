//! Translated from `src/pokenav_conditions.c` by tools/rustport/c2rs.py, then reviewed.
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

unsafe extern "C" {
    static mut gKeyRepeatStartDelay: u8;
    static mut gMain: u8;
    static mut gMonFrontPicTable: u8;
    static mut gPlayerParty: u8;
    static mut gSpeciesNames: u8;
    static mut gText_EggNickname: u8;
    static mut gText_InParty: u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn CalculatePlayerPartyCount() -> u8;
    fn ConditionGraph_CalcPositions(a0: *mut u8, a1: *mut u8);
    fn ConditionGraph_Init(a0: *mut u8);
    fn ConditionGraph_SetNewPositions(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn FreePokenavSubstruct(a0: u32);
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetBoxNamePtr(a0: u8) -> *mut u8;
    fn GetBoxOrPartyMonData(a0: u16, a1: u16, a2: i32, a3: *mut u8) -> i32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonMarkingsData() -> u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn HandleMonMarkingsMenuInput() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadSpecialPokePic(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32, a4: u8);
    fn PlaySE(a0: u16);
    fn SetBoxMonDataAt(a0: u8, a1: u8, a2: i32, a3: *mut u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionGraph_Party() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(11u32, 26508u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ConditionGraph_Init((menu).wrapping_add(25640));
        InitPartyConditionListParameters();
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(20u16);
        ((menu)
            .wrapping_add(25348)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(HandleConditionMenuInput));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_ConditionGraph_Search() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(11u32, 26508u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ConditionGraph_Init((menu).wrapping_add(25640));
        InitSearchResultsConditionList();
        ((&raw mut gKeyRepeatStartDelay).cast::<u16>()).write(20u16);
        ((menu)
            .wrapping_add(25348)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .write(Some(HandleConditionMenuInput));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuCallback() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return (((menu)
            .wrapping_add(25348)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .read())
        .unwrap_unchecked()(menu);
    }
}
pub(crate) unsafe extern "C" fn HandleConditionMenuInput(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        let mut ret: u32 = ((ConditionGraphHandleDpadInput(menu)) as u32);
        if ret == 0u32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                PlaySE(5u16);
                ((menu)
                    .wrapping_add(25348)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(GetConditionReturnCallback));
                ret = 2u32;
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    if !((((menu).wrapping_add(25344)).read()) != 0) {
                        if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                            == ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                        {
                            PlaySE(5u16);
                            ((menu)
                                .wrapping_add(25348)
                                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                            .write(Some(GetConditionReturnCallback));
                            ret = 2u32;
                        }
                    } else {
                        PlaySE(5u16);
                        ret = 5u32;
                        ((menu)
                            .wrapping_add(25348)
                            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                        .write(Some(OpenMarkingsMenu));
                    }
                }
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn OpenMarkingsMenu(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        let mut monListPtr: *mut u8 = core::ptr::null_mut();
        let mut markings: u8 = 0u8;
        let mut ret: u32 = 0u32;
        let mut boxId: u32 = 0u32;
        let mut monId: u32 = 0u32;
        if !((HandleMonMarkingsMenuInput()) != 0) {
            ((((menu).wrapping_add(26499)).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(26502).cast::<i8>()).read()) as i32) as isize,
            ))
            .write(GetMonMarkingsData());
            monListPtr = GetSubstructPtr(18u32);
            boxId = ((((((monListPtr).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4,
            ))
            .read()) as u32);
            monId = (((((((monListPtr).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4,
            ))
            .wrapping_add(1))
            .read()) as u32);
            markings = ((((menu).wrapping_add(26499)).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(26502).cast::<i8>()).read()) as i32) as isize,
            ))
            .read();
            if boxId == 14u32 {
                SetMonData(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monId) as i32) as isize * 100),
                    8i32,
                    &raw mut markings,
                );
            } else {
                SetBoxMonDataAt(((boxId) as u8), ((monId) as u8), 8i32, &raw mut markings);
            }
            ((menu)
                .wrapping_add(25348)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleConditionMenuInput));
            ret = 6u32;
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn GetConditionReturnCallback(menu: *mut u8) -> u32 {
    unsafe {
        let mut menu = menu;
        if !((((menu).wrapping_add(25344)).read()) != 0) {
            return 100002u32;
        } else {
            return 100010u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeConditionGraphMenuSubstruct1() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        if !((((menu).wrapping_add(25344)).read()) != 0) {
            FreePokenavSubstruct(18u32);
        }
        FreePokenavSubstruct(11u32);
    }
}
pub(crate) unsafe extern "C" fn ConditionGraphHandleDpadInput(menu: *mut u8) -> u8 {
    unsafe {
        let mut menu = menu;
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        let mut ret: u8 = 0u8;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            if (!((((menu).wrapping_add(25344)).read()) != 0))
                || (((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) != 0i32)
            {
                PlaySE(5u16);
                ret = SwitchConditionSummaryIndex(1u8);
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0
            {
                if (!((((menu).wrapping_add(25344)).read()) != 0))
                    || (((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        < ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32))
                {
                    PlaySE(5u16);
                    ret = SwitchConditionSummaryIndex(0u8);
                }
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn SwitchConditionSummaryIndex(moveUp: u8) -> u8 {
    unsafe {
        let mut moveUp = moveUp;
        let mut newLoadId: u16 = 0u16;
        let mut wasNotLastMon: u8 = 0u8;
        let mut isNotLastMon: u8 = 0u8;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        newLoadId = ((if (moveUp) != 0 {
            ((((menu).wrapping_add(26504).cast::<i8>()).read()) as i32)
        } else {
            ((((menu).wrapping_add(26503).cast::<i8>()).read()) as i32)
        }) as u16);
        ConditionGraph_SetNewPositions(
            (menu).wrapping_add(25640),
            (((((menu).wrapping_add(25640)).wrapping_add(20)).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(26502).cast::<i8>()).read()) as i32) as isize * 20,
            ))
            .cast::<u8>(),
            (((((menu).wrapping_add(25640)).wrapping_add(20)).cast::<u8>())
                .wrapping_offset(((newLoadId) as i32) as isize * 20))
            .cast::<u8>(),
        );
        wasNotLastMon = ((((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
            != (if (IsConditionMenuSearchMode()) != 0 {
                ((((monListPtr).cast::<u16>()).read()) as i32)
            } else {
                ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            })) as u8);
        if (moveUp) != 0 {
            ((menu).wrapping_add(26504).cast::<i8>())
                .write(((menu).wrapping_add(26503).cast::<i8>()).read());
            ((menu).wrapping_add(26503).cast::<i8>())
                .write(((menu).wrapping_add(26502).cast::<i8>()).read());
            ((menu).wrapping_add(26502).cast::<i8>()).write(((newLoadId) as i8));
            ((menu).wrapping_add(26505).cast::<i8>())
                .write(((menu).wrapping_add(26504).cast::<i8>()).read());
            ((monListPtr).wrapping_add(2).cast::<u16>()).write(
                ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) == 0i32 {
                    ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                } else {
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(1i32)
                }) as u16),
            );
            ((menu).wrapping_add(25346).cast::<i16>()).write(
                ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) != 0i32 {
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(1i32)
                } else {
                    ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                }) as i16),
            );
        } else {
            ((menu).wrapping_add(26503).cast::<i8>())
                .write(((menu).wrapping_add(26504).cast::<i8>()).read());
            ((menu).wrapping_add(26504).cast::<i8>())
                .write(((menu).wrapping_add(26502).cast::<i8>()).read());
            ((menu).wrapping_add(26502).cast::<i8>()).write(((newLoadId) as i8));
            ((menu).wrapping_add(26505).cast::<i8>())
                .write(((menu).wrapping_add(26503).cast::<i8>()).read());
            ((monListPtr).wrapping_add(2).cast::<u16>()).write(
                ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                    < ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                {
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_add(1i32)
                } else {
                    0i32
                }) as u16),
            );
            ((menu).wrapping_add(25346).cast::<i16>()).write(
                ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                    < ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                {
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_add(1i32)
                } else {
                    0i32
                }) as i16),
            );
        }
        isNotLastMon = ((((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
            != (if (IsConditionMenuSearchMode()) != 0 {
                ((((monListPtr).cast::<u16>()).read()) as i32)
            } else {
                ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            })) as u8);
        if !((wasNotLastMon) != 0) {
            return 3u8;
        } else {
            if !((isNotLastMon) != 0) {
                return 4u8;
            } else {
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
pub unsafe extern "C" fn LoadConditionGraphMenuGfx() -> u32 {
    unsafe {
        let mut var: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        'l1: {
            let __sw1 = ((((menu).wrapping_add(26506)).read()) as i32);
            if __sw1 == 0i32 {
                CopyMonNameGenderLocation(
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i16),
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                GetMonConditionGraphData(
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i16),
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ConditionGraphDrawMonPic(
                    ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i16),
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((monListPtr).cast::<u16>()).read()) as i32) == 1i32 {
                    ((menu).wrapping_add(26502).cast::<i8>()).write(0i8);
                    ((menu).wrapping_add(26503).cast::<i8>()).write(0i8);
                    ((menu).wrapping_add(26504).cast::<i8>()).write(0i8);
                    ((menu).wrapping_add(26506)).write(0u8);
                    return 1u32;
                } else {
                    ((menu).wrapping_add(26502).cast::<i8>()).write(0i8);
                    ((menu).wrapping_add(26503).cast::<i8>()).write(1i8);
                    ((menu).wrapping_add(26504).cast::<i8>()).write(2i8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                var = ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                    .wrapping_add(1i32);
                if var >= ((((monListPtr).cast::<u16>()).read()) as i32) {
                    var = 0i32;
                }
                CopyMonNameGenderLocation(((var) as i16), 1u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                var = ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                    .wrapping_add(1i32);
                if var >= ((((monListPtr).cast::<u16>()).read()) as i32) {
                    var = 0i32;
                }
                GetMonConditionGraphData(((var) as i16), 1u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                var = ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                    .wrapping_add(1i32);
                if var >= ((((monListPtr).cast::<u16>()).read()) as i32) {
                    var = 0i32;
                }
                ConditionGraphDrawMonPic(((var) as i16), 1u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                CopyMonNameGenderLocation(
                    ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(1i32)
                        >= 0i32
                    {
                        ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                            .wrapping_sub(1i32)
                    } else {
                        ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                    }) as i16),
                    2u8,
                );
                break 'l1;
            }
            if __sw1 == 8i32 {
                GetMonConditionGraphData(
                    ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(1i32)
                        >= 0i32
                    {
                        ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                            .wrapping_sub(1i32)
                    } else {
                        ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                    }) as i16),
                    2u8,
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                ConditionGraphDrawMonPic(
                    ((if ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                        .wrapping_sub(1i32)
                        >= 0i32
                    {
                        ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32)
                            .wrapping_sub(1i32)
                    } else {
                        ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
                    }) as i16),
                    2u8,
                );
                ((menu).wrapping_add(26506)).write(0u8);
                return 1u32;
            }
        }
        let __p2 = (menu).wrapping_add(26506);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadNextConditionMenuMonData(mode: u8) -> u32 {
    unsafe {
        let mut mode = mode;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                CopyMonNameGenderLocation(
                    ((menu).wrapping_add(25346).cast::<i16>()).read(),
                    ((((menu).wrapping_add(26505).cast::<i8>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                GetMonConditionGraphData(
                    ((menu).wrapping_add(25346).cast::<i16>()).read(),
                    ((((menu).wrapping_add(26505).cast::<i8>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ConditionGraphDrawMonPic(
                    ((menu).wrapping_add(25346).cast::<i16>()).read(),
                    ((((menu).wrapping_add(26505).cast::<i8>()).read()) as u8),
                );
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyStringLeftAlignedToConditionData(
    dst: *mut u8,
    src: *mut u8,
    n: i16,
) -> *mut u8 {
    unsafe {
        let mut dst = dst;
        let mut src = src;
        let mut n = n;
        'l1: loop {
            if !((((src).read()) as i32) != 255i32) {
                break 'l1;
            }
            ({
                let __t1 = dst;
                dst = (dst).wrapping_offset(1);
                __t1
            })
            .write(
                ({
                    let __t3 = src;
                    src = (src).wrapping_offset(1);
                    __t3
                })
                .read(),
            );
            n = (n).wrapping_sub(1);
        }
        'l2: loop {
            if !((({
                let __t4 = n;
                n = (n).wrapping_sub(1);
                __t4
            }) as i32)
                > 0i32)
            {
                break 'l2;
            }
            ({
                let __t5 = dst;
                dst = (dst).wrapping_offset(1);
                __t5
            })
            .write(0u8);
        }
        (dst).write(255u8);
        return dst;
    }
}
pub(crate) unsafe extern "C" fn CopyConditionMonNameGender(
    str: *mut u8,
    listId: u16,
    skipPadding: u8,
) -> *mut u8 {
    unsafe {
        let mut str = str;
        let mut listId = listId;
        let mut skipPadding = skipPadding;
        let mut boxId: u16 = 0u16;
        let mut monId: u16 = 0u16;
        let mut gender: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut level: u16 = 0u16;
        let mut lvlDigits: u16 = 0u16;
        let mut boxMon: *mut u8 = core::ptr::null_mut();
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut str_: *mut u8 = core::ptr::null_mut();
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        boxId = ((((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((listId) as i32) as isize * 4))
        .read()) as u16);
        monId = (((((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((listId) as i32) as isize * 4))
        .wrapping_add(1))
        .read()) as u16);
        ({
            let __t1 = str;
            str = (str).wrapping_offset(1);
            __t1
        })
        .write(252u8);
        ({
            let __t2 = str;
            str = (str).wrapping_offset(1);
            __t2
        })
        .write(4u8);
        ({
            let __t3 = str;
            str = (str).wrapping_offset(1);
            __t3
        })
        .write(8u8);
        ({
            let __t4 = str;
            str = (str).wrapping_offset(1);
            __t4
        })
        .write(0u8);
        ({
            let __t5 = str;
            str = (str).wrapping_offset(1);
            __t5
        })
        .write(9u8);
        if (GetBoxOrPartyMonData(boxId, monId, 45i32, core::ptr::null_mut())) != 0 {
            return StringCopyPadded(str, (&raw mut gText_EggNickname).cast::<u8>(), 0u8, 12u16);
        }
        GetBoxOrPartyMonData(boxId, monId, 2i32, str);
        StringGet_Nickname(str);
        species = ((GetBoxOrPartyMonData(boxId, monId, 11i32, core::ptr::null_mut())) as u16);
        if ((boxId) as i32) == 14i32 {
            level = ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
                56i32,
            )) as u16);
            gender = ((GetMonGender(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monId) as i32) as isize * 100),
            )) as u16);
        } else {
            boxMon = GetBoxedMonPtr(((boxId) as u8), ((monId) as u8));
            gender = ((GetBoxMonGender(boxMon)) as u16);
            level = ((GetLevelFromBoxMonExp(boxMon)) as u16);
        }
        if ((((species) as i32) == 29i32) || (((species) as i32) == 32i32))
            && (!((StringCompare(
                str,
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            )) != 0))
        {
            gender = 255u16;
        }
        str_ = str;
        'l1: loop {
            if !((((str_).read()) as i32) != 255i32) {
                break 'l1;
            }
            str_ = (str_).wrapping_offset(1);
        }
        ({
            let __t6 = str_;
            str_ = (str_).wrapping_offset(1);
            __t6
        })
        .write(252u8);
        ({
            let __t7 = str_;
            str_ = (str_).wrapping_offset(1);
            __t7
        })
        .write(18u8);
        ({
            let __t8 = str_;
            str_ = (str_).wrapping_offset(1);
            __t8
        })
        .write(60u8);
        'l2: {
            let __sw9 = ((gender) as i32);
            let __matched = __sw9 == 0i32 || __sw9 == 254i32;
            if !__matched {
                ({
                    let __t10 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t10
                })
                .write(119u8);
                break 'l2;
            }
            if __sw9 == 0i32 {
                ({
                    let __t11 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t11
                })
                .write(252u8);
                ({
                    let __t12 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t12
                })
                .write(1u8);
                ({
                    let __t13 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t13
                })
                .write(4u8);
                ({
                    let __t14 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t14
                })
                .write(252u8);
                ({
                    let __t15 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t15
                })
                .write(3u8);
                ({
                    let __t16 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t16
                })
                .write(5u8);
                ({
                    let __t17 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t17
                })
                .write(181u8);
                break 'l2;
            }
            if __sw9 == 254i32 {
                ({
                    let __t18 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t18
                })
                .write(252u8);
                ({
                    let __t19 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t19
                })
                .write(1u8);
                ({
                    let __t20 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t20
                })
                .write(6u8);
                ({
                    let __t21 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t21
                })
                .write(252u8);
                ({
                    let __t22 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t22
                })
                .write(3u8);
                ({
                    let __t23 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t23
                })
                .write(7u8);
                ({
                    let __t24 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t24
                })
                .write(182u8);
                break 'l2;
            }
        }
        ({
            let __t25 = str_;
            str_ = (str_).wrapping_offset(1);
            __t25
        })
        .write(252u8);
        ({
            let __t26 = str_;
            str_ = (str_).wrapping_offset(1);
            __t26
        })
        .write(4u8);
        ({
            let __t27 = str_;
            str_ = (str_).wrapping_offset(1);
            __t27
        })
        .write(8u8);
        ({
            let __t28 = str_;
            str_ = (str_).wrapping_offset(1);
            __t28
        })
        .write(0u8);
        ({
            let __t29 = str_;
            str_ = (str_).wrapping_offset(1);
            __t29
        })
        .write(9u8);
        ({
            let __t30 = str_;
            str_ = (str_).wrapping_offset(1);
            __t30
        })
        .write(186u8);
        ({
            let __t31 = str_;
            str_ = (str_).wrapping_offset(1);
            __t31
        })
        .write(249u8);
        ({
            let __t32 = str_;
            str_ = (str_).wrapping_offset(1);
            __t32
        })
        .write(5u8);
        txtPtr = str_;
        str_ = ConvertIntToDecimalStringN(str_, ((level) as i32), 0i32, 3u8);
        lvlDigits = ((((str_) as usize).wrapping_sub((txtPtr) as usize) as i32 / 1) as u16);
        ({
            let __t33 = str_;
            str_ = (str_).wrapping_offset(1);
            __t33
        })
        .write(0u8);
        if !((skipPadding) != 0) {
            lvlDigits = (((3i32).wrapping_sub(((lvlDigits) as i32))) as u16);
            'l3: loop {
                if !((({
                    let __t34 = lvlDigits;
                    lvlDigits = (lvlDigits).wrapping_sub(1);
                    __t34
                }) as i32)
                    != 0i32)
                {
                    break 'l3;
                }
                ({
                    let __t35 = str_;
                    str_ = (str_).wrapping_offset(1);
                    __t35
                })
                .write(0u8);
            }
        }
        (str_).write(255u8);
        return str_;
    }
}
pub(crate) unsafe extern "C" fn CopyMonNameGenderLocation(listId: i16, loadId: u8) {
    unsafe {
        let mut listId = listId;
        let mut loadId = loadId;
        let mut boxId: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        if ((listId) as i32)
            != (if (IsConditionMenuSearchMode()) != 0 {
                ((((monListPtr).cast::<u16>()).read()) as i32)
            } else {
                ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            })
        {
            CopyConditionMonNameGender(
                ((((menu).wrapping_add(25448)).cast::<u8>())
                    .wrapping_offset(((loadId) as i32) as isize * 64))
                .cast::<u8>(),
                ((listId) as u16),
                0u8,
            );
            boxId = ((((((monListPtr).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((listId) as i32) as isize * 4))
            .read()) as u16);
            (((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .write(252u8);
            ((((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .wrapping_offset(1))
            .write(4u8);
            ((((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .wrapping_offset(2))
            .write(8u8);
            ((((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .wrapping_offset(3))
            .write(0u8);
            ((((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .wrapping_offset(4))
            .write(9u8);
            if ((boxId) as i32) == 14i32 {
                CopyStringLeftAlignedToConditionData(
                    (((((menu).wrapping_add(25376)).cast::<u8>())
                        .wrapping_offset(((loadId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(5),
                    (&raw mut gText_InParty).cast::<u8>(),
                    8i16,
                );
            } else {
                CopyStringLeftAlignedToConditionData(
                    (((((menu).wrapping_add(25376)).cast::<u8>())
                        .wrapping_offset(((loadId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(5),
                    GetBoxNamePtr(((boxId) as u8)),
                    8i16,
                );
            }
        } else {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 12i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((menu).wrapping_add(25448)).cast::<u8>())
                            .wrapping_offset(((loadId) as i32) as isize * 64))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((((menu).wrapping_add(25448)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 64))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .write(255u8);
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((((menu).wrapping_add(25376)).cast::<u8>())
                            .wrapping_offset(((loadId) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((((menu).wrapping_add(25376)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 24))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize))
            .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn InitPartyConditionListParameters() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut count: u16 = 0u16;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = AllocSubstruct(18u32, 1708u32);
        ((menu).wrapping_add(25344)).write(0u8);
        {
            i = 0u16;
            count = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((CalculatePlayerPartyCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        45i32,
                    )) != 0)
                    {
                        ((((monListPtr).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize * 4))
                        .write(14u8);
                        (((((monListPtr).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize * 4))
                        .wrapping_add(1))
                        .write(((i) as u8));
                        (((((monListPtr).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(0u16);
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((count) as i32) as isize * 4))
        .write(0u8);
        (((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((count) as i32) as isize * 4))
        .wrapping_add(1))
        .write(0u8);
        (((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((count) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .write(0u16);
        ((monListPtr).wrapping_add(2).cast::<u16>()).write(0u16);
        ((monListPtr).cast::<u16>()).write(((((count) as i32).wrapping_add(1i32)) as u16));
        ((menu).wrapping_add(26506)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn InitSearchResultsConditionList() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        ((menu).wrapping_add(25344)).write(1u8);
        ((menu).wrapping_add(26506)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetMonConditionGraphData(listId: i16, loadId: u8) {
    unsafe {
        let mut listId = listId;
        let mut loadId = loadId;
        let mut boxId: u16 = 0u16;
        let mut monId: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        if ((listId) as i32)
            != (if (IsConditionMenuSearchMode()) != 0 {
                ((((monListPtr).cast::<u16>()).read()) as i32)
            } else {
                ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            })
        {
            boxId = ((((((monListPtr).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((listId) as i32) as isize * 4))
            .read()) as u16);
            monId = (((((((monListPtr).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((listId) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as u16);
            (((((menu).wrapping_add(25640)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 5))
            .cast::<u8>())
            .write(((GetBoxOrPartyMonData(boxId, monId, 22i32, core::ptr::null_mut())) as u8));
            ((((((menu).wrapping_add(25640)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 5))
            .cast::<u8>())
            .wrapping_offset(1))
            .write(((GetBoxOrPartyMonData(boxId, monId, 47i32, core::ptr::null_mut())) as u8));
            ((((((menu).wrapping_add(25640)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 5))
            .cast::<u8>())
            .wrapping_offset(2))
            .write(((GetBoxOrPartyMonData(boxId, monId, 33i32, core::ptr::null_mut())) as u8));
            ((((((menu).wrapping_add(25640)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 5))
            .cast::<u8>())
            .wrapping_offset(3))
            .write(((GetBoxOrPartyMonData(boxId, monId, 24i32, core::ptr::null_mut())) as u8));
            ((((((menu).wrapping_add(25640)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 5))
            .cast::<u8>())
            .wrapping_offset(4))
            .write(((GetBoxOrPartyMonData(boxId, monId, 23i32, core::ptr::null_mut())) as u8));
            ((((menu).wrapping_add(26496)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize))
            .write(
                ((if GetBoxOrPartyMonData(boxId, monId, 48i32, core::ptr::null_mut()) != 255i32 {
                    crate::c::div_u32(
                        ((GetBoxOrPartyMonData(boxId, monId, 48i32, core::ptr::null_mut())) as u32),
                        (crate::c::div_u32(255u32, 9u32)).wrapping_add(1u32),
                    )
                } else {
                    9u32
                }) as u8),
            );
            ((((menu).wrapping_add(26499)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize))
            .write(((GetBoxOrPartyMonData(boxId, monId, 8i32, core::ptr::null_mut())) as u8));
            ConditionGraph_CalcPositions(
                ((((menu).wrapping_add(25640)).cast::<u8>())
                    .wrapping_offset(((loadId) as i32) as isize * 5))
                .cast::<u8>(),
                (((((menu).wrapping_add(25640)).wrapping_add(20)).cast::<u8>())
                    .wrapping_offset(((loadId) as i32) as isize * 20))
                .cast::<u8>(),
            );
        } else {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((menu).wrapping_add(25640)).cast::<u8>())
                            .wrapping_offset(((loadId) as i32) as isize * 5))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                        ((((((((menu).wrapping_add(25640)).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset(((loadId) as i32) as isize * 20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .write(155u16);
                        ((((((((menu).wrapping_add(25640)).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset(((loadId) as i32) as isize * 20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write((((crate::c::div_i32(177i32, 2i32)).wrapping_add(3i32)) as u16));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConditionGraphDrawMonPic(listId: i16, loadId: u8) {
    unsafe {
        let mut listId = listId;
        let mut loadId = loadId;
        let mut boxId: u16 = 0u16;
        let mut monId: u16 = 0u16;
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        let mut tid: u32 = 0u32;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        if ((listId) as i32)
            == (if (IsConditionMenuSearchMode()) != 0 {
                ((((monListPtr).cast::<u16>()).read()) as i32)
            } else {
                ((((monListPtr).cast::<u16>()).read()) as i32).wrapping_sub(1i32)
            })
        {
            return;
        }
        boxId = ((((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((listId) as i32) as isize * 4))
        .read()) as u16);
        monId = (((((((monListPtr).wrapping_add(4)).cast::<u8>())
            .wrapping_offset(((listId) as i32) as isize * 4))
        .wrapping_add(1))
        .read()) as u16);
        species = ((GetBoxOrPartyMonData(boxId, monId, 65i32, core::ptr::null_mut())) as u16);
        tid = ((GetBoxOrPartyMonData(boxId, monId, 1i32, core::ptr::null_mut())) as u32);
        personality = ((GetBoxOrPartyMonData(boxId, monId, 0i32, core::ptr::null_mut())) as u32);
        LoadSpecialPokePic(
            ((&raw mut gMonFrontPicTable).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 8),
            (((((menu).wrapping_add(768)).cast::<u8>())
                .wrapping_offset(((loadId) as i32) as isize * 8192))
            .cast::<u32>())
            .cast::<u8>(),
            ((species) as i32),
            personality,
            1u8,
        );
        LZ77UnCompWram(
            GetMonSpritePalFromSpeciesAndPersonality(species, tid, personality),
            ((((menu).cast::<u8>()).wrapping_offset(((loadId) as i32) as isize * 128))
                .cast::<u32>())
            .cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonListCount() -> u16 {
    unsafe {
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        return ((monListPtr).cast::<u16>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphCurrentListIndex() -> u16 {
    unsafe {
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        return ((monListPtr).wrapping_add(2).cast::<u16>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphPtr() -> *mut u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return (menu).wrapping_add(25640);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuCurrentLoadIndex() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(26502).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuToLoadListIndex() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(25346).cast::<i16>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonPicGfx(loadId: u8) -> *mut u8 {
    unsafe {
        let mut loadId = loadId;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return (((((menu).wrapping_add(768)).cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 8192))
        .cast::<u32>())
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonPal(loadId: u8) -> *mut u8 {
    unsafe {
        let mut loadId = loadId;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).cast::<u8>()).wrapping_offset(((loadId) as i32) as isize * 128))
            .cast::<u32>())
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionGraphMenuToLoadId() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(26505).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonNameText(loadId: u8) -> *mut u8 {
    unsafe {
        let mut loadId = loadId;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(25448)).cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 64))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonLocationText(loadId: u8) -> *mut u8 {
    unsafe {
        let mut loadId = loadId;
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(25376)).cast::<u8>())
            .wrapping_offset(((loadId) as i32) as isize * 24))
        .cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetConditionMonDataBuffer() -> u16 {
    unsafe {
        let mut monListPtr: *mut u8 = GetSubstructPtr(18u32);
        return (((((monListPtr).wrapping_add(4)).cast::<u8>()).wrapping_offset(
            ((((monListPtr).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 4,
        ))
        .wrapping_add(2)
        .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsConditionMenuSearchMode() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        if ((((menu).wrapping_add(25344)).read()) as i32) == 1i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGetMonMarkId() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        if ((((menu).wrapping_add(25344)).read()) as i32) == 1i32 {
            return ((((menu).wrapping_add(26499)).cast::<u8>()).wrapping_offset(
                ((((menu).wrapping_add(26502).cast::<i8>()).read()) as i32) as isize,
            ))
            .read();
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
pub unsafe extern "C" fn GetNumConditionMonSparkles() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(11u32);
        return ((((menu).wrapping_add(26496)).cast::<u8>()).wrapping_offset(
            ((((menu).wrapping_add(26502).cast::<i8>()).read()) as i32) as isize,
        ))
        .read();
    }
}
