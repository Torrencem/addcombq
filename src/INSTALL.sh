#!/usr/bin/env bash

if hash ~/.cargo/bin/cargo 2> /dev/null;
then
	~/.cargo/bin/cargo build --release
else
	echo "Cargo (and so Rust) is not installed. Pre-build binaries are not available, so install Rust (step-by-step guide here: https://doc.rust-lang.org/cargo/getting-started/installation.html)"
	exit 1
fi

mkdir -p build

cp ./target/release/libpf_addcomb.so ./build/addcomb.so

python3 setup.py install
