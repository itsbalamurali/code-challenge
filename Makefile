export GIT_SHA := $(shell git rev-parse HEAD)

all: clean test build

.PHONY: test
test:
	cargo test

build:
	cargo build

.PHONY: clean
clean:
	cargo clean