#! /usr/bin/env bash

set -ex

export CXX="/opt/wasi-sdk/wasi-sdk-12.0/bin/clang++ --sysroot=/opt/wasi-sdk/wasi-sdk-12.0/share/wasi-sysroot"
export CXXFLAGS="-fno-exceptions -DRUST_CXX_NO_EXCEPTIONS"
export AR="/opt/wasi-sdk/wasi-sdk-12.0/bin/ar"
export LIBCLANG_PATH="/opt/wasi-sdk/wasi-sdk-12.0/share/wasi-sysroot/lib/wasm32-wasi"
export LIBCLANG_RT_PATH="/opt/wasi-sdk/wasi-sdk-12.0/lib/clang/11.0.0/lib/wasi"

if [ $# -eq 0 ]; then
  cargo build --release --target=wasm32-wasi
else
  cargo "$@" --release --target=wasm32-wasi
fi
