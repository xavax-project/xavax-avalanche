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

I recommend using this library for research purposes only.
___
## Documentation
`xavax-avalanche` 'n' [friends](https://api.xavax.io) will have quite a lot of documentation and examples, this includes the Metro wallet itself as an example *(although the early wallet will have a lot of spaghetti code)*.

That being said, there are many examples that you can read about and try here:

* [`xavax-api docs`](https://api.xavax.io) - Includes examples, and a few videos.
* [`docs.rs`](docs.rs/xavax-avalanche/0.1.0-beta0) - Standard documentation

*ps, this crate is very early and I am lazy so I'm future-proofing this README, some of these links may be temporarily invalid as a result...*
___

## Usage
At first, add this to your `Cargo.toml` file:
```toml
[dependencies]
xavax-avalanche = "0.1.0-beta"
```

You could create a base transaction, among other transactions:
```rust
use xavax_avalanche::avm::tx_format::*;

//Create a base tx
let mut tx: BaseTx = BaseTx::default();
//Manually set the values
tx.type_id = 0;
tx.network_id = 5;

//Turn the tx into a serialized byte payload
let payload = tx.to_bytes();

//Or parse a byte-payload into a base_tx:
let mut tx: BaseTx = BaseTx::default();

let tx_bytes: Vec<u8> = [0, 0, 0, 0, 0, 0, 0, 0, 0,].to_vec();
tx.from_bytes(&tx_bytes);
```
ps: the xavax-api *which might still be upcoming* will automatically generate
different transactions types

Then go read some examples, maybe watch the tutorial videos that I might make some day, and experiment!

 Although I heavily recommend checking out the full `xavax-api` crate, as this crate alone simple handles transaction formats for the Avalanche virtual machine!
___
 ## Changes, Roadmap & Info
 The entire `xavax-api` with its dependencies such as `xavax-avalanche` are very, very early, while I will try to keep the main API fairly consistent and stable, the back-end will get a lot of optimizations in the future. Not to mention the addition of better docs.

 More information about the future as well as a roadmap of the api can be found at the [xavax](https://api.xavax.net)

 ### Temorary note:
 The API is unfinished at the current time, if you want more information about what the full API will be you can follow me at [twitter](https://twitter.com/DiinkiTheImp), or look at the [xavax introduction](https://kayowo.net/Logs/XavaxIntroduction/) blog.
 ___

 ## License
 xavax-avalanche is distributed with the [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt) License.
 
