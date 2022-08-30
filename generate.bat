echo off
echo svd2rust and form are required by this script.
echo Install with cargo install.

echo ** Running svd2rust
svd2rust -i svd/psoc6_01.svd
rmdir /s /q src
echo ** Running form
form -i lib.rs -o src/
echo ** Tidying up
del lib.rs
cargo fmt

echo All Done.
