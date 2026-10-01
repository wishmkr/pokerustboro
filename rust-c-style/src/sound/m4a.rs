//! Translated from `src/m4a.c` by tools/rustport/c2rs.py.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSoundInfo: SoundInfo = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCrySongs: CArray<PokemonCrySong, 2> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCryMusicPlayers: CArray<MusicPlayerInfo, 2> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_BGM: MusicPlayerInfo = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayJumpTable: CArray<Option<unsafe extern "C" fn()>, 36> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gCgbChans: CArray<CgbChannel, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE1: MusicPlayerInfo = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE2: MusicPlayerInfo = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCryTracks: CArray<MusicPlayerTrack, 4> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPokemonCrySong: PokemonCrySong = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayMemAccArea: Aligned<CArray<u8, 16>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMPlayInfo_SE3: MusicPlayerInfo = unsafe { zeroed() };

unsafe extern "C" {
    static gCgb3Vol: CArray<u8, 0>;
    static gCgbFreqTable: CArray<i16, 0>;
    static gCgbScaleTable: CArray<u8, 0>;
    static gFreqTable: CArray<u32, 0>;
    static gMPlayTable: CArray<MusicPlayer, 0>;
    static mut gMaxLines: CArray<u8, 0>;
    static gNoiseTable: CArray<u8, 0>;
    static mut gNumMusicPlayers: CArray<u8, 0>;
    static gPcmSamplesPerVBlankTable: CArray<u16, 0>;
    static gPokemonCrySongTemplate: PokemonCrySong;
    static gScaleTable: CArray<u8, 0>;
    static gSongTable: CArray<Song, 0>;
    static gXcmdTable:
        CArray<Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>, 0>;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn MPlayJumpTableCopy(a0: *mut Option<unsafe extern "C" fn()>);
    fn MPlayMain(a0: *mut MusicPlayerInfo);
    fn SoundMain();
    fn TrackStop(a0: *mut MusicPlayerInfo, a1: *mut MusicPlayerTrack);
    fn ply_endtie(a0: *mut MusicPlayerInfo, a1: *mut MusicPlayerTrack);
    fn ply_lfos(a0: *mut MusicPlayerInfo, a1: *mut MusicPlayerTrack);
    fn ply_mod(a0: *mut MusicPlayerInfo, a1: *mut MusicPlayerTrack);
    fn ply_note(a0: u32, a1: *mut MusicPlayerInfo, a2: *mut MusicPlayerTrack);
    fn umul3232H32(a0: u32, a1: u32) -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MidiKeyToFreq(wav: *mut WaveData, mut key: u8, fineAdjust: u8) -> u32 {
    let mut val1: u32 = 0;
    let mut val2: u32 = 0;
    let mut fineAdjustShifted: u32 = (fineAdjust as u32) << 24;
    if key > 178 {
        key = 178;
        fineAdjustShifted = 0xff000000;
    }
    val1 = gScaleTable[key] as u32;
    val1 = shr_u32(gFreqTable[val1 & 0xF], val1 >> 4);
    val2 = gScaleTable[key as i32 + 1] as u32;
    val2 = shr_u32(gFreqTable[val2 & 0xF], val2 >> 4);
    return umul3232H32(
        (*wav).freq,
        val1 + umul3232H32(val2 - val1, fineAdjustShifted),
    );
}
pub(crate) unsafe extern "C" fn UnusedDummyFunc() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayContinue(mplayInfo: *mut MusicPlayerInfo) {
    if (*mplayInfo).ident == ID_NUMBER {
        (*mplayInfo).ident += 1;
        (*mplayInfo).status &= 0x7fffffff;
        (*mplayInfo).ident = ID_NUMBER;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayFadeOut(mplayInfo: *mut MusicPlayerInfo, speed: u16) {
    if (*mplayInfo).ident == ID_NUMBER {
        (*mplayInfo).ident += 1;
        (*mplayInfo).fadeOC = speed;
        (*mplayInfo).fadeOI = speed;
        (*mplayInfo).fadeOV = 256;
        (*mplayInfo).ident = ID_NUMBER;
    }
}
// hand-written: tools/rustport/overrides/m4a/m4aSoundInit.rs
// c2rs-uses: SoundInit MPlayExtender m4aSoundMode MPlayOpen gSoundInfo gCgbChans gNumMusicPlayers gMPlayTable gMPlayMemAccArea gPokemonCrySong gPokemonCrySongTemplate gPokemonCryMusicPlayers gPokemonCryTracks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundInit() {
    let mut i: i32 = 0;
    // The mixer (m4a_mixer.rs) is linked into IWRAM and copied there by
    // Init: no SoundMainRAM_Buffer to copy it into.
    SoundInit(&raw mut gSoundInfo);
    MPlayExtender(gCgbChans.as_mut_ptr());
    m4aSoundMode(0x94c500);
    i = 0;
    while i < gNumMusicPlayers.as_mut_ptr() as usize as u16 as i32 {
        let mut mplayInfo: *mut MusicPlayerInfo = gMPlayTable[i].info;
        MPlayOpen(mplayInfo, gMPlayTable[i].track, gMPlayTable[i].numTracks);
        (*mplayInfo).unk_B = gMPlayTable[i].unk_A as u8;
        (*mplayInfo).memAccArea = gMPlayMemAccArea.as_mut_ptr();
        i += 1;
    }
    memcpy(
        &raw mut gPokemonCrySong as *mut u8,
        (&raw const gPokemonCrySongTemplate).cast_mut() as *mut u8,
        52,
    );
    i = 0;
    while i < 2 {
        let mut mplayInfo: *mut MusicPlayerInfo = &raw mut gPokemonCryMusicPlayers[i];
        let mut track: *mut MusicPlayerTrack = &raw mut gPokemonCryTracks[i * 2];
        MPlayOpen(mplayInfo, track, 2);
        (*track).chan = null_mut();
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundMain() {
    SoundMain();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStart(n: u16) {
    let mut mplayTable: *mut MusicPlayer = gMPlayTable.as_ptr().cast_mut();
    let mut songTable: *mut Song = gSongTable.as_ptr().cast_mut();
    let mut song: *mut Song = songTable.at(n);
    let mut mplay: *mut MusicPlayer = mplayTable.at((*song).ms);
    MPlayStart((*mplay).info, (*song).header);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStartOrChange(n: u16) {
    let mut mplayTable: *mut MusicPlayer = gMPlayTable.as_ptr().cast_mut();
    let mut songTable: *mut Song = gSongTable.as_ptr().cast_mut();
    let mut song: *mut Song = songTable.at(n);
    let mut mplay: *mut MusicPlayer = mplayTable.at((*song).ms);
    if (*(*mplay).info).songHeader != (*song).header {
        MPlayStart((*mplay).info, (*song).header);
    } else {
        if (*(*mplay).info).status & MUSICPLAYER_STATUS_TRACK == 0
            || (*(*mplay).info).status & MUSICPLAYER_STATUS_PAUSE != 0
        {
            MPlayStart((*mplay).info, (*song).header);
        }
    }
}
pub(crate) unsafe extern "C" fn m4aSongNumStartOrContinue(n: u16) {
    let mut mplayTable: *mut MusicPlayer = gMPlayTable.as_ptr().cast_mut();
    let mut songTable: *mut Song = gSongTable.as_ptr().cast_mut();
    let mut song: *mut Song = songTable.at(n);
    let mut mplay: *mut MusicPlayer = mplayTable.at((*song).ms);
    if (*(*mplay).info).songHeader != (*song).header {
        MPlayStart((*mplay).info, (*song).header);
    } else if (*(*mplay).info).status & MUSICPLAYER_STATUS_TRACK == 0 {
        MPlayStart((*mplay).info, (*song).header);
    } else if (*(*mplay).info).status & MUSICPLAYER_STATUS_PAUSE != 0 {
        MPlayContinue((*mplay).info);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSongNumStop(n: u16) {
    let mut mplayTable: *mut MusicPlayer = gMPlayTable.as_ptr().cast_mut();
    let mut songTable: *mut Song = gSongTable.as_ptr().cast_mut();
    let mut song: *mut Song = songTable.at(n);
    let mut mplay: *mut MusicPlayer = mplayTable.at((*song).ms);
    if (*(*mplay).info).songHeader == (*song).header {
        m4aMPlayStop((*mplay).info);
    }
}
pub(crate) unsafe extern "C" fn m4aSongNumContinue(n: u16) {
    let mut mplayTable: *mut MusicPlayer = gMPlayTable.as_ptr().cast_mut();
    let mut songTable: *mut Song = gSongTable.as_ptr().cast_mut();
    let mut song: *mut Song = songTable.at(n);
    let mut mplay: *mut MusicPlayer = mplayTable.at((*song).ms);
    if (*(*mplay).info).songHeader == (*song).header {
        MPlayContinue((*mplay).info);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayAllStop() {
    let mut i: i32 = 0;
    i = 0;
    while i < gNumMusicPlayers.as_mut_ptr() as usize as u16 as i32 {
        m4aMPlayStop(gMPlayTable[i].info);
        i += 1;
    }
    i = 0;
    while i < MAX_POKEMON_CRIES {
        m4aMPlayStop(&raw mut gPokemonCryMusicPlayers[i]);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayContinue(mplayInfo: *mut MusicPlayerInfo) {
    MPlayContinue(mplayInfo);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayAllContinue() {
    let mut i: i32 = 0;
    i = 0;
    while i < gNumMusicPlayers.as_mut_ptr() as usize as u16 as i32 {
        MPlayContinue(gMPlayTable[i].info);
        i += 1;
    }
    i = 0;
    while i < MAX_POKEMON_CRIES {
        MPlayContinue(&raw mut gPokemonCryMusicPlayers[i]);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeOut(mplayInfo: *mut MusicPlayerInfo, speed: u16) {
    MPlayFadeOut(mplayInfo, speed);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeOutTemporarily(mplayInfo: *mut MusicPlayerInfo, speed: u16) {
    if (*mplayInfo).ident == ID_NUMBER {
        (*mplayInfo).ident += 1;
        (*mplayInfo).fadeOC = speed;
        (*mplayInfo).fadeOI = speed;
        (*mplayInfo).fadeOV = 257;
        (*mplayInfo).ident = ID_NUMBER;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayFadeIn(mplayInfo: *mut MusicPlayerInfo, speed: u16) {
    if (*mplayInfo).ident == ID_NUMBER {
        (*mplayInfo).ident += 1;
        (*mplayInfo).fadeOC = speed;
        (*mplayInfo).fadeOI = speed;
        (*mplayInfo).fadeOV = 2;
        (*mplayInfo).status &= 0x7fffffff;
        (*mplayInfo).ident = ID_NUMBER;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayImmInit(mplayInfo: *mut MusicPlayerInfo) {
    let mut trackCount: i32 = (*mplayInfo).trackCount as i32;
    let mut track: *mut MusicPlayerTrack = (*mplayInfo).tracks;
    while trackCount > 0 {
        if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
            if (*track).flags as i32 & MPT_FLG_START != 0 {
                Clear64byte(track as *mut c_void);
                (*track).flags = MPT_FLG_EXIST as u8;
                (*track).bendRange = 2;
                (*track).volX = 64;
                (*track).lfoSpeed = 22;
                (*track).tone.r#type = 1;
            }
        }
        trackCount -= 1;
        track = track.at(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayExtender(mut cgbChans: *mut CgbChannel) {
    let mut soundInfo: *mut SoundInfo = null_mut();
    let mut ident: u32 = 0;
    volatile_write(67108996 as usize as *mut u16, 143);
    volatile_write(67108992 as usize as *mut u16, 0);
    volatile_write(67108963 as usize as *mut u8, 0x8);
    volatile_write(67108969 as usize as *mut u8, 0x8);
    volatile_write(67108985 as usize as *mut u8, 0x8);
    volatile_write(67108965 as usize as *mut u8, 0x80);
    volatile_write(67108973 as usize as *mut u8, 0x80);
    volatile_write(67108989 as usize as *mut u8, 0x80);
    volatile_write(67108976 as usize as *mut u8, 0);
    volatile_write(67108992 as usize as *mut u8, 0x77);
    soundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    ident = (*soundInfo).ident;
    if ident != ID_NUMBER {
        return;
    }
    (*soundInfo).ident += 1;
    gMPlayJumpTable[8] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(ply_memacc));
    gMPlayJumpTable[17] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(ply_lfos));
    gMPlayJumpTable[19] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(ply_mod));
    gMPlayJumpTable[28] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(ply_xcmd));
    gMPlayJumpTable[29] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(ply_endtie));
    gMPlayJumpTable[30] = core::mem::transmute::<
        Option<unsafe extern "C" fn(u32)>,
        Option<unsafe extern "C" fn()>,
    >(Some(SampleFreqSet));
    gMPlayJumpTable[31] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(TrackStop));
    gMPlayJumpTable[32] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo)>,
        Option<unsafe extern "C" fn()>,
    >(Some(FadeOutBody));
    gMPlayJumpTable[33] = core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>,
        Option<unsafe extern "C" fn()>,
    >(Some(TrkVolPitSet));
    (*soundInfo).cgbChans = cgbChans;
    (*soundInfo).CgbSound = Some(CgbSound);
    (*soundInfo).CgbOscOff = Some(CgbOscOff);
    (*soundInfo).MidiKeyToCgbFreq = Some(MidiKeyToCgbFreq);
    (*soundInfo).maxLines = gMaxLines.as_mut_ptr() as usize as u32 as u8;
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                cgbChans as *mut c_void,
                0x5000040,
            );
        }
    }
    (*cgbChans).r#type = 1;
    (*cgbChans).panMask = 0x11;
    (*cgbChans.at(1)).r#type = 2;
    (*cgbChans.at(1)).panMask = 0x22;
    (*cgbChans.at(2)).r#type = 3;
    (*cgbChans.at(2)).panMask = 0x44;
    (*cgbChans.at(3)).r#type = 4;
    (*cgbChans.at(3)).panMask = 0x88;
    (*soundInfo).ident = ident;
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
pub unsafe extern "C" fn ClearChain(x: *mut c_void) {
    let mut func: Option<unsafe extern "C" fn()> = *(&raw mut gMPlayJumpTable[34]);
    core::mem::transmute::<_, unsafe extern "C" fn(*mut c_void)>(func.unwrap_unchecked())(x);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Clear64byte(x: *mut c_void) {
    let mut func: Option<unsafe extern "C" fn()> = *(&raw mut gMPlayJumpTable[35]);
    core::mem::transmute::<_, unsafe extern "C" fn(*mut c_void)>(func.unwrap_unchecked())(x);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundInit(soundInfo: *mut SoundInfo) {
    (*soundInfo).ident = 0;
    if (67109060 as usize as *mut u32).read_volatile() & 0x2000000 != 0 {
        volatile_write(67109060 as usize as *mut u32, 0x84400004);
    }
    if (67109072 as usize as *mut u32).read_volatile() & 0x2000000 != 0 {
        volatile_write(67109072 as usize as *mut u32, 0x84400004);
    }
    volatile_write(67109062 as usize as *mut u16, DMA_32BIT);
    volatile_write(67109074 as usize as *mut u16, DMA_32BIT);
    volatile_write(67108996 as usize as *mut u16, 143);
    volatile_write(67108994 as usize as *mut u16, 43278);
    volatile_write(
        67109001 as usize as *mut u8,
        (67109001 as usize as *mut u8).read_volatile() & 0x3F | 0x40,
    );
    volatile_write(
        67109052 as usize as *mut u32,
        (*soundInfo).pcmBuffer.as_mut_ptr() as usize as i32 as u32,
    );
    volatile_write(
        67109056 as usize as *mut u32,
        67109024 as usize as *mut u32 as usize as i32 as u32,
    );
    volatile_write(
        67109064 as usize as *mut u32,
        (*soundInfo).pcmBuffer.as_mut_ptr() as usize as i32 as u32 + PCM_DMA_BUF_SIZE,
    );
    volatile_write(
        67109068 as usize as *mut u32,
        67109028 as usize as *mut u32 as usize as i32 as u32,
    );
    *(0x3007FF0 as usize as *mut *mut SoundInfo) = soundInfo;
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                soundInfo as *mut c_void,
                0x50003ec,
            );
        }
    }
    (*soundInfo).maxChans = 8;
    (*soundInfo).masterVolume = 15;
    (*soundInfo).plynote = Some(ply_note);
    (*soundInfo).CgbSound = Some(DummyFunc);
    (*soundInfo).CgbOscOff = core::mem::transmute::<
        Option<unsafe extern "C" fn()>,
        Option<unsafe extern "C" fn(u8)>,
    >(Some(DummyFunc));
    (*soundInfo).MidiKeyToCgbFreq = core::mem::transmute::<
        Option<unsafe extern "C" fn()>,
        Option<unsafe extern "C" fn(u8, u8, u8) -> u32>,
    >(Some(DummyFunc));
    (*soundInfo).ExtVolPit = Some(DummyFunc);
    MPlayJumpTableCopy(gMPlayJumpTable.as_mut_ptr());
    (*soundInfo).MPlayJumpTable = gMPlayJumpTable.as_mut_ptr();
    SampleFreqSet(SOUND_MODE_FREQ_13379);
    (*soundInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SampleFreqSet(mut freq: u32) {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    freq = (freq & 0xF0000) >> 16;
    (*soundInfo).freq = freq as u8;
    (*soundInfo).pcmSamplesPerVBlank = gPcmSamplesPerVBlankTable[freq - 1] as i32;
    (*soundInfo).pcmDmaPeriod =
        div_i32(PCM_DMA_BUF_SIZE as i32, (*soundInfo).pcmSamplesPerVBlank) as u8;
    (*soundInfo).pcmFreq = (597275 * (*soundInfo).pcmSamplesPerVBlank + 5000) / 10000;
    (*soundInfo).divFreq = div_i32(0x1000000, (*soundInfo).pcmFreq) + 1 >> 1;
    volatile_write(67109122 as usize as *mut u16, 0);
    volatile_write(
        0x4000100 as usize as *mut u16,
        (div_i32(280896, (*soundInfo).pcmSamplesPerVBlank) as u16).wrapping_neg(),
    );
    m4aSoundVSyncOn();
    while (REG_ADDR_VCOUNT as usize as *mut u8).read_volatile() == 159 {}
    while (REG_ADDR_VCOUNT as usize as *mut u8).read_volatile() != 159 {}
    volatile_write(67109122 as usize as *mut u16, TIMER_ENABLE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundMode(mode: u32) {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    let mut temp: u32 = 0;
    if (*soundInfo).ident != ID_NUMBER {
        return;
    }
    (*soundInfo).ident += 1;
    temp = mode & 255;
    if temp != 0 {
        (*soundInfo).reverb = temp as u8 & SOUND_MODE_REVERB_VAL;
    }
    temp = mode & SOUND_MODE_MAXCHN;
    if temp != 0 {
        let mut chan: *mut SoundChannel = null_mut();
        (*soundInfo).maxChans = (temp >> 8) as u8;
        temp = MAX_DIRECTSOUND_CHANNELS;
        chan = &raw mut (*soundInfo).chans[0];
        while temp != 0 {
            (*chan).statusFlags = 0;
            temp -= 1;
            chan = chan.at(1);
        }
    }
    temp = mode & SOUND_MODE_MASVOL;
    if temp != 0 {
        (*soundInfo).masterVolume = (temp >> 12) as u8;
    }
    temp = mode & SOUND_MODE_DA_BIT;
    if temp != 0 {
        temp = (temp & 0x300000) >> 14;
        volatile_write(
            67109001 as usize as *mut u8,
            (67109001 as usize as *mut u8).read_volatile() & 0x3F | temp as u8,
        );
    }
    temp = mode & SOUND_MODE_FREQ;
    if temp != 0 {
        m4aSoundVSyncOff();
        SampleFreqSet(temp);
    }
    (*soundInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundClear() {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    let mut i: i32 = 0;
    let mut chan: *mut c_void = null_mut();
    if (*soundInfo).ident != ID_NUMBER {
        return;
    }
    (*soundInfo).ident += 1;
    i = MAX_DIRECTSOUND_CHANNELS as i32;
    chan = &raw mut (*soundInfo).chans[0] as *mut c_void;
    while i > 0 {
        (*(chan as *mut SoundChannel)).statusFlags = 0;
        i -= 1;
        chan = (chan as usize as i32 as u32 + 64) as usize as *mut c_void;
    }
    chan = (*soundInfo).cgbChans as *mut c_void;
    if !chan.is_null() {
        i = 1;
        while i <= 4 {
            (*soundInfo).CgbOscOff.unwrap_unchecked()(i as u8);
            (*(chan as *mut CgbChannel)).statusFlags = 0;
            i += 1;
            chan = (chan as usize as i32 as u32 + 64) as usize as *mut c_void;
        }
    }
    (*soundInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundVSyncOff() {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    if (*soundInfo).ident >= ID_NUMBER && (*soundInfo).ident <= 0x68736d54 {
        (*soundInfo).ident += 10;
        if (67109060 as usize as *mut u32).read_volatile() & 0x2000000 != 0 {
            volatile_write(67109060 as usize as *mut u32, 0x84400004);
        }
        if (67109072 as usize as *mut u32).read_volatile() & 0x2000000 != 0 {
            volatile_write(67109072 as usize as *mut u32, 0x84400004);
        }
        volatile_write(67109062 as usize as *mut u16, DMA_32BIT);
        volatile_write(67109074 as usize as *mut u16, DMA_32BIT);
        {
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    (*soundInfo).pcmBuffer.as_mut_ptr() as *mut c_void,
                    0x5000318,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundVSyncOn() {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    let mut ident: u32 = (*soundInfo).ident;
    if ident == ID_NUMBER {
        return;
    }
    volatile_write(67109062 as usize as *mut u16, 46592);
    volatile_write(67109074 as usize as *mut u16, 46592);
    volatile_write(&raw mut (*soundInfo).pcmDmaCounter, 0);
    (*soundInfo).ident = ident - 10;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayOpen(
    mplayInfo: *mut MusicPlayerInfo,
    mut tracks: *mut MusicPlayerTrack,
    mut trackCount: u8,
) {
    let mut soundInfo: *mut SoundInfo = null_mut();
    if trackCount == 0 {
        return;
    }
    if trackCount > MAX_MUSICPLAYER_TRACKS {
        trackCount = MAX_MUSICPLAYER_TRACKS;
    }
    soundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    if (*soundInfo).ident != ID_NUMBER {
        return;
    }
    (*soundInfo).ident += 1;
    Clear64byte(mplayInfo as *mut c_void);
    (*mplayInfo).tracks = tracks;
    (*mplayInfo).trackCount = trackCount;
    (*mplayInfo).status = MUSICPLAYER_STATUS_PAUSE;
    while trackCount != 0 {
        (*tracks).flags = 0;
        trackCount -= 1;
        tracks = tracks.at(1);
    }
    if (*soundInfo).MPlayMainHead.is_some() {
        (*mplayInfo).MPlayMainNext = (*soundInfo).MPlayMainHead;
        (*mplayInfo).musicPlayerNext = (*soundInfo).musicPlayerHead;
        (*soundInfo).MPlayMainHead = None;
    }
    (*soundInfo).musicPlayerHead = mplayInfo;
    (*soundInfo).MPlayMainHead = Some(MPlayMain);
    (*soundInfo).ident = ID_NUMBER;
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayStart(mplayInfo: *mut MusicPlayerInfo, songHeader: *mut SongHeader) {
    let mut i: i32 = 0;
    let mut unk_B: u8 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    unk_B = (*mplayInfo).unk_B;
    if unk_B == 0
        || ((*mplayInfo).songHeader.is_null()
            || (*(*mplayInfo).tracks).flags as i32 & MPT_FLG_START == 0)
            && ((*mplayInfo).status & MUSICPLAYER_STATUS_TRACK == 0
                || (*mplayInfo).status & MUSICPLAYER_STATUS_PAUSE != 0)
        || (*mplayInfo).priority <= (*songHeader).priority
    {
        (*mplayInfo).ident += 1;
        (*mplayInfo).status = 0;
        (*mplayInfo).songHeader = songHeader;
        (*mplayInfo).tone = (*songHeader).tone;
        (*mplayInfo).priority = (*songHeader).priority;
        (*mplayInfo).clock = 0;
        (*mplayInfo).tempoD = 150;
        (*mplayInfo).tempoI = 150;
        (*mplayInfo).tempoU = 0x100;
        (*mplayInfo).tempoC = 0;
        (*mplayInfo).fadeOI = 0;
        i = 0;
        track = (*mplayInfo).tracks;
        while i < (*songHeader).trackCount as i32 && i < (*mplayInfo).trackCount as i32 {
            TrackStop(mplayInfo, track);
            (*track).flags = 192;
            (*track).chan = null_mut();
            (*track).cmdPtr = (*songHeader).part[i];
            i += 1;
            track = track.at(1);
        }
        while i < (*mplayInfo).trackCount as i32 {
            TrackStop(mplayInfo, track);
            (*track).flags = 0;
            i += 1;
            track = track.at(1);
        }
        if (*songHeader).reverb as i32 & SOUND_MODE_REVERB_SET != 0 {
            m4aSoundMode((*songHeader).reverb as u32);
        }
        (*mplayInfo).ident = ID_NUMBER;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayStop(mplayInfo: *mut MusicPlayerInfo) {
    let mut i: i32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    (*mplayInfo).status |= MUSICPLAYER_STATUS_PAUSE;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    while i > 0 {
        TrackStop(mplayInfo, track);
        i -= 1;
        track = track.at(1);
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutBody(mplayInfo: *mut MusicPlayerInfo) {
    let mut i: i32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    let mut fadeOV: u16 = 0;
    if (*mplayInfo).fadeOI == 0 {
        return;
    }
    if ({
        (*mplayInfo).fadeOC -= 1;
        (*mplayInfo).fadeOC
    }) != 0
    {
        return;
    }
    (*mplayInfo).fadeOC = (*mplayInfo).fadeOI;
    if (*mplayInfo).fadeOV as i32 & FADE_IN != 0 {
        if ({
            (*mplayInfo).fadeOV += 16;
            (*mplayInfo).fadeOV
        }) >= 256
        {
            (*mplayInfo).fadeOV = 256;
            (*mplayInfo).fadeOI = 0;
        }
    } else {
        if ({
            (*mplayInfo).fadeOV -= 16;
            (*mplayInfo).fadeOV
        }) as i16
            <= 0
        {
            i = (*mplayInfo).trackCount as i32;
            track = (*mplayInfo).tracks;
            while i > 0 {
                let mut val: u32 = 0;
                TrackStop(mplayInfo, track);
                val = TEMPORARY_FADE;
                fadeOV = (*mplayInfo).fadeOV;
                val &= fadeOV as u32;
                if val == 0 {
                    (*track).flags = 0;
                }
                i -= 1;
                track = track.at(1);
            }
            if (*mplayInfo).fadeOV as i32 & TEMPORARY_FADE as i32 != 0 {
                (*mplayInfo).status |= MUSICPLAYER_STATUS_PAUSE;
            } else {
                (*mplayInfo).status = MUSICPLAYER_STATUS_PAUSE;
            }
            (*mplayInfo).fadeOI = 0;
            return;
        }
    }
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    while i > 0 {
        if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
            fadeOV = (*mplayInfo).fadeOV;
            (*track).volX = (fadeOV >> 2) as u8;
            (*track).flags |= MPT_FLG_VOLCHG;
        }
        i -= 1;
        track = track.at(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrkVolPitSet(
    mplayInfo: *mut MusicPlayerInfo,
    track: *mut MusicPlayerTrack,
) {
    if (*track).flags as i32 & MPT_FLG_VOLSET != 0 {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        x = ((*track).vol as u32 * (*track).volX as u32 >> 5) as i32;
        if (*track).modT == 1 {
            x = (x as u32 * ((*track).modM as u32 + 128) >> 7) as i32;
        }
        y = 2 * (*track).pan as i32 + (*track).panX as i32;
        if (*track).modT == 2 {
            y += (*track).modM as i32;
        }
        if y < -128 {
            y = -128;
        } else if y > 127 {
            y = 127;
        }
        (*track).volMR = ((y as u32 + 128) * x as u32 >> 8) as u8;
        (*track).volML = ((127 - y as u32) * x as u32 >> 8) as u8;
    }
    if (*track).flags as i32 & MPT_FLG_PITSET != 0 {
        let mut bend: i32 = (*track).bend as i32 * (*track).bendRange as i32;
        let mut x: i32 = ((*track).tune as i32 + bend) * 4
            + (((*track).keyShift as i32) << 8)
            + (((*track).keyShiftX as i32) << 8)
            + (*track).pitX as i32;
        if (*track).modT == 0 {
            x += 16 * (*track).modM as i32;
        }
        (*track).keyM = (x >> 8) as u8;
        (*track).pitM = x as u8;
    }
    (*track).flags &= 250;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MidiKeyToCgbFreq(chanNum: u8, mut key: u8, mut fineAdjust: u8) -> u32 {
    if chanNum == 4 {
        if key <= 20 {
            key = 0;
        } else {
            key -= 21;
            if key > 59 {
                key = 59;
            }
        }
        return gNoiseTable[key] as u32;
    } else {
        let mut val1: i32 = 0;
        let mut val2: i32 = 0;
        if key <= 35 {
            fineAdjust = 0;
            key = 0;
        } else {
            key -= 36;
            if key > 130 {
                key = 130;
                fineAdjust = 255;
            }
        }
        val1 = gCgbScaleTable[key] as i32;
        val1 = shr_i32(gCgbFreqTable[val1 & 0xF] as i32, (val1 >> 4) as u32);
        val2 = gCgbScaleTable[key as i32 + 1] as i32;
        val2 = shr_i32(gCgbFreqTable[val2 & 0xF] as i32, (val2 >> 4) as u32);
        return val1 as u32 + (fineAdjust as i32 * (val2 - val1) >> 8) as u32 + 2048;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CgbOscOff(chanNum: u8) {
    match chanNum {
        1 => {
            volatile_write(67108963 as usize as *mut u8, 8);
            volatile_write(67108965 as usize as *mut u8, 0x80);
        }
        2 => {
            volatile_write(67108969 as usize as *mut u8, 8);
            volatile_write(67108973 as usize as *mut u8, 0x80);
        }
        3 => {
            volatile_write(67108976 as usize as *mut u8, 0);
        }
        _ => {
            volatile_write(67108985 as usize as *mut u8, 8);
            volatile_write(67108989 as usize as *mut u8, 0x80);
        }
    }
}
pub(crate) unsafe extern "C" fn CgbPan(chan: *mut CgbChannel) -> i32 {
    let mut rightVolume: u32 = (*chan).rightVolume as u32;
    let mut leftVolume: u32 = (*chan).leftVolume as u32;
    if ({
        rightVolume = rightVolume as u8 as u32;
        rightVolume
    }) >= ({
        leftVolume = leftVolume as u8 as u32;
        leftVolume
    }) {
        if rightVolume / 2 >= leftVolume {
            (*chan).pan = 0x0F;
            return 1;
        }
    } else {
        if leftVolume / 2 >= rightVolume {
            (*chan).pan = 0xF0;
            return 1;
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CgbModVol(chan: *mut CgbChannel) {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    if (*soundInfo).mode as i32 & 1 != 0 || CgbPan(chan) == 0 {
        (*chan).pan = 0xFF;
        (*chan).envelopeGoal = (*chan).leftVolume as u32 as u8 + (*chan).rightVolume as u32 as u8;
        (*chan).envelopeGoal = ((*chan).envelopeGoal as i32 / 16) as u8;
    } else {
        (*chan).envelopeGoal = (*chan).leftVolume as u32 as u8 + (*chan).rightVolume as u32 as u8;
        (*chan).envelopeGoal = ((*chan).envelopeGoal as i32 / 16) as u8;
        if (*chan).envelopeGoal > 15 {
            (*chan).envelopeGoal = 15;
        }
    }
    (*chan).sustainGoal = ((*chan).envelopeGoal as i32 * (*chan).sustain as i32 + 15 >> 4) as u8;
    (*chan).pan &= (*chan).panMask;
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
                                CgbModVol(c.cast());
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
                            CgbModVol(c.cast());
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
pub unsafe extern "C" fn m4aMPlayTempoControl(mplayInfo: *mut MusicPlayerInfo, tempo: u16) {
    if (*mplayInfo).ident == ID_NUMBER {
        (*mplayInfo).ident += 1;
        (*mplayInfo).tempoU = tempo;
        (*mplayInfo).tempoI = ((*mplayInfo).tempoD as i32 * (*mplayInfo).tempoU as i32 >> 8) as u16;
        (*mplayInfo).ident = ID_NUMBER;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayVolumeControl(
    mplayInfo: *mut MusicPlayerInfo,
    trackBits: u16,
    volume: u16,
) {
    let mut i: i32 = 0;
    let mut bit: u32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    bit = 1;
    while i > 0 {
        if trackBits as u32 & bit != 0 {
            if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
                (*track).volX = (volume as i32 / 4) as u8;
                (*track).flags |= MPT_FLG_VOLCHG;
            }
        }
        i -= 1;
        track = track.at(1);
        bit <<= 1;
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayPitchControl(
    mplayInfo: *mut MusicPlayerInfo,
    trackBits: u16,
    pitch: i16,
) {
    let mut i: i32 = 0;
    let mut bit: u32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    bit = 1;
    while i > 0 {
        if trackBits as u32 & bit != 0 {
            if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
                (*track).keyShiftX = (pitch >> 8) as i8;
                (*track).pitX = pitch as u8;
                (*track).flags |= MPT_FLG_PITCHG;
            }
        }
        i -= 1;
        track = track.at(1);
        bit <<= 1;
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayPanpotControl(
    mplayInfo: *mut MusicPlayerInfo,
    trackBits: u16,
    pan: i8,
) {
    let mut i: i32 = 0;
    let mut bit: u32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    bit = 1;
    while i > 0 {
        if trackBits as u32 & bit != 0 {
            if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
                (*track).panX = pan;
                (*track).flags |= MPT_FLG_VOLCHG;
            }
        }
        i -= 1;
        track = track.at(1);
        bit <<= 1;
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearModM(track: *mut MusicPlayerTrack) {
    (*track).lfoSpeedC = 0;
    (*track).modM = 0;
    if (*track).modT == 0 {
        (*track).flags |= MPT_FLG_PITCHG;
    } else {
        (*track).flags |= MPT_FLG_VOLCHG;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayModDepthSet(
    mplayInfo: *mut MusicPlayerInfo,
    trackBits: u16,
    modDepth: u8,
) {
    let mut i: i32 = 0;
    let mut bit: u32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    bit = 1;
    while i > 0 {
        if trackBits as u32 & bit != 0 {
            if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
                (*track).r#mod = modDepth;
                if (*track).r#mod == 0 {
                    ClearModM(track);
                }
            }
        }
        i -= 1;
        track = track.at(1);
        bit <<= 1;
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aMPlayLFOSpeedSet(
    mplayInfo: *mut MusicPlayerInfo,
    trackBits: u16,
    lfoSpeed: u8,
) {
    let mut i: i32 = 0;
    let mut bit: u32 = 0;
    let mut track: *mut MusicPlayerTrack = null_mut();
    if (*mplayInfo).ident != ID_NUMBER {
        return;
    }
    (*mplayInfo).ident += 1;
    i = (*mplayInfo).trackCount as i32;
    track = (*mplayInfo).tracks;
    bit = 1;
    while i > 0 {
        if trackBits as u32 & bit != 0 {
            if (*track).flags as i32 & MPT_FLG_EXIST != 0 {
                (*track).lfoSpeed = lfoSpeed;
                if (*track).lfoSpeed == 0 {
                    ClearModM(track);
                }
            }
        }
        i -= 1;
        track = track.at(1);
        bit <<= 1;
    }
    (*mplayInfo).ident = ID_NUMBER;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_memacc(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    let mut op: u32 = 0;
    let mut addr: *mut u8 = null_mut();
    let mut data: u8 = 0;
    op = *(*track).cmdPtr as u32;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
    addr = (*mplayInfo).memAccArea.at(*(*track).cmdPtr);
    (*track).cmdPtr = (*track).cmdPtr.at(1);
    data = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
    'cond_false: {
        'cond_true: {
            match op {
                0 => {
                    *addr = data;
                    return;
                }
                1 => {
                    *addr += data;
                    return;
                }
                2 => {
                    *addr -= data;
                    return;
                }
                3 => {
                    *addr = *(*mplayInfo).memAccArea.at(data);
                    return;
                }
                4 => {
                    *addr += *(*mplayInfo).memAccArea.at(data);
                    return;
                }
                5 => {
                    *addr -= *(*mplayInfo).memAccArea.at(data);
                    return;
                }
                6 => {
                    if *addr == data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                7 => {
                    if *addr != data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                8 => {
                    if *addr > data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                9 => {
                    if *addr >= data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                10 => {
                    if *addr <= data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                11 => {
                    if *addr < data {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                12 => {
                    if *addr == *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                13 => {
                    if *addr != *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                14 => {
                    if *addr > *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                15 => {
                    if *addr >= *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                16 => {
                    if *addr <= *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                17 => {
                    if *addr < *(*mplayInfo).memAccArea.at(data) {
                        break 'cond_true;
                    } else {
                        break 'cond_false;
                    }
                    return;
                }
                _ => {
                    return;
                }
            }
        }
        core::mem::transmute::<_, unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>(
            (*(&raw mut gMPlayJumpTable[1])).unwrap_unchecked(),
        )(mplayInfo, track);
        return;
    }
    (*track).cmdPtr = (*track).cmdPtr.at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xcmd(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    let mut n: u32 = *(*track).cmdPtr as u32;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
    gXcmdTable[n].unwrap_unchecked()(mplayInfo, track);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xxx(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    core::mem::transmute::<_, unsafe extern "C" fn(*mut MusicPlayerInfo, *mut MusicPlayerTrack)>(
        gMPlayJumpTable[0].unwrap_unchecked(),
    )(mplayInfo, track);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xwave(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    let mut wav: u32 = 0;
    wav = 0;
    {
        let mut byte: u32 = *(*track).cmdPtr as u32;
        byte <<= 0;
        wav &= 0xffffff00;
        wav |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(1) as u32;
        byte <<= 8;
        wav &= 0xffff00ff;
        wav |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(2) as u32;
        byte <<= 16;
        wav &= 0xff00ffff;
        wav |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(3) as u32;
        byte <<= 24;
        wav &= 0xffffff;
        wav |= byte;
    }
    (*track).tone.wav = wav as usize as *mut WaveData;
    (*track).cmdPtr = (*track).cmdPtr.at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xtype(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.r#type = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xatta(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.attack = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xdeca(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.decay = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xsust(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.sustain = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xrele(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.release = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xiecv(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).pseudoEchoVolume = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xiecl(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).pseudoEchoLength = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xleng(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.length = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xswee(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    (*track).tone.pan_sweep = *(*track).cmdPtr;
    (*track).cmdPtr = (*track).cmdPtr.at(1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xwait(mplayInfo: *mut MusicPlayerInfo, track: *mut MusicPlayerTrack) {
    let mut len: u32 = 0;
    len = 0;
    {
        let mut byte: u32 = *(*track).cmdPtr as u32;
        byte <<= 0;
        len &= 0xffffff00;
        len |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(1) as u32;
        byte <<= 8;
        len &= 0xffff00ff;
        len |= byte;
    }
    if (*track).timer < len as u16 {
        (*track).timer += 1;
        (*track).cmdPtr = (*track).cmdPtr.at(-2);
        (*track).wait = 1;
    } else {
        (*track).timer = 0;
        (*track).cmdPtr = (*track).cmdPtr.at(2);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_xcmd_0D(
    mplayInfo: *mut MusicPlayerInfo,
    track: *mut MusicPlayerTrack,
) {
    let mut unk: u32 = 0;
    unk = 0;
    {
        let mut byte: u32 = *(*track).cmdPtr as u32;
        byte <<= 0;
        unk &= 0xffffff00;
        unk |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(1) as u32;
        byte <<= 8;
        unk &= 0xffff00ff;
        unk |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(2) as u32;
        byte <<= 16;
        unk &= 0xff00ffff;
        unk |= byte;
    }
    {
        let mut byte: u32 = *(*track).cmdPtr.at(3) as u32;
        byte <<= 24;
        unk &= 0xffffff;
        unk |= byte;
    }
    (*track).unk_3C = unk;
    (*track).cmdPtr = (*track).cmdPtr.at(4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DummyFunc() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryTone(tone: *mut ToneData) -> *mut MusicPlayerInfo {
    let mut maxClock: u32 = 0;
    let mut maxClockIndex: i32 = 0;
    let mut i: i32 = 0;
    let mut mplayInfo: *mut MusicPlayerInfo = null_mut();
    'start_song: {
        i = 0;
        while i < MAX_POKEMON_CRIES {
            let mut track: *mut MusicPlayerTrack = &raw mut gPokemonCryTracks[i * 2];
            if (*track).flags == 0 && ((*track).chan.is_null() || (*(*track).chan).track != track) {
                break 'start_song;
            }
            if maxClock < gPokemonCryMusicPlayers[i].clock {
                maxClock = gPokemonCryMusicPlayers[i].clock;
                maxClockIndex = i;
            }
            i += 1;
        }
        i = maxClockIndex;
    }
    mplayInfo = &raw mut gPokemonCryMusicPlayers[i];
    (*mplayInfo).ident += 1;
    gPokemonCrySongs[i] = gPokemonCrySong;
    gPokemonCrySongs[i].tone = tone;
    gPokemonCrySongs[i].part[0] = &raw mut gPokemonCrySongs[i].part0;
    gPokemonCrySongs[i].part[1] = &raw mut gPokemonCrySongs[i].part1;
    gPokemonCrySongs[i].gotoTarget = &raw mut gPokemonCrySongs[i].cont as usize as u32;
    (*mplayInfo).ident = ID_NUMBER;
    MPlayStart(mplayInfo, &raw mut gPokemonCrySongs[i] as *mut SongHeader);
    return mplayInfo;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryVolume(val: u8) {
    gPokemonCrySong.volumeValue = val & 0x7F;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPanpot(val: i8) {
    gPokemonCrySong.panValue = val as u8 + C_V & 0x7F;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPitch(val: i16) {
    let mut b: i16 = val + 0x80;
    let mut a: u8 = gPokemonCrySong.tuneValue2 - gPokemonCrySong.tuneValue;
    gPokemonCrySong.tieKeyValue = (b >> 8) as u8 & 0x7F;
    gPokemonCrySong.tuneValue = (b >> 1) as u8 & 0x7F;
    gPokemonCrySong.tuneValue2 = a + ((b >> 1) as u8 & 0x7F) & 0x7F;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryLength(val: u16) {
    gPokemonCrySong.length = val;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryRelease(val: u8) {
    gPokemonCrySong.releaseValue = val;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryProgress(val: u32) {
    gPokemonCrySong.unkCmd0DParam = val;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokemonCryPlaying(mplayInfo: *mut MusicPlayerInfo) -> u32 {
    let mut track: *mut MusicPlayerTrack = (*mplayInfo).tracks;
    if !(*track).chan.is_null() && (*(*track).chan).track == track {
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
pub unsafe extern "C" fn SetPokemonCryChorus(val: i8) {
    if val != 0 {
        gPokemonCrySong.trackCount = 2;
        gPokemonCrySong.tuneValue2 = val as u8 + gPokemonCrySong.tuneValue & 0x7F;
    } else {
        gPokemonCrySong.trackCount = 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryStereo(val: u32) {
    let mut soundInfo: *mut SoundInfo = *(0x3007FF0 as usize as *mut *mut SoundInfo);
    if val != 0 {
        volatile_write(67108994 as usize as *mut u16, 8462);
        (*soundInfo).mode &= 254;
    } else {
        volatile_write(67108994 as usize as *mut u16, 13058);
        (*soundInfo).mode |= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonCryPriority(val: u8) {
    gPokemonCrySong.priority = val;
}
