# Diamond Drops
[![Twitter](https://img.shields.io/twitter/follow/DropsofDiamond.svg?style=social)](https://twitter.com/DropsOfDiamond)
[![TravisCI](https://img.shields.io/travis/Drops-of-Diamond/diamond_drops/master.svg)](https://travis-ci.org/Drops-of-Diamond/diamond_drops)
[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/diamond_drops/blob/master/LICENSE)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Percentage of issues still open")
<!--Commenting out because it is unknown, codecov has not been set up. Refer to issue #11 and https://github.com/codecov/example-rust. [![codecov](https://codecov.io/gh/Drops-of-Diamond/diamond_drops/branch/master/graph/badge.svg)](https://codecov.io/gh/Drops-of-Diamond/diamond_drops)-->

Get in touch with us on Gitter: 
[![Gitter: DropsOfDiamond/Lobby](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Lobby-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Lobby)
[![Gitter: DropsOfDiamond/Development](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Development-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Development)

### What are we building?

We're implementing the Ethereum [Draft Sharding Phase 1 Specification](https://ethresear.ch/t/sharding-phase-1-spec/1407) by:

* Developing a CLI interface for a Sharding Client in Rust for Proposers and Collators to use
* Developing [Sharding Manager Contract (SMC) in the Vyper language](https://github.com/Drops-of-Diamond/sharding/blob/develop/smc/Sharding_Manager_Contract.v.py), which is a smart contract that will be deployed on the Ethereum Mainnet for the Sharding Clients to interact with. 
* Researching how to implement a Sharding P2P Network Protocol.
* Implementing further planned additions in the sharding roadmap, which is available in the above draft specification.
  
For further information please refer to the [Introduction and onboarding process for new developers](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers).

### Setup guide

#### Install dependencies

  * [Install Rust](https://github.com/rust-lang/book/blob/master/2018-edition/src/ch01-01-installation.md)

  * **Update Rust**
    ```bash
    rustup update
    ```

#### Clone a fork of the repository

  * Fork the repository https://github.com/Drops-of-Diamond/diamond_drops

  * Clone your fork of the repository (replace <USERNAME> below with your Github username):
    ```bash
    git clone https://github.com/<USERNAME>/Diamond-drops;
    cd Diamond-drops
    ```

  * Add the "upstream" repository to your remotes and show your list of remotes verbosely
    ```bash
    git remote add upstream https://github.com/Drops-of-Diamond/diamond_drops;
    git remote --verbose
    ```

  * Change from the "master" branch to the "develop" branch to see the latest features that are being integrated but are not officially ready for production

#### Run and Test the code

  * Build and run the code and provide arguments that it accepts from the CLI
    ```bash
    cargo run -- -mode b
    ```

    * Note: CLI Options available may include:
      * Proposer Mode: `cargo run -- -mode p`
      * Collator Mode: `cargo run -- -mode c`
      * Both Proposer and Collator Mode: `cargo run -- -mode b`

  * Run tests
    ```bash
    cargo test
    ```

See this wiki article [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Contributing-guidelines).

### View UML Diagram

  * UML diagram is updated each time `cargo build` is run and stored in /diagrams/ml.svg.
    ```bash
    open -a "Google Chrome" ./diagrams/ml.svg
    ```
![](
https://raw.githubusercontent.com/Drops-of-Diamond/diamond_drops/master/diagrams/ml.svg?sanitize=true)

### We're looking for more developers!

See [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers) for information.

### Introduction

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction. For information on sharding and implementations, refer to [here](https://github.com/ethereum/wiki/wiki/Sharding-introduction-and-implementations).

### Why Rust?

After comparing Rust, C++, Go, Javascript and Python, as well as corresponding implementations (Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust is most preferable, while I haven't tried being a user of other implementations, so I can't comment on a comparison. Rust has advantages such as safety, concurrency, practicality, better memory safety, zero-cost abstractions, and support for functional and imperative-procedural paradigms.

### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.
