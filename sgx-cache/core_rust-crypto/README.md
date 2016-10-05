# Rust-Crypto

**This is a modification of the Rust Crate *rust-crypto*** (<https://github.com/DaGenix/rust-crypto/>)

1. Runs with #[!no_std]
2. Everything that is not necessary for AES-GCM is removed.


A (mostly) pure-Rust implementation of various common cryptographic algorithms.

Rust-Crypto seeks to create practical, auditable, pure-Rust implementations of common cryptographic
algorithms with a minimum amount of assembly code where appropriate. The x86-64, x86, and
ARM architectures are supported, although the x86-64 architecture receives the most testing.

Rust-Crypto targets the current, stable build of Rust.
If you are having issues while using an older version, please try upgrading to the latest stable.

Rust-Crypto has not been thoroughly
audited for correctness, so any use where security is important is not recommended at this time.

## Usage

To use Rust-Crypto, add the following to your Cargo.toml:

```toml
[dependencies]
core_rust-crypto = "^0.2"
```

and the following to your crate root:

```rust
extern crate crypto;
```

## Contributions

Contributions are extremely welcome. The most significant needs are help
adding documentation, implementing new algorithms,
and general cleanup and improvement of the code. By submitting a pull request you are agreeing to
make you work available under the license
terms of the Rust-Crypto project.

## License

Rust-Crypto is dual licensed under the MIT and Apache 2.0 licenses, the same licenses
as the Rust compiler.

## Algorithms

Rust-Crypto already supports a significant number of algorithms and with your help
it will support even more in the future. Currently supported algorithms include:

* AES
* Ghash
