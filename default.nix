with import ./nix/lib.nix;

{ nixpkgs ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs.json)
, mozillaOverlay ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs-mozilla.json)
}:

with import nixpkgs {
  overlays = [
    (import mozillaOverlay)
    (import ./nix/rust-nightly.nix)
  ];
};

{

  drv = (callPackage ./Cargo.nix { }).hello {};

  env = stdenv.mkDerivation {
    name = "rust-in-action-env";
    buildInputs = [
      cargo
      gnumake
      pkgconfig
      rustc
    ];
    RUST_BACKTRACE = 1;
  };

}
