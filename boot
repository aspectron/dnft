#!/bin/sh

msg () {
    echo "\033[32m"
    echo "===================================================="
    echo "$1"
    echo "===================================================="
    echo "\033[0m"
}

# build and deploy solana app
cd program
msg "Building solana app"
./build
msg "Deploy solana app"
./deploy

# initialize dnft root and sample data
msg "Intilaizing data"
cd ../init
cargo run

# build wasm
msg "Building wasm"
cd ../wasm
./build-web-dev

# start server
msg "Starting server"
cd ../server
cargo run