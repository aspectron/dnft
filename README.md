# delta-nft

Delta NFT project - created for [Solana Grizzlython](https://solana.com/grizzlython) Hackathon

The goal of this project is to create a new NFT standard that provides users with an ability to create NFTs with custom data schema. The data schema that can be created is meant to represent different real or virtual assets, while carrying detailed information about them within each token issued.

Key features:

- A user can create a custom schema and register it with the DNFT program.
- Once created, any number of tokens can be issued against this schema by anyone (there is an unlimited number of tokens).
- Each token can have an associated price or specified exchange mechanism (sale, auction). The "for sale" flag as well as sale properties can be enabled or disabled only by the owner.
- Once published, if "for sale" flag is active, the token can be acquired by anyone making the corresponding payment or following through the sale process, such as an auction.
- Additional rules, such as % of sale revenue can be setup within each token specifically.
- Token category can also be configured to provide additional revenue sharing for the category creator (or a dedicated account address)
- Token creation can be restricted to a specific like of accounts - accounts are associated with the caregory and can be managed by the category creator.

Schema, Viewers and Editors:

- The schema is defined by custom data types, allowing it to be created from 2nd-tier languages such as TypeScript or JavaScript.
- A developer can use the shcema definitoin to programmatically or manually create an editor capable of submitting data needed for token creation or to create a Viewer needed to visualize the token data.

Security, Vulnerabilities and Spam:

- Like any asset, the token has a dedicated account and its authenticity should be validated by the account key.
- Anyone can create as many tokens as they like - it is the responsibility of the data service providers and applications to filter out invalid tokens.


#### Setup development environment
- Start test validator: `solana-test-validator`, then open new terminal window/tab
- clone this repository `git clone https://github.com/aspectron/dnft.git`
- Build and deploy program: `cd dnft/program && ./build && ./deploy`
- Build wasm : `cd ../wasm && ./build-web-dev`
- Start http server : `cd ./web && simple-http-server ./`
