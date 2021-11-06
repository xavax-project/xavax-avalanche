# xavax-avalanche
[![Test Status](https://github.com/diinki/xavax-avalanche/workflows/Rust/badge.svg?event=push)](https://github.com/diinki/xavax-avalanche/actions)

A Rust implementation of the Avalanche Virtual Machine transaction format,
including serialization.

`xavax-avalanche` in a nutshell:

* Parsing to and from bytes that represent AVM (avalanche virtual machine) Transactions.
* All [AVM](https://docs.avax.network/build/references/avm-transaction-serialization#signed-transaction) and [PVM](https://docs.avax.network/build/references/platform-transaction-serialization) data-types.
* [CB58](https://support.avax.network/en/articles/4587395-what-is-cb58) Encoding/Decoding.
* Creates byte payloads [ready](https://docs.avax.network/build/references/serialization-primitives) to be sent to avalanche nodes!

All that is missing is signing transactions and keychain management, and all that is taken care of by the `xavax-crypto` crate! *(see the xavax-api crate for a full avalanche network SDK)*

___

## Some things to point out

* `xavax-avalanche` is currently in very early development, but optimizations and better docs should be available not too long after release! Don't be surprised if there are bugs.


* `xavax-avalanche` is fairly simple in principle and is used together with other libraries in the `xavax-api` crate, this includes `xavax-crypto` and `xavax-eth` *(which might be upcoming)*

* All the xavax crates including `xavax-avalanche` are used in open-source projects by xavax, an example of which is the [Metro desktop wallet](https://wallet.xavax.io).

### Feature completeness:

The AVM is about 99% implemented with CreateAssetTx's being the last ones to be currently missing.

The Atmoic EVM tx format is entirely implemented.

The Platform VM (PVM) is currently not implemented.

This library is not fully tested yet, although I believe everything works.

I recommend using this library for research purposes only as of today.
___
## Documentation
`xavax-avalanche` 'n' [friends](https://api.xavax.io) are quite early, documentation will be a later priority.

That being said, there are many examples that you can read about and try here:

* [`xavax-api docs`](https://api.xavax.io) - Includes examples, and a few videos.
* [`docs.rs`](docs.rs/xavax-avalanche/0.1.0-beta0) - Standard documentation

*ps, this crate is very early and I am lazy so I'm future-proofing this README, some of these links may be temporarily invalid as a result...*
___

## Usage
At first, add this to your `Cargo.toml` file:
```toml
[dependencies]
xavax-avalanche = "0.1.0"
```

You could create a base transaction, amongst other Tx types:
```rust
use xavax_avalanche::avm::tx_format::*;

//Create a base tx
let mut tx: BaseTx = BaseTx::default();

//You can Manually set the values
tx.type_id = 0;
tx.network_id = 5;

//Turn the tx into a serialized byte payload
let payload = tx.to_bytes();
```
Or parse existing bytes into a struct

```rust
use xavax_avalanche::avm::tx_format::*;

let mut tx: BaseTx = BaseTx::default();

/*pretend that this is a BaseTx byte payload*/
let tx_bytes: Vec<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0,].to_vec();

//Parse the bytes into a BaseTx data struct.
tx.from_bytes(&tx_bytes);

//Turn the tx into a CB58 encoded payload
let cb58_serialized: String = tx.to_cb58();
```

**ps:** the xavax-api *which might still be upcoming* will have functions that automatically generate different transactions types, and also be able
to automatically sign & send the transaction, the `xavax-avalanche` library is a fairly low-level library.
___
 ## Changes, Roadmap & Info
 The entire `xavax-api` with its dependencies such as `xavax-avalanche` are very, very early, while I will try to keep the main API fairly consistent and stable, the back-end will get a lot of optimizations in the future. Not to mention the addition of better docs.


 More information about the future as well as a roadmap of the api can be found at the [xavax](https://api.xavax.net)

 ### Temorary note:
 The API is unfinished at the current time, if you want more information about what the full API will be you can follow me at [twitter](https://twitter.com/DiinkiTheImp), or look at the [xavax introduction](https://kayowo.net/Logs/XavaxIntroduction/) blog.
 ___

 ## License
 xavax-avalanche is distributed with the [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt) License.
 
