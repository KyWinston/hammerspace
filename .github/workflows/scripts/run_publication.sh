#!/bin/bash
set -xeu

IFS=";"
for crate_path in $CRATE_PATHS; do
    cd "$crate_path"
    cargo publish --token "$CRATES_TOKEN"
    cd - || exit 1
done