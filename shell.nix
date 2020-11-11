{ pkgs ? import ./nix }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    nixpkgs-fmt
    pandoc
    rustc
    rustfmt
  ];
  RUST_BACKTRACE = 1;
}
