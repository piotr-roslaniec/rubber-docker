#!/bin/bash

cargo build && sudo RUST_BACKTRACE=1 ./target/debug/rubber-docker run --command /bin/ls -lh /
