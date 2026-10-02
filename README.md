# PokeRustboro

**Pokémon Emerald, rewritten in Rust.** PokeRustboro is the [pret/pokeemerald](https://github.com/pret/pokeemerald)
decompilation with its entire game code ported from C to `no_std` Rust. It builds a real, playable Game Boy Advance
ROM that runs on emulators.

- **All of the game's code is Rust.** Every one of pokeemerald's 310 C files and its hand-written assembly (start-up
  code, interrupt handler, sound mixer, GameCube multiboot) is now Rust. No C is compiled into the ROM.
- **Two versions of the port, side by side.** `rust/` follows Rust's rules: modules use each other with `use`, the
  scripts, maps and songs are Rust data, and only 0.3% of the code still speaks C's calling convention.
  `rust-c-style/` keeps the port as it was at v0.3.0, structured exactly like the C. Both build the same game. See
  [The two versions](#the-two-versions).
- **It plays the same game.** Both versions are checked against the C build by comparing screens, memory, sound
  engine state and save files.
- **Assets are untouched.** Graphics, maps, music, text and event scripts stay in pret's formats and are built by
  pret's tools.

> PokeRustboro is an unofficial fan project. It is not affiliated with Nintendo, Game Freak or The Pokémon Company.
> No ROM is distributed here: you build it yourself from this repository.

---

## Contents

- [Status](#status)
- [The two versions](#the-two-versions)
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

What changed in each version: [CHANGELOG.md](CHANGELOG.md).

---

## Status

| | `rust/` (Rust rules) | `rust-c-style/` (C style) |
|---|---|---|
| Game code in Rust | 310 / 310 C files + the 4 assembly files | same |
| C compiled into the ROM | none | none |
| Items using C's calling convention (`extern "C"`) | **0.3%**: the start-up code, the BIOS calls and the interrupt handlers | nearly all |
| Event scripts, maps, songs | Rust statics, checked byte for byte against the assembled data | linked from the assembled `data/` objects |
| Functions in safe Rust | 2,626 of 20,707 (13%) | a few hand-written modules |
| Unit tests (`cargo test`) | 184 | 189 |
| Build | `make modern` | `make c-style` |
| Output | `pokeemerald_modern.gba` | `pokeemerald_c_style.gba` |

Target: ARM7TDMI (`thumbv4t-none-eabi`), Game Boy Advance.

What is still C/C++ is **build tooling only**: pret's asset converters in `tools/` (`gbagfx`, `mid2agb`,
`wav2agb`, `preproc`, `mapjson`...) and the headers the data files are assembled with. None of it ends up in
the ROM.

---

## The two versions

**`rust/`, the main port, follows Rust's rules.** It started from the C-style port and was rewritten step by step
with automated, behaviour-preserving tools, every step checked against the C build:

- Modules reach each other's functions and statics with `use`, not by re-declaring them in `extern "C"` blocks.
  Functions have the Rust calling convention; only the start-up code, the BIOS calls (naked assembly) and the
  interrupt handlers keep C's.
- The event and battle scripts, maps, songs and multiboot images are Rust statics, generated from the assembled
  data. Every build checks all 48,317 addresses in them and every byte against the assembler's output.
- Loops are `for` loops, variables are declared where they are first set, task and sprite data slots carry the
  names the C gave them (`data[tState]`), single-value globals are safe `Global<T>` cells, tasks are reached
  through safe accessors.
- About 35 small modules (money, coins, the bag, Pokémon data, tasks, the clock, flags and vars...) are written
  as idiomatic, safe Rust, with a thin bridge for the rest of the engine.

**`rust-c-style/` keeps the port as it was at v0.3.0.** Every C function is a Rust function with the same name,
structure and calling convention; modules talk to each other the C way. It is the best version for reading the
port side by side with pokeemerald, and it shows where the main port started from. Since v0.3.0 it has received
one data fix (eggs were drawn mirrored on the summary screen).

Both build the same game from the same assets; pick one with `make modern` or `make c-style`.

---

## What the code looks like

The C, from `src/tv.c`:

```c
void ClearTVShowData(void)
{
    u8 i, j;

    for (i = 0; i < ARRAY_COUNT(gSaveBlock1Ptr->tvShows); i++)
    {
        gSaveBlock1Ptr->tvShows[i].commonInit.kind = 0;
        gSaveBlock1Ptr->tvShows[i].commonInit.active = 0;
        for (j = 0; j < ARRAY_COUNT(gSaveBlock1Ptr->tvShows[i].commonInit.data); j++)
            gSaveBlock1Ptr->tvShows[i].commonInit.data[j] = 0;
    }
    ClearPokeNews();
}
```

`rust/src/field/tv.rs` (Rust rules):

```rust
pub unsafe fn ClearTVShowData() {
    for i in 0..25u8 {
        (*gSaveBlock1Ptr).tvShows[i].commonInit.kind = 0;
        (*gSaveBlock1Ptr).tvShows[i].commonInit.active = 0;
        for j in 0..34u8 {
            (*gSaveBlock1Ptr).tvShows[i].commonInit.data[j] = 0;
        }
    }
    ClearPokeNews();
}
```

`rust-c-style/src/field/tv.rs` (C style):

```rust
pub unsafe extern "C" fn ClearTVShowData() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
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
}
```

A task in the main port, with the C's names for its data slots (`src/evolution_scene.c` writes
`gTasks[taskId].tState++`):

```rust
match task_get(taskId, tState) {
    0 => {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
        task_set(taskId, tState, task_get(taskId, tState) + 1);
    }
    1 if gPaletteFade.active() == 0 => {
        mon = &raw mut gPlayerParty[task_get(taskId, tPartyId)];
        // ...
```

And one of the idiomatic modules, `rust/src/items/money.rs`:

```rust
impl<'a> Money<'a> {
    /// Adds money, up to [`MAX_MONEY`] (receiving money never leaves less).
    pub fn add(&mut self, amount: u32) {
        let total = self.get().saturating_add(amount).min(MAX_MONEY);
        self.set(total);
    }
}
```

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
make modern -j4     # the main port (Rust rules) -> pokeemerald_modern.gba
make c-style -j4    # optional: the C-style port  -> pokeemerald_c_style.gba
```

---

## Detailed setup

You need four things: a C compiler with `make` and `libpng` (for pret's asset tools), **devkitARM** (the
assembler and linker for the GBA), **Rust** with the `rust-src` component, and this repository. Both versions
of the port need exactly the same setup.

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

First build pret's asset converters (once):

```sh
make tools
```

Then build the version you want, or both:

| | Command | Crate | Objects | ROM |
|---|---|---|---|---|
| Main port (Rust rules) | `make modern -j4` | `rust/` | `build/modern/` | **`pokeemerald_modern.gba`** |
| C-style port | `make c-style -j4` | `rust-c-style/` | `build/c_style/` | **`pokeemerald_c_style.gba`** |

- The two builds don't share objects, so you can build both and switch freely.
- Use `make modern` / `make c-style`, not plain `make`: plain `make` is pret's build with the old agbcc compiler,
  which isn't part of this project.
- The first build converts every asset and compiles the Rust crate (including `core`), so it takes a few
  minutes. Later builds only redo what changed.
- `-j4` runs four jobs at once. Compiling a crate needs a few GB of RAM; if the build gets killed, use `-j2`.
- To start over: `make tidymodern` removes the main port's ROM and objects (`make tidymodern RUST_STYLE=c`
  the C-style one's); `make clean` also removes converted assets and the tools.

---

## Playing the ROM

**Emulators.** [mGBA](https://mgba.io) is recommended (also available as the mGBA core in RetroArch). Open
`pokeemerald_modern.gba` (or `pokeemerald_c_style.gba`) directly.

**Saves.** The game saves to 128 KB flash, like the cartridge; emulators detect this automatically. The save
format is pokeemerald's, so `.sav` files are interchangeable between the two versions and with a pokeemerald
build.

> Back up any save you care about before loading it into a development build.

**Hardware.** The ROM is a normal GBA ROM and should run from a flash cartridge on a Game Boy Advance, GBA SP,
Game Boy Micro or Nintendo DS (GBA slot), with the save type set to *Flash 128K* (1 Mbit). The port has so far
been tested on emulators only; reports from real hardware are welcome.

---

## Running the tests

The unit tests run on your computer (they don't need the GBA target):

```sh
cargo test --manifest-path rust/Cargo.toml           # main port
cargo test --manifest-path rust-c-style/Cargo.toml   # C-style port
```

They cover the parts of the engine that can run without the GBA: string and text utilities, random numbers,
math and trigonometry, the heap allocator, the task scheduler, money, coins and the bag, Pokémon data encryption
and checksums, event flags and similar.

Behaviour as a whole is checked by running each Rust ROM and the C ROM side by side in a headless emulator:

- **the new-game sequence and walking around**, compared frame by frame;
- **saves** written by one build and loaded by the other;
- **31 screens of Pokémon data**: a generated save with a party covering every data path (a shiny Pokémon with
  ribbons and a held item, an egg, a Japanese name, a Bad Egg, Unown), a PC box, every bag pocket and money,
  compared on the party menu, all four summary pages of each Pokémon, the PC and the bag;
- during the port, IO registers, palettes, VRAM and OAM through the intro, and the sound engine's state over
  thousands of frames.

Both versions pass all of them. Battles, contests, link play and the Battle Frontier are not covered by the
automated tests yet.

---

## Project layout

| Path | What |
|---|---|
| `rust/src/` | **The game, main port.** One Rust module per original C file, grouped by subject: |
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
| &nbsp;&nbsp;`asmdata/` | the event and battle scripts, maps, songs and multiboot images, generated from the assembled data |
| `rust-c-style/` | **The C-style port** (as at v0.3.0), same module layout without `asmdata/` |
| `graphics/`, `sound/`, `data/` | assets in pret's formats: PNGs, palettes, MIDI and WAV, maps, event scripts, text |
| `tools/` | pret's asset converters (C/C++), built by `make tools` |
| `include/`, `constants/`, `asm/` | headers and macros the data files are assembled with |
| `Makefile`, `*.mk` | build rules (pret's, with the C compilation replaced by the Rust crates) |
| `ld_script_modern.ld` | memory layout of the ROM, EWRAM and IWRAM |

---

## How the Rust code is organised

Both versions keep pokeemerald's structure so that they can be read side by side with it.

- **Names.** Functions, globals and struct fields keep their C names (`CB2_InitBattle`, `gSaveBlock1Ptr`,
  `tvShows`).
- **Types.** Every C struct and union is a `#[repr(C)]` Rust type in `types/`, with exactly GCC's layout.
  Offsets and sizes are checked at compile time, so a wrong layout can't build. Bitfields become a getter and a
  setter (`mon.species()`, `mon.set_species(x)`).
- **Arrays.** C arrays are `CArray<T, N>`, which indexes like C: no bounds check (the original reads past the end
  of some arrays, and the game depends on it), and any integer type as index.
- **Arithmetic.** Plain `+ - *`. Overflow checks are off in every build profile, so integers wrap exactly as in C.
- **Constants.** C constants keep their names and live in `consts/` (or in the module, if the C file defined
  them).
- **`crate::c`** (`system/c.rs`) holds the helpers the code relies on: `CArray`, `.at()` for `p + i`,
  `volatile_write` for hardware registers and DMA buffers, C-style division and shifts, `memcpy`/`memset`.

In the **main port**:

- Functions have the Rust calling convention and modules import each other with `use`. Where a module saw
  another's data through a different type (a byte view of a struct, say), the view is explicit at the use.
- Most functions are still `unsafe`: the engine works on shared global state and raw pointers like the original.
  Single-value globals are `Global<T>` cells read and written safely (`sTimer.get()`, `sTimer.set(x)`); the
  task scheduler has safe accessors (`task_get`, `task_set`); 2,626 functions are safe Rust, checked by the
  compiler.

In the **C-style port**, functions are `unsafe extern "C"` with the C signatures, and modules declare what they
use from each other in `extern "C"` blocks, as C headers do.

Much of the code was translated by a tool written for this port (C → Rust with GCC-measured layouts), and the
main port was then rewritten by further tools, each step checked against the C build as described in
[Running the tests](#running-the-tests). Some modules were written by hand, including the sound mixer, start-up
code, flash drivers and parts of the wireless adapter library.

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
cargo directly: `make modern CARGO=$HOME/.cargo/bin/cargo` (or `make c-style CARGO=...`).
</details>

<details>
<summary>The build stops with <code>Killed</code> or the computer runs out of memory</summary>

Compiling a Rust crate takes a few GB of RAM. Build with fewer jobs: `make modern -j2` (or `-j1`).
</details>

<details>
<summary>Plain <code>make</code> fails looking for <code>agbcc</code></summary>

Use `make modern` or `make c-style`. Plain `make` is pret's build for the original compiler.
</details>

<details>
<summary>The build is very slow on Windows</summary>

Keep the repository inside WSL's own file system (`~/...`), not on `/mnt/c/...`.
</details>

<details>
<summary>The emulator shows a white screen or doesn't save</summary>

Make sure you opened the ROM from the latest build (`pokeemerald_modern.gba` or `pokeemerald_c_style.gba`), and
that the emulator's save type is automatic or *Flash 128K*.
</details>

---

## Roadmap

- **Less `unsafe`.** Give the large globals (sprites, battle state, save blocks) safe accessors like the task
  scheduler's, then rewrite the core modules (battle calculations, party, bag, saving) as safe Rust.
- **Wider tests.** Battles, contests and link play compared against the C build automatically.
- **Readable data tables.** The constant tables in `data/` are still byte arrays generated from the C; they will
  become typed tables (`BattleMove { power: 40, ... }`).
- **A Rust build.** Replace pret's C/C++ asset tools and the assembler-based data build with Rust, so the ROM
  builds with `cargo` alone.

---

## Credits and license

- [**pret**](https://github.com/pret) and the contributors of
  [pokeemerald](https://github.com/pret/pokeemerald), whose decompilation this port is built on, including its
  build tools and asset formats.
- [devkitPro](https://devkitpro.org) for devkitARM, and [mGBA](https://mgba.io), which the port was tested with.

See [LICENSE](LICENSE).

Pokémon and Pokémon Emerald are trademarks of Nintendo, Creatures Inc. and Game Freak. This project is not
affiliated with or endorsed by them.
