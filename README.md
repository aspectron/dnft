# `delta-nft framework`

Delta NFT project - created for [Solana Grizzlython](https://solana.com/grizzlython) Hackathon

The goal of this project is to create a new NFT standard that provides users with the ability to create NFTs that contain custom authority-managed data schemas. Data schemas that can be created are meant to represent different real or virtual assets, while carrying detailed information about them within each token issued. Authority-managed means that data stored in each schema can be managed by pre-defined set of authorities.

DNFT mints are similar to database tables, where groups of fields can have different authorities managing them.

The secondary goal of this project is to demonstrate the upcoming [Solana Kaizen Framework](https://aspectron.com/en/projects/kaizen.html) - a Rust-centric framework that focuses on allowing the creation of business-oriented WebAssembly-powered (WASM32) DApps.

Detailed information about this project can be found at [https://deltanft.xyz](https://deltanft.xyz)

## Requirements

- Latest Rust: https://www.rust-lang.org
- WASM-PACK: https://rustwasm.github.io/wasm-pack/
- Solana Tool Suite: https://docs.solana.com/cli/install-solana-cli-tools

## Running this project

### Clone repositories
You need to clone this repository as well as Solana Kaizen repository into the same folder:
```bash
mkdir demo
cd demo
git clone https://github.com/solana-kaizen/kaizen
git clone https://github.com/aspectron/dnft
```

Solana Kaizen is currently under development, so it is not yet published to *crates.io*.

### Start Solana Test Validator
```bash
mkdir test-validator
cd test-validator
solana-test-validator
```

If you would like to see program execution logs start log monitoring:
```bash
solana logs
```

### Setup your wallet

This demo currently works only with [Solana Phantom wallet](https://phantom.app)

Install and configure Phantom to run against your local validator by selecting
`Developer Settings` -> `Change Network` -> `Localhost`

Get some SOL on your local validator and transfer it to the Phantom Wallet
```bash
solana airdrop 500
solana transfer 6Lmr...VuLP 400 --allow-unfunded-recipient
```
Documentaion for sending tokens is available here: [https://docs.solana.com/cli/transfer-tokens](https://docs.solana.com/cli/transfer-tokens)

### Build and start the project

We have setup a `boot` script that performs the following actions:
- Builds and deploys the program to the local validator
- Initializes root program accounts
- Deploys test DNFT mints (test data)
- Builds WASM application
- Starts a built-in HTTP server

Please note that the `boot` script is usable on MacOS or Linux platforms.
To run on Windows, you must run the contents of the scripts manually.
The `boot` script is available in the root of the dnft repository:
https://github.com/aspectron/dnft/blob/master/boot

### Load the application in the browser

The `boot` script will start a built-in HTTP server, following that, the application
will open a browser window pointing to [https://localhost:8085](https://localhost:8085)

If running manually, you can use any HTTP server (such as [`basic-http-server`](https://crates.io/search?q=basic-http-server))
capable of serving local content. The server needs to be started in the following folder as root: `dnft/wasm/web`
