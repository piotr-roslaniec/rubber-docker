#!/bin/bash

PID=$(cat container.pid)
if [ -n "$PID" ] && [ -e "/proc/$PID" ]; then
    sudo kill -9 "$PID"
fi

cargo build &&
    sudo RUST_BACKTRACE=1 DEBUG=1 \
        ./target/debug/rubber-docker \
        run --image-name ubuntu --command bash
