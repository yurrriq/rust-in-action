let
  sources = import ./sources.nix;
in
import sources.nixpkgs {
  overlays = [
    (self: super: {
      inherit (import sources.niv { pkgs = super; }) niv;
    })
    (self: super: {
      grep-lite = super.callPackage ../grep-lite;
      rust-in-action = super.callPackage ../.;
    })
  ];
}
