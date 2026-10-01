//! The m4a mixer (was SoundMain / SoundMainRAM in src/m4a_1.s).
//!
//! Once a frame SoundMain runs the music players and the CGB channels, then
//! mixes the Direct Sound channels into the PCM buffer the FIFO DMAs play.
//! The mixing runs from IWRAM (section `.iwram_code`, copied there by Init)
//! as ARM code, like the original. It must not call into ROM: a `bl` cannot
//! reach from IWRAM to ROM, so everything it needs is inlined.
//!
//! The assembly mixed four samples at a time in one register, rotating it a
//! byte per sample; per byte that is exactly
//! `out = out + ((volume * sample) >> 8)` modulo 256, which is what this does.

use core::ptr::{read_volatile, write_volatile};

type Ptr = *mut u8;

// struct SoundInfo
const SI_IDENT: usize = 0x00;
const SI_PCM_DMA_COUNTER: usize = 0x04;
const SI_REVERB: usize = 0x05;
const SI_MAX_CHANS: usize = 0x06;
const SI_MASTER_VOLUME: usize = 0x07;
const SI_PCM_DMA_PERIOD: usize = 0x0B;
const SI_MAX_LINES: usize = 0x0C;
const SI_PCM_SAMPLES_PER_VBLANK: usize = 0x10;
const SI_DIV_FREQ: usize = 0x18;
const SI_MPLAY_MAIN_HEAD: usize = 0x20;
const SI_MUSIC_PLAYER_HEAD: usize = 0x24;
const SI_CGB_SOUND: usize = 0x28;
const SI_CHANS: usize = 0x50;
const SI_PCM_BUFFER: usize = 0x350;

// struct SoundChannel
const C_STATUS: usize = 0x00;
const C_TYPE: usize = 0x01;
const C_RIGHT_VOLUME: usize = 0x02;
const C_LEFT_VOLUME: usize = 0x03;
const C_ATTACK: usize = 0x04;
const C_DECAY: usize = 0x05;
const C_SUSTAIN: usize = 0x06;
const C_RELEASE: usize = 0x07;
const C_ENVELOPE_VOLUME: usize = 0x09;
const C_ENVELOPE_VOLUME_RIGHT: usize = 0x0A;
const C_ENVELOPE_VOLUME_LEFT: usize = 0x0B;
const C_PSEUDO_ECHO_VOLUME: usize = 0x0C;
const C_PSEUDO_ECHO_LENGTH: usize = 0x0D;
const C_COUNT: usize = 0x18;
const C_FW: usize = 0x1C;
const C_FREQUENCY: usize = 0x20;
const C_WAV: usize = 0x24;
const C_CURRENT_POINTER: usize = 0x28;
const C_XPI: usize = 0x3C;
const CHANNEL_SIZE: usize = 0x40;

// struct WaveData
const W_TYPE: usize = 0x00;
const W_FLAGS: usize = 0x03;
const W_LOOP_START: usize = 0x08;
const W_SIZE: usize = 0x0C;
const W_DATA: usize = 0x10;

const ID_NUMBER: u32 = 0x6873_6D53;
const PCM_DMA_BUF_SIZE: usize = 1584;
const VCOUNT_VBLANK: u32 = 160;
const TOTAL_SCANLINES: u32 = 228;
const REG_VCOUNT: *const u8 = 0x0400_0006 as *const u8;

const SF_START: u8 = 0x80;
const SF_STOP: u8 = 0x40;
const SF_SPECIAL: u8 = 0x20;
const SF_LOOP: u8 = 0x10;
const SF_IEC: u8 = 0x04;
const SF_ENV: u8 = 0x03;
const SF_ENV_ATTACK: u8 = 0x03;
const SF_ENV_DECAY: u8 = 0x02;
const SF_ON: u8 = 0xC7;
const WAVE_DATA_FLAG_LOOP: u8 = 0xC0;
const TONEDATA_TYPE_FIX: u8 = 0x08;
const TONEDATA_TYPE_REV: u8 = 0x10;
const TONEDATA_TYPE_CMP: u8 = 0x20;

/// Samples decoded from one 64-sample block of compressed (DPCM) data. One
/// buffer for all channels, as in the original (each channel remembers
/// which block it last decoded, in `xpi`).
#[unsafe(no_mangle)]
static mut sDecodingBuffer: crate::ffi::Align4<[i8; 0x40]> = crate::ffi::Align4([0; 0x40]);

#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn r8(p: Ptr, off: usize) -> u8 {
    unsafe { read_volatile(p.add(off)) }
}
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn w8(p: Ptr, off: usize, v: u8) {
    unsafe { write_volatile(p.add(off), v) }
}
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn r32(p: Ptr, off: usize) -> u32 {
    unsafe { read_volatile(p.add(off).cast::<u32>()) }
}
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn w32(p: Ptr, off: usize, v: u32) {
    unsafe { write_volatile(p.add(off).cast::<u32>(), v) }
}
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn rp(p: Ptr, off: usize) -> Ptr {
    unsafe { read_volatile(p.add(off).cast::<Ptr>()) }
}
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn wp(p: Ptr, off: usize, v: Ptr) {
    unsafe { write_volatile(p.add(off).cast::<Ptr>(), v) }
}

/// The scanline, counting the vblank lines as after the visible ones.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn scanline() -> u32 {
    let v = u32::from(unsafe { read_volatile(REG_VCOUNT) });
    if v < VCOUNT_VBLANK {
        v + TOTAL_SCANLINES
    } else {
        v
    }
}

/// `void SoundMain(void)`, called once a frame (m4aSoundMain).
///
/// This one stays in ROM (Thumb), so it uses none of the IWRAM helpers below.
#[unsafe(no_mangle)]
pub unsafe fn SoundMain() {
    unsafe {
        let si = read_volatile(0x0300_7FF0 as *const Ptr);
        let ident = si.add(SI_IDENT).cast::<u32>();
        if read_volatile(ident) != ID_NUMBER {
            return;
        }
        write_volatile(ident, ID_NUMBER + 1);

        // A scanline budget: stop mixing channels once it is reached.
        let mut limit = u32::from(read_volatile(si.add(SI_MAX_LINES)));
        if limit != 0 {
            let v = u32::from(read_volatile(REG_VCOUNT));
            limit += if v < VCOUNT_VBLANK {
                v + TOTAL_SCANLINES
            } else {
                v
            };
        }
        let main_head = read_volatile(si.add(SI_MPLAY_MAIN_HEAD).cast::<usize>());
        if main_head != 0 {
            let f: unsafe fn(Ptr) = core::mem::transmute(main_head);
            f(read_volatile(si.add(SI_MUSIC_PLAYER_HEAD).cast::<Ptr>()));
        }
        let cgb: unsafe fn() =
            core::mem::transmute(read_volatile(si.add(SI_CGB_SOUND).cast::<usize>()));
        cgb();

        // This frame's part of the ring of PCM buffers.
        let samples = read_volatile(si.add(SI_PCM_SAMPLES_PER_VBLANK).cast::<u32>());
        let counter = read_volatile(si.add(SI_PCM_DMA_COUNTER));
        let mut buf = si.add(SI_PCM_BUFFER);
        if counter > 1 {
            let done = u32::from(read_volatile(si.add(SI_PCM_DMA_PERIOD)))
                .wrapping_sub(u32::from(counter) - 1);
            buf = buf.add(done.wrapping_mul(samples) as usize);
        }

        // The mixer is in IWRAM, out of `bl` range: call it through a pointer.
        let mixer: unsafe fn(Ptr, Ptr, u32, u32) = read_volatile(&raw const MIXER);
        mixer(si, buf, samples, limit);

        write_volatile(ident, ID_NUMBER);
    }
}

static MIXER: unsafe fn(Ptr, Ptr, u32, u32) = SoundMainRAM;

/// `SoundMainRAM`: reverb (or clear) this frame's buffer, then mix every
/// Direct Sound channel into it.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
#[inline(never)]
pub unsafe fn SoundMainRAM(si: Ptr, buf: Ptr, samples: u32, limit: u32) {
    unsafe {
        let n = samples as usize;
        let reverb = r8(si, SI_REVERB);
        if reverb != 0 {
            // Echo: mix the next buffer's (older) samples back in.
            let mut other = if r8(si, SI_PCM_DMA_COUNTER) == 2 {
                si.add(SI_PCM_BUFFER)
            } else {
                buf.add(n)
            };
            let mut p = buf;
            let mut left = if n == 0 { 1 } else { n };
            while left > 0 {
                left -= 1;
                let sum = i32::from(read_volatile(p.add(PCM_DMA_BUF_SIZE)) as i8)
                    + i32::from(read_volatile(p) as i8)
                    + i32::from(read_volatile(other.add(PCM_DMA_BUF_SIZE)) as i8)
                    + i32::from(read_volatile(other) as i8);
                let mut v = (sum * i32::from(reverb)) >> 9;
                if v & 0x80 != 0 {
                    v += 1;
                }
                write_volatile(p.add(PCM_DMA_BUF_SIZE), v as u8);
                write_volatile(p, v as u8);
                p = p.add(1);
                other = other.add(1);
            }
        } else {
            // (volatile, so that this does not become a call to memset in ROM)
            let p = buf.cast::<u32>();
            let mut i = 0;
            while i < n / 4 {
                write_volatile(p.add(i), 0);
                write_volatile(p.add(PCM_DMA_BUF_SIZE / 4 + i), 0);
                i += 1;
            }
        }

        let div_freq = r32(si, SI_DIV_FREQ);
        let master = u32::from(r8(si, SI_MASTER_VOLUME));
        let mut chan = si.add(SI_CHANS);
        let mut left = i32::from(r8(si, SI_MAX_CHANS));
        loop {
            if limit != 0 && scanline() >= limit {
                return;
            }
            mix_channel(chan, buf, n, div_freq, master);
            left -= 1;
            if left <= 0 {
                return;
            }
            chan = chan.add(CHANNEL_SIZE);
        }
    }
}

/// The channel being mixed. The mixing loops read it from here when they
/// need it (rarely) instead of keeping it in a register, which the volumes
/// need more.
static mut MIX_CHANNEL: Ptr = core::ptr::null_mut();

#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn chan() -> Ptr {
    unsafe { read_volatile(&raw const MIX_CHANNEL) }
}

/// Envelope, then mixing, for one channel.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn mix_channel(c: Ptr, buf: Ptr, n: usize, div_freq: u32, master: u32) {
    unsafe {
        write_volatile(&raw mut MIX_CHANNEL, c);
        let wav = rp(c, C_WAV);
        let mut sf = r8(c, C_STATUS);
        if sf & SF_ON == 0 {
            return;
        }
        let mut env: u32;
        // `start` is the attack branch target of the assembly.
        let mut attack = false;
        if sf & SF_START != 0 {
            if sf & SF_STOP != 0 {
                w8(c, C_STATUS, 0);
                return;
            }
            sf = SF_ENV_ATTACK;
            w8(c, C_STATUS, sf);
            let offset = r32(c, C_COUNT);
            wp(c, C_CURRENT_POINTER, wav.add(W_DATA).add(offset as usize));
            w32(c, C_COUNT, r32(wav, W_SIZE).wrapping_sub(offset));
            env = 0;
            w8(c, C_ENVELOPE_VOLUME, 0);
            w32(c, C_FW, 0);
            if r8(wav, W_FLAGS) & WAVE_DATA_FLAG_LOOP != 0 {
                sf |= SF_LOOP;
                w8(c, C_STATUS, sf);
            }
            attack = true;
        } else {
            env = u32::from(r8(c, C_ENVELOPE_VOLUME));
            if sf & SF_IEC != 0 {
                let len = r8(c, C_PSEUDO_ECHO_LENGTH);
                w8(c, C_PSEUDO_ECHO_LENGTH, len.wrapping_sub(1));
                if len < 2 {
                    w8(c, C_STATUS, 0);
                    return;
                }
            } else if sf & SF_STOP != 0 {
                env = (env * u32::from(r8(c, C_RELEASE))) >> 8;
                if env <= u32::from(r8(c, C_PSEUDO_ECHO_VOLUME)) {
                    // pseudo echo: hold at its volume, or end
                    env = u32::from(r8(c, C_PSEUDO_ECHO_VOLUME));
                    if env == 0 {
                        w8(c, C_STATUS, 0);
                        return;
                    }
                    sf |= SF_IEC;
                    w8(c, C_STATUS, sf);
                }
            } else {
                match sf & SF_ENV {
                    SF_ENV_DECAY => {
                        env = (env * u32::from(r8(c, C_DECAY))) >> 8;
                        if env <= u32::from(r8(c, C_SUSTAIN)) {
                            env = u32::from(r8(c, C_SUSTAIN));
                            if env == 0 {
                                // decayed to nothing: pseudo echo or end
                                env = u32::from(r8(c, C_PSEUDO_ECHO_VOLUME));
                                if env == 0 {
                                    w8(c, C_STATUS, 0);
                                    return;
                                }
                                sf |= SF_IEC;
                                w8(c, C_STATUS, sf);
                            } else {
                                sf -= 1; // to sustain
                                w8(c, C_STATUS, sf);
                            }
                        }
                    }
                    SF_ENV_ATTACK => attack = true,
                    _ => {}
                }
            }
        }
        if attack {
            env += u32::from(r8(c, C_ATTACK));
            if env >= 0xFF {
                env = 0xFF;
                sf -= 1; // to decay
                w8(c, C_STATUS, sf);
            }
        }
        w8(c, C_ENVELOPE_VOLUME, env as u8);
        let level = ((master + 1) * env) >> 4;
        let vol_r = (u32::from(r8(c, C_RIGHT_VOLUME)) * level) >> 8;
        let vol_l = (u32::from(r8(c, C_LEFT_VOLUME)) * level) >> 8;
        w8(c, C_ENVELOPE_VOLUME_RIGHT, vol_r as u8);
        w8(c, C_ENVELOPE_VOLUME_LEFT, vol_l as u8);
        // the mixer uses the byte values
        let vol_r = i32::from(vol_r as u8);
        let vol_l = i32::from(vol_l as u8);

        let out = Out {
            buf,
            vol: (vol_r << 16) + vol_l,
        };
        let kind = r8(c, C_TYPE);
        if kind & (TONEDATA_TYPE_CMP | TONEDATA_TYPE_REV) != 0 {
            mix_special(c, out, n, div_freq, kind);
        } else if kind & TONEDATA_TYPE_FIX != 0 {
            mix_fixed(c, out, n);
        } else {
            mix_resampled(c, out, n, div_freq);
        }
    }
}

/// Where a channel mixes to: this frame's right buffer (the left one follows
/// it at PCM_DMA_BUF_SIZE) and both volumes in one word,
/// `(right << 16) + left`.
#[derive(Clone, Copy)]
struct Out {
    buf: Ptr,
    vol: i32,
}

/// `x` rotated right by `s` bits (0..32). `u32::rotate_right` is Thumb code
/// in ROM, which ARM code in IWRAM cannot inline.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
fn ror(x: u32, s: u32) -> u32 {
    (x >> s) | (x << (32u32.wrapping_sub(s) & 31))
}

/// Mixes `$n` samples (a multiple of 4) into `$out`'s buffers: per output
/// sample, `$sample` gives the value and then `$advance` moves the channel
/// on; `$advance` ends the channel with `break $stop false`. Evaluates to
/// false if the channel stopped; the bytes mixed until then are stored.
///
/// As in the assembly, four output bytes are handled in one word: per
/// sample the word is rotated right a byte and the new value is added to
/// its top byte, which is exactly `byte += (volume * sample) >> 8` modulo 256.
///
/// One multiply gives both products: `p = sample * ((right << 16) + left)`
/// is `R * 0x10000 + L`, with `R = sample * right` and `|L| < 0x8000`, so
/// bits 8-15 of `p` are those of L, and `p + 0x8000 = R * 0x10000 + (L +
/// 0x8000)` without a borrow from R, so its bits 24-31 are bits 8-15 of R.
/// That also leaves a register free, which the loops are short of.
///
/// A macro rather than a function taking a closure: closures and core's
/// generic helpers are compiled as Thumb code in ROM, and the mixer must
/// not call out of IWRAM.
macro_rules! mix_words {
    ($out:expr, $n:expr, $stop:lifetime, $sample:expr, $advance:block) => {{
        let out: Out = $out;
        // the next output byte (right buffer); its low bits count the
        // samples in the current word, which keeps a register free
        let mut q = out.buf;
        let end = q.add($n);
        let (mut r, mut l) = (0u32, 0u32);
        let finished: bool = $stop: {
            while q != end {
                r = q.cast::<u32>().read();
                l = q.add(PCM_DMA_BUF_SIZE).cast::<u32>().read();
                loop {
                    let v: i32 = $sample;
                    let prod = out.vol.wrapping_mul(v) as u32;
                    r = ror(r, 8).wrapping_add(prod.wrapping_add(0x8000) & 0xFF00_0000);
                    l = ror(l, 8).wrapping_add((prod << 16) & 0xFF00_0000);
                    q = q.add(1);
                    $advance
                    if (q as usize) & 3 == 0 {
                        break;
                    }
                }
                q.sub(4).cast::<u32>().write(r);
                q.sub(4).add(PCM_DMA_BUF_SIZE).cast::<u32>().write(l);
            }
            true
        };
        if !finished {
            // store the word, its bytes not reached rotated back into place
            let j = ((q as u32).wrapping_sub(1) & 3) + 1;
            let w = q.sub(j as usize);
            w.cast::<u32>().write(ror(r, 32 - 8 * j));
            w.add(PCM_DMA_BUF_SIZE).cast::<u32>().write(ror(l, 32 - 8 * j));
        }
        finished
    }};
}

/// The channel's loop: its start and length, or a length of 0 if the wave
/// does not loop. Read from memory (volatile) where needed rather than kept
/// in a register through the mixing loops, which are short of registers.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn loop_info() -> (Ptr, i32) {
    unsafe {
        let c = chan();
        if r8(c, C_STATUS) & SF_LOOP == 0 {
            return (core::ptr::null_mut(), 0);
        }
        let wav = rp(c, C_WAV);
        let ls = r32(wav, W_LOOP_START);
        (
            wav.add(W_DATA).add(ls as usize),
            r32(wav, W_SIZE).wrapping_sub(ls) as i32,
        )
    }
}

#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn s8(p: Ptr) -> i32 {
    i32::from(unsafe { read_volatile(p) } as i8)
}

/// TONEDATA_TYPE_FIX: one sample per output sample.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn mix_fixed(c: Ptr, out: Out, n: usize) {
    unsafe {
        let mut count = r32(c, C_COUNT) as i32;
        let mut p = rp(c, C_CURRENT_POINTER);
        let go = mix_words!(out, n, 'stop, s8(p), {
            p = p.add(1);
            count -= 1;
            if count == 0 {
                let (loop_start, loop_len) = loop_info();
                if loop_len == 0 {
                    break 'stop false;
                }
                count = loop_len;
                p = loop_start;
            }
        });
        let c = chan();
        if !go {
            w8(c, C_STATUS, 0);
            return;
        }
        w32(c, C_COUNT, count as u32);
        wp(c, C_CURRENT_POINTER, p);
    }
}

/// Resampled by frequency with linear interpolation (`fw` is the 23-bit
/// fraction of the position).
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn mix_resampled(c: Ptr, out: Out, n: usize, div_freq: u32) {
    unsafe {
        let step = div_freq.wrapping_mul(r32(c, C_FREQUENCY));
        let mut count = r32(c, C_COUNT) as i32;
        let mut fw = r32(c, C_FW);
        // `next` points at the sample after the current one
        let mut next = rp(c, C_CURRENT_POINTER);
        let mut cur = s8(next);
        next = next.add(1);
        let mut delta = s8(next) - cur;
        let go = mix_words!(out, n, 'stop, cur + ((fw as i32).wrapping_mul(delta) >> 23), {
            'adv: {
                fw = fw.wrapping_add(step);
                let mut adv = (fw >> 23) as i32;
                if adv == 0 {
                    break 'adv;
                }
                fw &= !0x3F80_0000;
                count -= adv;
                if count <= 0 {
                    let (loop_start, loop_len) = loop_info();
                    if loop_len == 0 {
                        break 'stop false;
                    }
                    // wrap into the loop by the overshoot
                    next = loop_start;
                    adv = -count;
                    loop {
                        count += loop_len;
                        if count > 0 {
                            break;
                        }
                        adv -= loop_len;
                    }
                    next = next.offset(adv as isize);
                    cur = s8(next);
                } else if adv == 1 {
                    cur += delta;
                } else {
                    next = next.offset((adv - 1) as isize);
                    cur = s8(next);
                }
                next = next.add(1);
                delta = s8(next) - cur;
            }
        });
        let c = chan();
        if !go {
            w8(c, C_STATUS, 0);
            return;
        }
        w32(c, C_FW, fw);
        w32(c, C_COUNT, count as u32);
        wp(c, C_CURRENT_POINTER, next.sub(1));
    }
}

/// `SoundMainRAM_Unk2`: sample `index` of a compressed wave, decoding its
/// 64-sample block into sDecodingBuffer unless this channel already did.
#[inline(never)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn decode(index: u32) -> i32 {
    unsafe {
        let c = chan();
        let block = index >> 6;
        if block != r32(c, C_XPI) {
            w32(c, C_XPI, block);
            let mut src = rp(c, C_WAV).add(W_DATA).add((block * 0x21) as usize);
            let table = (&raw const (*(&raw const crate::data::m4a_tables::gDeltaEncodingTable)
                .cast::<u8>()))
                .cast::<i8>();
            let dst = (&raw mut sDecodingBuffer).cast::<u8>();
            // the first byte is a sample; nibbles of the rest are deltas
            // (the second byte's high nibble is unused)
            let mut acc = read_volatile(src);
            src = src.add(1);
            write_volatile(dst, acc);
            let mut byte = read_volatile(src);
            src = src.add(1);
            acc = acc.wrapping_add(read_volatile(table.add(usize::from(byte & 0xF))) as u8);
            write_volatile(dst.add(1), acc);
            let mut i = 2;
            while i < 0x40 {
                byte = read_volatile(src);
                src = src.add(1);
                acc = acc.wrapping_add(read_volatile(table.add(usize::from(byte >> 4))) as u8);
                write_volatile(dst.add(i), acc);
                acc = acc.wrapping_add(read_volatile(table.add(usize::from(byte & 0xF))) as u8);
                write_volatile(dst.add(i + 1), acc);
                i += 2;
            }
        }
        i32::from(read_volatile(
            (&raw const sDecodingBuffer)
                .cast::<i8>()
                .add((index & 0x3F) as usize),
        ))
    }
}

/// Compressed and/or reversed waves (`SoundMainRAM_Unk1`). For compressed
/// waves the channel's pointer holds a sample index instead of an address.
#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn mix_special(c: Ptr, out: Out, n: usize, div_freq: u32, kind: u8) {
    unsafe {
        let wav = rp(c, C_WAV);
        let compressed = read_volatile(wav.add(W_TYPE).cast::<u16>()) != 0;
        let reverse = kind & TONEDATA_TYPE_REV != 0;
        let mut pos = rp(c, C_CURRENT_POINTER) as u32;
        let sf = r8(c, C_STATUS);
        if sf & SF_SPECIAL == 0 {
            w8(c, C_STATUS, sf | SF_SPECIAL);
            if reverse {
                // mirror the position within the data
                pos = r32(wav, W_SIZE)
                    .wrapping_add((wav as u32) << 1)
                    .wrapping_add(0x20)
                    .wrapping_sub(pos);
                w32(c, C_CURRENT_POINTER, pos);
            }
            if compressed {
                pos = pos.wrapping_sub(wav as u32).wrapping_sub(0x10);
                w32(c, C_CURRENT_POINTER, pos);
            }
        }
        let step = if kind & TONEDATA_TYPE_FIX != 0 {
            0x80_0000
        } else {
            div_freq.wrapping_mul(r32(c, C_FREQUENCY))
        };
        let mut count = r32(c, C_COUNT) as i32;
        let mut fw = r32(c, C_FW);

        if compressed {
            w32(c, C_XPI, 0xFF00_0000);
            if !reverse {
                let mut cur = decode(pos);
                pos = pos.wrapping_add(1);
                let mut delta = decode(pos) - cur;
                let go = mix_words!(out, n, 'stop, cur + ((fw as i32).wrapping_mul(delta) >> 23), {
                    'adv: {
                        fw = fw.wrapping_add(step);
                        let mut adv = (fw >> 23) as i32;
                        if adv == 0 {
                            break 'adv;
                        }
                        fw &= !0x3F80_0000;
                        count -= adv;
                        if count <= 0 {
                            let (_, loop_len) = loop_info();
                            if loop_len == 0 {
                                break 'stop false;
                            }
                            pos = r32(rp(chan(), C_WAV), W_LOOP_START);
                            adv = -count;
                            loop {
                                count += loop_len;
                                if count > 0 {
                                    break;
                                }
                                adv -= loop_len;
                            }
                            pos = pos.wrapping_add(adv as u32);
                            cur = decode(pos);
                        } else if adv == 1 {
                            cur += delta;
                        } else {
                            pos = pos.wrapping_add((adv - 1) as u32);
                            cur = decode(pos);
                        }
                        pos = pos.wrapping_add(1);
                        delta = decode(pos) - cur;
                    }
                });
                let c = chan();
                if !go {
                    stop_special(c);
                    return;
                }
                pos = pos.wrapping_sub(1);
            } else {
                pos = pos.wrapping_sub(1);
                let mut cur = decode(pos);
                pos = pos.wrapping_sub(1);
                let mut delta = decode(pos) - cur;
                let go = mix_words!(out, n, 'stop, cur + ((fw as i32).wrapping_mul(delta) >> 23), {
                    'adv: {
                        fw = fw.wrapping_add(step);
                        let adv = (fw >> 23) as i32;
                        if adv == 0 {
                            break 'adv;
                        }
                        fw &= !0x3F80_0000;
                        count -= adv;
                        if count <= 0 {
                            break 'stop false;
                        }
                        if adv == 1 {
                            cur += delta;
                        } else {
                            pos = pos.wrapping_sub((adv - 1) as u32);
                            cur = decode(pos);
                        }
                        pos = pos.wrapping_sub(1);
                        delta = decode(pos) - cur;
                    }
                });
                let c = chan();
                if !go {
                    stop_special(c);
                    return;
                }
                pos = pos.wrapping_add(2);
            }
        } else if reverse {
            // raw samples played backwards
            let mut p = pos as Ptr;
            p = p.sub(1);
            let mut cur = s8(p);
            let mut delta = s8(p.sub(1)) - cur;
            let go = mix_words!(out, n, 'stop, cur + ((fw as i32).wrapping_mul(delta) >> 23), {
                'adv: {
                    fw = fw.wrapping_add(step);
                    let adv = (fw >> 23) as usize;
                    if adv == 0 {
                        break 'adv;
                    }
                    fw &= !0x3F80_0000;
                    count -= adv as i32;
                    if count <= 0 {
                        break 'stop false;
                    }
                    p = p.sub(adv);
                    cur = s8(p);
                    delta = s8(p.sub(1)) - cur;
                }
            });
            let c = chan();
            if !go {
                stop_special(c);
                return;
            }
            pos = p.add(1) as u32;
        } else {
            // not compressed and not reversed: the original mixes nothing
            return;
        }
        w32(c, C_FW, fw);
        w32(c, C_COUNT, count as u32);
        w32(c, C_CURRENT_POINTER, pos);
    }
}

#[inline(always)]
#[unsafe(link_section = ".iwram_code")]
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn stop_special(c: Ptr) {
    unsafe { w8(c, C_STATUS, 0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The packed multiply in `mix_words` gives, for every volume pair and
    /// sample, the byte `(volume * sample) >> 8` of each side in the top
    /// byte, as two separate multiplies would.
    #[test]
    fn packed_volumes_give_both_products() {
        for right in 0..=255i32 {
            for left in 0..=255i32 {
                let vol = (right << 16) + left;
                for v in -128..=127i32 {
                    let prod = vol.wrapping_mul(v) as u32;
                    assert_eq!(
                        prod.wrapping_add(0x8000) >> 24,
                        ((right * v) >> 8) as u32 & 0xFF
                    );
                    assert_eq!((prod << 16) >> 24, ((left * v) >> 8) as u32 & 0xFF);
                }
            }
        }
    }

    /// Rotating a word a byte per sample and adding to its top byte adds to
    /// each byte in turn, without carries between them.
    #[test]
    fn rotating_adds_touch_one_byte_each() {
        let mut w = 0xFF80_01FEu32;
        for inc in [3u32, 0x80, 0xFF, 1] {
            w = ror(w, 8).wrapping_add(inc << 24);
        }
        assert_eq!(w, 0x007F_8101);
        assert_eq!(ror(0x1234_5678, 32 - 8 * 4), 0x1234_5678);
        assert_eq!(ror(0x1234_5678, 32 - 8), 0x3456_7812);
    }
}
