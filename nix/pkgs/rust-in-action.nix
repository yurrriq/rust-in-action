{ rustPlatform, stdenv, ...}:

rustPlatform.buildRustPackage rec {
  name = "rust-in-action-${version}";
  version = "0.2.22";

  src = ../../.;

  cargoSha256 = "0lp6ixwk5x4j3vyabjg126j4i3rv6pkc3rbkdybqw0kj4n5k0fw4";

  meta = with stdenv.lib; {
    description = "Reading through Rust in Action";
    homepage = https://github.com/yurrriq/rust-in-action;
    license = licenses.unlicense;
    maintainers = [ maintainers.yurrriq ];
    platforms = platforms.all;
  };
}
