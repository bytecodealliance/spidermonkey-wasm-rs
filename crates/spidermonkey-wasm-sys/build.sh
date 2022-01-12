# TODO: Make this script generic

#!/usr/bin/env bash

base_dir=/Users/saulecabrera/Developer/spidermonkey-wasm-rs/crates/spidermonkey-wasm-sys

export CXX="${base_dir}/vendor/macos/wasi-sdk-12.0/bin/clang++
--sysroot=${base_dir}/vendor/macos/wasi-sdk-12.0/share/wasi-sysroot"
export AR="${base_dir}/vendor/macos/wasi-sdk-12.0/bin/ar"
export CC="${base_dir}/vendor/macos/wasi-sdk-12.0/bin/clang"

cargo build --release --target=wasm32-wasi
