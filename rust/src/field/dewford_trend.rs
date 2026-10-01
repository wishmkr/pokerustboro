//! The Dewford Trend: a pair of Easy Chat words repeated around Dewford
//! Hall. Up to five submitted phrases are kept, sorted by "trendiness",
//! which rises and falls a little every day. See the original file's
//! header comment for the whole design.

use crate::Random;
use crate::event_data::{FlagGet, FlagSet};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_Result, gStringVar1};
use crate::load_save::gSaveBlock1Ptr;
use crate::malloc::{Alloc, Free};

const SAVED_TRENDS_COUNT: usize = 5;
const SB1_DEWFORD_TRENDS: usize = 0x2e68;
const EC_GROUP_CONDITIONS: u16 = 0x0a;
const EC_GROUP_LIFESTYLE: u16 = 0x0c;
const EC_GROUP_HOBBIES: u16 = 0x0d;
const FLAG_SYS_CHANGED_DEWFORD_TREND: u16 = 0x893;
const FLAG_SYS_MIX_RECORD: u16 = 0x894;
/// `max(sizeof(trend) * 5 * MAX_LINK_PLAYERS, 0x100)`
const BUFFER_SIZE: u32 = 0x100;

const SORT_MODE_NORMAL: u8 = 0;
const SORT_MODE_MAX_FIRST: u8 = 1;
const SORT_MODE_FULL: u8 = 2;

/// `struct DewfordTrend`: `trendiness:7`, `maxTrendiness:7`,
/// `gainingTrendiness:1` in one halfword, then `rand` and two words.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DewfordTrend {
    bits: u16,
    rand: u16,
    words: [u16; 2],
}

impl DewfordTrend {
    fn trendiness(&self) -> u16 {
        self.bits & 0x7f
    }
    fn max_trendiness(&self) -> u16 {
        (self.bits >> 7) & 0x7f
    }
    fn gaining(&self) -> bool {
        self.bits & (1 << 14) != 0
    }
    fn set_trendiness(&mut self, value: u32) {
        self.bits = (self.bits & !0x7f) | (value as u16 & 0x7f);
    }
    fn set_max_trendiness(&mut self, value: u32) {
        self.bits = (self.bits & !(0x7f << 7)) | ((value as u16 & 0x7f) << 7);
    }
    fn set_gaining(&mut self, value: u32) {
        self.bits = (self.bits & !(1 << 14)) | ((value as u16 & 1) << 14);
    }
}

/// `GetRandomEasyChatWordFromGroup` with this module's view of its types.
#[inline]
unsafe fn GetRandomEasyChatWordFromGroup(a0: u16) -> u16 {
    unsafe { crate::easy_chat::GetRandomEasyChatWordFromGroup(a0) }
}
/// `TryPutTrendWatcherOnAir` with this module's view of its types.
#[inline]
unsafe fn TryPutTrendWatcherOnAir(a0: *const u16) {
    unsafe {
        crate::tv::TryPutTrendWatcherOnAir(a0 as _);
    }
}
/// `ConvertEasyChatWordsToString` with this module's view of its types.
#[inline]
unsafe fn ConvertEasyChatWordsToString(a0: *mut u8, a1: *const u16, a2: u16, a3: u16) -> *mut u8 {
    unsafe { crate::easy_chat::ConvertEasyChatWordsToString(a0 as _, a1 as _, a2, a3) as *mut u8 }
}
/// `GetLinkPlayerCount` with this module's view of its types.
#[inline]
unsafe fn GetLinkPlayerCount() -> u8 {
    unsafe { crate::link::GetLinkPlayerCount() }
}

#[inline]
unsafe fn saved_trends() -> *mut DewfordTrend {
    unsafe {
        (&raw const gSaveBlock1Ptr)
            .read()
            .cast::<u8>()
            .add(SB1_DEWFORD_TRENDS)
            .cast()
    }
}

#[inline]
fn random_bit() -> u32 {
    u32::from(Random() & 1)
}

#[unsafe(no_mangle)]
pub unsafe fn InitDewfordTrend() {
    let trends = unsafe { saved_trends() };
    for i in 0..SAVED_TRENDS_COUNT {
        let trend = unsafe { &mut *trends.add(i) };
        trend.words[0] = unsafe { GetRandomEasyChatWordFromGroup(EC_GROUP_CONDITIONS) };
        let group = if random_bit() != 0 {
            EC_GROUP_LIFESTYLE
        } else {
            EC_GROUP_HOBBIES
        };
        trend.words[1] = unsafe { GetRandomEasyChatWordFromGroup(group) };
        trend.set_gaining(random_bit());
        seed_trend_rng(trend);
    }
    unsafe { sort_trends(trends, SAVED_TRENDS_COUNT as u16, SORT_MODE_NORMAL) };
}

#[unsafe(no_mangle)]
pub unsafe fn UpdateDewfordTrendPerDay(days: u16) {
    if days == 0 {
        return;
    }
    let clock_rand = u32::from(days) * 5;
    let trends = unsafe { saved_trends() };
    for i in 0..SAVED_TRENDS_COUNT {
        let trend = unsafe { &mut *trends.add(i) };
        let mut rand = clock_rand;
        if !trend.gaining() {
            // A "boring" trend loses trendiness until it reaches 0.
            if trend.trendiness() >= rand as u16 {
                trend.set_trendiness(u32::from(trend.trendiness()).wrapping_sub(rand));
                if trend.trendiness() == 0 {
                    trend.set_gaining(1);
                }
                continue;
            }
            rand = rand.wrapping_sub(u32::from(trend.trendiness()));
            trend.set_trendiness(0);
            trend.set_gaining(1);
        }
        let trendiness = u32::from(trend.trendiness()).wrapping_add(rand);
        let max = u32::from(trend.max_trendiness());
        if trendiness as u16 > trend.max_trendiness() {
            // Reached the limit: bounce back and forth within it.
            let new_trendiness = trendiness.checked_rem(max).unwrap_or(0);
            let laps = trendiness.checked_div(max).unwrap_or(0);
            trend.set_gaining(laps ^ 1);
            if trend.gaining() {
                trend.set_trendiness(new_trendiness);
            } else {
                trend.set_trendiness(max.wrapping_sub(new_trendiness));
            }
        } else {
            trend.set_trendiness(trendiness);
            // At its peak the trend becomes boring and starts to fall.
            if trend.trendiness() == trend.max_trendiness() {
                trend.set_gaining(0);
            }
        }
    }
    unsafe { sort_trends(trends, SAVED_TRENDS_COUNT as u16, SORT_MODE_NORMAL) };
}

/// Saves the phrase among the trends, and says whether it became the
/// current trendy phrase.
#[unsafe(no_mangle)]
pub unsafe fn TrySetTrendyPhrase(phrase: *const u16) -> u8 {
    if unsafe { is_phrase_in_saved_trends(phrase) } {
        return 0;
    }
    let trends = unsafe { saved_trends() };
    let words = unsafe { [phrase.read(), phrase.add(1).read()] };
    if unsafe { FlagGet(FLAG_SYS_CHANGED_DEWFORD_TREND) } == 0 {
        unsafe { FlagSet(FLAG_SYS_CHANGED_DEWFORD_TREND) };
        // The very first submission (and no phrases received by mixing
        // records) just replaces the words.
        if unsafe { FlagGet(FLAG_SYS_MIX_RECORD) } == 0 {
            unsafe { (*trends).words = words };
            return 1;
        }
    }

    let mut trend = DewfordTrend {
        words,
        ..Default::default()
    };
    trend.set_gaining(1);
    seed_trend_rng(&mut trend);
    for i in 0..SAVED_TRENDS_COUNT {
        if unsafe { compare_trends(&trend, &*trends.add(i), SORT_MODE_NORMAL) } {
            // Trendier than trend i: shift the rest down and insert it.
            let mut j = SAVED_TRENDS_COUNT - 1;
            while j > i {
                unsafe { trends.add(j).write(trends.add(j - 1).read()) };
                j -= 1;
            }
            unsafe { trends.add(i).write(trend) };
            if i == SAVED_TRENDS_COUNT - 1 {
                unsafe { TryPutTrendWatcherOnAir(phrase) };
            }
            return u8::from(i == 0);
        }
    }
    unsafe { trends.add(SAVED_TRENDS_COUNT - 1).write(trend) };
    unsafe { TryPutTrendWatcherOnAir(phrase) };
    0
}

unsafe fn sort_trends(trends: *mut DewfordTrend, count: u16, mode: u8) {
    for i in 0..usize::from(count) {
        for j in i + 1..usize::from(count) {
            let (a, b) = unsafe { (trends.add(j).read(), trends.add(i).read()) };
            if compare_trends(&a, &b, mode) {
                unsafe { trends.add(j).write(b) };
                unsafe { trends.add(i).write(a) };
            }
        }
    }
}

/// Merges the trends received by record mixing with our own.
#[unsafe(no_mangle)]
pub unsafe fn ReceiveDewfordTrendData(linked_trends: *const u8, size: usize, _unused: u8) {
    let linked = unsafe { Alloc(BUFFER_SIZE) }.cast::<DewfordTrend>();
    if linked.is_null() {
        return;
    }
    let saved = unsafe { Alloc(BUFFER_SIZE) }.cast::<DewfordTrend>();
    if saved.is_null() {
        unsafe { Free(linked.cast()) };
        return;
    }

    let players = usize::from(unsafe { GetLinkPlayerCount() });
    for i in 0..players {
        unsafe {
            core::ptr::copy_nonoverlapping(
                linked_trends.add(i * size),
                linked.add(i * SAVED_TRENDS_COUNT).cast::<u8>(),
                SAVED_TRENDS_COUNT * core::mem::size_of::<DewfordTrend>(),
            )
        };
    }

    // Keep each distinct phrase once, preferring the trendier copy.
    let mut num_trends = 0usize;
    for k in 0..players * SAVED_TRENDS_COUNT {
        let src = unsafe { linked.add(k).read() };
        match unsafe { saved_trend_index(saved, &src, num_trends) } {
            None => {
                unsafe { saved.add(num_trends).write(src) };
                num_trends += 1;
            }
            Some(index) => {
                let existing = unsafe { &mut *saved.add(index) };
                if existing.trendiness() < src.trendiness() {
                    *existing = src;
                }
            }
        }
    }
    unsafe { sort_trends(saved, num_trends as u16, SORT_MODE_FULL) };

    let trends = unsafe { saved_trends() };
    unsafe { core::ptr::copy_nonoverlapping(saved, trends, SAVED_TRENDS_COUNT) };
    unsafe { Free(linked.cast()) };
    unsafe { Free(saved.cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn BufferTrendyPhraseString() {
    let index = usize::from(unsafe { (&raw const gSpecialVar_0x8004).read() });
    let trend = unsafe { saved_trends().add(index) };
    unsafe {
        ConvertEasyChatWordsToString(
            (&raw mut gStringVar1).cast(),
            (&raw const (*trend).words).cast(),
            2,
            1,
        )
    };
}

/// Whether the current phrase is "boring" (only changes an NPC's comment).
#[unsafe(no_mangle)]
pub unsafe fn IsTrendyPhraseBoring() {
    let trends = unsafe { saved_trends() };
    let (first, second) = unsafe { (trends.read(), trends.add(1).read()) };
    let boring = i32::from(first.trendiness()) - i32::from(second.trendiness()) <= 1
        && !first.gaining()
        && second.gaining();
    unsafe { (&raw mut gSpecialVar_Result).write(u16::from(boring)) };
}

/// The Dewford Hall painting's title depends on the current phrase.
#[unsafe(no_mangle)]
pub unsafe fn GetDewfordHallPaintingNameIndex() {
    let words = unsafe { (*saved_trends()).words };
    unsafe { (&raw mut gSpecialVar_Result).write(words[0].wrapping_add(words[1]) & 7) };
}

/// Whether `a` is trendier than `b`. Ties are broken at random, except in
/// the full mode, which compares everything and then says TRUE.
fn compare_trends(a: &DewfordTrend, b: &DewfordTrend, mode: u8) -> bool {
    use core::cmp::Ordering::{Equal, Greater, Less};
    let keys: &[(u16, u16)] = match mode {
        SORT_MODE_NORMAL => &[
            (a.trendiness(), b.trendiness()),
            (a.max_trendiness(), b.max_trendiness()),
        ],
        SORT_MODE_MAX_FIRST => &[
            (a.max_trendiness(), b.max_trendiness()),
            (a.trendiness(), b.trendiness()),
        ],
        SORT_MODE_FULL => &[
            (a.trendiness(), b.trendiness()),
            (a.max_trendiness(), b.max_trendiness()),
            (a.rand, b.rand),
            (a.words[0], b.words[0]),
            (a.words[1], b.words[1]),
        ],
        _ => &[],
    };
    for (x, y) in keys {
        match x.cmp(y) {
            Greater => return true,
            Less => return false,
            Equal => {}
        }
    }
    if mode == SORT_MODE_FULL {
        return true;
    }
    random_bit() != 0
}

fn seed_trend_rng(trend: &mut DewfordTrend) {
    let mut rand = Random() % 98;
    if rand > 50 {
        rand = Random() % 98;
        if rand > 80 {
            rand = Random() % 98;
        }
    }
    trend.set_max_trendiness(u32::from(rand) + 30);
    let roll = Random() % (rand + 1);
    trend.set_trendiness(u32::from(roll) + 30);
    trend.rand = Random();
}

unsafe fn is_phrase_in_saved_trends(phrase: *const u16) -> bool {
    let words = unsafe { [phrase.read(), phrase.add(1).read()] };
    let trends = unsafe { saved_trends() };
    (0..SAVED_TRENDS_COUNT).any(|i| unsafe { (*trends.add(i)).words } == words)
}

unsafe fn saved_trend_index(
    saved: *const DewfordTrend,
    trend: &DewfordTrend,
    count: usize,
) -> Option<usize> {
    (0..count).find(|&i| unsafe { (*saved.add(i)).words } == trend.words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitfields_pack_like_gcc() {
        let mut t = DewfordTrend::default();
        t.set_trendiness(0x7f);
        assert_eq!(t.bits, 0x007f);
        t.set_max_trendiness(0x7f);
        assert_eq!(t.bits, 0x3fff);
        t.set_gaining(1);
        assert_eq!(t.bits, 0x7fff);
        assert_eq!(core::mem::size_of::<DewfordTrend>(), 8);
    }

    #[test]
    fn full_mode_prefers_the_first_on_a_tie() {
        let a = DewfordTrend::default();
        assert!(compare_trends(&a, &a, SORT_MODE_FULL));
    }
}
