.PHONY: all build

all: build

build:
	nix-build -A drv

Cargo.lock: Cargo.toml
	nix-shell -A env --run 'cargo update'

Cargo.nix: Cargo.lock
	nix-shell --pure -p carnix --run 'carnix nix --src ./.'
