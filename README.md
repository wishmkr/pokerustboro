# PokeRustboro

PokeRustboro is an experimental project to rewrite the complete Pokémon Emerald game engine in `no_std` Rust while continuing to produce a real, playable Game Boy Advance ROM.

Based on the pret/pokeemerald decompilation, the project replaces existing C modules incrementally while preserving the original ABI, memory layout, save format, assets, and hardware behavior.

## Current Status

- [X] 310/310 C modules migrated to Rust
- [X] 200 Rust tests passing
- [X] Produces a valid `pokerustboro.gba`
- [X] Targets the ARM7TDMI processor used by the Game Boy Advance
- [X] Major systems are still being actively ported
- After the porting phase, we will make the project more readable and writable.

Graphics, maps, music, text, and other generated assets remain in their native formats and are consumed by the Rust implementation.

PokeRustboro is an unofficial experimental project and is not affiliated with Nintendo, Game Freak, or The Pokémon Company.

Pokémon and Pokémon Emerald are trademarks of their respective owners.
