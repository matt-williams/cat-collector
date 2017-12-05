# cat-collector

This was an attempt at an entry for the [Ludum Dare 40](https://ldjam.com/events/ludum-dare/40) game jam.  Unfortunately, I ran out of time and didn't get it into a playable state.

It was, however, successful in allowing me to play with [Rust](https://www.rust-lang.org/en-US/)/[WebAssembly](http://webassembly.org/).

[Play it here](https://matt-williams.github.io/cat-collector)

## Building

To build,

*   ensure you have the latest Rust nightly, the `wasm32-unknown-unknown` target and `wasm-gc` installed - follow [these instructions](https://www.hellorust.com/setup/wasm-target/)
*   run `cargo +nightly build --target=wasm32-unknown-unknown --release`
*   run `wasm-gc target/wasm32-unknown-unknown/release/cat_collector.wasm target/wasm32-unknown-unknown/release/cat_collector.wasm`.
