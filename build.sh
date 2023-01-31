#!/usr/bin/sh

clear

cargo build

mkdir -p anigo/plugins
mv target/debug/libanigo.so anigo/plugins/libanigo.so

cargo run

#RUST_BACKTRACE=1
#printf "$RUST_BACKTRACE\n"
#cargo run --release