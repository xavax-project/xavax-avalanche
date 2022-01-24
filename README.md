# xavax-avalanche
[![Test Status](https://github.com/diinki/xavax-avalanche/workflows/Rust/badge.svg?event=push)](https://github.com/diinki/xavax-avalanche/actions)

## All-in-one SDK for the Avalanche Network:
#### Transaction-format parsing, Cryptography, and more!

* Transaction Signing
* AVM and PVM Transaction format.
* Avalanche and EVM cryptography (Bech32 addresses, managing Keychains, BIP39 & BIP44 for HD wallets and mnemonic phrases, etc).
* The Entire Avalanche node JSONRPC API (get info from nodes, or submit transactions!)

This SDK is an all-in-one Crate which can help you build anything your heart desires on-top of the avalanche network! (wip).

#### See documentation at [docs.xavax.io](https://docs.xavax.io).


## Details: What can xavax-avalanche do?

**With no features selected, this crate can:**

Create transactions, or any data-type defined in the [AVM](https://docs.avax.network/build/references/avm-transaction-serialization#signed-transaction),  [PVM](https://docs.avax.network/build/references/platform-transaction-serialization) and Atomic EVM transaction format for avalanche.
You can also Parse from a raw encoded payload *(CB58 encoding, which is the encoding used in the avalanche network)* to a Rust data-type and back!


**`request-api` feature:**

The `request-api` feature adds support for the Avalanche JSON RPC API, and also
allows posting the https request and get responses accordingally, with this feature, you can indeed do anything from get the current transaction-fees, to sending a raw transaction to the network!

The request-api features the AVM & PVM json-rpc, as well as Ortelius indexer RPC. 

**`crypto-api` feature:**

The `crypto-api` feature adds Keychain creation for Avalanche and the EVM, as well as signing & verifying! That means secp256k1, SHA256, KECCAK256, and RIPEMD160.


This feature allows you to create keychains from bip39 seed-phrases, and also allows generating new seed phrases *(with a cryptographically secure source of entropy, crypto-rng).*

### With All features, you could:
> Create an avalanche wallet! Or perhaps create a game utilizing avalanche! 

> You could compile to WASM, and use the compiled WASM for web-related reasons!

> You could also create a subnet on avalanche as well as adding a custom blockchain, what that blockchain is for is something you and your imagination will answer...

> You could use the Avalanche JSONRPC and get any info you could ever want from avalanche nodes!

#### Create transactions, Sign them, and Send them!


## Some things to point out

* `xavax-avalanche` is currently in very early development, documentation is still very wip as well as the entire SDK.
Feel free to ask questions about the SDK, there will be constant updates to the Docs and more examples using the SDK
will be present.

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
* [`docs.rs`](docs.rs/xavax-avalanche/) - Standard rust doc documentation

___

## Usage & Examples

Please view the docs for more detailed examples.

Add this line to your `Cargo.toml` file:
```toml
[dependencies]
xavax-avalanche = "0.1.0"
```

If you want the extra features (which are very recommended):
```toml
[dependencies]
xavax-avalanche = { version = "0.1.0", features = ["request-api", "crypto-api"] }
```

You could create a BaseTx (a default transaction), amongst other Tx types:
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
 The crate is really early; docs, more tests, etc are in development, please feel free to follow me or xavax, or create github issues or pull requests!

 Information about the API can always be found at the xavax website.


 find more info on [xavax.io](https://www.xavax.io) and  [twitter](https://twitter.com/DiinkiTheImp)

 ___

 ## License
 xavax-avalanche is distributed with the permissive [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt) License <3
 ___

 ### Now go have a good time, all the time...
 
