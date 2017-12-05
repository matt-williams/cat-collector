# cat-collector

This was an attempt at an entry for the [Ludum Dare 40](https://ldjam.com/events/ludum-dare/40) game jam.  Unfortunately, I ran out of time and didn't get it into a playable state.

It was, however, successful in allowing me to play with [Rust](https://www.rust-lang.org/en-US/)/[WebAssembly](http://webassembly.org/).

[Play it here](https://matt-williams.github.io/cat-collector/docs)

It's a text adventure.  The theme of Ludum Dare 40 was "the more you have, the worse it is", which made me think of cats.  You start off with one cat, but more come in as long as you leave the doors open (and they scratch at the doors if you don't).  I didn't have time to implement anything to encourage you to take cats in, or to allow you to "fail" when there were too many cats.  :(

## Building

To build,

*   ensure you have the latest Rust nightly, the `wasm32-unknown-unknown` target and `wasm-gc` installed - follow [these instructions](https://www.hellorust.com/setup/wasm-target/)
*   run `cargo +nightly build --target=wasm32-unknown-unknown --release`
*   run `wasm-gc target/wasm32-unknown-unknown/release/cat_collector.wasm target/wasm32-unknown-unknown/release/cat_collector.wasm`.
