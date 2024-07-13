#!/bin/bash

# Ensure the script stops on errors
set -e

# Build the Solana program
echo "Building the Solana program: little-adventure"
anchor build --program-name "little-adventure"

# Deploy the program
echo "Deploying the Solana program: little-adventure"
solana program deploy "target/deploy/little_adventure.so"

cargo run "./client"