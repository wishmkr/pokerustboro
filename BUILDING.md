# Building PokeRustboro

Setup and build instructions for every system are in the README:
[Quick start](README.md#quick-start) and [Detailed setup](README.md#detailed-setup).

In short, with devkitARM and Rust (plus `rust-src`) installed:

```sh
make tools
make modern -j4     # the main port (rust/, Rust rules)  -> pokeemerald_modern.gba
make c-style -j4    # the C-style port (rust-c-style/)    -> pokeemerald_c_style.gba
```

The two versions are explained in [The two versions](README.md#the-two-versions).
