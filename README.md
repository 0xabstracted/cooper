[![Crate](https://img.shields.io/crates/v/cooper)](https://crates.io/crates/cooper)
[![Downloads](https://img.shields.io/crates/d/cooper)](https://crates.io/crates/cooper)
[![Build Status](https://img.shields.io/github/workflow/status/samuelvanderwaal/cooper/CI)](https://github.com/samuelvanderwaal/cooper/actions)
[![License](https://img.shields.io/crates/l/cooper)](https://github.com/samuelvanderwaal/cooper/blob/main/LICENSE)

# Cooper

![cooper logo](mb_logo.gif?raw=true)

## Overview

The Solana Metaplex NFT 'Swiss Army Knife' tool.

Features:

-   Decode the metadata of a token mint account

-   Mint new NFTs from a JSON file or URIs

-   Set `primary_sale_happened` bool on an NFT's metadata

-   Set `update_authority` address on an NFT's metadata

-   Verify a creator by signing the metadata accounts for all tokens in a list, for a given tars id or a single mint account

-   Get a snapshot of current NFT holders for a given tars ID or update authority

... and more! See the [docs](https://cooper.rs) for full features and usage instructions.


Suggestions and PRs welcome!

**Note: This is experimental software for a young ecosystem. Use at your own risk. The author is not responsible for misuse of the software or for the user failing to test specific commands on devnet before using on production NFTs.**


## Installation

### Install Binary
Copy the following to a terminal:

```bash
bash <(curl -sSf https://raw.githubusercontent.com/samuelvanderwaal/cooper/main/scripts/install.sh)
```

If you get errors you may need dependencies:

Ubuntu:

```bash
sudo apt install libssl-dev libudev-dev pkg-config
```

MacOS may need openssl:

```bash
brew install openssl@3
```

### Binaries

Linux, MacOS and Windows binaries available in [releases](https://github.com/samuelvanderwaal/cooper/releases), thanks to CI work done by [Kartik Soneji](https://github.com/KartikSoneji).

### Install From crates.io

```bash
cargo install cooper
```

### Install From Source

Requires Rust 1.58 or later.

Install [Rust](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the source:

```bash
git clone git@github.com:samuelvanderwaal/cooper.git
```

or

```bash
git clone https://github.com/samuelvanderwaal/cooper.git
```

Change directory and check out the `main` branch:

```bash
cd cooper
git checkout main
```

Install or build with Rust:

```bash
cargo install --path ./
```

or

```bash
cargo build --release
```



## Contact

Email: sam@vanderwaal.dev

Twitter: [@samvwaal](https://twitter.com/samvwaal)

Discord: @archaeopteryx#7615
