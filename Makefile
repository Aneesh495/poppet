# Poppet Build and Verification Automation
#
# Copyright (c) 2026 Aneesh Krishna Parthasarathy
# Licensed under the ISC License.

.PHONY: all build test test-lib test-kernels test-rhai test-sim check-dashes loc clean help

all: check-dashes build test

build:
	cargo build --release

test: test-lib test-kernels test-rhai test-sim

test-lib:
	cargo test --lib

test-kernels:
	cargo test --test test_kernels

test-rhai:
	cargo run --release -- test

test-sim:
	python3 tests/test_simulation.py

check-dashes:
	python3 tools/check_no_em_dash.py

loc:
	python3 tools/count_loc.py

clean:
	cargo clean
	rm -rf target/
