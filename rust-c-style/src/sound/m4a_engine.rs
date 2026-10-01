//! The music sequencer half of the m4a sound engine (was src/m4a_1.s, all
//! but the mixer): MPlayMain, the song commands (`ply_*`), note allocation
//! (`ply_note`), channel chains and the per-frame DMA restart.
//!
//! Structures are used through byte offsets from constants/m4a_constants.inc
//! (the same the assembly used). Several of the assembly routines passed
//! values in non-standard registers between themselves; as all of them move
//! together, they are ordinary functions here.

// struct MusicPlayerInfo
const MPI_STATUS: usize = 0x04;
const MPI_TRACK_COUNT: usize = 0x08;
const MPI_PRIORITY: usize = 0x09;
const MPI_CMD: usize = 0x0A;
const MPI_CLOCK: usize = 0x0C;
const MPI_TEMPO_D: usize = 0x1C;
const MPI_TEMPO_U: usize = 0x1E;
const MPI_TEMPO_I: usize = 0x20;
const MPI_TEMPO_C: usize = 0x22;
const MPI_TRACKS: usize = 0x2C;
const MPI_TONE: usize = 0x30;
const MPI_IDENT: usize = 0x34;
const MPI_FUNC: usize = 0x38;
const MPI_INTP: usize = 0x3C;

// struct MusicPlayerTrack
const T_FLAGS: usize = 0x00;
const T_WAIT: usize = 0x01;
const T_PATTERN_LEVEL: usize = 0x02;
const T_REP_N: usize = 0x03;
const T_GATE_TIME: usize = 0x04;
const T_KEY: usize = 0x05;
const T_VELOCITY: usize = 0x06;
const T_RUNNING_STATUS: usize = 0x07;
const T_KEY_M: usize = 0x08;
const T_PIT_M: usize = 0x09;
const T_KEY_SHIFT: usize = 0x0A;
const T_TUNE: usize = 0x0C;
const T_BEND: usize = 0x0E;
const T_BEND_RANGE: usize = 0x0F;
const T_VOL_MR: usize = 0x10;
const T_VOL_ML: usize = 0x11;
const T_VOL: usize = 0x12;
const T_VOL_X: usize = 0x13;
const T_PAN: usize = 0x14;
const T_MOD_M: usize = 0x16;
const T_MOD: usize = 0x17;
const T_MOD_T: usize = 0x18;
const T_LFO_SPEED: usize = 0x19;
const T_LFO_SPEED_C: usize = 0x1A;
const T_LFO_DELAY: usize = 0x1B;
const T_LFO_DELAY_C: usize = 0x1C;
const T_PRIORITY: usize = 0x1D;
const T_PSEUDO_ECHO_VOLUME: usize = 0x1E;
const T_CHAN: usize = 0x20;
const T_TONE: usize = 0x24; // struct ToneData (12 bytes)
const T_UNK_3C: usize = 0x3C;
const T_CMD_PTR: usize = 0x40;
const T_PATTERN_STACK: usize = 0x44;
const TRACK_SIZE: usize = 0x50;

// struct ToneData
const TD_TYPE: usize = 0x0;
const TD_KEY: usize = 0x1;
const TD_LENGTH: usize = 0x2;
const TD_PAN_SWEEP: usize = 0x3;
const TD_WAV: usize = 0x4; // also the sub-voice table of a split/rhythm voice
const TD_ATTACK: usize = 0x8; // also the key split table of a split voice
const TONE_SIZE: usize = 0xC;

// struct SoundChannel / CgbChannel (shared prefix)
const C_STATUS: usize = 0x00;
const C_TYPE: usize = 0x01;
const C_RIGHT_VOLUME: usize = 0x02;
const C_LEFT_VOLUME: usize = 0x03;
const C_ATTACK: usize = 0x04;
const C_KEY: usize = 0x08;
const C_PSEUDO_ECHO_VOLUME: usize = 0x0C;
const C_GATE_TIME: usize = 0x10;
const C_MIDI_KEY: usize = 0x11;
const C_VELOCITY: usize = 0x12;
const C_PRIORITY: usize = 0x13;
const C_RHYTHM_PAN: usize = 0x14;
const C_COUNT: usize = 0x18;
const CGB_MODIFY: usize = 0x1D;
const CGB_LENGTH: usize = 0x1E;
const CGB_SWEEP: usize = 0x1F;
const C_FREQUENCY: usize = 0x20;
const C_WAV: usize = 0x24;
const C_TRACK: usize = 0x2C;
const C_PREV: usize = 0x30;
const C_NEXT: usize = 0x34;
const CHANNEL_SIZE: usize = 0x40;

// struct SoundInfo
const SI_IDENT: usize = 0x00;
const SI_PCM_DMA_COUNTER: usize = 0x04;
const SI_MAX_CHANS: usize = 0x06;
const SI_PCM_DMA_PERIOD: usize = 0x0B;
const SI_CGB_CHANS: usize = 0x1C;
const SI_CGB_OSC_OFF: usize = 0x2C;
const SI_MIDI_KEY_TO_CGB_FREQ: usize = 0x30;
const SI_MPLAY_JUMP_TABLE: usize = 0x34;
const SI_PLYNOTE: usize = 0x38;
const SI_CHANS: usize = 0x50;

const ID_NUMBER: u32 = 0x6873_6D53;
const C_V: u8 = 0x40;

const TONEDATA_TYPE_CGB: u8 = 0x07;
const TONEDATA_TYPE_SPL: u8 = 0x40;
const TONEDATA_TYPE_RHY: u8 = 0x80;
const TONEDATA_P_S_PAN: u8 = 0xC0;

const SF_START: u8 = 0x80;
const SF_STOP: u8 = 0x40;
const SF_ENV: u8 = 0x03;
const SF_ON: u8 = 0xC7;

const CGB_MO_PIT: u8 = 0x02;
const CGB_MO_VOL: u8 = 0x01;

const MPT_FLG_VOLCHG: u8 = 0x03;
const MPT_FLG_PITCHG: u8 = 0x0C;
const MPT_FLG_START: u8 = 0x40;
const MPT_FLG_EXIST: u8 = 0x80;

const SOUND_INFO_PTR: *const *mut u8 = 0x0300_7FF0 as *const *mut u8;

type Ptr = *mut u8;

unsafe fn r8(p: Ptr, off: usize) -> u8 {
    unsafe { p.add(off).read() }
}
unsafe fn w8(p: Ptr, off: usize, v: u8) {
    unsafe { p.add(off).write(v) }
}
unsafe fn r16(p: Ptr, off: usize) -> u16 {
    unsafe { p.add(off).cast::<u16>().read() }
}
unsafe fn w16(p: Ptr, off: usize, v: u16) {
    unsafe { p.add(off).cast::<u16>().write(v) }
}
unsafe fn r32(p: Ptr, off: usize) -> u32 {
    unsafe { p.add(off).cast::<u32>().read() }
}
unsafe fn w32(p: Ptr, off: usize, v: u32) {
    unsafe { p.add(off).cast::<u32>().write(v) }
}
unsafe fn rp(p: Ptr, off: usize) -> Ptr {
    unsafe { p.add(off).cast::<Ptr>().read() }
}
unsafe fn wp(p: Ptr, off: usize, v: Ptr) {
    unsafe { p.add(off).cast::<Ptr>().write(v) }
}
unsafe fn or8(p: Ptr, off: usize, bits: u8) {
    unsafe { w8(p, off, r8(p, off) | bits) }
}

unsafe fn sound_info() -> Ptr {
    unsafe { SOUND_INFO_PTR.read_volatile() }
}

/// `chk_adr_r2`: reads through a song pointer are refused (read as 0) when
/// it points into the BIOS below the jump-table template (a copy
/// protection check of the original library).
fn checked(addr: usize, value: u32) -> u32 {
    if addr >> 25 != 0 {
        return value;
    }
    let template = (&raw const gMPlayJumpTableTemplate) as usize;
    if addr >= template && addr >> 14 == 0 {
        value
    } else {
        0
    }
}

/// `ld_r3_tp_adr_i`: the next command byte (checked).
unsafe fn next_byte(track: Ptr) -> u8 {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        wp(track, T_CMD_PTR, p.add(1));
        checked(p as usize, u32::from(p.read())) as u8
    }
}

/// `ld_r3_tp_adr_i_unchecked`
unsafe fn next_byte_unchecked(track: Ptr) -> u8 {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        wp(track, T_CMD_PTR, p.add(1));
        p.read()
    }
}

/// `u32 umul3232H32(u32, u32)`: the high word of the 64-bit product.
#[unsafe(no_mangle)]
pub extern "C" fn umul3232H32(a: u32, b: u32) -> u32 {
    ((u64::from(a) * u64::from(b)) >> 32) as u32
}

/// `SoundMainBTM`: clear 64 bytes (jump table entry 35, Clear64byte).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundMainBTM(p: *mut u32) {
    unsafe {
        for i in 0..16 {
            p.add(i).write(0);
        }
    }
}

/// `RealClearChain`: unlink a channel from its track's chain (entry 34).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RealClearChain(chan: Ptr) {
    unsafe {
        let track = rp(chan, C_TRACK);
        if track.is_null() {
            return;
        }
        let next = rp(chan, C_NEXT);
        let prev = rp(chan, C_PREV);
        if !prev.is_null() {
            wp(prev, C_NEXT, next);
        } else {
            wp(track, T_CHAN, next);
        }
        if !next.is_null() {
            wp(next, C_PREV, prev);
        }
        wp(chan, C_TRACK, core::ptr::null_mut());
    }
}

/// `ply_fine` (FINE): stop the track's notes and the track.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_fine(_mplay: Ptr, track: Ptr) {
    unsafe {
        let mut chan = rp(track, T_CHAN);
        while !chan.is_null() {
            let sf = r8(chan, C_STATUS);
            if sf & SF_ON != 0 {
                w8(chan, C_STATUS, sf | SF_STOP);
            }
            RealClearChain(chan);
            chan = rp(chan, C_NEXT);
        }
        w8(track, T_FLAGS, 0);
    }
}

/// `MPlayJumpTableCopy(dest)`: the 36 command handlers into `dest`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayJumpTableCopy(dest: *mut u32) {
    unsafe {
        let src = (&raw const gMPlayJumpTableTemplate).cast::<u32>();
        for i in 0..0x24 {
            let addr = src.add(i);
            dest.add(i).write(checked(addr as usize, addr.read()));
        }
    }
}

/// GOTO: jump to the 32-bit address that follows.
unsafe fn goto_target(track: Ptr) {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        let hi = (u32::from(p.add(3).read()) << 24)
            | (u32::from(p.add(2).read()) << 16)
            | (u32::from(p.add(1).read()) << 8);
        let lo = checked(p as usize, u32::from(p.read()));
        wp(track, T_CMD_PTR, (hi | lo) as usize as Ptr);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_goto(_mplay: Ptr, track: Ptr) {
    unsafe { goto_target(track) }
}

/// PATT: call a pattern (three levels deep at most; deeper ends the track).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_patt(mplay: Ptr, track: Ptr) {
    unsafe {
        let level = r8(track, T_PATTERN_LEVEL);
        if level >= 3 {
            ply_fine(mplay, track);
            return;
        }
        wp(
            track,
            T_PATTERN_STACK + usize::from(level) * 4,
            rp(track, T_CMD_PTR).add(4),
        );
        w8(track, T_PATTERN_LEVEL, level + 1);
        goto_target(track);
    }
}

/// PEND: return from a pattern.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_pend(_mplay: Ptr, track: Ptr) {
    unsafe {
        let level = r8(track, T_PATTERN_LEVEL);
        if level == 0 {
            return;
        }
        let level = level - 1;
        w8(track, T_PATTERN_LEVEL, level);
        wp(
            track,
            T_CMD_PTR,
            rp(track, T_PATTERN_STACK + usize::from(level) * 4),
        );
    }
}

/// REPT count, address: repeat `count` times (0 = forever).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_rept(_mplay: Ptr, track: Ptr) {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        if p.read() == 0 {
            wp(track, T_CMD_PTR, p.add(1));
            goto_target(track);
            return;
        }
        let rep = r8(track, T_REP_N).wrapping_add(1);
        w8(track, T_REP_N, rep);
        let count = next_byte(track);
        if rep < count {
            goto_target(track);
        } else {
            w8(track, T_REP_N, 0);
            wp(track, T_CMD_PTR, p.add(5));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_prio(_mplay: Ptr, track: Ptr) {
    unsafe { w8(track, T_PRIORITY, next_byte(track)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_tempo(mplay: Ptr, track: Ptr) {
    unsafe {
        let d = u32::from(next_byte(track)) << 1;
        w16(mplay, MPI_TEMPO_D, d as u16);
        w16(
            mplay,
            MPI_TEMPO_I,
            ((d * u32::from(r16(mplay, MPI_TEMPO_U))) >> 8) as u16,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_keysh(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_KEY_SHIFT, next_byte(track));
        or8(track, T_FLAGS, MPT_FLG_PITCHG);
    }
}

/// VOICE n: copy tone n of the player's voice group into the track.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_voice(mplay: Ptr, track: Ptr) {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        let n = usize::from(p.read());
        wp(track, T_CMD_PTR, p.add(1));
        let tone = rp(mplay, MPI_TONE).add(n * TONE_SIZE);
        // (the address check looks at the tone's base address each time)
        for off in [TD_TYPE, TD_WAV, TD_ATTACK] {
            let word = tone.add(off).cast::<u32>().read();
            w32(track, T_TONE + off, checked(tone as usize, word));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_vol(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_VOL, next_byte(track));
        or8(track, T_FLAGS, MPT_FLG_VOLCHG);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_pan(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_PAN, next_byte(track).wrapping_sub(C_V));
        or8(track, T_FLAGS, MPT_FLG_VOLCHG);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_bend(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_BEND, next_byte(track).wrapping_sub(C_V));
        or8(track, T_FLAGS, MPT_FLG_PITCHG);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_bendr(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_BEND_RANGE, next_byte(track));
        or8(track, T_FLAGS, MPT_FLG_PITCHG);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_lfodl(_mplay: Ptr, track: Ptr) {
    unsafe { w8(track, T_LFO_DELAY, next_byte(track)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_modt(_mplay: Ptr, track: Ptr) {
    unsafe {
        let v = next_byte(track);
        if r8(track, T_MOD_T) != v {
            w8(track, T_MOD_T, v);
            or8(track, T_FLAGS, MPT_FLG_VOLCHG | MPT_FLG_PITCHG);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_tune(_mplay: Ptr, track: Ptr) {
    unsafe {
        w8(track, T_TUNE, next_byte(track).wrapping_sub(C_V));
        or8(track, T_FLAGS, MPT_FLG_PITCHG);
    }
}

/// PORT reg, value: write a sound register (REG_SOUND1CNT_L + reg).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_port(_mplay: Ptr, track: Ptr) {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        let reg = (0x0400_0060usize + usize::from(p.read())) as *mut u8;
        let vp = p.add(1);
        wp(track, T_CMD_PTR, p.add(2));
        let value = checked(vp as usize, u32::from(vp.read())) as u8;
        crate::c::volatile_write(reg, value);
    }
}

/// Restart the Direct Sound FIFO DMAs every `pcmDmaPeriod` frames.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m4aSoundVSync() {
    const REG_DMA1CNT: *mut u32 = 0x0400_00C4 as *mut u32;
    const REG_DMA2CNT: *mut u32 = 0x0400_00D0 as *mut u32;
    const DMA_REPEAT_BIT: u32 = 1 << 25;
    const RESET: u32 = 0x8440_0004; // ENABLE | START_NOW | 32BIT | DEST_FIXED, 4 words
    unsafe {
        let si = sound_info();
        if r32(si, SI_IDENT).wrapping_sub(ID_NUMBER) > 1 {
            return;
        }
        let counter = i32::from(r8(si, SI_PCM_DMA_COUNTER)) - 1;
        w8(si, SI_PCM_DMA_COUNTER, counter as u8);
        if counter > 0 {
            return;
        }
        w8(si, SI_PCM_DMA_COUNTER, r8(si, SI_PCM_DMA_PERIOD));
        for cnt in [REG_DMA1CNT, REG_DMA2CNT] {
            if cnt.read_volatile() & DMA_REPEAT_BIT != 0 {
                crate::c::volatile_write(cnt, RESET);
            }
        }
        for cnt in [REG_DMA1CNT, REG_DMA2CNT] {
            let hi = cnt.cast::<u16>().add(1);
            crate::c::volatile_write(hi, 0x0400); // off
            crate::c::volatile_write(hi, 0xB600); // ENABLE | SPECIAL | 32BIT | REPEAT
        }
    }
}

unsafe fn clear_mod_m(track: Ptr) {
    unsafe {
        w8(track, T_MOD_M, 0);
        w8(track, T_LFO_SPEED_C, 0);
        or8(
            track,
            T_FLAGS,
            if r8(track, T_MOD_T) == 0 {
                MPT_FLG_PITCHG
            } else {
                MPT_FLG_VOLCHG
            },
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_lfos(_mplay: Ptr, track: Ptr) {
    unsafe {
        let v = next_byte_unchecked(track);
        w8(track, T_LFO_SPEED, v);
        if v == 0 {
            clear_mod_m(track);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_mod(_mplay: Ptr, track: Ptr) {
    unsafe {
        let v = next_byte_unchecked(track);
        w8(track, T_MOD, v);
        if v == 0 {
            clear_mod_m(track);
        }
    }
}

/// EOT [key]: release the tied note (the first matching one).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_endtie(_mplay: Ptr, track: Ptr) {
    unsafe {
        let p = rp(track, T_CMD_PTR);
        let key = if p.read() < 0x80 {
            let k = p.read();
            w8(track, T_KEY, k);
            wp(track, T_CMD_PTR, p.add(1));
            k
        } else {
            r8(track, T_KEY)
        };
        let mut chan = rp(track, T_CHAN);
        while !chan.is_null() {
            let sf = r8(chan, C_STATUS);
            if sf & (SF_START | SF_ENV) != 0 && sf & SF_STOP == 0 && r8(chan, C_MIDI_KEY) == key {
                w8(chan, C_STATUS, sf | SF_STOP);
                return;
            }
            chan = rp(chan, C_NEXT);
        }
    }
}

/// `TrackStop(mplayInfo, track)`: silence and unlink every channel.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrackStop(_mplay: Ptr, track: Ptr) {
    unsafe {
        if r8(track, T_FLAGS) & MPT_FLG_EXIST == 0 {
            return;
        }
        let mut chan = rp(track, T_CHAN);
        while !chan.is_null() {
            let sf = r8(chan, C_STATUS);
            if sf != 0 {
                let cgb = r8(chan, C_TYPE) & TONEDATA_TYPE_CGB;
                if cgb != 0 {
                    let osc_off: unsafe extern "C" fn(u8) =
                        core::mem::transmute(rp(sound_info(), SI_CGB_OSC_OFF));
                    osc_off(cgb);
                }
                w8(chan, C_STATUS, 0);
            }
            wp(chan, C_TRACK, core::ptr::null_mut());
            chan = rp(chan, C_NEXT);
        }
        wp(track, T_CHAN, core::ptr::null_mut());
    }
}

/// `ChnVolSetAsm`: channel volumes from velocity, rhythm pan and the
/// track's left/right volume.
unsafe fn chn_vol_set(chan: Ptr, track: Ptr) {
    unsafe {
        let velocity = i32::from(r8(chan, C_VELOCITY));
        let pan = i32::from(r8(chan, C_RHYTHM_PAN) as i8);
        let right = (i32::from(r8(track, T_VOL_MR)) * ((0x80 + pan) * velocity)) >> 14;
        w8(
            chan,
            C_RIGHT_VOLUME,
            if right as u32 > 0xFF {
                0xFF
            } else {
                right as u8
            },
        );
        let left = (i32::from(r8(track, T_VOL_ML)) * ((0x7F - pan) * velocity)) >> 14;
        w8(
            chan,
            C_LEFT_VOLUME,
            if left as u32 > 0xFF { 0xFF } else { left as u8 },
        );
    }
}

/// Note commands 0xCF..: `ply_note(noteLengthIndex, mplayInfo, track)`,
/// reached through SoundInfo.plynote.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ply_note(length_index: u32, mplay: Ptr, track: Ptr) {
    unsafe {
        let si = sound_info();
        let clock = (&raw const gClockTable).cast::<u8>();
        w8(track, T_GATE_TIME, clock.add(length_index as usize).read());

        // key [velocity [gate time extension]]
        let mut p = rp(track, T_CMD_PTR);
        if p.read() < 0x80 {
            w8(track, T_KEY, p.read());
            p = p.add(1);
            if p.read() < 0x80 {
                w8(track, T_VELOCITY, p.read());
                p = p.add(1);
                if p.read() < 0x80 {
                    w8(
                        track,
                        T_GATE_TIME,
                        r8(track, T_GATE_TIME).wrapping_add(p.read()),
                    );
                    p = p.add(1);
                }
            }
            wp(track, T_CMD_PTR, p);
        }

        // Pick the tone: the track's, or a split/rhythm voice's sub-voice.
        let mut rhythm_pan: u32 = 0;
        let track_type = r8(track, T_TONE + TD_TYPE);
        let tone: Ptr;
        let key: u8;
        if track_type & (TONEDATA_TYPE_RHY | TONEDATA_TYPE_SPL) != 0 {
            let track_key = r8(track, T_KEY);
            let index = if track_type & TONEDATA_TYPE_SPL != 0 {
                rp(track, T_TONE + TD_ATTACK)
                    .add(usize::from(track_key))
                    .read()
            } else {
                track_key
            };
            tone = rp(track, T_TONE + TD_WAV).add(usize::from(index) * TONE_SIZE);
            if r8(tone, TD_TYPE) & (TONEDATA_TYPE_SPL | TONEDATA_TYPE_RHY) != 0 {
                return;
            }
            if track_type & TONEDATA_TYPE_RHY != 0 {
                let ps = r8(tone, TD_PAN_SWEEP);
                if ps & 0x80 != 0 {
                    rhythm_pan = u32::from(ps.wrapping_sub(TONEDATA_P_S_PAN)) << 1;
                }
                key = r8(tone, TD_KEY);
            } else {
                key = track_key;
            }
        } else {
            tone = track.add(T_TONE);
            key = r8(track, T_KEY);
        }

        let priority =
            (u32::from(r8(mplay, MPI_PRIORITY)) + u32::from(r8(track, T_PRIORITY))).min(0xFF);
        let cgb = r8(tone, TD_TYPE) & TONEDATA_TYPE_CGB;

        // Find a channel.
        let chan: Ptr;
        if cgb != 0 {
            let chans = rp(si, SI_CGB_CHANS);
            if chans.is_null() {
                return;
            }
            let c = chans.add(usize::from(cgb - 1) * CHANNEL_SIZE);
            let sf = r8(c, C_STATUS);
            if sf & SF_ON != 0 && sf & SF_STOP == 0 {
                let p = u32::from(r8(c, C_PRIORITY));
                // take it from a lower priority, or an equal one on a
                // track at or after this one
                let take =
                    p < priority || (p == priority && rp(c, C_TRACK) as usize >= track as usize);
                if !take {
                    return;
                }
            }
            chan = c;
        } else {
            // a free channel, else the lowest-priority (stopping ones first)
            let mut best_prio = priority;
            let mut best_track = track as usize;
            let mut stopping = false;
            let mut best: Ptr = core::ptr::null_mut();
            let mut c = si.add(SI_CHANS);
            let mut found_free: Ptr = core::ptr::null_mut();
            // do-while in the assembly: one channel is looked at even if
            // maxChans is 0
            let mut count = i32::from(r8(si, SI_MAX_CHANS));
            loop {
                let sf = r8(c, C_STATUS);
                if sf & SF_ON == 0 {
                    found_free = c;
                    break;
                }
                let compare = if sf & SF_STOP != 0 {
                    if !stopping {
                        stopping = true;
                        best_prio = u32::from(r8(c, C_PRIORITY));
                        best_track = rp(c, C_TRACK) as usize;
                        best = c;
                        false
                    } else {
                        true
                    }
                } else {
                    !stopping
                };
                if compare {
                    let p = u32::from(r8(c, C_PRIORITY));
                    let t = rp(c, C_TRACK) as usize;
                    if p < best_prio {
                        best_prio = p;
                        best_track = t;
                        best = c;
                    } else if p == best_prio {
                        if t > best_track {
                            best_track = t;
                            best = c;
                        } else if t == best_track {
                            best = c;
                        }
                    }
                }
                c = c.add(CHANNEL_SIZE);
                count -= 1;
                if count <= 0 {
                    break;
                }
            }
            chan = if !found_free.is_null() {
                found_free
            } else {
                best
            };
            if chan.is_null() {
                return;
            }
        }

        // Put it at the head of the track's chain and set it up.
        crate::m4a::ClearChain(chan.cast());
        wp(chan, C_PREV, core::ptr::null_mut());
        let head = rp(track, T_CHAN);
        wp(chan, C_NEXT, head);
        if !head.is_null() {
            wp(head, C_PREV, chan);
        }
        wp(track, T_CHAN, chan);
        wp(chan, C_TRACK, track);
        let lfo_delay = r8(track, T_LFO_DELAY);
        w8(track, T_LFO_DELAY_C, lfo_delay);
        if lfo_delay != 0 {
            clear_mod_m(track);
        }
        crate::m4a::TrkVolPitSet(mplay.cast(), track.cast());
        // gate time, key, velocity, running status in one word; the last
        // three land on midiKey, velocity and priority (then overwritten)
        w32(chan, C_GATE_TIME, r32(track, T_GATE_TIME));
        w8(chan, C_PRIORITY, priority as u8);
        w8(chan, C_KEY, key);
        w8(chan, C_RHYTHM_PAN, rhythm_pan as u8);
        w8(chan, C_TYPE, r8(tone, TD_TYPE));
        let wav = rp(tone, TD_WAV);
        wp(chan, C_WAV, wav);
        w32(chan, C_ATTACK, r32(tone, TD_ATTACK)); // attack, decay, sustain, release
        w16(chan, C_PSEUDO_ECHO_VOLUME, r16(track, T_PSEUDO_ECHO_VOLUME)); // and its length
        chn_vol_set(chan, track);

        let mut k = i32::from(r8(chan, C_KEY)) + i32::from(r8(track, T_KEY_M) as i8);
        if k < 0 {
            k = 0;
        }
        let frequency = if cgb != 0 {
            w8(chan, CGB_LENGTH, r8(tone, TD_LENGTH));
            let ps = r8(tone, TD_PAN_SWEEP);
            w8(
                chan,
                CGB_SWEEP,
                if ps & 0x80 != 0 || ps & 0x70 == 0 {
                    8
                } else {
                    ps
                },
            );
            let to_freq: unsafe extern "C" fn(u8, u8, u8) -> u32 =
                core::mem::transmute(rp(si, SI_MIDI_KEY_TO_CGB_FREQ));
            to_freq(cgb, k as u8, r8(track, T_PIT_M))
        } else {
            w32(chan, C_COUNT, r32(track, T_UNK_3C));
            crate::m4a::MidiKeyToFreq(wav.cast(), k as u8, r8(track, T_PIT_M))
        };
        w32(chan, C_FREQUENCY, frequency);
        w8(chan, C_STATUS, SF_START);
        w8(track, T_FLAGS, r8(track, T_FLAGS) & 0xF0);
    }
}

/// `MPlayMain(mplayInfo)`: one frame of a music player: run as many ticks
/// as the tempo allows, then push volume/pitch changes to the channels.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MPlayMain(mplay: Ptr) {
    unsafe {
        if r32(mplay, MPI_IDENT) != ID_NUMBER {
            return;
        }
        w32(mplay, MPI_IDENT, ID_NUMBER + 1);
        let func = r32(mplay, MPI_FUNC);
        if func != 0 {
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(func as usize);
            f(r32(mplay, MPI_INTP));
        }
        run(mplay);
        w32(mplay, MPI_IDENT, ID_NUMBER);
    }
}

unsafe fn run(mplay: Ptr) {
    unsafe {
        if (r32(mplay, MPI_STATUS) as i32) < 0 {
            return;
        }
        let si = sound_info();
        crate::m4a::FadeOutBody(mplay.cast());
        if (r32(mplay, MPI_STATUS) as i32) < 0 {
            return;
        }

        let mut tempo = u32::from(r16(mplay, MPI_TEMPO_C)) + u32::from(r16(mplay, MPI_TEMPO_I));
        loop {
            w16(mplay, MPI_TEMPO_C, tempo as u16);
            if tempo < 150 {
                break;
            }
            // One tick for every track.
            let mut active: u32 = 0;
            let mut bit: u32 = 1;
            let mut track = rp(mplay, MPI_TRACKS);
            let mut left = i32::from(r8(mplay, MPI_TRACK_COUNT));
            loop {
                if r8(track, T_FLAGS) & MPT_FLG_EXIST != 0 {
                    active |= bit;
                    tick_track(si, mplay, track);
                }
                left -= 1;
                if left <= 0 {
                    break;
                }
                track = track.add(TRACK_SIZE);
                bit <<= 1;
            }
            w32(mplay, MPI_CLOCK, r32(mplay, MPI_CLOCK).wrapping_add(1));
            if active == 0 {
                w32(mplay, MPI_STATUS, 0x8000_0000);
                return;
            }
            w32(mplay, MPI_STATUS, active);
            tempo = u32::from(r16(mplay, MPI_TEMPO_C)).wrapping_sub(150);
        }

        // Apply volume and pitch changes.
        let mut track = rp(mplay, MPI_TRACKS);
        let mut left = i32::from(r8(mplay, MPI_TRACK_COUNT));
        loop {
            let flags = r8(track, T_FLAGS);
            if flags & 0x80 != 0 && flags & (MPT_FLG_VOLCHG | MPT_FLG_PITCHG) != 0 {
                crate::m4a::TrkVolPitSet(mplay.cast(), track.cast());
                let mut chan = rp(track, T_CHAN);
                while !chan.is_null() {
                    if r8(chan, C_STATUS) & SF_ON == 0 {
                        crate::m4a::ClearChain(chan.cast());
                    } else {
                        let cgb = r8(chan, C_TYPE) & TONEDATA_TYPE_CGB;
                        if r8(track, T_FLAGS) & MPT_FLG_VOLCHG != 0 {
                            chn_vol_set(chan, track);
                            if cgb != 0 {
                                or8(chan, CGB_MODIFY, CGB_MO_VOL);
                            }
                        }
                        if r8(track, T_FLAGS) & MPT_FLG_PITCHG != 0 {
                            let mut k =
                                i32::from(r8(chan, C_KEY)) + i32::from(r8(track, T_KEY_M) as i8);
                            if k < 0 {
                                k = 0;
                            }
                            if cgb != 0 {
                                let to_freq: unsafe extern "C" fn(u8, u8, u8) -> u32 =
                                    core::mem::transmute(rp(si, SI_MIDI_KEY_TO_CGB_FREQ));
                                w32(chan, C_FREQUENCY, to_freq(cgb, k as u8, r8(track, T_PIT_M)));
                                or8(chan, CGB_MODIFY, CGB_MO_PIT);
                            } else {
                                let f = crate::m4a::MidiKeyToFreq(
                                    rp(chan, C_WAV).cast(),
                                    k as u8,
                                    r8(track, T_PIT_M),
                                );
                                w32(chan, C_FREQUENCY, f);
                            }
                        }
                    }
                    chan = rp(chan, C_NEXT);
                }
                w8(track, T_FLAGS, r8(track, T_FLAGS) & 0xF0);
            }
            left -= 1;
            if left <= 0 {
                break;
            }
            track = track.add(TRACK_SIZE);
        }
    }
}

/// One tick of one (existing) track.
unsafe fn tick_track(si: Ptr, mplay: Ptr, track: Ptr) {
    unsafe {
        // Gate times: stop notes whose time is up; unlink finished channels.
        let mut chan = rp(track, T_CHAN);
        while !chan.is_null() {
            let sf = r8(chan, C_STATUS);
            if sf & SF_ON != 0 {
                let gate = r8(chan, C_GATE_TIME);
                if gate != 0 {
                    let gate = gate - 1;
                    w8(chan, C_GATE_TIME, gate);
                    if gate == 0 {
                        w8(chan, C_STATUS, sf | SF_STOP);
                    }
                }
            } else {
                crate::m4a::ClearChain(chan.cast());
            }
            chan = rp(chan, C_NEXT);
        }

        if r8(track, T_FLAGS) & MPT_FLG_START != 0 {
            crate::m4a::Clear64byte(track.cast());
            w8(track, T_FLAGS, MPT_FLG_EXIST);
            w8(track, T_BEND_RANGE, 2);
            w8(track, T_VOL_X, 0x40);
            w8(track, T_LFO_SPEED, 0x16);
            w8(track, T_TONE + TD_TYPE, 1);
        }

        // Commands until a wait.
        while r8(track, T_WAIT) == 0 {
            let p = rp(track, T_CMD_PTR);
            let mut cmd = p.read();
            if cmd < 0x80 {
                cmd = r8(track, T_RUNNING_STATUS);
            } else {
                wp(track, T_CMD_PTR, p.add(1));
                if cmd >= 0xBD {
                    w8(track, T_RUNNING_STATUS, cmd);
                }
            }
            if cmd >= 0xCF {
                let note: unsafe extern "C" fn(u32, Ptr, Ptr) =
                    core::mem::transmute(rp(si, SI_PLYNOTE));
                note(u32::from(cmd - 0xCF), mplay, track);
            } else if cmd > 0xB0 {
                let n = cmd - 0xB1;
                w8(mplay, MPI_CMD, n);
                let table = rp(si, SI_MPLAY_JUMP_TABLE).cast::<u32>();
                let handler: unsafe extern "C" fn(Ptr, Ptr) =
                    core::mem::transmute(table.add(usize::from(n)).read() as usize);
                handler(mplay, track);
                if r8(track, T_FLAGS) == 0 {
                    return; // the track ended
                }
            } else {
                let clock = (&raw const gClockTable).cast::<u8>();
                w8(track, T_WAIT, clock.add(usize::from(cmd - 0x80)).read());
            }
        }
        w8(track, T_WAIT, r8(track, T_WAIT) - 1);

        // LFO
        let speed = r8(track, T_LFO_SPEED);
        if speed == 0 || r8(track, T_MOD) == 0 {
            return;
        }
        let delay = r8(track, T_LFO_DELAY_C);
        if delay != 0 {
            w8(track, T_LFO_DELAY_C, delay - 1);
            return;
        }
        // The sum is used unwrapped (up to 510) below, as in the assembly.
        let sum = i32::from(r8(track, T_LFO_SPEED_C)) + i32::from(speed);
        w8(track, T_LFO_SPEED_C, sum as u8);
        // a triangle wave: phase as s8 while |.| < 0x40, folded otherwise
        let wave: i32 = if (sum - 0x40) & 0x80 != 0 {
            i32::from(sum as u8 as i8)
        } else {
            0x80 - sum
        };
        let m = (i32::from(r8(track, T_MOD)) * wave) >> 6;
        if (r8(track, T_MOD_M) ^ m as u8) != 0 {
            w8(track, T_MOD_M, m as u8);
            or8(
                track,
                T_FLAGS,
                if r8(track, T_MOD_T) == 0 {
                    MPT_FLG_PITCHG
                } else {
                    MPT_FLG_VOLCHG
                },
            );
        }
    }
}

unsafe extern "C" {
    static gMPlayJumpTableTemplate: u8;
    static gClockTable: u8;
}
