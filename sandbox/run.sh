#!/usr/bin/env bash

set -xe

INPUT="$(realpath "$1")"
shift 1

cd "$(dirname "$0")"

CARGO_FLAGS=(-psandbox)
export CARGO_TARGET_DIR=afl-target

cargo afl run "${CARGO_FLAGS[@]}" < "$INPUT"
