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

## Development
- `git submodule update --recursive --init` to pull in [spidermonkey-wasm-build](https://github.com/bytecodealliance/spidermonkey-wasm-build)
- `cd crates/spidermonkey-wasm-sys/spidermonkey-wasm-build && ./download.sh`
- `cd crates/spidermonkey-wasm-sys`
  - `make build` or `make build-release`
  - `make test`



