#!/bin/bash

cargo build && sudo ./target/debug/rubber-docker run --command /bin/ls /
