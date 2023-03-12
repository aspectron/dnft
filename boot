#!/bin/sh

msg () {
    echo "\033[32m"
    echo "===================================================="
    echo "$1"
    echo "===================================================="
    echo "\033[0m"
}

check_error(){
    if [ $? -ne 0 ]; then
        echo "\033[31m"
        echo "\n--> $1: failed\n"
        echo "\033[0m"
        exit $code
    fi
}

# build and deploy solana app
cd program
msg "Build solana app"
./build
check_error "Build solana app"
msg "Deploy solana app"
./deploy
check_error "Deploy solana app"

# initialize dnft root and sample data
msg "Initializing data"
cd ../init
cargo run
check_error "Initializing data"

# build wasm
msg "Building wasm"
cd ../wasm
./build-web-dev
check_error "Building wasm"

# start server
msg "Starting server"
cd ../server
cargo run -- --open
check_error "Starting server"