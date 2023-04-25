#!/bin/sh
set -e               #set -e sets an non-ignoring error state.
echo ">> Deploying ChainLinkBridge Contract"
near deploy \
    --wasmFile ./Build_Output/ChainLinkBridge.wasm \
    --initGas   300000000000000 \
   --initFunction "new" \
    --initArgs '{"prover_account": "prover2.unatrix.testnet", "min_block_delay_near": 0, "min_block_delay_eth": 0}' \
    --accountId oracle.unatrix.testnet


