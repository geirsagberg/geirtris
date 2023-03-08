#!/usr/bin/env bash

RUST_BACKTRACE=full cargo watch -w src -w assets -x run --features dev