.PHONY: test build fmt test-sys build-sys build-release-sys fmt-sys

test-sys:
	cd crates/spidermonkey-wasm-sys \
		&& make test \
		&& cd -

build-sys:
	cd crates/spidermonkey-wasm-sys \
		&& make build \
		&& cd -

build-release-sys:
	cd crates/spidermonkey-wasm-sys \
		&& make build-release \
		&& cd -

fmt-sys:
	cargo fmt --package=spidermonkey-wasm-sys

test: test-sys
build: build-sys
build-release: build-release-sys
fmt: fmt-sys