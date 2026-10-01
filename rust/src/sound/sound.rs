//! Translated from `src/sound.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    dead_code,
    unused_assignments
)]

use crate::agb_main::ClearPokemonCrySongs;
use crate::battle_main::gBattleTypeFlags;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::m4a::{
    IsPokemonCryPlaying, SetPokemonCryChorus, SetPokemonCryLength, SetPokemonCryPanpot,
    SetPokemonCryPitch, SetPokemonCryPriority, SetPokemonCryProgress, SetPokemonCryRelease,
    SetPokemonCryTone, SetPokemonCryVolume, gMPlayInfo_BGM, gMPlayInfo_SE1, gMPlayInfo_SE2,
    gMPlayInfo_SE3, m4aMPlayContinue, m4aMPlayFadeIn, m4aMPlayFadeOut, m4aMPlayFadeOutTemporarily,
    m4aMPlayImmInit, m4aMPlayPanpotControl, m4aMPlayStop, m4aMPlayVolumeControl, m4aSongNumStart,
    m4aSongNumStop,
};
use crate::pokemon::SpeciesToCryId;
use crate::task::DestroyTask;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
// Data tables (translate with cdata.py): sFanfares

/// `struct Fanfare`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Fanfare {
    pub songNum: u16,
    pub duration: u16,
}

unsafe impl Sync for Fanfare {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Fanfare>() == 4);
    assert!(offset_of!(Fanfare, songNum) == 0);
    assert!(offset_of!(Fanfare, duration) == 2);
};

static sFanfares: Table<CArray<Fanfare, 18>> =
    Table((&raw const crate::data::sound::sFanfares).cast());

#[unsafe(link_section = "ewram_data")]
pub static mut gMPlay_PokemonCry: *mut MusicPlayerInfo = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static gPokemonCryBGMDuckingCounter: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sCurrentMapMusic: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sNextMapMusic: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sMapMusicState: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sMapMusicFadeInSpeed: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sFanfareCounter: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gDisableMusic: crate::global::Global<u8> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
pub fn InitMapMusic() {
    gDisableMusic.set(FALSE);
    ResetMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe fn MapMusicMain() {
    match sMapMusicState.get() {
        0 => {}
        1 => {
            sMapMusicState.set(2);
            PlayBGM(sCurrentMapMusic.get());
        }
        2..=4 => {}
        5 => {
            if IsBGMStopped() != 0 {
                sNextMapMusic.set(0);
                sMapMusicState.set(0);
            }
        }
        6 => {
            if IsBGMStopped() != 0 && IsFanfareTaskInactive() != 0 {
                sCurrentMapMusic.set(sNextMapMusic.get());
                sNextMapMusic.set(0);
                sMapMusicState.set(2);
                PlayBGM(sCurrentMapMusic.get());
            }
        }
        7 if IsBGMStopped() != 0 && IsFanfareTaskInactive() != 0 => {
            FadeInNewBGM(sNextMapMusic.get(), sMapMusicFadeInSpeed.get());
            sCurrentMapMusic.set(sNextMapMusic.get());
            sNextMapMusic.set(0);
            sMapMusicState.set(2);
            sMapMusicFadeInSpeed.set(0);
        }
        _ => {}
    }
}
pub fn ResetMapMusic() {
    sCurrentMapMusic.set(0);
    sNextMapMusic.set(0);
    sMapMusicState.set(0);
    sMapMusicFadeInSpeed.set(0);
}
pub unsafe fn GetCurrentMapMusic() -> u16 {
    sCurrentMapMusic.get()
}
pub unsafe fn PlayNewMapMusic(songNum: u16) {
    sCurrentMapMusic.set(songNum);
    sNextMapMusic.set(0);
    sMapMusicState.set(1);
}
#[unsafe(no_mangle)]
pub fn StopMapMusic() {
    sCurrentMapMusic.set(0);
    sNextMapMusic.set(0);
    sMapMusicState.set(1);
}
pub unsafe fn FadeOutMapMusic(speed: u8) {
    if IsNotWaitingForBGMStop() != 0 {
        FadeOutBGM(speed);
    }
    sCurrentMapMusic.set(0);
    sNextMapMusic.set(0);
    sMapMusicState.set(5);
}
pub unsafe fn FadeOutAndPlayNewMapMusic(songNum: u16, speed: u8) {
    FadeOutMapMusic(speed);
    sCurrentMapMusic.set(0);
    sNextMapMusic.set(songNum);
    sMapMusicState.set(6);
}
pub unsafe fn FadeOutAndFadeInNewMapMusic(songNum: u16, fadeOutSpeed: u8, fadeInSpeed: u8) {
    FadeOutMapMusic(fadeOutSpeed);
    sCurrentMapMusic.set(0);
    sNextMapMusic.set(songNum);
    sMapMusicState.set(7);
    sMapMusicFadeInSpeed.set(fadeInSpeed);
}
unsafe fn FadeInNewMapMusic(songNum: u16, speed: u8) {
    FadeInNewBGM(songNum, speed);
    sCurrentMapMusic.set(songNum);
    sNextMapMusic.set(0);
    sMapMusicState.set(2);
    sMapMusicFadeInSpeed.set(0);
}
pub fn IsNotWaitingForBGMStop() -> u8 {
    if sMapMusicState.get() == 6 {
        return FALSE;
    }
    if sMapMusicState.get() == 5 {
        return FALSE;
    }
    if sMapMusicState.get() == 7 {
        return FALSE;
    }
    TRUE
}
pub unsafe fn PlayFanfareByFanfareNum(fanfareNum: u8) {
    let mut songNum: u16 = 0;
    m4aMPlayStop(&raw mut gMPlayInfo_BGM);
    songNum = sFanfares[fanfareNum].songNum;
    sFanfareCounter.set(sFanfares[fanfareNum].duration);
    m4aSongNumStart(songNum);
}
pub unsafe fn WaitFanfare(stop: u8) -> u8 {
    if sFanfareCounter.get() != 0 {
        sFanfareCounter.set(sFanfareCounter.get() - 1);
        return FALSE;
    } else {
        if stop == 0 {
            m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
        } else {
            m4aSongNumStart(MUS_DUMMY);
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn StopFanfareByFanfareNum(fanfareNum: u8) {
    m4aSongNumStop(sFanfares[fanfareNum].songNum);
}
pub unsafe fn PlayFanfare(songNum: u16) {
    let mut i: i32 = 0;
    while (i as u32) < 18 {
        if sFanfares[i].songNum == songNum {
            PlayFanfareByFanfareNum(i as u8);
            CreateFanfareTask();
            return;
        }
        i += 1;
    }
    PlayFanfareByFanfareNum(0);
    CreateFanfareTask();
}
pub unsafe fn IsFanfareTaskInactive() -> u8 {
    if FuncIsActiveTask(Some(Task_Fanfare)) == TRUE {
        return FALSE;
    }
    TRUE
}
pub(crate) unsafe fn Task_Fanfare(taskId: u8) {
    if sFanfareCounter.get() != 0 {
        sFanfareCounter.set(sFanfareCounter.get() - 1);
    } else {
        m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
        DestroyTask(taskId);
    }
}
unsafe fn CreateFanfareTask() {
    if FuncIsActiveTask(Some(Task_Fanfare)) != TRUE {
        CreateTask(Some(Task_Fanfare), 80);
    }
}
pub unsafe fn FadeInNewBGM(mut songNum: u16, speed: u8) {
    if gDisableMusic.get() != 0 {
        songNum = 0;
    }
    if songNum == MUS_NONE {
        songNum = 0;
    }
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_BGM);
    m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 0);
    m4aSongNumStop(songNum);
    m4aMPlayFadeIn(&raw mut gMPlayInfo_BGM, speed as u16);
}
pub unsafe fn FadeOutBGMTemporarily(speed: u8) {
    m4aMPlayFadeOutTemporarily(&raw mut gMPlayInfo_BGM, speed as u16);
}
pub unsafe fn IsBGMPausedOrStopped() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return TRUE;
    }
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return TRUE;
    }
    FALSE
}
pub unsafe fn FadeInBGM(speed: u8) {
    m4aMPlayFadeIn(&raw mut gMPlayInfo_BGM, speed as u16);
}
pub unsafe fn FadeOutBGM(speed: u8) {
    m4aMPlayFadeOut(&raw mut gMPlayInfo_BGM, speed as u16);
}
pub unsafe fn IsBGMStopped() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return TRUE;
    }
    FALSE
}
pub unsafe fn PlayCry_Normal(species: u16, pan: i8) {
    m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
    PlayCryInternal(
        species,
        pan,
        CRY_VOLUME,
        CRY_PRIORITY_NORMAL,
        CRY_MODE_NORMAL,
    );
    gPokemonCryBGMDuckingCounter.set(2);
    RestoreBGMVolumeAfterPokemonCry();
}
pub unsafe fn PlayCry_NormalNoDucking(species: u16, pan: i8, volume: i8, priority: u8) {
    PlayCryInternal(species, pan, volume, priority, CRY_MODE_NORMAL);
}
pub unsafe fn PlayCry_ByMode(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
        gPokemonCryBGMDuckingCounter.set(2);
        RestoreBGMVolumeAfterPokemonCry();
    }
}
pub unsafe fn PlayCry_ReleaseDouble(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        }
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    }
}
pub unsafe fn PlayCry_DuckNoRestore(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
        gPokemonCryBGMDuckingCounter.set(2);
    }
}
pub unsafe fn PlayCry_Script(species: u16, mode: u8) {
    m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
    PlayCryInternal(species, 0, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    gPokemonCryBGMDuckingCounter.set(2);
    RestoreBGMVolumeAfterPokemonCry();
}
pub unsafe fn PlayCryInternal(mut species: u16, pan: i8, mut volume: i8, priority: u8, mode: u8) {
    species -= 1;
    let mut length: u32 = 140;
    let mut reverse: u32 = FALSE as u32;
    let mut release: u32 = 0;
    let mut pitch: u32 = 15360;
    let mut chorus: u32 = 0;
    'l1: {
        let sw1: u8 = mode;
        let mut fall = false;
        if sw1 == CRY_MODE_NORMAL {
            break 'l1;
        }
        if sw1 == CRY_MODE_DOUBLES {
            length = 20;
            release = 225;
            break 'l1;
        }
        if sw1 == CRY_MODE_ENCOUNTER {
            release = 225;
            pitch = 15600;
            chorus = 20;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_HIGH_PITCH {
            length = 50;
            release = 200;
            pitch = 15800;
            chorus = 20;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_ECHO_START {
            length = 25;
            reverse = TRUE as u32;
            release = 100;
            pitch = 15600;
            chorus = 192;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_FAINT {
            release = 200;
            pitch = 14440;
            break 'l1;
        }
        if sw1 == CRY_MODE_ECHO_END {
            release = 220;
            pitch = 15555;
            chorus = 192;
            volume = 70;
            break 'l1;
        }
        if sw1 == CRY_MODE_ROAR_1 {
            length = 10;
            release = 100;
            pitch = 14848;
            break 'l1;
        }
        if sw1 == CRY_MODE_ROAR_2 {
            length = 60;
            release = 225;
            pitch = 15616;
            break 'l1;
        }
        if sw1 == CRY_MODE_GROWL_1 {
            length = 15;
            reverse = TRUE as u32;
            release = 125;
            pitch = 15200;
            break 'l1;
        }
        if sw1 == CRY_MODE_GROWL_2 {
            length = 100;
            release = 225;
            pitch = 15200;
            break 'l1;
        }
        if sw1 == CRY_MODE_WEAK_DOUBLES {
            fall = true;
            length = 20;
            release = 225;
        }
        if fall || sw1 == CRY_MODE_WEAK {
            pitch = 15000;
            break 'l1;
        }
    }
    SetPokemonCryVolume(volume as u8);
    SetPokemonCryPanpot(pan);
    SetPokemonCryPitch(pitch as i16);
    SetPokemonCryLength(length as u16);
    SetPokemonCryProgress(0);
    SetPokemonCryRelease(release as u8);
    SetPokemonCryChorus(chorus as i8);
    SetPokemonCryPriority(priority);
    species = SpeciesToCryId(species);
    let index: u32 = (species as i32 % 128) as u32;
    let table: u8 = (species as i32 / 128) as u8;
    match table {
        0 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut (*crate::asmdata::gCryTable_Reverse
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[index]
            } else {
                &raw mut (*crate::asmdata::gCryTable
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[index]
            });
        }
        1 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut (*crate::asmdata::gCryTable_Reverse
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[128 + index]
            } else {
                &raw mut (*crate::asmdata::gCryTable
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[128 + index]
            });
        }
        2 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut (*crate::asmdata::gCryTable_Reverse
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[256 + index]
            } else {
                &raw mut (*crate::asmdata::gCryTable
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[256 + index]
            });
        }
        3 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut (*crate::asmdata::gCryTable_Reverse
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[384 + index]
            } else {
                &raw mut (*crate::asmdata::gCryTable
                    .cast::<CArray<ToneData, 0>>()
                    .cast_mut())[384 + index]
            });
        }
        _ => {}
    }
}
pub unsafe fn IsCryFinished() -> u8 {
    if FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry)) == TRUE {
        return FALSE;
    } else {
        ClearPokemonCrySongs();
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn StopCryAndClearCrySongs() {
    m4aMPlayStop(gMPlay_PokemonCry);
    ClearPokemonCrySongs();
}
pub unsafe fn StopCry() {
    m4aMPlayStop(gMPlay_PokemonCry);
}
pub unsafe fn IsCryPlayingOrClearCrySongs() -> u8 {
    if IsPokemonCryPlaying(gMPlay_PokemonCry) != 0 {
        return TRUE;
    } else {
        ClearPokemonCrySongs();
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn IsCryPlaying() -> u8 {
    if IsPokemonCryPlaying(gMPlay_PokemonCry) != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_DuckBGMForPokemonCry(taskId: u8) {
    if gPokemonCryBGMDuckingCounter.get() != 0 {
        gPokemonCryBGMDuckingCounter.set(gPokemonCryBGMDuckingCounter.get() - 1);
        return;
    }
    if IsPokemonCryPlaying(gMPlay_PokemonCry) == 0 {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 256);
        DestroyTask(taskId);
    }
}
unsafe fn RestoreBGMVolumeAfterPokemonCry() {
    if FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry)) != TRUE {
        CreateTask(Some(Task_DuckBGMForPokemonCry), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PlayBGM(mut songNum: u16) {
    if gDisableMusic.get() != 0 {
        songNum = 0;
    }
    if songNum == MUS_NONE {
        songNum = 0;
    }
    m4aSongNumStart(songNum);
}
#[unsafe(no_mangle)]
pub unsafe fn PlaySE(songNum: u16) {
    m4aSongNumStart(songNum);
}
pub unsafe fn PlaySE12WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE1);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE2);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
pub unsafe fn PlaySE1WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE1);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
}
pub unsafe fn PlaySE2WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE2);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
pub unsafe fn SE12PanpotControl(pan: i8) {
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
#[unsafe(no_mangle)]
pub unsafe fn IsSEPlaying() -> u8 {
    if gMPlayInfo_SE1.status & MUSICPLAYER_STATUS_PAUSE != 0
        && gMPlayInfo_SE2.status & MUSICPLAYER_STATUS_PAUSE != 0
    {
        return FALSE;
    }
    if gMPlayInfo_SE1.status & MUSICPLAYER_STATUS_TRACK == 0
        && gMPlayInfo_SE2.status & MUSICPLAYER_STATUS_TRACK == 0
    {
        return FALSE;
    }
    TRUE
}
pub unsafe fn IsBGMPlaying() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return FALSE;
    }
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return FALSE;
    }
    TRUE
}
pub unsafe fn IsSpecialSEPlaying() -> u8 {
    if gMPlayInfo_SE3.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return FALSE;
    }
    if gMPlayInfo_SE3.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return FALSE;
    }
    TRUE
}
