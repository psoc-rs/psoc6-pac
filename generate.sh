#!/bin/sh

set -ex

svd2rust -i  svd/psoc6_01.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
