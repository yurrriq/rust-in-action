.PHONY: all build doc

all: build

build:
	@ nix build -f . rust-in-action

doc:
	@ cargo doc --no-deps --bin ch3-mock-files

Cargo.lock: Cargo.toml
	@ cargo update

README.md: $(wildcard src/bin/*.org)
	@ pandoc -f org -t gfm -o $@ $^

