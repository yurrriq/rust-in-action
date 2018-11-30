with import ./nix/lib.nix;

{ nixpkgs ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs.json)
, mozillaOverlay ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs-mozilla.json)
}:

with import nixpkgs {
  overlays = [
    # FIXME
    # (import mozillaOverlay)
    # (import ./nix/rust-pinned.nix)
  ];
};

# {
#   drv = callPackage ./rust-in-action.nix {};
#   grep-lite = callPackage ./grep-lite {};
#   env =
stdenv.mkDerivation {
  name = "rust-in-action-env";
  buildInputs = [
    cargo
    rustc
  ];
  RUST_BACKTRACE = 1;
}#;
# }
