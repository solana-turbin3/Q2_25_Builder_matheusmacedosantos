#!/bin/bash

# Kill any existing validators
pkill -f solana-test-validator

# Download Metaplex program if not exists
if [ ! -f mpl_token_metadata.so ]; then
    curl -L -o mpl_token_metadata.so https://github.com/metaplex-foundation/mpl-token-metadata/releases/download/v1.13.3/mpl_token_metadata.so
fi

# Start validator with Metaplex program on a different port
solana-test-validator --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s mpl_token_metadata.so --rpc-port 8900 &

# Wait for validator to start
sleep 5

# Run tests with custom port
ANCHOR_PROVIDER_URL=http://127.0.0.1:8900 anchor test

# Kill validator
pkill -f solana-test-validator 