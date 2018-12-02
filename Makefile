.PHONY: all build doc

all: build

build:
	@ nix-build -A rust-in-action

doc:
	@ cargo doc --no-deps --bin ch3-mock-files

Cargo.lock: Cargo.toml
	@ nix-shell --pure -A env --run 'cargo update'

README.md: $(wildcard src/bin/*.org)
	@ pandoc -f org -t gfm -o $@ $^

