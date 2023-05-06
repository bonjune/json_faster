all:
	cargo build --release
	python ./perf_test.py

build:
	cargo build --release

.PHONY: all build