{ pkgs ? import ../nix }:
let
  src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "grep-lite";
  version = "0.2.12";

  inherit src;

  cargoSha256 = "02a05d8xijccl48bc3h0p00zpx3w3i6g8qzrvy5pnish98snwjag";

  meta = with pkgs.stdenv.lib; {
    description = "grep-lite";
    homepage = "https://github.com/yurrriq/rust-in-action";
    license = licenses.unlicense;
    maintainers = [ maintainers.yurrriq ];
    platforms = platforms.all;
  };
}
