#!/bin/zsh
cargo clean
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/firefly.wasm res/
rm -rf neardev
near dev-deploy res/firefly.wasm