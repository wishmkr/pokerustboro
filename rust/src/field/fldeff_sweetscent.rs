use crate::ffi::{
    CreateTask, DestroyTask, FieldCallback, MenuFieldCallback, PLTT_SIZE, PlaySE,
    palette_fade_active, set_field_move_callback, set_task_data, set_task_func, sprite_palette_num,
    task_data,
};
use core::ffi::c_void;
use core::ptr::{addr_of, addr_of_mut};

const FLDEFF_SWEET_SCENT: u8 = 51;
const SE_M_SWEET_SCENT: u16 = 236;
const RGB_RED: u16 = 31;

/// `CpuFastCopy(src, dst, PLTT_SIZE)` expands to a `CpuFastSet` of
/// `PLTT_SIZE / 4` words.
const PLTT_FAST_COPY_WORDS: u32 = (PLTT_SIZE as u32 / 4) & 0x1f_ffff;

/// Every palette except the player's own sprite palette.
const fn all_palettes_except(palette_num: u8) -> u32 {
    !(1u32 << (palette_num as u32 + 16))
}

/// `FieldCallback_PrepareFadeInFromMenu` with this module's view of its types.
#[inline]
unsafe fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    unsafe { crate::party_menu::FieldCallback_PrepareFadeInFromMenu() }
}
/// `GetCursorSelectionMonId` with this module's view of its types.
#[inline]
unsafe fn GetCursorSelectionMonId() -> u8 {
    unsafe { crate::party_menu::GetCursorSelectionMonId() }
}
/// `CreateFieldMoveTask` with this module's view of its types.
#[inline]
unsafe fn CreateFieldMoveTask() -> u8 {
    unsafe { crate::fldeff_rocksmash::CreateFieldMoveTask() }
}
/// `FieldEffectStart` with this module's view of its types.
#[inline]
unsafe fn FieldEffectStart(a0: u8) -> u32 {
    unsafe { crate::field_effect::FieldEffectStart(a0) }
}
/// `FieldEffectActiveListRemove` with this module's view of its types.
#[inline]
unsafe fn FieldEffectActiveListRemove(a0: u8) {
    unsafe {
        crate::field_effect::FieldEffectActiveListRemove(a0);
    }
}
/// `GetPlayerAvatarSpriteId` with this module's view of its types.
#[inline]
unsafe fn GetPlayerAvatarSpriteId() -> u8 {
    unsafe { crate::field_player_avatar::GetPlayerAvatarSpriteId() }
}
/// `SetWeatherScreenFadeOut` with this module's view of its types.
#[inline]
unsafe fn SetWeatherScreenFadeOut() {
    unsafe {
        crate::field_weather::SetWeatherScreenFadeOut();
    }
}
/// `SetWeatherPalStateIdle` with this module's view of its types.
#[inline]
unsafe fn SetWeatherPalStateIdle() {
    unsafe {
        crate::field_weather::SetWeatherPalStateIdle();
    }
}
/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *const c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `BeginNormalPaletteFade` with this module's view of its types.
#[inline]
unsafe fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8 {
    unsafe { crate::palette::BeginNormalPaletteFade(a0, a1, a2, a3, a4) }
}
/// `BlendPalettes` with this module's view of its types.
#[inline]
unsafe fn BlendPalettes(a0: u32, a1: u8, a2: u16) {
    unsafe {
        crate::palette::BlendPalettes(a0, a1, a2);
    }
}
/// `ClearMirageTowerPulseBlendEffect` with this module's view of its types.
#[inline]
unsafe fn ClearMirageTowerPulseBlendEffect() {
    unsafe {
        crate::mirage_tower::ClearMirageTowerPulseBlendEffect();
    }
}
/// `TryStartMirageTowerPulseBlendEffect` with this module's view of its types.
#[inline]
unsafe fn TryStartMirageTowerPulseBlendEffect() {
    unsafe {
        crate::mirage_tower::TryStartMirageTowerPulseBlendEffect();
    }
}
/// `SweetScentWildEncounter` with this module's view of its types.
#[inline]
unsafe fn SweetScentWildEncounter() -> u8 {
    unsafe { crate::wild_encounter::SweetScentWildEncounter() }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *const u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}

unsafe fn field_callback_sweet_scent() {
    let _ = unsafe { FieldEffectStart(FLDEFF_SWEET_SCENT) };
    unsafe {
        (&raw mut (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe fn start_sweet_scent_field_effect() {
    unsafe { PlaySE(SE_M_SWEET_SCENT) };
    unsafe {
        CpuFastSet(
            (&raw const (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast(),
            (&raw mut (*(&raw const crate::palette::gPaletteDecompressionBuffer)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };
    unsafe {
        CpuFastSet(
            (&raw const (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast(),
            (&raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };

    let palette_num = unsafe { sprite_palette_num(GetPlayerAvatarSpriteId() as usize) };
    let _ = unsafe { BeginNormalPaletteFade(all_palettes_except(palette_num), 4, 0, 8, RGB_RED) };

    let task_id = unsafe { CreateTask(try_sweet_scent_encounter, 0) };
    unsafe { set_task_data(task_id, 0, 0) };
    unsafe { FieldEffectActiveListRemove(FLDEFF_SWEET_SCENT) };
}

unsafe fn try_sweet_scent_encounter(task_id: u8) {
    if unsafe { palette_fade_active() } {
        return;
    }

    unsafe { ClearMirageTowerPulseBlendEffect() };
    unsafe { BlendPalettes(0x0000_0040, 8, RGB_RED) };

    if unsafe { task_data(task_id, 0) } != 64 {
        let counter = unsafe { task_data(task_id, 0) };
        unsafe { set_task_data(task_id, 0, counter.wrapping_add(1)) };
        return;
    }

    unsafe { set_task_data(task_id, 0, 0) };
    if unsafe { SweetScentWildEncounter() } == 1 {
        unsafe { DestroyTask(task_id) };
    } else {
        unsafe { set_task_func(task_id, fail_sweet_scent_encounter) };
        let palette_num = unsafe { sprite_palette_num(GetPlayerAvatarSpriteId() as usize) };
        let _ =
            unsafe { BeginNormalPaletteFade(all_palettes_except(palette_num), 4, 8, 0, RGB_RED) };
        unsafe { TryStartMirageTowerPulseBlendEffect() };
    }
}

unsafe fn fail_sweet_scent_encounter(task_id: u8) {
    if unsafe { palette_fade_active() } {
        return;
    }

    unsafe {
        CpuFastSet(
            (&raw const (*(&raw const crate::palette::gPaletteDecompressionBuffer)
                .cast::<u8>()
                .cast_mut()))
                .cast(),
            (&raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };
    unsafe { SetWeatherPalStateIdle() };
    unsafe {
        ScriptContext_SetupScript(addr_of!(
            (*crate::asmdata::EventScript_FailSweetScent.cast::<u8>())
        ))
    };
    unsafe { DestroyTask(task_id) };
}

#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_SweetScent() -> u8 {
    unsafe {
        addr_of_mut!(
            (*(&raw const crate::overworld::gFieldCallback2)
                .cast::<Option<FieldCallback>>()
                .cast_mut())
        )
        .write(Some(FieldCallback_PrepareFadeInFromMenu))
    };
    unsafe {
        addr_of_mut!(
            (*(&raw const crate::party_menu::gPostMenuFieldCallback)
                .cast::<Option<MenuFieldCallback>>()
                .cast_mut())
        )
        .write(Some(field_callback_sweet_scent))
    };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn FldEff_SweetScent() -> u8 {
    unsafe { SetWeatherScreenFadeOut() };
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_sweet_scent_field_effect) };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_mask_clears_only_the_player_sprite_palette() {
        assert_eq!(all_palettes_except(0), 0xfffe_ffff);
        assert_eq!(all_palettes_except(5), !(1u32 << 21));
        assert_eq!(all_palettes_except(15), !(1u32 << 31));
    }

    #[test]
    fn palette_copy_uses_the_expanded_cpufastcopy_word_count() {
        assert_eq!(PLTT_FAST_COPY_WORDS, 256);
    }
}
