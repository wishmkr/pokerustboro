//! Translated from `src/sound.c` by tools/rustport/c2rs.py.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMPlay_PokemonCry: *mut MusicPlayerInfo = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPokemonCryBGMDuckingCounter: u8 = 0;
pub(crate) static mut sCurrentMapMusic: u16 = 0;
pub(crate) static mut sNextMapMusic: u16 = 0;
pub(crate) static mut sMapMusicState: u8 = 0;
pub(crate) static mut sMapMusicFadeInSpeed: u8 = 0;
pub(crate) static mut sFanfareCounter: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gDisableMusic: u8 = 0;

unsafe extern "C" {
    static mut gBattleTypeFlags: u32;
    static mut gCryTable: CArray<ToneData, 0>;
    static mut gCryTable_Reverse: CArray<ToneData, 0>;
    static mut gMPlayInfo_BGM: MusicPlayerInfo;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMPlayInfo_SE3: MusicPlayerInfo;
    fn ClearPokemonCrySongs();
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn IsPokemonCryPlaying(a0: *mut MusicPlayerInfo) -> u32;
    fn SetPokemonCryChorus(a0: i8);
    fn SetPokemonCryLength(a0: u16);
    fn SetPokemonCryPanpot(a0: i8);
    fn SetPokemonCryPitch(a0: i16);
    fn SetPokemonCryPriority(a0: u8);
    fn SetPokemonCryProgress(a0: u32);
    fn SetPokemonCryRelease(a0: u8);
    fn SetPokemonCryTone(a0: *mut ToneData) -> *mut MusicPlayerInfo;
    fn SetPokemonCryVolume(a0: u8);
    fn SpeciesToCryId(a0: u16) -> u16;
    fn m4aMPlayContinue(a0: *mut MusicPlayerInfo);
    fn m4aMPlayFadeIn(a0: *mut MusicPlayerInfo, a1: u16);
    fn m4aMPlayFadeOut(a0: *mut MusicPlayerInfo, a1: u16);
    fn m4aMPlayFadeOutTemporarily(a0: *mut MusicPlayerInfo, a1: u16);
    fn m4aMPlayImmInit(a0: *mut MusicPlayerInfo);
    fn m4aMPlayPanpotControl(a0: *mut MusicPlayerInfo, a1: u16, a2: i8);
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn m4aMPlayVolumeControl(a0: *mut MusicPlayerInfo, a1: u16, a2: u16);
    fn m4aSongNumStart(a0: u16);
    fn m4aSongNumStop(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMapMusic() {
    gDisableMusic = FALSE;
    ResetMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MapMusicMain() {
    match sMapMusicState {
        0 => {}
        1 => {
            sMapMusicState = 2;
            PlayBGM(sCurrentMapMusic);
        }
        2 | 3 | 4 => {}
        5 => {
            if IsBGMStopped() != 0 {
                sNextMapMusic = 0;
                sMapMusicState = 0;
            }
        }
        6 => {
            if IsBGMStopped() != 0 && IsFanfareTaskInactive() != 0 {
                sCurrentMapMusic = sNextMapMusic;
                sNextMapMusic = 0;
                sMapMusicState = 2;
                PlayBGM(sCurrentMapMusic);
            }
        }
        7 => {
            if IsBGMStopped() != 0 && IsFanfareTaskInactive() != 0 {
                FadeInNewBGM(sNextMapMusic, sMapMusicFadeInSpeed);
                sCurrentMapMusic = sNextMapMusic;
                sNextMapMusic = 0;
                sMapMusicState = 2;
                sMapMusicFadeInSpeed = 0;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetMapMusic() {
    sCurrentMapMusic = 0;
    sNextMapMusic = 0;
    sMapMusicState = 0;
    sMapMusicFadeInSpeed = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapMusic() -> u16 {
    return sCurrentMapMusic;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayNewMapMusic(songNum: u16) {
    sCurrentMapMusic = songNum;
    sNextMapMusic = 0;
    sMapMusicState = 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopMapMusic() {
    sCurrentMapMusic = 0;
    sNextMapMusic = 0;
    sMapMusicState = 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutMapMusic(speed: u8) {
    if IsNotWaitingForBGMStop() != 0 {
        FadeOutBGM(speed);
    }
    sCurrentMapMusic = 0;
    sNextMapMusic = 0;
    sMapMusicState = 5;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutAndPlayNewMapMusic(songNum: u16, speed: u8) {
    FadeOutMapMusic(speed);
    sCurrentMapMusic = 0;
    sNextMapMusic = songNum;
    sMapMusicState = 6;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutAndFadeInNewMapMusic(
    songNum: u16,
    fadeOutSpeed: u8,
    fadeInSpeed: u8,
) {
    FadeOutMapMusic(fadeOutSpeed);
    sCurrentMapMusic = 0;
    sNextMapMusic = songNum;
    sMapMusicState = 7;
    sMapMusicFadeInSpeed = fadeInSpeed;
}
pub(crate) unsafe extern "C" fn FadeInNewMapMusic(songNum: u16, speed: u8) {
    FadeInNewBGM(songNum, speed);
    sCurrentMapMusic = songNum;
    sNextMapMusic = 0;
    sMapMusicState = 2;
    sMapMusicFadeInSpeed = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsNotWaitingForBGMStop() -> u8 {
    if sMapMusicState == 6 {
        return FALSE;
    }
    if sMapMusicState == 5 {
        return FALSE;
    }
    if sMapMusicState == 7 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayFanfareByFanfareNum(fanfareNum: u8) {
    let mut songNum: u16 = 0;
    m4aMPlayStop(&raw mut gMPlayInfo_BGM);
    songNum = sFanfares[fanfareNum].songNum;
    sFanfareCounter = sFanfares[fanfareNum].duration;
    m4aSongNumStart(songNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitFanfare(stop: u8) -> u8 {
    if sFanfareCounter != 0 {
        sFanfareCounter -= 1;
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopFanfareByFanfareNum(fanfareNum: u8) {
    m4aSongNumStop(sFanfares[fanfareNum].songNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayFanfare(songNum: u16) {
    let mut i: i32 = 0;
    i = 0;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFanfareTaskInactive() -> u8 {
    if FuncIsActiveTask(Some(Task_Fanfare)) == TRUE {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_Fanfare(taskId: u8) {
    if sFanfareCounter != 0 {
        sFanfareCounter -= 1;
    } else {
        m4aMPlayContinue(&raw mut gMPlayInfo_BGM);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CreateFanfareTask() {
    if FuncIsActiveTask(Some(Task_Fanfare)) != TRUE {
        CreateTask(Some(Task_Fanfare), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInNewBGM(mut songNum: u16, speed: u8) {
    if gDisableMusic != 0 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBGMTemporarily(speed: u8) {
    m4aMPlayFadeOutTemporarily(&raw mut gMPlayInfo_BGM, speed as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMPausedOrStopped() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return TRUE;
    }
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInBGM(speed: u8) {
    m4aMPlayFadeIn(&raw mut gMPlayInfo_BGM, speed as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBGM(speed: u8) {
    m4aMPlayFadeOut(&raw mut gMPlayInfo_BGM, speed as u16);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMStopped() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_Normal(species: u16, pan: i8) {
    m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
    PlayCryInternal(
        species,
        pan,
        CRY_VOLUME,
        CRY_PRIORITY_NORMAL,
        CRY_MODE_NORMAL,
    );
    gPokemonCryBGMDuckingCounter = 2;
    RestoreBGMVolumeAfterPokemonCry();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_NormalNoDucking(species: u16, pan: i8, volume: i8, priority: u8) {
    PlayCryInternal(species, pan, volume, priority, CRY_MODE_NORMAL);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_ByMode(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
        gPokemonCryBGMDuckingCounter = 2;
        RestoreBGMVolumeAfterPokemonCry();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_ReleaseDouble(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
            m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        }
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_DuckNoRestore(species: u16, pan: i8, mode: u8) {
    if mode == CRY_MODE_DOUBLES {
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    } else {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
        PlayCryInternal(species, pan, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
        gPokemonCryBGMDuckingCounter = 2;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCry_Script(species: u16, mode: u8) {
    m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 85);
    PlayCryInternal(species, 0, CRY_VOLUME, CRY_PRIORITY_NORMAL, mode);
    gPokemonCryBGMDuckingCounter = 2;
    RestoreBGMVolumeAfterPokemonCry();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayCryInternal(
    mut species: u16,
    pan: i8,
    mut volume: i8,
    priority: u8,
    mode: u8,
) {
    let mut reverse: u32 = 0;
    let mut release: u32 = 0;
    let mut length: u32 = 0;
    let mut pitch: u32 = 0;
    let mut chorus: u32 = 0;
    let mut index: u32 = 0;
    let mut table: u8 = 0;
    species -= 1;
    length = 140;
    reverse = FALSE as u32;
    release = 0;
    pitch = 15360;
    chorus = 0;
    'l1: {
        let sw1: u8 = mode;
        let mut fall = false;
        if sw1 == CRY_MODE_NORMAL {
            fall = true;
            break 'l1;
        }
        if sw1 == CRY_MODE_DOUBLES {
            fall = true;
            length = 20;
            release = 225;
            break 'l1;
        }
        if sw1 == CRY_MODE_ENCOUNTER {
            fall = true;
            release = 225;
            pitch = 15600;
            chorus = 20;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_HIGH_PITCH {
            fall = true;
            length = 50;
            release = 200;
            pitch = 15800;
            chorus = 20;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_ECHO_START {
            fall = true;
            length = 25;
            reverse = TRUE as u32;
            release = 100;
            pitch = 15600;
            chorus = 192;
            volume = 90;
            break 'l1;
        }
        if sw1 == CRY_MODE_FAINT {
            fall = true;
            release = 200;
            pitch = 14440;
            break 'l1;
        }
        if sw1 == CRY_MODE_ECHO_END {
            fall = true;
            release = 220;
            pitch = 15555;
            chorus = 192;
            volume = 70;
            break 'l1;
        }
        if sw1 == CRY_MODE_ROAR_1 {
            fall = true;
            length = 10;
            release = 100;
            pitch = 14848;
            break 'l1;
        }
        if sw1 == CRY_MODE_ROAR_2 {
            fall = true;
            length = 60;
            release = 225;
            pitch = 15616;
            break 'l1;
        }
        if sw1 == CRY_MODE_GROWL_1 {
            fall = true;
            length = 15;
            reverse = TRUE as u32;
            release = 125;
            pitch = 15200;
            break 'l1;
        }
        if sw1 == CRY_MODE_GROWL_2 {
            fall = true;
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
            fall = true;
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
    index = (species as i32 % 128) as u32;
    table = (species as i32 / 128) as u8;
    match table {
        0 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut gCryTable_Reverse[0 + index]
            } else {
                &raw mut gCryTable[0 + index]
            });
        }
        1 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut gCryTable_Reverse[128 + index]
            } else {
                &raw mut gCryTable[128 + index]
            });
        }
        2 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut gCryTable_Reverse[256 + index]
            } else {
                &raw mut gCryTable[256 + index]
            });
        }
        3 => {
            gMPlay_PokemonCry = SetPokemonCryTone(if reverse != 0 {
                &raw mut gCryTable_Reverse[384 + index]
            } else {
                &raw mut gCryTable[384 + index]
            });
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryFinished() -> u8 {
    if FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry)) == TRUE {
        return FALSE;
    } else {
        ClearPokemonCrySongs();
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopCryAndClearCrySongs() {
    m4aMPlayStop(gMPlay_PokemonCry);
    ClearPokemonCrySongs();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopCry() {
    m4aMPlayStop(gMPlay_PokemonCry);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryPlayingOrClearCrySongs() -> u8 {
    if IsPokemonCryPlaying(gMPlay_PokemonCry) != 0 {
        return TRUE;
    } else {
        ClearPokemonCrySongs();
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCryPlaying() -> u8 {
    if IsPokemonCryPlaying(gMPlay_PokemonCry) != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_DuckBGMForPokemonCry(taskId: u8) {
    if gPokemonCryBGMDuckingCounter != 0 {
        gPokemonCryBGMDuckingCounter -= 1;
        return;
    }
    if IsPokemonCryPlaying(gMPlay_PokemonCry) == 0 {
        m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 256);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn RestoreBGMVolumeAfterPokemonCry() {
    if FuncIsActiveTask(Some(Task_DuckBGMForPokemonCry)) != TRUE {
        CreateTask(Some(Task_DuckBGMForPokemonCry), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayBGM(mut songNum: u16) {
    if gDisableMusic != 0 {
        songNum = 0;
    }
    if songNum == MUS_NONE {
        songNum = 0;
    }
    m4aSongNumStart(songNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE(songNum: u16) {
    m4aSongNumStart(songNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE12WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE1);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE2);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE1WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE1);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySE2WithPanning(songNum: u16, pan: i8) {
    m4aSongNumStart(songNum);
    m4aMPlayImmInit(&raw mut gMPlayInfo_SE2);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SE12PanpotControl(pan: i8) {
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE1, TRACKS_ALL, pan);
    m4aMPlayPanpotControl(&raw mut gMPlayInfo_SE2, TRACKS_ALL, pan);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSEPlaying() -> u8 {
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBGMPlaying() -> u8 {
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return FALSE;
    }
    if gMPlayInfo_BGM.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSpecialSEPlaying() -> u8 {
    if gMPlayInfo_SE3.status & MUSICPLAYER_STATUS_PAUSE != 0 {
        return FALSE;
    }
    if gMPlayInfo_SE3.status & MUSICPLAYER_STATUS_TRACK == 0 {
        return FALSE;
    }
    return TRUE;
}
