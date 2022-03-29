#!/bin/bash
RUSTDOCFLAGS="--html-in-header ./docs/katex-header.html" cargo doc --no-deps --open