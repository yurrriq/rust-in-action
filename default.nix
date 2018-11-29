with import ./nix/lib.nix;

{ nixpkgs ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs.json)
, mozillaOverlay ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs-mozilla.json)
}:

with import nixpkgs {
  overlays = [
    (import mozillaOverlay)
    (import ./nix/rust-pinned.nix)
  ];
};

{

  drv = (callPackage ./Cargo.nix { }).rust_in_action {};

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
