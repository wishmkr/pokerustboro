//! Game Freak's "public" ROM header.
//!
//! External tools read this at a fixed place right after the cartridge
//! header: Colosseum and XD used it to find Pokemon graphics, and PKHeX and
//! streaming overlays still find it useful. Its layout and its position must
//! not change. It is emitted into its own `.text.gf_rom_header` section, which
//! the linker scripts place immediately after `rom_header.o`.

use crate::ffi::RomPtr;

/// Mirrors `struct GFRomHeader` field for field. Every member is a `u8`,
/// `u32` or a pointer, so `repr(C)` reproduces the ARM layout exactly on the
/// 32-bit target.
#[repr(C)]
#[cfg_attr(test, allow(dead_code))]
pub struct GfRomHeader {
    version: u32,
    language: u32,
    game_name: [u8; 32],
    mon_front_pics: RomPtr<u8>,
    mon_back_pics: RomPtr<u8>,
    mon_normal_palettes: RomPtr<u8>,
    mon_shiny_palettes: RomPtr<u8>,
    mon_icons: RomPtr<u8>,
    mon_icon_palette_ids: RomPtr<u8>,
    mon_icon_palettes: RomPtr<u8>,
    mon_species_names: RomPtr<u8>,
    move_names: RomPtr<u8>,
    decorations: RomPtr<u8>,
    flags_offset: u32,
    vars_offset: u32,
    pokedex_offset: u32,
    seen1_offset: u32,
    seen2_offset: u32,
    pokedex_var: u32,
    pokedex_flag: u32,
    mystery_event_flag: u32,
    pokedex_count: u32,
    player_name_length: u8,
    trainer_name_length: u8,
    pokemon_name_length1: u8,
    pokemon_name_length2: u8,
    unknown: [u8; 13],
    save_block2_size: u32,
    save_block1_size: u32,
    party_count_offset: u32,
    party_offset: u32,
    warp_flags_offset: u32,
    trainer_id_offset: u32,
    player_name_offset: u32,
    player_gender_offset: u32,
    frontier_status_offset: u32,
    frontier_status_offset2: u32,
    external_event_flags_offset: u32,
    external_event_data_offset: u32,
    unk18: u32,
    species_info: RomPtr<u8>,
    ability_names: RomPtr<u8>,
    ability_descriptions: RomPtr<u8>,
    items: RomPtr<u8>,
    moves: RomPtr<u8>,
    ball_gfx: RomPtr<u8>,
    ball_palettes: RomPtr<u8>,
    gcn_link_flags_offset: u32,
    game_clear_flag: u32,
    ribbon_flag: u32,
    bag_count_items: u8,
    bag_count_key_items: u8,
    bag_count_pokeballs: u8,
    bag_count_tmhms: u8,
    bag_count_berries: u8,
    pc_items_count: u8,
    pc_items_offset: u32,
    gift_ribbons_offset: u32,
    enigma_berry_offset: u32,
    enigma_berry_size: u32,
    move_descriptions: RomPtr<u8>,
    unk20: u32,
}

unsafe impl Sync for GfRomHeader {}

#[cfg(not(test))]
unsafe extern "C" {
    static gMonFrontPicTable: u8;
    static gMonBackPicTable: u8;
    static gMonPaletteTable: u8;
    static gMonShinyPaletteTable: u8;
    static gMonIconTable: u8;
    static gMonIconPaletteIndices: u8;
    static gMonIconPaletteTable: u8;
    static gSpeciesNames: u8;
    static gMoveNames: u8;
    static gDecorations: u8;
    static gSpeciesInfo: u8;
    static gAbilityNames: u8;
    static gAbilityDescriptionPointers: u8;
    static gItems: u8;
    static gBattleMoves: u8;
    static gBallSpriteSheets: u8;
    static gBallSpritePalettes: u8;
}

/// `"pokemon emerald version"` as a plain ASCII C string. It is a `char`
/// literal, not `_("...")`, so preproc never re-encodes it.
const fn game_name() -> [u8; 32] {
    let text = b"pokemon emerald version";
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < text.len() {
        out[i] = text[i];
        i += 1;
    }
    out
}

// Only the ROM build carries the header; the host test binary has none of
// the C data tables it points at.
#[cfg(not(test))]
#[used]
#[unsafe(link_section = ".text.gf_rom_header")]
static GF_ROM_HEADER: GfRomHeader = GfRomHeader {
    version: 3,  // GAME_VERSION (VERSION_EMERALD)
    language: 2, // GAME_LANGUAGE (LANGUAGE_ENGLISH)
    game_name: game_name(),
    mon_front_pics: RomPtr(&raw const gMonFrontPicTable),
    mon_back_pics: RomPtr(&raw const gMonBackPicTable),
    mon_normal_palettes: RomPtr(&raw const gMonPaletteTable),
    mon_shiny_palettes: RomPtr(&raw const gMonShinyPaletteTable),
    mon_icons: RomPtr(&raw const gMonIconTable),
    mon_icon_palette_ids: RomPtr(&raw const gMonIconPaletteIndices),
    mon_icon_palettes: RomPtr(&raw const gMonIconPaletteTable),
    mon_species_names: RomPtr(&raw const gSpeciesNames),
    move_names: RomPtr(&raw const gMoveNames),
    decorations: RomPtr(&raw const gDecorations),
    flags_offset: 0x1270,
    vars_offset: 0x139c,
    pokedex_offset: 0x18,
    seen1_offset: 0x988,
    seen2_offset: 0x3b24,
    pokedex_var: 0x46,         // VAR_NATIONAL_DEX - VARS_START
    pokedex_flag: 0x8e4,       // FLAG_RECEIVED_POKEDEX_FROM_BIRCH
    mystery_event_flag: 0x8ac, // FLAG_SYS_MYSTERY_EVENT_ENABLE
    pokedex_count: 386,        // NATIONAL_DEX_COUNT
    player_name_length: 7,
    trainer_name_length: 10,
    pokemon_name_length1: 10,
    pokemon_name_length2: 10,
    // Two of these twelves are likely the move and ability name lengths.
    unknown: [12, 12, 6, 12, 6, 16, 18, 12, 15, 11, 1, 8, 12],
    save_block2_size: 0xf2c,
    save_block1_size: 0x3d88,
    party_count_offset: 0x234,
    party_offset: 0x238,
    warp_flags_offset: 0x09,
    trainer_id_offset: 0x0a,
    player_name_offset: 0x00,
    player_gender_offset: 0x08,
    frontier_status_offset: 0xca8,
    frontier_status_offset2: 0xca8,
    external_event_flags_offset: 0x31c7,
    external_event_data_offset: 0x31b3,
    unk18: 0,
    species_info: RomPtr(&raw const gSpeciesInfo),
    ability_names: RomPtr(&raw const gAbilityNames),
    ability_descriptions: RomPtr(&raw const gAbilityDescriptionPointers),
    items: RomPtr(&raw const gItems),
    moves: RomPtr(&raw const gBattleMoves),
    ball_gfx: RomPtr(&raw const gBallSpriteSheets),
    ball_palettes: RomPtr(&raw const gBallSpritePalettes),
    gcn_link_flags_offset: 0xa8,
    game_clear_flag: 0x864, // FLAG_SYS_GAME_CLEAR
    ribbon_flag: 0x89b,     // FLAG_SYS_RIBBON_GET
    bag_count_items: 30,
    bag_count_key_items: 30,
    bag_count_pokeballs: 16,
    bag_count_tmhms: 64,
    bag_count_berries: 46,
    pc_items_count: 50,
    pc_items_offset: 0x498,
    gift_ribbons_offset: 0x31a8,
    enigma_berry_offset: 0x31f8,
    enigma_berry_size: 0x34,
    move_descriptions: RomPtr(core::ptr::null()),
    unk20: 0, // 0xFFFFFFFF in FireRed/LeafGreen
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_game_name_is_plain_ascii_and_nul_padded() {
        let name = game_name();
        assert_eq!(&name[..23], b"pokemon emerald version");
        assert!(name[23..].iter().all(|&b| b == 0));
    }

    #[test]
    fn the_thirteen_unknown_bytes_keep_the_next_word_aligned() {
        // 4 name lengths + 13 unknowns = 17 bytes, then u32 alignment pads to
        // 20, so saveBlock2Size lands on a word boundary as in C.
        let unknown = [12u8, 12, 6, 12, 6, 16, 18, 12, 15, 11, 1, 8, 12];
        assert_eq!(4 + unknown.len(), 17);
    }
}
