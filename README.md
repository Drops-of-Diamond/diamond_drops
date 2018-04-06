# Diamond Drops
[![Twitter](https://img.shields.io/twitter/follow/DropsofDiamond.svg?style=social)](https://twitter.com/DropsOfDiamond)
[![TravisCI](https://img.shields.io/travis/Drops-of-Diamond/Diamond-drops/master.svg)](https://travis-ci.org/Drops-of-Diamond/diamond_drops)
[![codecov](https://codecov.io/gh/Drops-of-Diamond/Diamond-drops/branch/master/graph/badge.svg)](https://codecov.io/gh/Drops-of-Diamond/diamond_drops)
[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/diamond_drops/blob/master/LICENSE)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Drops-of-Diamond/Diamond-drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Drops-of-Diamond/Diamond-drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/diamond_drops "Percentage of issues still open")

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

### We're looking for more developers!

See [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers) for information.

### Introduction

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction. For information on sharding and implementations, refer to [here](https://github.com/ethereum/wiki/wiki/Sharding-introduction-and-implementations).

### Why Rust?

After comparing Rust, C++, Go, Javascript and Python, as well as corresponding implementations (Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust is most preferable, while I haven't tried being a user of other implementations, so I can't comment on a comparison. Rust has advantages such as safety, concurrency, practicality, better memory safety, zero-cost abstractions, and support for functional and imperative-procedural paradigms.

### How to contribute to this Github repository?

See this wiki article [here](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Contributing-guidelines).

### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.
