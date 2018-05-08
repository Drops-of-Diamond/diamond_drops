# Diamond Drops
[![Twitter](https://img.shields.io/twitter/follow/DropsofDiamond.svg?style=social)](https://twitter.com/DropsOfDiamond)
[![TravisCI](https://img.shields.io/travis/Drops-of-Diamond/diamond_drops/master.svg)](https://travis-ci.org/Drops-of-Diamond/diamond_drops)
[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/diamond_drops/blob/master/LICENSE)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Percentage of issues still open")
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
<!--Commenting out because it is unknown, codecov has not been set up. Refer to issue #11 and https://github.com/codecov/example-rust. [![codecov](https://codecov.io/gh/Drops-of-Diamond/diamond_drops/branch/master/graph/badge.svg)](https://codecov.io/gh/Drops-of-Diamond/diamond_drops)-->

Get in touch with us on Gitter: 
[![Gitter: DropsOfDiamond/Lobby](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Lobby-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Lobby)
[![Gitter: DropsOfDiamond/Development](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Development-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Development)

[Gitter room for discussion on sharding, e.g. the latest developments in research and implementations](https://gitter.im/ethereum/sharding).

### What are we building?

We are working on a [sharding](https://github.com/ethereum/wiki/wiki/Sharding-introduction-R&D-compendium) implementation for [Ethereum](https://ethereum.org) with the [Rust](https://www.rust-lang.org/en-US/) programming language, which is fast, safe and concurrent. The below information may be outdated. For the latest information, please ask on Gitter, while additional information on plans and work tasks is in the issues and [projects](https://github.com/Drops-of-Diamond/diamond_drops/projects).

We're implementing sharding according to collaboration with Ethereum research at https://ethresear.ch and other teams, which includes tasks outlined in the projects.
  
For further information please refer to [our wiki](https://github.com/Drops-of-Diamond/diamond_drops/wiki).

### Help support us!

Send a donation to our multi-sig wallet at [0x6D446f9545dBC380A6BBDde8A285A7A8030D4381](https://etherscan.io/address/0x6d446f9545dbc380a6bbdde8a285a7a8030d4381).

### We're looking for more developers!

See [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers) for information. There is a lot of work to do in the [sharding roadmap](https://github.com/ethereum/wiki/wiki/Sharding-roadmap).

### Setup guide

#### Show help menus

* Command help
```
cargo run -- --help
```

* Sub-command "mode" help
```bash
cargo run -- mode --help
```

#### Install dependencies

  * [Install Rust](https://github.com/rust-lang/book/blob/master/2018-edition/src/ch01-01-installation.md)

  * **Update Rust**
    ```bash
    rustup update
    ```

  * Install [Rust Formatter](https://github.com/rust-lang-nursery/rustfmt)
    ```bash
    rustup component add rustfmt-preview
    ```

  * Install [Cargo-make](https://github.com/sagiegurari/cargo-make) Task runner and build tool
    ```bash
    cargo install --force cargo-make
    ```

  * Execute specific task runner flow. Default is Makefile.toml
    ```bash
    cargo make --makefile tasks.toml <TASK_NAME>
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

#### Build, Run the code

  * Proposer Mode
    ```bash
    cargo make p
    ```

  * Notary Mode
    ```bash
    cargo make n
    ```

  * Both Proposer and Notary Modes
    ```bash
    cargo make b
    ```
  
  * All above
    ```bash
    cargo make all
    ```

#### Unit and Integration Tests

  * Run all tests (unit and integration tests for main package and libraries) 
    ```bash
    cargo make test-all
    ```

  * Watch all tests (polls continuously for code changes during development, automatically running tests, and reports issues)
    ```bash
    cargo make build;
    cargo install cargo-watch;
    cargo make watch;
    ```

### Show Rust Docs

```bash
cargo make docs
```

### View UML Diagram

* View UML with Google Chrome, Mozilla Firefox, or Brave browser

    ```bash
    cargo make uml-chrome;
    cargo make uml-firefox;
    cargo make uml-brave;
    ```

* Custom 
  * Choose an application on your computer to open the SVG (i.e. `ls /Applications`)
  * Run `open -a "<APPLICATION>" "./diagrams/ml.svg"`
  * Optionally create a Pull Request to update Makefile.toml

![](
https://raw.githubusercontent.com/ltfschoen/Diamond-drops/develop/diagrams/ml.svg?sanitize=true)

### Contibuting guidelines

See this wiki article [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Contributing-guidelines).

### Introduction

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction. For information on sharding and implementations, refer to [here](https://github.com/ethereum/wiki/wiki/Sharding-introduction-and-implementations).

### Why Rust?

After comparing Rust, C++, Go, Javascript and Python, as well as corresponding implementations (Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust is most preferable, while I haven't tried being a user of other implementations, so I can't comment on a comparison. Rust has advantages such as safety, concurrency, practicality, better memory safety, zero-cost abstractions, and support for functional and imperative-procedural paradigms.

### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.
