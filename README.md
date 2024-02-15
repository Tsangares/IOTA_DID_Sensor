# Quickstart

To run this project you need to first clone it. 

## Environment Variables

Then you need to setup a `.env` file, in this repository there is an example `.env_example`. Copy it,

    cp .env_example .env

Then edit the varibale `PASSPHRASE` to some unique string. 

## Rust

Once you set a `PASSPHRASE` run the project by going to the root and running

    cargo run

There should be an output prompting you an address and a faucet link. Request funds, press enter. Then a DID will be issues. Following this then a random temperature will be sampled.

Then a NFT with the temperature data will be minted and signed by the DID. This will make your computer run a little hot. 

In the output should be links related to the explorer to view the addresss, DID and NFT with metadata.