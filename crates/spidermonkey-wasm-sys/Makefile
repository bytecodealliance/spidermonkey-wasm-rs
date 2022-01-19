.PHONY: test build build-release

makefile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
root_dir := $(dir $(makefile_path))
# TODO: Figure if OS can be used instead
UNAME_S := $(shell uname -s)
WASI_SDK_V := $(shell cat WASI_SDK).0


wasi_sdk_target_root :=

ifeq ($(UNAME_S), Darwin)
		wasi_sdk_target_root := $(root_dir)/vendor/macos/wasi-sdk-$(WASI_SDK_V)
endif

ifeq ($(UNAME_S), Linux)
		wasi_sdk_target_root := $(root_dir)/vendor/linux/wasi-sdk-$(WASI_SDK_V)
endif

export CXX = $(wasi_sdk_target_root)/bin/clang++ --sysroot=$(wasi_sdk_target_root)/share/wasi-sysroot
export AR = $(wasi_sdk_target_root)/bin/ar
export CXXFLAGS=-fno-exceptions -DRUST_CXX_NO_EXCEPTIONS

test:
	cargo wasi test

build:
	cargo build

build-release:
	cargo build --release