#!/bin/sh

RUSTFLAGS="-L $(pwd)/lib/athena/shared" cargo build
