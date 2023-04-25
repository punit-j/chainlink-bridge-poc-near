#!/bin/sh
set -e             #set -e sets an non-ignoring error state.
echo ">> Building ChainLinkBridge Contract"

cargo build --target wasm32-unknown-unknown --release   # cmd to compile the contract
echo ">> Build completed"
mkdir -p ./Build_Output
cp ./target/wasm32-unknown-unknown/release/*.wasm   ./Build_Output/ChainLinkBridge.wasm 
