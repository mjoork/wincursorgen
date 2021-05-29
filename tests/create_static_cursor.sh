#!/bin/sh
[ ! -e "./target/release/wincursorgen" ] && cargo build --release
[ ! -d "./tests/cursors" ] && mkdir "./tests/cursors"

./target/release/wincursorgen \
    -p "./tests/images/" \
    "./tests/configs/test-cursor.in" \
    "./tests/cursors/test.cur"