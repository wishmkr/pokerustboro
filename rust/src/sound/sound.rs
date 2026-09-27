//! Translated from `src/sound.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sFanfares
#[allow(unused_imports)]
use crate::data::sound::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMPlay_PokemonCry: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokemonCryBGMDuckingCounter: u8 = 0u8;
pub(crate) static mut sCurrentMapMusic: u16 = 0u16;
pub(crate) static mut sNextMapMusic: u16 = 0u16;
pub(crate) static mut sMapMusicState: u8 = 0u8;
pub(crate) static mut sMapMusicFadeInSpeed: u8 = 0u8;
pub(crate) static mut sFanfareCounter: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDisableMusic: u8 = 0u8;

unsafe extern "C" {
    static mut gBattleTypeFlags: u8;
    static mut gCryTable: u8;
    static mut gCryTable_Reverse: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMPlayInfo_SE3: u8;
    fn ClearPokemonCrySongs();
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IsPokemonCryPlaying(a0: *mut u8) -> u32;
    fn SetPokemonCryChorus(a0: i8);
    fn SetPokemonCryLength(a0: u16);
    fn SetPokemonCryPanpot(a0: i8);
    fn SetPokemonCryPitch(a0: i16);
    fn SetPokemonCryPriority(a0: u8);
    fn SetPokemonCryProgress(a0: u32);
    fn SetPokemonCryRelease(a0: u8);
    fn SetPokemonCryTone(a0: *mut u8) -> *mut u8;
    fn SetPokemonCryVolume(a0: u8);
    fn SpeciesToCryId(a0: u16) -> u16;
    fn m4aMPlayContinue(a0: *mut u8);
    fn m4aMPlayFadeIn(a0: *mut u8, a1: u16);
    fn m4aMPlayFadeOut(a0: *mut u8, a1: u16);
    fn m4aMPlayFadeOutTemporarily(a0: *mut u8, a1: u16);
    fn m4aMPlayImmInit(a0: *mut u8);
    fn m4aMPlayPanpotControl(a0: *mut u8, a1: u16, a2: i8);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
    fn m4aSongNumStart(a0: u16);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMapMusic() {
    unsafe {
        ((&raw mut gDisableMusic).cast::<u8>().cast::<u8>()).write(0u8);
        ResetMapMusic();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapMusicMain() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(2u8);
                PlayBGM(((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (IsBGMStopped()) != 0 {
                    ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
                    ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((IsBGMStopped()) != 0) && ((IsFanfareTaskInactive()) != 0) {
                    ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>())
                        .write(((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).read());
                    ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
                    ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(2u8);
                    PlayBGM(((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).read());
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((IsBGMStopped()) != 0) && ((IsFanfareTaskInactive()) != 0) {
                    FadeInNewBGM(
                        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).read(),
                        ((&raw mut sMapMusicFadeInSpeed).cast::<u8>().cast::<u8>()).read(),
                    );
                    ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>())
                        .write(((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).read());
                    ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
                    ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(2u8);
                    ((&raw mut sMapMusicFadeInSpeed).cast::<u8>().cast::<u8>()).write(0u8);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetMapMusic() {
    unsafe {
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sMapMusicFadeInSpeed).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapMusic() -> u16 {
    unsafe {
        return ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayNewMapMusic(songNum: u16) {
    unsafe {
        let mut songNum = songNum;
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(songNum);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopMapMusic() {
    unsafe {
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutMapMusic(speed: u8) {
    unsafe {
        let mut speed = speed;
        if (IsNotWaitingForBGMStop()) != 0 {
            FadeOutBGM(speed);
        }
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(5u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutAndPlayNewMapMusic(songNum: u16, speed: u8) {
    unsafe {
        let mut songNum = songNum;
        let mut speed = speed;
        FadeOutMapMusic(speed);
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(songNum);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(6u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutAndFadeInNewMapMusic(
    songNum: u16,
    fadeOutSpeed: u8,
    fadeInSpeed: u8,
) {
    unsafe {
        let mut songNum = songNum;
        let mut fadeOutSpeed = fadeOutSpeed;
        let mut fadeInSpeed = fadeInSpeed;
        FadeOutMapMusic(fadeOutSpeed);
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(songNum);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(7u8);
        ((&raw mut sMapMusicFadeInSpeed).cast::<u8>().cast::<u8>()).write(fadeInSpeed);
    }
}
pub(crate) unsafe extern "C" fn FadeInNewMapMusic(songNum: u16, speed: u8) {
    unsafe {
        let mut songNum = songNum;
        let mut speed = speed;
        FadeInNewBGM(songNum, speed);
        ((&raw mut sCurrentMapMusic).cast::<u8>().cast::<u16>()).write(songNum);
        ((&raw mut sNextMapMusic).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).write(2u8);
        ((&raw mut sMapMusicFadeInSpeed).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsNotWaitingForBGMStop() -> u8 {
    unsafe {
        if ((((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).read()) as i32) == 6i32 {
            return 0u8;
        }
        if ((((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).read()) as i32) == 5i32 {
            return 0u8;
        }
        if ((((&raw mut sMapMusicState).cast::<u8>().cast::<u8>()).read()) as i32) == 7i32 {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayFanfareByFanfareNum(fanfareNum: u8) {
    unsafe {
        let mut fanfareNum = fanfareNum;
        let mut songNum: u16 = 0u16;
        m4aMPlayStop((&raw mut gMPlayInfo_BGM).cast::<u8>());
        songNum = (((((&raw const sFanfares).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((fanfareNum) as i32) as isize * 4))
        .cast::<u16>())
        .read();
        ((&raw mut sFanfareCounter).cast::<u8>().cast::<u16>()).write(
            (((((&raw const sFanfares).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((fanfareNum) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        m4aSongNumStart(songNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitFanfare(stop: u8) -> u8 {
    unsafe {
        let mut stop = stop;
        if (((&raw mut sFanfareCounter).cast::<u8>().cast::<u16>()).read()) != 0 {
            let __p1 = (&raw mut sFanfareCounter).cast::<u8>().cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return 0u8;
        } else {
            if !((stop) != 0) {
                m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
            } else {
                m4aSongNumStart(0u16);
            }
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopFanfareByFanfareNum(fanfareNum: u8) {
    unsafe {
        let mut fanfareNum = fanfareNum;
        m4aSongNumStop(
            (((((&raw const sFanfares).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((fanfareNum) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayFanfare(songNum: u16) {
    unsafe {
        let mut songNum = songNum;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(72u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sFanfares).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((songNum) as i32)
                    {
                        PlayFanfareByFanfareNum(((i) as u8));
                        CreateFanfareTask();
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        PlayFanfareByFanfareNum(0u8);
        CreateFanfareTask();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFanfareTaskInactive() -> u8 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_Fanfare))) as i32) == 1i32 {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_Fanfare(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((&raw mut sFanfareCounter).cast::<u8>().cast::<u16>()).read()) != 0 {
            let __p1 = (&raw mut sFanfareCounter).cast::<u8>().cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            m4aMPlayContinue((&raw mut gMPlayInfo_BGM).cast::<u8>());
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateFanfareTask() {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_Fanfare))) as i32) != 1i32 {
            CreateTask(Some(Task_Fanfare), 80u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInNewBGM(songNum: u16, speed: u8) {
    unsafe {
        let mut songNum = songNum;
        let mut speed = speed;
        if (((&raw mut gDisableMusic).cast::<u8>().cast::<u8>()).read()) != 0 {
            songNum = 0u16;
        }
        if ((songNum) as i32) == 65535i32 {
            songNum = 0u16;
        }
        m4aSongNumStart(songNum);
        m4aMPlayImmInit((&raw mut gMPlayInfo_BGM).cast::<u8>());
        m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 0u16);
        m4aSongNumStop(songNum);
        m4aMPlayFadeIn((&raw mut gMPlayInfo_BGM).cast::<u8>(), ((speed) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBGMTemporarily(speed: u8) {
    unsafe {
        let mut speed = speed;
        m4aMPlayFadeOutTemporarily((&raw mut gMPlayInfo_BGM).cast::<u8>(), ((speed) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMPausedOrStopped() -> u8 {
    unsafe {
        if ((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 2147483648u32)
            != 0
        {
            return 1u8;
        }
        if !(((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 65535u32)
            != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInBGM(speed: u8) {
    unsafe {
        let mut speed = speed;
        m4aMPlayFadeIn((&raw mut gMPlayInfo_BGM).cast::<u8>(), ((speed) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBGM(speed: u8) {
    unsafe {
        let mut speed = speed;
        m4aMPlayFadeOut((&raw mut gMPlayInfo_BGM).cast::<u8>(), ((speed) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMStopped() -> u8 {
    unsafe {
        if !(((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 65535u32)
            != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_Normal(species: u16, pan: i8) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 85u16);
        PlayCryInternal(species, pan, 120i8, 10u8, 0u8);
        ((&raw mut gPokemonCryBGMDuckingCounter)
            .cast::<u8>()
            .cast::<u8>())
        .write(2u8);
        RestoreBGMVolumeAfterPokemonCry();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_NormalNoDucking(species: u16, pan: i8, volume: i8, priority: u8) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        let mut volume = volume;
        let mut priority = priority;
        PlayCryInternal(species, pan, volume, priority, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_ByMode(species: u16, pan: i8, mode: u8) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        let mut mode = mode;
        if ((mode) as i32) == 1i32 {
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
        } else {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 85u16);
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
            ((&raw mut gPokemonCryBGMDuckingCounter)
                .cast::<u8>()
                .cast::<u8>())
            .write(2u8);
            RestoreBGMVolumeAfterPokemonCry();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_ReleaseDouble(species: u16, pan: i8, mode: u8) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        let mut mode = mode;
        if ((mode) as i32) == 1i32 {
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
        } else {
            if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0) {
                m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 85u16);
            }
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_DuckNoRestore(species: u16, pan: i8, mode: u8) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        let mut mode = mode;
        if ((mode) as i32) == 1i32 {
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
        } else {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 85u16);
            PlayCryInternal(species, pan, 120i8, 10u8, mode);
            ((&raw mut gPokemonCryBGMDuckingCounter)
                .cast::<u8>()
                .cast::<u8>())
            .write(2u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_Script(species: u16, mode: u8) {
    unsafe {
        let mut species = species;
        let mut mode = mode;
        m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 85u16);
        PlayCryInternal(species, 0i8, 120i8, 10u8, mode);
        ((&raw mut gPokemonCryBGMDuckingCounter)
            .cast::<u8>()
            .cast::<u8>())
        .write(2u8);
        RestoreBGMVolumeAfterPokemonCry();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCryInternal(
    species: u16,
    pan: i8,
    volume: i8,
    priority: u8,
    mode: u8,
) {
    unsafe {
        let mut species = species;
        let mut pan = pan;
        let mut volume = volume;
        let mut priority = priority;
        let mut mode = mode;
        let mut reverse: u32 = 0u32;
        let mut release: u32 = 0u32;
        let mut length: u32 = 0u32;
        let mut pitch: u32 = 0u32;
        let mut chorus: u32 = 0u32;
        let mut index: u32 = 0u32;
        let mut table: u8 = 0u8;
        species = (species).wrapping_sub(1);
        length = 140u32;
        reverse = 0u32;
        release = 0u32;
        pitch = 15360u32;
        chorus = 0u32;
        'l1: {
            let __sw1 = ((mode) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                length = 20u32;
                release = 225u32;
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                release = 225u32;
                pitch = 15600u32;
                chorus = 20u32;
                volume = 90i8;
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                length = 50u32;
                release = 200u32;
                pitch = 15800u32;
                chorus = 20u32;
                volume = 90i8;
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                length = 25u32;
                reverse = 1u32;
                release = 100u32;
                pitch = 15600u32;
                chorus = 192u32;
                volume = 90i8;
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                release = 200u32;
                pitch = 14440u32;
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                release = 220u32;
                pitch = 15555u32;
                chorus = 192u32;
                volume = 70i8;
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                length = 10u32;
                release = 100u32;
                pitch = 14848u32;
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                length = 60u32;
                release = 225u32;
                pitch = 15616u32;
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                length = 15u32;
                reverse = 1u32;
                release = 125u32;
                pitch = 15200u32;
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                length = 100u32;
                release = 225u32;
                pitch = 15200u32;
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                length = 20u32;
                release = 225u32;
            }
            if __fall || __sw1 == 11i32 {
                __fall = true;
                pitch = 15000u32;
                break 'l1;
            }
        }
        SetPokemonCryVolume(((volume) as u8));
        SetPokemonCryPanpot(pan);
        SetPokemonCryPitch(((pitch) as i16));
        SetPokemonCryLength(((length) as u16));
        SetPokemonCryProgress(0u32);
        SetPokemonCryRelease(((release) as u8));
        SetPokemonCryChorus(((chorus) as i8));
        SetPokemonCryPriority(priority);
        species = SpeciesToCryId(species);
        index = ((crate::c::rem_i32(((species) as i32), 128i32)) as u32);
        table = ((crate::c::div_i32(((species) as i32), 128i32)) as u8);
        'l2: {
            let __sw2 = ((table) as i32);
            if __sw2 == 0i32 {
                ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).write(
                    SetPokemonCryTone(
                        (if (reverse) != 0 {
                            ((&raw mut gCryTable_Reverse).cast::<u8>()).wrapping_offset(
                                (((0u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        } else {
                            ((&raw mut gCryTable).cast::<u8>()).wrapping_offset(
                                (((0u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        }),
                    ),
                );
                break 'l2;
            }
            if __sw2 == 1i32 {
                ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).write(
                    SetPokemonCryTone(
                        (if (reverse) != 0 {
                            ((&raw mut gCryTable_Reverse).cast::<u8>()).wrapping_offset(
                                (((128u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        } else {
                            ((&raw mut gCryTable).cast::<u8>()).wrapping_offset(
                                (((128u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        }),
                    ),
                );
                break 'l2;
            }
            if __sw2 == 2i32 {
                ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).write(
                    SetPokemonCryTone(
                        (if (reverse) != 0 {
                            ((&raw mut gCryTable_Reverse).cast::<u8>()).wrapping_offset(
                                (((256u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        } else {
                            ((&raw mut gCryTable).cast::<u8>()).wrapping_offset(
                                (((256u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        }),
                    ),
                );
                break 'l2;
            }
            if __sw2 == 3i32 {
                ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).write(
                    SetPokemonCryTone(
                        (if (reverse) != 0 {
                            ((&raw mut gCryTable_Reverse).cast::<u8>()).wrapping_offset(
                                (((384u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        } else {
                            ((&raw mut gCryTable).cast::<u8>()).wrapping_offset(
                                (((384u32).wrapping_add(index)) as i32) as isize * 12,
                            )
                        }),
                    ),
                );
                break 'l2;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryFinished() -> u8 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry))) as i32) == 1i32 {
            return 0u8;
        } else {
            ClearPokemonCrySongs();
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopCryAndClearCrySongs() {
    unsafe {
        m4aMPlayStop(((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).read());
        ClearPokemonCrySongs();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopCry() {
    unsafe {
        m4aMPlayStop(((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryPlayingOrClearCrySongs() -> u8 {
    unsafe {
        if (IsPokemonCryPlaying(
            ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).read(),
        )) != 0
        {
            return 1u8;
        } else {
            ClearPokemonCrySongs();
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryPlaying() -> u8 {
    unsafe {
        if (IsPokemonCryPlaying(
            ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).read(),
        )) != 0
        {
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
pub(crate) unsafe extern "C" fn Task_DuckBGMForPokemonCry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((&raw mut gPokemonCryBGMDuckingCounter)
            .cast::<u8>()
            .cast::<u8>())
        .read())
            != 0
        {
            let __p1 = (&raw mut gPokemonCryBGMDuckingCounter)
                .cast::<u8>()
                .cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        if !((IsPokemonCryPlaying(
            ((&raw mut gMPlay_PokemonCry).cast::<u8>().cast::<*mut u8>()).read(),
        )) != 0)
        {
            m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 256u16);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreBGMVolumeAfterPokemonCry() {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry))) as i32) != 1i32 {
            CreateTask(Some(Task_DuckBGMForPokemonCry), 80u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayBGM(songNum: u16) {
    unsafe {
        let mut songNum = songNum;
        if (((&raw mut gDisableMusic).cast::<u8>().cast::<u8>()).read()) != 0 {
            songNum = 0u16;
        }
        if ((songNum) as i32) == 65535i32 {
            songNum = 0u16;
        }
        m4aSongNumStart(songNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE(songNum: u16) {
    unsafe {
        let mut songNum = songNum;
        m4aSongNumStart(songNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE12WithPanning(songNum: u16, pan: i8) {
    unsafe {
        let mut songNum = songNum;
        let mut pan = pan;
        m4aSongNumStart(songNum);
        m4aMPlayImmInit((&raw mut gMPlayInfo_SE1).cast::<u8>());
        m4aMPlayImmInit((&raw mut gMPlayInfo_SE2).cast::<u8>());
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE1).cast::<u8>(), 65535u16, pan);
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE2).cast::<u8>(), 65535u16, pan);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE1WithPanning(songNum: u16, pan: i8) {
    unsafe {
        let mut songNum = songNum;
        let mut pan = pan;
        m4aSongNumStart(songNum);
        m4aMPlayImmInit((&raw mut gMPlayInfo_SE1).cast::<u8>());
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE1).cast::<u8>(), 65535u16, pan);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE2WithPanning(songNum: u16, pan: i8) {
    unsafe {
        let mut songNum = songNum;
        let mut pan = pan;
        m4aSongNumStart(songNum);
        m4aMPlayImmInit((&raw mut gMPlayInfo_SE2).cast::<u8>());
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE2).cast::<u8>(), 65535u16, pan);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SE12PanpotControl(pan: i8) {
    unsafe {
        let mut pan = pan;
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE1).cast::<u8>(), 65535u16, pan);
        m4aMPlayPanpotControl((&raw mut gMPlayInfo_SE2).cast::<u8>(), 65535u16, pan);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSEPlaying() -> u8 {
    unsafe {
        if (((((&raw mut gMPlayInfo_SE1).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 2147483648u32)
            != 0)
            && (((((&raw mut gMPlayInfo_SE2).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                & 2147483648u32)
                != 0)
        {
            return 0u8;
        }
        if (!(((((&raw mut gMPlayInfo_SE1).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 65535u32)
            != 0))
            && (!(((((&raw mut gMPlayInfo_SE2).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                & 65535u32)
                != 0))
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMPlaying() -> u8 {
    unsafe {
        if ((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 2147483648u32)
            != 0
        {
            return 0u8;
        }
        if !(((((&raw mut gMPlayInfo_BGM).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 65535u32)
            != 0)
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpecialSEPlaying() -> u8 {
    unsafe {
        if ((((&raw mut gMPlayInfo_SE3).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 2147483648u32)
            != 0
        {
            return 0u8;
        }
        if !(((((&raw mut gMPlayInfo_SE3).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .read()
            & 65535u32)
            != 0)
        {
            return 0u8;
        }
        return 1u8;
    }
}
