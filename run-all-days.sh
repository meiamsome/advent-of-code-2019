#!/bin/bash
set -e

for folder in day-*/part-*; do
  pushd $folder
  cat stdin.txt 2>/dev/null | cargo run --release
  popd
done
