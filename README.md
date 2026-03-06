# Kaspa Vanity Address Generator (Rust)

Fast multithreaded vanity address generator for Kaspa written in Rust.

Generate addresses containing custom prefixes or suffixes.

The generator works fully offline, meaning:

- no network connection required
- no node required
- no RPC calls
- no wallet services
- no external APIs

All keys are generated locally on your machine, making the tool secure for cold-wallet generation.

---

# Features

- Fully offline and secure
- Multithreaded
- Extremely fast
- Prefix search
- Suffix search
- JSON export of results
- Clean CLI interface

Typical speeds on modern CPUs:

300k - 2M addresses / second

---

# Installation

Install Rust:

https://rustup.rs

Clone the repository:

git clone https://github.com/k0n1ev/kaspa-vanity
cd kaspa-vanity

Build:

cargo build --release

Run:

./target/release/kaspa-vanity

---

# Example: suffix vanity

Search for an address ending with `kaspa`

./target/release/kaspa-vanity --suffix kaspa

Example output:

Searching mainnet Kaspa vanity address | prefix='' suffix='kaspa' after_q=true threads=8
Tried     28560619 |     394418 addr/s total |    76031 addr/s last sec

Found match!

Address:          kaspa:qqq30xhq386pxpvgjqlnzss8qqjl2dsweqxj2827m7tvyrcq8t62csf7kaspa

Private key hex:  51a7b6ba0701411327f32bc13e79d2746005df02464a0156d1ece5709343325e

XOnly pubkey hex: 01179ae089f4130588903f3142070025f5360ec80d251d5edf96c20f003af4ac

Attempts:         28560610

Elapsed:          71.63 s

Saved to:         kaspa_vanity_result.json

---

# Address character rules

Kaspa addresses use Bech32 encoding.

Allowed characters:

qpzry9x8gf2tvdw0s3jn54khce6mua7l

This means some characters cannot appear in addresses:

1 b i o

---

# Performance

Difficulty grows exponentially with pattern length.

pattern length | expected attempts
3 characters   | ~32k
4 characters   | ~1M
5 characters   | ~33M
6 characters   | ~1B

Typical search times:

4 characters → seconds  
5 characters → seconds / minutes  
6 characters → minutes  
7+ characters → hours+

---

# Security

This tool is designed for secure vanity generation.

Properties:

- Fully offline
- No network activity
- No telemetry
- No dependencies on nodes
- Deterministic cryptographic key generation

You may safely run it on an air-gapped machine.

---

# Example vanity ideas

kaspa:qgpu...
kaspa:qdev...
kaspa:qminer...
kaspa:qpool...

Suffix examples:

...kaspa  
...gpu  
...dev  
...mine

---

# Support the project

If you found this tool useful you can support development:

kaspa:qp83863lsstc5rmm3mppgke0wwyklqdryund4z8e8hnnjrmffcvdsn5c0ffee

---

# License

MIT
