# build and deploy solana app
cd program
./build && ./deploy

# initialize dnft root and sample data
cd ../init
cargo run

# build wasm
cd ../wasm
./build-web-dev

