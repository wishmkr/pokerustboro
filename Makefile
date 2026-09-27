# GBA rom header
TITLE       := POKEMON EMER
GAME_CODE   := BPEE
MAKER_CODE  := 01
REVISION    := 0
MODERN      ?= 0
KEEP_TEMPS  ?= 0

# `File name`.gba ('_modern' will be appended to the modern builds)
FILE_NAME := pokeemerald
BUILD_DIR := build

# Builds the ROM using a modern compiler
MODERN      ?= 0
# Compares the ROM to a checksum of the original - only makes sense using when non-modern
COMPARE     ?= 0

ifeq (modern,$(MAKECMDGOALS))
  MODERN := 1
endif
ifeq (compare,$(MAKECMDGOALS))
  COMPARE := 1
endif

# Default make rule
all: rom

# Toolchain selection
TOOLCHAIN := $(DEVKITARM)
# don't use dkP's base_tools anymore
# because the redefinition of $(CC) conflicts
# with when we want to use $(CC) to preprocess files
# thus, manually create the variables for the bin
# files, or use arm-none-eabi binaries on the system
# if dkP is not installed on this system
ifneq (,$(TOOLCHAIN))
  ifneq ($(wildcard $(TOOLCHAIN)/bin),)
    export PATH := $(TOOLCHAIN)/bin:$(PATH)
  endif
endif

PREFIX := arm-none-eabi-
OBJCOPY := $(PREFIX)objcopy
OBJDUMP := $(PREFIX)objdump
AS := $(PREFIX)as
LD := $(PREFIX)ld

EXE :=
ifeq ($(OS),Windows_NT)
  EXE := .exe
endif

# use arm-none-eabi-cpp for macOS
# as macOS's default compiler is clang
# and clang's preprocessor will warn on \u
# when preprocessing asm files, expecting a unicode literal
# we can't unconditionally use arm-none-eabi-cpp
# as installations which install binutils-arm-none-eabi
# don't come with it
ifneq ($(MODERN),1)
  ifeq ($(shell uname -s),Darwin)
    CPP := $(PREFIX)cpp
  else
    CPP := $(CC) -E
  endif
else
  CPP := $(PREFIX)cpp
endif

ROM_NAME := $(FILE_NAME).gba
OBJ_DIR_NAME := $(BUILD_DIR)/emerald
MODERN_ROM_NAME := $(FILE_NAME)_modern.gba
MODERN_OBJ_DIR_NAME := $(BUILD_DIR)/modern
ASSETS_DIR_NAME := $(BUILD_DIR)/assets

ELF_NAME := $(ROM_NAME:.gba=.elf)
MAP_NAME := $(ROM_NAME:.gba=.map)
MODERN_ELF_NAME := $(MODERN_ROM_NAME:.gba=.elf)
MODERN_MAP_NAME := $(MODERN_ROM_NAME:.gba=.map)

# Pick our active variables
ifeq ($(MODERN),0)
  ROM := $(ROM_NAME)
  OBJ_DIR := $(OBJ_DIR_NAME)
else
  ROM := $(MODERN_ROM_NAME)
  OBJ_DIR := $(MODERN_OBJ_DIR_NAME)
endif
ELF := $(ROM:.gba=.elf)
MAP := $(ROM:.gba=.map)
SYM := $(ROM:.gba=.sym)

# Commonly used directories
C_SUBDIR = src
RUST_SUBDIR = rust
ASM_SUBDIR = asm
DATA_SRC_SUBDIR = src/data
DATA_ASM_SUBDIR = data
MID_SUBDIR = sound/songs/midi

C_BUILDDIR = $(OBJ_DIR)/$(C_SUBDIR)
ASM_BUILDDIR = $(OBJ_DIR)/$(ASM_SUBDIR)
DATA_ASM_BUILDDIR = $(OBJ_DIR)/$(DATA_ASM_SUBDIR)
MID_BUILDDIR = $(OBJ_DIR)/$(MID_SUBDIR)

SHELL := bash -o pipefail

# Set flags for tools
ASFLAGS := -mcpu=arm7tdmi --defsym MODERN=$(MODERN)

INCLUDE_DIRS := include
INCLUDE_CPP_ARGS := $(INCLUDE_DIRS:%=-iquote %)
INCLUDE_SCANINC_ARGS := $(INCLUDE_DIRS:%=-I %)

O_LEVEL ?= 2
CPPFLAGS := $(INCLUDE_CPP_ARGS) -Wno-trigraphs -DMODERN=$(MODERN)
ifeq ($(MODERN),0)
  CPPFLAGS += -I tools/agbcc/include -I tools/agbcc -nostdinc -undef -std=gnu89
  CC1 := tools/agbcc/bin/agbcc$(EXE)
  override CFLAGS += -mthumb-interwork -Wimplicit -Wparentheses -Werror -O$(O_LEVEL) -fhex-asm -g
  LIBPATH := -L ../../tools/agbcc/lib
  LIB := $(LIBPATH) -lgcc -lc -L../../libagbsyscall -lagbsyscall
else
  # Note: The makefile must be set up to not call these if modern == 0
  MODERNCC := $(PREFIX)gcc
  PATH_MODERNCC := PATH="$(PATH)" $(MODERNCC)
  CC1 := $(shell $(PATH_MODERNCC) --print-prog-name=cc1) -quiet
  override CFLAGS += -mthumb -mthumb-interwork -O$(O_LEVEL) -mabi=apcs-gnu -mtune=arm7tdmi -march=armv4t -fno-toplevel-reorder -Wno-pointer-to-int-cast
  LIBPATH := -L "$(dir $(shell $(PATH_MODERNCC) -mthumb -print-file-name=libgcc.a))" -L "$(dir $(shell $(PATH_MODERNCC) -mthumb -print-file-name=libnosys.a))" -L "$(dir $(shell $(PATH_MODERNCC) -mthumb -print-file-name=libc.a))"
  # Only the Rust runtime: compiler_builtins provides the ARM EABI helpers
  # (division, soft float) and memcpy/memset; BIOS calls are rust/src/syscall.rs.
  LIB := src/rust_builtins.a
endif
# Enable debug info if set
ifeq ($(DINFO),1)
  override CFLAGS += -g
endif

# Variable filled out in other make files
AUTO_GEN_TARGETS :=
include make_tools.mk
# Tool executables
GFX       := $(TOOLS_DIR)/gbagfx/gbagfx$(EXE)
WAV2AGB   := $(TOOLS_DIR)/wav2agb/wav2agb$(EXE)
MID       := $(TOOLS_DIR)/mid2agb/mid2agb$(EXE)
SCANINC   := $(TOOLS_DIR)/scaninc/scaninc$(EXE)
PREPROC   := $(TOOLS_DIR)/preproc/preproc$(EXE)
RAMSCRGEN := $(TOOLS_DIR)/ramscrgen/ramscrgen$(EXE)
FIX       := $(TOOLS_DIR)/gbafix/gbafix$(EXE)
MAPJSON   := $(TOOLS_DIR)/mapjson/mapjson$(EXE)
JSONPROC  := $(TOOLS_DIR)/jsonproc/jsonproc$(EXE)

PERL := perl
SHA1 := $(shell { command -v sha1sum || command -v shasum; } 2>/dev/null) -c

MAKEFLAGS += --no-print-directory

# Clear the default suffixes
.SUFFIXES:
# Don't delete intermediate files
.SECONDARY:
# Delete files that weren't built properly
.DELETE_ON_ERROR:

RULES_NO_SCAN += libagbsyscall clean clean-assets tidy tidymodern tidynonmodern generated clean-generated
.PHONY: all rom modern compare
.PHONY: $(RULES_NO_SCAN)

infoshell = $(foreach line, $(shell $1 | sed "s/ /__SPACE__/g"), $(info $(subst __SPACE__, ,$(line))))

# Check if we need to scan dependencies based on the chosen rule OR user preference
NODEP ?= 0
# Check if we need to pre-build tools and generate assets based on the chosen rule.
SETUP_PREREQS ?= 1
# Disable dependency scanning for rules that don't need it.
ifneq (,$(MAKECMDGOALS))
  ifeq (,$(filter-out $(RULES_NO_SCAN),$(MAKECMDGOALS)))
    NODEP := 1
    SETUP_PREREQS := 0
  endif
endif

.SHELLSTATUS ?= 0

ifeq ($(SETUP_PREREQS),1)
  # If set on: Default target or a rule requiring a scan
  # Forcibly execute `make tools` since we need them for what we are doing.
  $(foreach line, $(shell $(MAKE) -f make_tools.mk | sed "s/ /__SPACE__/g"), $(info $(subst __SPACE__, ,$(line))))
  ifneq ($(.SHELLSTATUS),0)
    $(error Errors occurred while building tools. See error messages above for more details)
  endif
  # Oh and also generate mapjson sources before we use `SCANINC`.
  $(foreach line, $(shell $(MAKE) generated | sed "s/ /__SPACE__/g"), $(info $(subst __SPACE__, ,$(line))))
  ifneq ($(.SHELLSTATUS),0)
    $(error Errors occurred while generating map-related sources. See error messages above for more details)
  endif
endif

# Collect sources
C_SRCS_IN := $(wildcard $(C_SUBDIR)/*.c $(C_SUBDIR)/*/*.c $(C_SUBDIR)/*/*/*.c)
# C modules that have been replaced by the Rust crate. Their objects must
# never be re-added to the link; rust.o provides the same C ABI symbols.
RUST_PORTED_C :=\
	random.c\
	math_util.c\
	io_reg.c\
	trig.c\
	heal_location.c\
	coord_event_weather.c\
	dynamic_placeholder_text_util.c\
	play_time.c\
	birch_pc.c\
	coins.c\
	time_events.c\
	hof_pc.c\
	save_location.c\
	fldeff_teleport.c\
	fldeff_strength.c\
	fldeff_dig.c\
	reload_save.c\
	give_gift_ribbon_to_party.c\
	clock.c\
	post_battle_event_funcs.c\
	gym_leader_rematch.c\
	field_poison.c\
	field_message_box.c\
	wonder_news.c\
	lottery_corner.c\
	decoration_inventory.c\
	pokemon_size_record.c\
	fldeff_sweetscent.c\
	fldeff_softboiled.c\
	event_object_lock.c\
	mail_data.c\
	fldeff_rocksmash.c\
	money.c\
	event_data.c\
	berry_powder.c\
	confetti_util.c\
	international_string_util.c\
	task.c\
	dma3_manager.c\
	malloc.c\
	gpu_regs.c\
	util.c\
	blit.c\
	string_util.c\
	window.c\
	bg.c\
	sprite.c\
	trader.c\
	roamer.c\
	walda_phrase.c\
	fldeff_escalator.c\
	safari_zone.c\
	script_movement.c\
	script_pokemon_util.c\
	load_save.c\
	rom_header_gf.c\
	mystery_gift_link.c\
	mystery_gift_client.c\
	mystery_gift_server.c\
	text_window.c\
	pokedex_area_region_map.c\
	new_game.c\
	clear_save_data_screen.c\
	field_region_map.c\
	diploma.c\
	union_room_battle.c\
	battle_anim_smokescreen.c\
	battle_util2.c\
	battle_palace.c\
	bard_music.c\
	strings.c\
	mystery_event_msg.c\
	text_input_strings.c\
	fonts.c\
	mystery_gift_scripts.c\
	m4a_tables.c\
	agb_flash_le.c\
	tilesets.c\
	anim_mon_front_pics.c\
	data.c\
	graphics.c\
	text.c\
	landmark.c\
	item_icon.c\
	berry_fix_graphics.c\
	braille.c\
	scanline_effect.c\
	map_name_popup.c\
	reshow_battle_screen.c\
	libisagbprn.c\
	mini_printf.c\
	rotating_tile_puzzle.c\
	mystery_event_menu.c\
	decompress.c\
	rtc.c\
	trainer_pokemon_sprites.c\
	save_failed_screen.c\
	dewford_trend.c\
	mystery_event_script.c\
	field_special_scene.c\
	main.c\
	script.c\
	field_camera.c\
	battle_anim_poison.c\
	contest_link_util.c\
	berry_fix_program.c\
	battle_anim_sound_tasks.c\
	battle_tent.c\
	digit_obj_util.c\
	battle_anim_dragon.c\
	menu_helpers.c\
	faraway_island.c\
	wireless_communication_status_screen.c\
	battle_anim_bug.c\
	palette_util.c\
	pokenav_menu_handler.c\
	pokenav_match_call_list.c\
	battle_records.c\
	ereader_screen.c\
	battle_intro.c\
	battle_anim_status_effects.c\
	contest_link.c\
	field_door.c\
	pokedex_cry_screen.c\
	pokenav.c\
	contest_painting.c\
	union_room_player_avatar.c\
	mon_markings.c\
	pokenav_conditions.c\
	sound.c\
	item_menu_icons.c\
	mystery_gift.c\
	starter_choose.c\
	option_menu.c\
	battle_transition_frontier.c\
	battle_controller_safari.c\
	evolution_graphics.c\
	berry_tag_screen.c\
	reset_rtc_screen.c\
	pokenav_region_map.c\
	pokenav_conditions_search_results.c\
	pokenav_ribbons_list.c\
	mail.c\
	minigame_countdown.c\
	script_menu.c\
	battle_anim_ground.c\
	lilycove_lady.c\
	mirage_tower.c\
	pokedex_area_screen.c\
	battle_arena.c\
	pokenav_main_menu.c\
	title_screen.c\
	battle_anim_rock.c\
	pokenav_conditions_gfx.c\
	battle_factory.c\
	battle_ai_switch_items.c\
	egg_hatch.c\
	field_tasks.c\
	move_relearner.c\
	link_rfu_3.c\
	battle_anim_dark.c\
	field_control_avatar.c\
	rotating_gate.c\
	battle_anim_fight.c\
	palette.c\
	battle_anim_mon_movement.c\
	bike.c\
	trainer_hill.c\
	contest_effect.c\
	wallclock.c\
	battle_anim_normal.c\
	item_use.c\
	pokenav_match_call_data.c\
	battle_anim_psychic.c\
	intro_credits_graphics.c\
	tileset_anims.c\
	pokeblock_feed.c\
	image_processing_effects.c\
	battle_anim_flying.c\
	field_screen_effect.c\
	pokenav_ribbons_summary.c\
	field_weather.c\
	fieldmap.c\
	daycare.c\
	pokenav_match_call_gfx.c\
	pokemon_icon.c\
	apprentice.c\
	cable_club.c\
	battle_anim_electric.c\
	battle_anim_ghost.c\
	battle_gfx_sfx_util.c\
	berry.c\
	battle_anim_fire.c\
	pokenav_menu_handler_gfx.c\
	AgbRfu_LinkManager.c\
	metatile_behavior.c\
	battle_bg.c\
	start_menu.c\
	pokeblock.c\
	player_pc.c\
	hall_of_fame.c\
	battle_controller_wally.c\
	battle_controllers.c\
	battle_anim_water.c\
	battle_tv.c\
	battle_anim_ice.c\
	mystery_gift_menu.c\
	credits.c\
	battle_pike.c\
	use_pokeblock.c\
	evolution_scene.c\
	battle_controller_link_partner.c\
	field_effect_helpers.c\
	frontier_pass.c\
	contest_ai.c\
	battle_controller_recorded_opponent.c\
	battle_controller_recorded_player.c\
	battle_anim.c\
	battle_controller_link_opponent.c\
	battle_setup.c\
	battle_controller_player_partner.c\
	battle_controller_opponent.c\
	secret_base.c\
	match_call.c\
	field_player_avatar.c\
	battle_ai_script_commands.c\
	scrcmd.c\
	battle_anim_throw.c\
	battle_anim_mons.c\
	item_menu.c\
	battle_interface.c\
	frontier_util.c\
	field_weather_effect.c\
	decoration.c\
	battle_message.c\
	battle_controller_player.c\
	rayquaza_scene.c\
	union_room_chat.c\
	berry_crush.c\
	battle_anim_effects_2.c\
	berry_blender.c\
	field_effect.c\
	battle_util.c\
	pokemon_summary_screen.c\
	pokemon_jump.c\
	union_room.c\
	trade.c\
	battle_main.c\
	battle_anim_effects_3.c\
	battle_anim_effects_1.c\
	pokedex.c\
	battle_dome.c\
	menu.c\
	intro.c\
	pokeball.c\
	main_menu.c\
	naming_screen.c\
	battle_anim_utility_funcs.c\
	battle_factory_screen.c\
	battle_script_commands.c\
	battle_tower.c\
	battle_transition.c\
	braille_puzzles.c\
	cable_car.c\
	contest.c\
	contest_util.c\
	dodrio_berry_picking.c\
	easy_chat.c\
	ereader_helpers.c\
	field_specials.c\
	fldeff_cut.c\
	fldeff_flash.c\
	fldeff_misc.c\
	link.c\
	link_rfu_2.c\
	mystery_gift_view.c\
	pokemon.c\
	pokemon_animation.c\
	pokemon_storage_system.c\
	pokenav_list.c\
	recorded_battle.c\
	roulette.c\
	save.c\
	shop.c\
	slot_machine.c\
	trainer_card.c\
	trainer_see.c\
	tv.c\
	menu_specialized.c\
	list_menu.c\
	overworld.c\
	battle_pyramid.c\
	battle_pyramid_bag.c\
	event_object_movement.c\
	mauville_old_man.c\
	party_menu.c\
	record_mixing.c\
	wild_encounter.c\
	item.c\
	region_map.c\
	siirtc.c\
	agb_flash.c\
	agb_flash_mx.c\
	agb_flash_1m.c\
	librfu_stwi.c\
	librfu_sio32id.c\
	librfu_intr.c\
	librfu_rfu.c\
	multiboot.c\
	m4a.c

C_SRCS := $(filter-out $(RUST_PORTED_C:%=$(C_SUBDIR)/%),$(foreach src,$(C_SRCS_IN),$(if $(findstring .inc.c,$(src)),,$(src))))
C_OBJS := $(patsubst $(C_SUBDIR)/%.c,$(C_BUILDDIR)/%.o,$(C_SRCS))

CARGO ?= cargo
RUST_TARGET := thumbv4t-none-eabi
RUST_SRCS := $(shell find $(RUST_SUBDIR)/src -name "*.rs")
RUST_OBJS := $(C_BUILDDIR)/rust.o

# Assembly files whose code now lives in the Rust crate.
RUST_PORTED_ASM := rom_header.s crt0.s libgcnmultiboot.s
C_ASM_SRCS := $(filter-out $(RUST_PORTED_ASM:%=$(C_SUBDIR)/%),$(wildcard $(C_SUBDIR)/*.s $(C_SUBDIR)/*/*.s $(C_SUBDIR)/*/*/*.s))
C_ASM_OBJS := $(patsubst $(C_SUBDIR)/%.s,$(C_BUILDDIR)/%.o,$(C_ASM_SRCS))

ASM_SRCS := $(wildcard $(ASM_SUBDIR)/*.s)
ASM_OBJS := $(patsubst $(ASM_SUBDIR)/%.s,$(ASM_BUILDDIR)/%.o,$(ASM_SRCS))

DATA_ASM_SRCS := $(wildcard $(DATA_ASM_SUBDIR)/*.s)
DATA_ASM_OBJS := $(patsubst $(DATA_ASM_SUBDIR)/%.s,$(DATA_ASM_BUILDDIR)/%.o,$(DATA_ASM_SRCS))

MID_SRCS := $(wildcard $(MID_SUBDIR)/*.mid)
MID_OBJS := $(patsubst $(MID_SUBDIR)/%.mid,$(MID_BUILDDIR)/%.o,$(MID_SRCS))

OBJS     := $(C_OBJS) $(RUST_OBJS) $(C_ASM_OBJS) $(ASM_OBJS) $(DATA_ASM_OBJS) $(MID_OBJS)
OBJS_REL := $(patsubst $(OBJ_DIR)/%,%,$(OBJS))

SUBDIRS  := $(sort $(dir $(OBJS)))
$(shell mkdir -p $(SUBDIRS))

# Pretend rules that are actually flags defer to `make all`
modern: all
compare: all

# Other rules
rom: $(ROM)
ifeq ($(COMPARE),1)
	@$(SHA1) rom.sha1
endif

syms: $(SYM)

clean: tidy clean-tools clean-generated clean-assets
	@$(MAKE) clean -C libagbsyscall

clean-assets:
	rm -rf $(ASSETS_DIR_NAME)
	rm -f $(MID_SUBDIR)/*.s
	rm -f $(DATA_ASM_SUBDIR)/layouts/layouts.inc $(DATA_ASM_SUBDIR)/layouts/layouts_table.inc
	rm -f $(DATA_ASM_SUBDIR)/maps/connections.inc $(DATA_ASM_SUBDIR)/maps/events.inc $(DATA_ASM_SUBDIR)/maps/groups.inc $(DATA_ASM_SUBDIR)/maps/headers.inc
	find sound -iname '*.bin' -exec rm {} +
	find . \( -iname '*.1bpp' -o -iname '*.4bpp' -o -iname '*.8bpp' -o -iname '*.gbapal' -o -iname '*.lz' -o -iname '*.rl' -o -iname '*.latfont' -o -iname '*.hwjpnfont' -o -iname '*.fwjpnfont' \) -exec rm {} +
	find $(DATA_ASM_SUBDIR)/maps \( -iname 'connections.inc' -o -iname 'events.inc' -o -iname 'header.inc' \) -exec rm {} +

tidy: tidynonmodern tidymodern

tidynonmodern:
	rm -f $(ROM_NAME) $(ELF_NAME) $(MAP_NAME)
	rm -rf $(OBJ_DIR_NAME)

tidymodern:
	rm -f $(MODERN_ROM_NAME) $(MODERN_ELF_NAME) $(MODERN_MAP_NAME)
	rm -rf $(MODERN_OBJ_DIR_NAME)

# Other rules
include graphics_file_rules.mk
include rust_assets.mk
include map_data_rules.mk
include json_data_rules.mk
include audio_rules.mk

# NOTE: Tools must have been built prior (FIXME)
# so you can't really call this rule directly
generated: $(AUTO_GEN_TARGETS)
	@: # Silence the "Nothing to be done for `generated'" message, which some people were confusing for an error.


%.s:   ;
%.png: ;
%.pal: ;
%.wav: ;

%.1bpp:   %.png  ; $(GFX) $< $@
%.4bpp:   %.png  ; $(GFX) $< $@
%.8bpp:   %.png  ; $(GFX) $< $@
%.gbapal: %.pal  ; $(GFX) $< $@
%.gbapal: %.png  ; $(GFX) $< $@
%.lz:     %      ; $(GFX) $< $@
%.rl:     %      ; $(GFX) $< $@

clean-generated:
	@rm -f $(AUTO_GEN_TARGETS)
	@echo "rm -f <AUTO_GEN_TARGETS>"

ifeq ($(MODERN),0)
$(C_BUILDDIR)/libc.o: CC1 := $(TOOLS_DIR)/agbcc/bin/old_agbcc$(EXE)
$(C_BUILDDIR)/libc.o: CFLAGS := -O2
$(C_BUILDDIR)/siirtc.o: CFLAGS := -mthumb-interwork
$(C_BUILDDIR)/agb_flash.o: CFLAGS := -O -mthumb-interwork
$(C_BUILDDIR)/agb_flash_1m.o: CFLAGS := -O -mthumb-interwork
$(C_BUILDDIR)/agb_flash_mx.o: CFLAGS := -O -mthumb-interwork
$(C_BUILDDIR)/m4a.o: CC1 := tools/agbcc/bin/old_agbcc$(EXE)
$(C_BUILDDIR)/record_mixing.o: CFLAGS += -ffreestanding
$(C_BUILDDIR)/librfu_intr.o: CC1 := $(TOOLS_DIR)/agbcc/bin/agbcc_arm$(EXE)
$(C_BUILDDIR)/librfu_intr.o: CFLAGS := -O2 -mthumb-interwork -quiet
else
$(C_BUILDDIR)/librfu_intr.o: CFLAGS := -mthumb-interwork -O2 -mabi=apcs-gnu -mtune=arm7tdmi -march=armv4t -fno-toplevel-reorder -Wno-pointer-to-int-cast
$(C_BUILDDIR)/berry_crush.o: override CFLAGS += -Wno-address-of-packed-member
endif

# Dependency rules (for the *.c & *.s sources to .o files)
# Have to be explicit or else missing files won't be reported.

# As a side effect, they're evaluated immediately instead of when the rule is invoked.
# It doesn't look like $(shell) can be deferred so there might not be a better way (Icedude_907: there is soon).

$(C_BUILDDIR)/rust.o: $(RUST_SRCS) $(RUST_SUBDIR)/Cargo.toml $(RUST_ASSETS)
	@echo "$(CARGO) rustc <flags> -o $@"
	@RUSTC_BOOTSTRAP=1 $(CARGO) rustc --manifest-path $(RUST_SUBDIR)/Cargo.toml \
		-Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem \
		--release --target $(RUST_TARGET) -- \
		-Zmerge-functions=disabled \
		--emit=obj=$(abspath $@)

$(C_BUILDDIR)/%.o: $(C_SUBDIR)/%.c
ifneq ($(KEEP_TEMPS),1)
	@echo "$(CC1) <flags> -o $@ $<"
	@$(CPP) $(CPPFLAGS) $< | $(PREPROC) -i -g $(ASSETS_DIR_NAME) $< charmap.txt | $(CC1) $(CFLAGS) -o - - | cat - <(echo -e ".text\n\t.align\t2, 0") | $(AS) $(ASFLAGS) -o $@ -
else
	@$(CPP) $(CPPFLAGS) $< -o $(C_BUILDDIR)/$*.i
	@$(PREPROC) -g $(ASSETS_DIR_NAME) $(C_BUILDDIR)/$*.i charmap.txt | $(CC1) $(CFLAGS) -o $(C_BUILDDIR)/$*.s
	@echo -e ".text\n\t.align\t2, 0\n" >> $(C_BUILDDIR)/$*.s
	$(AS) $(ASFLAGS) -o $@ $(C_BUILDDIR)/$*.s
endif

$(C_BUILDDIR)/%.d: $(C_SUBDIR)/%.c
	$(SCANINC) -M $@ -g $(ASSETS_DIR_NAME) $(INCLUDE_SCANINC_ARGS) -I tools/agbcc/include $<

ifneq ($(NODEP),1)
-include $(addprefix $(OBJ_DIR)/,$(C_SRCS:.c=.d))
endif

$(ASM_BUILDDIR)/%.o: $(ASM_SUBDIR)/%.s
	$(AS) $(ASFLAGS) -o $@ $<

$(ASM_BUILDDIR)/%.d: $(ASM_SUBDIR)/%.s
	$(SCANINC) -M $@ -g $(ASSETS_DIR_NAME) $(INCLUDE_SCANINC_ARGS) -I "" $<

ifneq ($(NODEP),1)
-include $(addprefix $(OBJ_DIR)/,$(ASM_SRCS:.s=.d))
endif

$(C_BUILDDIR)/%.o: $(C_SUBDIR)/%.s
	$(PREPROC) $< charmap.txt | $(CPP) $(INCLUDE_SCANINC_ARGS) - | $(PREPROC) -ie $< charmap.txt | $(AS) $(ASFLAGS) -o $@

# m4a_1.s still provides the mixer (SoundMain, SoundMainRAM); its sequencer
# routines are Rust now (m4a_engine.rs), so the assembly's copies are made weak.
M4A_1_RUST := umul3232H32 SoundMainBTM RealClearChain ply_fine MPlayJumpTableCopy \
	ply_goto ply_patt ply_pend ply_rept ply_prio ply_tempo ply_keysh ply_voice ply_vol \
	ply_pan ply_bend ply_bendr ply_lfodl ply_modt ply_tune ply_port m4aSoundVSync MPlayMain \
	TrackStop ChnVolSetAsm ply_note ply_endtie clear_modM ply_lfos ply_mod
$(C_BUILDDIR)/m4a_1.o: $(C_SUBDIR)/m4a_1.s
	$(PREPROC) $< charmap.txt | $(CPP) $(INCLUDE_SCANINC_ARGS) - | $(PREPROC) -ie $< charmap.txt | $(AS) $(ASFLAGS) -o $@
	$(OBJCOPY) $(addprefix --weaken-symbol=,$(M4A_1_RUST)) $@

$(C_BUILDDIR)/%.d: $(C_SUBDIR)/%.s
	$(SCANINC) -M $@ -g $(ASSETS_DIR_NAME) $(INCLUDE_SCANINC_ARGS) -I "" $<

ifneq ($(NODEP),1)
-include $(addprefix $(OBJ_DIR)/,$(C_ASM_SRCS:.s=.d))
endif

$(DATA_ASM_BUILDDIR)/%.o: $(DATA_ASM_SUBDIR)/%.s
	$(PREPROC) $< charmap.txt | $(CPP) $(INCLUDE_SCANINC_ARGS) - | $(PREPROC) -ie $< charmap.txt | $(AS) $(ASFLAGS) -o $@

$(DATA_ASM_BUILDDIR)/%.d: $(DATA_ASM_SUBDIR)/%.s
	$(SCANINC) -M $@ -g $(ASSETS_DIR_NAME) $(INCLUDE_SCANINC_ARGS) -I "" $<

ifneq ($(NODEP),1)
-include $(addprefix $(OBJ_DIR)/,$(DATA_ASM_SRCS:.s=.d))
endif

$(OBJ_DIR)/sym_bss.ld: sym_bss.txt
	$(RAMSCRGEN) .bss $< ENGLISH > $@

$(OBJ_DIR)/sym_common.ld: sym_common.txt $(C_OBJS) $(RUST_OBJS) $(wildcard common_syms/*.txt)
	$(RAMSCRGEN) COMMON $< ENGLISH -c $(C_BUILDDIR),common_syms > $@

$(OBJ_DIR)/sym_ewram.ld: sym_ewram.txt
	$(RAMSCRGEN) ewram_data $< ENGLISH > $@

# Linker script
ifeq ($(MODERN),0)
LD_SCRIPT := ld_script.ld
LD_SCRIPT_DEPS := $(OBJ_DIR)/sym_bss.ld $(OBJ_DIR)/sym_common.ld $(OBJ_DIR)/sym_ewram.ld
else
LD_SCRIPT := ld_script_modern.ld
LD_SCRIPT_DEPS :=
endif

# Final rules

libagbsyscall:
	@$(MAKE) -C libagbsyscall TOOLCHAIN=$(TOOLCHAIN) MODERN=$(MODERN)

# Elf from object files
LDFLAGS = -Map ../../$(MAP)
# compiler_builtins, built with the crate, as a plain archive for ld (its
# .rlib also carries Rust metadata that ld does not understand).
$(C_BUILDDIR)/rust_builtins.a: $(C_BUILDDIR)/rust.o
	@rm -rf $(C_BUILDDIR)/rust_builtins && mkdir -p $(C_BUILDDIR)/rust_builtins
	@cd $(C_BUILDDIR)/rust_builtins && $(PREFIX)ar x $(abspath $(shell ls -t $(RUST_SUBDIR)/target/$(RUST_TARGET)/release/deps/libcompiler_builtins-*.rlib | head -1))
	@rm -f $@ && $(PREFIX)ar rcs $@ $(C_BUILDDIR)/rust_builtins/*.o

$(ELF): $(LD_SCRIPT) $(LD_SCRIPT_DEPS) $(OBJS) $(C_BUILDDIR)/rust_builtins.a
	@cd $(OBJ_DIR) && $(LD) $(LDFLAGS) -T ../../$< --print-memory-usage -o ../../$@ $(OBJS_REL) $(LIB) | cat
	@echo "cd $(OBJ_DIR) && $(LD) $(LDFLAGS) -T ../../$< --print-memory-usage -o ../../$@ <objs> <libs> | cat"
	$(FIX) $@ -t"$(TITLE)" -c$(GAME_CODE) -m$(MAKER_CODE) -r$(REVISION) --silent

# Builds the rom from the elf file
$(ROM): $(ELF)
	$(OBJCOPY) -O binary $< $@
	$(FIX) $@ -p --silent

# Symbol file (`make syms`)
$(SYM): $(ELF)
	$(OBJDUMP) -t $< | sort -u | grep -E "^0[2389]" | $(PERL) -p -e 's/^(\w{8}) (\w).{6} \S+\t(\w{8}) (\S+)$$/\1 \2 \3 \4/g' > $@
