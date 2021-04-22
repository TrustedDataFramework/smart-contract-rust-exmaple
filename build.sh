#!/usr/bin/env bash
cargo build --target wasm32-unknown-unknown
rm -rf /Users/sal/Documents/Github/maze-protocol/layer2/main.wasm
cp target/wasm32-unknown-unknown/debug/hello_wasm.wasm /Users/sal/Documents/Github/maze-protocol/layer2/main.wasm