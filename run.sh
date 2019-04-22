#!/bin/bash

cargo build && sudo ./target/debug/rubber-docker run --command /usr/bin/whoami
