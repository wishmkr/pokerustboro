ENTRY(Start)

/* Fixed places in the cartridge header (rom_header.rs). */
Start = 0x08000000;
RomHeaderNintendoLogo = 0x08000004;
RomHeaderGameCode = 0x080000AC;
RomHeaderSoftwareVersion = 0x080000BC;
GPIOPortData = 0x080000C4;
GPIOPortDirection = 0x080000C6;
GPIOPortReadEnable = 0x080000C8;

/* (used by the C-style Rust in rust-c-style/) */
gNumMusicPlayers = 4;
gMaxLines = 0;


MEMORY
{
    EWRAM (rwx) : ORIGIN = 0x2000000, LENGTH = 256K
    IWRAM (rwx) : ORIGIN = 0x3000000, LENGTH = 32K
    ROM    (rx) : ORIGIN = 0x8000000, LENGTH = 32M
}

SECTIONS {

    ewram 0x2000000 (NOLOAD) :
    ALIGN(4)
    {
        src/*.o(ewram_data);
    } > EWRAM

    iwram 0x3000000 (NOLOAD) :
    ALIGN(4)
    {
        /* Rust emits each BSS static into its own .bss.<symbol>
           subsection, so the glob has to cover them. m4a is excluded because
           its .bss.code is placed deliberately further down. */
        src/*.o(EXCLUDE_FILE(*m4a.o) .bss*);
        data/*.o(.bss*);
        *rust_builtins.a:*.o(.bss*);


        src/*.o(common_data);
        src/*.o(COMMON);
        *rust_builtins.a:*.o(COMMON);
    } > IWRAM

    /* BEGIN ROM DATA */
    . = 0x8000000;

    .text :
    ALIGN(4)
    {
        /* The cartridge header and start-up code are Rust (rom_header.rs, crt0.rs). */
        KEEP(src/rust.o(.text.rom_header));
        /* Game Freak's public ROM header, now emitted by the Rust crate. */
        src/rust.o(.text.gf_rom_header);
        src/*.o(.text*);
        asm/*.o(.text*);
    } > ROM =0

    script_data :
    ALIGN(4)
    {
		data/*.o(script_data);
    } > ROM =0

    lib_text :
    ALIGN(4)
    {
        *rust_builtins.a:*.o(.text*);
    } > ROM =0

    .rodata :
    ALIGN(4)
    {
        src/*.o(.rodata*);
        data/*.o(.rodata*);
    } > ROM =0

    song_data :
    ALIGN(4)
    {
        sound/songs/*.o(.rodata);
    } > ROM =0

    lib_rodata :
    SUBALIGN(4)
    {
        *rust_builtins.a:*.o(.rodata*);
    } > ROM =0

    multiboot_data :
    ALIGN(4)
    {
        data/multiboot_ereader.[o](.rodata);
        data/multiboot_berry_glitch_fix.[o](.rodata);
        data/multiboot_pokemon_colosseum.[o](.rodata);
    } > ROM =0

    anim_mon_front_pic_data :
    ALIGN(4)
    {
    } > ROM =0

    gfx_data :
    ALIGN(4)
    {
    } > ROM =0

    /* Code that runs from IWRAM (the sound mixer). Stored at the end of the
       ROM, copied to the end of the IWRAM variables by Init. */
    iwram_code :
    ALIGN(4)
    {
        __iwram_code_start = .;
        *(.iwram_code .iwram_code.*);
        . = ALIGN(4);
        __iwram_code_end = .;
    } > IWRAM AT> ROM
    __iwram_code_lma = LOADADDR(iwram_code);

    /* DWARF debug sections.
       Symbols in the DWARF debugging sections are relative to the beginning
       of the section so we begin them at 0.  */

    /* DWARF 1 */
    .debug          0 : { *(.debug) }
    .line           0 : { *(.line) }

    /* GNU DWARF 1 extensions */
    .debug_srcinfo  0 : { *(.debug_srcinfo) }
    .debug_sfnames  0 : { *(.debug_sfnames) }

    /* DWARF 1.1 and DWARF 2 */
    .debug_aranges  0 : { *(.debug_aranges) }
    .debug_pubnames 0 : { *(.debug_pubnames) }

    /* DWARF 2 */
    .debug_info     0 : { *(.debug_info .gnu.linkonce.wi.*) }
    .debug_abbrev   0 : { *(.debug_abbrev) }
    .debug_line     0 : { *(.debug_line) }
    .debug_frame    0 : { *(.debug_frame) }
    .debug_str      0 : { *(.debug_str) }
    .debug_loc      0 : { *(.debug_loc) }
    .debug_macinfo  0 : { *(.debug_macinfo) }

    /* DWARF 5*/
    .debug_line_str     0 : { *(.debug_line_str) }

    /* Discard everything not specifically mentioned above. */
    /DISCARD/ :
    {
        *(*);
    }
}
