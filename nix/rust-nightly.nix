self: super:

let

  inherit (import ./lib.nix) fromJSONFile;

  nightly = (super.rustChannelOf (fromJSONFile ./rust-channel.json)).rust;

in

{

  rust = {
    cargo = nightly;
    rustc = nightly;
  };

}
