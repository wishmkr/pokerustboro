//! Translated from `src/m4a.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSoundInfo: crate::ffi::Align4<[u8; 4016]> = crate::ffi::Align4([0; 4016]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCrySongs: crate::ffi::Align4<[u8; 104]> = crate::ffi::Align4([0; 104]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCryMusicPlayers: crate::ffi::Align4<[u8; 128]> =
    crate::ffi::Align4([0; 128]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_BGM: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayJumpTable: crate::ffi::Align4<[u8; 144]> = crate::ffi::Align4([0; 144]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCgbChans: crate::ffi::Align4<[u8; 256]> = crate::ffi::Align4([0; 256]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE1: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE2: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCryTracks: crate::ffi::Align4<[u8; 320]> = crate::ffi::Align4([0; 320]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCrySong: crate::ffi::Align4<[u8; 52]> = crate::ffi::Align4([0; 52]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayMemAccArea: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE3: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);

unsafe extern "C" {
    static mut gCgb3Vol: u8;
    static mut gCgbFreqTable: u8;
    static mut gCgbScaleTable: u8;
    static mut gFreqTable: u8;
    static mut gMPlayTable: u8;
    static mut gMaxLines: u8;
    static mut gNoiseTable: u8;
    static mut gNumMusicPlayers: u8;
    static mut gPcmSamplesPerVBlankTable: u8;
    static mut gPokemonCrySongTemplate: u8;
    static mut gScaleTable: u8;
    static mut gSongTable: u8;
    static mut gXcmdTable: u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn MPlayJumpTableCopy(a0: *mut Option<unsafe extern "C" fn()>);
    fn MPlayMain(a0: *mut u8);
    fn SoundMain();
    fn TrackStop(a0: *mut u8, a1: *mut u8);
    fn ply_endtie(a0: *mut u8, a1: *mut u8);
    fn ply_lfos(a0: *mut u8, a1: *mut u8);
    fn ply_mod(a0: *mut u8, a1: *mut u8);
    fn ply_note(a0: u32, a1: *mut u8, a2: *mut u8);
    fn umul3232H32(a0: u32, a1: u32) -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MidiKeyToFreq(wav: *mut u8, key: u8, fineAdjust: u8) -> u32 {
    unsafe {
        let mut wav = wav;
        let mut key = key;
        let mut fineAdjust = fineAdjust;
        let mut val1: u32 = 0u32;
        let mut val2: u32 = 0u32;
        let mut fineAdjustShifted: u32 = ((((fineAdjust) as i32) << 24) as u32);
        if ((key) as i32) > 178i32 {
            key = 178u8;
            fineAdjustShifted = 4278190080u32;
        }
        val1 = (((((&raw mut gScaleTable).cast::<u8>()).wrapping_offset(((key) as i32) as isize))
            .read()) as u32);
        val1 = crate::c::shr_u32(
            ((((&raw mut gFreqTable).cast::<u32>()).cast::<u32>())
                .wrapping_offset(((val1 & 15u32) as i32) as isize))
            .read(),
            (val1 >> 4),
        );
        val2 = (((((&raw mut gScaleTable).cast::<u8>())
            .wrapping_offset((((key) as i32).wrapping_add(1i32)) as isize))
        .read()) as u32);
        val2 = crate::c::shr_u32(
            ((((&raw mut gFreqTable).cast::<u32>()).cast::<u32>())
                .wrapping_offset(((val2 & 15u32) as i32) as isize))
            .read(),
            (val2 >> 4),
        );
        return umul3232H32(
            ((wav).wrapping_add(4).cast::<u32>()).read(),
            (val1).wrapping_add(umul3232H32((val2).wrapping_sub(val1), fineAdjustShifted)),
        );
    }
}
pub(crate) unsafe extern "C" fn UnusedDummyFunc() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayContinue(mplayInfo: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() == 1752395091u32 {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (mplayInfo).wrapping_add(4).cast::<u32>();
            (__p2).write(((__p2).read() & 2147483647u32));
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayFadeOut(mplayInfo: *mut u8, speed: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut speed = speed;
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() == 1752395091u32 {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((mplayInfo).wrapping_add(38).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(36).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(40).cast::<u16>()).write(256u16);
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundInit() {
    unsafe {
        let mut i: i32 = 0i32;
        // The mixer (m4a_mixer.rs) is linked into IWRAM and copied there by
        // Init, so there is no SoundMainRAM_Buffer to copy it into here.
        SoundInit((&raw mut gSoundInfo).cast::<u8>());
        MPlayExtender(((&raw mut gCgbChans).cast::<u8>()).cast::<u8>());
        m4aSoundMode(9749760u32);
        {
            i = 0i32;
            'l5: loop {
                if !(i < ((((&raw mut gNumMusicPlayers).cast::<u8>()) as usize as u16) as i32)) {
                    break 'l5;
                }
                'l6: {
                    let mut mplayInfo: *mut u8 = ((((&raw mut gMPlayTable).cast::<u8>())
                        .wrapping_offset((i) as isize * 12))
                    .cast::<*mut u8>())
                    .read();
                    MPlayOpen(
                        mplayInfo,
                        ((((&raw mut gMPlayTable).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                        .read(),
                        ((((&raw mut gMPlayTable).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .wrapping_add(8))
                        .read(),
                    );
                    ((mplayInfo).wrapping_add(11)).write(
                        ((((((&raw mut gMPlayTable).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .wrapping_add(10)
                        .cast::<u16>())
                        .read()) as u8),
                    );
                    ((mplayInfo).wrapping_add(24).cast::<*mut u8>())
                        .write(((&raw mut gMPlayMemAccArea).cast::<u8>()).cast::<u8>());
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::memcpy(
            (&raw mut gPokemonCrySong).cast::<u8>(),
            (&raw mut gPokemonCrySongTemplate).cast::<u8>(),
            52u32,
        );
        {
            i = 0i32;
            'l7: loop {
                if !(i < 2i32) {
                    break 'l7;
                }
                'l8: {
                    let mut mplayInfo: *mut u8 =
                        (((&raw mut gPokemonCryMusicPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 64);
                    let mut track: *mut u8 = (((&raw mut gPokemonCryTracks).cast::<u8>())
                        .cast::<u8>())
                    .wrapping_offset(((i).wrapping_mul(2i32)) as isize * 80);
                    MPlayOpen(mplayInfo, track, 2u8);
                    ((track).wrapping_add(32).cast::<*mut u8>()).write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundMain() {
    unsafe {
        SoundMain();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStart(n: u16) {
    unsafe {
        let mut n = n;
        let mut mplayTable: *mut u8 = (&raw mut gMPlayTable).cast::<u8>();
        let mut songTable: *mut u8 = (&raw mut gSongTable).cast::<u8>();
        let mut song: *mut u8 = (songTable).wrapping_offset(((n) as i32) as isize * 8);
        let mut mplay: *mut u8 = (mplayTable).wrapping_offset(
            ((((song).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 12,
        );
        MPlayStart(
            ((mplay).cast::<*mut u8>()).read(),
            ((song).cast::<*mut u8>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStartOrChange(n: u16) {
    unsafe {
        let mut n = n;
        let mut mplayTable: *mut u8 = (&raw mut gMPlayTable).cast::<u8>();
        let mut songTable: *mut u8 = (&raw mut gSongTable).cast::<u8>();
        let mut song: *mut u8 = (songTable).wrapping_offset(((n) as i32) as isize * 8);
        let mut mplay: *mut u8 = (mplayTable).wrapping_offset(
            ((((song).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 12,
        );
        if ((((((mplay).cast::<*mut u8>()).read()).cast::<*mut u8>()).read()) as usize)
            != ((((song).cast::<*mut u8>()).read()) as usize)
        {
            MPlayStart(
                ((mplay).cast::<*mut u8>()).read(),
                ((song).cast::<*mut u8>()).read(),
            );
        } else {
            if ((((((mplay).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                & 65535u32)
                == 0u32)
                || ((((((mplay).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()
                    & 2147483648u32)
                    != 0)
            {
                MPlayStart(
                    ((mplay).cast::<*mut u8>()).read(),
                    ((song).cast::<*mut u8>()).read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn m4aSongNumStartOrContinue(n: u16) {
    unsafe {
        let mut n = n;
        let mut mplayTable: *mut u8 = (&raw mut gMPlayTable).cast::<u8>();
        let mut songTable: *mut u8 = (&raw mut gSongTable).cast::<u8>();
        let mut song: *mut u8 = (songTable).wrapping_offset(((n) as i32) as isize * 8);
        let mut mplay: *mut u8 = (mplayTable).wrapping_offset(
            ((((song).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 12,
        );
        if ((((((mplay).cast::<*mut u8>()).read()).cast::<*mut u8>()).read()) as usize)
            != ((((song).cast::<*mut u8>()).read()) as usize)
        {
            MPlayStart(
                ((mplay).cast::<*mut u8>()).read(),
                ((song).cast::<*mut u8>()).read(),
            );
        } else {
            if (((((mplay).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                & 65535u32)
                == 0u32
            {
                MPlayStart(
                    ((mplay).cast::<*mut u8>()).read(),
                    ((song).cast::<*mut u8>()).read(),
                );
            } else {
                if (((((mplay).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()
                    & 2147483648u32)
                    != 0
                {
                    MPlayContinue(((mplay).cast::<*mut u8>()).read());
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStop(n: u16) {
    unsafe {
        let mut n = n;
        let mut mplayTable: *mut u8 = (&raw mut gMPlayTable).cast::<u8>();
        let mut songTable: *mut u8 = (&raw mut gSongTable).cast::<u8>();
        let mut song: *mut u8 = (songTable).wrapping_offset(((n) as i32) as isize * 8);
        let mut mplay: *mut u8 = (mplayTable).wrapping_offset(
            ((((song).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 12,
        );
        if ((((((mplay).cast::<*mut u8>()).read()).cast::<*mut u8>()).read()) as usize)
            == ((((song).cast::<*mut u8>()).read()) as usize)
        {
            m4aMPlayStop(((mplay).cast::<*mut u8>()).read());
        }
    }
}
pub(crate) unsafe extern "C" fn m4aSongNumContinue(n: u16) {
    unsafe {
        let mut n = n;
        let mut mplayTable: *mut u8 = (&raw mut gMPlayTable).cast::<u8>();
        let mut songTable: *mut u8 = (&raw mut gSongTable).cast::<u8>();
        let mut song: *mut u8 = (songTable).wrapping_offset(((n) as i32) as isize * 8);
        let mut mplay: *mut u8 = (mplayTable).wrapping_offset(
            ((((song).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 12,
        );
        if ((((((mplay).cast::<*mut u8>()).read()).cast::<*mut u8>()).read()) as usize)
            == ((((song).cast::<*mut u8>()).read()) as usize)
        {
            MPlayContinue(((mplay).cast::<*mut u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayAllStop() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gNumMusicPlayers).cast::<u8>()) as usize as u16) as i32)) {
                    break 'l1;
                }
                'l2: {
                    m4aMPlayStop(
                        ((((&raw mut gMPlayTable).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .cast::<*mut u8>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 2i32) {
                    break 'l3;
                }
                'l4: {
                    m4aMPlayStop(
                        (((&raw mut gPokemonCryMusicPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 64),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayContinue(mplayInfo: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        MPlayContinue(mplayInfo);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayAllContinue() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gNumMusicPlayers).cast::<u8>()) as usize as u16) as i32)) {
                    break 'l1;
                }
                'l2: {
                    MPlayContinue(
                        ((((&raw mut gMPlayTable).cast::<u8>())
                            .wrapping_offset((i) as isize * 12))
                        .cast::<*mut u8>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 2i32) {
                    break 'l3;
                }
                'l4: {
                    MPlayContinue(
                        (((&raw mut gPokemonCryMusicPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 64),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeOut(mplayInfo: *mut u8, speed: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut speed = speed;
        MPlayFadeOut(mplayInfo, speed);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeOutTemporarily(mplayInfo: *mut u8, speed: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut speed = speed;
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() == 1752395091u32 {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((mplayInfo).wrapping_add(38).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(36).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(40).cast::<u16>()).write(257u16);
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeIn(mplayInfo: *mut u8, speed: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut speed = speed;
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() == 1752395091u32 {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((mplayInfo).wrapping_add(38).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(36).cast::<u16>()).write(speed);
            ((mplayInfo).wrapping_add(40).cast::<u16>()).write(2u16);
            let __p2 = (mplayInfo).wrapping_add(4).cast::<u32>();
            (__p2).write(((__p2).read() & 2147483647u32));
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayImmInit(mplayInfo: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackCount: i32 = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        let mut track: *mut u8 = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        'l1: loop {
            if !(trackCount > 0i32) {
                break 'l1;
            }
            if ((((track).read()) as i32) & 128i32) != 0 {
                if ((((track).read()) as i32) & 64i32) != 0 {
                    Clear64byte(track);
                    (track).write(128u8);
                    ((track).wrapping_add(15)).write(2u8);
                    ((track).wrapping_add(19)).write(64u8);
                    ((track).wrapping_add(25)).write(22u8);
                    ((track).wrapping_add(36)).write(1u8);
                }
            }
            trackCount = (trackCount).wrapping_sub(1);
            track = (track).wrapping_offset(80);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayExtender(cgbChans: *mut u8) {
    unsafe {
        let mut cgbChans = cgbChans;
        let mut soundInfo: *mut u8 = core::ptr::null_mut();
        let mut ident: u32 = 0u32;
        crate::c::volatile_write(((67108996i32) as usize as *mut u16), 143u16);
        crate::c::volatile_write(((67108992i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67108963i32) as usize as *mut u8), 8u8);
        crate::c::volatile_write(((67108969i32) as usize as *mut u8), 8u8);
        crate::c::volatile_write(((67108985i32) as usize as *mut u8), 8u8);
        crate::c::volatile_write(((67108965i32) as usize as *mut u8), 128u8);
        crate::c::volatile_write(((67108973i32) as usize as *mut u8), 128u8);
        crate::c::volatile_write(((67108989i32) as usize as *mut u8), 128u8);
        crate::c::volatile_write(((67108976i32) as usize as *mut u8), 0u8);
        crate::c::volatile_write(((67108992i32) as usize as *mut u8), 119u8);
        soundInfo = ((50364400i32) as usize as *mut *mut u8).read();
        ident = ((soundInfo).cast::<u32>()).read();
        if ident != 1752395091u32 {
            return;
        }
        let __p1 = (soundInfo).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(8))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            ply_memacc as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(17))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            ply_lfos as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(19))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            ply_mod as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(28))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            ply_xcmd as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(29))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            ply_endtie as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(30))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(u32)>,
            Option<unsafe extern "C" fn()>,
        >(Some(SampleFreqSet as unsafe extern "C" fn(u32))));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(31))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            TrackStop as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(32))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            FadeOutBody as unsafe extern "C" fn(*mut u8),
        )));
        ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(33))
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut u8, *mut u8)>,
            Option<unsafe extern "C" fn()>,
        >(Some(
            TrkVolPitSet as unsafe extern "C" fn(*mut u8, *mut u8),
        )));
        ((soundInfo).wrapping_add(28).cast::<*mut u8>()).write(cgbChans);
        ((soundInfo)
            .wrapping_add(40)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CgbSound as unsafe extern "C" fn()));
        ((soundInfo)
            .wrapping_add(44)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(CgbOscOff as unsafe extern "C" fn(u8)));
        ((soundInfo)
            .wrapping_add(48)
            .cast::<Option<unsafe extern "C" fn(u8, u8, u8) -> u32>>())
        .write(Some(
            MidiKeyToCgbFreq as unsafe extern "C" fn(u8, u8, u8) -> u32,
        ));
        ((soundInfo).wrapping_add(12))
            .write(((((&raw mut gMaxLines).cast::<u8>()) as usize as u32) as u8));
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                cgbChans,
                                (83886080u32
                                    | (crate::c::div_u32(
                                        256u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        ((cgbChans).wrapping_add(1)).write(1u8);
        ((cgbChans).wrapping_add(28)).write(17u8);
        (((cgbChans).wrapping_offset(64)).wrapping_add(1)).write(2u8);
        (((cgbChans).wrapping_offset(64)).wrapping_add(28)).write(34u8);
        (((cgbChans).wrapping_offset(128)).wrapping_add(1)).write(3u8);
        (((cgbChans).wrapping_offset(128)).wrapping_add(28)).write(68u8);
        (((cgbChans).wrapping_offset(192)).wrapping_add(1)).write(4u8);
        (((cgbChans).wrapping_offset(192)).wrapping_add(28)).write(136u8);
        ((soundInfo).cast::<u32>()).write(ident);
    }
}
// hand-written: tools/rustport/overrides/m4a/MusicPlayerJumpTableCopy.rs
/// `MusicPlayerJumpTableCopy`: the BIOS call `swi 0x2A` (unused by the game).
unsafe extern "C" fn MusicPlayerJumpTableCopy() {
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!("swi 0x2A", out("r0") _, out("r1") _, out("r2") _, out("r3") _, out("r12") _, out("lr") _)
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearChain(x: *mut u8) {
    unsafe {
        let mut x = x;
        let mut func: Option<unsafe extern "C" fn()> = ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(34))
        .read();
        core::mem::transmute::<_, unsafe extern "C" fn(*mut u8)>((func).unwrap_unchecked())(x);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clear64byte(x: *mut u8) {
    unsafe {
        let mut x = x;
        let mut func: Option<unsafe extern "C" fn()> = ((((&raw mut gMPlayJumpTable)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(35))
        .read();
        core::mem::transmute::<_, unsafe extern "C" fn(*mut u8)>((func).unwrap_unchecked())(x);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundInit(soundInfo: *mut u8) {
    unsafe {
        let mut soundInfo = soundInfo;
        ((soundInfo).cast::<u32>()).write(0u32);
        if (((67109060i32) as usize as *mut u32).read_volatile() & 33554432u32) != 0 {
            crate::c::volatile_write(((67109060i32) as usize as *mut u32), 2218786820u32);
        }
        if (((67109072i32) as usize as *mut u32).read_volatile() & 33554432u32) != 0 {
            crate::c::volatile_write(((67109072i32) as usize as *mut u32), 2218786820u32);
        }
        crate::c::volatile_write(((67109062i32) as usize as *mut u16), 1024u16);
        crate::c::volatile_write(((67109074i32) as usize as *mut u16), 1024u16);
        crate::c::volatile_write(((67108996i32) as usize as *mut u16), 143u16);
        crate::c::volatile_write(((67108994i32) as usize as *mut u16), 43278u16);
        crate::c::volatile_write(
            ((67109001i32) as usize as *mut u8),
            (((((((67109001i32) as usize as *mut u8).read_volatile()) as i32) & 63i32) | 64i32)
                as u8),
        );
        crate::c::volatile_write(
            ((67109052i32) as usize as *mut u32),
            (((((soundInfo).wrapping_add(848)).cast::<i8>()) as usize as i32) as u32),
        );
        crate::c::volatile_write(
            ((67109056i32) as usize as *mut u32),
            ((((67109024i32) as usize as *mut u32) as usize as i32) as u32),
        );
        crate::c::volatile_write(
            ((67109064i32) as usize as *mut u32),
            ((((((soundInfo).wrapping_add(848)).cast::<i8>()) as usize as i32)
                .wrapping_add(1584i32)) as u32),
        );
        crate::c::volatile_write(
            ((67109068i32) as usize as *mut u32),
            ((((67109028i32) as usize as *mut u32) as usize as i32) as u32),
        );
        ((50364400i32) as usize as *mut *mut u8).write(soundInfo);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                soundInfo,
                                (83886080u32
                                    | (crate::c::div_u32(
                                        4016u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        ((soundInfo).wrapping_add(6)).write(8u8);
        ((soundInfo).wrapping_add(7)).write(15u8);
        ((soundInfo)
            .wrapping_add(56)
            .cast::<Option<unsafe extern "C" fn(u32, *mut u8, *mut u8)>>())
        .write(Some(
            ply_note as unsafe extern "C" fn(u32, *mut u8, *mut u8),
        ));
        ((soundInfo)
            .wrapping_add(40)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(DummyFunc as unsafe extern "C" fn()));
        ((soundInfo)
            .wrapping_add(44)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn()>,
            Option<unsafe extern "C" fn(u8)>,
        >(Some(DummyFunc as unsafe extern "C" fn())));
        ((soundInfo)
            .wrapping_add(48)
            .cast::<Option<unsafe extern "C" fn(u8, u8, u8) -> u32>>())
        .write(core::mem::transmute::<
            Option<unsafe extern "C" fn()>,
            Option<unsafe extern "C" fn(u8, u8, u8) -> u32>,
        >(Some(DummyFunc as unsafe extern "C" fn())));
        ((soundInfo)
            .wrapping_add(60)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(DummyFunc as unsafe extern "C" fn()));
        MPlayJumpTableCopy(
            ((&raw mut gMPlayJumpTable)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>(),
        );
        ((soundInfo)
            .wrapping_add(52)
            .cast::<*mut Option<unsafe extern "C" fn()>>())
        .write(
            ((&raw mut gMPlayJumpTable)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>(),
        );
        SampleFreqSet(262144u32);
        ((soundInfo).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SampleFreqSet(freq: u32) {
    unsafe {
        let mut freq = freq;
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        freq = ((freq & 983040u32) >> 16);
        ((soundInfo).wrapping_add(8)).write(((freq) as u8));
        ((soundInfo).wrapping_add(16).cast::<i32>()).write(
            ((((((&raw mut gPcmSamplesPerVBlankTable).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((freq).wrapping_sub(1u32)) as i32) as isize))
            .read()) as i32),
        );
        ((soundInfo).wrapping_add(11)).write(
            ((crate::c::div_i32(1584i32, ((soundInfo).wrapping_add(16).cast::<i32>()).read()))
                as u8),
        );
        ((soundInfo).wrapping_add(20).cast::<i32>()).write(crate::c::div_i32(
            ((597275i32).wrapping_mul(((soundInfo).wrapping_add(16).cast::<i32>()).read()))
                .wrapping_add(5000i32),
            10000i32,
        ));
        ((soundInfo).wrapping_add(24).cast::<i32>()).write(
            ((crate::c::div_i32(
                16777216i32,
                ((soundInfo).wrapping_add(20).cast::<i32>()).read(),
            ))
            .wrapping_add(1i32)
                >> 1),
        );
        crate::c::volatile_write(((67109122i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(
            ((67109120i32) as usize as *mut u16),
            (((crate::c::div_i32(
                280896i32,
                ((soundInfo).wrapping_add(16).cast::<i32>()).read(),
            ))
            .wrapping_neg()) as u16),
        );
        m4aSoundVSyncOn();
        'l1: loop {
            if !(((((67108870i32) as usize as *mut u8).read_volatile()) as i32) == 159i32) {
                break 'l1;
            }
        }
        'l2: loop {
            if !(((((67108870i32) as usize as *mut u8).read_volatile()) as i32) != 159i32) {
                break 'l2;
            }
        }
        crate::c::volatile_write(((67109122i32) as usize as *mut u16), 128u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundMode(mode: u32) {
    unsafe {
        let mut mode = mode;
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        let mut temp: u32 = 0u32;
        if ((soundInfo).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (soundInfo).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        temp = (mode & 255u32);
        if (temp) != 0 {
            ((soundInfo).wrapping_add(5)).write(((temp & 127u32) as u8));
        }
        temp = (mode & 3840u32);
        if (temp) != 0 {
            let mut chan: *mut u8 = core::ptr::null_mut();
            ((soundInfo).wrapping_add(6)).write(((temp >> 8) as u8));
            temp = 12u32;
            chan = ((soundInfo).wrapping_add(80)).cast::<u8>();
            'l1: loop {
                if !(temp != 0u32) {
                    break 'l1;
                }
                (chan).write(0u8);
                temp = (temp).wrapping_sub(1);
                chan = (chan).wrapping_offset(64);
            }
        }
        temp = (mode & 61440u32);
        if (temp) != 0 {
            ((soundInfo).wrapping_add(7)).write(((temp >> 12) as u8));
        }
        temp = (mode & 11534336u32);
        if (temp) != 0 {
            temp = ((temp & 3145728u32) >> 14);
            crate::c::volatile_write(
                ((67109001i32) as usize as *mut u8),
                ((((((((67109001i32) as usize as *mut u8).read_volatile()) as i32) & 63i32) as u32)
                    | temp) as u8),
            );
        }
        temp = (mode & 983040u32);
        if (temp) != 0 {
            m4aSoundVSyncOff();
            SampleFreqSet(temp);
        }
        ((soundInfo).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundClear() {
    unsafe {
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        let mut i: i32 = 0i32;
        let mut chan: *mut u8 = core::ptr::null_mut();
        if ((soundInfo).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (soundInfo).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = 12i32;
        chan = ((soundInfo).wrapping_add(80)).cast::<u8>();
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            (chan).write(0u8);
            i = (i).wrapping_sub(1);
            chan = (((((chan) as usize as i32) as u32).wrapping_add(64u32)) as usize as *mut u8);
        }
        chan = ((soundInfo).wrapping_add(28).cast::<*mut u8>()).read();
        if !(chan).is_null() {
            i = 1i32;
            'l2: loop {
                if !(i <= 4i32) {
                    break 'l2;
                }
                (((soundInfo)
                    .wrapping_add(44)
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(((i) as u8));
                (chan).write(0u8);
                i = (i).wrapping_add(1);
                chan =
                    (((((chan) as usize as i32) as u32).wrapping_add(64u32)) as usize as *mut u8);
            }
        }
        ((soundInfo).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundVSyncOff() {
    unsafe {
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        if (((soundInfo).cast::<u32>()).read() >= 1752395091u32)
            && (((soundInfo).cast::<u32>()).read() <= 1752395092u32)
        {
            let __p1 = (soundInfo).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(10u32));
            if (((67109060i32) as usize as *mut u32).read_volatile() & 33554432u32) != 0 {
                crate::c::volatile_write(((67109060i32) as usize as *mut u32), 2218786820u32);
            }
            if (((67109072i32) as usize as *mut u32).read_volatile() & 33554432u32) != 0 {
                crate::c::volatile_write(((67109072i32) as usize as *mut u32), 2218786820u32);
            }
            crate::c::volatile_write(((67109062i32) as usize as *mut u16), 1024u16);
            crate::c::volatile_write(((67109074i32) as usize as *mut u16), 1024u16);
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u32 = 0u32;
                        (&raw mut tmp).write_volatile(0u32);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((soundInfo).wrapping_add(848)).cast::<i8>()).cast::<u8>(),
                                    (83886080u32
                                        | (crate::c::div_u32(
                                            3168u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundVSyncOn() {
    unsafe {
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        let mut ident: u32 = ((soundInfo).cast::<u32>()).read();
        if ident == 1752395091u32 {
            return;
        }
        crate::c::volatile_write(((67109062i32) as usize as *mut u16), 46592u16);
        crate::c::volatile_write(((67109074i32) as usize as *mut u16), 46592u16);
        crate::c::volatile_write((soundInfo).wrapping_add(4), 0u8);
        ((soundInfo).cast::<u32>()).write((ident).wrapping_sub(10u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayOpen(mplayInfo: *mut u8, tracks: *mut u8, trackCount: u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut tracks = tracks;
        let mut trackCount = trackCount;
        let mut soundInfo: *mut u8 = core::ptr::null_mut();
        if ((trackCount) as i32) == 0i32 {
            return;
        }
        if ((trackCount) as i32) > 16i32 {
            trackCount = 16u8;
        }
        soundInfo = ((50364400i32) as usize as *mut *mut u8).read();
        if ((soundInfo).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (soundInfo).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        Clear64byte(mplayInfo);
        ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).write(tracks);
        ((mplayInfo).wrapping_add(8)).write(trackCount);
        ((mplayInfo).wrapping_add(4).cast::<u32>()).write(2147483648u32);
        'l1: loop {
            if !(((trackCount) as i32) != 0i32) {
                break 'l1;
            }
            (tracks).write(0u8);
            trackCount = (trackCount).wrapping_sub(1);
            tracks = (tracks).wrapping_offset(80);
        }
        if core::mem::transmute::<_, usize>(
            ((soundInfo)
                .wrapping_add(32)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) != 0usize
        {
            ((mplayInfo)
                .wrapping_add(56)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(
                ((soundInfo)
                    .wrapping_add(32)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            );
            ((mplayInfo).wrapping_add(60).cast::<*mut u8>())
                .write(((soundInfo).wrapping_add(36).cast::<*mut u8>()).read());
            ((soundInfo)
                .wrapping_add(32)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(None);
        }
        ((soundInfo).wrapping_add(36).cast::<*mut u8>()).write(mplayInfo);
        ((soundInfo)
            .wrapping_add(32)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(MPlayMain as unsafe extern "C" fn(*mut u8)));
        ((soundInfo).cast::<u32>()).write(1752395091u32);
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayStart(mplayInfo: *mut u8, songHeader: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut songHeader = songHeader;
        let mut i: i32 = 0i32;
        let mut unk_B: u8 = 0u8;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        unk_B = ((mplayInfo).wrapping_add(11)).read();
        if ((!((unk_B) != 0))
            || (((!(!(((mplayInfo).cast::<*mut u8>()).read()).is_null()))
                || (!(((((((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read()).read())
                    as i32)
                    & 64i32)
                    != 0)))
                && (((((mplayInfo).wrapping_add(4).cast::<u32>()).read() & 65535u32) == 0u32)
                    || ((((mplayInfo).wrapping_add(4).cast::<u32>()).read() & 2147483648u32)
                        != 0))))
            || (((((mplayInfo).wrapping_add(9)).read()) as i32)
                <= ((((songHeader).wrapping_add(2)).read()) as i32))
        {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((mplayInfo).wrapping_add(4).cast::<u32>()).write(0u32);
            ((mplayInfo).cast::<*mut u8>()).write(songHeader);
            ((mplayInfo).wrapping_add(48).cast::<*mut u8>())
                .write(((songHeader).wrapping_add(4).cast::<*mut u8>()).read());
            ((mplayInfo).wrapping_add(9)).write(((songHeader).wrapping_add(2)).read());
            ((mplayInfo).wrapping_add(12).cast::<u32>()).write(0u32);
            ((mplayInfo).wrapping_add(28).cast::<u16>()).write(150u16);
            ((mplayInfo).wrapping_add(32).cast::<u16>()).write(150u16);
            ((mplayInfo).wrapping_add(30).cast::<u16>()).write(256u16);
            ((mplayInfo).wrapping_add(34).cast::<u16>()).write(0u16);
            ((mplayInfo).wrapping_add(36).cast::<u16>()).write(0u16);
            i = 0i32;
            track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
            'l1: loop {
                if !((i < (((songHeader).read()) as i32))
                    && (i < ((((mplayInfo).wrapping_add(8)).read()) as i32)))
                {
                    break 'l1;
                }
                TrackStop(mplayInfo, track);
                (track).write(192u8);
                ((track).wrapping_add(32).cast::<*mut u8>()).write(core::ptr::null_mut());
                ((track).wrapping_add(64).cast::<*mut u8>()).write(
                    ((((songHeader).wrapping_add(8)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read(),
                );
                i = (i).wrapping_add(1);
                track = (track).wrapping_offset(80);
            }
            'l2: loop {
                if !(i < ((((mplayInfo).wrapping_add(8)).read()) as i32)) {
                    break 'l2;
                }
                TrackStop(mplayInfo, track);
                (track).write(0u8);
                i = (i).wrapping_add(1);
                track = (track).wrapping_offset(80);
            }
            if (((((songHeader).wrapping_add(3)).read()) as i32) & 128i32) != 0 {
                m4aSoundMode(((((songHeader).wrapping_add(3)).read()) as u32));
            }
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayStop(mplayInfo: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut i: i32 = 0i32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (mplayInfo).wrapping_add(4).cast::<u32>();
        (__p2).write(((__p2).read() | 2147483648u32));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            TrackStop(mplayInfo, track);
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBody(mplayInfo: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut i: i32 = 0i32;
        let mut track: *mut u8 = core::ptr::null_mut();
        let mut fadeOV: u16 = 0u16;
        if ((((mplayInfo).wrapping_add(36).cast::<u16>()).read()) as i32) == 0i32 {
            return;
        }
        if (({
            let __p1 = (mplayInfo).wrapping_add(38).cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            != 0i32
        {
            return;
        }
        ((mplayInfo).wrapping_add(38).cast::<u16>())
            .write(((mplayInfo).wrapping_add(36).cast::<u16>()).read());
        if (((((mplayInfo).wrapping_add(40).cast::<u16>()).read()) as i32) & 2i32) != 0 {
            if (({
                let __p3 = (mplayInfo).wrapping_add(40).cast::<u16>();
                let __v4 = (((((__p3).read()) as i32).wrapping_add(16i32)) as u16);
                (__p3).write(__v4);
                __v4
            }) as i32)
                >= 256i32
            {
                ((mplayInfo).wrapping_add(40).cast::<u16>()).write(256u16);
                ((mplayInfo).wrapping_add(36).cast::<u16>()).write(0u16);
            }
        } else {
            if ((({
                let __p5 = (mplayInfo).wrapping_add(40).cast::<u16>();
                let __v6 = (((((__p5).read()) as i32).wrapping_sub(16i32)) as u16);
                (__p5).write(__v6);
                __v6
            }) as i16) as i32)
                <= 0i32
            {
                i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
                track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
                'l1: loop {
                    if !(i > 0i32) {
                        break 'l1;
                    }
                    let mut val: u32 = 0u32;
                    TrackStop(mplayInfo, track);
                    val = 1u32;
                    fadeOV = ((mplayInfo).wrapping_add(40).cast::<u16>()).read();
                    val = (val & ((fadeOV) as u32));
                    if !((val) != 0) {
                        (track).write(0u8);
                    }
                    i = (i).wrapping_sub(1);
                    track = (track).wrapping_offset(80);
                }
                if (((((mplayInfo).wrapping_add(40).cast::<u16>()).read()) as i32) & 1i32) != 0 {
                    let __p7 = (mplayInfo).wrapping_add(4).cast::<u32>();
                    (__p7).write(((__p7).read() | 2147483648u32));
                } else {
                    ((mplayInfo).wrapping_add(4).cast::<u32>()).write(2147483648u32);
                }
                ((mplayInfo).wrapping_add(36).cast::<u16>()).write(0u16);
                return;
            }
        }
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        'l2: loop {
            if !(i > 0i32) {
                break 'l2;
            }
            if ((((track).read()) as i32) & 128i32) != 0 {
                fadeOV = ((mplayInfo).wrapping_add(40).cast::<u16>()).read();
                ((track).wrapping_add(19)).write(((((fadeOV) as i32) >> 2) as u8));
                let __p8 = (track);
                (__p8).write((((((__p8).read()) as i32) | 3i32) as u8));
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrkVolPitSet(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        if ((((track).read()) as i32) & 1i32) != 0 {
            let mut x: i32 = 0i32;
            let mut y: i32 = 0i32;
            x = ((((((((track).wrapping_add(18)).read()) as i32)
                .wrapping_mul(((((track).wrapping_add(19)).read()) as i32)))
                as u32)
                >> 5) as i32);
            if ((((track).wrapping_add(24)).read()) as i32) == 1i32 {
                x = (((((x).wrapping_mul(
                    ((((track).wrapping_add(22).cast::<i8>()).read()) as i32).wrapping_add(128i32),
                )) as u32)
                    >> 7) as i32);
            }
            y = ((2i32).wrapping_mul(((((track).wrapping_add(20).cast::<i8>()).read()) as i32)))
                .wrapping_add(((((track).wrapping_add(21).cast::<i8>()).read()) as i32));
            if ((((track).wrapping_add(24)).read()) as i32) == 2i32 {
                y = (y).wrapping_add(((((track).wrapping_add(22).cast::<i8>()).read()) as i32));
            }
            if y < (-128i32) {
                y = (-128i32);
            } else {
                if y > 127i32 {
                    y = 127i32;
                }
            }
            ((track).wrapping_add(16))
                .write(((((((y).wrapping_add(128i32)).wrapping_mul(x)) as u32) >> 8) as u8));
            ((track).wrapping_add(17))
                .write(((((((127i32).wrapping_sub(y)).wrapping_mul(x)) as u32) >> 8) as u8));
        }
        if ((((track).read()) as i32) & 4i32) != 0 {
            let mut bend: i32 = ((((track).wrapping_add(14).cast::<i8>()).read()) as i32)
                .wrapping_mul(((((track).wrapping_add(15)).read()) as i32));
            let mut x: i32 = ((((((((track).wrapping_add(12).cast::<i8>()).read()) as i32)
                .wrapping_add(bend))
            .wrapping_mul(4i32))
            .wrapping_add((((((track).wrapping_add(10).cast::<i8>()).read()) as i32) << 8)))
            .wrapping_add((((((track).wrapping_add(11).cast::<i8>()).read()) as i32) << 8)))
            .wrapping_add(((((track).wrapping_add(13)).read()) as i32));
            if ((((track).wrapping_add(24)).read()) as i32) == 0i32 {
                x = (x).wrapping_add(
                    (16i32).wrapping_mul(((((track).wrapping_add(22).cast::<i8>()).read()) as i32)),
                );
            }
            ((track).wrapping_add(8)).write(((x >> 8) as u8));
            ((track).wrapping_add(9)).write(((x) as u8));
        }
        let __p1 = (track);
        (__p1).write((((((__p1).read()) as i32) & (-6i32)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MidiKeyToCgbFreq(chanNum: u8, key: u8, fineAdjust: u8) -> u32 {
    unsafe {
        let mut chanNum = chanNum;
        let mut key = key;
        let mut fineAdjust = fineAdjust;
        if ((chanNum) as i32) == 4i32 {
            if ((key) as i32) <= 20i32 {
                key = 0u8;
            } else {
                key = ((((key) as i32).wrapping_sub(21i32)) as u8);
                if ((key) as i32) > 59i32 {
                    key = 59u8;
                }
            }
            return (((((&raw mut gNoiseTable).cast::<u8>())
                .wrapping_offset(((key) as i32) as isize))
            .read()) as u32);
        } else {
            let mut val1: i32 = 0i32;
            let mut val2: i32 = 0i32;
            if ((key) as i32) <= 35i32 {
                fineAdjust = 0u8;
                key = 0u8;
            } else {
                key = ((((key) as i32).wrapping_sub(36i32)) as u8);
                if ((key) as i32) > 130i32 {
                    key = 130u8;
                    fineAdjust = 255u8;
                }
            }
            val1 = (((((&raw mut gCgbScaleTable).cast::<u8>())
                .wrapping_offset(((key) as i32) as isize))
            .read()) as i32);
            val1 = crate::c::shr_i32(
                ((((((&raw mut gCgbFreqTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((val1 & 15i32) as isize))
                .read()) as i32),
                ((val1 >> 4) as u32),
            );
            val2 = (((((&raw mut gCgbScaleTable).cast::<u8>())
                .wrapping_offset((((key) as i32).wrapping_add(1i32)) as isize))
            .read()) as i32);
            val2 = crate::c::shr_i32(
                ((((((&raw mut gCgbFreqTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((val2 & 15i32) as isize))
                .read()) as i32),
                ((val2 >> 4) as u32),
            );
            return ((((val1).wrapping_add(
                (((fineAdjust) as i32).wrapping_mul((val2).wrapping_sub(val1)) >> 8),
            ))
            .wrapping_add(2048i32)) as u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CgbOscOff(chanNum: u8) {
    unsafe {
        let mut chanNum = chanNum;
        'l1: {
            let __sw1 = ((chanNum) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 {
                crate::c::volatile_write(((67108963i32) as usize as *mut u8), 8u8);
                crate::c::volatile_write(((67108965i32) as usize as *mut u8), 128u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::volatile_write(((67108969i32) as usize as *mut u8), 8u8);
                crate::c::volatile_write(((67108973i32) as usize as *mut u8), 128u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::volatile_write(((67108976i32) as usize as *mut u8), 0u8);
                break 'l1;
            }
            if !__matched {
                crate::c::volatile_write(((67108985i32) as usize as *mut u8), 8u8);
                crate::c::volatile_write(((67108989i32) as usize as *mut u8), 128u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CgbPan(chan: *mut u8) -> i32 {
    unsafe {
        let mut chan = chan;
        let mut rightVolume: u32 = ((((chan).wrapping_add(2)).read()) as u32);
        let mut leftVolume: u32 = ((((chan).wrapping_add(3)).read()) as u32);
        if {
            let __v1 = (((rightVolume) as u8) as u32);
            rightVolume = __v1;
            __v1
        } >= {
            let __v2 = (((leftVolume) as u8) as u32);
            leftVolume = __v2;
            __v2
        } {
            if crate::c::div_u32(rightVolume, 2u32) >= leftVolume {
                ((chan).wrapping_add(27)).write(15u8);
                return 1i32;
            }
        } else {
            if crate::c::div_u32(leftVolume, 2u32) >= rightVolume {
                ((chan).wrapping_add(27)).write(240u8);
                return 1i32;
            }
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CgbModVol(chan: *mut u8) {
    unsafe {
        let mut chan = chan;
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        if ((((((soundInfo).wrapping_add(9)).read()) as i32) & 1i32) != 0)
            || (!((CgbPan(chan)) != 0))
        {
            ((chan).wrapping_add(27)).write(255u8);
            ((chan).wrapping_add(10)).write(
                (((((((chan).wrapping_add(3)).read()) as i32)
                    .wrapping_add(((((chan).wrapping_add(2)).read()) as i32)))
                    as u32) as u8),
            );
            let __p1 = (chan).wrapping_add(10);
            (__p1).write(((crate::c::div_i32((((__p1).read()) as i32), 16i32)) as u8));
        } else {
            ((chan).wrapping_add(10)).write(
                (((((((chan).wrapping_add(3)).read()) as i32)
                    .wrapping_add(((((chan).wrapping_add(2)).read()) as i32)))
                    as u32) as u8),
            );
            let __p2 = (chan).wrapping_add(10);
            (__p2).write(((crate::c::div_i32((((__p2).read()) as i32), 16i32)) as u8));
            if ((((chan).wrapping_add(10)).read()) as i32) > 15i32 {
                ((chan).wrapping_add(10)).write(15u8);
            }
        }
        ((chan).wrapping_add(25)).write(
            (((((((chan).wrapping_add(10)).read()) as i32)
                .wrapping_mul(((((chan).wrapping_add(6)).read()) as i32)))
            .wrapping_add(15i32)
                >> 4) as u8),
        );
        let __p3 = (chan).wrapping_add(27);
        (__p3).write(
            (((((__p3).read()) as i32) & ((((chan).wrapping_add(28)).read()) as i32)) as u8),
        );
    }
}
// hand-written: tools/rustport/overrides/m4a/CgbSound.rs
// c2rs-uses: CgbModVol CgbOscOff gCgb3Vol
/// `CgbSound`: the GB-compatible (CGB) sound channels, once per frame.
///
/// The C jumps between labels in different branches (`goto oscillator_off`
/// into an `else if`, `goto init_env_step_time_dir` into a switch's
/// `default`...). Here each label is a state of a small machine; the order
/// of hardware writes is the C's. Offsets are GCC-probed:
/// struct CgbChannel (0x40 bytes) and SoundInfo.c15 @0xA / cgbChans @0x1C.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CgbSound() {
    #[derive(Clone, Copy)]
    enum S {
        Entry,
        StepRepeat,
        PseudoechoStart,
        Sustain,
        SustainStart,
        DecayStart,
        OscOff,
        StepComplete,
        EnvComplete,
        ChannelComplete,
    }
    // struct CgbChannel
    const STATUS: usize = 0x00;
    const TYPE: usize = 0x01;
    const ATTACK: usize = 0x04;
    const DECAY: usize = 0x05;
    const SUSTAIN: usize = 0x06;
    const RELEASE: usize = 0x07;
    const ENV_VOLUME: usize = 0x09;
    const ENV_GOAL: usize = 0x0A;
    const ENV_COUNTER: usize = 0x0B;
    const ECHO_VOLUME: usize = 0x0C;
    const ECHO_LENGTH: usize = 0x0D;
    const SUSTAIN_GOAL: usize = 0x19;
    const N4: usize = 0x1A;
    const PAN: usize = 0x1B;
    const PAN_MASK: usize = 0x1C;
    const MODIFY: usize = 0x1D;
    const LENGTH: usize = 0x1E;
    const SWEEP: usize = 0x1F;
    const FREQUENCY: usize = 0x20;
    const WAVE_POINTER: usize = 0x24;
    const CURRENT_POINTER: usize = 0x28;
    const CHANNEL_SIZE: usize = 0x40;
    // flags
    const SF_ON: u8 = 0xC7;
    const SF_START: u8 = 0x80;
    const SF_STOP: u8 = 0x40;
    const SF_IEC: u8 = 0x04;
    const SF_ENV: u8 = 0x03;
    const SF_ENV_ATTACK: u8 = 0x03;
    const SF_ENV_RELEASE: u8 = 0x00;
    const SF_ENV_SUSTAIN: u8 = 0x01;
    const SF_ENV_DECAY: u8 = 0x02;
    const MO_PIT: u8 = 0x02;
    const MO_VOL: u8 = 0x01;
    const ENV_DIR_INC: i32 = 0x08;
    const ENV_DIR_DEC: i32 = 0x00;
    const TONEDATA_TYPE_FIX: u8 = 0x08;
    const REG_NR51: *mut u8 = 0x0400_0081 as *mut u8;
    const REG_SOUNDBIAS_H: *mut u8 = 0x0400_0089 as *mut u8;
    const REG_WAVE_RAM0: *mut u32 = 0x0400_0090 as *mut u32;

    unsafe {
        let sound_info = (0x0300_7FF0 as *const *mut u8).read();
        let c15 = sound_info.add(0x0A);
        if c15.read() != 0 {
            c15.write(c15.read().wrapping_sub(1));
        } else {
            c15.write(14);
        }

        let mut ch: i32 = 1;
        // soundInfo->cgbChans is a pointer (to gCgbChans), not the field.
        let mut chan = sound_info.add(0x1C).cast::<*mut u8>().read();
        while ch <= 4 {
            let c = chan;
            let get = |off: usize| c.add(off).read();
            let set = |off: usize, v: u8| c.add(off).write(v);

            if get(STATUS) & SF_ON == 0 {
                ch += 1;
                chan = chan.add(CHANNEL_SIZE);
                continue;
            }

            // 1. hardware channel registers
            let (nrx0, nrx1, nrx2, nrx3, nrx4): (*mut u8, *mut u8, *mut u8, *mut u8, *mut u8) =
                match ch {
                    1 => (
                        0x0400_0060 as _,
                        0x0400_0062 as _,
                        0x0400_0063 as _,
                        0x0400_0064 as _,
                        0x0400_0065 as _,
                    ),
                    2 => (
                        0x0400_0061 as _,
                        0x0400_0068 as _,
                        0x0400_0069 as _,
                        0x0400_006C as _,
                        0x0400_006D as _,
                    ),
                    3 => (
                        0x0400_0070 as _,
                        0x0400_0072 as _,
                        0x0400_0073 as _,
                        0x0400_0074 as _,
                        0x0400_0075 as _,
                    ),
                    _ => (
                        0x0400_0071 as _,
                        0x0400_0078 as _,
                        0x0400_0079 as _,
                        0x0400_007C as _,
                        0x0400_007D as _,
                    ),
                };
            let wr = |p: *mut u8, v: u8| crate::c::volatile_write(p, v);

            let mut prev_c15: i32 = i32::from(c15.read());
            let mut env: i32 = i32::from(nrx2.read_volatile());
            let mut state = S::Entry;

            loop {
                match state {
                    S::Entry => {
                        let sf = get(STATUS);
                        if sf & SF_START != 0 {
                            if sf & SF_STOP == 0 {
                                set(STATUS, SF_ENV_ATTACK);
                                set(MODIFY, MO_PIT | MO_VOL);
                                CgbModVol(c);
                                let wave = c.add(WAVE_POINTER).cast::<*mut u32>().read();
                                let init_env = match ch {
                                    1 | 2 => {
                                        if ch == 1 {
                                            wr(nrx0, get(SWEEP));
                                        }
                                        wr(
                                            nrx1,
                                            ((wave as u32) << 6)
                                                .wrapping_add(u32::from(get(LENGTH)))
                                                as u8,
                                        );
                                        true
                                    }
                                    3 => {
                                        let current =
                                            c.add(CURRENT_POINTER).cast::<*mut u32>().read();
                                        if wave != current {
                                            wr(nrx0, 0x40);
                                            crate::c::volatile_write(REG_WAVE_RAM0, wave.read());
                                            crate::c::volatile_write(
                                                REG_WAVE_RAM0.add(1),
                                                wave.add(1).read(),
                                            );
                                            crate::c::volatile_write(
                                                REG_WAVE_RAM0.add(2),
                                                wave.add(2).read(),
                                            );
                                            crate::c::volatile_write(
                                                REG_WAVE_RAM0.add(3),
                                                wave.add(3).read(),
                                            );
                                            c.add(CURRENT_POINTER).cast::<*mut u32>().write(wave);
                                        }
                                        wr(nrx0, 0);
                                        wr(nrx1, get(LENGTH));
                                        set(N4, if get(LENGTH) != 0 { 0xC0 } else { 0x80 });
                                        false
                                    }
                                    _ => {
                                        wr(nrx1, get(LENGTH));
                                        wr(nrx3, ((wave as u32) << 3) as u8);
                                        true
                                    }
                                };
                                if init_env {
                                    // init_env_step_time_dir:
                                    env = i32::from(get(ATTACK)) + ENV_DIR_INC;
                                    set(N4, if get(LENGTH) != 0 { 0x40 } else { 0x00 });
                                }
                                set(ENV_COUNTER, get(ATTACK));
                                if get(ATTACK) as i8 != 0 {
                                    set(ENV_VOLUME, 0);
                                    state = S::StepComplete;
                                } else {
                                    // skip attack phase if attack is instantaneous (=0)
                                    state = S::DecayStart;
                                }
                            } else {
                                state = S::OscOff;
                            }
                        } else if sf & SF_IEC != 0 {
                            set(ECHO_LENGTH, get(ECHO_LENGTH).wrapping_sub(1));
                            state = if get(ECHO_LENGTH) as i8 <= 0 {
                                S::OscOff
                            } else {
                                S::EnvComplete
                            };
                        } else if sf & SF_STOP != 0 && sf & SF_ENV != 0 {
                            set(STATUS, sf & !SF_ENV);
                            set(ENV_COUNTER, get(RELEASE));
                            if get(RELEASE) as i8 != 0 {
                                set(MODIFY, get(MODIFY) | MO_VOL);
                                if ch != 3 {
                                    env = i32::from(get(RELEASE)) | ENV_DIR_DEC;
                                }
                                state = S::StepComplete;
                            } else {
                                state = S::PseudoechoStart;
                            }
                        } else {
                            state = S::StepRepeat;
                        }
                    }
                    S::StepRepeat => {
                        state = S::StepComplete;
                        if get(ENV_COUNTER) == 0 {
                            if ch == 3 {
                                set(MODIFY, get(MODIFY) | MO_VOL);
                            }
                            CgbModVol(c);
                            match get(STATUS) & SF_ENV {
                                SF_ENV_RELEASE => {
                                    set(ENV_VOLUME, get(ENV_VOLUME).wrapping_sub(1));
                                    if get(ENV_VOLUME) as i8 <= 0 {
                                        state = S::PseudoechoStart;
                                    } else {
                                        set(ENV_COUNTER, get(RELEASE));
                                    }
                                }
                                SF_ENV_SUSTAIN => state = S::Sustain,
                                SF_ENV_DECAY => {
                                    set(ENV_VOLUME, get(ENV_VOLUME).wrapping_sub(1));
                                    let volume = i32::from(get(ENV_VOLUME) as i8);
                                    let goal = i32::from(get(SUSTAIN_GOAL) as i8);
                                    if volume <= goal {
                                        state = S::SustainStart;
                                    } else {
                                        set(ENV_COUNTER, get(DECAY));
                                    }
                                }
                                _ => {
                                    set(ENV_VOLUME, get(ENV_VOLUME).wrapping_add(1));
                                    if get(ENV_VOLUME) >= get(ENV_GOAL) {
                                        state = S::DecayStart;
                                    } else {
                                        set(ENV_COUNTER, get(ATTACK));
                                    }
                                }
                            }
                        }
                    }
                    S::PseudoechoStart => {
                        let v =
                            (u32::from(get(ENV_GOAL)) * u32::from(get(ECHO_VOLUME)) + 0xFF) >> 8;
                        set(ENV_VOLUME, v as u8);
                        if get(ENV_VOLUME) != 0 {
                            set(STATUS, get(STATUS) | SF_IEC);
                            set(MODIFY, get(MODIFY) | MO_VOL);
                            if ch != 3 {
                                env = ENV_DIR_INC;
                            }
                            state = S::EnvComplete;
                        } else {
                            state = S::OscOff;
                        }
                    }
                    S::Sustain => {
                        set(ENV_VOLUME, get(SUSTAIN_GOAL));
                        set(ENV_COUNTER, 7);
                        state = S::StepComplete;
                    }
                    S::SustainStart => {
                        if get(SUSTAIN) == 0 {
                            set(STATUS, get(STATUS) & !SF_ENV);
                            state = S::PseudoechoStart;
                        } else {
                            set(STATUS, get(STATUS).wrapping_sub(1));
                            set(MODIFY, get(MODIFY) | MO_VOL);
                            if ch != 3 {
                                env = ENV_DIR_INC;
                            }
                            state = S::Sustain;
                        }
                    }
                    S::DecayStart => {
                        set(STATUS, get(STATUS).wrapping_sub(1));
                        set(ENV_COUNTER, get(DECAY));
                        if get(ENV_COUNTER) != 0 {
                            set(MODIFY, get(MODIFY) | MO_VOL);
                            set(ENV_VOLUME, get(ENV_GOAL));
                            if ch != 3 {
                                env = i32::from(get(DECAY)) | ENV_DIR_DEC;
                            }
                            state = S::StepComplete;
                        } else {
                            state = S::SustainStart;
                        }
                    }
                    S::OscOff => {
                        CgbOscOff(ch as u8);
                        set(STATUS, 0);
                        state = S::ChannelComplete;
                    }
                    S::StepComplete => {
                        // every 15 frames the envelope is stepped twice to keep
                        // up with the hardware envelope rate (1/64 s)
                        set(ENV_COUNTER, get(ENV_COUNTER).wrapping_sub(1));
                        if prev_c15 == 0 {
                            prev_c15 -= 1;
                            state = S::StepRepeat;
                        } else {
                            state = S::EnvComplete;
                        }
                    }
                    S::EnvComplete => {
                        // 3. pitch
                        if get(MODIFY) & MO_PIT != 0 {
                            let freq = c.add(FREQUENCY).cast::<u32>();
                            if ch < 4 && get(TYPE) & TONEDATA_TYPE_FIX != 0 {
                                let dac_pwm_rate = i32::from(REG_SOUNDBIAS_H.read_volatile());
                                if dac_pwm_rate < 0x40 {
                                    freq.write(freq.read().wrapping_add(2) & 0x7FC);
                                } else if dac_pwm_rate < 0x80 {
                                    freq.write(freq.read().wrapping_add(1) & 0x7FE);
                                }
                            }
                            if ch != 4 {
                                wr(nrx3, freq.read() as u8);
                            } else {
                                wr(nrx3, (nrx3.read_volatile() & 0x08) | freq.read() as u8);
                            }
                            set(
                                N4,
                                (get(N4) & 0xC0).wrapping_add(c.add(FREQUENCY + 1).read()),
                            );
                            wr(nrx4, get(N4));
                        }
                        // 4. envelope and volume
                        if get(MODIFY) & MO_VOL != 0 {
                            let nr51 = REG_NR51.read_volatile();
                            wr(REG_NR51, (nr51 & !get(PAN_MASK)) | get(PAN));
                            if ch == 3 {
                                let table = (&raw const gCgb3Vol).cast::<u8>();
                                wr(nrx2, table.add(usize::from(get(ENV_VOLUME))).read());
                                if get(N4) & 0x80 != 0 {
                                    wr(nrx0, 0x80);
                                    wr(nrx4, get(N4));
                                    set(N4, get(N4) & 0x7F);
                                }
                            } else {
                                let env_mask = 0xF;
                                wr(
                                    nrx2,
                                    ((env & env_mask) + (i32::from(get(ENV_VOLUME)) << 4)) as u8,
                                );
                                wr(nrx4, get(N4) | 0x80);
                                if ch == 1 && nrx0.read_volatile() & 0x08 == 0 {
                                    wr(nrx4, get(N4) | 0x80);
                                }
                            }
                        }
                        state = S::ChannelComplete;
                    }
                    S::ChannelComplete => {
                        set(MODIFY, 0);
                        break;
                    }
                }
            }

            ch += 1;
            chan = chan.add(CHANNEL_SIZE);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayTempoControl(mplayInfo: *mut u8, tempo: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut tempo = tempo;
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() == 1752395091u32 {
            let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((mplayInfo).wrapping_add(30).cast::<u16>()).write(tempo);
            ((mplayInfo).wrapping_add(32).cast::<u16>()).write(
                ((((((mplayInfo).wrapping_add(28).cast::<u16>()).read()) as i32)
                    .wrapping_mul(((((mplayInfo).wrapping_add(30).cast::<u16>()).read()) as i32))
                    >> 8) as u16),
            );
            ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayVolumeControl(mplayInfo: *mut u8, trackBits: u16, volume: u16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackBits = trackBits;
        let mut volume = volume;
        let mut i: i32 = 0i32;
        let mut bit: u32 = 0u32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        bit = 1u32;
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            if (((trackBits) as u32) & bit) != 0 {
                if ((((track).read()) as i32) & 128i32) != 0 {
                    ((track).wrapping_add(19))
                        .write(((crate::c::div_i32(((volume) as i32), 4i32)) as u8));
                    let __p2 = (track);
                    (__p2).write((((((__p2).read()) as i32) | 3i32) as u8));
                }
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
            bit = (bit << 1);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayPitchControl(mplayInfo: *mut u8, trackBits: u16, pitch: i16) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackBits = trackBits;
        let mut pitch = pitch;
        let mut i: i32 = 0i32;
        let mut bit: u32 = 0u32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        bit = 1u32;
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            if (((trackBits) as u32) & bit) != 0 {
                if ((((track).read()) as i32) & 128i32) != 0 {
                    ((track).wrapping_add(11).cast::<i8>()).write(((((pitch) as i32) >> 8) as i8));
                    ((track).wrapping_add(13)).write(((pitch) as u8));
                    let __p2 = (track);
                    (__p2).write((((((__p2).read()) as i32) | 12i32) as u8));
                }
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
            bit = (bit << 1);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayPanpotControl(mplayInfo: *mut u8, trackBits: u16, pan: i8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackBits = trackBits;
        let mut pan = pan;
        let mut i: i32 = 0i32;
        let mut bit: u32 = 0u32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        bit = 1u32;
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            if (((trackBits) as u32) & bit) != 0 {
                if ((((track).read()) as i32) & 128i32) != 0 {
                    ((track).wrapping_add(21).cast::<i8>()).write(pan);
                    let __p2 = (track);
                    (__p2).write((((((__p2).read()) as i32) | 3i32) as u8));
                }
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
            bit = (bit << 1);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearModM(track: *mut u8) {
    unsafe {
        let mut track = track;
        ((track).wrapping_add(26)).write(0u8);
        ((track).wrapping_add(22).cast::<i8>()).write(0i8);
        if ((((track).wrapping_add(24)).read()) as i32) == 0i32 {
            let __p1 = (track);
            (__p1).write((((((__p1).read()) as i32) | 12i32) as u8));
        } else {
            let __p2 = (track);
            (__p2).write((((((__p2).read()) as i32) | 3i32) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayModDepthSet(mplayInfo: *mut u8, trackBits: u16, modDepth: u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackBits = trackBits;
        let mut modDepth = modDepth;
        let mut i: i32 = 0i32;
        let mut bit: u32 = 0u32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        bit = 1u32;
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            if (((trackBits) as u32) & bit) != 0 {
                if ((((track).read()) as i32) & 128i32) != 0 {
                    ((track).wrapping_add(23)).write(modDepth);
                    if !((((track).wrapping_add(23)).read()) != 0) {
                        ClearModM(track);
                    }
                }
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
            bit = (bit << 1);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayLFOSpeedSet(mplayInfo: *mut u8, trackBits: u16, lfoSpeed: u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut trackBits = trackBits;
        let mut lfoSpeed = lfoSpeed;
        let mut i: i32 = 0i32;
        let mut bit: u32 = 0u32;
        let mut track: *mut u8 = core::ptr::null_mut();
        if ((mplayInfo).wrapping_add(52).cast::<u32>()).read() != 1752395091u32 {
            return;
        }
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        i = ((((mplayInfo).wrapping_add(8)).read()) as i32);
        track = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        bit = 1u32;
        'l1: loop {
            if !(i > 0i32) {
                break 'l1;
            }
            if (((trackBits) as u32) & bit) != 0 {
                if ((((track).read()) as i32) & 128i32) != 0 {
                    ((track).wrapping_add(25)).write(lfoSpeed);
                    if !((((track).wrapping_add(25)).read()) != 0) {
                        ClearModM(track);
                    }
                }
            }
            i = (i).wrapping_sub(1);
            track = (track).wrapping_offset(80);
            bit = (bit << 1);
        }
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_memacc(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        let mut op: u32 = 0u32;
        let mut addr: *mut u8 = core::ptr::null_mut();
        let mut data: u8 = 0u8;
        op = (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as u32);
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        addr = (((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read()).wrapping_offset(
            (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as i32) as isize,
        );
        let __p2 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p2).write(((__p2).read()).wrapping_offset(1));
        data = (((track).wrapping_add(64).cast::<*mut u8>()).read()).read();
        let __p3 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p3).write(((__p3).read()).wrapping_offset(1));
        'g_cond_false: {
            'g_cond_true: {
                'l1: {
                    let __sw4 = op;
                    let __matched = __sw4 == 0u32
                        || __sw4 == 1u32
                        || __sw4 == 2u32
                        || __sw4 == 3u32
                        || __sw4 == 4u32
                        || __sw4 == 5u32
                        || __sw4 == 6u32
                        || __sw4 == 7u32
                        || __sw4 == 8u32
                        || __sw4 == 9u32
                        || __sw4 == 10u32
                        || __sw4 == 11u32
                        || __sw4 == 12u32
                        || __sw4 == 13u32
                        || __sw4 == 14u32
                        || __sw4 == 15u32
                        || __sw4 == 16u32
                        || __sw4 == 17u32;
                    if __sw4 == 0u32 {
                        (addr).write(data);
                        return;
                    }
                    if __sw4 == 1u32 {
                        (addr).write(
                            (((((addr).read()) as i32).wrapping_add(((data) as i32))) as u8),
                        );
                        return;
                    }
                    if __sw4 == 2u32 {
                        (addr).write(
                            (((((addr).read()) as i32).wrapping_sub(((data) as i32))) as u8),
                        );
                        return;
                    }
                    if __sw4 == 3u32 {
                        (addr).write(
                            ((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read(),
                        );
                        return;
                    }
                    if __sw4 == 4u32 {
                        (addr).write(
                            (((((addr).read()) as i32).wrapping_add(
                                ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                    .wrapping_offset(((data) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                        );
                        return;
                    }
                    if __sw4 == 5u32 {
                        (addr).write(
                            (((((addr).read()) as i32).wrapping_sub(
                                ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                    .wrapping_offset(((data) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                        );
                        return;
                    }
                    if __sw4 == 6u32 {
                        if (((addr).read()) as i32) == ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 7u32 {
                        if (((addr).read()) as i32) != ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 8u32 {
                        if (((addr).read()) as i32) > ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 9u32 {
                        if (((addr).read()) as i32) >= ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 10u32 {
                        if (((addr).read()) as i32) <= ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 11u32 {
                        if (((addr).read()) as i32) < ((data) as i32) {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 12u32 {
                        if (((addr).read()) as i32)
                            == ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 13u32 {
                        if (((addr).read()) as i32)
                            != ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 14u32 {
                        if (((addr).read()) as i32)
                            > ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 15u32 {
                        if (((addr).read()) as i32)
                            >= ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 16u32 {
                        if (((addr).read()) as i32)
                            <= ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if __sw4 == 17u32 {
                        if (((addr).read()) as i32)
                            < ((((((mplayInfo).wrapping_add(24).cast::<*mut u8>()).read())
                                .wrapping_offset(((data) as i32) as isize))
                            .read()) as i32)
                        {
                            break 'g_cond_true;
                        } else {
                            break 'g_cond_false;
                        }
                        return;
                    }
                    if !__matched {
                        return;
                    }
                }
            }
            {
                core::mem::transmute::<_, unsafe extern "C" fn(*mut u8, *mut u8)>(
                    (((((&raw mut gMPlayJumpTable)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                    .wrapping_offset(1))
                    .read())
                    .unwrap_unchecked(),
                )(mplayInfo, track);
                return;
            }
        }
        let __p5 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p5).write(((__p5).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xcmd(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        let mut n: u32 = (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as u32);
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
        (((((&raw mut gXcmdTable).cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .wrapping_offset(((n) as i32) as isize))
        .read())
        .unwrap_unchecked()(mplayInfo, track);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xxx(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        core::mem::transmute::<_, unsafe extern "C" fn(*mut u8, *mut u8)>(
            ((((&raw mut gMPlayJumpTable)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked(),
        )(mplayInfo, track);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xwave(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        let mut wav: u32 = 0u32;
        wav = 0u32;
        {
            let mut byte: u32 =
                (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as u32);
            byte = (byte << 0);
            wav = (wav & 4294967040u32);
            wav = (wav | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32);
            byte = (byte << 8);
            wav = (wav & 4294902015u32);
            wav = (wav | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32);
            byte = (byte << 16);
            wav = (wav & 4278255615u32);
            wav = (wav | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as u32);
            byte = (byte << 24);
            wav = (wav & 16777215u32);
            wav = (wav | byte);
        }
        (((track).wrapping_add(36)).wrapping_add(4).cast::<*mut u8>())
            .write(((wav) as usize as *mut u8));
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xtype(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        ((track).wrapping_add(36))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xatta(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(8))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xdeca(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(9))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xsust(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(10))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xrele(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(11))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xiecv(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        ((track).wrapping_add(30))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xiecl(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        ((track).wrapping_add(31))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xleng(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(2))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xswee(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        (((track).wrapping_add(36)).wrapping_add(3))
            .write((((track).wrapping_add(64).cast::<*mut u8>()).read()).read());
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xwait(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        let mut len: u32 = 0u32;
        len = 0u32;
        {
            let mut byte: u32 =
                (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as u32);
            byte = (byte << 0);
            len = (len & 4294967040u32);
            len = (len | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32);
            byte = (byte << 8);
            len = (len & 4294902015u32);
            len = (len | byte);
        }
        if ((((track).wrapping_add(58).cast::<u16>()).read()) as i32) < (((len) as u16) as i32) {
            let __p1 = (track).wrapping_add(58).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (track).wrapping_add(64).cast::<*mut u8>();
            (__p2).write(((__p2).read()).wrapping_offset(-2));
            ((track).wrapping_add(1)).write(1u8);
        } else {
            ((track).wrapping_add(58).cast::<u16>()).write(0u16);
            let __p3 = (track).wrapping_add(64).cast::<*mut u8>();
            (__p3).write(((__p3).read()).wrapping_offset(2));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xcmd_0D(mplayInfo: *mut u8, track: *mut u8) {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track = track;
        let mut unk: u32 = 0u32;
        unk = 0u32;
        {
            let mut byte: u32 =
                (((((track).wrapping_add(64).cast::<*mut u8>()).read()).read()) as u32);
            byte = (byte << 0);
            unk = (unk & 4294967040u32);
            unk = (unk | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(1))
            .read()) as u32);
            byte = (byte << 8);
            unk = (unk & 4294902015u32);
            unk = (unk | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(2))
            .read()) as u32);
            byte = (byte << 16);
            unk = (unk & 4278255615u32);
            unk = (unk | byte);
        }
        {
            let mut byte: u32 = ((((((track).wrapping_add(64).cast::<*mut u8>()).read())
                .wrapping_offset(3))
            .read()) as u32);
            byte = (byte << 24);
            unk = (unk & 16777215u32);
            unk = (unk | byte);
        }
        ((track).wrapping_add(60).cast::<u32>()).write(unk);
        let __p1 = (track).wrapping_add(64).cast::<*mut u8>();
        (__p1).write(((__p1).read()).wrapping_offset(4));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DummyFunc() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryTone(tone: *mut u8) -> *mut u8 {
    unsafe {
        let mut tone = tone;
        let mut maxClock: u32 = 0u32;
        let mut maxClockIndex: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut mplayInfo: *mut u8 = core::ptr::null_mut();
        'g_start_song: {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        let mut track: *mut u8 = (((&raw mut gPokemonCryTracks).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(((i).wrapping_mul(2i32)) as isize * 80);
                        if (!(((track).read()) != 0))
                            && ((!(!(((track).wrapping_add(32).cast::<*mut u8>()).read())
                                .is_null()))
                                || (((((((track).wrapping_add(32).cast::<*mut u8>()).read())
                                    .wrapping_add(44)
                                    .cast::<*mut u8>())
                                .read()) as usize)
                                    != ((track) as usize)))
                        {
                            break 'g_start_song;
                        }
                        if maxClock
                            < (((((&raw mut gPokemonCryMusicPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 64))
                            .wrapping_add(12)
                            .cast::<u32>())
                            .read()
                        {
                            maxClock = (((((&raw mut gPokemonCryMusicPlayers).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset((i) as isize * 64))
                            .wrapping_add(12)
                            .cast::<u32>())
                            .read();
                            maxClockIndex = i;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            i = maxClockIndex;
        }
        mplayInfo = (((&raw mut gPokemonCryMusicPlayers).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 64);
        let __p1 = (mplayInfo).wrapping_add(52).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        (((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 52)
            .cast::<crate::c::Rec4<52>>()
            .write_unaligned(
                (&raw mut gPokemonCrySong)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<52>>()
                    .read_unaligned(),
            );
        (((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 52))
        .wrapping_add(4)
        .cast::<*mut u8>())
        .write(tone);
        ((((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 52))
        .wrapping_add(8))
        .cast::<*mut u8>())
        .write(
            ((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 52))
            .wrapping_add(17),
        );
        (((((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 52))
        .wrapping_add(8))
        .cast::<*mut u8>())
        .wrapping_offset(1))
        .write(
            ((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 52))
            .wrapping_add(24),
        );
        (((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
            .wrapping_offset((i) as isize * 52))
        .wrapping_add(20)
        .cast::<u32>())
        .write(
            ((((((((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 52))
            .wrapping_add(26))
            .cast::<u8>())
            .cast::<u8>()) as usize as u32),
        );
        ((mplayInfo).wrapping_add(52).cast::<u32>()).write(1752395091u32);
        MPlayStart(
            mplayInfo,
            (((&raw mut gPokemonCrySongs).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 52),
        );
        return mplayInfo;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryVolume(val: u8) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(29))
            .write(((((val) as i32) & 127i32) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPanpot(val: i8) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(40))
            .write(((((val) as i32).wrapping_add(64i32) & 127i32) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPitch(val: i16) {
    unsafe {
        let mut val = val;
        let mut b: i16 = ((((val) as i32).wrapping_add(128i32)) as i16);
        let mut a: u8 = (((((((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(25)).read())
            as i32)
            .wrapping_sub(
                (((((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(18)).read()) as i32),
            )) as u8);
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(42))
            .write((((((b) as i32) >> 8) & 127i32) as u8));
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(18))
            .write((((((b) as i32) >> 1) & 127i32) as u8));
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(25))
            .write(((((a) as i32).wrapping_add(((((b) as i32) >> 1) & 127i32)) & 127i32) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryLength(val: u16) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .write(val);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryRelease(val: u8) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(38)).write(val);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryProgress(val: u32) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>())
            .wrapping_add(32)
            .cast::<u32>())
        .write(val);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonCryPlaying(mplayInfo: *mut u8) -> u32 {
    unsafe {
        let mut mplayInfo = mplayInfo;
        let mut track: *mut u8 = ((mplayInfo).wrapping_add(44).cast::<*mut u8>()).read();
        if (!(((track).wrapping_add(32).cast::<*mut u8>()).read()).is_null())
            && (((((((track).wrapping_add(32).cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<*mut u8>())
            .read()) as usize)
                == ((track) as usize))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryChorus(val: i8) {
    unsafe {
        let mut val = val;
        if (val) != 0 {
            ((&raw mut gPokemonCrySong).cast::<u8>()).write(2u8);
            (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(25)).write(
                ((((val) as i32).wrapping_add(
                    (((((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(18)).read()) as i32),
                ) & 127i32) as u8),
            );
        } else {
            ((&raw mut gPokemonCrySong).cast::<u8>()).write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryStereo(val: u32) {
    unsafe {
        let mut val = val;
        let mut soundInfo: *mut u8 = ((50364400i32) as usize as *mut *mut u8).read();
        if (val) != 0 {
            crate::c::volatile_write(((67108994i32) as usize as *mut u16), 8462u16);
            let __p1 = (soundInfo).wrapping_add(9);
            (__p1).write((((((__p1).read()) as i32) & (-2i32)) as u8));
        } else {
            crate::c::volatile_write(((67108994i32) as usize as *mut u16), 13058u16);
            let __p2 = (soundInfo).wrapping_add(9);
            (__p2).write((((((__p2).read()) as i32) | 1i32) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPriority(val: u8) {
    unsafe {
        let mut val = val;
        (((&raw mut gPokemonCrySong).cast::<u8>()).wrapping_add(2)).write(val);
    }
}
