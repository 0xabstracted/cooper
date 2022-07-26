## Quick Start

### Install Binary

Copy the following to a terminal:

```
bash <(curl -sSf https://raw.githubusercontent.com/samuelvanderwaal/cooper/main/scripts/install.sh)
```

If you get errors you may need dependencies:

Ubuntu:

```
sudo apt install libssl-dev libudev-dev pkg-config
```

MacOS may need openssl:

```
brew install openssl@3
```

Or get the binary yourself: [binary](https://github.com/samuelvanderwaal/cooper/releases).

### Install With Cargo

So you're a Rust dev. . .

```bash
cargo install cooper
```

### See Usage Commands

```bash
cooper -h
```

Decode a mint account's metadata:

```bash
cooper decode mint -a 23gaZq8578xHozMWADsmZ2hAFqZ15iyHmQtRw14meds2
```

Get a snapshot of mint accounts by first verified creator:

```bash
cooper snapshot mint -c PanbgtcTiZ2PveV96t2FHSffiLHXXjMuhvoabUUKKm8
```