# source for all athn programs


## Overview

The programs utilize solana native programs


## Prereq

Follow direction to install the [Solana Cli](https://docs.solanalabs.com/cli/install)

## Build 

```bash
sudo cargo build-bpf --manifest-path programs/athn/Cargo.toml
```

**Note**

If failure on build due to rustc version mismatch, run this before `cargo build-bpf`:
```bash
solana-install update
```