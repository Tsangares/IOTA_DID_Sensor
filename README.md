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

# Example

This is an example of the output from running the program:
```
Connecting to node: https://api.testnet.shimmer.network
Connected to node: HORNET true 2025032 rms
Creating a new Stronghold file: ./stronghold_datastore.data
Your mnemonic is: (badge shine pupil pond chef stuff ocean garment saddle caution extend mountain piece quantum code canoe pudding fall figure rocket giraffe shift diet negative)
This has been stored in stronghold!
Your wallet address is: rms1qraf9e0el2pyf3h0zndktappwsamq6c7rt2mj853u3nuhs5ps2pf5yf86ae
Please request funds from https://faucet.testnet.shimmer.network, wait for a couple of seconds and then press Enter.
To check for funds use the explorer: https://explorer.shimmer.network/testnet

Alias Output: {
  "type": 4,
  "amount": "93100",
  "aliasId": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "stateIndex": 0,
  "stateMetadata": "0x444944010083017b22646f63223a7b226964223a226469643a303a30222c22766572696669636174696f6e4d6574686f64223a5b7b226964223a226469643a303a302368754436567a364f6457366476526a644e3674566b4f696639562d6975324a484a6d3369596c5146714567222c22636f6e74726f6c6c6572223a226469643a303a30222c2274797065223a224a736f6e5765624b6579222c227075626c69634b65794a776b223a7b226b7479223a224f4b50222c22616c67223a224564445341222c226b6964223a2268754436567a364f6457366476526a644e3674566b4f696639562d6975324a484a6d3369596c5146714567222c22637276223a2245643235353139222c2278223a224d71597a6b747a5268694f44566c62717457474d71396d4b2d39396d59457636656e6a3570386d5f474659227d7d5d7d2c226d657461223a7b2263726561746564223a22323032342d30322d31355431323a30323a32385a222c2275706461746564223a22323032342d30322d31355431323a30323a32385a227d7d",
  "foundryCounter": 0,
  "unlockConditions": [
    {
      "type": 4,
      "address": {
        "type": 0,
        "pubKeyHash": "0xfa92e5f9fa8244c6ef14db65f421743bb06b1e1ad5b91e91e467cbc28182829a"
      }
    },
    {
      "type": 5,
      "address": {
        "type": 0,
        "pubKeyHash": "0xfa92e5f9fa8244c6ef14db65f421743bb06b1e1ad5b91e91e467cbc28182829a"
      }
    }
  ],
  "features": [
    {
      "type": 0,
      "address": {
        "type": 0,
        "pubKeyHash": "0xfa92e5f9fa8244c6ef14db65f421743bb06b1e1ad5b91e91e467cbc28182829a"
      }
    }
  ]
}
Published DID document: {
  "doc": {
    "id": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3",
    "verificationMethod": [
      {
        "id": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3#huD6Vz6OdW6dvRjdN6tVkOif9V-iu2JHJm3iYlQFqEg",
        "controller": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3",
        "type": "JsonWebKey",
        "publicKeyJwk": {
          "kty": "OKP",
          "alg": "EdDSA",
          "kid": "huD6Vz6OdW6dvRjdN6tVkOif9V-iu2JHJm3iYlQFqEg",
          "crv": "Ed25519",
          "x": "MqYzktzRhiODVlbqtWGMq9mK-99mYEv6enj5p8m_GFY"
        }
      }
    ]
  },
  "meta": {
    "created": "2024-02-15T12:02:28Z",
    "updated": "2024-02-15T12:02:28Z",
    "governorAddress": "rms1qraf9e0el2pyf3h0zndktappwsamq6c7rt2mj853u3nuhs5ps2pf5yf86ae",
    "stateControllerAddress": "rms1qraf9e0el2pyf3h0zndktappwsamq6c7rt2mj853u3nuhs5ps2pf5yf86ae"
  }
}
Temperature: 79.5169458533268
The issuer of the Digital Product Passport NFT is: {
  "doc": {
    "id": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3",
    "verificationMethod": [
      {
        "id": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3#huD6Vz6OdW6dvRjdN6tVkOif9V-iu2JHJm3iYlQFqEg",
        "controller": "did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3",
        "type": "JsonWebKey",
        "publicKeyJwk": {
          "kty": "OKP",
          "alg": "EdDSA",
          "kid": "huD6Vz6OdW6dvRjdN6tVkOif9V-iu2JHJm3iYlQFqEg",
          "crv": "Ed25519",
          "x": "MqYzktzRhiODVlbqtWGMq9mK-99mYEv6enj5p8m_GFY"
        }
      }
    ]
  },
  "meta": {
    "created": "2024-02-15T12:02:28Z",
    "updated": "2024-02-15T12:02:28Z",
    "governorAddress": "rms1qraf9e0el2pyf3h0zndktappwsamq6c7rt2mj853u3nuhs5ps2pf5yf86ae",
    "stateControllerAddress": "rms1qraf9e0el2pyf3h0zndktappwsamq6c7rt2mj853u3nuhs5ps2pf5yf86ae"
  }
}

To view the DID go to https://explorer.shimmer.network/testnet/search/did:iota:rms:0xaacacbf88f43e59843c6e9eb2a993ffd37e9ef774cc9c0377b1a41aa15cb6fb3
To view the NFT go to https://explorer.shimmer.network/testnet/search/0xe52cfc0da5da119e542c41a47a3b2df8937ef86c00c9ae30229ec78c2e380cf0
```
