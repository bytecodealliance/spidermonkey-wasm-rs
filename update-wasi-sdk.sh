!# /usr/bin/env bash

set -ex

version=$(cat WASI_SDK)
mkdir -p /opt/wasi-sdk

cd /opt/wasi-sdk

name=wasi-sdk-$version
qualified_name=$name.0

if [[ -d $qualified_name ]]; then
  echo "$qualified_name exists";
  exit 0;
fi

asset=$qualified_name
unamestr=$(uname)

if [[ "$unamestr" == "Linux" ]]; then
  asset=$asset-linux.tar.gz
elif [[ "$unamestr" == "Darwin" ]]; then
  asset=$asset-macos.tar.gz
else
  echo "Unsupoorted platform $unnamestr";
  exit 1;
fi

curl --fail -L -O \
https://github.com/WebAssembly/wasi-sdk/releases/download/$name/$asset

tar xf $asset
rm -rf $asset
