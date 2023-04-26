#!/bin/sh
set -e               #set -e sets an non-ignoring error state.
ENCODED_DATA_PROOF=$(cargo run --manifest-path utils/Cargo.toml -- encode-data-proof --proof "ss" | tail -n 1)
echo "^^^^^^^^^^^^^^^^^^^^ENCODED DATA IS: ^^^^^^^^^^^^^^^^^^^^\n"$ENCODED_DATA_PROOF

near call oracle6.adityarout.testnet \
add_feed_data \
'{"symbol": "ETH/USD", "data_proof": '"$ENCODED_DATA_PROOF"'}' \
--accountId adityarout.testnet