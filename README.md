# Diamond Drops

[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/Diamond-drops/blob/master/LICENSE)
[![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/Drops-of-Diamond/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Are you interested in making [Ethereum](https://ethereum.org/) faster and more mainstream?

If so, read this first: [the draft phase 1 sharding spec](https://ethresear.ch/t/sharding-phase-1-spec/).

Tasks:

- [ ] WIP: [learn Rust](https://doc.rust-lang.org/book/second-edition). (I'm almost finished, up to part way through chaper 19 out of 21. I will refer back to this and finish reading it on an as-needed basis.)
- [x] Read the draft [phase 1 sharding spec](https://ethresear.ch/t/sharding-phase-1-spec/).
- [x] Start building sharding and Ethereum 2.0 in this repo, which involves:
    - [ ] Use a test-driven development approach to aim for 100% test coverage at all times.
    - [ ] Implement the draft [phase 1 sharding spec](https://ethresear.ch/t/sharding-phase-1-spec/).
    - [ ] Research how to implement a sharding networking protocol.
    - [ ] Research and implement sharding syncing methods.
    - [ ] Contribute to research for further developments in the subsequent phases of the [sharding roadmap](https://ethresear.ch/t/sharding-phase-1-spec).
    - [ ] Research and implement further phases as outlined in the draft phase 1 spec.
- [ ] (ongoing operation): keep track of major [Ethereum Research](https://ethresear.ch) topics, particularly those that are on the [sharding roadmap](https://ethresear.ch/t/sharding-phase-1-spec). For more specific sharding updates, you can contact apply@ethereum.org. But probably only do this on an as-needed basis once you get up to these topics in the development pipeline.
- [ ] Possibly further familiarize with the sharding implementation on top of PyEVM [here](https://github.com/ethereum/py-evm/tree/sharding). Since they are ahead in the development of sharding, it will be useful as a reference implementation, as well as because of Python's readability;
- [ ] Possibly further familiarize with Parity. While it's probably best to understand it in detail that may not be needed. Having an idea of the overall design/structure may be sufficient. However, note that the EVM execution engine in phase 2 may be almost completely rewritten and abstracted to allow multiple execution engines.
- [ ] This is not a high immediate priority as others have already tested Parity with EWasm: further familiarise with [EWasm](https://github.com/ewasm); and
- [x] This is **not essential** according to the [draft phase 1 sharding spec](https://ethresear.ch/t/sharding-phase-1-spec/) since as mentioned, the EVM execution engine may be almost completely rewritten and abstracted to allow multiple execution engines**: read the [Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf) (this is necessary for building a client from scratch, and important for developing one that may be integrated with an existing client like [Parity](https://github.com/paritytech/parity), although with Ethereum 2.0 the EVM may be almost completely rewritten).
- [x] This is **not essential for [phase 1 sharding](https://ethresear.ch/t/sharding-phase-1-spec/)**: read about the [stateless client concept](https://ethresear.ch/t/the-stateless-client-concept/172).
- [x] Possibly not essential (could wait for the draft phase 1 spec to be updated and continue building): familiarize with the (now **depecrated**) Python sharding repo [here](https://github.com/ethereum/sharding/tree/develop/sharding) (a doc is available [here](https://github.com/ethereum/sharding/blob/develop/docs/doc.md)). It may still be useful to refer to how the functions are implemented.
- [x] This is **not essential** but it is helpful to know for development: learn about abstract programming language features.

### Introductions

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction.

For an introduction to WIP sharding implementations, see https://github.com/ethereum/wiki/wiki/Sharding-and-stateless-client-implementations.

### Why Rust?

After comparing Rust, C++, Go, Javascript and Python, as well as corresponding implementations (Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust and Parity seem like they are most preferable. Rust has advantages such as safety, concurrency, practicality, better memory safety, zero-cost abstractions, and support for functional and imperative-procedural paradigms.  It also is compatible with Wasm. More details are here: https://gitter.im/ewasm/Lobby?at=5a92381135dd17022eedc444. Additionally, [Go is not compatible with Wasm at present](https://github.com/golang/go/issues/18892), so [Go-ethereum isn't either](https://github.com/ethereum/go-ethereum/issues/16192). However, Wasm isn't very practical at the moment with its MVP, e.g. it doesn't support parallel threads and other feautures, but it is planned to support other languages and these features.

### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.

At the moment I'm working pro-bono, living off my savings. Once funds are raised or a grant is given by the Ethereum Foundation I can pay other developers. Compensation to developers at this stage can't be offered, as it would depend on some form of revenue, which isn't available at this stage. However, revenue will be more likely to be obtained the more a product is demonstrated. All are welcome to contribute, in the spirit of open-source code and friendly collaboration. If revenue is obtained it will be shared accordingly and fairly with a budget, after setting up a more formal organization, e.g. controlled in a similar fashion to how the Ethereum Foundation is run. (A grant from the Ethereum Foundation has been applied for, as per [this blog post](https://blog.ethereum.org/2018/01/02/ethereum-scalability-research-development-subsidy-programs/).) If you are interested in making a donation but would prefer to not send it to an individual, please say so on Gitter (click the badge above). It's probably best to have an MVP, or alpha or beta release, before actively raising funds. There are alternatives for revenue, e.g. [here](https://ethresear.ch/t/incentives-for-running-full-ethereum-nodes/1239/33).

### Further information about the founding developer, James Ray

Note: I have also worked on other projects with Ethereum, mainly docs such as the [Yellow Paper](https://github.com/ethereum/yellowpaper), [EIPs](https://github.com/ethereum/eips), the White paper, the [Ethereum Wiki](https://github.com/ethereum/wiki/wiki), Solidity, [Casper papers](https://github.com/ethereum/research/tree/master/papers); reading and participating on [ethresear.ch](https://ethresear.ch/), as well as reading and learning with these and other areas such as WebAssembly, Javascript and cryptography (and the maths behind it). For day-to-day details of what I've done roughly since June 2017 you can refer to [here](https://docs.google.com/spreadsheets/d/1dDglpWBhWlPyv0tfDntPQc8F-yBsP41wFQnIKgwA068/edit#gid=114542833). I started working on Ethereum in January 2017 by learning about half of C++ at learncpp.com, with the intention to work on the C++ client, but have since changed plans and work area several times as I learned more about Ethereum.
