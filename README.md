<div align="center">
  <h1><code>spidermonkey-wasm-rs</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>
  <p>
    <strong>Rust bindings and generic builtins for SpiderMonkey for the <code>wasm32-wasi</code> target</strong>
  </p>
</div>

## Requirements

- [wasi-sdk-12](https://github.com/WebAssembly/wasi-sdk/releases/tag/wasi-sdk-12), installed under `/opt/wasi-sdk/`
- [cargo-wasi](https://github.com/bytecodealliance/cargo-wasi) for testing
- Rust 1.56

## Development

- `git submodule update --recursive --init` to pull in [spidermonkey-wasm-build](https://github.com/bytecodealliance/spidermonkey-wasm-build)
- `cd crates/spidermonkey-wasm-sys/spidermonkey-wasm-build && ./download.sh`
- `cd - && cargo build --release --target=wasm32-wasi --package spidermonkey-wasm-sys`
- `cargo wasi test sanity` to run tests




