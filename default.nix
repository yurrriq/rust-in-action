with import ./nix/lib.nix;

{ nixpkgs ? fetchTarballFromGitHub (fromJSONFile ./nix/nixpkgs.json) }:

with import nixpkgs {

  overlays = [
    (self: super: {
      grep-lite = super.callPackage ./nix/pkgs/grep-lite.nix {};
      rust-in-action = super.callPackage ./nix/pkgs/rust-in-action.nix {};
    })
  ];

};

{

  inherit grep-lite rust-in-action;

  shell = mkShell {
    buildInputs = [
      cargo
      pandoc
      rustc
    ];
    RUST_BACKTRACE = 1;
  };

}
