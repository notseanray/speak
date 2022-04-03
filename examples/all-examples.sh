#!/bin/bash

for filename in ./*.rs; do
	echo -e "RUNNING EXAMPLE"
	RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo run --example "$(basename "$filename" .rs)"
done