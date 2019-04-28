#!/bin/bash

cargo build &&
    sudo RUST_BACKTRACE=1 DEBUG=1 ./target/debug/rubber-docker run --command hostname
