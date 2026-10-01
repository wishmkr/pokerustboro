# The C-style Rust port

This folder keeps the port in the shape it had at v0.3.0: every C function
became a Rust function with the same name and the same structure, and the
modules still talk to each other the C way (`extern "C"` declarations,
`#[no_mangle]` symbols, raw pointers). The event scripts, maps and songs
are linked from the assembled `data/` objects, as in the C game.

It is kept for comparison with the main port in `../rust/`, which follows
Rust's rules instead: modules import each other with `use`, the data is
Rust statics, and only the start-up code, the BIOS calls and the
interrupt handlers still use the C calling convention.

Both build the same game; this one is built with

    make c-style

which writes `pokeemerald_c_style.gba` (the main port is `make modern`,
writing `pokeemerald_modern.gba`). See the top-level README for setup.

Since v0.3.0 one data fix was carried over from the main port: the
species table keeps the bytes the C reads past its end, so eggs are no
longer drawn mirrored on the summary screen.
