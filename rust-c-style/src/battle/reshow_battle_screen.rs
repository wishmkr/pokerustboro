//! Rebuilds the battle screen after a menu (bag, party, summary) covered it:
//! backgrounds, battler and healthbox sprites, then fades back in. One step
//! per frame, driven by `gBattleScripting.reshowMainState`.

use crate::bg::{SetBgAttribute, ShowBg};
use crate::ffi::{
    CpuFastSet, GetMonData2, MainCallback, POKEMON_SIZE, SpriteTemplate, gEnemyParty, gPlayerParty,
    set_oam_palette_num, set_sprite_callback, sprite, sprite_data,
};
use crate::gpu_regs::SetGpuReg;
use crate::load_save::gSaveBlock2Ptr;
use crate::scanline_effect::ScanlineEffect_Clear;
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

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gBattleScripting: u8;
    static gBattleSpritesDataPtr: *const *const u8;
    static gBattleTypeFlags: u32;
    static gBattlersCount: u8;
    static gBattlerPartyIndexes: [u16; 4];
    static mut gBattlerSpriteIds: [u8; 4];
    static mut gHealthboxSpriteIds: [u8; 4];
    static gBattleMonForms: [u8; 4];
    static gActionSelectionCursor: [u8; 4];
    static gBattlerInMenuId: u8;
    static gWirelessCommType: u8;
    static gReceivedRemoteLinkPlayers: u8;
    static mut gReservedSpritePaletteCount: u8;
    static gMonFrontAnimsPtrTable: [*const u8; 1];
    static gTrainerBackPicCoords: u8;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gBattle_BG0_X: u16;
    static mut gBattle_BG0_Y: u16;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;

    fn SetHBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(callback: MainCallback);
    fn ResetPaletteFade();
    fn BattleInitBgsAndWindows();
    fn LoadBattleTextboxAndBackground();
    fn ClearSpritesHealthboxAnimData();
    fn BattleLoadAllHealthBoxesGfx(state: u8) -> u8;
    fn LoadAndCreateEnemyShadowSprites();
    fn SetBattlerShadowSpriteCallback(battler: u8, species: u16);
    fn ActionSelectionCreateCursorAt(cursor_position: u8, base_tile_num: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn CreateWirelessStatusIndicatorSprite(x: u8, y: u8);
    fn VBlankCB_Battle();
    fn BeginHardwarePaletteFade(blend_cnt: u8, delay: u8, y: u8, target_y: u8, reset: u8);
    fn BattleMainCB2();
    fn FillAroundBattleWindows();
    fn BattleLoadOpponentMonSpriteGfx(mon: *mut u8, battler: u8);
    fn BattleLoadSubstituteOrMonSpriteGfx(battler: u8, load_mon_sprite: u8);
    fn DecompressTrainerBackPic(back_pic_id: u16, battler: u8);
    fn BattleLoadPlayerMonSpriteGfx(mon: *mut u8, battler: u8);
    fn GetSubstituteSpriteDefault_Y(battler: u8) -> u8;
    fn GetBattlerSpriteDefault_Y(battler: u8) -> u8;
    fn SetMultiuseSpriteTemplateToPokemon(species_tag: u16, battler_position: u8);
    fn SetMultiuseSpriteTemplateToTrainerBack(trainer_pic_id: u16, battler_position: u8);
    fn GetBattlerSpriteCoord(battler: u8, coord_type: u8) -> u8;
    fn GetBattlerSpriteSubpriority(battler: u8) -> u8;
    fn CreateSafariPlayerHealthboxSprites() -> u8;
    fn CreateBattlerHealthboxSprites(battler: u8) -> u8;
    fn InitBattlerHealthboxCoords(battler: u8);
    fn SetHealthboxSpriteVisible(healthbox_sprite_id: u8);
    fn SetHealthboxSpriteInvisible(healthbox_sprite_id: u8);
    fn UpdateHealthboxAttribute(healthbox_sprite_id: u8, mon: *mut u8, element_id: u8);
    fn DummyBattleInterfaceFunc(healthbox_sprite_id: u8, is_double_battler_only: u8);
    fn GetBattlerPosition(battler: u8) -> u8;
    fn GetBattlerAtPosition(position: u8) -> u8;
    fn GetBattlerSide(battler: u8) -> u8;
    fn IsDoubleBattle() -> u8;
}

#[inline]
unsafe fn scripting(offset: usize) -> *mut u8 {
    unsafe { (&raw mut gBattleScripting).add(offset) }
}

#[inline]
fn battle_type_has(flag: u32) -> bool {
    let flags = unsafe { (&raw const gBattleTypeFlags).read() };
    flags & flag != 0
}

#[inline]
unsafe fn set_buffer_transfer_disabled(disabled: bool) {
    let byte = unsafe { (&raw mut gPaletteFade).add(PALETTE_FADE_BUFFER_TRANSFER_BYTE) };
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
    let data = unsafe { (&raw const gBattleSpritesDataPtr).read() };
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
    usize::from(unsafe { gBattlerPartyIndexes[usize::from(battler) % 4] })
}

#[inline]
unsafe fn battler_mon(battler: u8, enemy: bool) -> *mut u8 {
    let party = if enemy {
        (&raw mut gEnemyParty).cast::<u8>()
    } else {
        (&raw mut gPlayerParty).cast::<u8>()
    };
    unsafe { party.add(party_index(battler) * POKEMON_SIZE) }
}

#[inline]
unsafe fn player_gender() -> u8 {
    unsafe {
        (&raw const gSaveBlock2Ptr)
            .read()
            .add(SB2_PLAYER_GENDER)
            .read()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReshowBattleScreenDummy() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReshowBattleScreenAfterMenu() {
    unsafe { set_buffer_transfer_disabled(true) };
    unsafe { SetHBlankCallback(None) };
    unsafe { SetVBlankCallback(None) };
    unsafe { SetGpuReg(REG_OFFSET_MOSAIC, 0) };
    unsafe { scripting(SCRIPTING_RESHOW_MAIN_STATE).write(0) };
    unsafe { scripting(SCRIPTING_RESHOW_HELPER_STATE).write(0) };
    unsafe { SetMainCallback2(cb2_reshow_battle_screen_after_menu) };
}

unsafe extern "C" fn cb2_reshow_battle_screen_after_menu() {
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
    unsafe { ActionSelectionCreateCursorAt(gActionSelectionCursor[menu_battler], 0) };
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
    let id = unsafe { gBattlerSpriteIds[usize::from(battler) % 4] };
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
    let coords =
        unsafe { (&raw const gTrainerBackPicCoords).add(usize::from(pic_id) * MON_COORDS_SIZE) };
    let size = i16::from(unsafe { coords.read() });
    let y = (8 - size) * 4 + 80;
    let id = unsafe {
        CreateSprite(
            &raw const gMultiuseSpriteTemplate,
            0x50,
            y,
            GetBattlerSpriteSubpriority(0),
        )
    };
    unsafe { gBattlerSpriteIds[usize::from(battler) % 4] = id };
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
            &raw const gMultiuseSpriteTemplate,
            i16::from(GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2)),
            i16::from(y),
            GetBattlerSpriteSubpriority(battler),
        )
    };
    unsafe { gBattlerSpriteIds[usize::from(battler) % 4] = id };
    let s = unsafe { finish_battler_sprite(battler) };
    let species_again = unsafe { GetMonData2(mon, MON_DATA_SPECIES) };
    unsafe { sprite_data(s, 2).write(species_again as i16) };
    unsafe { StartSpriteAnim(s, gBattleMonForms[usize::from(battler) % 4]) };
    let transform = unsafe {
        sprite_info(battler)
            .add(SPRITE_INFO_TRANSFORM_SPECIES)
            .cast::<u16>()
            .read()
    };
    if transform == SPECIES_CASTFORM {
        let anims = unsafe {
            (&raw const gMonFrontAnimsPtrTable)
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

    let id = unsafe { gBattlerSpriteIds[usize::from(battler) % 4] };
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

    unsafe { gHealthboxSpriteIds[usize::from(battler) % 4] = healthbox };
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
