# xavax-avalanche
[![Test Status](https://github.com/diinki/xavax-avalanche/workflows/Rust/badge.svg?event=push)](https://github.com/diinki/xavax-avalanche/actions)

An implementation of the Avalanche Virtual Machine transaction format.

`xavax-avalanche` in a nutshell:

* Parsing to and from bytes that represent AVM (avalanche virtual machine) Transactions.
* All [`AVM`](https://docs.avax.network/build/references/avm-transaction-serialization#signed-transaction) and [`PVM`](https://docs.avax.network/build/references/platform-transaction-serialization) data-types.
* [`CB58`](https://support.avax.network/en/articles/4587395-what-is-cb58) Encoding/Decoding.
* Creates byte payloads [`ready`](https://docs.avax.network/build/references/serialization-primitives) to be sent to avalanche nodes!

All that is missing is signing transaction and key management, and all that is taken care of by the `xavax-crypto` crate! *(see the xavax-api crate for a full avalanche network SDK)*

## Some things to point out

>`xavax-avalanche` is currently in very early development, but optimizations and better
docs should be available not too long after release!

>`xavax-avalanche `is fairly simple in principle and is used together with other APIs in the `xavax-api` crate, this includes `xavax-crypto` and `xavax-eth`.

>All the xavax crates including `xavax-avalanche` are used in open-source products by xavax, an example of which is the [`Avalanche`](https://www.avax.network/) Metro desktop wallet.


## Documentation
`xavax-avalanche` 'n' [`friends`](https://api.xavax.net) have quite a lot of documentation and examples, this includes the Metro wallet (although the early wallet will have a lot of spaghetti code).

That being said, there are many examples that you can read about and try here:

* [xavax-api docs](https://api.xavax.net/docs) - Includes examples, and a few videos.
* [docs.rs](docs.rs/xavax-avalanche/0.1.0-beta0) - Standard documentation

*ps, this crate is very early and I am lazy so I'm future-proofing the README,some of these links may be temporarily invalid as a result...*

## Usage
At first, add this to your `Cargo.toml` file:
```toml
[dependencies]
xavax-avalanche = "0.1.0-beta"
```
Then go read some examples, maybe watch the tutorial videos that I might make some day, and experiment!

 Although I heavily recommend checking out the full `xavax-api` crate, as this crate alone simple handles transaction formats for the Avalanche virtual machine!

 ## Changes and Roadmap
 The entire `xavax-api` with its dependencies such as `xavax-avalanche` are very, very early, while I will try to keep the main API fairly consistent and stable, the back-end will get a lot of optimizations in the future. Not to mention the addition of better docs.

 More information about the future and roadmap of the api can be found at the [`xavax`](https://api.xavax.net)

 ## License
 xavax-avalanche is distributed with the Apache 2.0 License.
 
