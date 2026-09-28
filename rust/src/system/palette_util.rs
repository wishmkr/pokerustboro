//! Translated from `src/palette_util.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Reset(flash: *mut RouletteFlashUtil) {
    (*flash).enabled = 0;
    (*flash).flags = 0;
    memset(&raw mut (*flash).palettes as *mut u8, 0, 192);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Add(
    flash: *mut RouletteFlashUtil,
    id: u8,
    settings: *mut RouletteFlashSettings,
) -> u8 {
    if id >= 16 || (*flash).palettes[id].available() != 0 {
        return 0xFF;
    }
    (*flash).palettes[id].settings.color = (*settings).color;
    (*flash).palettes[id].settings.paletteOffset = (*settings).paletteOffset;
    (*flash).palettes[id].settings.numColors = (*settings).numColors;
    (*flash).palettes[id].settings.delay = (*settings).delay;
    (*flash).palettes[id].settings.unk6 = (*settings).unk6;
    (*flash).palettes[id]
        .settings
        .set_numFadeCycles((*settings).numFadeCycles());
    (*flash).palettes[id]
        .settings
        .set_unk7_5((*settings).unk7_5());
    (*flash).palettes[id]
        .settings
        .set_colorDeltaDir((*settings).colorDeltaDir());
    (*flash).palettes[id].set_state(0);
    (*flash).palettes[id].set_available(TRUE);
    (*flash).palettes[id].fadeCycleCounter = 0;
    (*flash).palettes[id].delayCounter = 0;
    if (*flash).palettes[id].settings.colorDeltaDir() < 0 {
        (*flash).palettes[id].colorDelta = -1;
    } else {
        (*flash).palettes[id].colorDelta = 1;
    }
    return id;
}
pub(crate) unsafe extern "C" fn RouletteFlash_Remove(flash: *mut RouletteFlashUtil, id: u8) -> u8 {
    if id >= 16 {
        return 0xFF;
    }
    if (*flash).palettes[id].available() == 0 {
        return 0xFF;
    }
    memset(&raw mut (*flash).palettes[id] as *mut u8, 0, 12);
    return id;
}
pub(crate) unsafe extern "C" fn RouletteFlash_FadePalette(pal: *mut RouletteFlashPalette) -> u8 {
    let mut i: u8 = 0;
    let mut returnval: u8 = 0;
    i = 0;
    while i < (*pal).settings.numColors {
        let mut faded: *mut PlttData = &raw mut gPlttBufferFaded
            [(*pal).settings.paletteOffset as i32 + i as i32]
            as *mut PlttData;
        let mut unfaded: *mut PlttData = &raw mut gPlttBufferUnfaded
            [(*pal).settings.paletteOffset as i32 + i as i32]
            as *mut PlttData;
        match (*pal).state() {
            1 => {
                if (*faded).r() as i32 + (*pal).colorDelta as i32 >= 0
                    && ((*faded).r() as i32 + (*pal).colorDelta as i32) < 32
                {
                    (*faded).set_r((*faded).r() + (*pal).colorDelta as u16);
                }
                if (*faded).g() as i32 + (*pal).colorDelta as i32 >= 0
                    && ((*faded).g() as i32 + (*pal).colorDelta as i32) < 32
                {
                    (*faded).set_g((*faded).g() + (*pal).colorDelta as u16);
                }
                if (*faded).b() as i32 + (*pal).colorDelta as i32 >= 0
                    && ((*faded).b() as i32 + (*pal).colorDelta as i32) < 32
                {
                    (*faded).set_b((*faded).b() + (*pal).colorDelta as u16);
                }
            }
            2 => {
                if (*pal).colorDelta < 0 {
                    if (*faded).r() as i32 + (*pal).colorDelta as i32 >= (*unfaded).r() as i32 {
                        (*faded).set_r((*faded).r() + (*pal).colorDelta as u16);
                    }
                    if (*faded).g() as i32 + (*pal).colorDelta as i32 >= (*unfaded).g() as i32 {
                        (*faded).set_g((*faded).g() + (*pal).colorDelta as u16);
                    }
                    if (*faded).b() as i32 + (*pal).colorDelta as i32 >= (*unfaded).b() as i32 {
                        (*faded).set_b((*faded).b() + (*pal).colorDelta as u16);
                    }
                } else {
                    if (*faded).r() as i32 + (*pal).colorDelta as i32 <= (*unfaded).r() as i32 {
                        (*faded).set_r((*faded).r() + (*pal).colorDelta as u16);
                    }
                    if (*faded).g() as i32 + (*pal).colorDelta as i32 <= (*unfaded).g() as i32 {
                        (*faded).set_g((*faded).g() + (*pal).colorDelta as u16);
                    }
                    if (*faded).b() as i32 + (*pal).colorDelta as i32 <= (*unfaded).b() as i32 {
                        (*faded).set_b((*faded).b() + (*pal).colorDelta as u16);
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    if ({
        let t1 = (*pal).fadeCycleCounter;
        (*pal).fadeCycleCounter += 1;
        t1
    }) as u32
        != (*pal).settings.numFadeCycles() as u32
    {
        returnval = 0;
    } else {
        (*pal).fadeCycleCounter = 0;
        (*pal).colorDelta *= -1;
        if (*pal).state() == 1 {
            (*pal).set_state((*pal).state() + 1);
        } else {
            (*pal).set_state((*pal).state() - 1);
        }
        returnval = 1;
    }
    return returnval;
}
pub(crate) unsafe extern "C" fn RouletteFlash_FlashPalette(pal: *mut RouletteFlashPalette) -> u8 {
    let mut i: u8 = 0;
    match (*pal).state() {
        1 => {
            while i < (*pal).settings.numColors {
                gPlttBufferFaded[(*pal).settings.paletteOffset as i32 + i as i32] =
                    (*pal).settings.color;
                i += 1;
            }
            (*pal).set_state((*pal).state() + 1);
        }
        2 => {
            while i < (*pal).settings.numColors {
                gPlttBufferFaded[(*pal).settings.paletteOffset as i32 + i as i32] =
                    gPlttBufferUnfaded[(*pal).settings.paletteOffset as i32 + i as i32];
                i += 1;
            }
            (*pal).set_state((*pal).state() - 1);
        }
        _ => {}
    }
    return 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Run(flash: *mut RouletteFlashUtil) {
    let mut i: u8 = 0;
    if (*flash).enabled != 0 {
        i = 0;
        while i < 16 {
            if shr_i32((*flash).flags as i32, i as u32) & 1 != 0 {
                if ({
                    (*flash).palettes[i].delayCounter -= 1;
                    (*flash).palettes[i].delayCounter
                }) == 255
                {
                    if (*flash).palettes[i].settings.color as i32 & FLASHUTIL_USE_EXISTING_COLOR
                        != 0
                    {
                        RouletteFlash_FadePalette(&raw mut (*flash).palettes[i]);
                    } else {
                        RouletteFlash_FlashPalette(&raw mut (*flash).palettes[i]);
                    }
                    (*flash).palettes[i].delayCounter = (*flash).palettes[i].settings.delay;
                }
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Enable(flash: *mut RouletteFlashUtil, flags: u16) {
    let mut i: u8 = 0;
    (*flash).enabled += 1;
    i = 0;
    while i < 16 {
        if shr_i32(flags as i32, i as u32) & 1 != 0 {
            if (*flash).palettes[i].available() != 0 {
                (*flash).flags |= shl_i32(1, i as u32) as u16;
                (*flash).palettes[i].set_state(1);
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Stop(flash: *mut RouletteFlashUtil, flags: u16) {
    let mut i: u8 = 0;
    i = 0;
    while i < 16 {
        if shr_i32((*flash).flags as i32, i as u32) & 1 != 0 {
            if (*flash).palettes[i].available() != 0 {
                if shr_i32(flags as i32, i as u32) & 1 != 0 {
                    let mut offset: u32 = (*flash).palettes[i].settings.paletteOffset as u32;
                    let mut faded: *mut u16 = &raw mut gPlttBufferFaded[offset];
                    let mut unfaded: *mut u16 = &raw mut gPlttBufferUnfaded[offset];
                    memcpy(
                        faded as *mut u8,
                        unfaded as *mut u8,
                        (*flash).palettes[i].settings.numColors as u32 * 2,
                    );
                    (*flash).palettes[i].set_state(0);
                    (*flash).palettes[i].fadeCycleCounter = 0;
                    (*flash).palettes[i].delayCounter = 0;
                    if (*flash).palettes[i].settings.colorDeltaDir() < 0 {
                        (*flash).palettes[i].colorDelta = -1;
                    } else {
                        (*flash).palettes[i].colorDelta = 1;
                    }
                }
            }
        }
        i += 1;
    }
    if flags == 0xFFFF {
        (*flash).enabled = 0;
        (*flash).flags = 0;
    } else {
        (*flash).flags &= !flags;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPulseBlend(pulseBlend: *mut PulseBlend) {
    let mut i: u8 = 0;
    (*pulseBlend).usedPulseBlendPalettes = 0;
    memset(&raw mut (*pulseBlend).pulseBlendPalettes as *mut u8, 0, 192);
    while i < 16 {
        (*pulseBlend).pulseBlendPalettes[i].paletteSelector = i;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPulseBlendPaletteSettings(
    pulseBlend: *mut PulseBlend,
    settings: *mut PulseBlendSettings,
) -> i32 {
    let mut i: u8 = 0;
    let mut pulseBlendPalette: *mut PulseBlendPalette = null_mut();
    if (*pulseBlend).pulseBlendPalettes[0].inUse() == 0 {
        pulseBlendPalette = &raw mut (*pulseBlend).pulseBlendPalettes[0];
    } else {
        while ({
            i += 1;
            i
        }) < 16
        {
            if (*pulseBlend).pulseBlendPalettes[i].inUse() == 0 {
                pulseBlendPalette = &raw mut (*pulseBlend).pulseBlendPalettes[i];
                break;
            }
        }
    }
    if pulseBlendPalette.is_null() {
        return 0xFF;
    }
    (*pulseBlendPalette).set_blendCoeff(0);
    (*pulseBlendPalette).set_fadeDirection(0);
    (*pulseBlendPalette).set_available(1);
    (*pulseBlendPalette).set_inUse(1);
    (*pulseBlendPalette).delayCounter = 0;
    (*pulseBlendPalette).fadeCycleCounter = 0;
    memcpy(
        &raw mut (*pulseBlendPalette).pulseBlendSettings as *mut u8,
        settings as *mut u8,
        8,
    );
    return i as i32;
}
pub(crate) unsafe extern "C" fn ClearPulseBlendPalettesSettings(
    pulseBlendPalette: *mut PulseBlendPalette,
) {
    let mut i: u16 = 0;
    if (*pulseBlendPalette).available() == 0
        && (*pulseBlendPalette)
            .pulseBlendSettings
            .restorePaletteOnUnload()
            != 0
    {
        i = (*pulseBlendPalette).pulseBlendSettings.paletteOffset;
        while (i as i32)
            < (*pulseBlendPalette).pulseBlendSettings.paletteOffset as i32
                + (*pulseBlendPalette).pulseBlendSettings.numColors as i32
        {
            gPlttBufferFaded[i] = gPlttBufferUnfaded[i];
            i += 1;
        }
    }
    memset(
        &raw mut (*pulseBlendPalette).pulseBlendSettings as *mut u8,
        0,
        8,
    );
    (*pulseBlendPalette).set_blendCoeff(0);
    (*pulseBlendPalette).set_fadeDirection(0);
    (*pulseBlendPalette).set_unk1_5(0);
    (*pulseBlendPalette).set_available(1);
    (*pulseBlendPalette).set_inUse(0);
    (*pulseBlendPalette).fadeCycleCounter = 0;
    (*pulseBlendPalette).delayCounter = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnloadUsedPulseBlendPalettes(
    pulseBlend: *mut PulseBlend,
    mut pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    let mut i: u16 = 0;
    if multiSelection == 0 {
        ClearPulseBlendPalettesSettings(
            &raw mut (*pulseBlend).pulseBlendPalettes[pulseBlendPaletteSelector as i32 & 0xF],
        );
    } else {
        i = 0;
        while i < 16 {
            if pulseBlendPaletteSelector as i32 & 1 != 0
                && (*pulseBlend).pulseBlendPalettes[i].inUse() != 0
            {
                ClearPulseBlendPalettesSettings(&raw mut (*pulseBlend).pulseBlendPalettes[i]);
            }
            pulseBlendPaletteSelector >>= 1;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkUsedPulseBlendPalettes(
    pulseBlend: *mut PulseBlend,
    mut pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    let mut i: u8 = 0;
    if multiSelection == 0 {
        i = pulseBlendPaletteSelector as u8 & 0xF;
        (*pulseBlend).pulseBlendPalettes[i].set_available(0);
        (*pulseBlend).usedPulseBlendPalettes |= shl_i32(1, i as u32) as u16;
    } else {
        i = 0;
        while i < 16 {
            if pulseBlendPaletteSelector as i32 & 1 == 0
                || (*pulseBlend).pulseBlendPalettes[i].inUse() == 0
                || (*pulseBlend).pulseBlendPalettes[i].available() == 0
            {
                pulseBlendPaletteSelector <<= 1;
            } else {
                (*pulseBlend).pulseBlendPalettes[i].set_available(0);
                (*pulseBlend).usedPulseBlendPalettes |= shl_i32(1, i as u32) as u16;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnmarkUsedPulseBlendPalettes(
    pulseBlend: *mut PulseBlend,
    mut pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    let mut i: u16 = 0;
    let mut pulseBlendPalette: *mut PulseBlendPalette = null_mut();
    let mut j: u8 = 0;
    if multiSelection == 0 {
        pulseBlendPalette =
            &raw mut (*pulseBlend).pulseBlendPalettes[pulseBlendPaletteSelector as i32 & 0xF];
        if (*pulseBlendPalette).available() == 0 && (*pulseBlendPalette).inUse() != 0 {
            if (*pulseBlendPalette)
                .pulseBlendSettings
                .restorePaletteOnUnload()
                != 0
            {
                i = (*pulseBlendPalette).pulseBlendSettings.paletteOffset;
                while (i as i32)
                    < (*pulseBlendPalette).pulseBlendSettings.paletteOffset as i32
                        + (*pulseBlendPalette).pulseBlendSettings.numColors as i32
                {
                    gPlttBufferFaded[i] = gPlttBufferUnfaded[i];
                    i += 1;
                }
            }
            (*pulseBlendPalette).set_available(1);
            (*pulseBlend).usedPulseBlendPalettes &= !(shl_i32(1, j as u32) as u16);
        }
    } else {
        j = 0;
        while j < 16 {
            pulseBlendPalette = &raw mut (*pulseBlend).pulseBlendPalettes[j];
            if pulseBlendPaletteSelector as i32 & 1 == 0
                || (*pulseBlendPalette).available() != 0
                || (*pulseBlendPalette).inUse() == 0
            {
                pulseBlendPaletteSelector <<= 1;
            } else {
                if (*pulseBlendPalette)
                    .pulseBlendSettings
                    .restorePaletteOnUnload()
                    != 0
                {
                    i = (*pulseBlendPalette).pulseBlendSettings.paletteOffset;
                    while (i as i32)
                        < (*pulseBlendPalette).pulseBlendSettings.paletteOffset as i32
                            + (*pulseBlendPalette).pulseBlendSettings.numColors as i32
                    {
                        gPlttBufferFaded[i] = gPlttBufferUnfaded[i];
                        i += 1;
                    }
                }
                (*pulseBlendPalette).set_available(1);
                (*pulseBlend).usedPulseBlendPalettes &= !(shl_i32(1, j as u32) as u16);
            }
            j += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePulseBlend(pulseBlend: *mut PulseBlend) {
    let mut pulseBlendPalette: *mut PulseBlendPalette = null_mut();
    let mut i: u8 = 0;
    if (*pulseBlend).usedPulseBlendPalettes != 0 {
        i = 0;
        while i < 16 {
            pulseBlendPalette = &raw mut (*pulseBlend).pulseBlendPalettes[i];
            if (*pulseBlendPalette).available() == 0
                && (*pulseBlendPalette).inUse() != 0
                && (gPaletteFade.active() == 0
                    || (*pulseBlendPalette).pulseBlendSettings.unk7_7() == 0)
            {
                if ({
                    (*pulseBlendPalette).delayCounter -= 1;
                    (*pulseBlendPalette).delayCounter
                }) == 0xFF
                {
                    (*pulseBlendPalette).delayCounter =
                        (*pulseBlendPalette).pulseBlendSettings.delay;
                    BlendPalette(
                        (*pulseBlendPalette).pulseBlendSettings.paletteOffset,
                        (*pulseBlendPalette).pulseBlendSettings.numColors as u16,
                        (*pulseBlendPalette).blendCoeff(),
                        (*pulseBlendPalette).pulseBlendSettings.blendColor,
                    );
                    match (*pulseBlendPalette).pulseBlendSettings.fadeType() {
                        0 => {
                            if ({
                                let t2 = (*pulseBlendPalette).blendCoeff();
                                (*pulseBlendPalette)
                                    .set_blendCoeff((*pulseBlendPalette).blendCoeff() + 1);
                                t2
                            }) as i32
                                == (*pulseBlendPalette).pulseBlendSettings.maxBlendCoeff() as i32
                            {
                                (*pulseBlendPalette).fadeCycleCounter += 1;
                                (*pulseBlendPalette).set_blendCoeff(0);
                            }
                        }
                        1 => {
                            if (*pulseBlendPalette).fadeDirection() != 0 {
                                if ({
                                    (*pulseBlendPalette)
                                        .set_blendCoeff((*pulseBlendPalette).blendCoeff() - 1);
                                    (*pulseBlendPalette).blendCoeff()
                                }) == 0
                                {
                                    (*pulseBlendPalette).fadeCycleCounter += 1;
                                    (*pulseBlendPalette).set_fadeDirection(
                                        (*pulseBlendPalette).fadeDirection() ^ 1,
                                    );
                                }
                            } else {
                                let mut max: u8 =
                                    (*pulseBlendPalette).pulseBlendSettings.maxBlendCoeff() as u8
                                        - 1
                                        & 0xF;
                                if ({
                                    let t4 = (*pulseBlendPalette).blendCoeff();
                                    (*pulseBlendPalette)
                                        .set_blendCoeff((*pulseBlendPalette).blendCoeff() + 1);
                                    t4
                                }) == max
                                {
                                    (*pulseBlendPalette).fadeCycleCounter += 1;
                                    (*pulseBlendPalette).set_fadeDirection(
                                        (*pulseBlendPalette).fadeDirection() ^ 1,
                                    );
                                }
                            }
                        }
                        -2 => {
                            if (*pulseBlendPalette).fadeDirection() != 0 {
                                (*pulseBlendPalette).set_blendCoeff(0);
                            } else {
                                (*pulseBlendPalette).set_blendCoeff(
                                    (*pulseBlendPalette).pulseBlendSettings.maxBlendCoeff() as u8
                                        & 0xF,
                                );
                            }
                            (*pulseBlendPalette)
                                .set_fadeDirection((*pulseBlendPalette).fadeDirection() ^ 1);
                            (*pulseBlendPalette).fadeCycleCounter += 1;
                        }
                        _ => {}
                    }
                    if (*pulseBlendPalette).pulseBlendSettings.numFadeCycles != 0xFF
                        && (*pulseBlendPalette).fadeCycleCounter
                            == (*pulseBlendPalette).pulseBlendSettings.numFadeCycles
                    {
                        UnmarkUsedPulseBlendPalettes(
                            pulseBlend,
                            (*pulseBlendPalette).paletteSelector as u16,
                            FALSE,
                        );
                    }
                }
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillTilemapRect(
    mut dest: *mut u16,
    value: u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    let mut _dest: *mut u16 = null_mut();
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    dest = dest.at(top as i32 * 32 + left as i32);
    while i < height {
        _dest = dest.at(i as i32 * 32);
        j = 0;
        while j < width {
            *({
                let t1 = _dest;
                _dest = _dest.at(1);
                t1
            }) = value;
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTilemapRect(
    mut dest: *mut u16,
    src: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    let mut _dest: *mut u16 = null_mut();
    let mut _src: *mut u16 = src;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    dest = dest.at(top as i32 * 32 + left as i32);
    while i < height {
        _dest = dest.at(i as i32 * 32);
        j = 0;
        while j < width {
            *({
                let t1 = _dest;
                _dest = _dest.at(1);
                t1
            }) = *({
                let t3 = _src;
                _src = _src.at(1);
                t3
            });
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FillTilemapRect_Unused(
    dest: *mut c_void,
    value: u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    i = 0;
    y = top;
    while i < height {
        x = left;
        j = 0;
        while j < width {
            *((dest as *mut u8).at(y as i32 * 64 + x as i32 * 2) as *mut c_void as *mut u16) =
                value;
            x = ((x as i32 + 1) % 32) as u8;
            j += 1;
        }
        y = ((y as i32 + 1) % 32) as u8;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetTilemapRect_Unused(
    dest: *mut c_void,
    src: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut _src: *mut u16 = null_mut();
    i = 0;
    _src = src;
    y = top;
    while i < height {
        x = left;
        j = 0;
        while j < width {
            *((dest as *mut u8).at(y as i32 * 64 + x as i32 * 2) as *mut c_void as *mut u16) = *({
                let t2 = _src;
                _src = _src.at(1);
                t2
            });
            x = ((x as i32 + 1) % 32) as u8;
            j += 1;
        }
        y = ((y as i32 + 1) % 32) as u8;
        i += 1;
    }
}
