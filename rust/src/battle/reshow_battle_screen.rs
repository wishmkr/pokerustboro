//! Rebuilds the battle screen after a menu (bag, party, summary) covered it:
//! backgrounds, battler and healthbox sprites, then fades back in. One step
//! per frame, driven by `gBattleScripting.reshowMainState`.

use crate::battle_main::{
    gBattle_BG0_X, gBattle_BG0_Y, gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y,
    gBattle_BG3_X, gBattle_BG3_Y, gBattleTypeFlags, gBattlerInMenuId, gBattlersCount,
};
use crate::bg::{SetBgAttribute, ShowBg};
use crate::ffi::{
    CpuFastSet, GetMonData2, MainCallback, POKEMON_SIZE, SpriteTemplate, set_oam_palette_num,
    set_sprite_callback, sprite, sprite_data,
};
use crate::gpu_regs::SetGpuReg;
use crate::link::{gReceivedRemoteLinkPlayers, gWirelessCommType};
use crate::load_save::gSaveBlock2Ptr;
use crate::scanline_effect::ScanlineEffect_Clear;
use crate::sprite::gReservedSpritePaletteCount;
use crate::sprite::{
    CreateSprite, FreeAllSpritePalettes, ResetSpriteData, SpriteCallbackDummy, StartSpriteAnim,
};

const REG_OFFSET_MOSAIC: u8 = 0x4c;
const REG_BG1CNT: usize = 0x0400_000a;
const REG_BG2CNT: usize = 0x0400_000c;
/// `vBgCnt.charBaseBlock`: bits 2-3.
const BGCNT_CHAR_BASE_MASK: u16 = 0x000c;
const VRAM: usize = 0x0600_0000;
const VRAM_SIZE: u32 = 0x1_8000;
const BG_ATTR_CHARBASEINDEX: u8 = 1;
const MAX_BATTLERS_COUNT: u8 = 4;

const B_SIDE_PLAYER: u8 = 0;
const B_POSITION_PLAYER_LEFT: u8 = 0;
const B_POSITION_OPPONENT_LEFT: u8 = 1;
const B_POSITION_PLAYER_RIGHT: u8 = 2;
const B_POSITION_OPPONENT_RIGHT: u8 = 3;
const BATTLE_TYPE_SAFARI: u32 = 0x80;
const BATTLE_TYPE_WALLY_TUTORIAL: u32 = 0x200;
const TRAINER_BACK_PIC_WALLY: u16 = 6;
const SPECIES_CASTFORM: u16 = 0x181;
const BATTLER_COORD_X_2: u8 = 2;
const HEALTHBOX_ALL: u8 = 0;
const HEALTHBOX_SAFARI_ALL_TEXT: u8 = 0x0a;
const MON_DATA_SPECIES: i32 = 0x0b;
const MON_DATA_HP: i32 = 0x39;

const SCRIPTING_RESHOW_MAIN_STATE: usize = 0x21;
const SCRIPTING_RESHOW_HELPER_STATE: usize = 0x22;
/// `gPaletteFade.bufferTransferDisabled`: byte 8, bit 7.
const PALETTE_FADE_BUFFER_TRANSFER_BYTE: usize = 8;
const PALETTE_FADE_BUFFER_TRANSFER_BIT: u8 = 0x80;
/// `struct BattleSpriteInfo`: flags halfword, then `transformSpecies`.
const SPRITE_INFO_SIZE: usize = 4;
const SPRITE_INFO_INVISIBLE: u8 = 0x1;
const SPRITE_INFO_BEHIND_SUBSTITUTE: u8 = 0x4;
const SPRITE_INFO_TRANSFORM_SPECIES: usize = 2;
const SB2_PLAYER_GENDER: usize = 8;
/// `struct MonCoords { u8 size; u8 y_offset; }`, padded to four bytes.
const MON_COORDS_SIZE: usize = 4;
const SPRITE_ANIMS: usize = 0x08;
const SPRITE_FLAGS0: usize = 0x3e;
const SPRITE_INVISIBLE: u8 = 0x04;

/// `SetHBlankCallback` with this module's view of its types.
#[inline]
unsafe fn SetHBlankCallback(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetHBlankCallback(a0);
    }
}
/// `SetVBlankCallback` with this module's view of its types.
#[inline]
unsafe fn SetVBlankCallback(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetVBlankCallback(a0);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: MainCallback) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `ResetPaletteFade` with this module's view of its types.
#[inline]
unsafe fn ResetPaletteFade() {
    unsafe {
        crate::palette::ResetPaletteFade();
    }
}
/// `BattleInitBgsAndWindows` with this module's view of its types.
#[inline]
unsafe fn BattleInitBgsAndWindows() {
    unsafe {
        crate::battle_bg::BattleInitBgsAndWindows();
    }
}
/// `LoadBattleTextboxAndBackground` with this module's view of its types.
#[inline]
unsafe fn LoadBattleTextboxAndBackground() {
    unsafe {
        crate::battle_bg::LoadBattleTextboxAndBackground();
    }
}
/// `ClearSpritesHealthboxAnimData` with this module's view of its types.
#[inline]
unsafe fn ClearSpritesHealthboxAnimData() {
    unsafe {
        crate::battle_gfx_sfx_util::ClearSpritesHealthboxAnimData();
    }
}
/// `BattleLoadAllHealthBoxesGfx` with this module's view of its types.
#[inline]
unsafe fn BattleLoadAllHealthBoxesGfx(a0: u8) -> u8 {
    unsafe { crate::battle_gfx_sfx_util::BattleLoadAllHealthBoxesGfx(a0) }
}
/// `LoadAndCreateEnemyShadowSprites` with this module's view of its types.
#[inline]
unsafe fn LoadAndCreateEnemyShadowSprites() {
    unsafe {
        crate::battle_gfx_sfx_util::LoadAndCreateEnemyShadowSprites();
    }
}
/// `SetBattlerShadowSpriteCallback` with this module's view of its types.
#[inline]
unsafe fn SetBattlerShadowSpriteCallback(a0: u8, a1: u16) {
    unsafe {
        crate::battle_gfx_sfx_util::SetBattlerShadowSpriteCallback(a0, a1);
    }
}
/// `ActionSelectionCreateCursorAt` with this module's view of its types.
#[inline]
unsafe fn ActionSelectionCreateCursorAt(a0: u8, a1: u8) {
    unsafe {
        crate::battle_controller_player::ActionSelectionCreateCursorAt(a0, a1);
    }
}
/// `LoadWirelessStatusIndicatorSpriteGfx` with this module's view of its types.
#[inline]
unsafe fn LoadWirelessStatusIndicatorSpriteGfx() {
    unsafe {
        crate::link_rfu_3::LoadWirelessStatusIndicatorSpriteGfx();
    }
}
/// `CreateWirelessStatusIndicatorSprite` with this module's view of its types.
#[inline]
unsafe fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8) {
    unsafe {
        crate::link_rfu_3::CreateWirelessStatusIndicatorSprite(a0, a1);
    }
}
/// `VBlankCB_Battle` with this module's view of its types.
#[inline]
unsafe fn VBlankCB_Battle() {
    unsafe {
        crate::battle_main::VBlankCB_Battle();
    }
}
/// `BeginHardwarePaletteFade` with this module's view of its types.
#[inline]
unsafe fn BeginHardwarePaletteFade(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) {
    unsafe {
        crate::palette::BeginHardwarePaletteFade(a0, a1, a2, a3, a4);
    }
}
/// `BattleMainCB2` with this module's view of its types.
#[inline]
unsafe fn BattleMainCB2() {
    unsafe {
        crate::battle_main::BattleMainCB2();
    }
}
/// `FillAroundBattleWindows` with this module's view of its types.
#[inline]
unsafe fn FillAroundBattleWindows() {
    unsafe {
        crate::battle_gfx_sfx_util::FillAroundBattleWindows();
    }
}
/// `BattleLoadOpponentMonSpriteGfx` with this module's view of its types.
#[inline]
unsafe fn BattleLoadOpponentMonSpriteGfx(a0: *mut u8, a1: u8) {
    unsafe {
        crate::battle_gfx_sfx_util::BattleLoadOpponentMonSpriteGfx(a0 as _, a1);
    }
}
/// `BattleLoadSubstituteOrMonSpriteGfx` with this module's view of its types.
#[inline]
unsafe fn BattleLoadSubstituteOrMonSpriteGfx(a0: u8, a1: u8) {
    unsafe {
        crate::battle_gfx_sfx_util::BattleLoadSubstituteOrMonSpriteGfx(a0, a1);
    }
}
/// `DecompressTrainerBackPic` with this module's view of its types.
#[inline]
unsafe fn DecompressTrainerBackPic(a0: u16, a1: u8) {
    unsafe {
        crate::battle_gfx_sfx_util::DecompressTrainerBackPic(a0, a1);
    }
}
/// `BattleLoadPlayerMonSpriteGfx` with this module's view of its types.
#[inline]
unsafe fn BattleLoadPlayerMonSpriteGfx(a0: *mut u8, a1: u8) {
    unsafe {
        crate::battle_gfx_sfx_util::BattleLoadPlayerMonSpriteGfx(a0 as _, a1);
    }
}
/// `GetSubstituteSpriteDefault_Y` with this module's view of its types.
#[inline]
unsafe fn GetSubstituteSpriteDefault_Y(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetSubstituteSpriteDefault_Y(a0) }
}
/// `GetBattlerSpriteDefault_Y` with this module's view of its types.
#[inline]
unsafe fn GetBattlerSpriteDefault_Y(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerSpriteDefault_Y(a0) }
}
/// `SetMultiuseSpriteTemplateToPokemon` with this module's view of its types.
#[inline]
unsafe fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8) {
    unsafe {
        crate::pokemon::SetMultiuseSpriteTemplateToPokemon(a0, a1);
    }
}
/// `SetMultiuseSpriteTemplateToTrainerBack` with this module's view of its types.
#[inline]
unsafe fn SetMultiuseSpriteTemplateToTrainerBack(a0: u16, a1: u8) {
    unsafe {
        crate::pokemon::SetMultiuseSpriteTemplateToTrainerBack(a0, a1);
    }
}
/// `GetBattlerSpriteCoord` with this module's view of its types.
#[inline]
unsafe fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerSpriteCoord(a0, a1) }
}
/// `GetBattlerSpriteSubpriority` with this module's view of its types.
#[inline]
unsafe fn GetBattlerSpriteSubpriority(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerSpriteSubpriority(a0) }
}
/// `CreateSafariPlayerHealthboxSprites` with this module's view of its types.
#[inline]
unsafe fn CreateSafariPlayerHealthboxSprites() -> u8 {
    unsafe { crate::battle_interface::CreateSafariPlayerHealthboxSprites() }
}
/// `CreateBattlerHealthboxSprites` with this module's view of its types.
#[inline]
unsafe fn CreateBattlerHealthboxSprites(a0: u8) -> u8 {
    unsafe { crate::battle_interface::CreateBattlerHealthboxSprites(a0) }
}
/// `InitBattlerHealthboxCoords` with this module's view of its types.
#[inline]
unsafe fn InitBattlerHealthboxCoords(a0: u8) {
    unsafe {
        crate::battle_interface::InitBattlerHealthboxCoords(a0);
    }
}
/// `SetHealthboxSpriteVisible` with this module's view of its types.
#[inline]
unsafe fn SetHealthboxSpriteVisible(a0: u8) {
    unsafe {
        crate::battle_interface::SetHealthboxSpriteVisible(a0);
    }
}
/// `SetHealthboxSpriteInvisible` with this module's view of its types.
#[inline]
unsafe fn SetHealthboxSpriteInvisible(a0: u8) {
    unsafe {
        crate::battle_interface::SetHealthboxSpriteInvisible(a0);
    }
}
/// `UpdateHealthboxAttribute` with this module's view of its types.
#[inline]
unsafe fn UpdateHealthboxAttribute(a0: u8, a1: *mut u8, a2: u8) {
    unsafe {
        crate::battle_interface::UpdateHealthboxAttribute(a0, a1 as _, a2);
    }
}
/// `DummyBattleInterfaceFunc` with this module's view of its types.
#[inline]
unsafe fn DummyBattleInterfaceFunc(a0: u8, a1: u8) {
    unsafe {
        crate::battle_interface::DummyBattleInterfaceFunc(a0, a1);
    }
}
/// `GetBattlerPosition` with this module's view of its types.
#[inline]
unsafe fn GetBattlerPosition(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerPosition(a0) }
}
/// `GetBattlerAtPosition` with this module's view of its types.
#[inline]
unsafe fn GetBattlerAtPosition(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerAtPosition(a0) }
}
/// `GetBattlerSide` with this module's view of its types.
#[inline]
unsafe fn GetBattlerSide(a0: u8) -> u8 {
    unsafe { crate::battle_anim_mons::GetBattlerSide(a0) }
}
/// `IsDoubleBattle` with this module's view of its types.
#[inline]
unsafe fn IsDoubleBattle() -> u8 {
    unsafe { crate::battle_anim_mons::IsDoubleBattle() }
}

#[inline]
unsafe fn scripting(offset: usize) -> *mut u8 {
    unsafe {
        (&raw mut (*(&raw const crate::battle_main::gBattleScripting)
            .cast::<u8>()
            .cast_mut()))
            .add(offset)
    }
}

#[inline]
fn battle_type_has(flag: u32) -> bool {
    let flags = unsafe { (&raw const gBattleTypeFlags).read() };
    flags & flag != 0
}

#[inline]
unsafe fn set_buffer_transfer_disabled(disabled: bool) {
    let byte = unsafe {
        (&raw mut (*(&raw const crate::palette::gPaletteFade)
            .cast::<u8>()
            .cast_mut()))
            .add(PALETTE_FADE_BUFFER_TRANSFER_BYTE)
    };
    let value = unsafe { byte.read_volatile() };
    let value = if disabled {
        value | PALETTE_FADE_BUFFER_TRANSFER_BIT
    } else {
        value & !PALETTE_FADE_BUFFER_TRANSFER_BIT
    };
    unsafe { byte.write_volatile(value) };
}

/// `&gBattleSpritesDataPtr->battlerData[battler]`
#[inline]
unsafe fn sprite_info(battler: u8) -> *const u8 {
    let data = unsafe {
        (&raw const (*(&raw const crate::battle_main::gBattleSpritesDataPtr)
            .cast::<*const *const u8>()))
            .read()
    };
    let battler_data = unsafe { data.read() };
    unsafe { battler_data.add(usize::from(battler) * SPRITE_INFO_SIZE) }
}

#[inline]
unsafe fn behind_substitute(battler: u8) -> bool {
    let flags = unsafe { sprite_info(battler).read() };
    flags & SPRITE_INFO_BEHIND_SUBSTITUTE != 0
}

#[inline]
unsafe fn party_index(battler: u8) -> usize {
    usize::from(unsafe {
        (*(&raw const crate::battle_main::gBattlerPartyIndexes).cast::<[u16; 4]>())
            [usize::from(battler) % 4]
    })
}

#[inline]
unsafe fn battler_mon(battler: u8, enemy: bool) -> *mut u8 {
    let party = if enemy {
        (&raw mut (*(&raw const crate::pokemon::gEnemyParty)
            .cast::<u8>()
            .cast_mut()))
            .cast::<u8>()
    } else {
        (&raw mut (*(&raw const crate::pokemon::gPlayerParty)
            .cast::<u8>()
            .cast_mut()))
            .cast::<u8>()
    };
    unsafe { party.add(party_index(battler) * POKEMON_SIZE) }
}

#[inline]
unsafe fn player_gender() -> u8 {
    unsafe {
        (&raw const gSaveBlock2Ptr)
            .read()
            .cast::<u8>()
            .add(SB2_PLAYER_GENDER)
            .read()
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ReshowBattleScreenDummy() {}

#[unsafe(no_mangle)]
pub unsafe fn ReshowBattleScreenAfterMenu() {
    unsafe { set_buffer_transfer_disabled(true) };
    unsafe { SetHBlankCallback(None) };
    unsafe { SetVBlankCallback(None) };
    unsafe { SetGpuReg(REG_OFFSET_MOSAIC, 0) };
    unsafe { scripting(SCRIPTING_RESHOW_MAIN_STATE).write(0) };
    unsafe { scripting(SCRIPTING_RESHOW_HELPER_STATE).write(0) };
    unsafe { SetMainCallback2(cb2_reshow_battle_screen_after_menu) };
}

unsafe fn cb2_reshow_battle_screen_after_menu() {
    let main_state = unsafe { scripting(SCRIPTING_RESHOW_MAIN_STATE) };
    let helper_state = unsafe { scripting(SCRIPTING_RESHOW_HELPER_STATE) };
    let step_back = || unsafe { main_state.write(main_state.read().wrapping_sub(1)) };

    match unsafe { main_state.read() } {
        0 => unsafe {
            ScanlineEffect_Clear();
            BattleInitBgsAndWindows();
            SetBgAttribute(1, BG_ATTR_CHARBASEINDEX, 0);
            SetBgAttribute(2, BG_ATTR_CHARBASEINDEX, 0);
            for bg in 0..4 {
                ShowBg(bg);
            }
            ResetPaletteFade();
            for register in [
                &raw mut gBattle_BG0_X,
                &raw mut gBattle_BG0_Y,
                &raw mut gBattle_BG1_X,
                &raw mut gBattle_BG1_Y,
                &raw mut gBattle_BG2_X,
                &raw mut gBattle_BG2_Y,
                &raw mut gBattle_BG3_X,
                &raw mut gBattle_BG3_Y,
            ] {
                register.write(0);
            }
        },
        1 => {
            // CpuFastFill(0, VRAM, VRAM_SIZE)
            let zero = 0u32;
            unsafe {
                CpuFastSet(
                    (&raw const zero).cast(),
                    VRAM as *mut _,
                    (1 << 24) | (VRAM_SIZE / 4),
                )
            };
        }
        2 => unsafe { LoadBattleTextboxAndBackground() },
        3 => unsafe { ResetSpriteData() },
        4 => {
            unsafe { FreeAllSpritePalettes() };
            unsafe { (&raw mut gReservedSpritePaletteCount).write(MAX_BATTLERS_COUNT) };
        }
        5 => unsafe { ClearSpritesHealthboxAnimData() },
        6 => {
            if unsafe { BattleLoadAllHealthBoxesGfx(helper_state.read()) } != 0 {
                unsafe { helper_state.write(0) };
            } else {
                unsafe { helper_state.write(helper_state.read().wrapping_add(1)) };
                step_back();
            }
        }
        state @ 7..=10 => {
            if !unsafe { load_battler_sprite_gfx(state - 7) } {
                step_back();
            }
        }
        state @ 11..=14 => unsafe { create_battler_sprite(state - 11) },
        state @ 15..=18 => unsafe { create_healthbox_sprite(state - 15) },
        19 => unsafe { reshow_shadows_and_cursor() },
        _ => {
            unsafe { SetVBlankCallback(Some(VBlankCB_Battle)) };
            unsafe { clear_battle_bg_cnt_base_blocks() };
            unsafe { BeginHardwarePaletteFade(0xff, 0, 0x10, 0, 1) };
            unsafe { set_buffer_transfer_disabled(false) };
            unsafe { SetMainCallback2(BattleMainCB2) };
            unsafe { FillAroundBattleWindows() };
        }
    }
    unsafe { main_state.write(main_state.read().wrapping_add(1)) };
}

unsafe fn reshow_shadows_and_cursor() {
    unsafe { LoadAndCreateEnemyShadowSprites() };
    let mut positions = [B_POSITION_OPPONENT_LEFT, B_POSITION_OPPONENT_RIGHT].into_iter();
    let count = if unsafe { IsDoubleBattle() } != 0 {
        2
    } else {
        1
    };
    for position in positions.by_ref().take(count) {
        let opponent = unsafe { GetBattlerAtPosition(position) };
        let species = unsafe { GetMonData2(battler_mon(opponent, true), MON_DATA_SPECIES) } as u16;
        unsafe { SetBattlerShadowSpriteCallback(opponent, species) };
    }
    let menu_battler = usize::from(unsafe { (&raw const gBattlerInMenuId).read() }) % 4;
    unsafe {
        ActionSelectionCreateCursorAt(
            (*(&raw const crate::battle_main::gActionSelectionCursor).cast::<[u8; 4]>())
                [menu_battler],
            0,
        )
    };
    let wireless = unsafe { (&raw const gWirelessCommType).read() } != 0;
    if wireless && unsafe { (&raw const gReceivedRemoteLinkPlayers).read() } != 0 {
        unsafe { LoadWirelessStatusIndicatorSpriteGfx() };
        unsafe { CreateWirelessStatusIndicatorSprite(0, 0) };
    }
}

unsafe fn clear_battle_bg_cnt_base_blocks() {
    for register in [REG_BG1CNT, REG_BG2CNT] {
        let register = register as *mut u16;
        unsafe { register.write_volatile(register.read_volatile() & !BGCNT_CHAR_BASE_MASK) };
    }
}

unsafe fn load_battler_sprite_gfx(battler: u8) -> bool {
    if battler >= unsafe { (&raw const gBattlersCount).read() } {
        return true;
    }
    if unsafe { GetBattlerSide(battler) } != B_SIDE_PLAYER {
        if !unsafe { behind_substitute(battler) } {
            unsafe { BattleLoadOpponentMonSpriteGfx(battler_mon(battler, true), battler) };
        } else {
            unsafe { BattleLoadSubstituteOrMonSpriteGfx(battler, 0) };
        }
    // These check the battler id where they mean the position, as in the original.
    } else if battle_type_has(BATTLE_TYPE_SAFARI) && battler == B_POSITION_PLAYER_LEFT {
        unsafe { DecompressTrainerBackPic(u16::from(player_gender()), battler) };
    } else if battle_type_has(BATTLE_TYPE_WALLY_TUTORIAL) && battler == B_POSITION_PLAYER_LEFT {
        unsafe { DecompressTrainerBackPic(TRAINER_BACK_PIC_WALLY, battler) };
    } else if !unsafe { behind_substitute(battler) } {
        unsafe { BattleLoadPlayerMonSpriteGfx(battler_mon(battler, false), battler) };
    } else {
        unsafe { BattleLoadSubstituteOrMonSpriteGfx(battler, 0) };
    }
    unsafe { scripting(SCRIPTING_RESHOW_HELPER_STATE).write(0) };
    true
}

unsafe fn finish_battler_sprite(battler: u8) -> *mut u8 {
    let id = unsafe {
        (*(&raw const crate::battle_main::gBattlerSpriteIds)
            .cast::<[u8; 4]>()
            .cast_mut())[usize::from(battler) % 4]
    };
    let s = unsafe { sprite(usize::from(id)) };
    unsafe { set_oam_palette_num(s, battler) };
    unsafe { set_sprite_callback(s, SpriteCallbackDummy) };
    unsafe { sprite_data(s, 0).write(i16::from(battler)) };
    s
}

/// A trainer back sprite at x 0x50, lowered by how short the picture is.
unsafe fn create_trainer_back_sprite(battler: u8, pic_id: u16) {
    unsafe {
        SetMultiuseSpriteTemplateToTrainerBack(pic_id, GetBattlerPosition(B_POSITION_PLAYER_LEFT))
    };
    let coords = unsafe {
        (&raw const (*(&raw const crate::data::data_tables::gTrainerBackPicCoords).cast::<u8>()))
            .add(usize::from(pic_id) * MON_COORDS_SIZE)
    };
    let size = i16::from(unsafe { coords.read() });
    let y = (8 - size) * 4 + 80;
    let id = unsafe {
        CreateSprite(
            &raw const (*(&raw const crate::pokemon::gMultiuseSpriteTemplate)
                .cast::<SpriteTemplate>()
                .cast_mut()),
            0x50,
            y,
            GetBattlerSpriteSubpriority(0),
        )
    };
    unsafe {
        (*(&raw const crate::battle_main::gBattlerSpriteIds)
            .cast::<[u8; 4]>()
            .cast_mut())[usize::from(battler) % 4] = id
    };
    unsafe { finish_battler_sprite(battler) };
}

unsafe fn create_mon_sprite(battler: u8, enemy: bool, y: u8) -> bool {
    let mon = unsafe { battler_mon(battler, enemy) };
    if unsafe { GetMonData2(mon, MON_DATA_HP) } == 0 {
        return false;
    }
    let species = unsafe { GetMonData2(mon, MON_DATA_SPECIES) } as u16;
    unsafe { SetMultiuseSpriteTemplateToPokemon(species, GetBattlerPosition(battler)) };
    let id = unsafe {
        CreateSprite(
            &raw const (*(&raw const crate::pokemon::gMultiuseSpriteTemplate)
                .cast::<SpriteTemplate>()
                .cast_mut()),
            i16::from(GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2)),
            i16::from(y),
            GetBattlerSpriteSubpriority(battler),
        )
    };
    unsafe {
        (*(&raw const crate::battle_main::gBattlerSpriteIds)
            .cast::<[u8; 4]>()
            .cast_mut())[usize::from(battler) % 4] = id
    };
    let s = unsafe { finish_battler_sprite(battler) };
    let species_again = unsafe { GetMonData2(mon, MON_DATA_SPECIES) };
    unsafe { sprite_data(s, 2).write(species_again as i16) };
    unsafe {
        StartSpriteAnim(
            s,
            (*(&raw const crate::battle_main::gBattleMonForms).cast::<[u8; 4]>())
                [usize::from(battler) % 4],
        )
    };
    let transform = unsafe {
        sprite_info(battler)
            .add(SPRITE_INFO_TRANSFORM_SPECIES)
            .cast::<u16>()
            .read()
    };
    if transform == SPECIES_CASTFORM {
        let anims = unsafe {
            (&raw const (*(&raw const crate::data::data_tables::gMonFrontAnimsPtrTable)
                .cast::<[*const u8; 1]>()))
                .cast::<*const u8>()
                .add(usize::from(SPECIES_CASTFORM))
                .read()
        };
        unsafe { s.add(SPRITE_ANIMS).cast::<*const u8>().write(anims) };
    }
    true
}

unsafe fn create_battler_sprite(battler: u8) {
    if battler >= unsafe { (&raw const gBattlersCount).read() } {
        return;
    }
    let y = if unsafe { behind_substitute(battler) } {
        unsafe { GetSubstituteSpriteDefault_Y(battler) }
    } else {
        unsafe { GetBattlerSpriteDefault_Y(battler) }
    };

    if unsafe { GetBattlerSide(battler) } != B_SIDE_PLAYER {
        if !unsafe { create_mon_sprite(battler, true, y) } {
            return;
        }
    } else if battle_type_has(BATTLE_TYPE_SAFARI) && battler == B_POSITION_PLAYER_LEFT {
        unsafe { create_trainer_back_sprite(battler, u16::from(player_gender())) };
    } else if battle_type_has(BATTLE_TYPE_WALLY_TUTORIAL) && battler == B_POSITION_PLAYER_LEFT {
        unsafe { create_trainer_back_sprite(battler, TRAINER_BACK_PIC_WALLY) };
    } else if !unsafe { create_mon_sprite(battler, false, y) } {
        return;
    }

    let id = unsafe {
        (*(&raw const crate::battle_main::gBattlerSpriteIds)
            .cast::<[u8; 4]>()
            .cast_mut())[usize::from(battler) % 4]
    };
    let s = unsafe { sprite(usize::from(id)) };
    let invisible = unsafe { sprite_info(battler).read() } & SPRITE_INFO_INVISIBLE != 0;
    let flags = unsafe { s.add(SPRITE_FLAGS0).read() };
    let flags = if invisible {
        flags | SPRITE_INVISIBLE
    } else {
        flags & !SPRITE_INVISIBLE
    };
    unsafe { s.add(SPRITE_FLAGS0).write(flags) };
}

unsafe fn create_healthbox_sprite(battler: u8) {
    if battler >= unsafe { (&raw const gBattlersCount).read() } {
        return;
    }
    let safari = battle_type_has(BATTLE_TYPE_SAFARI);
    let healthbox = if safari && battler == B_POSITION_PLAYER_LEFT {
        unsafe { CreateSafariPlayerHealthboxSprites() }
    } else if battle_type_has(BATTLE_TYPE_WALLY_TUTORIAL) && battler == B_POSITION_PLAYER_LEFT {
        return;
    } else {
        unsafe { CreateBattlerHealthboxSprites(battler) }
    };

    unsafe {
        (*(&raw const crate::battle_main::gHealthboxSpriteIds)
            .cast::<[u8; 4]>()
            .cast_mut())[usize::from(battler) % 4] = healthbox
    };
    unsafe { InitBattlerHealthboxCoords(battler) };
    unsafe { SetHealthboxSpriteVisible(healthbox) };

    let enemy = unsafe { GetBattlerSide(battler) } != B_SIDE_PLAYER;
    let element = if !enemy && safari {
        HEALTHBOX_SAFARI_ALL_TEXT
    } else {
        HEALTHBOX_ALL
    };
    unsafe { UpdateHealthboxAttribute(healthbox, battler_mon(battler, enemy), element) };

    let position = unsafe { GetBattlerPosition(battler) };
    let right = position == B_POSITION_OPPONENT_RIGHT || position == B_POSITION_PLAYER_RIGHT;
    unsafe { DummyBattleInterfaceFunc(healthbox, u8::from(right)) };

    if (enemy || !safari) && unsafe { GetMonData2(battler_mon(battler, enemy), MON_DATA_HP) } == 0 {
        unsafe { SetHealthboxSpriteInvisible(healthbox) };
    }
}
