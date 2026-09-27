# Generated graphics/binary assets that the Rust crate embeds with
# `include_bytes!`.
#
# In C these come from the INCGFX macros, and `scaninc` writes the build rules
# for them into each object's .d file. Rust has no such scanner, so every asset
# a Rust module embeds is listed here by hand, together with the gbagfx
# arguments the original INCGFX call used.
#
# Paths follow the same convention preproc uses:
#   INCGFX_U8("graphics/foo/bar.png", ".4bpp")
#     -> $(ASSETS_DIR_NAME)/graphics/foo/bar.png.4bpp
# and the Rust side includes it relative to rust/src/.

#
# `rust_gfx` declares one converted asset: $(1) is the output path below
# $(ASSETS_DIR_NAME), $(2) the source file, $(3) extra gbagfx arguments. It uses
# the same `ifndef` guard as scaninc's generated rules, so a C object that still
# needs the same asset does not redefine the recipe. This file is included
# before any .d file, so the Rust definition always wins.

define rust_gfx
RUST_ASSETS += $(ASSETS_DIR_NAME)/$(1)
ifndef $(ASSETS_DIR_NAME)/$(1)
$(ASSETS_DIR_NAME)/$(1) := defined
$(ASSETS_DIR_NAME)/$(1): $(2)
	@mkdir -p $$(@D)
	$$(GFX) $$< $$@ $(3)
endif
endef

# Compressed variants: the uncompressed file is an intermediate that gets
# its own rust_gfx entry, and the generic `%.lz: %` rule compresses it.
define rust_gfx_lz
$(call rust_gfx,$(1),$(2),$(3))
RUST_ASSETS += $(ASSETS_DIR_NAME)/$(1).lz
endef

RUST_ASSETS :=

$(eval $(call rust_gfx,graphics/interface/blank.png.4bpp,graphics/interface/blank.png))

# text_window
$(foreach n,1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20,\
  $(eval $(call rust_gfx,graphics/text_window/$(n).png.4bpp,graphics/text_window/$(n).png))\
  $(eval $(call rust_gfx,graphics/text_window/$(n).png.gbapal,graphics/text_window/$(n).png)))
$(eval $(call rust_gfx,graphics/text_window/message_box.png.gbapal,graphics/text_window/message_box.png))
$(foreach n,1 2 3 4,\
  $(eval $(call rust_gfx,graphics/text_window/text_pal$(n).pal.gbapal,graphics/text_window/text_pal$(n).pal)))

# pokedex_area_region_map
$(eval $(call rust_gfx,graphics/pokedex/region_map.pal.gbapal,graphics/pokedex/region_map.pal))
$(eval $(call rust_gfx_lz,graphics/pokedex/region_map.png_num_tiles_232__Wnum_tiles.8bpp,graphics/pokedex/region_map.png,-num_tiles 232 -Wnum_tiles))
$(eval $(call rust_gfx_lz,graphics/pokedex/region_map_affine.png_num_tiles_233__Wnum_tiles.8bpp,graphics/pokedex/region_map_affine.png,-num_tiles 233 -Wnum_tiles))
$(eval $(call rust_gfx,graphics/pokedex/region_map.bin.lz,graphics/pokedex/region_map.bin))
$(eval $(call rust_gfx,graphics/pokedex/region_map_affine.bin.lz,graphics/pokedex/region_map_affine.bin))

# diploma
$(eval $(call rust_gfx,graphics/diploma/national.pal.gbapal,graphics/diploma/national.pal))
$(eval $(call rust_gfx,graphics/diploma/hoenn.pal.gbapal,graphics/diploma/hoenn.pal))
$(eval $(call rust_gfx,graphics/diploma/tilemap.bin.lz,graphics/diploma/tilemap.bin))
$(eval $(call rust_gfx_lz,graphics/diploma/tiles.png.4bpp,graphics/diploma/tiles.png))

# Per-module fragments written by tools/rustport/cdata.py --mk.
include $(wildcard rust/assets/*.mk)
