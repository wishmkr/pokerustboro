# Building PokeRustboro

The game code is Rust (`rust/`). The assets (graphics, music, maps, event
scripts) keep pret/pokeemerald's formats and are converted by its build
tools (`tools/`), so the build uses pret's Makefile.

## Requirements

- **devkitARM** (the `arm-none-eabi-*` binutils and GCC, used for the asset
  tools, the assembler and the linker). See [INSTALL.md](INSTALL.md) for how
  pret installs it on each OS.
- **Rust** via rustup, with the `rust-src` component (the crate builds `core`
  for the GBA's `thumbv4t-none-eabi` target):

  ```sh
  rustup component add rust-src
  ```

- A C/C++ compiler, `make` and `libpng` for the asset tools.

## Build

```sh
make tools          # build the asset tools once
make modern -j4     # build pokeemerald_modern.gba
```

The ROM is `pokeemerald_modern.gba`. It is not the original's byte-for-byte
image (a Rust compiler cannot produce that); it plays the same game.

## Layout

| Path | What |
|---|---|
| `rust/src/` | the game, grouped by subject (`battle/`, `field/`, `menus/`, `sound/`, `system/`...); `rust/src/data/` holds the constant tables |
| `graphics/`, `sound/`, `data/` | assets, as in pret/pokeemerald |
| `src/m4a_1.s` | the sound mixer, the last piece still in assembly |
| `tools/`, `Makefile`, `*.mk`, `ld_script_modern.ld` | pret's asset tools and build rules |
| `include/`, `constants/`, `asm/` | constants and macros the data files are assembled with |
