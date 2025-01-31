#!/bin/bash
set -xeu

RUSTFLAGS="-C instrument-coverage" sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev && cargo test --lib
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/ --excl-start "#\[cfg\(test\)\]" --keep-only **/src/**/*