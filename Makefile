V1 := color-v1 types-v1 fs-v1 vec-v1 http-v1 os-v1
V2 := color-v2 types-v2 fs-v2 vec-v2 http-v2 os-v2

.PHONY: all test test-v1 test-v2 fmt fmt-check clippy clippy-v1 clippy-v2 lint check clean help

all: test

test: test-v1 test-v2

test-v1:
	cargo test --features "$(V1)"

test-v2:
	cargo test --features "$(V2)"

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy: clippy-v1 clippy-v2

clippy-v1:
	cargo clippy --all-targets --features "$(V1)" -- -D warnings

clippy-v2:
	cargo clippy --all-targets --features "$(V2)" -- -D warnings

lint: fmt-check clippy

check: test lint

clean:
	cargo clean

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "  test        Run tests for both generations (v1 + v2)"
	@echo "  test-v1     Run tests for v1 generation"
	@echo "  test-v2     Run tests for v2 generation"
	@echo "  fmt         Format code"
	@echo "  fmt-check   Check formatting"
	@echo "  clippy      Run clippy for both generations (-D warnings)"
	@echo "  clippy-v1   Run clippy for v1 (-D warnings)"
	@echo "  clippy-v2   Run clippy for v2 (-D warnings)"
	@echo "  lint        fmt-check + clippy"
	@echo "  check       test + lint (everything)"
	@echo "  clean       cargo clean"
	@echo "  help        Show this help"
	@echo ""
	@echo "Note: do NOT use 'cargo test --all-features' — v1/v2 are mutually exclusive."
