.PHONY: test build fmt-check test-sys build-sys build-release-sys fmt-check-sys

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

fmt-check-sys:
	cargo fmt --package=spidermonkey-wasm-sys -- --check

test: test-sys
build: build-sys
build-release: build-release-sys
fmt-check: fmt-check-sys
