# Changelog

## v0.5.0 (2026-10-02)

The game code now follows Rust's rules instead of C's, and the C-style port from v0.3.0 is kept next to it.

**Two versions in one repository**

| | `rust/`, Rust rules (main) | `rust-c-style/`, C style |
|---|---|---|
| Build | `make modern` | `make c-style` |
| ROM | `pokeemerald_modern.gba` | `pokeemerald_c_style.gba` |
| `extern "C"` | 0.3% of items (start-up code, BIOS calls, interrupt handlers) | nearly everything |
| Scripts, maps, songs | Rust statics | assembled `data/` objects |

**Main port (`rust/`)**

- Modules reach each other with `use`; functions have the Rust calling convention. Items using C's calling
  convention: 58% → 0.3%.
- Event and battle scripts, maps, songs and multiboot images are Rust statics (`rust/src/asmdata`), generated
  from the assembled data; every build checks all 48,317 addresses and every byte against the assembler's
  output. No assembled object is linked any more.
- `for` loops, variables declared where first set, task and sprite data slots with the C's names
  (`data[tState]`), globals with their real types (`(*gSaveBlock1Ptr).tvShows[i]`).
- Single-value globals are safe `Global<T>` cells; the task scheduler has safe accessors; 2,626 functions are
  safe Rust. About 35 small modules (money, coins, bag, Pokémon data, tasks, clock, flags and vars) are
  idiomatic, safe Rust.

**Both versions**

- Fixed: eggs were drawn mirrored on the summary screen.
- New automated check: 31 screens of Pokémon data (party, every summary page, PC, bag) compared with the C build
  from a generated save.

## v0.3.0 (2026-09-28)

- The game code is regenerated in a readable style: C structs are real Rust types with GCC's layout (checked at
  compile time), fields are reached by name, arrays index like C (`CArray`), constants keep their names.
- About 790k lines of Rust became 420k.
- Detailed README: setup on Linux, macOS and Windows (WSL), building, playing, tests, troubleshooting.

## v0.2.0 (2026-09-28)

- The sound mixer is Rust (it was the last hand-written assembly file of the game). All game code is Rust.

## v0.1.0 (2026-09-27)

- All 310 C files of pokeemerald ported to `no_std` Rust; no C is compiled into the ROM. The sound mixer is
  still assembly.
