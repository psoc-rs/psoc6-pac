#!/bin/sh

set -ex

# TODO: You currently need the unreleased version of svd2rust from git for this to work:
# i.e. cargo install --git https://github.com/rust-embedded/svd2rust.git --force svd2rust
svd2rust -i svd/psoc63.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt

# FIXME: The following is a hack to work around https://github.com/rust-embedded/svd2rust/issues/249
sed -i '1s/^/use crate::prot::mpu::MPU_STRUCT;\nuse crate::prot::smpu::SMPU_STRUCT;\n\n/' src/prot.rs
sed -i '1s/^/use crate::ble::rcb::RCBLL;\n\n/' src/ble.rs
