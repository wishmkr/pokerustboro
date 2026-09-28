# PokeRustboro

**Pokémon Emerald, rewritten in Rust.** PokeRustboro is the [pret/pokeemerald](https://github.com/pret/pokeemerald)
decompilation with its entire game code ported from C to `no_std` Rust. It builds a real, playable Game Boy Advance
ROM that runs on emulators.

- **All of the game's code is Rust.** Every one of pokeemerald's 310 C files and its hand-written assembly (start-up
  code, interrupt handler, sound mixer, GameCube multiboot) is now Rust. No C is compiled into the ROM.
- **It plays the same game.** Battles, the overworld, menus, sound and saves behave like the C build; the port is
  checked against it by comparing screens, memory, sound engine state and save files.
- **The code is readable.** Structs are real Rust structs with the C field names, constants keep their names, and
  loops look like loops. See [What the code looks like](#what-the-code-looks-like).
- **Assets are untouched.** Graphics, maps, music, text and event scripts stay in pret's formats and are built by
  pret's tools.

> PokeRustboro is an unofficial fan project. It is not affiliated with Nintendo, Game Freak or The Pokémon Company.
> No ROM is distributed here: you build it yourself from this repository.

---

## Contents

- [Status](#status)
- [What the code looks like](#what-the-code-looks-like)
- [Quick start](#quick-start)
- [Detailed setup](#detailed-setup)
  - [1. System packages](#1-system-packages)
  - [2. devkitARM](#2-devkitarm)
  - [3. Rust](#3-rust)
  - [4. Get the source](#4-get-the-source)
  - [5. Build](#5-build)
- [Playing the ROM](#playing-the-rom)
- [Running the tests](#running-the-tests)
- [Project layout](#project-layout)
- [How the Rust code is organised](#how-the-rust-code-is-organised)
- [Differences from the original](#differences-from-the-original)
- [Troubleshooting](#troubleshooting)
- [Roadmap](#roadmap)
- [Credits and license](#credits-and-license)

---

## Status

| | |
|---|---|
| Game code in Rust | **310 / 310** C files, plus the 4 assembly files |
| C compiled into the ROM | **none** |
| Assembly left in the Rust code | 163 lines of inline `asm!`: BIOS calls (`svc`), the CPU mode switches of start-up and the interrupt handler, and one cycle-counted delay loop |
| Unit tests | 189 (`cargo test`) |
| Target | ARM7TDMI (`thumbv4t-none-eabi`), Game Boy Advance |
| Output | `pokeemerald_modern.gba` |

What is still C/C++ is **build tooling only**: pret's asset converters in `tools/` (`gbagfx`, `mid2agb`,
`wav2agb`, `preproc`, `mapjson`...) and the headers the data files are assembled with. None of it ends up in
the ROM. Replacing that tooling with Rust is on the [roadmap](#roadmap).

---

## What the code looks like

The C, from `src/tv.c`:

```c
for (i = 0; i < ARRAY_COUNT(gSaveBlock1Ptr->tvShows); i++)
{
    gSaveBlock1Ptr->tvShows[i].commonInit.kind = 0;
    gSaveBlock1Ptr->tvShows[i].commonInit.active = 0;
    for (j = 0; j < ARRAY_COUNT(gSaveBlock1Ptr->tvShows[i].commonInit.data); j++)
        gSaveBlock1Ptr->tvShows[i].commonInit.data[j] = 0;
}
ClearPokeNews();
```

The Rust, in `rust/src/field/tv.rs`:

```rust
i = 0;
while i < 25 {
    (*gSaveBlock1Ptr).tvShows[i].commonInit.kind = 0;
    (*gSaveBlock1Ptr).tvShows[i].commonInit.active = 0;
    j = 0;
    while j < 34 {
        (*gSaveBlock1Ptr).tvShows[i].commonInit.data[j] = 0;
        j += 1;
    }
    i += 1;
}
ClearPokeNews();
```

Constants keep their names:

```rust
show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
(*show).pokemonToday.kind = TVSHOW_POKEMON_TODAY_CAUGHT;
(*show).pokemonToday.active = FALSE;
if gBattleResults.usedMasterBall() != 0 {
    ballsUsed = 1;
    itemLastUsed = ITEM_MASTER_BALL;
}
```

The functions keep their C names and the C ABI, so the code reads side by side with pokeemerald and its
documentation.

---

## Quick start

On Debian/Ubuntu (other systems: see [Detailed setup](#detailed-setup)):

```sh
# 1. system packages
sudo apt install build-essential git libpng-dev

# 2. devkitARM (the GBA assembler and linker)
wget https://apt.devkitpro.org/install-devkitpro-pacman
chmod +x ./install-devkitpro-pacman
sudo ./install-devkitpro-pacman
sudo dkp-pacman -S gba-dev
export DEVKITPRO=/opt/devkitpro
export DEVKITARM=$DEVKITPRO/devkitARM

# 3. Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add rust-src

# 4. build
git clone https://github.com/wishmkr/pokerustboro
cd pokerustboro
make tools
make modern -j4
```

The ROM is `pokeemerald_modern.gba`.

---

## Detailed setup

You need four things: a C compiler with `make` and `libpng` (for pret's asset tools), **devkitARM** (the
assembler and linker for the GBA), **Rust** with the `rust-src` component, and this repository.

### 1. System packages

<details open>
<summary><b>Debian / Ubuntu / Linux Mint</b></summary>

```sh
sudo apt update
sudo apt install build-essential git libpng-dev wget
```
</details>

<details>
<summary><b>Fedora</b></summary>

```sh
sudo dnf install gcc gcc-c++ make git libpng-devel wget
```
</details>

<details>
<summary><b>Arch Linux / Manjaro</b></summary>

```sh
sudo pacman -S base-devel git libpng wget
```
</details>

<details>
<summary><b>macOS</b></summary>

Install the Xcode command line tools and [Homebrew](https://brew.sh), then libpng:

```sh
xcode-select --install
brew install libpng git
```
</details>

<details>
<summary><b>Windows</b></summary>

Build inside **WSL** (Windows Subsystem for Linux) with Ubuntu, then follow the Debian/Ubuntu instructions in
the WSL terminal. In PowerShell as Administrator:

```powershell
wsl --install -d Ubuntu
```

Restart, open *Ubuntu* from the Start menu and continue there.

- Keep the repository **inside the Linux file system** (for example `~/pokerustboro`), not under `/mnt/c/`:
  building on the Windows drive is many times slower.
- pret's [INSTALL.md](INSTALL.md) has more detail on WSL1, msys2 and Cygwin if you need them.
</details>

### 2. devkitARM

devkitARM provides `arm-none-eabi-as` and `arm-none-eabi-ld`, which assemble the game's data and link the ROM,
and the GCC that pret's build rules use to preprocess assembly.

<details open>
<summary><b>Debian / Ubuntu / WSL</b></summary>

```sh
wget https://apt.devkitpro.org/install-devkitpro-pacman
chmod +x ./install-devkitpro-pacman
sudo ./install-devkitpro-pacman
sudo dkp-pacman -S gba-dev
```
</details>

<details>
<summary><b>Arch Linux</b></summary>

Add the devkitPro repositories as described in the
[devkitPro pacman guide](https://devkitpro.org/wiki/devkitPro_pacman), then:

```sh
sudo pacman -S gba-dev
```
</details>

<details>
<summary><b>macOS, Fedora and other systems</b></summary>

Follow the [devkitPro Getting Started guide](https://devkitpro.org/wiki/Getting_Started) for your system (macOS
has a `.pkg` installer), then install the `gba-dev` group with `dkp-pacman`.
</details>

Then tell the build where it is. Add these lines to your `~/.bashrc` (or `~/.zshrc`) so they are set in every
terminal:

```sh
export DEVKITPRO=/opt/devkitpro
export DEVKITARM=$DEVKITPRO/devkitARM
```

Check it:

```sh
$DEVKITARM/bin/arm-none-eabi-ld --version
```

### 3. Rust

Install Rust with [rustup](https://rustup.rs):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

The GBA's CPU (`thumbv4t-none-eabi`) has no prebuilt standard library, so the build compiles `core` itself from
source. That needs the `rust-src` component:

```sh
rustup component add rust-src
```

A **stable** toolchain is enough: Rust **1.88 or newer** (the project is tested with 1.98). The Makefile sets
`RUSTC_BOOTSTRAP=1` for the one unstable flag it needs (`-Zbuild-std`), so no nightly toolchain is required.

Check it:

```sh
rustc --version
rustup component list --installed | grep rust-src
```

### 4. Get the source

```sh
git clone https://github.com/wishmkr/pokerustboro
cd pokerustboro
```

The repository includes the game's assets (graphics, music, maps) in pret's source formats; you don't need a
ROM of the original game to build.

### 5. Build

```sh
make tools        # once: builds pret's asset converters (gbagfx, mid2agb, preproc, ...)
make modern -j4   # builds the ROM
```

- The result is **`pokeemerald_modern.gba`** in the repository root.
- Use `make modern`, not plain `make`: plain `make` is pret's build with the old agbcc compiler, which isn't
  part of this project.
- The first build converts every asset and compiles the Rust crate (including `core`), so it takes a few
  minutes. Later builds only redo what changed.
- `-j4` runs four jobs at once. Compiling the crate needs a few GB of RAM; if the build gets killed, use `-j2`.
- To start over: `make tidymodern` removes the ROM and the objects; `make clean` also removes converted assets
  and the tools.

---

## Playing the ROM

**Emulators.** [mGBA](https://mgba.io) is recommended (also available as the mGBA core in RetroArch). Open
`pokeemerald_modern.gba` directly.

**Saves.** The game saves to 128 KB flash, like the cartridge; emulators detect this automatically. The save
format is pokeemerald's, so `.sav` files are interchangeable with a pokeemerald build.

> Back up any save you care about before loading it into a development build.

**Hardware.** The ROM is a normal GBA ROM and should run from a flash cartridge on a Game Boy Advance, GBA SP,
Game Boy Micro or Nintendo DS (GBA slot), with the save type set to *Flash 128K* (1 Mbit). The port has so far
been tested on emulators only; reports from real hardware are welcome.

---

## Running the tests

The unit tests run on your computer (they don't need the GBA target):

```sh
cargo test --manifest-path rust/Cargo.toml
```

They are spread over about 70 modules and cover parts of the engine that can run without the GBA: string and
text utilities, random numbers, math and trigonometry, the heap allocator, the task scheduler, sprite, background
and window helpers, money and coins, event flags and similar.

Behaviour as a whole was checked during the port by running the Rust ROM and the C ROM side by side in a
headless emulator: comparing IO registers, palettes, VRAM and OAM frame by frame through the intro, screenshots
through the new-game sequence, sound engine state over thousands of frames, and saves written by one build and
loaded by the other.

---

## Project layout

| Path | What |
|---|---|
| `rust/src/` | **The game.** One Rust module per original C file, grouped by subject: |
| &nbsp;&nbsp;`battle/` | battle engine, AI, battle animations, battle controllers |
| &nbsp;&nbsp;`field/` | overworld: player, objects, map scripts, weather, field effects, TV, secret bases |
| &nbsp;&nbsp;`pokemon/` | Pokémon data, storage system, summary screen, evolution, daycare |
| &nbsp;&nbsp;`menus/`, `items/`, `pokenav/` | menus, bag and items, PokéNav |
| &nbsp;&nbsp;`contest/`, `frontier/`, `minigames/` | contests, Battle Frontier, Berry Blender, Dodrio, Pokémon Jump... |
| &nbsp;&nbsp;`link/` | link cable, wireless adapter (RFU), record mixing, trading, multiboot |
| &nbsp;&nbsp;`save/` | save blocks and the flash chip drivers |
| &nbsp;&nbsp;`scenes/` | intro, title screen, credits, Hall of Fame, cutscenes |
| &nbsp;&nbsp;`sound/` | the m4a sound engine: sequencer, CGB channels and the IWRAM mixer |
| &nbsp;&nbsp;`system/` | start-up, interrupts, BIOS calls, main loop, tasks, sprites, backgrounds, windows, text |
| &nbsp;&nbsp;`types/` | the C structs and unions as Rust types, one file per C header |
| &nbsp;&nbsp;`consts/` | the C `#define` and `enum` constants the code uses, one file per C header |
| &nbsp;&nbsp;`data/` | constant tables (species, moves, trainers...) and embedded graphics |
| `rust/Cargo.toml` | the crate |
| `graphics/`, `sound/`, `data/` | assets in pret's formats: PNGs, palettes, MIDI and WAV, maps, event scripts, text |
| `tools/` | pret's asset converters (C/C++), built by `make tools` |
| `include/`, `constants/`, `asm/` | headers and macros the data files are assembled with |
| `Makefile`, `*.mk` | build rules (pret's, with the C compilation replaced by the Rust crate) |
| `ld_script_modern.ld` | memory layout of the ROM, EWRAM and IWRAM |

---

## How the Rust code is organised

The port keeps pokeemerald's structure so that the two can be read side by side.

- **Names.** Functions, globals and struct fields keep their C names (`CB2_InitBattle`, `gSaveBlock1Ptr`,
  `tvShows`). Functions are `extern "C"` with the C signatures, so the engine's function pointers (tasks,
  callbacks, sprite callbacks) work as in C.
- **Types.** Every C struct and union is a `#[repr(C)]` Rust type in `rust/src/types/`, with exactly GCC's
  layout. Offsets and sizes are checked at compile time, so a wrong layout can't build. Bitfields become a
  getter and a setter (`mon.species()`, `mon.set_species(x)`).
- **Arrays.** C arrays are `CArray<T, N>`, which indexes like C: no bounds check (the original reads past the end
  of some arrays, and the game depends on it), and any integer type as index.
- **Pointers.** The game is built on raw pointers, as in C: `(*sprite).x`, and `p.at(i)` for `p + i`.
- **Arithmetic.** Plain `+ - *`. Overflow checks are off in every build profile, so integers wrap exactly as in C.
- **Constants.** C constants keep their names and live in `rust/src/consts/` (or in the module, if the C file
  defined them).
- **`unsafe`.** Most functions are `unsafe extern "C"`: they work on shared global state and raw pointers exactly
  like the original. Making the engine safe Rust is a later step.
- **`crate::c`** (`rust/src/system/c.rs`) holds the helpers the code relies on: `CArray`, `.at()`,
  `volatile_write` for hardware registers and DMA buffers, C-style division and shifts, `memcpy`/`memset`.

Much of the code was translated by a tool written for this port (C → Rust with GCC-measured layouts), then
checked against the C build as described in [Running the tests](#running-the-tests). Some modules were written by
hand, including the sound mixer, start-up code, flash drivers and parts of the wireless adapter library.

---

## Differences from the original

- **The ROM is not byte-identical** to pokeemerald's (a Rust compiler produces different machine code). It plays
  the same game with the same data.
- **Sound mixer speed.** The Rust mixer produces bit-identical audio but uses about 44 scanlines of CPU time per
  frame against the hand-written assembly's 35. It is inaudible; the only visible effect is that the title
  screen music starts one frame (1/60 s) later.
- **Timing.** Code runs at a different speed than the C build, so some loads finish a frame earlier or later.
  Game logic and random number sequences are unaffected.

---

## Troubleshooting

<details>
<summary><code>error[E0463]: can't find crate for `core`</code> or <code>could not find `Cargo.toml` in ... rust-src</code></summary>

The `rust-src` component is missing: `rustup component add rust-src`. If you have several toolchains, add it to
the one `rustc --version` reports.
</details>

<details>
<summary><code>arm-none-eabi-ld: command not found</code> / <code>arm-none-eabi-cpp: No such file or directory</code></summary>

devkitARM isn't installed or `DEVKITARM` isn't set in this terminal. Run
`export DEVKITPRO=/opt/devkitpro; export DEVKITARM=$DEVKITPRO/devkitARM` (and add it to `~/.bashrc`), then
`ls $DEVKITARM/bin` should list the `arm-none-eabi-*` tools.
</details>

<details>
<summary><code>png.h: No such file or directory</code> during <code>make tools</code></summary>

libpng's development headers are missing: `libpng-dev` (Debian/Ubuntu), `libpng-devel` (Fedora), `libpng`
(Arch, Homebrew).
</details>

<details>
<summary><code>cargo: command not found</code> inside <code>make</code></summary>

Open a new terminal after installing Rust, or run `source "$HOME/.cargo/env"`. You can also point the build at
cargo directly: `make modern CARGO=$HOME/.cargo/bin/cargo`.
</details>

<details>
<summary>The build stops with <code>Killed</code> or the computer runs out of memory</summary>

Compiling the Rust crate takes a few GB of RAM. Build with fewer jobs: `make modern -j2` (or `-j1`).
</details>

<details>
<summary>Plain <code>make</code> fails looking for <code>agbcc</code></summary>

Use `make modern`. Plain `make` is pret's build for the original compiler.
</details>

<details>
<summary>The build is very slow on Windows</summary>

Keep the repository inside WSL's own file system (`~/...`), not on `/mnt/c/...`.
</details>

<details>
<summary>The emulator shows a white screen or doesn't save</summary>

Make sure you opened `pokeemerald_modern.gba` from the latest build, and that the emulator's save type is
automatic or *Flash 128K*.
</details>

---

## Roadmap

- **Readable data tables.** The constant tables in `rust/src/data/` (species, moves, trainers, map events) are
  still byte arrays generated from the C; they will become typed tables (`BattleMove { power: 40, ... }`).
- **A Rust build.** Replace pret's C/C++ asset tools and the assembler-based data build with Rust, so the ROM
  builds with `cargo` alone.
- **Safer Rust.** Move from raw pointers and global state toward safe Rust, module by module, keeping the
  behaviour checked against the C build.

---

## Credits and license

- [**pret**](https://github.com/pret) and the contributors of
  [pokeemerald](https://github.com/pret/pokeemerald), whose decompilation this port is built on, including its
  build tools and asset formats.
- [devkitPro](https://devkitpro.org) for devkitARM, and [mGBA](https://mgba.io), which the port was tested with.

See [LICENSE](LICENSE).

Pokémon and Pokémon Emerald are trademarks of Nintendo, Creatures Inc. and Game Freak. This project is not
affiliated with or endorsed by them.
