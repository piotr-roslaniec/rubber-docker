#!/bin/bash

RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build &&
    sudo RUST_BACKTRACE=1 DEBUG=1 ./target/debug/rubber-docker run --command /bin/echo true
