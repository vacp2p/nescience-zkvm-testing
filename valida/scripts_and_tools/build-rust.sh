#!/usr/bin/env bash
TEST_NAME=$1

rustc --emit=llvm-ir -Cpanic="abort" -C opt-level=3  -C target-cpu=generic ../tests/$TEST_NAME.rs
./llc -march=delendum -filetype=obj $TEST_NAME.ll
./ld.lld --script=./valida.ld $TEST_NAME.o -L . -lstdio -o $TEST_NAME
