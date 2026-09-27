use crate::ffi::{GetVarPointer, VarSet, gSpecialVar_Result};
use core::ptr::{addr_of, addr_of_mut};

const VAR_WONDER_NEWS_STEP_COUNTER: u16 = 0x402e;
const FIRST_BERRY_INDEX: u32 = 134;
const MAX_SENT_REWARD: u8 = 4;
const MAX_REWARD: u8 = 5;

const NEWS_REWARD_NONE: u16 = 0;
const NEWS_REWARD_RECV_SMALL: u16 = 1;
const NEWS_REWARD_RECV_BIG: u16 = 2;
const NEWS_REWARD_WAITING: u16 = 3;
const NEWS_REWARD_SENT_SMALL: u16 = 4;
const NEWS_REWARD_SENT_BIG: u16 = 5;
const NEWS_REWARD_AT_MAX: u16 = 6;

#[repr(C)]
struct WonderNewsMetadata {
    counters: u8,
    berry: u8,
}

unsafe extern "C" {

    fn GetSavedWonderNewsMetadata() -> *mut WonderNewsMetadata;
    fn Random() -> u16;
    fn IsMysteryEventEnabled() -> u32;
    fn ValidateSavedWonderNews() -> u32;
}

unsafe fn news_type(data: *const WonderNewsMetadata) -> u8 {
    (unsafe { addr_of!((*data).counters).read() }) & 0x03
}

unsafe fn set_news_type(data: *mut WonderNewsMetadata, value: u8) {
    let counters = unsafe { addr_of!((*data).counters).read() };
    unsafe { addr_of_mut!((*data).counters).write((counters & !0x03) | (value & 0x03)) };
}

unsafe fn sent_reward_counter(data: *const WonderNewsMetadata) -> u8 {
    (unsafe { addr_of!((*data).counters).read() } >> 2) & 0x07
}

unsafe fn set_sent_reward_counter(data: *mut WonderNewsMetadata, value: u8) {
    let counters = unsafe { addr_of!((*data).counters).read() };
    unsafe { addr_of_mut!((*data).counters).write((counters & !(0x07 << 2)) | ((value & 7) << 2)) };
}

unsafe fn reward_counter(data: *const WonderNewsMetadata) -> u8 {
    (unsafe { addr_of!((*data).counters).read() }) >> 5
}

unsafe fn set_reward_counter(data: *mut WonderNewsMetadata, value: u8) {
    let counters = unsafe { addr_of!((*data).counters).read() };
    unsafe { addr_of_mut!((*data).counters).write((counters & 0x1f) | ((value & 7) << 5)) };
}

unsafe fn increment_reward_counter(data: *mut WonderNewsMetadata) {
    unsafe { set_reward_counter(data, reward_counter(data).saturating_add(1).min(MAX_REWARD)) };
}

unsafe fn increment_sent_reward_counter(data: *mut WonderNewsMetadata) {
    unsafe {
        set_sent_reward_counter(
            data,
            sent_reward_counter(data)
                .saturating_add(1)
                .min(MAX_SENT_REWARD),
        )
    };
}

unsafe fn reward_type(data: *const WonderNewsMetadata) -> u16 {
    if unsafe { reward_counter(data) } == MAX_REWARD {
        return NEWS_REWARD_AT_MAX;
    }
    match unsafe { news_type(data) } {
        0 => NEWS_REWARD_WAITING,
        1 => NEWS_REWARD_RECV_SMALL,
        2 => NEWS_REWARD_RECV_BIG,
        3 if unsafe { sent_reward_counter(data) } < MAX_SENT_REWARD - 1 => NEWS_REWARD_SENT_SMALL,
        3 => NEWS_REWARD_SENT_BIG,
        _ => NEWS_REWARD_NONE,
    }
}

unsafe fn take_reward_item(data: *mut WonderNewsMetadata) -> u32 {
    unsafe { set_news_type(data, 0) };
    let item = u32::from(unsafe { addr_of!((*data).berry).read() }) + FIRST_BERRY_INDEX - 1;
    unsafe { addr_of_mut!((*data).berry).write(0) };
    unsafe { increment_reward_counter(data) };
    item
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_SetReward(value: u32) {
    let data = unsafe { GetSavedWonderNewsMetadata() };
    unsafe { set_news_type(data, value as u8) };
    let berry = match value {
        1 | 2 => (unsafe { Random() } % 15) as u8 + 16,
        3 => (unsafe { Random() } % 15) as u8 + 1,
        _ => return,
    };
    unsafe { addr_of_mut!((*data).berry).write(berry) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_Reset() {
    let data = unsafe { GetSavedWonderNewsMetadata() };
    unsafe { addr_of_mut!((*data).counters).write(0) };
    unsafe { addr_of_mut!((*data).berry).write(0) };
    let _ = unsafe { VarSet(VAR_WONDER_NEWS_STEP_COUNTER, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_IncrementStepCounter() {
    let step_counter = unsafe { GetVarPointer(VAR_WONDER_NEWS_STEP_COUNTER) };
    let data = unsafe { GetSavedWonderNewsMetadata() };
    if unsafe { reward_counter(data) } >= MAX_REWARD {
        let steps = unsafe { step_counter.read() }.wrapping_add(1);
        unsafe { step_counter.write(steps) };
        if steps >= 500 {
            unsafe { set_reward_counter(data, 0) };
            unsafe { step_counter.write(0) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonderNews_GetRewardInfo() -> u16 {
    if unsafe { IsMysteryEventEnabled() } == 0 || unsafe { ValidateSavedWonderNews() } == 0 {
        return NEWS_REWARD_NONE;
    }

    let data = unsafe { GetSavedWonderNewsMetadata() };
    let kind = unsafe { reward_type(data) };
    match kind {
        NEWS_REWARD_RECV_SMALL | NEWS_REWARD_RECV_BIG => {
            unsafe { addr_of_mut!(gSpecialVar_Result).write(take_reward_item(data) as u16) };
        }
        NEWS_REWARD_SENT_SMALL => {
            unsafe { addr_of_mut!(gSpecialVar_Result).write(take_reward_item(data) as u16) };
            unsafe { increment_sent_reward_counter(data) };
        }
        NEWS_REWARD_SENT_BIG => {
            unsafe { addr_of_mut!(gSpecialVar_Result).write(take_reward_item(data) as u16) };
            unsafe { set_sent_reward_counter(data, 0) };
        }
        _ => {}
    }
    kind
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_matches_the_packed_c_bitfields() {
        assert_eq!(core::mem::size_of::<WonderNewsMetadata>(), 2);
        let mut data = WonderNewsMetadata {
            counters: 0,
            berry: 0,
        };
        unsafe {
            set_news_type(&raw mut data, 3);
            set_sent_reward_counter(&raw mut data, 4);
            set_reward_counter(&raw mut data, 5);
        }
        assert_eq!(data.counters, 0b1011_0011);
    }
}
