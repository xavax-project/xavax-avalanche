# xavax-avalanche
[![Test Status](https://github.com/diinki/xavax-avalanche/workflows/Rust/badge.svg?event=push)](https://github.com/diinki/xavax-avalanche/actions)

A Rust implementation of the entire Avalanche network transaction format
including serialization, with optional Features which can add:

* Transaction Signing
* Avalanche and EVM cryptography (Creating addresses and key-chains).
* The Avalanche node JSONRPC API (get info from nodes, or submit transactions!)

This SDK is an all-in-one Crate which can help you build anything your heart desires for the avalanche network! (wip).

See official xavax docs for the xavax-avalanche SDK at [the xavax docs page](https://docs.xavax.io).
___
## What can xavax-avalanche do?

**With no features selected, this crate can:**

Create transactions, or any data-type defined in the [AVM](https://docs.avax.network/build/references/avm-transaction-serialization#signed-transaction),  [PVM](https://docs.avax.network/build/references/platform-transaction-serialization) and Atomic EVM transaction format for avalanche.
You can also Parse from a raw encoded payload to a data-type and back!


**`request-api` feature:**

The `request-api` feature adds support for the Avalanche JSON RPC API, and also
allows posting the https request and get responses accordingally, with this feature, you can indeed do anything from get the current transaction-fees, to sending a raw transaction to the network!

**`crypto-api` feature:**

The `crypto-api` feature adds Keychain creation for Avalanche and the EVM! As well as the secp256k1 cryptographic primitives, which allows signing transactions.

This feature allows you to create keychains from bip39 seed-phrases, and also allows generating new seed phrases entirely (cryptographically secure).

The feature also adds "interoperable-message" signing, which allows the user to sign messages which aren't transactions safely.

### With All features, you could:
Create an avalanche wallet! Or perhaps create a game of some-sort utilizing avalanche! 

You could compile to WASM, and use the WASM for web-related reasons!

You could also create a subnet on avalanche as well as adding a custom blockchain, what that blockchain is for is something you and your imagination will answer...

Create transactions, Sign them, and Send them.
___

## Some things to point out

* `xavax-avalanche` is currently in very early development, documentation is still very wip as well as the entire SDK.

### Feature completeness:

The Avalanche VM (AVM) is about 99% implemented with CreateAssetTx's being the last ones to be currently missing.

The Atomic EVM format is entirely implemented, this allows importing/exporting to EVM chains such as the C-Chain on avalanche.

The Platform VM (PVM) is entirely implemented, although subnets and custom-chains are still very much in-development by ava-labs.

This library is not fully tested yet, although I believe most things work.

I recommend using this library for research purposes as of today, if you find an issue or a bug, please create an [issue on the github repo](https://github.com/diinki/xavax-avalanche)!
___
## Documentation
`xavax-avalanche` 'n' [friends](https://api.xavax.io) are quite early, documentation will be be introduced with time.

The documentation will be located at:

* [`xavax-api docs`](https://docs.xavax.io) - Custom, better documentation.
* [`docs.rs`](docs.rs/xavax-avalanche/0.1.0-beta0) - Standard rust doc documentation

___

## Usage

Please view the docs for more detailed examples.

Add this line to your `Cargo.toml` file:
```toml
[dependencies]
xavax-avalanche = "0.1.0"
```

If you want the extra features (which are very recommended) file:
```toml
[dependencies]
xavax-avalanche = { version = "0.1.0", features = ["request-api", "crypto-api"]
```

You could create a base transaction, amongst other Tx types:
```rust
use xavax_avalanche::avm::tx_format::*;

//Create a base tx
let mut tx: BaseTx = BaseTx::default();

//You can Manually set the values
tx.type_id = 0;
tx.network_id = 5;

//Turn the payload into bytes:
let payload = tx.to_bytes();

// Or encode the BaseTx into a CB58 string (this the format in which it will be sent to the avalanche network when signed)
let cb58_encoded_tx = tx.to_cb58();

```
You can parse an existing byte-payload into a struct:

```rust
use xavax_avalanche::avm::tx_format::*;

let mut tx: BaseTx = BaseTx::default();

/* pretend that this is a BaseTx byte payload */
let tx_bytes: Vec<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0,].to_vec();

/* you could also decode CB58 to bytes */
let tx_bytes: Vec<u8> = decode_cb58("cSjfNPretendthatthisisaCB58StringPayload!FGlweODksd");

//Parse the bytes into a BaseTx data struct.
tx.from_bytes(&tx_bytes);
```

___
 ## Changes, Roadmap & Info
 The crate is really early, so docs, more tests, etc are in development, please feel free to follow me or xavax, or create github issues or pull requests, etc!

 Information about the API can always be found at [xavax](https://api.xavax.net)

 ### Follow
 The API is unfinished at the current time, the way its designed might change in the future.
 [twitter](https://twitter.com/DiinkiTheImp).
 [xavax](https://www.xavax.io).

 ___

 ## License
 xavax-avalanche is distributed with the permissive [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt) License <3

 ### Have a good time, all the time.
 
