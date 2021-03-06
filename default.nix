{ pkgs ? import ./nix }:
let
  src = pkgs.nix-gitignore.gitignoreSource [ "grep-lite/" ] ./.;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "rust-in-action";
  version = "0.3.21";

  inherit src;

  cargoSha256 = "1z4k4pz4i1daqgdxhgf0c69c4l2b1kn7p3vnhl3n2kimmdyra4w4";

  meta = with pkgs.stdenv.lib; {
    description = "Reading through Rust in Action";
    homepage = "https://github.com/yurrriq/rust-in-action";
    license = licenses.unlicense;
    maintainers = [ maintainers.yurrriq ];
    platforms = platforms.all;
  };
}
