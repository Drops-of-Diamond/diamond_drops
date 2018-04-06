# Diamond Drops
[![Twitter](https://img.shields.io/twitter/follow/DropsofDiamond.svg?style=social)](https://twitter.com/DropsOfDiamond)
[![TravisCI](https://img.shields.io/travis/Drops-of-Diamond/Diamond-drops/master.svg)](https://travis-ci.org/Drops-of-Diamond/Diamond-drops)
[![codecov](https://codecov.io/gh/Drops-of-Diamond/Diamond-drops/branch/master/graph/badge.svg)](https://codecov.io/gh/Drops-of-Diamond/Diamond-drops)
[![License: Unlicense](https://img.shields.io/badge/License-Unlicense-lightgrey.svg)](https://github.com/Drops-of-Diamond/Diamond-drops/blob/master/LICENSE)
[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/Drops-of-Diamond/Diamond-drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/Diamond-drops "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/Drops-of-Diamond/Diamond-drops.svg)](http://isitmaintained.com/project/Drops-of-Diamond/Diamond-drops "Percentage of issues still open")

Get in touch with us on Gitter: 
[![Gitter: DropsOfDiamond/Lobby](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Lobby-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Lobby)
[![Gitter: DropsOfDiamond/Development](https://img.shields.io/badge/gitter-Drops%20of%20Diamond/Development-4AB495.svg)](https://gitter.im/Drops-of-Diamond/Development)

### We're looking for more developers!

See [here](https://github.com/Drops-of-Diamond/Diamond-drops/wiki/Introduction-and-onboarding-process-for-new-developers) for information.

### Introduction

For an introduction to Ethereum, see https://ethereum.org/ or https://github.com/ethereum/wiki/wiki/Ethereum-introduction.

### Why Rust?

After comparing Rust, C++, Go, Javascript and Python, as well as corresponding implementations (Parity, cppethereum, Go-ethereum, ethereumJS and Py-EVM), Rust is most preferable, while I haven't tried being a user of other implementations, so I can't comment on a comparison. Rust has advantages such as safety, concurrency, practicality, better memory safety, zero-cost abstractions, and support for functional and imperative-procedural paradigms.

### What are we building?

We're implementing the [Draft Sharding Phase 1 Specification](https://ethresear.ch/t/sharding-phase-1-spec/1407) by:

* Developing a CLI interface for a Sharding Client in Rust for Proposers and Collators to use
* Developing [Sharding Manager Contract (SMC) in the Vyper language](https://github.com/Drops-of-Diamond/sharding/blob/develop/smc/Sharding_Manager_Contract.v.py), which is a smart contract that will be deployed on the Ethereum Mainnet for the Sharding Clients to interact with. 
* Researching how to implement a Sharding P2P Network Protocol. 
  
For further information please refer to the [Introduction and onboarding process for new developers](https://github.com/Drops-of-Diamond/diamond_drops/wiki/Introduction-and-onboarding-process-for-new-developers).

### How to contribute to this Github repository?

#### Learn to use Git proficiently
  
  * Complete [Git Training](https://github.com/charleso/git-training)

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

#### Contribute changes to an existing remote branch by others

  * Change to the "develop" branch from "master"
    ```bash
    git checkout develop
    ```

  * Fetch latest changes from an "upstream" branch that is in progress. Let's assume the remote branch is called 'upstream/basic_cli', and map it to your local machine
    ```bash
    git fetch upstream basic_cli:basic_cli
    ```

  * Show branches that now include the branch called 'basic_cli' locally
    ```bash
    git branch
    ```

  * Change to the basic_cli branch
    ```bash
    git checkout basic_cli
    ```
  
  * Make changes to the basic_cli branch

  * Prior to pushing changes update the local 'develop' branch and the 'basic_cli' branch with latest changes from upstream develop
    * Change to the 'develop' branch and fetch latest changes from upstream using rebase
      ```bash
      git checkout develop
      ```

    * Fetch latest changes from 'develop' branch
      ```bash
      git pull --rebase upstream develop
      ```

    * Switch back into the 'basic_cli' branch and rebase it interactively with latest changes from 'develop' branch
      ```bash
      git checkout basic_cli;
      git rebase -i develop
      ```

      * Important Note: If you prefer not to use Git Rebase then use Git Merge (i.e. `git merge develop` instead or similar equivalent

  * Push the changes that you made to a remote branch called 'basic_cli' on "origin", which is your fork of the "upstream"
    ```bash
    git push origin basic_cli
    ```
  
  * Create a Pull Request from your remote branch of the "origin" fork into the 'basic_cli' branch of the "upstream" for another user to accept your proposed changes

#### Contribute new changes in a new branch that you author

  * Change to your local 'develop' branch (integration branch for features). If it does not yet exist then create it with `git checkout -b develop`
    ```bash
    git checkout develop
    ```

  * Fetch latest changes from the "upstream" repository that is in progress
    ```bash
    git fetch upstream
    ```
  
  * Create a new branch for your feature or bug fix
    ```bash
    git checkout -b <INSERT_YOUR_BRANCH_NAME>
    ```

  * Make changes to your branch

  * Add and commit changes. Use this [Git Styleguide](https://udacity.github.io/git-styleguide/) for your messages
    ```bash
    git add .;
    git commit -m "<INSERT_YOUR_MESSAGE>"
    ```

  * Prior to pushing changes update the local 'develop' branch and your current branch with latest changes from upstream 'develop' branch
    * Change to 'develop' branch and fetch latest changes from upstream using rebase
      ```bash
      git checkout develop
      ```

    * Fetch latest changes from remote develop branch
      ```bash
      git pull --rebase upstream develop
      ```

    * Switch back into your branch and rebase it interactively with latest changes from the 'develop' branch
      ```bash
      git checkout basic_cli;
      git rebase -i master
      ```
      * Important Note: If you prefer not to use Git Rebase then use Git Merge instead or similar equivalent

  * Push the changes that you made to a remote branch on "origin" with a name matching the branch you created, where "origin" is your fork of the "upstream" repository
    ```bash
    git push origin <INSERT_YOUR_BRANCH_NAME>
    ```
  
  * Create a Pull Request from your remote branch of the "origin" fork into the 'develop' branch of the "upstream" for another user to accept your proposed changes

### Further information

This repo and the Drops of Diamond project it belongs to is not a part of or owned by the Ethereum Foundation, nor is it endorsed by the Foundation. A different project name and logo may be used (the logo could use a more modern design rather than just using a photo in the public domain), and alternative proposals are welcome. The Drops of Diamond project is not legally incorporated as of yet, so legally it is not an organisation. That should be done, but probably only as needed once the project is more well-developed.
