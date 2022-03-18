#!/usr/bin/env sh
svd2rust -i S32K118.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
