!# /usr/bin/env bash
set -ex

version=$(cat WASI_SDK)
mkdir -p vendor/linux
mkdir -p vendor/macos

name=wasi-sdk-$version
qualified_name=$name.0
linux_asset=$qualified_name-linux.tar.gz
macos_asset=$qualified_name-macos.tar.gz

cd vendor/linux

curl --fail -L -O \
https://github.com/WebAssembly/wasi-sdk/releases/download/$name/$linux_asset
tar xf $linux_asset
rm -rf $linux_asset

cd -
cd vendor/macos

curl --fail -L -O \
https://github.com/WebAssembly/wasi-sdk/releases/download/$name/$macos_asset
tar xf $macos_asset
rm -rf $macos_asset
