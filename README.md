<div align="center">
  <h1><code>spidermonkey-wasm-rs</code></h1>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>
  <p>
    <strong>Experimental Rust bindings and generic builtins for SpiderMonkey for the <code>wasm32-wasi</code> target</strong>
  </p>
  <a href="https://github.com/bytecodealliance/spidermonkey-wasm-rs/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/spidermonkey-wasm-rs/workflows/CI/badge.svg" alt="build status" /></a>
</div>

## Requirements
- [cargo-wasi](https://github.com/bytecodealliance/cargo-wasi) for testing
- Rust 1.56
- WASI-SDK 12 at /opt/wasi-sdk/wasi-sdk-12.0 (can be downloaded by executing `sudo ./update-wasi-sdk.sh`)

## Development
- `git submodule update --recursive --init` to pull in [spidermonkey-wasm-build](https://github.com/bytecodealliance/spidermonkey-wasm-build)
- `cd crates/spidermonkey-wasm-sys/spidermonkey-wasm-build && ./download.sh` to pull in SpiderMonkey build artifacts
- In `cd crates/spidermonkey-wasm`, run:
  - `cargo build`  to build
  - `cargo wasi test` to run tests



