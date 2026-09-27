use crate::ffi::{
    CreateTask, DestroyTask, FieldCallback, MenuFieldCallback, PLTT_SIZE, PlaySE, gPlttBufferFaded,
    gPlttBufferUnfaded, palette_fade_active, set_field_move_callback, set_task_data, set_task_func,
    sprite_palette_num, task_data,
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

unsafe extern "C" {
    static mut gFieldCallback2: Option<FieldCallback>;
    static mut gPostMenuFieldCallback: Option<MenuFieldCallback>;
    static mut gFieldEffectArguments: [i32; 8];
    static mut gPaletteDecompressionBuffer: u8;
    static EventScript_FailSweetScent: u8;

    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn CreateFieldMoveTask() -> u8;
    fn FieldEffectStart(effect: u8) -> u32;
    fn FieldEffectActiveListRemove(effect: u8);
    fn GetPlayerAvatarSpriteId() -> u8;
    fn SetWeatherScreenFadeOut();
    fn SetWeatherPalStateIdle();
    fn CpuFastSet(src: *const c_void, dest: *mut c_void, control: u32);
    fn BeginNormalPaletteFade(
        selected_palettes: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        blend_color: u16,
    ) -> u8;
    fn BlendPalettes(selected_palettes: u32, coeff: u8, color: u16);
    fn ClearMirageTowerPulseBlendEffect();
    fn TryStartMirageTowerPulseBlendEffect();
    fn SweetScentWildEncounter() -> u8;
    fn ScriptContext_SetupScript(script: *const u8);
}

unsafe extern "C" fn field_callback_sweet_scent() {
    let _ = unsafe { FieldEffectStart(FLDEFF_SWEET_SCENT) };
    unsafe {
        (&raw mut gFieldEffectArguments)
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe extern "C" fn start_sweet_scent_field_effect() {
    unsafe { PlaySE(SE_M_SWEET_SCENT) };
    unsafe {
        CpuFastSet(
            (&raw const gPlttBufferUnfaded).cast(),
            (&raw mut gPaletteDecompressionBuffer).cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };
    unsafe {
        CpuFastSet(
            (&raw const gPlttBufferFaded).cast(),
            (&raw mut gPlttBufferUnfaded).cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };

    let palette_num = unsafe { sprite_palette_num(GetPlayerAvatarSpriteId() as usize) };
    let _ = unsafe { BeginNormalPaletteFade(all_palettes_except(palette_num), 4, 0, 8, RGB_RED) };

    let task_id = unsafe { CreateTask(try_sweet_scent_encounter, 0) };
    unsafe { set_task_data(task_id, 0, 0) };
    unsafe { FieldEffectActiveListRemove(FLDEFF_SWEET_SCENT) };
}

unsafe extern "C" fn try_sweet_scent_encounter(task_id: u8) {
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

unsafe extern "C" fn fail_sweet_scent_encounter(task_id: u8) {
    if unsafe { palette_fade_active() } {
        return;
    }

    unsafe {
        CpuFastSet(
            (&raw const gPaletteDecompressionBuffer).cast(),
            (&raw mut gPlttBufferUnfaded).cast(),
            PLTT_FAST_COPY_WORDS,
        )
    };
    unsafe { SetWeatherPalStateIdle() };
    unsafe { ScriptContext_SetupScript(addr_of!(EventScript_FailSweetScent)) };
    unsafe { DestroyTask(task_id) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_SweetScent() -> u8 {
    unsafe { addr_of_mut!(gFieldCallback2).write(Some(FieldCallback_PrepareFadeInFromMenu)) };
    unsafe { addr_of_mut!(gPostMenuFieldCallback).write(Some(field_callback_sweet_scent)) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SweetScent() -> u8 {
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
