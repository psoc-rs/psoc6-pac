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
psoc6-pac = "0.0.1"
```

Check the [API Documentation](https://docs.rs/psoc6-pac/) for how to use the
crate's functionality.

## Rust version support

This crate supports the 3 latest stable Rust releases. Bumping the minimum
supported Rust version (MSRV) is not considered a breaking change as long as
these 3 versions are still supported.

The MSRV is also explicitly tested against in [.travis.yml](.travis.yml).

## Development

To regenerate the bindings, make sure that the current versions of `svd2rust`
and `form` are installed, and that you're on the current stable Rust version.

> __NOTE__: The unreleased version of `svd2rust` is required to generate the
> bindings currently. You can install this version via the following command:
>
> ```
> cargo install --git https://github.com/rust-embedded/svd2rust.git --force svd2rust
> ```

Then run this script to regenerate the bindings:

```
./generate.sh
```
