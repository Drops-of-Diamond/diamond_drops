# Diamond Drops _(diamond_drops)_
![Diamond Drops image](https://raw.githubusercontent.com/Drops-of-Diamond/diamond_drops/develop/images/diamond_drops.png)

[![Twitter](https://img.shields.io/twitter/follow/DropsofDiamond.svg?style=social)](https://twitter.com/DropsOfDiamond)
[![TravisCI](https://img.shields.io/travis/Drops-of-Diamond/diamond_drops/master.svg)](https://travis-ci.org/Drops-of-Diamond/diamond_drops)
[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/diamond_drops/blob/master/LICENSE)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Drops-of-Diamond/diamond_drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Percentage of issues still open")
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
[![codecov](https://codecov.io/gh/Drops-of-Diamond/diamond_drops/branch/master/graph/badge.svg)](https://codecov.io/gh/Drops-of-Diamond/diamond_drops)
[![Gitter: DropsOfDiamond/Lobby](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Lobby-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Lobby)
[![Gitter: DropsOfDiamond/Development](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Development-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Development)
[![Gitter: sharding](https://img.shields.io/badge/gitter-sharding-4AB495.svg)](https://gitter.im/ethereum/sharding)
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

> WIP on sharding and Ethereum 2.0 with enshrined-in-consensus data availability and Rust: a fast, safe, concurrent and practical programming language.

This repo contains code that is implemented to a [deprecated sharding specification (notaries and proposers with a sharding manager contract](https://ethresear.ch/t/a-minimal-sharding-protocol-that-may-be-worthwhile-as-a-development-target-now/1650/3?u=justindrake)), as well as previous code for [collators and proposers](https://ethresear.ch/t/sharding-phase-1-spec-retired/1407). The [latest specification](https://notes.ethereum.org/SCIg8AH5SA-O4C1G1LYZHQ) is not finalised. Currently as of July 31 2018 work is underway on the networking aspect, with [gossipsub](https://github.com/libp2p/rust-libp2p/pull/421). Note also that for a more up-to-date, but still outdated, implementation of the first version of the shasper (sharding + Casper) specification, see https://github.com/sigp/lighthouse.

We were working on a [sharding](https://github.com/ethereum/wiki/wiki/Sharding-introduction-R&D-compendium) implementation for [Ethereum](https://github.com/ethereum/wiki/wiki/Ethereum-Introduction) with the [Rust](https://www.rust-lang.org/en-US/) programming language, which is fast, safe and concurrent. See also this ["Why Rust?"](https://medium.com/paritytech/why-rust-846fd3320d3f) post by ParityTech. Sharding plans to scale [Ethereum](https://github.com/ethereum/wiki/wiki/Ethereum-introduction), at first quadratically, then exponentially.

We're implementing sharding according to collaboration with Ethereum research at https://ethresear.ch and other teams, which includes tasks outlined in the projects and issues. In the first phase of the [roadmap](https://github.com/ethereum/wiki/wiki/Sharding-roadmap), we only enshrine data availabilty, without execution, thus abstracting execution, and enabling faster  and simpler implementation, with each phase being useful. For further information, please refer to [our wiki](https://github.com/Drops-of-Diamond/diamond_drops/wiki), which includes an [introductory video](https://youtu.be/h_8nIRTApMc), background info, contributing info, and more. The plan is for this codebase to eventually be integrated into the Rust implementation of the [EVM](https://github.com/ethereum/wiki/wiki/Ethereum-Virtual-Machine-(EVM)-Awesome-List), [Parity](https://github.com/paritytech/parity). diamond_drops is used as the name of the cargo crate for compatibility, and so the repo name also follows this.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Contents**

- [Background](#background)
- [Install](#install)
  - [Install dependencies](#install-dependencies)
  - [Clone a fork of the repository](#clone-a-fork-of-the-repository)
  - [Follow us on Twitter for release updates](#follow-us-on-twitter-for-release-updates)
- [Usage](#usage)
  - [Build and Run the code](#build-and-run-the-code)
  - [Unit and Integration Tests](#unit-and-integration-tests)
  - [Show Rust Docs](#show-rust-docs)
  - [View UML Diagram](#view-uml-diagram)
  - [Contibuting guidelines](#contibuting-guidelines)
- [Security](#security)
- [Help support us](#help-support-us)
- [We're looking for more developers](#were-looking-for-more-developers)
- [Why Rust?](#why-rust)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Background

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction. For information on sharding and implementations, refer to [here](https://github.com/ethereum/wiki/wiki/Sharding-introduction-and-implementations).

## Install

### Install dependencies

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
### Show help menus

* Command help
```bash
cd ~/diamond_drops; # you may wish to rename this to dod for convenience
cargo run -- --help
```

* Sub-command "mode" help
```bash
cargo run -- mode --help
```

### Clone a fork of the repository

  * Fork the repository https://github.com/Drops-of-Diamond/diamond_drops

  * Clone your fork of the repository (replace <USERNAME> below with your Github username):
    ```bash
    git clone https://github.com/<USERNAME>/diamond_drops;
    cd diamond_drops
    ```

  * Add the "upstream" repository to your remotes and show your list of remotes verbosely
    ```bash
    git remote add upstream https://github.com/Drops-of-Diamond/diamond_drops;
    git remote --verbose
    ```

  * Change from the "master" branch to the "develop" branch to see the latest features that are being integrated but are not officially ready for production
  
### Follow us on Twitter for release updates

Click on the Twitter badge above.

## Usage
### Build and Run the code

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

  * All above with collation example
    ```bash
    cargo make all-with-collation
    ```

### Unit and Integration Tests

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
    cargo make uml-default-recommended;
    # The below options will only work for Mac OS X, otherwise use the above.
    cargo make uml-chrome;
    cargo make uml-firefox;
    cargo make uml-brave;
    ```
  * On a Mac you can also use `ls /Applications; open SVG-COMPATIBLE-APPLICATION ./diagrams/ml.svg`.
  * Optionally create a Pull Request to update Makefile.toml or open the diagrams/ml.svg file manually after building with the graphviz dependency for MML or an alternative installed.

![](
https://raw.githubusercontent.com/Drops-of-Diamond/diamond_drops/develop/diagrams/ml.svg?sanitize=true)

### Contibuting guidelines

See this wiki article [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Contributing-guidelines).

## Security

This is WIP, pre-release software. It is planned to be integrated into Parity. There is a lot of research available on security alone; for example, CTRL+F "security" [here](https://github.com/ethereum/wiki/wiki/Sharding-FAQ) or search in the pages [here](https://github.com/ethereum/wiki/wiki/Sharding-introduction-R&D-compendium#information)

## Help support us

Send a donation to jamesray.eth. All contributors will be paid according to their contributions and timesheets, once funds are received. As an example, my timesheet is available [here](https://docs.google.com/spreadsheets/d/1Fv8XqLkMjdBkGPkVWfJulJU-5Qv6TSUR4oD5uKSEHW0/edit#gid=1828020164). A multisig will be used when other developers are contributing full-time. Funds are needed to pay researchers and developers a reasonable hourly rate or salary, which may fairly be quite high due to the time required to learn about Ethereum, sharding, etc.; 
and because of the scarcity of those interested and qualified to be able to contribute in the blockchain industry. For more details see [this issue](https://github.com/Drops-of-Diamond/diamond_drops/issues/18).<!omitting until other regular contributors are well established, although I still vouch that contributions will be paid for according to assessing hours on timesheets--our multi-sig wallet at [0x6D446f9545dBC380A6BBDde8A285A7A8030D4381](https://etherscan.io/address/0x6d446f9545dbc380a6bbdde8a285a7a8030d4381)-->

## We're looking for more developers

See [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers) for information. There is a lot of work to do in the [sharding roadmap](https://github.com/ethereum/wiki/wiki/Sharding-roadmap). 

## Why Rust?

After comparing languages such as Rust, C++, Go, Javascript and Python, as well as implementations (e.g. Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust is most preferable, while I haven't tried being a user much of other implementations, so I can't comment on a comparison. Rust has advantages such as safety, concurrency, practicality, better memory safety (thanks to its ownership rules), zero-cost abstractions, and support for functional and imperative-procedural paradigms.

<!-- Not necessary
### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.
-->

## License

[UNLICENSE](LICENSE)
