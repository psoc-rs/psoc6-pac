# PSoC 6 Peripheral Access Crate

[![crates.io](https://img.shields.io/crates/v/psoc6-pac.svg)](https://crates.io/crates/psoc6-pac)
[![docs.rs](https://docs.rs/psoc6-pac/badge.svg)](https://docs.rs/psoc6-pac/)
[![Build Status](https://travis-ci.org/psoc-rs/psoc6-pac.svg?branch=master)](https://travis-ci.org/psoc-rs/psoc6-pac)

This crate provides svd2rust-generated bindings to the peripherals of [PSoC 6]
microcontrollers.

Please refer to the [changelog](CHANGELOG.md) to see what changed in the last
releases.

[PSoC 6]: https://www.cypress.com/products/32-bit-arm-cortex-m4-psoc-6

## Usage

Add an entry to your `Cargo.toml`:

```toml
[dependencies]
psoc6-pac = "0.1.1"
```

To generate the documentation, clone the repo then
```
cd <to psoc6-pac folder>
cargo doc --open
```

## Rust version support

This crate supports the 3 latest stable Rust releases. Bumping the minimum
supported Rust version (MSRV) is not considered a breaking change as long as
these 3 versions are still supported.

The MSRV is also explicitly tested against in [.travis.yml](.travis.yml).

## Development
To simplify initial development the pac is targeting the CY8CPROTO-063-BLE board. This board uses the psoc6_01.svd with both generate scripts hardcoding this version. This version is also used by the CY8CKIT-062-BLE Pioneer board.

To regenerate the bindings, ensure that the current versions of `svd2rust`
and `form` are installed, and that you're on the current stable Rust version.

Then open lib.rs and copy all blocks prepended with #[cfg(not(armv7em))] (all CM0+ specific blocks). To another file.

Then run either run generate.sh or generate.bat (Windows) to regenerate the bindings:

```
./generate.sh
```
or
```
>generate.bat
```

Then copy the #[cfg(not(armv7em))] blocks back into the file.

Removing this manual step is on the todo list.

Forgetting this step will cause a compile error when enabling the rt feature and compiling for the CM0+.
