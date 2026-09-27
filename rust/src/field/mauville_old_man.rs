//! Translated from `src/mauville_old_man.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDefaultBardSongLyrics sGiddyAdjectives sGiddyQuestions sStorytellerStories sNumStories sUnused
#[allow(unused_imports)]
use crate::data::mauville_old_man::*;

pub(crate) static mut sSelectedStory: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBardSong: crate::ffi::Align4<[u8; 52]> = crate::ffi::Align4([0; 52]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedPitchTableIndex: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStorytellerPtr: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStorytellerWindowId: u8 = 0u8;

unsafe extern "C" {
    static mut GiddyText_DontYouAgree: u8;
    static mut GiddyText_Is: u8;
    static mut gDisableTextPrinters: u8;
    static mut gGameLanguage: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_Exit: u8;
    static mut gText_Friend: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn CalcWordSounds(a0: *mut u8, a1: u16);
    fn ClearToTransparentAndRemoveWindow(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPixelWidthToTileWidth(a0: i32) -> i32;
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowFromRect(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn EasyChat_GetNumWordsInGroup(a0: u8) -> u16;
    fn FadeInBGM(a0: u8);
    fn FadeOutBGMTemporarily(a0: u8);
    fn GetGameStat(a0: u8) -> u32;
    fn GetRandomEasyChatWordFromUnlockedGroup(a0: u16) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWordSoundTemplates(a0: u16) -> *mut u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsBGMPausedOrStopped() -> u8;
    fn IsStringJapanese(a0: *mut u8) -> u32;
    fn Menu_ProcessInput() -> i8;
    fn Random() -> u16;
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn ScriptContext_Enable();
    fn ScriptContext_Stop();
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TraderSetup();
    fn Trader_ResetFlag();
    fn UnlockRandomTrendySaying() -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn m4aMPlayFadeOutTemporarily(a0: *mut u8, a1: u16);
    fn m4aMPlayPitchControl(a0: *mut u8, a1: u16, a2: i16);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
    fn m4aSongNumStart(a0: u16);
}

pub(crate) unsafe extern "C" fn SetupBard() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut bard: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        (bard).write(0u8);
        ((bard).wrapping_add(41)).write(0u8);
        ((bard).wrapping_add(42)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((bard).wrapping_add(2)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((&raw const sDefaultBardSongLyrics)
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
    }
}
pub(crate) unsafe extern "C" fn SetupHipster() {
    unsafe {
        let mut hipster: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        (hipster).write(1u8);
        ((hipster).wrapping_add(1)).write(0u8);
        ((hipster).wrapping_add(2)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn SetupStoryteller() {
    unsafe {
        StorytellerSetup();
    }
}
pub(crate) unsafe extern "C" fn SetupGiddy() {
    unsafe {
        let mut giddy: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        (giddy).write(4u8);
        ((giddy).wrapping_add(1)).write(0u8);
        ((giddy).wrapping_add(32)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn SetupTrader() {
    unsafe {
        TraderSetup();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMauvilleOldMan() {
    unsafe {
        let mut trainerId: u16 = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            << 8)
            | (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .read()) as i32)) as u16);
        'l1: {
            let __sw1 = crate::c::div_i32(crate::c::rem_i32(((trainerId) as i32), 10i32), 2i32);
            if __sw1 == 0i32 {
                SetupBard();
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetupHipster();
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetupTrader();
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetupStoryteller();
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetupGiddy();
                break 'l1;
            }
        }
        SetMauvilleOldManObjEventGfx();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMauvilleOldMan() -> u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_GetCurrentMauvilleMan() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetCurrentMauvilleOldMan()) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasBardSongBeenChanged() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816))
                .wrapping_add(41))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveBardSongLyrics() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut bard: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        StringCopy(
            ((bard).wrapping_add(26)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((bard).wrapping_add(37)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
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
                    ((((bard).wrapping_add(2)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((bard).wrapping_add(14)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((bard).wrapping_add(41)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn PrepareSongText() {
    unsafe {
        let mut bard: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        let mut lyrics: *mut u16 =
            (if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                ((bard).wrapping_add(2)).cast::<u16>()
            } else {
                ((bard).wrapping_add(14)).cast::<u16>()
            });
        let mut wordEnd: *mut u8 = (&raw mut gStringVar4).cast::<u8>();
        let mut str: *mut u8 = wordEnd;
        let mut paragraphNum: u16 = 0u16;
        {
            paragraphNum = 0u16;
            'l1: loop {
                if !(((paragraphNum) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    wordEnd = CopyEasyChatWord(
                        wordEnd,
                        ({
                            let __t2 = lyrics;
                            lyrics = (lyrics).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                    'l3: loop {
                        if !(((wordEnd) as usize) != ((str) as usize)) {
                            break 'l3;
                        }
                        if (((str).read()) as i32) == 0i32 {
                            (str).write(55u8);
                        }
                        str = (str).wrapping_offset(1);
                    }
                    str = (str).wrapping_offset(1);
                    ({
                        let __t3 = wordEnd;
                        wordEnd = (wordEnd).wrapping_offset(1);
                        __t3
                    })
                    .write(0u8);
                    wordEnd = CopyEasyChatWord(
                        wordEnd,
                        ({
                            let __t5 = lyrics;
                            lyrics = (lyrics).wrapping_offset(1);
                            __t5
                        })
                        .read(),
                    );
                    'l4: loop {
                        if !(((wordEnd) as usize) != ((str) as usize)) {
                            break 'l4;
                        }
                        if (((str).read()) as i32) == 0i32 {
                            (str).write(55u8);
                        }
                        str = (str).wrapping_offset(1);
                    }
                    str = (str).wrapping_offset(1);
                    ({
                        let __t6 = wordEnd;
                        wordEnd = (wordEnd).wrapping_offset(1);
                        __t6
                    })
                    .write(254u8);
                    wordEnd = CopyEasyChatWord(
                        wordEnd,
                        ({
                            let __t8 = lyrics;
                            lyrics = (lyrics).wrapping_offset(1);
                            __t8
                        })
                        .read(),
                    );
                    'l5: loop {
                        if !(((wordEnd) as usize) != ((str) as usize)) {
                            break 'l5;
                        }
                        if (((str).read()) as i32) == 0i32 {
                            (str).write(55u8);
                        }
                        str = (str).wrapping_offset(1);
                    }
                    if ((paragraphNum) as i32) == 0i32 {
                        ({
                            let __t9 = wordEnd;
                            wordEnd = (wordEnd).wrapping_offset(1);
                            __t9
                        })
                        .write(252u8);
                        ({
                            let __t10 = wordEnd;
                            wordEnd = (wordEnd).wrapping_offset(1);
                            __t10
                        })
                        .write(15u8);
                    }
                }
                paragraphNum = (paragraphNum).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayBardSong() {
    unsafe {
        StartBardSong(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        ScriptContext_Stop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasHipsterTaughtWord() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816))
                .wrapping_add(1))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHipsterTaughtWord() {
    unsafe {
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816))
            .wrapping_add(1))
        .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HipsterTryTeachWord() {
    unsafe {
        let mut word: u16 = UnlockRandomTrendySaying();
        if ((word) as i32) == 65535i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            CopyEasyChatWord((&raw mut gStringVar1).cast::<u8>(), word);
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiddyShouldTellAnotherTale() {
    unsafe {
        let mut giddy: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        if ((((giddy).wrapping_add(1)).read()) as i32) == 10i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            ((giddy).wrapping_add(1)).write(0u8);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateGiddyLine() {
    unsafe {
        let mut giddy: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        if ((((giddy).wrapping_add(1)).read()) as i32) == 0i32 {
            InitGiddyTaleList();
        }
        if ((((((giddy).wrapping_add(4)).cast::<u16>())
            .wrapping_offset(((((giddy).wrapping_add(1)).read()) as i32) as isize))
        .read()) as i32)
            != 65535i32
        {
            let mut stringPtr: *mut u8 = core::ptr::null_mut();
            let mut adjective: u32 = ((Random()) as u32);
            adjective = crate::c::rem_u32(adjective, crate::c::div_u32(32u32, 4u32));
            stringPtr = CopyEasyChatWord(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((giddy).wrapping_add(4)).cast::<u16>())
                    .wrapping_offset(((((giddy).wrapping_add(1)).read()) as i32) as isize))
                .read(),
            );
            stringPtr = StringCopy(stringPtr, (&raw mut GiddyText_Is).cast::<u8>());
            stringPtr = StringCopy(
                stringPtr,
                ((((&raw const sGiddyAdjectives)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((adjective) as i32) as isize))
                .read(),
            );
            StringCopy(stringPtr, (&raw mut GiddyText_DontYouAgree).cast::<u8>());
        } else {
            StringCopy(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sGiddyQuestions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((giddy).wrapping_add(24)).cast::<u8>()).wrapping_offset(
                        (({
                            let __p1 = (giddy).wrapping_add(2);
                            let __t2 = (__p1).read();
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            __t2
                        }) as i32) as isize,
                    ))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
        }
        if !((crate::c::rem_i32(((Random()) as i32), 10i32)) != 0) {
            ((giddy).wrapping_add(1)).write(10u8);
        } else {
            let __p3 = (giddy).wrapping_add(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
    }
}
pub(crate) unsafe extern "C" fn InitGiddyTaleList() {
    unsafe {
        let mut giddy: *mut u8 =
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816));
        let mut wordGroupsAndCount = crate::ffi::Align4([0u8; 24]);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<u16>()
            .write(12u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<u16>()
            .write(13u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(12)
            .wrapping_add(0)
            .cast::<u16>()
            .write(18u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(12)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(0)
            .cast::<u16>()
            .write(19u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(16)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(20)
            .wrapping_add(0)
            .cast::<u16>()
            .write(21u16);
        (&raw mut wordGroupsAndCount)
            .cast::<u8>()
            .wrapping_add(20)
            .wrapping_add(2)
            .cast::<u16>()
            .write(0u16);
        let mut i: u16 = 0u16;
        let mut totalWords: u16 = 0u16;
        let mut temp: u16 = 0u16;
        let mut var: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((giddy).wrapping_add(24)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l3;
                }
                'l4: {
                    var = ((crate::c::rem_i32(((Random()) as i32), ((i) as i32).wrapping_add(1i32)))
                        as u16);
                    {
                        temp = ((((((giddy).wrapping_add(24)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as u16);
                        ((((giddy).wrapping_add(24)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((giddy).wrapping_add(24)).cast::<u8>())
                                .wrapping_offset(((var) as i32) as isize))
                            .read(),
                        );
                        ((((giddy).wrapping_add(24)).cast::<u8>())
                            .wrapping_offset(((var) as i32) as isize))
                        .write(((temp) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        totalWords = 0u16;
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(24u32, 4u32)) {
                    break 'l5;
                }
                'l6: {
                    (((((&raw mut wordGroupsAndCount).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(EasyChat_GetNumWordsInGroup(
                        ((((((&raw mut wordGroupsAndCount).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read()) as u8),
                    ));
                    totalWords = ((((totalWords) as i32).wrapping_add(
                        (((((((&raw mut wordGroupsAndCount).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((giddy).wrapping_add(2)).write(0u8);
        temp = 0u16;
        {
            i = 0u16;
            'l7: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l7;
                }
                'l8: {
                    var = ((crate::c::rem_i32(((Random()) as i32), 10i32)) as u16);
                    if (((var) as i32) < 3i32) && (((temp) as i32) < 8i32) {
                        ((((giddy).wrapping_add(4)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(65535u16);
                        temp = (temp).wrapping_add(1);
                    } else {
                        let mut randWord: i16 =
                            ((crate::c::rem_i32(((Random()) as i32), ((totalWords) as i32)))
                                as i16);
                        {
                            var = 0u16;
                            'l9: loop {
                                if !(((i) as u32) < crate::c::div_u32(24u32, 4u32)) {
                                    break 'l9;
                                }
                                'l10: {
                                    if (({
                                        let __v1 = ((((randWord) as i32).wrapping_sub(
                                            (((((((&raw mut wordGroupsAndCount).cast::<u8>())
                                                .wrapping_offset(((var) as i32) as isize * 4))
                                            .cast::<u16>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32),
                                        ))
                                            as i16);
                                        randWord = __v1;
                                        __v1
                                    }) as i32)
                                        <= 0i32
                                    {
                                        break 'l9;
                                    }
                                }
                                var = (var).wrapping_add(1);
                            }
                        }
                        if ((var) as u32) == crate::c::div_u32(24u32, 4u32) {
                            var = 0u16;
                        }
                        ((((giddy).wrapping_add(4)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(GetRandomEasyChatWordFromUnlockedGroup(
                            ((((&raw mut wordGroupsAndCount).cast::<u8>())
                                .wrapping_offset(((var) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                        ));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetBardFlag() {
    unsafe {
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816))
            .wrapping_add(41))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn ResetHipsterFlag() {
    unsafe {
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816))
            .wrapping_add(1))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn ResetTraderFlag() {
    unsafe {
        Trader_ResetFlag();
    }
}
pub(crate) unsafe extern "C" fn ResetStorytellerFlag() {
    unsafe {
        Storyteller_ResetFlag();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetMauvilleOldManFlag() {
    unsafe {
        'l1: {
            let __sw1 = ((GetCurrentMauvilleOldMan()) as i32);
            if __sw1 == 0i32 {
                ResetBardFlag();
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetHipsterFlag();
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetStorytellerFlag();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetTraderFlag();
                break 'l1;
            }
            if __sw1 == 4i32 {
                break 'l1;
            }
        }
        SetMauvilleOldManObjEventGfx();
    }
}
pub(crate) unsafe extern "C" fn StartBardSong(useNewSongLyrics: u8) {
    unsafe {
        let mut useNewSongLyrics = useNewSongLyrics;
        let mut taskId: u8 = CreateTask(Some(Task_BardSong), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((useNewSongLyrics) as i16));
    }
}
pub(crate) unsafe extern "C" fn EnableTextPrinters() {
    unsafe {
        ((&raw mut gDisableTextPrinters).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DisableTextPrinters(printer: *mut u8, renderCmd: u16) {
    unsafe {
        let mut printer = printer;
        let mut renderCmd = renderCmd;
        ((&raw mut gDisableTextPrinters).cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn DrawSongTextWindow(str: *mut u8) {
    unsafe {
        let mut str = str;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(0u8, 1u8, str, 0u8, 1u8, 1u8, Some(DisableTextPrinters));
        ((&raw mut gDisableTextPrinters).cast::<u8>()).write(1u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn BardSing(task: *mut u8, song: *mut u8) {
    unsafe {
        let mut task = task;
        let mut song = song;
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                {
                    let mut bard: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(11816));
                    let mut lyrics: *mut u16 = core::ptr::null_mut();
                    let mut i: i32 = 0i32;
                    if !((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) != 0) {
                        lyrics = ((bard).wrapping_add(2)).cast::<u16>();
                    } else {
                        lyrics = ((bard).wrapping_add(14)).cast::<u16>();
                    }
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 6i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((song).wrapping_add(12)).cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                .write(((lyrics).wrapping_offset((i) as isize)).read());
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    (song).write(0u8);
                    break 'l1;
                }
            }
            if __sw1 == 2i32 {
                {
                    let mut easyChatWord: u16 = ((((song).wrapping_add(12)).cast::<u16>())
                        .wrapping_offset((((song).read()) as i32) as isize))
                    .read();
                    ((song).wrapping_add(48).cast::<*mut u8>())
                        .write(GetWordSoundTemplates(easyChatWord));
                    CalcWordSounds(
                        song,
                        (((if (0i32) != 0 {
                            crate::c::rem_i32(((easyChatWord) as i32), 4i32)
                        } else {
                            (((easyChatWord) as i32) & 3i32)
                        })
                        .wrapping_add(((((easyChatWord) as i32) >> 3) & 1i32)))
                            as u16),
                    );
                    let __p2 = (song);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    if (((((song).wrapping_add(48).cast::<*mut u8>()).read()).read()) as i32)
                        != 255i32
                    {
                        ((song).wrapping_add(3)).write(0u8);
                    } else {
                        ((song).wrapping_add(3)).write(3u8);
                        ((song).wrapping_add(2)).write(2u8);
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3i32 || __sw1 == 4i32 {
                {
                    let mut template: *mut u8 = (((song).wrapping_add(48).cast::<*mut u8>())
                        .read())
                    .wrapping_offset(((((song).wrapping_add(1)).read()) as i32) as isize * 8);
                    'l4: {
                        let __sw3 = ((((song).wrapping_add(3)).read()) as i32);
                        if __sw3 == 0i32 {
                            ((song).wrapping_add(2)).write(
                                (((((((song).wrapping_add(24)).cast::<u8>()).wrapping_offset(
                                    ((((song).wrapping_add(1)).read()) as i32) as isize * 4,
                                ))
                                .cast::<u16>())
                                .read()) as u8),
                            );
                            if (((template).read()) as i32) < 51i32 {
                                let mut phonemeTripletId: u8 =
                                    ((crate::c::div_i32((((template).read()) as i32), 3i32)) as u8);
                                m4aSongNumStart(
                                    (((560i32).wrapping_add(
                                        ((phonemeTripletId) as i32).wrapping_mul(3i32),
                                    )) as u16),
                                );
                            }
                            ((song).wrapping_add(3)).write(2u8);
                            let __p4 = (song).wrapping_add(2);
                            (__p4).write(((__p4).read()).wrapping_sub(1));
                            break 'l4;
                        }
                        if __sw3 == 2i32 {
                            ((song).wrapping_add(3)).write(1u8);
                            if (((template).read()) as i32) < 51i32 {
                                ((song).wrapping_add(6).cast::<u16>()).write(
                                    (((256i32).wrapping_add(
                                        ((((template).wrapping_add(4).cast::<i16>()).read())
                                            as i32)
                                            .wrapping_mul(16i32),
                                    )) as u16),
                                );
                                m4aMPlayVolumeControl(
                                    (&raw mut gMPlayInfo_SE2).cast::<u8>(),
                                    65535u16,
                                    ((song).wrapping_add(6).cast::<u16>()).read(),
                                );
                                ((song).wrapping_add(8).cast::<i16>()).write(
                                    (((512i32).wrapping_add(
                                        (((((((song).wrapping_add(24)).cast::<u8>())
                                            .wrapping_offset(
                                                ((((song).wrapping_add(1)).read()) as i32) as isize
                                                    * 4,
                                            ))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .read()) as i32),
                                    )) as i16),
                                );
                                m4aMPlayPitchControl(
                                    (&raw mut gMPlayInfo_SE2).cast::<u8>(),
                                    65535u16,
                                    ((song).wrapping_add(8).cast::<i16>()).read(),
                                );
                            }
                            break 'l4;
                        }
                        if __sw3 == 1i32 {
                            if ((((song).wrapping_add(10).cast::<i16>()).read()) as i32) > 10i32 {
                                let __p5 = (song).wrapping_add(6).cast::<u16>();
                                (__p5)
                                    .write((((((__p5).read()) as i32).wrapping_sub(2i32)) as u16));
                            }
                            if (((((song).wrapping_add(10).cast::<i16>()).read()) as i32) & 1i32)
                                != 0
                            {
                                let __p6 = (song).wrapping_add(8).cast::<i16>();
                                (__p6)
                                    .write((((((__p6).read()) as i32).wrapping_add(64i32)) as i16));
                            } else {
                                let __p7 = (song).wrapping_add(8).cast::<i16>();
                                (__p7)
                                    .write((((((__p7).read()) as i32).wrapping_sub(64i32)) as i16));
                            }
                            m4aMPlayVolumeControl(
                                (&raw mut gMPlayInfo_SE2).cast::<u8>(),
                                65535u16,
                                ((song).wrapping_add(6).cast::<u16>()).read(),
                            );
                            m4aMPlayPitchControl(
                                (&raw mut gMPlayInfo_SE2).cast::<u8>(),
                                65535u16,
                                ((song).wrapping_add(8).cast::<i16>()).read(),
                            );
                            let __p8 = (song).wrapping_add(10).cast::<i16>();
                            (__p8).write(((__p8).read()).wrapping_add(1));
                            let __p9 = (song).wrapping_add(2);
                            (__p9).write(((__p9).read()).wrapping_sub(1));
                            if ((((song).wrapping_add(2)).read()) as i32) == 0i32 {
                                if ((({
                                    let __p10 = (song).wrapping_add(1);
                                    let __t11 = ((__p10).read()).wrapping_add(1);
                                    (__p10).write(__t11);
                                    __t11
                                }) as i32)
                                    != 6i32)
                                    && (((((((song).wrapping_add(48).cast::<*mut u8>()).read())
                                        .wrapping_offset(
                                            ((((song).wrapping_add(1)).read()) as i32) as isize * 8,
                                        ))
                                    .read()) as i32)
                                        != 255i32)
                                {
                                    ((song).wrapping_add(3)).write(0u8);
                                } else {
                                    ((song).wrapping_add(3)).write(3u8);
                                    ((song).wrapping_add(2)).write(2u8);
                                }
                            }
                            break 'l4;
                        }
                        if __sw3 == 3i32 {
                            if (({
                                let __p12 = (song).wrapping_add(2);
                                let __t13 = ((__p12).read()).wrapping_sub(1);
                                (__p12).write(__t13);
                                __t13
                            }) as i32)
                                == 0i32
                            {
                                m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
                                ((song).wrapping_add(3)).write(4u8);
                            }
                            break 'l4;
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 5i32 || __sw1 == 1i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BardSong(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        BardSing(task, (&raw mut gBardSong).cast::<u8>());
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                PrepareSongText();
                DrawSongTextWindow((&raw mut gStringVar4).cast::<u8>());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                FadeOutBGMTemporarily(4u8);
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsBGMPausedOrStopped()) != 0 {
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    let mut bard: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                        .read())
                    .wrapping_add(11816));
                    let mut str: *mut u8 = ((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32) as isize,
                    );
                    let mut wordLen: u16 = 0u16;
                    'l2: loop {
                        if !(((((((str).read()) as i32) != 0i32)
                            && ((((str).read()) as i32) != 254i32))
                            && ((((str).read()) as i32) != 252i32))
                            && ((((str).read()) as i32) != 255i32))
                        {
                            break 'l2;
                        }
                        str = (str).wrapping_offset(1);
                        wordLen = (wordLen).wrapping_add(1);
                    }
                    if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        != 0)
                    {
                        ((&raw mut sUnusedPitchTableIndex).cast::<u8>().cast::<u16>()).write(
                            (((if (0i32) != 0 {
                                crate::c::rem_i32(
                                    ((((((bard).wrapping_add(2)).cast::<u16>()).wrapping_offset(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(4))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32),
                                    4i32,
                                )
                            } else {
                                (((((((bard).wrapping_add(2)).cast::<u16>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    & 3i32)
                            })
                            .wrapping_add(
                                ((((((((bard).wrapping_add(2)).cast::<u16>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 3)
                                    & 1i32),
                            )) as u16),
                        );
                    } else {
                        ((&raw mut sUnusedPitchTableIndex).cast::<u8>().cast::<u16>()).write(
                            (((if (0i32) != 0 {
                                crate::c::rem_i32(
                                    ((((((bard).wrapping_add(14)).cast::<u16>()).wrapping_offset(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(4))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()) as i32),
                                    4i32,
                                )
                            } else {
                                (((((((bard).wrapping_add(14)).cast::<u16>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    & 3i32)
                            })
                            .wrapping_add(
                                ((((((((bard).wrapping_add(14)).cast::<u16>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    >> 3)
                                    & 1i32),
                            )) as u16),
                        );
                    }
                    let __p2 = ((&raw mut gBardSong).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i16>();
                    (__p2).write(
                        ((crate::c::div_i32((((__p2).read()) as i32), ((wordLen) as i32))) as i16),
                    );
                    if (((((&raw mut gBardSong).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i16>())
                    .read()) as i32)
                        <= 0i32
                    {
                        (((&raw mut gBardSong).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<i16>())
                        .write(1i16);
                    }
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 0i32
                    {
                        (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    } else {
                        (((task).wrapping_add(8)).cast::<i16>()).write(5i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 0i32
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                } else {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p4).write(((__p4).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    == 255i32
                {
                    FadeInBGM(6u8);
                    m4aMPlayFadeOutTemporarily((&raw mut gMPlayInfo_SE2).cast::<u8>(), 2u16);
                    ScriptContext_Enable();
                    DestroyTask(taskId);
                } else {
                    if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        == 0i32
                    {
                        EnableTextPrinters();
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    } else {
                        if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                                as i32) as isize,
                        ))
                        .read()) as i32)
                            == 254i32
                        {
                            let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                            (__p6).write(((__p6).read()).wrapping_add(1));
                            (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .write(0i16);
                        } else {
                            if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                    .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                == 252i32
                            {
                                let __p7 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                                (__p7)
                                    .write((((((__p7).read()) as i32).wrapping_add(2i32)) as i16));
                                (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .write(8i16);
                            } else {
                                if (((((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    == 55i32
                                {
                                    (((&raw mut gStringVar4).cast::<u8>()).wrapping_offset(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(3))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .write(0u8);
                                    EnableTextPrinters();
                                    let __p8 =
                                        (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                                    (__p8).write(((__p8).read()).wrapping_add(1));
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                        .write(0i16);
                                } else {
                                    'l3: {
                                        let __sw9 = ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(1))
                                        .read())
                                            as i32);
                                        if __sw9 == 0i32 {
                                            EnableTextPrinters();
                                            let __p10 = (((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(1);
                                            (__p10).write(((__p10).read()).wrapping_add(1));
                                            break 'l3;
                                        }
                                        if __sw9 == 1i32 {
                                            let __p11 = (((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(1);
                                            (__p11).write(((__p11).read()).wrapping_add(1));
                                            break 'l3;
                                        }
                                        if __sw9 == 2i32 {
                                            let __p12 = (((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(3);
                                            (__p12).write(((__p12).read()).wrapping_add(1));
                                            ((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(1))
                                            .write(0i16);
                                            ((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(2))
                                            .write(
                                                (((&raw mut gBardSong).cast::<u8>())
                                                    .wrapping_add(4)
                                                    .cast::<i16>())
                                                .read(),
                                            );
                                            (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                                            break 'l3;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p13).write(((__p13).read()).wrapping_sub(1));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 0i32
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                }
                break 'l1;
            }
        }
        RunTextPrintersAndIsPrinter0Active();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMauvilleOldManObjEventGfx() {
    unsafe {
        VarSet(16400u16, 69u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeMauvilleOldManForRuby(oldMan: *mut u8) {
    unsafe {
        let mut oldMan = oldMan;
        let mut i: i32 = 0i32;
        let mut playerName = crate::ffi::Align4([0u8; 8]);
        'l1: {
            let __sw1 = (((oldMan).read()) as i32);
            if __sw1 == 2i32 {
                {
                    let mut trader: *mut u8 = (oldMan);
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                if ((((((trader).wrapping_add(50)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == 1i32
                                {
                                    ConvertInternationalString(
                                        ((((trader).wrapping_add(5)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 11))
                                        .cast::<u8>(),
                                        1u8,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 3i32 {
                {
                    let mut storyteller: *mut u8 = (oldMan);
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                if ((((((storyteller).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    != 0i32
                                {
                                    crate::c::memcpy(
                                        (&raw mut playerName).cast::<u8>(),
                                        ((((storyteller).wrapping_add(8)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 7))
                                        .cast::<u8>(),
                                        7u32,
                                    );
                                    (((&raw mut playerName).cast::<u8>()).wrapping_offset(7))
                                        .write(255u8);
                                    if (IsStringJapanese((&raw mut playerName).cast::<u8>())) != 0 {
                                        crate::c::memset(
                                            (&raw mut playerName).cast::<u8>(),
                                            0i32,
                                            8u32,
                                        );
                                        StringCopy(
                                            (&raw mut playerName).cast::<u8>(),
                                            (&raw mut gText_Friend).cast::<u8>(),
                                        );
                                        crate::c::memcpy(
                                            ((((storyteller).wrapping_add(8)).cast::<u8>())
                                                .wrapping_offset((i) as isize * 7))
                                            .cast::<u8>(),
                                            (&raw mut playerName).cast::<u8>(),
                                            7u32,
                                        );
                                        ((((storyteller).wrapping_add(52)).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .write(2u8);
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetMauvilleOldManLanguage(
    oldMan: *mut u8,
    language1: u32,
    language2: u32,
    language3: u32,
) {
    unsafe {
        let mut oldMan = oldMan;
        let mut language1 = language1;
        let mut language2 = language2;
        let mut language3 = language3;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((oldMan).read()) as i32);
            if __sw1 == 2i32 {
                {
                    let mut trader: *mut u8 = (oldMan);
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                if (IsStringJapanese(
                                    ((((trader).wrapping_add(5)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 11))
                                    .cast::<u8>(),
                                )) != 0
                                {
                                    ((((trader).wrapping_add(50)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(((language1) as u8));
                                } else {
                                    ((((trader).wrapping_add(50)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(((language2) as u8));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut storyteller: *mut u8 = (oldMan);
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                if (IsStringJapanese(
                                    ((((storyteller).wrapping_add(8)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 7))
                                    .cast::<u8>(),
                                )) != 0
                                {
                                    ((((storyteller).wrapping_add(52)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(((language1) as u8));
                                } else {
                                    ((((storyteller).wrapping_add(52)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .write(((language2) as u8));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                {
                    let mut bard: *mut u8 = (oldMan);
                    if language3 == 1u32 {
                        ((bard).wrapping_add(42)).write(((language1) as u8));
                    } else {
                        ((bard).wrapping_add(42)).write(((language2) as u8));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    let mut hipster: *mut u8 = (oldMan);
                    if language3 == 1u32 {
                        ((hipster).wrapping_add(2)).write(((language1) as u8));
                    } else {
                        ((hipster).wrapping_add(2)).write(((language2) as u8));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    let mut giddy: *mut u8 = (oldMan);
                    if language3 == 1u32 {
                        ((giddy).wrapping_add(32)).write(((language1) as u8));
                    } else {
                        ((giddy).wrapping_add(32)).write(((language2) as u8));
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeReceivedEmeraldOldMan(
    oldMan: *mut u8,
    version: u32,
    language: u32,
) {
    unsafe {
        let mut oldMan = oldMan;
        let mut version = version;
        let mut language = language;
        let mut playerName = crate::ffi::Align4([0u8; 8]);
        let mut i: i32 = 0i32;
        if ((((oldMan).read()) as i32) == 3i32) && (language == 1u32) {
            let mut storyteller: *mut u8 = (oldMan);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((storyteller).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            crate::c::memcpy(
                                (&raw mut playerName).cast::<u8>(),
                                ((((storyteller).wrapping_add(8)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 7))
                                .cast::<u8>(),
                                7u32,
                            );
                            (((&raw mut playerName).cast::<u8>()).wrapping_offset(7)).write(255u8);
                            if (IsStringJapanese((&raw mut playerName).cast::<u8>())) != 0 {
                                ((((storyteller).wrapping_add(52)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(1u8);
                            } else {
                                ((((storyteller).wrapping_add(52)).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .write(2u8);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeReceivedRubyOldMan(oldMan: *mut u8, version: u32, language: u32) {
    unsafe {
        let mut oldMan = oldMan;
        let mut version = version;
        let mut language = language;
        let mut isRuby: u32 = (((version == 1u32) || (version == 2u32)) as u32);
        'l1: {
            let __sw1 = (((oldMan).read()) as i32);
            if __sw1 == 2i32 {
                {
                    let mut trader: *mut u8 = (oldMan);
                    let mut i: i32 = 0i32;
                    if (isRuby) != 0 {
                        {
                            i = 0i32;
                            'l2: loop {
                                if !(i < 4i32) {
                                    break 'l2;
                                }
                                'l3: {
                                    let mut str: *mut u8 = ((((trader).wrapping_add(5))
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize * 11))
                                    .cast::<u8>();
                                    if ((((str).read()) as i32) == 252i32)
                                        && (((((str).wrapping_offset(1)).read()) as i32) == 21i32)
                                    {
                                        StripExtCtrlCodes(str);
                                        ((((trader).wrapping_add(50)).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .write(1u8);
                                    } else {
                                        ((((trader).wrapping_add(50)).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .write(((language) as u8));
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    } else {
                        {
                            i = 0i32;
                            'l4: loop {
                                if !(i < 4i32) {
                                    break 'l4;
                                }
                                'l5: {
                                    if ((((((trader).wrapping_add(50)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        == 1i32
                                    {
                                        StripExtCtrlCodes(
                                            ((((trader).wrapping_add(5)).cast::<u8>())
                                                .wrapping_offset((i) as isize * 11))
                                            .cast::<u8>(),
                                        );
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut storyteller: *mut u8 = (oldMan);
                    let mut i: i32 = 0i32;
                    if (isRuby) != 0 {
                        {
                            i = 0i32;
                            'l6: loop {
                                if !(i < 4i32) {
                                    break 'l6;
                                }
                                'l7: {
                                    if ((((((storyteller).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        ((((storyteller).wrapping_add(52)).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .write(((language) as u8));
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                {
                    let mut bard: *mut u8 = (oldMan);
                    if (isRuby) != 0 {
                        ((bard).wrapping_add(42)).write(((language) as u8));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    let mut hipster: *mut u8 = (oldMan);
                    if (isRuby) != 0 {
                        ((hipster).wrapping_add(2)).write(((language) as u8));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    let mut giddy: *mut u8 = (oldMan);
                    if (isRuby) != 0 {
                        ((giddy).wrapping_add(32)).write(((language) as u8));
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StorytellerSetup() {
    unsafe {
        let mut i: i32 = 0i32;
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        (((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).write(3u8);
        ((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(0u8);
                    (((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .cast::<u8>())
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Storyteller_ResetFlag() {
    unsafe {
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        (((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).write(3u8);
        ((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn StorytellerGetGameStat(stat: u8) -> u32 {
    unsafe {
        let mut stat = stat;
        if ((stat) as i32) == 50i32 {
            stat = 0u8;
        }
        return GetGameStat(stat);
    }
}
pub(crate) unsafe extern "C" fn GetStoryByStat(stat: u32) -> *mut u8 {
    unsafe {
        let mut stat = stat;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((&raw const sNumStories)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<i32>())
                    .read())
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sStorytellerStories).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 16))
                    .read()) as u32)
                        == stat
                    {
                        return (((&raw const sStorytellerStories).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (((&raw const sStorytellerStories).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((&raw const sNumStories)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i32>())
                .read())
                .wrapping_sub(1i32)) as isize
                    * 16,
            );
    }
}
pub(crate) unsafe extern "C" fn GetStoryTitleByStat(stat: u32) -> *mut u8 {
    unsafe {
        let mut stat = stat;
        return ((GetStoryByStat(stat)).wrapping_add(4).cast::<*mut u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetStoryTextByStat(stat: u32) -> *mut u8 {
    unsafe {
        let mut stat = stat;
        return ((GetStoryByStat(stat)).wrapping_add(12).cast::<*mut u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetStoryActionByStat(stat: u32) -> *mut u8 {
    unsafe {
        let mut stat = stat;
        return ((GetStoryByStat(stat)).wrapping_add(8).cast::<*mut u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetFreeStorySlot() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return i;
    }
}
pub(crate) unsafe extern "C" fn StorytellerGetRecordedTrainerStat(trainer: u32) -> u32 {
    unsafe {
        let mut trainer = trainer;
        let mut ptr: *mut u8 = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(36))
        .cast::<u8>())
        .wrapping_offset(((trainer) as i32) as isize * 4))
        .cast::<u8>();
        return (((((((ptr).read()) as i32) | (((((ptr).wrapping_offset(1)).read()) as i32) << 8))
            | (((((ptr).wrapping_offset(2)).read()) as i32) << 16))
            | (((((ptr).wrapping_offset(3)).read()) as i32) << 24)) as u32);
    }
}
pub(crate) unsafe extern "C" fn StorytellerSetRecordedTrainerStat(trainer: u32, val: u32) {
    unsafe {
        let mut trainer = trainer;
        let mut val = val;
        let mut ptr: *mut u8 = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(36))
        .cast::<u8>())
        .wrapping_offset(((trainer) as i32) as isize * 4))
        .cast::<u8>();
        (ptr).write(((val) as u8));
        ((ptr).wrapping_offset(1)).write(((val >> 8) as u8));
        ((ptr).wrapping_offset(2)).write(((val >> 16) as u8));
        ((ptr).wrapping_offset(3)).write(((val >> 24) as u8));
    }
}
pub(crate) unsafe extern "C" fn HasTrainerStatIncreased(trainer: u32) -> u32 {
    unsafe {
        let mut trainer = trainer;
        if StorytellerGetGameStat(
            ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .cast::<u8>())
            .wrapping_offset(((trainer) as i32) as isize))
            .read(),
        ) > StorytellerGetRecordedTrainerStat(trainer)
        {
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
pub(crate) unsafe extern "C" fn GetStoryByStattellerPlayerName(player: u32, dst: *mut u8) {
    unsafe {
        let mut player = player;
        let mut dst = dst;
        let mut name: *mut u8 = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(((player) as i32) as isize * 7))
        .cast::<u8>();
        crate::c::memset(dst, 255i32, 8u32);
        crate::c::memcpy(dst, name, 7u32);
    }
}
pub(crate) unsafe extern "C" fn StorytellerSetPlayerName(player: u32, src: *mut u8) {
    unsafe {
        let mut player = player;
        let mut src = src;
        let mut name: *mut u8 = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(8))
        .cast::<u8>())
        .wrapping_offset(((player) as i32) as isize * 7))
        .cast::<u8>();
        crate::c::memset(name, 255i32, 7u32);
        crate::c::memcpy(name, src, 7u32);
    }
}
pub(crate) unsafe extern "C" fn StorytellerRecordNewStat(player: u32, stat: u32) {
    unsafe {
        let mut player = player;
        let mut stat = stat;
        ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<u8>())
        .wrapping_offset(((player) as i32) as isize))
        .write(((stat) as u8));
        StorytellerSetPlayerName(
            player,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        StorytellerSetRecordedTrainerStat(player, StorytellerGetGameStat(((stat) as u8)));
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((StorytellerGetGameStat(((stat) as u8))) as i32),
            0i32,
            10u8,
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            GetStoryActionByStat(stat),
        );
        ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
            .cast::<u8>())
        .wrapping_offset(((player) as i32) as isize))
        .write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn ScrambleStatList(arr: *mut u8, count: i32) {
    unsafe {
        let mut arr = arr;
        let mut count = count;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    ((arr).wrapping_offset((i) as isize)).write(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < count) {
                    break 'l3;
                }
                'l4: {
                    let mut a: u32 = ((crate::c::rem_i32(((Random()) as i32), count)) as u32);
                    let mut b: u32 = ((crate::c::rem_i32(((Random()) as i32), count)) as u32);
                    let mut temp: u8 = 0u8;
                    {
                        temp = ((arr).wrapping_offset(((a) as i32) as isize)).read();
                        ((arr).wrapping_offset(((a) as i32) as isize))
                            .write(((arr).wrapping_offset(((b) as i32) as isize)).read());
                        ((arr).wrapping_offset(((b) as i32) as isize)).write(temp);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StorytellerInitializeRandomStat() -> u8 {
    unsafe {
        let mut storyIds = crate::ffi::Align4([0u8; 36]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        ScrambleStatList(
            (&raw mut storyIds).cast::<u8>(),
            ((&raw const sNumStories)
                .cast::<u8>()
                .cast_mut()
                .cast::<i32>())
            .read(),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((&raw const sNumStories)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<i32>())
                    .read())
                {
                    break 'l1;
                }
                'l2: {
                    let mut stat: u8 =
                        ((((&raw const sStorytellerStories).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut storyIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 16,
                            ))
                        .read();
                    let mut minVal: u8 =
                        (((((&raw const sStorytellerStories).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut storyIds).cast::<u8>()).wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                * 16,
                        ))
                        .wrapping_add(1))
                        .read();
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((((&raw mut sStorytellerPtr)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == ((stat) as i32)
                                {
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if (j == 4i32) && (StorytellerGetGameStat(stat) >= ((minVal) as u32)) {
                        ((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1))
                        .write(1u8);
                        if ((GetFreeStorySlot()) as i32) == 4i32 {
                            StorytellerRecordNewStat(
                                ((((&raw mut sSelectedStory).cast::<u8>().cast::<u8>()).read())
                                    as u32),
                                ((stat) as u32),
                            );
                        } else {
                            StorytellerRecordNewStat(
                                ((GetFreeStorySlot()) as u32),
                                ((stat) as u32),
                            );
                        }
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StorytellerDisplayStory(player: u32) {
    unsafe {
        let mut player = player;
        let mut stat: u8 = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(4))
        .cast::<u8>())
        .wrapping_offset(((player) as i32) as isize))
        .read();
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((StorytellerGetRecordedTrainerStat(player)) as i32),
            0i32,
            10u8,
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            GetStoryActionByStat(((stat) as u32)),
        );
        GetStoryByStattellerPlayerName(player, (&raw mut gStringVar3).cast::<u8>());
        ConvertInternationalString(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52))
            .cast::<u8>())
            .wrapping_offset(((player) as i32) as isize))
            .read(),
        );
        ShowFieldMessage(GetStoryTextByStat(((stat) as u32)));
    }
}
pub(crate) unsafe extern "C" fn PrintStoryList() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut width: i32 = GetStringWidth(1u8, (&raw mut gText_Exit).cast::<u8>(), 0i16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut curWidth: i32 = 0i32;
                    let mut gameStatID: u16 =
                        ((((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as u16);
                    if ((gameStatID) as i32) == 0i32 {
                        break 'l1;
                    }
                    curWidth =
                        GetStringWidth(1u8, GetStoryTitleByStat(((gameStatID) as u32)), 0i16);
                    if curWidth > width {
                        width = curWidth;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).write(CreateWindowFromRect(
            0u8,
            0u8,
            ((ConvertPixelWidthToTileWidth(width)) as u8),
            (((((GetFreeStorySlot()) as i32).wrapping_mul(2i32)).wrapping_add(2i32)) as u8),
        ));
        SetStandardWindowBorderStyle(
            ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    let mut gameStatID: u16 =
                        ((((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as u16);
                    if ((gameStatID) as i32) == 0i32 {
                        break 'l3;
                    }
                    AddTextPrinterParameterized(
                        ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                        GetStoryTitleByStat(((gameStatID) as u32)),
                        8u8,
                        ((((16i32).wrapping_mul(i)).wrapping_add(1i32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        AddTextPrinterParameterized(
            ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Exit).cast::<u8>(),
            8u8,
            ((((16i32).wrapping_mul(i)).wrapping_add(1i32)) as u8),
            255u8,
            None,
        );
        InitMenuInUpperLeftCornerNormal(
            ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
            ((((GetFreeStorySlot()) as i32).wrapping_add(1i32)) as u8),
            0u8,
        );
        CopyWindowToVram(
            ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StoryListMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut selection: i32 = 0i32;
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                PrintStoryList();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                selection = ((Menu_ProcessInput()) as i32);
                if selection == (-2i32) {
                    break 'l1;
                }
                if (selection == (-1i32)) || (selection == ((GetFreeStorySlot()) as i32)) {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                } else {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                    ((&raw mut sSelectedStory).cast::<u8>().cast::<u8>())
                        .write(((selection) as u8));
                }
                ClearToTransparentAndRemoveWindow(
                    ((&raw mut sStorytellerWindowId).cast::<u8>().cast::<u8>()).read(),
                );
                DestroyTask(taskId);
                ScriptContext_Enable();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorytellerStoryListMenu() {
    unsafe {
        CreateTask(Some(Task_StoryListMenu), 80u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_StorytellerDisplayStory() {
    unsafe {
        StorytellerDisplayStory(
            ((((&raw mut sSelectedStory).cast::<u8>().cast::<u8>()).read()) as u32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorytellerGetFreeStorySlot() -> u8 {
    unsafe {
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        return GetFreeStorySlot();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorytellerUpdateStat() -> u8 {
    unsafe {
        let mut stat: u8 = 0u8;
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        stat = ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sSelectedStory).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        if HasTrainerStatIncreased(
            ((((&raw mut sSelectedStory).cast::<u8>().cast::<u8>()).read()) as u32),
        ) == 1u32
        {
            StorytellerRecordNewStat(
                ((((&raw mut sSelectedStory).cast::<u8>().cast::<u8>()).read()) as u32),
                ((stat) as u32),
            );
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasStorytellerAlreadyRecorded() -> u8 {
    unsafe {
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        if ((((((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .read()) as i32)
            == 0i32
        {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_StorytellerInitializeRandomStat() -> u8 {
    unsafe {
        ((&raw mut sStorytellerPtr).cast::<u8>().cast::<*mut u8>())
            .write(((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11816)));
        return StorytellerInitializeRandomStat();
    }
}
