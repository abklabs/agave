#!/usr/bin/env bash

set -xe
cd "$(dirname "$0")"

CARGO_FLAGS=(-psandbox)
export CARGO_TARGET_DIR=afl-target

printf '\x1b[1;38;5;2m=> Configuring cargo-afl (this may take awhile)...\x1b[m\n'
CFG_RESULT=0
cargo afl config --build || CFG_RESULT="$?"

if
  (( CFG_RESULT )) && \
    timeout 2 cargo afl config --build 2>&1 \
      | grep -qi 'already built'
then
  CFG_RESULT=0
  CFG_OUT=''
fi

(( ! CFG_RESULT )) || exit "$CFG_RESULT"

printf '\x1b[1;38;5;2m=> Running cargo afl build...\x1b[m\n'
cargo afl build "${CARGO_FLAGS[@]}"
printf '\x1b[1;38;5;2m=> Running cargo afl fuzz...\x1b[m\n'
cargo afl fuzz -i in -o out "$CARGO_TARGET_DIR"/debug/sandbox
