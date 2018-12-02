.PHONY: all build

all: build

build:
	@ nix-build -A rust-in-action

Cargo.lock: Cargo.toml
	@ nix-shell --pure -A env --run 'cargo update'

README.md: $(wildcard src/bin/*.org)
	@ pandoc -f org -t gfm -o $@ $^
